use extargsparse_codegen::{ArgSet,extargs_load_commandline,extargs_map_function};
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::{extargs_error_class,extargs_new_error};
use extargsparse_worker::namespace::NameSpaceEx;
use extargsparse_worker::funccall::ExtArgsParseFunc;
use extargsparse_worker::parser::ExtArgsParser;
use extargsparse_worker::key::{ExtKeyParse,KEYWORD_LIST};


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
    let mut setval :Vec<String> = Vec::new();

    if !keycls.is_flag() || keycls.type_name() != KEYWORD_LIST {
        extargs_new_error!{TestCaseError,"keycls [{}] not string type", keycls.string()}
    }

    match value {
        Value::Null => {
        },
        Value::Array(_v) => {
        	let mut _idx :i32 = 0;
            for cv in _v.iter() {
            	match cv {
            		Value::String(_a) => {
            			setval.push(format!("{}",_a).to_uppercase());
            		},
            		_ => {
            			extargs_new_error!{TestCaseError,"[{}] not string type",_idx}
            		}
            	}
            	_idx += 1;
            }
        },
        _ => { extargs_new_error!{TestCaseError,"value [{:?}] not valid" , value } }
    }

    return ns.set_array(&keycls.opt_dest(), setval);
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


#[extargs_map_function(dep_handler,rdep_handler,jsonfunc=debug_upper_jsonfunc,opthelp=debug_opthelp_set,optparse=debug_set_2_args)]
fn main() -> Result<(),Box<dyn Error>> {
    let cmdline = r#"{
    	"verbose|v" : "+",
    	"port|p" : 3000,
    	"ccval|C!jsonfunc=debug_upper_jsonfunc;opthelp=debug_opthelp_set;optparse=debug_set_2_args!" : [],
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
command:
cargo run --release -- -h
output:
multifunc.exe  [OPTIONS] [SUBCOMMANDS] [args...]

 [OPTIONS]
    --json        json     json input file to get the value set            
    --help|-h              to display this help information                
    --ccval|-C    ccval    opthelp function set [ccval] default value ([]) 
    --port|-p     port     port set default 3000                           
    --verbose|-v  verbose  count set default 0                             

[SUBCOMMANDS]
    [dep]   dep handler  
    [rdep]  rdep handler 
notice:
--ccval|-C    ccval    opthelp function set [ccval] default value ([]) 
line is formatted by debug_opthelp_set

command:
cargo run --release -- -C bbs wwww depargs dep
output:
validx [1]
set [ccval] value ["bbs", "wwww"]
ccval=["bbs", "wwww"]
call dep_handler
ns.verbose 0
ns.port 3000
subcommand dep
rdep_list []
rdep_string []
subnargs ["depargs"]
argv.verbose 0
argv.port 3000
argv.args []
argv.rdep.subnargs []
argv.rdep.list []
argv.rdep.string "s_rdep"
argv.ccval ["bbs", "wwww"]
ccval ["bbs", "wwww"]
notice:
-C bbs wwww will give two args format debug_set_2_args

file c.json:
{
	"ccval" : ["bb","aa"]
}

command:
cargo run --release -- --json .\c.json depargs dep

output:
call dep_handler
ns.verbose 0
ns.port 3000
subcommand dep
rdep_list []
rdep_string []
subnargs ["depargs"]
argv.verbose 0
argv.port 3000
argv.args []
argv.rdep.subnargs []
argv.rdep.list []
argv.rdep.string "s_rdep"
argv.ccval ["BB", "AA"]
ccval ["BB", "AA"]

notice:
call json function debug_upper_jsonfunc to uppercase

*/