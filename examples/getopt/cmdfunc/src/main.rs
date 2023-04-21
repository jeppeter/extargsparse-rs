use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
//use extargsparse_worker::argset::{ArgSetImpl};
//use extargsparse_worker::{extargs_error_class,extargs_new_error};
//use extargsparse_worker::{extargs_log_trace};
//use extargsparse_worker::key::{ExtKeyParse,KEYWORD_ATTR};
use extargsparse_worker::key::{ExtKeyParse};
//use extargsparse_worker::options::ExtArgsOptions;
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
    let parser :ExtArgsParser = ExtArgsParser::new(None,None)?;
    extargs_load_commandline!(parser,loads)?;
    let mut flag :ExtKeyParse;
    let mut funcname :String;

    let ores = parser.get_cmd_key_ex("")?;
    assert!(ores.is_some());
    flag = ores.unwrap();
    funcname = flag.func_name();
    println!("main function:{}", funcname);
    let ores = parser.get_cmd_key_ex("dep")?;
    assert!(ores.is_some());
    flag = ores.unwrap();
    funcname = flag.func_name();
    println!("dep function:{}", funcname);

    let ores = parser.get_cmd_key_ex("rdep")?;
    assert!(ores.is_some());
    flag = ores.unwrap();
    funcname = flag.func_name();
    println!("rdep function:{}", funcname);

    Ok(())
}