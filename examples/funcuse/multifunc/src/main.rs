use extargsparse_codegen::{ArgSet,extargs_load_commandline,extargs_map_function};
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::{extargs_error_class,extargs_new_error};
use extargsparse_worker::namespace::NameSpaceEx;
use extargsparse_worker::funccall::ExtArgsParseFunc;
use extargsparse_worker::parser::ExtArgsParser;
use extargsparse_worker::key::{ExtKeyParse,KEYWORD_STRING};


use std::error::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::sync::Arc;
use std::cell::RefCell;
use std::any::Any;
use std::collections::HashMap;
use serde_json::Value;


extargs_error_class!{TestCaseError}

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
	ccval :Vec<String>,
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

fn debug_upper_jsonfunc(ns :NameSpaceEx, keycls :ExtKeyParse, value :Value) -> Result<(),Box<dyn Error>> {
    let mut setval :String;

    if !keycls.is_flag() || keycls.type_name() != KEYWORD_STRING {
        extargs_new_error!{TestCaseError,"keycls [{}] not string type", keycls.string()}
    }

    match value {
        Value::Null => {
            setval = "".to_string();
        },
        Value::String(_a) => {
            setval = format!("{}",_a);
        },
        _ => { extargs_new_error!{TestCaseError,"value [{:?}] not valid" , value } }
    }

    setval = setval.to_uppercase();
    return ns.set_string(&keycls.opt_dest(), setval);
}

fn debug_opthelp_set(keycls :&ExtKeyParse) -> String {
    let mut cs :String = "".to_string();
    let mut idx :i32 = 0;
    match keycls.value() {
        Value::Array(_a) => {
            cs.push_str("[");
            for curv in _a {
                match curv {
                    Value::String(v) => {
                        if idx > 0 {
                            cs.push_str(",");
                        }
                        cs.push_str(&(v.to_string()));
                        idx += 1;
                    },
                    _ => {}
                }
            }
            cs.push_str("]");
        },
        _ => {}
    }
    return format!("opthelp function set [{}] default value ({})", keycls.opt_dest(), cs);
}

fn debug_set_2_args(ns :NameSpaceEx, validx :i32, keycls :ExtKeyParse, params :Vec<String>) -> Result<i32,Box<dyn Error>> {
    let mut sarr :Vec<String>;
    println!("validx [{}]",validx);
    if (validx + 2) > params.len() as i32 {
        extargs_new_error!{TestCaseError,"[{}+2] > len({}) {:?}",validx,params.len(),params}
    }

    sarr = ns.get_array(&keycls.opt_dest());
    sarr.push(format!("{}",params[validx as usize]));
    sarr.push(format!("{}",params[(validx + 1) as usize]));
    println!("set [{}] value {:?}", keycls.opt_dest(),sarr);
    ns.set_array(&keycls.opt_dest(),sarr)?;
    println!("{}={:?}", keycls.opt_dest(), ns.get_array(&keycls.opt_dest()));
    return Ok(2);
}


#[extargs_map_function(dep_handler,rdep_handler,jsonfunc=debug_upper_jsonfunc,opthelp=debug_opthelp_set,actfunc=debug_set_2_args)]
fn main() -> Result<(),Box<dyn Error>> {
    let cmdline = r#"{
    	"verbose|v" : "+",
    	"port|p" : 3000,
    	"ccval|C!jsonfunc=debug_upper_jsonfunc;opthelp=debug_opthelp_set;actfunc=debug_set_2_args!" : [],
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
    let ns :NameSpaceEx = parser.parse_commandline_ex(None,None,Some(argv.clone()),None)?;
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
    println!("argv.ccval {:?}",argv.borrow().ccval );
    println!("ccval {:?}", ns.get_array("ccval"));

    Ok(())
}
/*
output:

*/