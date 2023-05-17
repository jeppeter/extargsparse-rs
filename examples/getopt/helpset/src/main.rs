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
		"verbose|v##we used verbose##" : "+",
		"$##this is args help##" : "*",
		"dep##dep help set##" : {
			"cc|c##cc sss##" : "",
			"$##this is dep subnargs help##" : "*"
		},
		"rdep##can not set rdep help##": {
			"dd|C##capital C##" : "",
			"$##this is rdep subnargs help##" : "*"
		}
	}"#;
    let parser :ExtArgsParser = ExtArgsParser::new(None,None)?;
    extargs_load_commandline!(parser,loads)?;
    let mut flag :ExtKeyParse;
    let mut flags :Vec<ExtKeyParse>;
    let ores = parser.get_cmd_key_ex("")?;
    assert!(ores.is_some());
    flag = ores.unwrap();
    println!("main helpinfo:{}", flag.help_info());

    let ores = parser.get_cmd_key_ex("dep")?;
    assert!(ores.is_some());
    flag = ores.unwrap();
    println!("dep helpinfo:{}", flag.help_info());


    let ores = parser.get_cmd_key_ex("rdep")?;
    assert!(ores.is_some());
    flag = ores.unwrap();
    println!("rdep helpinfo:{}", flag.help_info());

    flags = parser.get_cmd_opts_ex("")?;
    for f in flags.iter() {
    	if f.type_name() == "args" {
    		println!("main.args.HelpInfo={}", f.help_info());
    	} else if f.flag_name() == "verbose" {
    		println!("main.verbose.HelpInfo={}",f.help_info());
    	}
    }


    flags = parser.get_cmd_opts_ex("dep")?;
    for f in flags.iter() {
    	if f.type_name() == "args" {
    		println!("dep.subnargs.HelpInfo={}", f.help_info());
    	} else if f.flag_name() == "cc" {
    		println!("dep.cc.HelpInfo={}",f.help_info());
    	}
    }

    flags = parser.get_cmd_opts_ex("rdep")?;
    for f in flags.iter() {
    	if f.type_name() == "args" {
    		println!("rdep.subnargs.HelpInfo={}", f.help_info());
    	} else if f.flag_name() == "dd" {
    		println!("rdep.dd.HelpInfo={}",f.help_info());
    	}
    }

    Ok(())
}
/*
output:
main helpinfo:
dep helpinfo:dep help set
rdep helpinfo:can not set rdep help
main.args.HelpInfo=this is args help
main.verbose.HelpInfo=we used verbose
dep.subnargs.HelpInfo=this is dep subnargs help
dep.cc.HelpInfo=cc sss
rdep.subnargs.HelpInfo=this is rdep subnargs help
rdep.dd.HelpInfo=capital C
*/
