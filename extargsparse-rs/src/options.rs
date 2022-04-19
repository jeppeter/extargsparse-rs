use std::collections::HashMap;
use serde_json::{Value};
use lazy_static::lazy_static;
use log::{error, info, trace};
use super::logger::{extargs_proc_log_init};


const OPT_PROG :&str = "prog";
const OPT_USAGE :&str = "usage";
const OPT_DESCRIPTION :&str= "description";
const OPT_EPILOG :&str = "epilog";
const OPT_VERSION :&str = "version";
const OPT_ERROR_HANDLER :&str = "errorhandler";
const OPT_HELP_HANDLER :&str = "helphandler";
const OPT_LONG_PREFIX :&str = "longprefix";
const OPT_SHORT_PREFIX :&str = "shortprefix";


lazy_static! {
	static ref EXT_OPTIONS_LOG_LEVEL : i32 = {
		extargs_proc_log_init("EXT_OPTIONS")
	};
}

#[macro_export]
macro_rules! ext_opts_call_error {
	($($arg:tt)+) => {
		if *EXT_OPTIONS_LOG_LEVEL >= 0 {
			error!($($arg)+);
		}
	}
}

#[macro_export]
macro_rules! ext_opts_call_info {
	($($arg:tt)+) => {
		if *EXT_OPTIONS_LOG_LEVEL >= 20 {
			info!($($arg)+);
		}
	}
}

#[macro_export]
macro_rules! ext_opts_call_trace {
	($($arg:tt)+) => {
		if *EXT_OPTIONS_LOG_LEVEL >= 40 {
			trace!($($arg)+);
		}
	}
}


pub struct ExtArgsOptions {
	values :HashMap<String,Value>,
}

macro_rules! OPT_DEFAULT_S {
	() => {
		format!(r#"
{{
	"{}" : "",
	"{}" : "",
	"{}" : "",
	"{}" : "",
	"{}" : "0.0.1",
	"{}" : "exit",
	"{}" : null,
	"{}" : "--",
	"{}" : "-",
}}
"#,OPT_PROG,OPT_USAGE,OPT_DESCRIPTION,OPT_EPILOG,OPT_VERSION,
	OPT_ERROR_HANDLER,OPT_HELP_HANDLER,OPT_LONG_PREFIX,OPT_SHORT_PREFIX)
	}
}

lazy_static! {
	static ref OPT_KEYS :Vec<String>= {
		let mut retv :Vec<String>= Vec::new();
		retv.push(OPT_PROG.to_string());
		retv.push(OPT_USAGE.to_string());
		retv.push(OPT_DESCRIPTION.to_string());
		retv
	};
	static ref OPT_DEFAULT_VALUE :HashMap<String,Value> = {
		let defs = OPT_DEFAULT_S!();
		let tmpv :HashMap<String,Value>;
		ext_opts_call_info!("parse opt default\n{}",defs);
		match serde_json::from_str(&defs) {
			Ok(d) => {
				tmpv = d;
			},
			Err(e) => {
				ext_opts_call_error!("can not parse error[{}]\n{}",e,defs);
				panic!("exit");
			}
		}
		tmpv	
	};
}