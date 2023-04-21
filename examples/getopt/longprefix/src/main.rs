use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
//use extargsparse_worker::argset::{ArgSetImpl};
//use extargsparse_worker::{extargs_error_class,extargs_new_error};
//use extargsparse_worker::{extargs_log_trace};
use extargsparse_worker::key::{ExtKeyParse,KEYWORD_COUNT};
use extargsparse_worker::options::{ExtArgsOptions,OPT_LONG_PREFIX,OPT_SHORT_PREFIX};
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
		"dep<dep_handler>" : {
			"cc|c" : ""
		},
		"rdep<rdep_handler>": {
			"dd|C" : ""
		}
	}"#;
    let mut parser :ExtArgsParser = ExtArgsParser::new(None,None)?;
    extargs_load_commandline!(parser,loads)?;
    let mut flags :Vec<ExtKeyParse>;

    flags = parser.get_cmd_opts_ex("")?;
    for f in flags.iter() {
    	if f.flag_name() == "verbose" && f.type_name() == KEYWORD_COUNT {
    		println!("longprefix={}", f.long_prefix());
    		println!("longopt={}",f.long_opt());
    		println!("shortopt={}", f.short_opt());
    	}
    }

    let confstr = format!("{{ \"{}\" : \"++\", \"{}\" : \"+\"}}",
    		OPT_LONG_PREFIX,OPT_SHORT_PREFIX);
    let options = ExtArgsOptions::new(&confstr)?;
    parser = ExtArgsParser::new(Some(options.clone()),None)?;
    extargs_load_commandline!(parser,loads)?;

    flags = parser.get_cmd_opts_ex("")?;
    for f in flags.iter() {
    	if f.flag_name() == "verbose" && f.type_name() == KEYWORD_COUNT {
    		println!("longprefix={}", f.long_prefix());
    		println!("longopt={}",f.long_opt());
    		println!("shortopt={}", f.short_opt());
    	}
    }
/*
output:
longprefix=--
longopt=--verbose
shortopt=-v
longprefix=++
longopt=++verbose
shortopt=+v
*/
    Ok(())
}