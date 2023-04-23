use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
//use extargsparse_worker::argset::{ArgSetImpl};
//use extargsparse_worker::{extargs_error_class,extargs_new_error};
use extargsparse_worker::options::{ExtArgsOptions,OPT_NO_JSON_OPTION,OPT_NO_HELP_OPTION};
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
use std::io::{Write};
//use std::fs::{File};
use tempfile::{NamedTempFile};


fn make_temp_file(s :&str) -> NamedTempFile {
    let retv = NamedTempFile::new().unwrap();
    let mut f = retv.reopen().unwrap();
    f.write_all(s.as_bytes()).unwrap();
    f.sync_all().unwrap();
    return retv;
}

fn set_env_var(k :&str, v :&str) {
    std::env::set_var(k,v);
}

fn before_parser() {
    let mut cont : i32= 1;
    while cont > 0 {
        cont = 0;
        for (k,_) in std::env::vars() {
            let sk = k.to_uppercase();
            if sk.starts_with("EXTARGS_") || 
            sk.starts_with("DEP_") || 
            sk.starts_with("RDEP_") || 
            sk.starts_with("HTTP_")  ||
            sk.starts_with("SSL_") || 
            sk.starts_with("EXTARGSPARSE_JSON") || 
            sk.starts_with("EXTARGSPARSE_JSONFILE"){
                std::env::remove_var(k);
                cont = 1;
                break;
            }
        }
    }
    return;
}



#[extargs_map_function()]
fn main() -> Result<(),Box<dyn Error>> {
    let cmdline = r#"{
            "verbose|v" : "+",
            "$port|p" : {
                "value" : 3000,
                "type" : "int",
                "nargs" : 1 ,
                "helpinfo" : "port to connect"
            },
            "dep" : {
                "list|l" : [],
                "string|s" : "s_var",
                "$" : "+"
            }
        }"#;
    let jsonfile :String;
    let depjsonfile :String;
    let depstrval :String = r#"newval"#.to_string();
    let deplistval :String =r#"["depenv1","depenv2"]"#.to_string();
    let jsonfilecon = r#"{"dep":{"list" : ["jsonval1","jsonval2"],"string" : "jsonstring"},"port":6000,"verbose":3}"#.to_string();
    let f = make_temp_file(&jsonfilecon);
    jsonfile = format!("{}",f.path().display());
    let depjsoncon = r#"{"list":["depjson1","depjson2"]}"#;
    let f = make_temp_file(&depjsoncon);
    depjsonfile = format!("{}",f.path().display());

    before_parser();

    set_env_var("EXTARGSPARSE_JSONFILE",&jsonfile);
    set_env_var("DEP_JSONFILE",&depjsonfile);
    let optstr = format!("{{ \"{}\" : true,\"{}\"  : true}}",OPT_NO_JSON_OPTION,OPT_NO_HELP_OPTION);
    let options = ExtArgsOptions::new(&optstr)?;
    let parser :ExtArgsParser = ExtArgsParser::new(Some(options.clone()),None)?;
    extargs_load_commandline!(parser,cmdline)?;

    set_env_var("DEP_STRING",&depstrval);
    set_env_var("DEP_LIST", &deplistval);
    let params = vec!["-p".to_string(), "9000".to_string(), "dep".to_string(), "--dep-string".to_string(), "ee".to_string(), "ww".to_string()];
    let ns = parser.parse_commandline(Some(params.clone()),None)?;
    println!("verbose={}", ns.get_int("verbose"));
    println!("port={}", ns.get_int("port"));
    println!("subcommand={}", ns.get_string("subcommand"));
    println!("dep_list={:?}", ns.get_array("dep_list"));
    println!("dep_string={}", ns.get_string("dep_string"));
    println!("subnargs={:?}", ns.get_array("subnargs"));


    Ok(())
}
/*
output:
verbose=0
port=9000
subcommand=dep
dep_list=["depenv1", "depenv2"]
dep_string=ee
subnargs=["ww"]
*/