
use super::options;
use super::options::{ExtArgsOptions,OPT_HELP_HANDLER,OPT_LONG_PREFIX,OPT_SHORT_PREFIX,OPT_NO_HELP_OPTION,OPT_NO_JSON_OPTION,OPT_HELP_LONG,OPT_HELP_SHORT,OPT_JSON_LONG,OPT_CMD_PREFIX_ADDED};
use super::parser_compat;
use super::parser_compat::{ParserCompat};
use super::parser_state::{ParserState,StateOptVal};
use super::const_value::{COMMAND_SET,SUB_COMMAND_JSON_SET,COMMAND_JSON_SET,ENVIRONMENT_SET,ENV_SUB_COMMAND_JSON_SET,ENV_COMMAND_JSON_SET,DEFAULT_SET};
use lazy_static::lazy_static;
use std::fmt;
use std::error::Error;

use super::{error_class,new_error,debug_output,error_output};


error_class!{ParserError}

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