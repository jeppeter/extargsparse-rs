use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
use extargsparse_worker::key::{ExtKeyParse};
use extargsparse_worker::funccall::ExtArgsParseFunc;
use extargsparse_worker::parser::ExtArgsParser;


use std::error::Error;
use lazy_static::lazy_static;
use std::collections::HashMap;


#[extargs_map_function()]
fn main() -> Result<(),Box<dyn Error>> {
    let loads = r#"{
		"dep" : {

		},
		"rdep": {

		}
	}"#;
    let parser :ExtArgsParser = ExtArgsParser::new(None,None)?;
    extargs_load_commandline!(parser,loads)?;
    let mut flag :ExtKeyParse;
    let ores = parser.get_cmd_key_ex("")?;
    assert!(ores.is_some());
    flag = ores.unwrap();
    println!("cmdname={}", flag.cmd_name());

    let ores = parser.get_cmd_key_ex("dep")?;
    assert!(ores.is_some());
    flag = ores.unwrap();
    println!("cmdname={}", flag.cmd_name());

    let ores = parser.get_cmd_key_ex("rdep")?;
    assert!(ores.is_some());
    flag = ores.unwrap();
    println!("cmdname={}", flag.cmd_name());

    Ok(())
}
/*
output:
cmdname=main
cmdname=dep
cmdname=rdep
*/
