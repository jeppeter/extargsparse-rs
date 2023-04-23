use extargsparse_codegen::{ArgSet,extargs_load_commandline,extargs_map_function};
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::{extargs_error_class,extargs_new_error};
//use extargsparse_worker::options::{ExtArgsOptions,OPT_PROG};
use extargsparse_worker::namespace::NameSpaceEx;
use extargsparse_worker::key::ExtKeyParse;
use extargsparse_worker::funccall::ExtArgsParseFunc;
use extargsparse_worker::parser::ExtArgsParser;


use std::error::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::sync::Arc;
use std::cell::RefCell;
use std::any::Any;
use std::collections::HashMap;
//use std::fs::{File};

extargs_error_class!{HelpError}

#[derive(ArgSet)]
struct DepSt {
	list :Vec<String>,
	strv :String,
	subnargs :Vec<String>,
}

#[derive(ArgSet)]
struct RdepSt {
	list :Vec<String>,
	strv :String,
	subnargs :Vec<String>,
}

#[derive(ArgSet)]
struct SubCmdStruct {
	verbose :i32,
	pair :Vec<String>,
	dep :DepSt,
	rdep :RdepSt,
	args :Vec<String>,
}

fn dep_handler(_ns :NameSpaceEx, _args :Option<Arc<RefCell<dyn ArgSetImpl>>>, _ctx : Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {

    if _ctx.is_some() {
        let ctx = _ctx.as_ref().unwrap().clone();
        let mut bctx = ctx.borrow_mut();
        match bctx.downcast_mut::<SubCmdStruct>() {
            Some(_v) => {
            	println!("subcommand={}", _ns.get_string("subcommand"));
            	println!("verbose={}", _v.verbose);
            	println!("pair={:?}", _v.pair);
            	return Ok(());
            },
            _ => {
            	extargs_new_error!{HelpError,"can not downcast_mut to SubCmdStruct"}
            }
        }
    }

	extargs_new_error!{HelpError,"no _ctx"}
}

fn rdep_handler(_ns :NameSpaceEx, _args :Option<Arc<RefCell<dyn ArgSetImpl>>>, _ctx : Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {

    if _ctx.is_some() {
        let ctx = _ctx.as_ref().unwrap().clone();
        let mut bctx = ctx.borrow_mut();
        match bctx.downcast_mut::<SubCmdStruct>() {
            Some(_v) => {
            	println!("subcommand={}", _ns.get_string("subcommand"));
            	println!("verbose={}", _v.verbose);
            	println!("pair={:?}", _v.pair);
            	return Ok(());
            },
            _ => {
            	extargs_new_error!{HelpError,"can not downcast_mut to SubCmdStruct"}
            }
        }
    }

	extargs_new_error!{HelpError,"no _ctx"}
}


fn debug_opthelp_set(_keycls :&ExtKeyParse) -> String {
	return format!("pair set 2 args");
}


#[extargs_map_function(dep_handler,rdep_handler,opthelp=debug_opthelp_set)]
fn main() -> Result<(),Box<dyn Error>> {
    let cmdline = r#"{
		"verbose" : "+",
		"pair|p!opthelp=debug_opthelp_set!" : [],
		"dep<dep_handler>" : {
			"$" : "*",
			"list|L" :  [],
			"str|S" : ""
		},
		"rdep<rdep_handler>" : {
			"$" : "*",
			"list|l" : [],
			"str|s" : ""
		}
		}"#;
    let parser :ExtArgsParser = ExtArgsParser::new(None,None)?;
    extargs_load_commandline!(parser,cmdline)?;
    let ctx :SubCmdStruct = SubCmdStruct::new();
    let ctxpi : Arc<RefCell<SubCmdStruct>> = Arc::new(RefCell::new(ctx));
    let _ = parser.parse_commandline(None,Some(ctxpi.clone()))?;
    Ok(())
/*
command:
helpfunc.exe -h
output:
helpfunc.exe  [OPTIONS] [SUBCOMMANDS] [args...]

 [OPTIONS]
    --json     json     json input file to get the value set 
    --help|-h           to display this help information     
    --pair|-p  pair     pair set 2 args                      
    --verbose  verbose  count set default 0                  

[SUBCOMMANDS]
    [dep]   dep handler  
    [rdep]  rdep handler 
*/
}
