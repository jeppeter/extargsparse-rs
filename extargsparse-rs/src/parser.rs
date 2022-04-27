
use super::options::{ExtArgsOptions,OPT_HELP_HANDLER,OPT_LONG_PREFIX,OPT_SHORT_PREFIX,OPT_NO_HELP_OPTION,OPT_NO_JSON_OPTION,OPT_HELP_LONG,OPT_HELP_SHORT,OPT_JSON_LONG,OPT_CMD_PREFIX_ADDED, OPT_FLAG_NO_CHANGE};
use super::parser_compat::{ParserCompat};
use super::parser_state::{ParserState};
use super::key::{ExtKeyParse,KEYWORD_DOLLAR_SIGN,KEYWORD_HELP,KEYWORD_JSONFILE,KEYWORD_STRING,KEYWORD_INT,KEYWORD_FLOAT,KEYWORD_LIST,KEYWORD_BOOL,KEYWORD_COUNT,KEYWORD_ARGS,KEYWORD_COMMAND,KEYWORD_PREFIX};
use super::const_value::{COMMAND_SET,SUB_COMMAND_JSON_SET,COMMAND_JSON_SET,ENVIRONMENT_SET,ENV_SUB_COMMAND_JSON_SET,ENV_COMMAND_JSON_SET,DEFAULT_SET};
use super::util::{check_in_array,format_array_string};
use lazy_static::lazy_static;

use std::fmt;
use std::error::Error;
use std::boxed::Box;
use serde_json::Value;
use std::rc::Rc;
use std::cell::RefCell;
use super::logger::{extargs_debug_out};
use super::{extargs_assert,extargs_log_info};


use super::{error_class,new_error};


error_class!{ParserError}

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
		let retv :InnerExtArgsParser = InnerExtArgsParser {
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
		};
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

	fn call_load_command_map_func(&mut self,prefix :String,keycls :ExtKeyParse, parsers :Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		if prefix == KEYWORD_STRING || prefix == KEYWORD_INT || prefix == KEYWORD_FLOAT ||
		prefix == KEYWORD_LIST || prefix == KEYWORD_BOOL || prefix == KEYWORD_COUNT ||
		prefix == KEYWORD_HELP || prefix == KEYWORD_JSONFILE {
			return self.load_commandline_base(prefix,keycls,parsers);
		}  else if prefix == KEYWORD_ARGS {
			return self.load_commandline_args(prefix,keycls,parsers);
		} else if prefix == KEYWORD_COMMAND {
			return self.load_command_subparser(prefix,keycls,parsers);
		} else if prefix == KEYWORD_PREFIX {
			return self.load_command_prefix(prefix,keycls,parsers);
		}
		new_error!{ParserError,"not {} prefix parse",prefix}
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