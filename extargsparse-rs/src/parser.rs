
use super::options;
use super::options::{ExtArgsOptions,OPT_HELP_HANDLER,OPT_LONG_PREFIX,OPT_SHORT_PREFIX,OPT_NO_HELP_OPTION,OPT_NO_JSON_OPTION,OPT_HELP_LONG,OPT_HELP_SHORT,OPT_JSON_LONG,OPT_CMD_PREFIX_ADDED};
use super::parser_compat;
use super::parser_compat::{ParserCompat};
use super::parser_state::{ParserState};
use super::key::{ExtKeyParse,KEYWORD_DOLLAR_SIGN,KEYWORD_HELP,KEYWORD_JSONFILE,KEYWORD_STRING,KEYWORD_INT,KEYWORD_FLOAT,KEYWORD_LIST,KEYWORD_BOOL,KEYWORD_COUNT,KEYWORD_ARGS};
use super::const_value::{COMMAND_SET,SUB_COMMAND_JSON_SET,COMMAND_JSON_SET,ENVIRONMENT_SET,ENV_SUB_COMMAND_JSON_SET,ENV_COMMAND_JSON_SET,DEFAULT_SET};
use super::util::{check_in_array,format_array_string};
use lazy_static::lazy_static;

use std::fmt;
use std::error::Error;
use std::boxed::Box;
use serde_json::Value;
//use super::logger::{extargs_debug_out};
use super::{extargs_assert};


use super::{error_class,new_error};


error_class!{ParserError}

#[allow(dead_code)]
#[derive(Clone)]
pub struct ExtArgsParser {
	options :Option<ExtArgsOptions>,
	maincmd :Option<ParserCompat>,
	arg_state :Option<ParserState>,
	error_handler :String,
	help_handler :String,
	output_mode :Vec<String>,
	ended : i32,
	long_prefix :String,
	short_prefix :String,
	no_help_option : bool,
	no_json_option : bool,
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


pub fn new(opt :Option<ExtArgsOptions>,priority :Option<Vec<i32>>) -> Result<ExtArgsParser,Box<dyn Error>> {
	let mut retv :ExtArgsParser = ExtArgsParser {
		options : None,
		maincmd : None,
		arg_state : None,
		error_handler : "".to_string(),
		help_handler : "".to_string(),
		output_mode : Vec::new(),
		ended : 0,
		long_prefix : "".to_string(),
		short_prefix : "".to_string(),
		no_help_option : false,
		no_json_option : false,
		help_long : "".to_string(),
		help_short : "".to_string(),
		json_long : "".to_string(),
		cmd_prefix_added : true,
		load_priority : Vec::new(),
	};
	let mut setopt = options::new("{}")?.clone();
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

	retv.options = Some(setopt.clone());
	retv.maincmd = Some(parser_compat::new(None,Some(setopt.clone())));
	retv.arg_state = None;
	retv.help_handler = format!("{}",setopt.get_string(OPT_HELP_HANDLER));
	retv.output_mode = Vec::new();
	retv.ended = 0;
	retv.long_prefix = setopt.get_string(OPT_LONG_PREFIX);
	retv.short_prefix = setopt.get_string(OPT_SHORT_PREFIX);
	retv.no_help_option = setopt.get_bool(OPT_NO_HELP_OPTION);
	retv.no_json_option = setopt.get_bool(OPT_NO_JSON_OPTION);
	retv.help_long = setopt.get_string(OPT_HELP_LONG);
	retv.help_short = setopt.get_string(OPT_HELP_SHORT);
	retv.json_long = setopt.get_string(OPT_JSON_LONG);
	retv.cmd_prefix_added = setopt.get_bool(OPT_CMD_PREFIX_ADDED);
	retv.load_priority = setpriority.clone();
	

	Ok(retv)
}

#[allow(dead_code)]
impl ExtArgsParser {
	fn check_flag_insert(&mut self,keycls :ExtKeyParse,parsers :&mut Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		let lastparser :ParserCompat;
		let mut parserclone :i32 = 0;
		if parsers.len() > 0 {
			lastparser = parsers[parsers.len() - 1].clone();
			parserclone = 1;
		} else {
			lastparser = self.maincmd.as_ref().unwrap().clone();
		}

		for opt in lastparser.cmdopts.iter() {
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
			parsers[uc].cmdopts.push(keycls);
		} else {
			if self.maincmd.is_some() {
				self.maincmd.as_mut().unwrap().cmdopts.push(keycls);
			} else {
				new_error!{ParserError,"no maincmd set"}
			}
		}

		Ok(())
	}

	fn format_cmd_from_cmd_array(&self,parsers :Vec<ParserCompat>) -> String {
		let mut rets :String = "".to_string();
		for v in parsers.iter() {
			if rets.len() > 0 {
				rets.push_str(".");
			}
			rets.push_str(&v.cmdname);
		}
		rets
	}

	fn load_commandline_json_file(&mut self,keycls :ExtKeyParse,parsers :&mut Vec<ParserCompat>) -> Result<(), Box<dyn Error>> {
		return self.check_flag_insert(keycls,parsers);
	}

	fn load_commandline_json_added(&mut self,parsers :&mut Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
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

	fn load_commandline_help(&mut self, keycls :ExtKeyParse, parsers :&mut Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		return self.check_flag_insert(keycls,parsers);
	}

	fn load_commandline_help_added(&mut self,parsers :&mut Vec<ParserCompat>) -> Result<(), Box<dyn Error>> {
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

	fn load_commandline_base(&mut self, _prefix :String, keycls :ExtKeyParse, parsers :&mut Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		if keycls.is_flag() && keycls.flag_name() != KEYWORD_DOLLAR_SIGN && check_in_array(PARSER_RESERVE_ARGS.clone(),&(keycls.flag_name())) {
			new_error!{ParserError,"{} in the {}", keycls.flag_name(), format_array_string(PARSER_RESERVE_ARGS.clone())}
		}
		return self.check_flag_insert(keycls,parsers);
	}

	fn load_commandline_args(&mut self, _prefix :String, keycls :ExtKeyParse, parsers :&mut Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		return self.check_flag_insert(keycls,parsers);
	}

	fn  find_subparser_inner(&self,_name :&str, _oparser :Option<ParserCompat>) -> Option<ParserCompat> {
		let retv :Option<ParserCompat> = None;
		retv
	}

	fn get_subparser_inner(&self,_keycls :ExtKeyParse,_parsers :&mut Vec<ParserCompat>) -> Option<ParserCompat> {
		let retv :Option<ParserCompat> = None;
		retv
	}

	fn load_commandline_subparser(&mut self,_prefix :String, _keycls :ExtKeyParse, _parsers :&mut Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		Ok(())
	}

	fn call_load_command_map_func(&mut self,prefix :String,keycls :ExtKeyParse, parsers :&mut Vec<ParserCompat>) -> Result<(),Box<dyn Error>> {
		if prefix == KEYWORD_STRING || prefix == KEYWORD_INT || prefix == KEYWORD_FLOAT ||
			prefix == KEYWORD_LIST || prefix == KEYWORD_BOOL || prefix == KEYWORD_COUNT ||
			prefix == KEYWORD_HELP || prefix == KEYWORD_JSONFILE {
				return self.load_commandline_base(prefix,keycls,parsers);
		}  else if prefix == KEYWORD_ARGS {
			return self.load_commandline_args(prefix,keycls,parsers);
		}
		new_error!{ParserError,"not {} prefix parse",prefix}
	}
}