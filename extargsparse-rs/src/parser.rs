use std::fmt;
use std::i64;
use std::error::Error;
use std::boxed::Box;
use std::io::Write;

use serde_json::Value;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use std::collections::HashMap;


use super::options::{ExtArgsOptions,OPT_HELP_HANDLER,OPT_LONG_PREFIX,OPT_SHORT_PREFIX,OPT_NO_HELP_OPTION,OPT_NO_JSON_OPTION,OPT_HELP_LONG,OPT_HELP_SHORT,OPT_JSON_LONG,OPT_CMD_PREFIX_ADDED, OPT_FLAG_NO_CHANGE};
use super::parser_compat::{ParserCompat};
use super::parser_state::{ParserState};
use super::key::{ExtKeyParse,KEYWORD_DOLLAR_SIGN,KEYWORD_HELP,KEYWORD_JSONFILE,KEYWORD_STRING,KEYWORD_INT,KEYWORD_FLOAT,KEYWORD_LIST,KEYWORD_BOOL,KEYWORD_COUNT,KEYWORD_ARGS,KEYWORD_COMMAND,KEYWORD_PREFIX ,KEYWORD_VARNAME,KEYWORD_LONGOPT, KEYWORD_SHORTOPT};
use super::const_value::{COMMAND_SET,SUB_COMMAND_JSON_SET,COMMAND_JSON_SET,ENVIRONMENT_SET,ENV_SUB_COMMAND_JSON_SET,ENV_COMMAND_JSON_SET,DEFAULT_SET};
use super::util::{check_in_array,format_array_string};
use lazy_static::lazy_static;

use super::logger::{extargs_debug_out};
use super::{extargs_assert,extargs_log_info,extargs_log_trace,extargs_log_warn};
use super::namespace::{NameSpaceEx};
use super::funccall::{ExtArgsMatchFuncMap};
use super::helpsize::{HelpSize};
use super::optchk::{OptChk};


use super::{error_class,new_error};


error_class!{ParserError}

#[allow(dead_code)]
#[derive(Clone)]
enum ExtArgsFunc {
	LoadFunc(Rc<dyn Fn(String,ExtKeyParse,Vec<ParserCompat>) -> Result<(),Box<dyn Error>>>),
	ActionFunc(Rc<dyn Fn(NameSpaceEx,i32,ExtKeyParse,Vec<String>) -> Result<i32,Box<dyn Error>>>),
	LoadJsonFunc(Rc<dyn Fn(NameSpaceEx) -> Result<(),Box<dyn Error>>>),
	JsonFunc(Rc<dyn Fn(NameSpaceEx,ExtKeyParse,Value) -> Result<(),Box<dyn Error>>>),	
}

#[allow(dead_code)]
#[derive(Clone)]
struct InnerExtArgsParser {
	options :ExtArgsOptions,
	maincmd :ParserCompat,
	arg_state :Option<ParserState>,
	error_handler :String,
	help_handler :String,
	output_mode :Vec<String>,
	ended : i32,
	long_prefix :String,
	short_prefix :String,
	no_help_option : bool,
	no_json_option : bool,
	opt_flag_no_change : bool,
	help_long :String,
	help_short : String,
	json_long :String,
	cmd_prefix_added :bool,
	load_priority :Vec<i32>,
	extfuncs :Rc<RefCell<HashMap<String,Rc<RefCell<ExtArgsFunc>>>>>,
	outfuncs :ExtArgsMatchFuncMap,
}

lazy_static ! {
	static ref PARSER_PRIORITY_ARGS :Vec<i32> = {
		vec![COMMAND_SET,SUB_COMMAND_JSON_SET,COMMAND_JSON_SET,ENVIRONMENT_SET,ENV_SUB_COMMAND_JSON_SET,ENV_COMMAND_JSON_SET,DEFAULT_SET]
	};

	static ref PARSER_RESERVE_ARGS :Vec<String> = {
		vec![String::from("subcommand"),String::from("subnargs"),String::from("nargs")]
	};
}

fn is_valid_priority (k :i32) -> bool {
	for v in PARSER_PRIORITY_ARGS.iter() {
		if *v == k {
			return true;
		}
	}
	return false;
}



#[allow(dead_code)]
impl InnerExtArgsParser {
	fn insert_load_command_funcs(&mut self)  {
		let b = Arc::new(RefCell::new(self.clone()));
		let s1 = b.clone();
		self.extfuncs.borrow_mut().insert(format!("{}",KEYWORD_STRING),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| {  s1.borrow_mut().load_commandline_base(n,k,v) } )))));
		let s1 = b.clone();
		self.extfuncs.borrow_mut().insert(format!("{}",KEYWORD_INT),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| {  s1.borrow_mut().load_commandline_base(n,k,v) } )))));
		let s1 = b.clone();
		self.extfuncs.borrow_mut().insert(format!("{}",KEYWORD_FLOAT),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| {  s1.borrow_mut().load_commandline_base(n,k,v) } )))));
		let s1 = b.clone();
		self.extfuncs.borrow_mut().insert(format!("{}",KEYWORD_LIST),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| {  s1.borrow_mut().load_commandline_base(n,k,v) } )))));
		let s1 = b.clone();
		self.extfuncs.borrow_mut().insert(format!("{}",KEYWORD_BOOL),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| {  s1.borrow_mut().load_commandline_base(n,k,v) } )))));
		let s1 = b.clone();
		self.extfuncs.borrow_mut().insert(format!("{}",KEYWORD_ARGS),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| {  s1.borrow_mut().load_commandline_args(n,k,v) } )))));
		let s1 = b.clone();
		self.extfuncs.borrow_mut().insert(format!("{}",KEYWORD_COMMAND),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| {  s1.borrow_mut().load_command_subparser(n,k,v) } )))));
		let s1 = b.clone();
		self.extfuncs.borrow_mut().insert(format!("{}",KEYWORD_PREFIX),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| {  s1.borrow_mut().load_command_prefix(n,k,v) } )))));
		let s1 = b.clone();
		self.extfuncs.borrow_mut().insert(format!("{}",KEYWORD_COUNT),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| {  s1.borrow_mut().load_commandline_base(n,k,v) } )))));
		let s1 = b.clone();
		self.extfuncs.borrow_mut().insert(format!("{}",KEYWORD_HELP),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| {  s1.borrow_mut().load_commandline_base(n,k,v) } )))));
		let s1 = b.clone();
		self.extfuncs.borrow_mut().insert(format!("{}",KEYWORD_JSONFILE),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| {  s1.borrow_mut().load_commandline_base(n,k,v) } )))));
		return;
	}
	pub fn new(opt :Option<ExtArgsOptions>,priority :Option<Vec<i32>>) -> Result<InnerExtArgsParser,Box<dyn Error>> {
		let mut setopt = ExtArgsOptions::new("{}")?.clone();
		let mut setpriority = PARSER_PRIORITY_ARGS.clone();
		if opt.is_some() {
			setopt = opt.as_ref().unwrap().clone();
		}

		if priority.is_some() {
			setpriority = priority.as_ref().unwrap().clone();
		}
		for v in setpriority.iter() {
			if !is_valid_priority(*v) {
				new_error!{ParserError,"unknown type [{}]",  *v}
			}
		}
		let mut retv :InnerExtArgsParser = InnerExtArgsParser {
			options : setopt.clone(),
			maincmd : ParserCompat::new(None,Some(setopt.clone())),
			arg_state : None,
			error_handler : "".to_string(),
			help_handler : format!("{}",setopt.get_string(OPT_HELP_HANDLER)),
			output_mode : Vec::new(),
			ended : 0,
			long_prefix : setopt.get_string(OPT_LONG_PREFIX),
			short_prefix : setopt.get_string(OPT_SHORT_PREFIX),
			no_help_option : setopt.get_bool(OPT_NO_HELP_OPTION),
			no_json_option : setopt.get_bool(OPT_NO_JSON_OPTION),
			opt_flag_no_change : setopt.get_bool(OPT_FLAG_NO_CHANGE),
			help_long : setopt.get_string(OPT_HELP_LONG),
			help_short : setopt.get_string(OPT_HELP_SHORT),
			json_long : setopt.get_string(OPT_JSON_LONG),
			cmd_prefix_added : setopt.get_bool(OPT_CMD_PREFIX_ADDED),
			load_priority : setpriority.clone(),
			extfuncs : Rc::new(RefCell::new(HashMap::new())),
			outfuncs : ExtArgsMatchFuncMap::new(),
		};
		retv.insert_load_command_funcs();

		Ok(retv)
	}

	fn check_flag_insert(&mut self,keycls :ExtKeyParse,parsers :Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		let lastparser :ParserCompat;
		let mut parserclone :i32 = 0;
		if parsers.len() > 0 {
			lastparser = parsers[parsers.len() - 1].clone();
			parserclone = 1;
		} else {
			lastparser = self.maincmd.clone();
		}

		for opt in lastparser.get_cmdopts().iter() {
			if opt.flag_name() != KEYWORD_DOLLAR_SIGN && keycls.flag_name() != KEYWORD_DOLLAR_SIGN {
				if opt.type_name() != KEYWORD_HELP && keycls.type_name() != KEYWORD_HELP {
					if opt.opt_dest().eq(&keycls.opt_dest()) {
						new_error!{ParserError,"[{}] already inserted", keycls.opt_dest()}
					}
				} else if opt.type_name() == KEYWORD_HELP && keycls.type_name() == KEYWORD_HELP {
					new_error!{ParserError,"help [{}] had already inserted", keycls.string()}
				}
			} else if opt.flag_name() == KEYWORD_DOLLAR_SIGN && keycls.flag_name() == KEYWORD_DOLLAR_SIGN {
				new_error!{ParserError,"args [{}] already inserted", keycls.string()}
			}
		}

		if parserclone > 0 {
			let uc = parsers.len() -1;
			parsers[uc].push_cmdopts(keycls);
		} else {
			self.maincmd.push_cmdopts(keycls);
		}

		Ok(())
	}

	fn format_cmd_from_cmd_array(&self,parsers :Vec<ParserCompat>) -> String {
		let mut rets :String = "".to_string();
		for v in parsers.iter() {
			if rets.len() > 0 {
				rets.push_str(".");
			}
			rets.push_str(&v.cmd_name());
		}
		rets
	}

	fn load_commandline_json_file(&mut self,keycls :ExtKeyParse,parsers :Vec<ParserCompat>) -> Result<(), Box<dyn Error>> {
		return self.check_flag_insert(keycls,parsers);
	}

	fn load_commandline_json_added(&mut self,parsers :Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		let mut prefix :String;
		let key1 :String;
		let v :Value;
		let keycls :ExtKeyParse;
		key1 = format!("{}##json input file to get the value set##",self.json_long);
		prefix = self.format_cmd_from_cmd_array(parsers.clone());
		prefix = prefix.replace(".","_");
		v = Value::Null;
		let res = ExtKeyParse::new(&prefix,&key1,&v,true,false,true,&self.long_prefix,&self.short_prefix,false);
		extargs_assert!(res.is_ok(), "create json keycls error [{:?}]", res.err().unwrap());
		keycls = res.unwrap();
		return self.load_commandline_json_file(keycls,parsers);
	}

	fn load_commandline_help(&mut self, keycls :ExtKeyParse, parsers :Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		return self.check_flag_insert(keycls,parsers);
	}

	fn load_commandline_help_added(&mut self,parsers :Vec<ParserCompat>) -> Result<(), Box<dyn Error>> {
		let mut key1 :String = "".to_string();
		let v :Value;

		key1.push_str(&format!("{}",self.help_long));
		if self.help_short.len() > 0 {
			key1.push_str(&format!("|{}",self.help_short));
		}
		v = Value::Null;
		let res = ExtKeyParse::new("",&key1,&v,true,true,false,&self.long_prefix,&self.short_prefix,false);
		extargs_assert!(res.is_ok(),"create help keycls error [{:?}]", res.err().unwrap());
		let keycls = res.unwrap();
		return self.load_commandline_help(keycls,parsers);
	}

	fn load_commandline_base(&mut self, _prefix :String, keycls :ExtKeyParse, parsers :Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		if keycls.is_flag() && keycls.flag_name() != KEYWORD_DOLLAR_SIGN && check_in_array(PARSER_RESERVE_ARGS.clone(),&(keycls.flag_name())) {
			new_error!{ParserError,"{} in the {}", keycls.flag_name(), format_array_string(PARSER_RESERVE_ARGS.clone())}
		}
		return self.check_flag_insert(keycls,parsers);
	}

	fn load_commandline_args(&mut self, _prefix :String, keycls :ExtKeyParse, parsers :Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		return self.check_flag_insert(keycls,parsers);
	}

	fn  find_subparser_inner(&self,_name :&str, _oparser :Option<ParserCompat>) -> Option<ParserCompat> {
		let sparser :ParserCompat;
		if _oparser.is_none() {
			sparser = self.maincmd.clone();
		} else {
			sparser = _oparser.as_ref().unwrap().clone();
		}

		if _name.len() == 0 {
			return Some(sparser.clone());
		}

		let bname :String = format!("{}",_name);
		let sarr : Vec<&str> = bname.split(".").collect();
		for v in sparser.sub_cmds().iter() {
			if  sarr.len() > 0 && v.cmd_name().eq(sarr[0]) {
				let sname = sarr[1..sarr.len()].join(".");
				let f = self.find_subparser_inner(&sname,Some(v.clone()));
				if f.is_some() {
					return f;
				}
			}
		}

		return None;
	}

	fn get_subparser_inner(&self, keycls :ExtKeyParse,parsers :Vec<ParserCompat>) -> Option<ParserCompat> {
		let retv :Option<ParserCompat>;
		let mut cmdname :String = "".to_string();
		let parentname :String;
		let cparser :ParserCompat;
		let curparser :ParserCompat;
		parentname = self.format_cmd_from_cmd_array(parsers.clone());
		cmdname.push_str(&parentname);
		if cmdname.len() > 0 {
			cmdname.push_str(".");
		}
		cmdname.push_str(&keycls.clone().cmd_name());
		let oparser = self.find_subparser_inner(&cmdname,None);
		if oparser.is_some() {
			return oparser;
		}
		let setopt :Option<ExtArgsOptions>;
		setopt = Some(self.options.clone());
		cparser = ParserCompat::new(Some(keycls.clone()),setopt);
		extargs_log_info!("{}",cparser.string());
		if parsers.len() > 0 {
			curparser = parsers[parsers.len() - 1].clone();
			curparser.push_subcmds(cparser.clone());
			retv= Some(cparser.clone());
		} else {
			curparser = self.maincmd.clone();
			curparser.push_subcmds(cparser.clone());
			retv = Some(cparser.clone());
		}
		retv
	}


	fn load_commandline_inner(&mut self, prefix :String, vmap :Value, parsers :Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		if !self.no_json_option && self.json_long.len() > 0 {
			self.load_commandline_json_added(parsers.clone())?;
		}

		if !self.no_help_option && self.help_long.len() > 0 {
			self.load_commandline_help_added(parsers.clone())?;
		}


		if !vmap.is_object() {
			new_error!{ParserError,"{:?} not object", vmap}
		}
		for (k, v) in vmap.as_object().unwrap() {
			extargs_log_info!("{} , {} , {:?} , False",prefix,k,v);
			let curkeycls = ExtKeyParse::new(&prefix,k,v,false,false,false,&self.long_prefix,&self.short_prefix,self.opt_flag_no_change)?;
			self.call_load_command_map_func(format!("{}",prefix),curkeycls.clone(),parsers.clone())?;
		}
		Ok(())
	}

	fn load_command_subparser(&mut self,prefix :String, keycls :ExtKeyParse, parsers :Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		let mut newprefix :String;
		if keycls.type_name() != KEYWORD_COMMAND {
			new_error!{ParserError,"{} not valid command", keycls.string()}
		}
		if keycls.cmd_name().len() > 0 && check_in_array(PARSER_RESERVE_ARGS.clone(),&(keycls.cmd_name())) {
			new_error!{ParserError,"{} in reserved {}", keycls.cmd_name(), format_array_string(PARSER_RESERVE_ARGS.clone())}
		}
		extargs_log_info!("load [{}]",keycls.string());

		let oparser = self.get_subparser_inner(keycls.clone(),parsers.clone());
		if oparser.is_none() {
			new_error!{ParserError,"can not find [{}] ", keycls.string()}
		}

		let mut nextparsers :Vec<ParserCompat>;
		if parsers.len() > 0 {
			nextparsers = parsers.clone();
		} else {
			nextparsers = Vec::new();
			nextparsers.push(self.maincmd.clone());
		}
		nextparsers.push(oparser.unwrap().clone());
		newprefix = "".to_string();
		if self.cmd_prefix_added {
			newprefix.push_str(&prefix);
			if newprefix.len() > 0 {
				newprefix.push_str("_");
			}
			newprefix.push_str(&(keycls.cmd_name()));
		}

		return self.load_commandline_inner(newprefix,keycls.value().clone(),nextparsers);
	}

	fn load_command_prefix(&mut self,_prefix :String, keycls :ExtKeyParse, parsers :Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		if keycls.prefix().len() > 0 && check_in_array(PARSER_RESERVE_ARGS.clone(),&(keycls.prefix())) {
			new_error!{ParserError,"prefix [{}] in [{}]", keycls.prefix(), format_array_string(PARSER_RESERVE_ARGS.clone())}
		}
		return self.load_commandline_inner(keycls.prefix(),keycls.value().clone(),parsers.clone());
	}

	fn get_ext_func(&mut self, k :&str) -> Option<ExtArgsFunc> {
		let mut retv : Option<ExtArgsFunc> = None;
		match self.extfuncs.borrow().get(k) {
			Some(f1) => {
				let f2 :&ExtArgsFunc = &f1.borrow();
				retv = Some(f2.clone());
			},
			None => {}
		}
		retv
	}

	fn call_load_command_map_func(&mut self,prefix :String,keycls :ExtKeyParse, parsers :Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		let fnptr :Option<ExtArgsFunc>;
		fnptr = self.get_ext_func(&prefix);
		if fnptr.is_some() {
			let f2 = fnptr.unwrap();
			match f2 {
				ExtArgsFunc::LoadFunc(f) => {
					return f(prefix,keycls.clone(),parsers.clone());
				},
				_ => {
					new_error!{ParserError,"return [{}] not load function", prefix}
				}
			}
		} else {
			new_error!{ParserError,"can not found [{}] load command map function", prefix}
		}
	}

	fn string_action(&mut self,ns :NameSpaceEx,validx :i32,keycls :ExtKeyParse,params :Vec<String>) -> Result<i32,Box<dyn Error>> {
		if validx >= params.len() as i32 {
			new_error!{ParserError,"need args [{}] [{}] [{:?}]", validx, keycls.string(), params}
		}
		extargs_log_trace!("set [{}] [{}]",keycls.opt_dest(),params[validx as usize]);
		let n = format!("{}",keycls.opt_dest());
		let v = format!("{}",params[validx as usize]);
		ns.set_string(n,v)?;
		Ok(1)
	}

	fn bool_action(&mut self,ns :NameSpaceEx, _validx :i32 , keycls :ExtKeyParse, _params :Vec<String>) -> Result<i32,Box<dyn Error>> {
		let mut b :bool = false;
		match keycls.value() {
			Value::Bool(bv) => {
				b = bv;
			},
			_ => {
				extargs_log_warn!("[{}] not set true|false", keycls.string());
			}
		}
		ns.set_bool(keycls.opt_dest(),b)?;
		Ok(0)
	}

	fn int_action(&mut self,ns :NameSpaceEx, validx :i32 , keycls :ExtKeyParse, params :Vec<String>) -> Result<i32, Box<dyn Error>> {
		let mut base : u32 = 10;
		let mut cparse :String;
		if validx >= params.len() as i32 {
			new_error!{ParserError, "[{}] >= [{}]", validx, params.len()}
		}
		cparse = format!("{}",params[validx as usize]);
		if cparse.starts_with("0x") || cparse.starts_with("0X") {
			cparse = cparse[2..].to_string();
			base = 16;
		} else if cparse.starts_with("x") || cparse.starts_with("X") {
			cparse = cparse[1..].to_string();
			base = 16;
		}

		match i64::from_str_radix(&cparse,base) {
			Ok(v) => {
				ns.set_int(&(keycls.opt_dest()),v)?;
			},
			Err(e) => {
				new_error!{ParserError, "parse [{}] error [{:?}]", params[ validx as usize], e}
			}
		}
		Ok(1)
	}

	fn append_action(&mut self,ns :NameSpaceEx, validx :i32 , keycls :ExtKeyParse, params :Vec<String>) -> Result<i32, Box<dyn Error>> {
		let mut carr :Vec<String>;
		if validx >= params.len() as i32 {
			new_error!{ParserError,"[{}] >= [{}]", validx,params.len()}
		}
		carr = ns.get_array(&(keycls.opt_dest()));
		carr.push(format!("{}",params[validx as usize]));
		ns.set_array(&(keycls.opt_dest()), carr)?;
		Ok(1)
	}

	fn print_help(&self,parsers :Vec<ParserCompat>) -> String {
		let mut curcmd :ParserCompat;
		let mut cmdpaths :Vec<ParserCompat> = Vec::new();
		if self.help_handler == "nohelp" {
			return format!("no help information");
		}
		curcmd = self.maincmd.clone();
		if parsers.len() > 0 {
			let ilen :usize = parsers.len() - 1;
			curcmd = parsers[ilen].clone();
			for i in 0..ilen {
				cmdpaths.push(parsers[i].clone());
			}
		}

		return curcmd.get_help_info_ex(HelpSize::new(),cmdpaths,self.outfuncs.clone());
	}

	fn set_commandline_self_args_inner(&mut self,paths :Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		let mut parentpaths :Vec<ParserCompat> = Vec::new();
		let mut setted :bool;
		let ilen :usize;
		if paths.len() > 0 {
			parentpaths = paths.clone();
			ilen = parentpaths.len() - 1;
		} else {
			parentpaths.push(self.maincmd.clone());
			ilen = 0;
		}
		setted = false;
		for opt in parentpaths[ilen].get_cmdopts() {
			if opt.is_flag() && opt.flag_name() == KEYWORD_DOLLAR_SIGN {
				setted = true;
				break;
			}
		}

		if !setted {
			let cmdname = self.format_cmd_from_cmd_array(parentpaths.clone());
			let prefix = cmdname.replace(".","_");
			let vstart = serde_json::from_str("\"*\"").unwrap();
			let curkey = ExtKeyParse::new("","$",&vstart,true,false,false,&(self.long_prefix),&(self.short_prefix),self.opt_flag_no_change)?;
			self.load_commandline_args(prefix,curkey,parentpaths.clone())?;
		}

		for chld in parentpaths[ilen].sub_cmds() {
			let mut curparsers : Vec<ParserCompat> = parentpaths.clone();
			curparsers.push(chld);
			self.set_commandline_self_args_inner(curparsers)?;
		}
		Ok(())
	}

	fn check_var_name_inner(&self,paths :Vec<ParserCompat>,optchk :OptChk) -> Result<(),Box<dyn Error>> {
		let mut parentpaths :Vec<ParserCompat>;
		let ilen :usize;
		let mut retb : bool;

		if paths.len() > 0 {
			parentpaths = paths.clone();
		} else {
			parentpaths = Vec::new();
			parentpaths.push(self.maincmd.clone());
		}

		ilen = parentpaths.len() - 1;
		for opt in parentpaths[ilen].get_cmdopts().iter() {
			if opt.is_flag() {
				if opt.type_name() == KEYWORD_HELP || opt.type_name() == KEYWORD_ARGS {
					continue;
				}
				retb = optchk.add_and_check(format!("{}",KEYWORD_VARNAME),&(opt.var_name()));
				if !retb {
					new_error!{ParserError,"opt varname[{}] is already", opt.var_name()}
				}

				retb = optchk.add_and_check(format!("{}",KEYWORD_LONGOPT),&(opt.long_opt()));
				if !retb {
					new_error!{ParserError,"opt longopt[{}] is already", opt.long_opt()}
				}

				if opt.short_opt().len() > 0 {
					retb = optchk.add_and_check(format!("{}",KEYWORD_SHORTOPT),&(opt.short_opt()));
					if !retb {
						new_error!{ParserError,"opt shortopt[{}] is already",opt.short_opt()}
					}
				}
			}
		}

		for c in parentpaths[ilen].sub_cmds() {
			let mut curpaths :Vec<ParserCompat>;
			let cpychk :OptChk;
			curpaths = parentpaths.clone();
			curpaths.push(c);
			cpychk = OptChk::new();
			cpychk.copy(&optchk);
			self.check_var_name_inner(curpaths,cpychk)?;
		}
		Ok(())
	}

	fn set_commandline_self_args(&mut self) -> Result<(),Box<dyn Error>> {
		if self.ended != 0 {
			return Ok(());
		}
		let paths :Vec<ParserCompat> = Vec::new();
		self.set_commandline_self_args_inner(paths.clone())?;
		self.check_var_name_inner(paths.clone(),OptChk::new())?;
		Ok(())
	}

	fn find_command_inner(&self,cmdname :String,parsers :Vec<ParserCompat>) -> Option<ParserCompat> {
		let sarr :Vec<&str> = cmdname.split(".").collect();
		let curroot : ParserCompat;
		let mut nextparsers :Vec<ParserCompat>;
		let ilen :usize;
		if parsers.len() > 0 {
			nextparsers = parsers.clone();
			ilen = nextparsers.len() - 1;
			curroot = nextparsers[ilen].clone();
		} else {
			nextparsers = Vec::new();
			nextparsers.push(self.maincmd.clone());
			curroot = self.maincmd.clone();
		}

		if sarr.len() > 1 {
			nextparsers.push(curroot.clone());
			for c in curroot.sub_cmds() {
				if c.cmd_name().eq(sarr[0]) {
					let sname = sarr[1..sarr.len()].join(".");
					nextparsers = Vec::new();
					if parsers.len() > 0 {
						nextparsers = parsers.clone();
					}
					nextparsers.push(c);
					return self.find_command_inner(sname,nextparsers);
				}
			}
		} else if sarr.len() == 1 {
			for c in curroot.sub_cmds() {
				if c.cmd_name().eq(sarr[0]) {
					return Some(c.clone());
				}
			}
		}
		None
	}

	fn find_command_in_path(&self, cmdname :String, _parsers :Vec<ParserCompat>) -> Vec<ParserCompat> {
		let mut sarr :Vec<&str> = Vec::new();
		let mut commands :Vec<ParserCompat> = Vec::new();
		let mut i :i32;
		if cmdname.len() > 0 {
			sarr = cmdname.split(".").collect();
		}
		extargs_log_trace!("append [{}]",self.maincmd.string());
		commands.push(self.maincmd.clone());

		i = 0;
		while i <= sarr.len() as i32 && cmdname.len() > 0 {
			if i > 0 {
				let curcommand = self.find_command_inner(format!("{}",sarr[(i-1) as usize]),commands.clone());
				if curcommand.is_none() {
					break;
				}
				let cmd = curcommand.unwrap();
				extargs_log_trace!("append [{}]",cmd.string());
				commands.push(cmd.clone());
			}
			i += 1;
		}
		return commands;
	}

	pub (crate) fn print_help_ex<T : std::io::Write>(&mut self, iowriter :&mut T,cmdname :String) -> Result<usize,Box<dyn Error>> {
		let mut parsers :Vec<ParserCompat>;
		self.set_commandline_self_args()?;
		parsers = Vec::new();
		parsers = self.find_command_in_path(format!("{}",cmdname),parsers.clone());
		if parsers.len() == 0 {
			new_error!{ParserError,"can not find [{}] for help", cmdname}
		}

		let s = self.print_help(parsers.clone());
		if self.output_mode.len() > 0 {
			let ilen :usize = self.output_mode.len() - 1;
			if self.output_mode[ilen] == "bash" {
				let outs = format!("cat <<EOFMM\n{}\nEOFMM\nexit 0", s);
				let mut of = std::io::stdout();
				of.write(outs.as_bytes()).unwrap();
				std::process::exit(0);
			}
		}

		let totallen = iowriter.write(s.as_bytes())?;
		Ok(totallen)
	}

	fn help_action(&mut self,_ns :NameSpaceEx,_valid :i32, _keycls :ExtKeyParse, params :Vec<String>) -> Result<i32, Box<dyn Error>> {
		if params.len() == 0 {
			new_error!{ParserError,"no params in help action"}
		}
		let mut of = std::io::stdout();
		_ = self.print_help_ex(&mut of,format!("{}",params[0]))?;
		std::process::exit(0);
	}

	fn inc_action(&mut self, ns :NameSpaceEx, _validx :i32, keycls :ExtKeyParse, _params :Vec<String>) -> Result<i32,Box<dyn Error>> {
		let mut iv :i64;
		iv = ns.get_int(&(keycls.opt_dest()));
		iv += 1;
		ns.set_int(&(keycls.opt_dest()),iv)?;
		Ok(0)
	}

	fn command_action(&mut self, _ns :NameSpaceEx, _validx :i32, _keycls :ExtKeyParse, _params :Vec<String>) -> Result<i32,Box<dyn Error>> {
		Ok(0)
	}

	fn float_action(&mut self, _ns :NameSpaceEx, _validx :i32, _keycls :ExtKeyParse, _params :Vec<String>) -> Result<i32,Box<dyn Error>> {
		if _validx >= _params.len() as i32  {
			new_error!{ParserError,"need args [{}] [{}] [{:?}]",_validx, _keycls.string(),_params}
		}
		match _params[_validx as usize].parse::<f64>() {
			Ok(fv) => {
				_ns.set_float(&(_keycls.opt_dest()),fv)?;
			},
			Err(e) => {
				new_error!{ParserError,"parse [{}] not float [{:?}]", _params[_validx as usize], e}
			}
		}
		Ok(1)
	}

}

#[allow(dead_code)]
#[derive(Clone)]
pub struct ExtArgsParser {
	innerrc : Rc<RefCell<InnerExtArgsParser>>,
}

impl  ExtArgsParser {
	pub fn new(opt :Option<ExtArgsOptions>,priority :Option<Vec<i32>>) -> Result<ExtArgsParser,Box<dyn Error>> {
		let k = InnerExtArgsParser::new(opt,priority)?;
		Ok(ExtArgsParser {
			innerrc : Rc::new(RefCell::new(k)),
		})
	}
}