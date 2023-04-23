use extargsparse_codegen::{ArgSet,extargs_load_commandline,extargs_map_function};
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::{extargs_error_class,extargs_new_error};
//use extargsparse_worker::{extargs_log_trace};
//use extargsparse_worker::key::{ExtKeyParse};
//use extargsparse_worker::options::{ExtArgsOptions,OPT_PROG};
use extargsparse_worker::namespace::NameSpaceEx;
use extargsparse_worker::funccall::ExtArgsParseFunc;
use extargsparse_worker::parser::ExtArgsParser;


use std::error::Error;
use lazy_static::lazy_static;
use std::sync::Arc;
use std::cell::RefCell;
//use std::any::Any;
use std::collections::HashMap;


#[derive(ArgSet)]
struct ParseArgs {
	m_verbose :i32,
	m_removed :bool,
	m_floatv :f64,
	intv :i32,
	arr1 :Vec<String>,
	strv :String,
	args :Vec<String>,
}





#[extargs_map_function()]
fn main() -> Result<(),Box<dyn Error>> {
    let loads = r#"{
		"verbose|v<m_verbose>" : "+",
		"removed|R<m_removed>" : false,
		"floatv|f<m_floatv>" : 3.3,
		"intv|i" : 5,
		"arrl|a" : [],
		"strv|s" : null,
		"$" : "+"
		}"#;
    let parser :ExtArgsParser = ExtArgsParser::new(None,None)?;
    let v :ParseArgs = ParseArgs::new();
    let argv :Arc<RefCell<ParseArgs>> = Arc::new(RefCell::new(v));
    extargs_load_commandline!(parser,loads)?;
    let vv :Vec<String> = vec!["-vvv".to_string(), "-R".to_string(), "-a".to_string(), "cc".to_string(), "-s".to_string(), "csw".to_string(), "ww".to_string(), "ee".to_string()];
    let _ns : NameSpaceEx =  parser.parse_commandline_ex(Some(vv.clone()), None, Some(argv.clone()), None) ? ;
    println! ("verbose={}", argv.borrow().m_verbose) ; 
    println! ("removed={}", argv.borrow().m_removed) ; 
    println! ("floatv={}", argv.borrow().m_floatv) ;
    println! ("intv={}", argv.borrow().intv) ; 
    println! ("arr1={:?}", argv.borrow().arr1) ;
    println! ("strv={}", argv.borrow().strv) ; 
    println! ("args={:?}", argv.borrow().args) ;

    let opts = parser.get_cmd_opts_ex("")?;

    for f in opts.iter() {
    	if f.type_name() == "args" {
    		println!("args.varname={}", f.var_name());
    	} else {
    		println!("{}.varname={}", f.flag_name(), f.var_name());
    	}
    }

/*
output:
verbose=3
removed=true
floatv=3.3
intv=5
arr1=[]
strv=csw
args=["ww", "ee"]
args.varname=args
arrl.varname=arrl
floatv.varname=m_floatv
help.varname=help
intv.varname=intv
json.varname=json
removed.varname=m_removed
strv.varname=strv
verbose.varname=m_verbose
*/
    Ok(())
}


