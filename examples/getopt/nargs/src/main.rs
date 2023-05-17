use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
use extargsparse_worker::key::{ExtKeyParse,KEYWORD_ARGS};
use extargsparse_worker::funccall::ExtArgsParseFunc;
use extargsparse_worker::parser::ExtArgsParser;


use std::error::Error;
use lazy_static::lazy_static;
use std::collections::HashMap;


#[extargs_map_function()]
fn main() -> Result<(),Box<dyn Error>> {
    let loads = r#"{
		"verbose|v" : "+",
		"dep<dep_handler>" : {
			"cc|c" : "",
			"$" : "+"
		},
		"rdep<rdep_handler>": {
			"dd|C" : "",
			"$" : "?"
		},
		"$port" : {
			"nargs" : 1,
			"type" : "int",
			"value" : 9000
		}
	}"#;
    let parser :ExtArgsParser = ExtArgsParser::new(None,None)?;
    extargs_load_commandline!(parser,loads)?;
    let mut flags :Vec<ExtKeyParse>;

    flags = parser.get_cmd_opts_ex("")?;
    for f in flags.iter() {
    	if  f.type_name() == KEYWORD_ARGS {
    		println!("args.nargs={:?}", f.get_nargs_v());
    	} else if f.flag_name() == "port" {
    		println!("port.nargs={:?}", f.get_nargs_v());
    	}
    }


    flags = parser.get_cmd_opts_ex("dep")?;
    for f in flags.iter() {
    	if  f.type_name() == KEYWORD_ARGS {
    		println!("dep.args.nargs={:?}", f.get_nargs_v());
    	}
    }

    flags = parser.get_cmd_opts_ex("rdep")?;
    for f in flags.iter() {
    	if  f.type_name() == KEYWORD_ARGS {
    		println!("rdep.args.nargs={:?}", f.get_nargs_v());
    	}
    }

    Ok(())
}
/*
output:
args.nargs=*
port.nargs=1
dep.args.nargs=+
rdep.args.nargs=?
*/