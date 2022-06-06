use extargsparse_codegen::{ArgSet,extargs_load_commandline,extargs_map_function};
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::{extargs_error_class,extargs_new_error};
use extargsparse_worker::namespace::NameSpaceEx;
use extargsparse_worker::funccall::ExtArgsParseFunc;
use extargsparse_worker::parser::ExtArgsParser;


use std::error::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::sync::Arc;
use std::cell::RefCell;
use std::any::Any;
use std::collections::HashMap;

#[derive(ArgSet)]
struct DepSt {
	subnargs :Vec<String>,
}

#[derive(ArgSet)]
struct ImpArg {
	verbose :i32,
	port :i32,
	dep :DepSt,
	args :Vec<String>,
}

fn dep_handler(_ns :NameSpaceEx, _args :Option<Arc<RefCell<dyn ArgSetImpl>>>, _ctx : Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	println!("call dep_handler");
	Ok(())
}

#[extargs_map_function(dep_handler)]
fn main() -> Result<(),Box<dyn Error>> {
    let cmdline = r#"{
    	"verbose|v" : "+",
    	"port|p" : 9000,
    	"dep<dep_handler>## to make dep handler##" : {
    		"$" : "*"
    	}
    }"#;
    let parser :ExtArgsParser = ExtArgsParser::new(None,None)?;
    extargs_load_commandline!(parser,cmdline)?;
    //parser.load_commandline_string(cmdline,Some(ST_FUNCTIONS_MFHGDTXIBZ9MXQY.clone()))?;
    let v :ImpArg = ImpArg::new();
    let argv :Arc<RefCell<ImpArg>> = Arc::new(RefCell::new(v));
    let ns :NameSpaceEx = parser.parse_commandline_ex(None,None,Some(argv.clone()),None)?;
    println!("ns.verbose {}", ns.get_int("verbose"));
    println!("ns.port {}",ns.get_int("port") );
    println!("subcommand {}",ns.get_string("subcommand"));
    println!("args {:?}", ns.get_array("args"));
    println!("subnargs {:?}", ns.get_array("subnargs"));
    println!("argv.verbose {}", argv.borrow().verbose);
    println!("argv.port {}",argv.borrow().port);
    println!("argv.args {:?}",argv.borrow().args);
    println!("argv.dep.subnargs {:?}", argv.borrow().dep.subnargs);

    Ok(())
}
