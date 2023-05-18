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
	string : String,
	list :Vec<String>,
}

#[derive(ArgSet)]
struct RdepSt {
	string :String,
	subnargs : Vec<String>,
	list :Vec<String>,
}

#[derive(ArgSet)]
struct ImpArg {
	verbose :i32,
	port :i32,
	dep :DepSt,
	rdep :RdepSt,
	args :Vec<String>,
}

fn dep_handler(_ns :NameSpaceEx, _args :Option<Arc<RefCell<dyn ArgSetImpl>>>, _ctx : Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	println!("call dep_handler");
	Ok(())
}

fn rdep_handler(_ns :NameSpaceEx, _args :Option<Arc<RefCell<dyn ArgSetImpl>>>, _ctx : Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	println!("call rdep_handler");
	Ok(())
}


#[extargs_map_function(dep_handler,rdep_handler)]
fn main() -> Result<(),Box<dyn Error>> {
    let cmdline = r#"{
    	"verbose|v" : "+",
    	"port|p" : 3000,
    	"dep<dep_handler>" : {
    		"list|l" : [],
    		"string|s" : "s_var",
    		"$" : "+"
    	},
    	"rdep<rdep_handler>" : {
    		"list|L" : [],
    		"string|S" : "s_rdep",
    		"$" : 2
    	}
    }"#;
    let parser :ExtArgsParser = ExtArgsParser::new(None,None)?;
    extargs_load_commandline!(parser,cmdline)?;
    let v :ImpArg = ImpArg::new();
    let argv :Arc<RefCell<ImpArg>> = Arc::new(RefCell::new(v));
    let args :Vec<String> = vec!["-vvvv".to_string(),
    	"-p".to_string(),"5000".to_string(),"rdep".to_string(),"-L".to_string(),"arg1".to_string(),
    	"--rdep-list".to_string(),"arg2".to_string(),"cc".to_string(),"dd".to_string()];
    let ns :NameSpaceEx = parser.parse_commandline_ex(Some(args),None,Some(argv.clone()),None)?;
    println!("ns.verbose {}", ns.get_int("verbose"));
    println!("ns.port {}",ns.get_int("port") );
    println!("subcommand {}",ns.get_string("subcommand"));
    println!("rdep_list {:?}", ns.get_array("rdep_list"));
    println!("rdep_string {:?}", ns.get_array("rdep_string"));
    println!("subnargs {:?}", ns.get_array("subnargs"));
    println!("argv.verbose {}", argv.borrow().verbose);
    println!("argv.port {}",argv.borrow().port);
    println!("argv.args {:?}",argv.borrow().args);
    println!("argv.rdep.subnargs {:?}", argv.borrow().rdep.subnargs);
    println!("argv.rdep.list {:?}", argv.borrow().rdep.list);
    println!("argv.rdep.string {:?}", argv.borrow().rdep.string);

    Ok(())
}
/*
output:
call rdep_handler
ns.verbose 4
ns.port 5000
subcommand rdep
rdep_list ["arg1", "arg2"]
rdep_string []
subnargs ["cc", "dd"]
argv.verbose 4
argv.port 5000
argv.args []
argv.rdep.subnargs ["cc", "dd"]
argv.rdep.list ["arg1", "arg2"]
argv.rdep.string "s_rdep"
*/