use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
//use extargsparse_worker::argset::{ArgSetImpl};
//use extargsparse_worker::{extargs_error_class,extargs_new_error};
use extargsparse_worker::options::{ExtArgsOptions,OPT_PROG};
//use extargsparse_worker::namespace::NameSpaceEx;
use extargsparse_worker::funccall::ExtArgsParseFunc;
use extargsparse_worker::parser::ExtArgsParser;


use std::error::Error;
use lazy_static::lazy_static;
//use regex::Regex;
//use std::sync::Arc;
//use std::cell::RefCell;
//use std::any::Any;
use std::collections::HashMap;
use std::fs::{File};
//use std::io::prelude::*;



#[extargs_map_function()]
fn main() -> Result<(),Box<dyn Error>> {
    let cmdline = r#"{
            "verbose|v" : "+",
            "+http" : {
                "url|u" : "http://www.google.com",
                "visual_mode|V": false
            },
            "$port|p" : {
                "value" : 3000,
                "type" : "int",
                "nargs" : 1 ,
                "helpinfo" : "port to connect"
            },
            "dep" : {
                "list|l" : [],
                "string|s" : "s_var",
                "$" : "+",
                "ip" : {
                    "verbose" : "+",
                    "list" : [],
                    "cc" : []
                }
            },
            "rdep" : {
                "ip" : {
                    "verbose" : "+",
                    "list" : [],
                    "cc" : []
                }
            }
        }"#;
    let mut f : File = File::create("help.out")?;
    let optstr :String = format!("{{\"{}\" : \"cmd1\"}}", OPT_PROG);
    let options = ExtArgsOptions::new(&optstr)?;
    let parser :ExtArgsParser = ExtArgsParser::new(Some(options.clone()),None)?;
    extargs_load_commandline!(parser,cmdline)?;
    parser.print_help_ex(&mut f,"")?;
    parser.print_help_ex(&mut f,"dep")?;
    parser.print_help_ex(&mut f,"rdep")?;
/*
output:
cmd1  [OPTIONS] [SUBCOMMANDS] [args...]

 [OPTIONS]
    --json                 json     
                    json input file to get the value set
    --help|-h                       
                    to display this help information
    --port|-p              port     
                    port to connect
    --http-url|-u          http_url 
                    http_url set default http://www.google.com
    --http-visual-mode|-V           
                    http_visual_mode set true default(False)
    --verbose|-v           verbose  
                    count set default 0

[SUBCOMMANDS]
    [dep]   dep handler  
    [rdep]  rdep handler 
cmd1  dep dep handler

 [OPTIONS]
    --dep-json       dep_json    json input file to get the value set 
    --help|-h                    to display this help information     
    --dep-list|-l    dep_list    dep_list set default []              
    --dep-string|-s  dep_string  dep_string set default s_var         

[SUBCOMMANDS]
    [ip]   ip handler  
cmd1  rdep rdep handler

 [OPTIONS]
    --rdep-json  rdep_json  json input file to get the value set 
    --help|-h               to display this help information     

[SUBCOMMANDS]
    [ip]    ip handler   
*/
    Ok(())
}
