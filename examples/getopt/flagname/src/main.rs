use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
//use extargsparse_worker::argset::{ArgSetImpl};
//use extargsparse_worker::{extargs_error_class,extargs_new_error};
//use extargsparse_worker::{extargs_log_trace};
use extargsparse_worker::key::{ExtKeyParse};
//use extargsparse_worker::options::{ExtArgsOptions,OPT_PROG};
//use extargsparse_worker::namespace::NameSpaceEx;
use extargsparse_worker::funccall::ExtArgsParseFunc;
use extargsparse_worker::parser::ExtArgsParser;


use std::error::Error;
use lazy_static::lazy_static;
//use std::sync::Arc;
//use std::cell::RefCell;
//use std::any::Any;
use std::collections::HashMap;





#[extargs_map_function()]
fn main() -> Result<(),Box<dyn Error>> {
    let loads = r#"{
		"verbose|v" : "+",
		"dep" : {
			"cc|c" : ""
		},
		"rdep": {
			"dd|C" : ""
		}
	}"#;
    let parser :ExtArgsParser = ExtArgsParser::new(None,None)?;
    extargs_load_commandline!(parser,loads)?;
    let mut flags :Vec<ExtKeyParse>;
    let flags = parser.get_cmd_opts_ex("")?;
    for flag in flags.iter() {
    	if flag.flag_name() == "verbose" {
    		println!("flagname={}", flag.flag_name());
    	}
    }

    let flags = parser.get_cmd_opts_ex("dep")?;
    for flag in flags.iter() {
    	if flag.flag_name() == "cc" {
    		println!("flagname={}", flag.flag_name());
    	}
    }

    let flags = parser.get_cmd_opts_ex("rdep")?;
    for flag in flags.iter() {
    	if flag.flag_name() == "dd" {
    		println!("flagname={}", flag.flag_name());
    	}
    }

    let flags = parser.get_cmd_opts_ex("rdep")?;
    let mut finded :bool = false;
    for flag in flags.iter() {
    	if flag.flag_name() == "cc" {
    		finded = true;
    		println!("flagname={}", flag.flag_name());
    		break;
    	}
    }

    if !finded {
    	println!("can not found cc for rdep cmd");
    }

/*
output:
flagname=verbose
flagname=cc
flagname=dd
can not found cc for rdep cmd
*/

    Ok(())
}