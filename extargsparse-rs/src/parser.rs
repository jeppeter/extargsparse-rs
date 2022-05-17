use std::i64;
use std::error::Error;
use std::boxed::Box;
use std::io::Write;
use std::fs::File;
use std::io::prelude::*;
use std::env;


use serde_json::Value;
use std::rc::Rc;
use std::cell::{RefCell,UnsafeCell};
use std::sync::Arc;
use std::collections::HashMap;
use std::any::Any;


use super::options::{ExtArgsOptions,OPT_HELP_HANDLER,OPT_LONG_PREFIX,OPT_SHORT_PREFIX,OPT_NO_HELP_OPTION,OPT_NO_JSON_OPTION,OPT_HELP_LONG,OPT_HELP_SHORT,OPT_JSON_LONG,OPT_CMD_PREFIX_ADDED, OPT_FLAG_NO_CHANGE};
use super::parser_compat::{ParserCompat};
use super::parser_state::{ParserState,StateOptVal};
use super::key::{ExtKeyParse,KEYWORD_DOLLAR_SIGN,KEYWORD_HELP,KEYWORD_JSONFILE,KEYWORD_STRING,KEYWORD_INT,KEYWORD_FLOAT,KEYWORD_LIST,KEYWORD_BOOL,KEYWORD_COUNT,KEYWORD_ARGS,KEYWORD_COMMAND,KEYWORD_PREFIX ,KEYWORD_VARNAME,KEYWORD_LONGOPT, KEYWORD_SHORTOPT,KEYWORD_ATTR,KEYWORD_SUBCOMMAND,KEYWORD_NARGS,KEYWORD_SUBNARGS,Nargs};
use super::const_value::{COMMAND_SET,SUB_COMMAND_JSON_SET,COMMAND_JSON_SET,ENVIRONMENT_SET,ENV_SUB_COMMAND_JSON_SET,ENV_COMMAND_JSON_SET,DEFAULT_SET};
use super::util::{check_in_array,format_array_string};
use super::argset::{ArgSetImpl};
use lazy_static::lazy_static;

use super::logger::{extargs_debug_out};
use super::{extargs_assert,extargs_log_info,extargs_log_trace,extargs_log_warn};
use super::namespace::{NameSpaceEx};
use super::funccall::{ExtArgsMatchFuncMap,ExtArgsParseFunc};
use super::helpsize::{HelpSize};
use super::optchk::{OptChk};


use super::{error_class,new_error};


error_class!{ParserError}

#[derive(Clone)]
enum ExtArgsFunc {
	LoadFunc(Rc<dyn Fn(String,ExtKeyParse,Vec<ParserCompat>) -> Result<(),Box<dyn Error>>>),
	ActionFunc(Rc<dyn Fn(NameSpaceEx,i32,ExtKeyParse,Vec<String>) -> Result<i32,Box<dyn Error>>>),
	LoadJsonFunc(Rc<dyn Fn(NameSpaceEx,Option<ParserState>) -> Result<(),Box<dyn Error>>>),
	JsonFunc(Rc<dyn Fn(NameSpaceEx,ExtKeyParse,Value) -> Result<(),Box<dyn Error>>>),	
}


#[derive(Clone)]
struct InnerExtArgsParser {
	options :ExtArgsOptions,
	maincmd :ParserCompat,
	arg_state :Option<ParserState>,
	//error_handler :String,
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
	loadfuncs :Rc<RefCell<HashMap<String,Rc<RefCell<ExtArgsFunc>>>>>,
	jsonfuncs :Rc<RefCell<HashMap<String,Rc<RefCell<ExtArgsFunc>>>>>,
	optfuncs :Rc<RefCell<HashMap<String,Rc<RefCell<ExtArgsFunc>>>>>,
	setmapfuncs :Rc<RefCell<HashMap<i32,Rc<RefCell<ExtArgsFunc>>>>>,
	outfuncs :ExtArgsMatchFuncMap,
}



lazy_static ! {
	static ref PARSER_PRIORITY_ARGS :Vec<i32> = {
		vec![COMMAND_SET,SUB_COMMAND_JSON_SET,COMMAND_JSON_SET,ENVIRONMENT_SET,ENV_SUB_COMMAND_JSON_SET,ENV_COMMAND_JSON_SET,DEFAULT_SET]
	};

	static ref PARSER_RESERVE_ARGS :Vec<String> = {
		vec![String::from(KEYWORD_SUBCOMMAND),String::from(KEYWORD_SUBNARGS),String::from(KEYWORD_NARGS)]
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

impl InnerExtArgsParser {
	fn insert_load_command_funcs(&mut self)  {
		let b = Arc::new(UnsafeCell::new(self.clone()));
		let mut bmut =  self.loadfuncs.borrow_mut();
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_STRING),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| {  
			extargs_log_trace!("call [{}]", KEYWORD_STRING) ;
			let  c :&mut InnerExtArgsParser = unsafe {&mut *s1.get()};
			c.load_commandline_base(n,k,v)
		} )))));
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_INT),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| { extargs_log_trace!("call [{}]", KEYWORD_INT) ; 
			let  c :&mut InnerExtArgsParser = unsafe {&mut *s1.get()};
			c.load_commandline_base(n,k,v) } )))));
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_FLOAT),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| { extargs_log_trace!("call [{}]", KEYWORD_FLOAT) ;  
			let  c :&mut InnerExtArgsParser = unsafe {&mut *s1.get()};
			c.load_commandline_base(n,k,v) } )))));
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_LIST),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| { extargs_log_trace!("call [{}]", KEYWORD_LIST) ; 
			let  c :&mut InnerExtArgsParser = unsafe {&mut *s1.get()};
			c.load_commandline_base(n,k,v) } )))));
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_BOOL),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| { extargs_log_trace!("call [{}]", KEYWORD_BOOL) ; 
			let  c :&mut InnerExtArgsParser = unsafe {&mut *s1.get()};
			c.load_commandline_base(n,k,v) } )))));
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_ARGS),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| { extargs_log_trace!("call [{}]", KEYWORD_ARGS) ; 
			let  c :&mut InnerExtArgsParser = unsafe {&mut *s1.get()};
			c.load_commandline_args(n,k,v) } )))));
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_COMMAND),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| { extargs_log_trace!("call [{}]", KEYWORD_COMMAND) ; 
			let  c :&mut InnerExtArgsParser = unsafe {&mut *s1.get()};
			c.load_command_subparser(n,k,v) 
		} )))));
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_PREFIX),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| { extargs_log_trace!("call [{}]", KEYWORD_PREFIX) ; 
			let  c :&mut InnerExtArgsParser = unsafe {&mut *s1.get()};
			c.load_command_prefix(n,k,v) } )))));
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_COUNT),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| { extargs_log_trace!("call [{}]", KEYWORD_COUNT) ; 
			let  c :&mut InnerExtArgsParser = unsafe {&mut *s1.get()};
			c.load_commandline_base(n,k,v) } )))));
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_HELP),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| { extargs_log_trace!("call [{}]", KEYWORD_HELP) ; 
			let  c :&mut InnerExtArgsParser = unsafe {&mut *s1.get()};
			c.load_commandline_base(n,k,v) } )))));
		let s1 = b.clone();
		bmut.insert(format!("{}",KEYWORD_JSONFILE),Rc::new(RefCell::new(ExtArgsFunc::LoadFunc(Rc::new(move |n,k,v| { extargs_log_trace!("call [{}]", KEYWORD_JSONFILE) ;  
			let  c :&mut InnerExtArgsParser = unsafe {&mut *s1.get()};
			c.load_commandline_base(n,k,v) } )))));
		return;
	}	


	fn insert_json_funcs(&mut self) {
		let b = Arc::new(RefCell::new(self.clone()));
		let s1 = b.clone();
		self.jsonfuncs.borrow_mut().insert(format!("{}",KEYWORD_STRING),Rc::new(RefCell::new(ExtArgsFunc::JsonFunc(Rc::new(move |n,k,v| { s1.borrow_mut().json_value_base(n,k,v) })))));
		let s1 = b.clone();
		self.jsonfuncs.borrow_mut().insert(format!("{}",KEYWORD_BOOL),Rc::new(RefCell::new(ExtArgsFunc::JsonFunc(Rc::new(move |n,k,v| { s1.borrow_mut().json_value_base(n,k,v) })))));
		let s1 = b.clone();
		self.jsonfuncs.borrow_mut().insert(format!("{}",KEYWORD_INT),Rc::new(RefCell::new(ExtArgsFunc::JsonFunc(Rc::new(move |n,k,v| { s1.borrow_mut().json_value_base(n,k,v) })))));
		let s1 = b.clone();
		self.jsonfuncs.borrow_mut().insert(format!("{}",KEYWORD_LIST),Rc::new(RefCell::new(ExtArgsFunc::JsonFunc(Rc::new(move |n,k,v| { s1.borrow_mut().json_value_base(n,k,v) })))));
		let s1 = b.clone();
		self.jsonfuncs.borrow_mut().insert(format!("{}",KEYWORD_COUNT),Rc::new(RefCell::new(ExtArgsFunc::JsonFunc(Rc::new(move |n,k,v| { s1.borrow_mut().json_value_base(n,k,v) })))));
		let s1 = b.clone();
		self.jsonfuncs.borrow_mut().insert(format!("{}",KEYWORD_JSONFILE),Rc::new(RefCell::new(ExtArgsFunc::JsonFunc(Rc::new(move |n,k,v| { s1.borrow_mut().json_value_base(n,k,v) })))));
		let s1 = b.clone();
		self.jsonfuncs.borrow_mut().insert(format!("{}",KEYWORD_FLOAT),Rc::new(RefCell::new(ExtArgsFunc::JsonFunc(Rc::new(move |n,k,v| { s1.borrow_mut().json_value_base(n,k,v) })))));
		let s1 = b.clone();
		self.jsonfuncs.borrow_mut().insert(format!("{}",KEYWORD_COMMAND),Rc::new(RefCell::new(ExtArgsFunc::JsonFunc(Rc::new(move |n,k,v| { s1.borrow_mut().json_value_error(n,k,v) })))));
		let s1 = b.clone();
		self.jsonfuncs.borrow_mut().insert(format!("{}",KEYWORD_HELP),Rc::new(RefCell::new(ExtArgsFunc::JsonFunc(Rc::new(move |n,k,v| { s1.borrow_mut().json_value_error(n,k,v) })))));
		return;
	}

	fn insert_opt_funcs(&mut self) {
		let b = Arc::new(RefCell::new(self.clone()));
		let s1 = b.clone();
		self.optfuncs.borrow_mut().insert(format!("{}",KEYWORD_STRING),Rc::new(RefCell::new(ExtArgsFunc::ActionFunc(Rc::new(move |n,i,k,p| { s1.borrow_mut().string_action(n,i,k,p) })))));
		let s1 = b.clone();
		self.optfuncs.borrow_mut().insert(format!("{}",KEYWORD_BOOL),Rc::new(RefCell::new(ExtArgsFunc::ActionFunc(Rc::new(move |n,i,k,p| { s1.borrow_mut().bool_action(n,i,k,p) })))));
		let s1 = b.clone();
		self.optfuncs.borrow_mut().insert(format!("{}",KEYWORD_INT),Rc::new(RefCell::new(ExtArgsFunc::ActionFunc(Rc::new(move |n,i,k,p| { s1.borrow_mut().int_action(n,i,k,p) })))));
		let s1 = b.clone();
		self.optfuncs.borrow_mut().insert(format!("{}",KEYWORD_LIST),Rc::new(RefCell::new(ExtArgsFunc::ActionFunc(Rc::new(move |n,i,k,p| { s1.borrow_mut().append_action(n,i,k,p) })))));
		let s1 = b.clone();
		self.optfuncs.borrow_mut().insert(format!("{}",KEYWORD_COUNT),Rc::new(RefCell::new(ExtArgsFunc::ActionFunc(Rc::new(move |n,i,k,p| { s1.borrow_mut().inc_action(n,i,k,p) })))));
		let s1 = b.clone();
		self.optfuncs.borrow_mut().insert(format!("{}",KEYWORD_HELP),Rc::new(RefCell::new(ExtArgsFunc::ActionFunc(Rc::new(move |n,i,k,p| { s1.borrow_mut().help_action(n,i,k,p) })))));
		let s1 = b.clone();
		self.optfuncs.borrow_mut().insert(format!("{}",KEYWORD_JSONFILE),Rc::new(RefCell::new(ExtArgsFunc::ActionFunc(Rc::new(move |n,i,k,p| { s1.borrow_mut().string_action(n,i,k,p) })))));
		let s1 = b.clone();
		self.optfuncs.borrow_mut().insert(format!("{}",KEYWORD_COMMAND),Rc::new(RefCell::new(ExtArgsFunc::ActionFunc(Rc::new(move |n,i,k,p| { s1.borrow_mut().command_action(n,i,k,p) })))));
		let s1 = b.clone();
		self.optfuncs.borrow_mut().insert(format!("{}",KEYWORD_FLOAT),Rc::new(RefCell::new(ExtArgsFunc::ActionFunc(Rc::new(move |n,i,k,p| { s1.borrow_mut().float_action(n,i,k,p) })))));
		return;
	}

	fn insert_setmap_funcs(&mut self) {
		let b = Arc::new(RefCell::new(self.clone()));
		let s1 = b.clone();
		extargs_log_trace!("setmapfuncs [{}]",SUB_COMMAND_JSON_SET);
		self.setmapfuncs.borrow_mut().insert(SUB_COMMAND_JSON_SET,Rc::new(RefCell::new(ExtArgsFunc::LoadJsonFunc(Rc::new(move |n,s| { s1.borrow().parse_subcommand_json_set(n,s) })))));
		let s1 = b.clone();
		extargs_log_trace!("setmapfuncs [{}]",COMMAND_JSON_SET);
		self.setmapfuncs.borrow_mut().insert(COMMAND_JSON_SET,Rc::new(RefCell::new(ExtArgsFunc::LoadJsonFunc(Rc::new(move |n,s| { s1.borrow().parse_command_json_set(n,s) })))));
		let s1 = b.clone();
		extargs_log_trace!("setmapfuncs [{}]",ENVIRONMENT_SET);
		self.setmapfuncs.borrow_mut().insert(ENVIRONMENT_SET,Rc::new(RefCell::new(ExtArgsFunc::LoadJsonFunc(Rc::new(move |n,s| { s1.borrow().parse_environment_set(n,s) })))));
		let s1 = b.clone();
		extargs_log_trace!("setmapfuncs [{}]",ENV_SUB_COMMAND_JSON_SET);
		self.setmapfuncs.borrow_mut().insert(ENV_SUB_COMMAND_JSON_SET,Rc::new(RefCell::new(ExtArgsFunc::LoadJsonFunc(Rc::new(move |n,s| { s1.borrow().parse_env_subcommand_json_set(n,s) })))));
		let s1 = b.clone();
		extargs_log_trace!("setmapfuncs [{}]",ENV_COMMAND_JSON_SET);
		self.setmapfuncs.borrow_mut().insert(ENV_COMMAND_JSON_SET,Rc::new(RefCell::new(ExtArgsFunc::LoadJsonFunc(Rc::new(move |n,s| { s1.borrow().parse_env_command_json_set(n,s) })))));
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
			//error_handler : "".to_string(),
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
			loadfuncs : Rc::new(RefCell::new(HashMap::new())),
			jsonfuncs : Rc::new(RefCell::new(HashMap::new())),
			optfuncs : Rc::new(RefCell::new(HashMap::new())),
			setmapfuncs :Rc::new(RefCell::new(HashMap::new())),
			outfuncs : ExtArgsMatchFuncMap::new(),
		};
		retv.insert_load_command_funcs();
		retv.insert_json_funcs();
		retv.insert_opt_funcs();
		retv.insert_setmap_funcs();

		Ok(retv)
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

	fn get_load_func(&self, k :&str) -> Option<ExtArgsFunc> {
		let mut retv : Option<ExtArgsFunc> = None;
		match self.loadfuncs.borrow().get(k) {
			Some(f1) => {
				let f2 :&ExtArgsFunc = &f1.borrow();
				retv = Some(f2.clone());
			},
			None => {}
		}
		retv
	}

	fn get_json_func(&self, k :&str) -> Option<ExtArgsFunc> {
		let mut retv : Option<ExtArgsFunc> = None;
		match self.jsonfuncs.borrow().get(k) {
			Some(f1) => {
				let f2 :&ExtArgsFunc = &f1.borrow();
				retv = Some(f2.clone());
			},
			None => {}
		}
		retv
	}


	fn string_action(&mut self,ns :NameSpaceEx,validx :i32,keycls :ExtKeyParse,params :Vec<String>) -> Result<i32,Box<dyn Error>> {
		if validx >= params.len() as i32 {
			new_error!{ParserError,"need args [{}] [{}] [{:?}]", validx, keycls.string(), params}
		}
		extargs_log_trace!("set [{}] [{}]",keycls.opt_dest(),params[validx as usize]);
		let n = format!("{}",keycls.opt_dest());
		let v = format!("{}",params[validx as usize]);
		ns.set_string(&n,v)?;
		Ok(1)
	}

	fn bool_action(&mut self,ns :NameSpaceEx, _validx :i32 , keycls :ExtKeyParse, _params :Vec<String>) -> Result<i32,Box<dyn Error>> {
		let mut b :bool = false;
		match keycls.value() {
			Value::Bool(bv) => {
				if bv {
					b = false;	
				} else {
					b = true;
				}
				
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
		extargs_log_trace!("validx [{}]", validx);
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

	fn load_json_value(&self, ns :NameSpaceEx,prefix :String,vmap :serde_json::Map<String,Value>) -> Result<(),Box<dyn Error>> {
		let mut newprefix :String;
		for (k,v) in vmap.clone() {
			match v {
				Value::Object(ref _o) => {
					newprefix = "".to_string();
					if prefix.len() > 0 {
						newprefix.push_str(&prefix);
						newprefix.push_str("_");
					}
					newprefix.push_str(&k);
					self.load_json_value(ns.clone(),format!("{}",newprefix),_o.clone())?;
				},
				_ => {
					newprefix = "".to_string();
					if prefix.len() > 0 {
						newprefix.push_str(&prefix);
						newprefix.push_str("_");
					}
					newprefix.push_str(&k);
					self.set_json_value_not_defined(ns.clone(),self.maincmd.clone(),&newprefix,v.clone())?;
				},
			}
		}

		Ok(())
	}

	fn read_file(&self, fname :&str) -> Result<String,Box<dyn Error>> {
		let mut content :String = String::new();
		match File::open(fname) {
			Ok(mut f) => {
				match f.read_to_string(&mut content) {
					Ok(_s) => {
						return Ok(content);
					},
					Err(e) => {
						new_error!{ParserError,"read [{}] error[{:?}]", fname,e}
					}
				}
			},
			Err(e) => {
				new_error!{ParserError,"open [{}] error[{:?}]", fname,e}
			}
		}
	}

	fn load_json_file(&self,ns :NameSpaceEx,cmdname :String,jsonfile :String) -> Result<(),Box<dyn Error>> {
		let mut prefix : String = "".to_string();
		if cmdname.len() > 0 {
			prefix.push_str(&cmdname);
		}
		prefix = prefix.replace(".","_");
		extargs_log_trace!("load json file [{}]", jsonfile);
		let jsoncontent = self.read_file(&jsonfile)?;
		let jres = serde_json::from_str(&jsoncontent);
		if jres.is_err() {
			new_error!{ParserError,"parse jsonfile [{}] erorr[{:?}]\n{}", jsonfile,jres,jsoncontent}
		}
		let vobj :Value = jres.unwrap();
		match vobj {
			Value::Object(ref _obj) => {
				return self.load_json_value(ns,prefix,_obj.clone());
			}
			_ => {
				new_error!{ParserError,"[{}] not object\n{}",jsonfile,jsoncontent}
			}
		}
	}

	fn parse_subcommand_json_set(&self, ns :NameSpaceEx,pstate :Option<ParserState>) -> Result<(),Box<dyn Error>> {
		let s = ns.get_string(KEYWORD_SUBNARGS);
		if s.len() > 0 && !self.no_json_option {
			if pstate.is_some() {
				let cmds = pstate.as_ref().unwrap().get_cmd_paths();
				let mut idx :usize = cmds.len();
				while idx >= 2 {
					let mut curcmds :Vec<ParserCompat> = Vec::new();
					let mut i :usize =0;
					while i < idx {
						curcmds.push(cmds[i].clone());
						i += 1;
					}
					let subname = self.format_cmd_from_cmd_array(curcmds);
					let prefix = subname.replace(".","_");
					let jsondest = format!("{}_{}",prefix,self.json_long);
					let jsonfile = ns.get_string(&jsondest);
					if jsonfile.len() > 0 {
						self.load_json_file(ns.clone(),subname,jsonfile)?;
					}
					idx -= 1;
				}				
			}
		}
		Ok(())
	}

	fn parse_command_json_set(&self, ns :NameSpaceEx,_pstate :Option<ParserState>) -> Result<(),Box<dyn Error>> {
		if !self.no_json_option && self.json_long.len() > 0 {
			let jsonfile = ns.get_string(&(self.json_long));
			extargs_log_trace!("jsonfile [{}]",jsonfile);
			if jsonfile.len() > 0 {
				self.load_json_file(ns.clone(),"".to_string(),jsonfile)?;
			}
		}
		Ok(())
	}

	fn set_environment_value_inner(&self,ns:NameSpaceEx,prefix :String,parser :ParserCompat) -> Result<(),Box<dyn Error>> {
		for chld in parser.sub_cmds() {
			self.set_environment_value_inner(ns.clone(),format!("{}",prefix),chld.clone())?;
		}

		for opt in parser.get_cmdopts() {
			if !opt.is_flag() || opt.type_name() == KEYWORD_PREFIX || 
			opt.type_name() == KEYWORD_ARGS || opt.type_name() == KEYWORD_HELP {
				continue;
			}
			let oldopt = opt.opt_dest();
			let mut valstr :String  = "".to_string();
			
			if ns.is_accessed(&oldopt) {
				continue;
			}
			let mut optdest = oldopt.to_uppercase();
			let jsonval :Value;
			optdest = optdest.replace("-","_");
			if !optdest.contains("_") {
				optdest = format!("EXTARGS_{}",optdest);
			}

			match env::var(&optdest) {
				Ok(v) => {
					valstr = v.to_string();
				},
				Err(_e) => {}
			}
			if valstr.len() == 0 {
				continue;
			}

			if opt.type_name() == KEYWORD_STRING || opt.type_name() == KEYWORD_JSONFILE {
				valstr = format!("\"{}\"",valstr);
				let ojson = serde_json::from_str(&valstr);
				if ojson.is_err() {
					new_error!{ParserError,"get [{}] value [{}] parse error {:?}", optdest,valstr,ojson}
				}
				jsonval = ojson.unwrap();
				self.call_json_value(ns.clone(),opt.clone(),jsonval.clone())?;
			} else if opt.type_name()  == KEYWORD_BOOL {
				if valstr.to_uppercase() != "TRUE" {
					valstr = "false".to_string();
				} else {
					valstr = "true".to_string();
				}
				jsonval = serde_json::from_str(&valstr).unwrap();
				self.call_json_value(ns.clone(),opt.clone(),jsonval.clone())?;
			} else if opt.type_name() == KEYWORD_INT || opt.type_name() == KEYWORD_COUNT  {
				let mut base : u32 = 10;
				if valstr.starts_with("0x") || valstr.starts_with("0X") {
					valstr = valstr[2..].to_string();
					base = 16;
				} else if valstr.starts_with("x") || valstr.starts_with("X") {
					valstr = valstr[1..].to_string();
					base = 16;
				}
				match i64::from_str_radix(&valstr,base) {
					Ok(v) => {
						let cparse = format!("{}",v);
						jsonval = serde_json::from_str(&cparse).unwrap();
						self.call_json_value(ns.clone(),opt.clone(),jsonval.clone())?;
					},
					Err(e) => {
						new_error!{ParserError, "parse [{}] error [{:?}]", valstr, e}
					}
				}
			} else if opt.type_name() == KEYWORD_FLOAT {
				match valstr.parse::<f64>() {
					Ok(_v) => {
						jsonval = serde_json::from_str(&valstr).unwrap();
						self.call_json_value(ns.clone(),opt.clone(),jsonval.clone())?;
					},
					Err(e) => {
						new_error!{ParserError,"parse [{}] for float error [{:?}]", valstr, e}
					}
				}
			} else if opt.type_name() == KEYWORD_LIST {
				let ojson = serde_json::from_str(&valstr);
				if ojson.is_err() {
					new_error!{ParserError,"can not parse [{}] [{:?}]", valstr, ojson}
				}
				jsonval = ojson.unwrap();
				self.call_json_value(ns.clone(),opt.clone(), jsonval.clone())?;
			} else {
				new_error!{ParserError,"unknown opt [{}]", opt.string()}
			}
		}
		Ok(())
	}

	fn set_environment_value(&self, ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
		return self.set_environment_value_inner(ns.clone(),"".to_string(),self.maincmd.clone());
	}

	fn parse_environment_set(&self, ns :NameSpaceEx,_pstate : Option<ParserState>) ->  Result<(),Box<dyn Error>> {
		return self.set_environment_value(ns.clone());
	}

	fn parse_env_subcommand_json_set(&self,ns :NameSpaceEx,pstate : Option<ParserState>) -> Result<(),Box<dyn Error>> {
		let s :String;
		s = ns.get_string(KEYWORD_SUBCOMMAND);
		if s.len() > 0 && !self.no_json_option && self.json_long.len() > 0 {
			if pstate.is_none() {
				new_error!{ParserError,"not set arg_state yet"}
			}
			let cmds = pstate.as_ref().unwrap().get_cmd_paths();
			let mut idx :usize = cmds.len();
			while idx >= 2 {
				let mut curcmds :Vec<ParserCompat> = Vec::new();
				let mut i :usize = 0;
				while i < idx {
					curcmds.push(cmds[i].clone());
					i += 1;
				}
				let subname = self.format_cmd_from_cmd_array(curcmds);
				let mut prefix : String = subname.replace(".","_");
				prefix = format!("{}_{}",self.json_long, prefix);
				let jsondest = prefix.to_uppercase();
				let mut jsonfile :String = "".to_string();

				match env::var(&jsondest) {
					Ok(v) => {
						jsonfile = v.to_string();
					},
					Err(_e) => {}
				}

				if jsonfile.len() > 0 {
					self.load_json_file(ns.clone(),subname,jsonfile)?;
				}
				idx -= 1;
			}
		}
		Ok(())
	}

	fn parse_env_command_json_set(&self, ns :NameSpaceEx,_pstate :Option<ParserState>)  -> Result<(),Box<dyn Error>> {
		if !self.no_json_option && self.json_long.len() > 0 {
			let mut jsonenv :String = format!("EXTARGSPARSE_{}",self.json_long);
			jsonenv = jsonenv.replace("-","_");
			jsonenv = jsonenv.replace(".","_");
			jsonenv = jsonenv.to_uppercase();
			let mut jsonfile :String = "".to_string();
			match env::var(&jsonenv) {
				Ok(v) => {
					jsonfile = v.to_string();
				},
				Err(_e) => {}
			}
			if jsonfile.len() > 0 {
				self.load_json_file(ns.clone(), "".to_string(),jsonfile)?;
			}
		}
		Ok(())
	}

	fn json_value_base(&self,ns :NameSpaceEx,opt :ExtKeyParse, val :Value) -> Result<(),Box<dyn Error>> {
		let mut idx :i32;
		match val {
			Value::String(ref _s) => {
				if opt.type_name() != KEYWORD_STRING && opt.type_name() != KEYWORD_JSONFILE {
					new_error!{ParserError, "[{}] [{}] not for [{:?}] set", opt.type_name(), opt.opt_dest(), val.clone()}
				}
				extargs_log_trace!("set [{}] [{:?}]", opt.opt_dest(), val);
				ns.set_value(&(opt.opt_dest()),val.clone());
			},
			Value::Object(ref _o) => {
				new_error!{ParserError,"could not set [{}] for object [{:?}]", opt.opt_dest(),val}
			},
			Value::Array(ref a) => {
				let mut narr :Vec<String> = Vec::new();
				if opt.type_name() != KEYWORD_LIST {
					new_error!{ParserError,"[{}] not for list [{:?}]", opt.opt_dest(),val}
				}

				idx = 0;
				for s in a.iter() {
					match s {
						Value::String(s) => {
							narr.push(format!("{}",s));
						},
						_ => {
							new_error!{ParserError,"at [{}] not string  [{:?}]", idx, s}
						}
					}
					idx += 1;
				}
				ns.set_value(&(opt.opt_dest()),val.clone());
			},
			Value::Bool(_b) => {
				if opt.type_name() != KEYWORD_BOOL {
					new_error!{ParserError,"[{}] not for [{:?}] set", opt.opt_dest(),val}
				}
				ns.set_value(&(opt.opt_dest()),val.clone());
			},
			Value::Number(ref n) =>  {
				if opt.type_name() == KEYWORD_INT || opt.type_name() == KEYWORD_COUNT {
					if n.is_i64() || n.is_u64() {
						ns.set_value(&(opt.opt_dest()),val.clone());
					} else {
						new_error!{ParserError,"[{}] not for [{:?}] set", opt.opt_dest(),val}
					}
				} else if opt.type_name() == KEYWORD_FLOAT {
					if n.is_f64() {
						ns.set_value(&(opt.opt_dest()), val.clone());
					} else {
						new_error!{ParserError,"[{}] not for [{:?}] set", opt.opt_dest(),val}
					}
				} else {
					new_error!{ParserError,"[{}] not for [{:?}] set", opt.opt_dest(),val}
				}
			},
			Value::Null => {
				if opt.type_name() == KEYWORD_JSONFILE || opt.type_name() == KEYWORD_STRING {
					ns.set_string(&(opt.opt_dest()), "".to_string())?;
				} else {
					new_error!{ParserError,"[{}] not for [{:?}] set", opt.opt_dest(),val}	
				}
			}
		}
		Ok(())
	}

	fn json_value_error(&self,_ns :NameSpaceEx,opt :ExtKeyParse, _val :Value) -> Result<(),Box<dyn Error>> {
		new_error!{ParserError,"set [{}]", opt.opt_dest()}
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
			extargs_log_trace!("input [{}] keycls [{}]", parsers[uc].string(), keycls.string());
			parsers[uc].push_cmdopts(keycls);
		} else {
			extargs_log_trace!("input [{}] keycls [{}]", self.maincmd.string(), keycls.string());
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
		extargs_log_trace!("json_long [{}]",self.json_long);
		key1 = format!("{}##json input file to get the value set##",self.json_long);
		prefix = self.format_cmd_from_cmd_array(parsers.clone());
		prefix = prefix.replace(".","_");
		v = Value::Null;
		extargs_log_trace!("prefix [{}]",prefix);
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

	fn call_load_command_map_func(&mut self,prefix :String,keycls :ExtKeyParse, parsers :Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		let fnptr :Option<ExtArgsFunc>;
		extargs_log_trace!("typename [{}]",keycls.type_name());
		fnptr = self.get_load_func(&(keycls.type_name()));
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
		extargs_log_trace!(" ");
		Ok(())
	}

	fn load_commandline(&mut self,vmap :Value) -> Result<(),Box<dyn Error>> {
		if self.ended != 0 {
			new_error!{ParserError,"you have call parse_command_line before call load_commandline"}
		}
		let parsers :Vec<ParserCompat> = Vec::new();
		return self.load_commandline_inner("".to_string(),vmap.clone(),parsers);
	}

	pub (crate) fn load_commandline_string(&mut self, s :&str, fnptrs1 :Option<HashMap<String,ExtArgsParseFunc>>) -> Result<(),Box<dyn Error>> {
		let val :Value;
		let ov = serde_json::from_str(s);
		if ov.is_err() {
			new_error!{ParserError,"parse [{}] error[{:?}]", s,ov}
		}
		val = ov.unwrap();
		if fnptrs1.is_some() {
			let fnptrs = fnptrs1.unwrap();
			for (k,v) in fnptrs.clone().iter() {
				self.outfuncs.insert_map(k,v.clone());
			}
		}
		return self.load_commandline(val);
	}

	#[allow(unused_assignments)]
	fn set_args(&self,ns :NameSpaceEx, cmdpaths :Vec<ParserCompat>,optval :Option<StateOptVal>) -> Result<(),Box<dyn Error>> {
		let ilen :usize ;
		let mut argskeycls :Option<ExtKeyParse> = None;
		extargs_assert!(cmdpaths.len() > 0 , "cmdpaths [0]");
		let cmdname = self.format_cmd_from_cmd_array(cmdpaths.clone());
		let  mut params :Vec<String> = Vec::new();
		ilen = cmdpaths.len() - 1;
		for c in cmdpaths[ilen].clone().get_cmdopts() {
			if c.flag_name() == KEYWORD_DOLLAR_SIGN {
				argskeycls = Some(c.clone());
				break;
			}
		}

		if argskeycls.is_none() {
			new_error!{ParserError, "can not find [{}]", cmdname}
		}
		let argskey = argskeycls.unwrap();

		match optval {
			Some(v) => {
				match v {
					StateOptVal::LeftArgs(_lv) => {
						params = _lv.clone();
					},
					_ => {
						new_error!{ParserError,"StateOptVal {:?}",v.clone()}
					}
				}
			},
			None => {
				new_error!{ParserError,"optval is none"}
			}
		}

		match argskey.get_nargs_v() {
			Nargs::Argtype(_s) => {
				let s  = format!("{}",_s);
				if s == "+" {
					if params.len() == 0 {
						new_error!{ParserError,"[{}] args [{}] < 1", cmdname, params.len()}
					}
				} else if s == "?" {
					if params.len() > 1 {
						new_error!{ParserError,"[{}] args [{}] [{}] > 1", cmdname, params[0],params.len()}
					}
				} else if s != "*" {
					new_error!{ParserError, "[{}] args [{}] not valid",cmdname,s}
				}
			},
			Nargs::Argnum(n) => {
				if params.len() as i32 != n {
					new_error!{ParserError,"[{}] args [{}] != [{}]", cmdname,params.len(), n}
				}
			}
		}

		if cmdname.len() > 0 {
			ns.set_array(KEYWORD_SUBNARGS,params)?;
			ns.set_string(KEYWORD_SUBCOMMAND,cmdname)?;
		} else {
			ns.set_array(KEYWORD_ARGS,params)?;
		}

		Ok(())
	}

	fn get_opt_func(&self, k :&str) -> Option<ExtArgsFunc> {
		let mut retv : Option<ExtArgsFunc> = None;
		match self.optfuncs.borrow().get(k) {
			Some(f1) => {
				let f2 :&ExtArgsFunc = &f1.borrow();
				retv = Some(f2.clone());
			},
			None => {}
		}
		retv
	}


	fn call_opt_method_func(&self,ns :NameSpaceEx,validx :i32 ,keycls :ExtKeyParse,params :Vec<String>) -> Result<i32,Box<dyn Error>> {
		let fnptr = self.get_opt_func(&(keycls.type_name()));
		if fnptr.is_some() {
			let f2 = fnptr.unwrap();
			match f2 {
				ExtArgsFunc::ActionFunc(f) => {
					return f(ns,validx,keycls.clone(),params);
				},
				_ => {
					new_error!{ParserError,"return [{}] not action function", keycls.type_name()}
				}
			}
		} else {
			new_error!{ParserError,"can not found [{}] load command map function", keycls.string()}
		}
	}

	fn call_key_opt_method_func(&self,ns :NameSpaceEx,validx :i32, keycls :ExtKeyParse, params :Vec<String>) -> Result<i32,Box<dyn Error>> {
		let oattr =  keycls.get_keyattr(KEYWORD_ATTR) ;
		if oattr.is_some() {
			let attr = oattr.unwrap();
			let funcname = attr.get_attr("optparse");
			if funcname.len() > 0 {
				extargs_log_trace!("get [{}]",funcname);
				let fo = self.outfuncs.get_action_func(&funcname);
				if fo.is_some() {
					let actfunc = fo.unwrap();
					return actfunc(ns.clone(),validx,keycls.clone(),params.clone());
				}
			}
		}
		new_error!{ParserError,"internal error on [{}]", keycls.string()}
	}

	fn call_opt_method(&self, ns :NameSpaceEx,validx :i32 , keycls :ExtKeyParse, params :Vec<String>) -> Result<i32,Box<dyn Error>> {
		let oattr =  keycls.get_keyattr(KEYWORD_ATTR) ;
		if oattr.is_some() {
			let attr = oattr.unwrap();
			let funcname = attr.get_attr("optparse");
			if funcname.len() > 0 {
				return self.call_key_opt_method_func(ns,validx,keycls,params);
			}
		} 
		return self.call_opt_method_func(ns,validx,keycls,params);
	}

	fn parse_args(&mut self,params :Vec<String>) -> Result<NameSpaceEx,Box<dyn Error>> {
		let pstate = ParserState::new(params.clone(),self.maincmd.clone(),self.options.clone());
		let ns = NameSpaceEx::new();

		loop {
			let (validx,optval,okey) = pstate.step_one()?;
			let step :i32;
			if okey.is_none() {
				let cmdpaths = pstate.get_cmd_paths();
				self.set_args(ns.clone(),cmdpaths,optval)?;
				break;
			} else {
				let keycls = okey.unwrap();
				if keycls.type_name() == KEYWORD_HELP {
					let cmdpaths = pstate.get_cmd_paths();
					let helpcmdname = self.format_cmd_from_cmd_array(cmdpaths);
					let mut helpparams : Vec<String> = Vec::new();
					helpparams.push(format!("{}",helpcmdname));
					step = self.call_opt_method(ns.clone(),validx,keycls.clone(),helpparams)?;
				} else {
					step = self.call_opt_method(ns.clone(),validx,keycls.clone(),params.clone())?;
				}
			}

			pstate.add_parse_args(step)?;
		}

		extargs_log_trace!("set arg_state");
		self.arg_state = Some(pstate.clone());
		Ok(ns)
	}

	fn get_setmap_func(&self, val :i32) -> Option<ExtArgsFunc> {
		let mut retv : Option<ExtArgsFunc> = None;
		match self.setmapfuncs.borrow().get(&val) {
			Some(f1) => {
				let f2 :&ExtArgsFunc = &f1.borrow();
				retv = Some(f2.clone());
			},
			None => {}
		}
		retv
	}

	fn call_parse_setmap_func(&self,idx :i32,ns:NameSpaceEx,pstate :Option<ParserState>) -> Result<(),Box<dyn Error>> {
		let fnptr = self.get_setmap_func(idx);
		if fnptr.is_some() {
			let f2 = fnptr.unwrap();
			match f2 {
				ExtArgsFunc::LoadJsonFunc(f) => {
					return f(ns,pstate);
				},
				_ => {
					new_error!{ParserError,"return [{}] not LoadJsonFunc", idx}
				}
			}
		} else {
			new_error!{ParserError,"can not found [{}] load json  function", idx}
		}
	}


	fn call_json_bind_map(&self,ns :NameSpaceEx,keycls :ExtKeyParse, val :Value) -> Result<(),Box<dyn Error>> {
		let fnptr :Option<ExtArgsFunc>;
		let typename = keycls.type_name();
		fnptr = self.get_json_func(&(typename));
		if fnptr.is_some() {
			let f2 = fnptr.unwrap();
			match f2 {
				ExtArgsFunc::JsonFunc(f) => {
					return f(ns.clone(),keycls.clone(),val.clone());
				},
				_ => {
					new_error!{ParserError,"return [{}] not load function", typename}
				}
			}
		} else {
			new_error!{ParserError,"can not found [{}] load command map function", typename}
		}		
	}

	fn call_json_value(&self,ns :NameSpaceEx, keycls :ExtKeyParse,val :Value) -> Result<(),Box<dyn Error>> {
		let oattr =  keycls.get_keyattr(KEYWORD_ATTR) ;
		if oattr.is_some() {
			let attr = oattr.unwrap();
			let funcname = attr.get_attr("jsonfunc");
			if funcname.len() > 0 {
				let fo = self.outfuncs.get_json_func(&funcname);
				if fo.is_some() {
					let jsonfunc = fo.unwrap();
					return jsonfunc(ns.clone(),keycls.clone(),val.clone());
				}
			}
		}
		return self.call_json_bind_map(ns.clone(),keycls.clone(),val.clone());
	}

	fn set_json_value_not_defined(&self,ns :NameSpaceEx,parser :ParserCompat,dest :&str,val :Value) -> Result<(),Box<dyn Error>> {
		for c in parser.sub_cmds() {
			self.set_json_value_not_defined(ns.clone(),c,dest,val.clone())?;
		}

		for opt in parser.get_cmdopts() {
			if opt.is_flag() && opt.flag_name() != KEYWORD_PREFIX && opt.type_name() != KEYWORD_ARGS && 
			opt.type_name() != KEYWORD_HELP {
				if opt.opt_dest() == dest && !ns.is_accessed(dest) {
					self.call_json_value(ns.clone(),opt.clone(),val.clone())?;
				}
			}
		}
		Ok(())
	}

	fn set_parser_default_value(&self, ns:NameSpaceEx, parser :ParserCompat) -> Result<(),Box<dyn Error>> {
		for c in parser.sub_cmds() {
			self.set_parser_default_value(ns.clone(),c.clone())?;
		}

		for opt in parser.get_cmdopts() {
			if opt.is_flag() && opt.type_name() != KEYWORD_PREFIX && 
			opt.type_name() != KEYWORD_HELP && opt.type_name() != KEYWORD_ARGS {
				self.set_json_value_not_defined(ns.clone(),parser.clone(),&(opt.opt_dest()),opt.value())?;
			}
		}
		Ok(())
	}

	fn set_default_value(&self,ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
		return self.set_parser_default_value(ns, self.maincmd.clone());
	}

	fn set_struct_part_for_single(&self,ns:NameSpaceEx, ostruct :Arc<RefCell<dyn ArgSetImpl>>,parser :ParserCompat,parsers :Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		let name :String;
		name = self.format_cmd_from_cmd_array(parsers.clone());
		extargs_log_trace!("name [{}]",name);
		for opt in parser.get_cmdopts() {
			let mut curname :String = "".to_string();
			extargs_log_trace!("[{}] opt [{}]",name,opt.string());
			if opt.is_flag() && opt.type_name() != KEYWORD_HELP && opt.type_name() != KEYWORD_JSONFILE && opt.type_name() != KEYWORD_ARGS {
				if name.len() > 0 {
					curname.push_str(&format!("{}_",name));
				}
				curname.push_str(&opt.flag_name());
				extargs_log_trace!("set [{}][{}]",name, curname);
				ostruct.borrow_mut().set_value("",&curname,ns.clone())?;	
			}
		}
		Ok(())
	}

	fn set_struct_part_inner(&self, ns :NameSpaceEx,ostruct :Arc<RefCell<dyn ArgSetImpl>> ,parsers :Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		let mut curparsers :Vec<ParserCompat>;
		let curparser :ParserCompat;
		let ilen :usize;
		if parsers.len() > 0 {
			curparsers = parsers.clone();
		} else {
			curparsers = Vec::new();
			curparsers.push(self.maincmd.clone());
		}
		ilen = curparsers.len() - 1;
		curparser = curparsers[ilen].clone();
		self.set_struct_part_for_single(ns.clone(),ostruct.clone(), curparser.clone(), curparsers.clone())?;
		for parser in curparser.sub_cmds() {
			let mut nparsers :Vec<ParserCompat> = curparsers.clone();
			nparsers.push(parser);
			self.set_struct_part_inner(ns.clone(),ostruct.clone(),nparsers)?;
		}
		Ok(())
	}

	fn set_struct_part(&self,ns :NameSpaceEx,ostruct1 :Option<Arc<RefCell<dyn ArgSetImpl>>>) -> Result<(),Box<dyn Error>> {
		if self.arg_state.is_none() {
			new_error!{ParserError,"not parse args yet"}
		}

		if ostruct1.is_none() {
			extargs_log_trace!("is_none");
			return Ok(());
		}
		extargs_log_trace!("not none");
		let ostruct = ostruct1.unwrap();
		let mut parsers :Vec<ParserCompat> = Vec::new();
		self.set_struct_part_inner(ns.clone(),ostruct.clone(),parsers.clone())?;
		let argstate = self.arg_state.as_ref().unwrap().clone();
		parsers = argstate.get_cmd_paths();
		let mut curparsers :Vec<ParserCompat> = Vec::new();
		let mut idx :usize = 0;
		while idx < parsers.len() {
			extargs_log_trace!("[{}] parser [{}]",idx,parsers[idx].string());
			curparsers.push(parsers[idx].clone());			
			self.set_struct_part_for_single(ns.clone(),ostruct.clone(),curparsers[idx].clone(),curparsers.clone())?;
			idx += 1;
		}
		Ok(())
	}

	fn call_back_func(&self, funcname :&str,ns :NameSpaceEx,ostruct :Option<Arc<RefCell<dyn ArgSetImpl>>>, context :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
		let fo = self.outfuncs.get_callback_func(funcname);
		if fo.is_some() {
			let callbackfunc = fo.unwrap();
			return callbackfunc(ns.clone(),ostruct,context);
		}
		new_error!{ParserError,"can not get callback [{}]",funcname}
	}

	fn set_mode_resume(&mut self, setmode :i32) {
		if setmode > 0 {
			let mut ccmode :Vec<String> = Vec::new();
			let mut ii :usize = 0;
			while ii < (self.output_mode.len() - 1) {
				ccmode.push(format!("{}",self.output_mode[ii]));
				ii += 1;
			}
			self.output_mode = ccmode.clone();
		}
		return;
	}

	pub (crate) fn parse_commandline_ex(&mut self,args :Option<Vec<String>>,context :Option<Arc<RefCell<dyn Any>>>, ostruct : Option<Arc<RefCell<dyn ArgSetImpl>>>,mode :Option<String>) -> Result<NameSpaceEx,Box<dyn Error>> {
		let ns :NameSpaceEx;
		let mut setmode :i32 = 0;
		let realargs :Vec<String>;
		let mut ctx :Option<Arc<RefCell<dyn Any>>> = None;
		let mut stx :Option<Arc<RefCell<dyn ArgSetImpl>>> = None;
		let mut stx1 :Option<Arc<RefCell<dyn ArgSetImpl>>> = None;

		if mode.is_some() {
			/*that is */
			let s = mode.unwrap();
			self.output_mode.push(format!("{}",s));
			setmode = 1;
		}
		let c = self.set_commandline_self_args();
		if c.is_err() {
			self.set_mode_resume(setmode);
			return Err(c.err().unwrap());
		}

		if args.is_none() {
			realargs = env::args().collect();
		} else {
			realargs = args.unwrap();
		}

		let c = self.parse_args(realargs.clone());
		if c.is_err() {
			self.set_mode_resume(setmode);
			return Err(c.err().unwrap());
		}
		ns = c.unwrap();

		for idx in self.load_priority.clone() {
			if idx != COMMAND_SET && idx != DEFAULT_SET {
				let pstate :Option<ParserState>;
				if self.arg_state.is_some() {
					extargs_log_trace!("[{}]arg_state set",idx);
				} else {
					extargs_log_trace!("[{}]arg_state not set",idx);
				}
				if self.arg_state.is_some() {
					pstate = Some(self.arg_state.as_ref().unwrap().clone());
				} else {
					pstate = None;
				}
				let c = self.call_parse_setmap_func(idx,ns.clone(),pstate);
				if c.is_err() {
					self.set_mode_resume(setmode);
					return Err(c.err().unwrap());
				}				
			}
		}

		let c = self.set_default_value(ns.clone());
		if c.is_err() {
			self.set_mode_resume(setmode);
			return Err(c.err().unwrap());
		}

		if ostruct.is_some() {
			let cv = ostruct.unwrap();
			stx = Some(cv.clone());
			stx1 = Some(cv.clone());
		}

		if context.is_some() {
			ctx = Some(context.unwrap().clone());
		}

		extargs_log_trace!(" ");
		let c = self.set_struct_part(ns.clone(),stx);
		if c.is_err() {
			self.set_mode_resume(setmode);
			return Err(c.err().unwrap());
		}

		let subcmd = ns.get_string(KEYWORD_SUBCOMMAND);
		if subcmd.len() > 0 {
			let cmds = self.arg_state.as_ref().unwrap().get_cmd_paths();
			if cmds.len() > 0 {
				let ilen :usize = cmds.len() - 1;
				let funcname :String = cmds[ilen].get_keycls().unwrap().func_name();
				extargs_log_trace!("[{}] funcname [{}]", cmds[ilen].get_keycls().unwrap().string(), funcname);
				if funcname.len() > 0 {
					let cclen :usize = self.output_mode.len() - 1;
					let mut valid :i32 = 0;
					if self.output_mode.len() == 0 {
						valid = 1;
					} else if self.output_mode[cclen] == "" {
						valid = 1;
					}

					if valid > 0 {

						let c = self.call_back_func(&funcname,ns.clone(),stx1,ctx);
						if c.is_err() {
							self.set_mode_resume(setmode);
							return Err(c.err().unwrap());
						}
					}
				}
			}
		}
		self.set_mode_resume(setmode);
		return Ok(ns);
	}

	pub (crate) fn parse_commandline(&mut self,params : Option<Vec<String>>,context :Option<Arc<RefCell<dyn Any>>>) -> Result<NameSpaceEx,Box<dyn Error>> {
		return self.parse_commandline_ex(params,context,None,None);
	}

	fn get_sub_commands(&self,name :String,cmdpaths :Vec<ParserCompat>) -> Vec<String> {
		let mut retv :Vec<String> = Vec::new();
		let mut curpaths :Vec<ParserCompat> = cmdpaths.clone();
		if curpaths.len() == 0 {
			curpaths.push(self.maincmd.clone());
		}
		let ilen :usize = curpaths.len() - 1;

		if name.len() == 0 {
			for c in curpaths[ilen].sub_cmds() {
				retv.push(format!("{}",c.cmd_name()));
			}
			retv.sort();
			return retv;
		}

		let sarr : Vec<&str> = name.split(".").collect();
		for c in curpaths[ilen].sub_cmds() {
			if c.cmd_name() == sarr[0] {
				curpaths.push(c.clone());
				return self.get_sub_commands(sarr[1..sarr.len()].join("."),curpaths);
			}
		}

		return retv;
	}

	pub (crate) fn get_sub_commands_ex(&mut self,name :String) -> Result<Vec<String>,Box<dyn Error>> {
		self.set_commandline_self_args()?;
		let cmdpaths :Vec<ParserCompat> = Vec::new();
		let retv = self.get_sub_commands(name,cmdpaths);
		return Ok(retv);
	}

	fn get_cmd_key(&self,cmdname :String,cmdpaths :Vec<ParserCompat>) -> Option<ExtKeyParse> {
		let mut curpaths :Vec<ParserCompat> = cmdpaths.clone();
		let ilen :usize ;
		if curpaths.len() == 0 {
			curpaths.push(self.maincmd.clone());
		}

		ilen = curpaths.len() - 1;
		if cmdname.len() == 0 {
			return curpaths[ilen].get_keycls();
		}

		let sarr :Vec<&str> = cmdname.split(".").collect();
		for c in curpaths[ilen].sub_cmds() {
			if c.cmd_name() == sarr[0] {
				curpaths.push(c.clone());
				return self.get_cmd_key(sarr[1..sarr.len()].join("."), curpaths);
			}
		}
		return None;
	}

	pub (crate) fn get_cmd_key_ex(&mut self,cmdname :String) -> Result<Option<ExtKeyParse>,Box<dyn Error>> {
		self.set_commandline_self_args()?;
		let cmdpaths :Vec<ParserCompat>= Vec::new();
		let retv = self.get_cmd_key(cmdname,cmdpaths);
		return Ok(retv);
	}

	fn get_cmd_opts(&self,cmdname :String,cmdpaths :Vec<ParserCompat>) -> Vec<ExtKeyParse> {
		let mut curpaths :Vec<ParserCompat> = cmdpaths.clone();
		let ilen :usize;
		let mut retv :Vec<ExtKeyParse> = Vec::new();
		if curpaths.len() == 0 {
			curpaths.push(self.maincmd.clone());
		}
		ilen = curpaths.len() - 1;
		if cmdname.len() == 0{
			for opt in curpaths[ilen].get_cmdopts() {
				if opt.is_flag() {
					retv.push(opt.clone());
				}
			}
			retv.sort();
			return retv;
		}

		let sarr :Vec<&str> = cmdname.split(".").collect();
		for c in curpaths[ilen].sub_cmds() {
			if c.cmd_name() == sarr[0] {
				curpaths.push(c.clone());
				return self.get_cmd_opts(sarr[1..sarr.len()].join("."), curpaths);
			}
		}

		return retv;
	}

	pub (crate) fn get_cmd_opts_ex(&mut self,cmdname :String) -> Result<Vec<ExtKeyParse>,Box<dyn Error>> {
		self.set_commandline_self_args()?;
		let cmdpaths :Vec<ParserCompat> = Vec::new();
		let retv = self.get_cmd_opts(cmdname,cmdpaths);
		return Ok(retv);
	}
}

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

	pub fn load_commandline_string(&self,s :&str, fnptrs :Option<HashMap<String,ExtArgsParseFunc>>) -> Result<(),Box<dyn Error>> {
		return self.innerrc.borrow_mut().load_commandline_string(s,fnptrs);
	}

	pub fn parse_commandline_ex(&self,args :Option<Vec<String>>,context :Option<Arc<RefCell<dyn Any>>>, ostruct : Option<Arc<RefCell<dyn ArgSetImpl>>>,mode :Option<String>) -> Result<NameSpaceEx,Box<dyn Error>> {
		return self.innerrc.borrow_mut().parse_commandline_ex(args,context,ostruct,mode);
	}

	pub fn parse_commandline(&self,args :Option<Vec<String>>,context :Option<Arc<RefCell<dyn Any>>>) -> Result<NameSpaceEx,Box<dyn Error>> {
		return self.innerrc.borrow_mut().parse_commandline(args,context);
	}

	pub fn get_sub_commands_ex(&self,name :String) -> Result<Vec<String>,Box<dyn Error>> {
		return self.innerrc.borrow_mut().get_sub_commands_ex(name);
	}

	pub fn get_cmd_key_ex(&self,name :String) -> Result<Option<ExtKeyParse>,Box<dyn Error>> {
		return self.innerrc.borrow_mut().get_cmd_key_ex(name);
	}

	pub fn get_cmd_opts_ex(&self, name :String) -> Result<Vec<ExtKeyParse>, Box<dyn Error>> {
		return self.innerrc.borrow_mut().get_cmd_opts_ex(name);
	}
}