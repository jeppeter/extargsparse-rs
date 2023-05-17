//use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
//use extargsparse_worker::argset::{ArgSetImpl};
//use extargsparse_worker::{extargs_error_class,extargs_new_error};
use extargsparse_worker::options::{ExtArgsOptions,OPT_SCREEN_WIDTH};
//use extargsparse_worker::namespace::NameSpaceEx;
//use extargsparse_worker::key::ExtKeyParse;
//use extargsparse_worker::funccall::ExtArgsParseFunc;
//use extargsparse_worker::parser::ExtArgsParser;


use std::error::Error;
//use lazy_static::lazy_static;
//use regex::Regex;
//use std::sync::Arc;
//use std::cell::RefCell;
//use std::any::Any;
//use std::collections::HashMap;
//use std::io::{Write};
//use std::fs::{File};





fn main() -> Result<(),Box<dyn Error>> {


    let optstr = format!("{{ \"{}\" : 60}}",OPT_SCREEN_WIDTH);
    let options = ExtArgsOptions::new(&optstr)?;
    println!("screenwidth={}", options.get_int(OPT_SCREEN_WIDTH));
    let optstr = format!("{{ \"{}\" : 100}}",OPT_SCREEN_WIDTH);
    let options = ExtArgsOptions::new(&optstr)?;
    println!("screenwidth={}", options.get_int(OPT_SCREEN_WIDTH));
    Ok(())
}