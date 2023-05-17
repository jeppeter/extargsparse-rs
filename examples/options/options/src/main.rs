use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
//use extargsparse_worker::argset::{ArgSetImpl};
//use extargsparse_worker::{extargs_error_class,extargs_new_error};
use extargsparse_worker::options::{ExtArgsOptions,OPT_HELP_LONG,OPT_HELP_SHORT,OPT_LONG_PREFIX,OPT_SHORT_PREFIX};
//use extargsparse_worker::namespace::NameSpaceEx;
//use extargsparse_worker::key::ExtKeyParse;
use extargsparse_worker::funccall::ExtArgsParseFunc;
use extargsparse_worker::parser::ExtArgsParser;


use std::error::Error;
use lazy_static::lazy_static;
//use regex::Regex;
//use std::sync::Arc;
//use std::cell::RefCell;
//use std::any::Any;
use std::collections::HashMap;
//use std::io::{Write};
//use std::fs::{File};





#[extargs_map_function()]
fn main() -> Result<(),Box<dyn Error>> {
    let cmdline = r#"        {
            "verbose|v" : "+",
            "dep" : {
                "list|l" : [],
                "string|s" : "s_var",
                "$" : "+"
            }
        }"#;


    let optstr = format!("{{ \"{}\" : \"usage\",\"{}\"  : \"?\" , \"{}\" : \"++\",\"{}\" : \"+\" }}",OPT_HELP_LONG,OPT_HELP_SHORT,OPT_LONG_PREFIX,OPT_SHORT_PREFIX);
    let options = ExtArgsOptions::new(&optstr)?;
    let parser :ExtArgsParser = ExtArgsParser::new(Some(options.clone()),None)?;
    extargs_load_commandline!(parser,cmdline)?;

    let ns = parser.parse_commandline(None,None)?;
    println!("subcommand={}", ns.get_string("subcommand"));
    println!("verbose={}", ns.get_int("verbose"));
    println!("dep_list={:?}", ns.get_array("dep_list"));
    println!("dep_string={}", ns.get_string("dep_string"));
    println!("subnargs={:?}", ns.get_array("subnargs"));
    println!("args={:?}", ns.get_array("args"));
    Ok(())
}
