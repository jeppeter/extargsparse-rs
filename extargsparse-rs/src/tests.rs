use super::parser::{ExtArgsParser};
use super::logger::{extargs_debug_out};
use super::argset::{ArgSetImpl};
use super::funccall::{ExtArgsParseFunc};
use super::{extargs_log_trace};
use super::{error_class};
use super::namespace::{NameSpaceEx};
use super::key::{ExtKeyParse,KEYWORD_DOLLAR_SIGN,Nargs,KEYWORD_COUNT,KEYWORD_JSONFILE,KEYWORD_HELP,KEYWORD_BOOL};
use super::options::{ExtArgsOptions,OPT_PROG};
use super::const_value::{ENV_COMMAND_JSON_SET, ENVIRONMENT_SET, ENV_SUB_COMMAND_JSON_SET};
use std::cell::RefCell;
use std::sync::Arc;
use std::error::Error;
use std::boxed::Box;
use regex::Regex;
use std::any::Any;
use lazy_static::lazy_static;
use std::collections::HashMap;
use tempfile::{NamedTempFile};
//use std::fs::File;
use serde_json::Value;
use std::io::{Write,BufWriter};


use extargsparse_codegen::{extargs_load_commandline,ArgSet,extargs_map_function};


fn before_parser() {
	let mut cont : i32= 1;
	while cont > 0 {
		cont = 0;
		for (k,_) in std::env::vars() {
			let sk = k.to_uppercase();
			if sk.starts_with("EXTARGS_") || 
				sk.starts_with("DEP_") || 
				sk.starts_with("RDEP_") || 
				sk.starts_with("HTTP_")	 ||
				sk.starts_with("SSL_") || 
				sk.starts_with("EXTARGSPARSE_JSON") || 
				sk.starts_with("EXTARGSPARSE_JSONFILE"){
					extargs_log_trace!("remove_var [{}]",k);
					std::env::remove_var(k);
					cont = 1;
					break;
			}
		}
	}
	return;
}

fn set_env_var(k :&str, v :&str) {
	extargs_log_trace!("set_var [{}] = [{}]",k,v);
	std::env::set_var(k,v);
}

fn format_string_array(v :Vec<&str>) -> Vec<String> {
	let mut retv :Vec<String> = Vec::new();
	for i in v.iter() {
		retv.push(format!("{}",i));
	}
	retv
}

fn check_array_equal(a1 :Vec<String>, a2 :Vec<String>) -> bool {
	if a1.len() != a2.len() {
		extargs_log_trace!("[{}] != [{}]",a1.len(),a2.len());
		return false;
	}

	let mut idx :usize = 0;
	while idx < a1.len() {
		if a1[idx] != a2[idx] {
			extargs_log_trace!("[{}] [{}] != [{}]", idx,a1[idx],a2[idx]);
			return false;
		}
		idx += 1;
	}
	return true;
}

#[derive(ArgSet)]
struct Depst {
	list :Vec<String>,
	string : String,
	subnargs :Vec<String>,
}

#[derive(ArgSet)]
struct ParserTest2 {
	verbose :i32,
	port :i32,
	dep :Depst,
	args : Vec<String>,
}

#[test]
fn test_a001() {
	let loads = r#"
	        {
            "verbose|v##increment verbose mode##" : "+",
            "flag|f## flag set##" : false,
            "number|n" : 0,
            "list|l" : [],
            "string|s" : "string_var",
            "$" : {
                "value" : [],
                "nargs" : "*",
                "type" : "string"
            }
        }
	"#;
	let params :Vec<String> = format_string_array(vec!["-vvvv", "-f", "-n", "30", "-l", "bar1", "-l", "bar2", "var1", "var2"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	before_parser();
	extargs_load_commandline!(parser,loads).unwrap();
	let ns = parser.parse_commandline(Some(params.clone()),None).unwrap();
	assert!(ns.get_int("verbose") == 4);
	assert!(ns.get_bool("flag") == true);
	assert!(ns.get_int("number") == 30);
	assert!(check_array_equal(ns.get_array("list"),format_string_array(vec!["bar1", "bar2"])));
	assert!(ns.get_string("string") == "string_var");
	assert!(check_array_equal(ns.get_array("args"), format_string_array(vec!["var1","var2"])));
	return;
}



#[test]
fn test_a002() {
	let loads = r#"
        {
            "verbose|v" : "+",
            "port|p" : 3000,
            "dep" : {
                "list|l" : [],
                "string|s" : "s_var",
                "$" : "+"
            }
        }
    "#;
	let params :Vec<String> = format_string_array(vec!["-vvvv", "-p", "5000", "dep", "-l", "arg1", "--dep-list", "arg2", "cc", "dd"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	before_parser();
	extargs_log_trace!(" ");
	extargs_load_commandline!(parser,loads).unwrap();
	extargs_log_trace!(" ");
	let p :ParserTest2 = ParserTest2::new();
	let pi :Arc<RefCell<ParserTest2>> = Arc::new(RefCell::new(p));
	extargs_log_trace!(" ");
	let _ns = parser.parse_commandline_ex(Some(params.clone()),None,Some(pi.clone()),None).unwrap();
	extargs_log_trace!("verbose [{}]",pi.borrow().verbose);
	assert!(pi.borrow().verbose == 4);
	assert!(pi.borrow().port == 5000);
	assert!(_ns.get_string("subcommand") == "dep" );
	extargs_log_trace!("list [{:?}]", pi.borrow().dep.list);
	assert!(check_array_equal(pi.borrow().dep.list.clone(), format_string_array(vec!["arg1", "arg2"])) );
	assert!(pi.borrow().dep.string == "s_var");
	assert!(check_array_equal(pi.borrow().dep.subnargs.clone(), format_string_array(vec!["cc", "dd"])));
	return;
}

#[derive(ArgSet)]
struct ParserTest3 {
	verbose :i32,
	port :i32,
	dep_list :Vec<String>,
	dep_string :String,
	dep_subnargs : Vec<String>,
	rdep_list :Vec<String>,
	rdep_string : String,
	rdep_subnargs : Vec<String>,
	args : Vec<String>,
}

#[test]
fn test_a003() {
	let loads = r#"{
            "verbose|v" : "+",
            "port|p" : 3000,
            "dep" : {
                "list|l" : [],
                "string|s" : "s_var",
                "$" : "+"
            },
            "rdep" : {
                "list|L" : [],
                "string|S" : "s_rdep",
                "$" : 2
            }
        }"#;
	let params :Vec<String> = format_string_array(vec!["-vvvv", "-p", "5000", "rdep", "-L", "arg1", "--rdep-list", "arg2", "cc", "dd"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	before_parser();
	extargs_log_trace!(" ");
	extargs_load_commandline!(parser,loads).unwrap();
	extargs_log_trace!(" ");
	let p :ParserTest3 = ParserTest3::new();
	let pi :Arc<RefCell<ParserTest3>> = Arc::new(RefCell::new(p));
	extargs_log_trace!(" ");
	let _ns = parser.parse_commandline_ex(Some(params.clone()),None,Some(pi.clone()),None).unwrap();
	assert!(pi.borrow().verbose == 4);
	assert!(pi.borrow().port == 5000);
	assert!(_ns.get_string("subcommand") == "rdep" );
	assert!(check_array_equal(pi.borrow().rdep_list.clone(), format_string_array(vec!["arg1", "arg2"])) );
	assert!(pi.borrow().rdep_string == "s_rdep");
	assert!(check_array_equal(pi.borrow().rdep_subnargs.clone(), format_string_array(vec!["cc", "dd"])));
	assert!(check_array_equal(pi.borrow().dep_subnargs.clone(),format_string_array(vec![])));
	assert!(check_array_equal(pi.borrow().dep_list.clone(),format_string_array(vec![])));
	assert!(pi.borrow().dep_string== "s_var");
	assert!(check_array_equal(pi.borrow().args.clone(),format_string_array(vec![])));
	return;
}


#[test]
fn test_a003_2() {
	let loads = r#"{
            "verbose|v" : "+",
            "port|p" : 3000,
            "dep" : {
                "list|l" : [],
                "string|s" : "s_var",
                "$" : "+"
            },
            "rdep" : {
                "list|L" : [],
                "string|S" : "s_rdep",
                "$" : 2
            }
        }"#;
	let params :Vec<String> = format_string_array(vec!["-vvvv", "-p", "5000", "rdep", "-L", "arg1", "--rdep-list", "arg2", "cc", "dd"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	before_parser();
	extargs_log_trace!(" ");
	extargs_load_commandline!(parser,loads).unwrap();
	let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
	assert!(ns.get_int("verbose") == 4);
	assert!(ns.get_int("port") == 5000);
	assert!(ns.get_string("subcommand") == "rdep" );
	assert!(check_array_equal(ns.get_array("rdep_list"), format_string_array(vec!["arg1", "arg2"])) );
	assert!(ns.get_string("rdep_string") == "s_rdep");
	assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["cc", "dd"])));
	assert!(check_array_equal(ns.get_array("dep_list"),format_string_array(vec![])));
	assert!(ns.get_string("dep_string")  == "s_var");
	assert!(check_array_equal(ns.get_array("args"),format_string_array(vec![])));
	return;
}


#[derive(ArgSet)]
struct ParserTest4 {
	verbose :i32,
	port :i32,
	dep :Depst,
	rdep :Depst,
	args : Vec<String>,
}

#[test]
fn test_a004() {
	let loads = r#"{
            "verbose|v" : "+",
            "port|p" : 3000,
            "dep" : {
                "list|l" : [],
                "string|s" : "s_var",
                "$" : "+"
            },
            "rdep" : {
                "list|L" : [],
                "string|S" : "s_rdep",
                "$" : 2
            }
        }"#;
	let params :Vec<String> = format_string_array(vec!["-vvvv", "-p", "5000", "rdep", "-L", "arg1", "--rdep-list", "arg2", "cc", "dd"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	before_parser();
	extargs_log_trace!(" ");
	extargs_load_commandline!(parser,loads).unwrap();
	extargs_log_trace!(" ");
	let p :ParserTest4 = ParserTest4::new();
	let pi :Arc<RefCell<ParserTest4>> = Arc::new(RefCell::new(p));
	extargs_log_trace!(" ");
	let _ns = parser.parse_commandline_ex(Some(params.clone()),None,Some(pi.clone()),None).unwrap();
	assert!(pi.borrow().verbose == 4);
	assert!(pi.borrow().port == 5000);
	assert!(_ns.get_string("subcommand") == "rdep" );
	assert!(check_array_equal(pi.borrow().rdep.list.clone(), format_string_array(vec!["arg1", "arg2"])) );
	assert!(pi.borrow().rdep.string == "s_rdep");
	assert!(check_array_equal(pi.borrow().rdep.subnargs.clone(), format_string_array(vec!["cc", "dd"])));
	assert!(check_array_equal(pi.borrow().dep.subnargs.clone(),format_string_array(vec![])));
	assert!(check_array_equal(pi.borrow().dep.list.clone(),format_string_array(vec![])));
	assert!(pi.borrow().dep.string== "s_var");
	assert!(check_array_equal(pi.borrow().args.clone(),format_string_array(vec![])));
	return;
}


#[test]
fn test_a004_2() {
	let loads = r#"{
            "verbose|v" : "+",
            "port|p" : 3000,
            "dep" : {
                "list|l" : [],
                "string|s" : "s_var",
                "$" : "+"
            },
            "rdep" : {
                "list|L" : [],
                "string|S" : "s_rdep",
                "$" : 2
            }
        }"#;
	let params :Vec<String> = format_string_array(vec!["-vvvv", "-p", "5000", "rdep", "-L", "arg1", "--rdep-list", "arg2", "cc", "dd"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	before_parser();
	extargs_log_trace!(" ");
	extargs_load_commandline!(parser,loads).unwrap();
	extargs_log_trace!(" ");
	extargs_log_trace!(" ");
	let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
	assert!(ns.get_int("verbose") == 4);
	assert!(ns.get_int("port") == 5000);
	assert!(ns.get_string("subcommand") == "rdep" );
	assert!(check_array_equal(ns.get_array("rdep_list"), format_string_array(vec!["arg1", "arg2"])) );
	assert!(ns.get_string("rdep_string") == "s_rdep");
	assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["cc", "dd"])));
	assert!(check_array_equal(ns.get_array("dep_list"),format_string_array(vec![])));
	assert!(ns.get_string("dep_string")== "s_var");
	assert!(check_array_equal(ns.get_array("args"),format_string_array(vec![])));
	return;
}

struct ParserTest5Ctx {
	has_called_args : String,
}

fn debug_args_function(_ns :NameSpaceEx, _args :Option<Arc<RefCell<dyn ArgSetImpl>>>, _parser :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	if _parser.is_some() {
		let ctx = _parser.as_ref().unwrap().clone();
		let mut bctx = ctx.borrow_mut();
		match bctx.downcast_mut::<ParserTest5Ctx>() {
			Some(_v) => {
				extargs_log_trace!("call ParserTest5Ctx downcast_mut");
				_v.has_called_args = _ns.get_string("subcommand");
			},
			_ => {

			}
		}
	}
	Ok(())
}

#[extargs_map_function(debug_args_function)]
#[test]
fn test_a005() {
	let loads = r#"
	        {
            "verbose|v" : "+",
            "port|p" : 3000,
            "dep<debug_args_function>" : {
                "list|l" : [],
                "string|s" : "s_var",
                "$" : "+"
            },
            "rdep" : {
                "list|L" : [],
                "string|S" : "s_rdep",
                "$" : 2
            }
        }"#;
	let params :Vec<String> = format_string_array(vec!["-p", "7003", "-vvvvv", "dep", "-l", "foo1", "-s", "new_var", "zz"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	before_parser();
	extargs_log_trace!(" ");
	extargs_load_commandline!(parser,loads).unwrap();
	let ctx :ParserTest5Ctx = ParserTest5Ctx{ has_called_args : "".to_string(),};
	let ctxpi : Arc<RefCell<ParserTest5Ctx>> = Arc::new(RefCell::new(ctx));
	extargs_log_trace!(" ");
	let ns = parser.parse_commandline(Some(params.clone()),Some(ctxpi.clone())).unwrap();
	assert!(ns.get_int("port") == 7003);
	assert!(ns.get_int("verbose") == 5);
	assert!(ns.get_string("subcommand") == "dep" );
	assert!(check_array_equal(ns.get_array("dep_list"), format_string_array(vec!["foo1"])) );
	assert!(ns.get_string("dep_string") == "new_var");
	assert!(ctxpi.borrow().has_called_args == "dep");
	assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["zz"])));
	return;
}

#[test]
fn test_a006() {
	let loads1 = r#"{
            "verbose|v" : "+",
            "port|p" : 3000,
            "dep" : {
                "list|l" : [],
                "string|s" : "s_var",
                "$" : "+"
            }
        }"#;
    let loads2 = r#"{
            "rdep" : {
                "list|L" : [],
                "string|S" : "s_rdep",
                "$" : 2
            }
        }"#;
	let params :Vec<String> = format_string_array(vec!["-p", "7003", "-vvvvv", "rdep", "-L", "foo1", "-S", "new_var", "zz", "64"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	before_parser();
	extargs_log_trace!(" ");
	extargs_load_commandline!(parser,loads1).unwrap();
	extargs_load_commandline!(parser,loads2).unwrap();
	extargs_log_trace!(" ");
	let ns = parser.parse_commandline(Some(params.clone()),None).unwrap();
	assert!(ns.get_int("port") == 7003);
	assert!(ns.get_int("verbose") == 5);
	assert!(ns.get_string("subcommand") == "rdep" );
	assert!(check_array_equal(ns.get_array("rdep_list"), format_string_array(vec!["foo1"])) );
	assert!(ns.get_string("rdep_string") == "new_var");
	assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["zz","64"])));
	return;
}

#[test]
fn test_a007() {
	let loads = r#"{
            "verbose|v" : "+",
            "port|p+http" : 3000,
            "dep" : {
                "list|l" : [],
                "string|s" : "s_var",
                "$" : "+"
            }
        }"#;
	let params :Vec<String> = format_string_array(vec!["-vvvv", "dep", "-l", "cc", "--dep-string", "ee", "ww"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	before_parser();
	extargs_log_trace!(" ");
	extargs_load_commandline!(parser,loads).unwrap();
	extargs_log_trace!(" ");
	let ns = parser.parse_commandline(Some(params.clone()),None).unwrap();
	assert!(ns.get_int("verbose") == 4);
	assert!(ns.get_int("http_port") == 3000);
	assert!(ns.get_string("subcommand") == "dep" );
	assert!(check_array_equal(ns.get_array("dep_list"), format_string_array(vec!["cc"])) );
	assert!(ns.get_string("dep_string") == "ee");
	assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["ww"])));
	return;
}

#[derive(ArgSet)]
struct ParserTest7 {
	verbose :i32,
	http_port :i32,
	dep_string : String,
	dep_list : Vec<String>,
	subnargs : Vec<String>,
}

#[test]
fn test_a007_2() {
	let loads = r#"{
            "verbose|v" : "+",
            "port|p+http" : 3000,
            "dep" : {
                "list|l" : [],
                "string|s" : "s_var",
                "$" : "+"
            }
        }"#;
	let params :Vec<String> = format_string_array(vec!["-vvvv", "dep", "-l", "cc", "--dep-string", "ee", "ww"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	before_parser();
	extargs_log_trace!(" ");
	extargs_load_commandline!(parser,loads).unwrap();
	extargs_log_trace!(" ");
	let p :ParserTest7 = ParserTest7::new();
	let pi :Arc<RefCell<ParserTest7>> = Arc::new(RefCell::new(p));
	extargs_log_trace!(" ");
	let _ns = parser.parse_commandline_ex(Some(params.clone()),None,Some(pi.clone()),None).unwrap();
	assert!(pi.borrow().verbose == 4);
	assert!(pi.borrow().http_port == 3000);
	assert!(_ns.get_string("subcommand") == "dep" );
	assert!(check_array_equal(pi.borrow().dep_list.clone(), format_string_array(vec!["cc"])) );
	assert!(pi.borrow().dep_string == "ee");
	assert!(check_array_equal(pi.borrow().subnargs.clone(), format_string_array(vec!["ww"])));
	return;
}

#[derive(ArgSet)]
struct ParserTest8 {
	verbose :i32,
	http_port :i32,
	http_visual_mode: bool,
	dep_string : String,
	dep_list : Vec<String>,
	subnargs : Vec<String>,
}

#[test]
fn test_a008() {
	let loads = r#"{
            "verbose|v" : "+",
            "+http" : {
                "port|p" : 3000,
                "visual_mode|V" : false
            },
            "dep" : {
                "list|l" : [],
                "string|s" : "s_var",
                "$" : "+"
            }
        }"#;
	let params :Vec<String> = format_string_array(vec!["-vvvv", "--http-port", "9000", "--http-visual-mode", "dep", "-l", "cc", "--dep-string", "ee", "ww"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	before_parser();
	extargs_log_trace!(" ");
	extargs_load_commandline!(parser,loads).unwrap();
	extargs_log_trace!(" ");
	let p :ParserTest8 = ParserTest8::new();
	let pi :Arc<RefCell<ParserTest8>> = Arc::new(RefCell::new(p));
	extargs_log_trace!(" ");
	let _ns = parser.parse_commandline_ex(Some(params.clone()),None,Some(pi.clone()),None).unwrap();
	assert!(pi.borrow().verbose == 4);
	assert!(pi.borrow().http_port == 9000);
	assert!(pi.borrow().http_visual_mode == true);
	assert!(_ns.get_string("subcommand") == "dep" );
	assert!(check_array_equal(pi.borrow().dep_list.clone(), format_string_array(vec!["cc"])) );
	assert!(pi.borrow().dep_string == "ee");
	assert!(check_array_equal(pi.borrow().subnargs.clone(), format_string_array(vec!["ww"])));
	return;
}


#[test]
fn test_a008_2() {
	let loads = r#"{
            "verbose|v" : "+",
            "+http" : {
                "port|p" : 3000,
                "visual_mode|V" : false
            },
            "dep" : {
                "list|l" : [],
                "string|s" : "s_var",
                "$" : "+"
            }
        }"#;
	let params :Vec<String> = format_string_array(vec!["-vvvv", "--http-port", "9000", "--http-visual-mode", "dep", "-l", "cc", "--dep-string", "ee", "ww"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	before_parser();
	extargs_log_trace!(" ");
	extargs_load_commandline!(parser,loads).unwrap();
	extargs_log_trace!(" ");
	extargs_log_trace!(" ");
	let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
	assert!(ns.get_int("verbose") == 4);
	assert!(ns.get_int("http_port") == 9000);
	assert!(ns.get_bool("http_visual_mode") == true);
	assert!(ns.get_string("subcommand") == "dep" );
	assert!(check_array_equal(ns.get_array("dep_list"), format_string_array(vec!["cc"])) );
	assert!(ns.get_string("dep_string")== "ee");
	assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["ww"])));
	return;
}

#[derive(ArgSet)]
struct Depvv {
	list :Vec<String>,
	strv :String,
	subnargs :Vec<String>,
}

#[derive(ArgSet)]
struct ParserTest9 {
	verbose :i32,
	port :i32,
	dep :Depvv,
	args : Vec<String>,
}

#[test]
fn test_a009() {
	let loads = r#"        {
            "verbose|v" : "+",
            "$port|p" : {
                "value" : 3000,
                "type" : "int",
                "nargs" : 1 ,
                "helpinfo" : "port to connect"
            },
            "dep" : {
                "list|l" : [],
                "string|s<dep.strv>" : "s_var",
                "$" : "+"
            }
        }"#;
	let params :Vec<String> = format_string_array(vec!["-vvvv", "-p", "9000", "dep", "-l", "cc", "--dep-string", "ee", "ww"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	before_parser();
	extargs_log_trace!(" ");
	extargs_load_commandline!(parser,loads).unwrap();
	extargs_log_trace!(" ");
	let p :ParserTest9 = ParserTest9::new();
	let pi :Arc<RefCell<ParserTest9>> = Arc::new(RefCell::new(p));
	extargs_log_trace!(" ");
	let _ns = parser.parse_commandline_ex(Some(params.clone()),None,Some(pi.clone()),None).unwrap();
	assert!(pi.borrow().verbose == 4);
	assert!(pi.borrow().port == 9000);
	assert!(_ns.get_string("subcommand") == "dep" );
	assert!(check_array_equal(pi.borrow().dep.list.clone(), format_string_array(vec!["cc"])) );
	assert!(pi.borrow().dep.strv == "ee");
	assert!(check_array_equal(pi.borrow().dep.subnargs.clone(), format_string_array(vec!["ww"])));
	return;
}


#[test]
fn test_a009_2() {
	let loads = r#"        {
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
	let params :Vec<String> = format_string_array(vec!["-vvvv", "-p", "9000", "dep", "-l", "cc", "--dep-string", "ee", "ww"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	before_parser();
	extargs_log_trace!(" ");
	extargs_load_commandline!(parser,loads).unwrap();
	extargs_log_trace!(" ");
	extargs_log_trace!(" ");
	let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
	assert!(ns.get_int("verbose") == 4);
	assert!(ns.get_int("port") == 9000);
	assert!(ns.get_string("subcommand") == "dep" );
	assert!(check_array_equal(ns.get_array("dep_list"), format_string_array(vec!["cc"])) );
	assert!(ns.get_string("dep_string")== "ee");
	assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["ww"])));
	return;
}

fn make_temp_file(s :&str) -> NamedTempFile {
	let retv = NamedTempFile::new().unwrap();
	let mut f = retv.reopen().unwrap();
	f.write_all(s.as_bytes()).unwrap();
	f.sync_all().unwrap();
	return retv;
}

#[derive(ArgSet)]
struct Depvv10 {
	list :Vec<String>,
	string :String,
	subnargs :Vec<String>,
}

#[derive(ArgSet)]
struct ParserTest10 {
	verbose :i32,
	port :i32,
	dep :Depvv10,
	args : Vec<String>,
}

#[test]
fn test_a010() {
	let loads = r#"        {
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
    let ws = r#"{"list" : ["jsonval1","jsonval2"],"string" : "jsonstring"}"#;
    let f = make_temp_file(ws);
    let depjsonfile = format!("{}",f.path().display());
	let params :Vec<String> = format_string_array(vec!["-vvvv", "-p", "9000", "dep", "--dep-json", &depjsonfile, "--dep-string", "ee", "ww"]);
	extargs_log_trace!("params {:?}", params);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	before_parser();
	extargs_load_commandline!(parser,loads).unwrap();
	let p :ParserTest10 = ParserTest10::new();
	let pi :Arc<RefCell<ParserTest10>> = Arc::new(RefCell::new(p));
	extargs_log_trace!(" ");
	let _ns = parser.parse_commandline_ex(Some(params.clone()),None,Some(pi.clone()),None).unwrap();
	assert!(pi.borrow().verbose == 4);
	assert!(pi.borrow().port == 9000);
	assert!(_ns.get_string("subcommand") == "dep" );
	assert!(check_array_equal(pi.borrow().dep.list.clone(), format_string_array(vec!["jsonval1", "jsonval2"])) );
	assert!(pi.borrow().dep.string == "ee");
	assert!(check_array_equal(pi.borrow().dep.subnargs.clone(), format_string_array(vec!["ww"])));
	return;
}

#[derive(ArgSet)]
struct ParserTest11 {
	verbose :i32,
	port :i32,
	dep :Depvv10,
	args : Vec<String>,
}

#[test]
fn test_a011() {
	let loads = r#"        {
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
    let ws = r#"{"list" : ["jsonval1","jsonval2"],"string" : "jsonstring"}"#;
    let f = make_temp_file(ws);
    let depjsonfile = format!("{}",f.path().display());
	let params :Vec<String> = format_string_array(vec!["-vvvv", "-p", "9000", "dep", "--dep-json", &depjsonfile, "--dep-string", "ee", "ww"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	before_parser();
	extargs_load_commandline!(parser,loads).unwrap();
	let p :ParserTest11 = ParserTest11::new();
	let pi :Arc<RefCell<ParserTest11>> = Arc::new(RefCell::new(p));
	extargs_log_trace!(" ");
	let ns = parser.parse_commandline_ex(Some(params.clone()),None,Some(pi.clone()),None).unwrap();
	assert!(ns.get_int("verbose") == 4);
	assert!(ns.get_int("port")== 9000);
	assert!(ns.get_string("subcommand") == "dep" );
	assert!(check_array_equal(ns.get_array("dep_list"), format_string_array(vec!["jsonval1", "jsonval2"])) );
	assert!(ns.get_string("dep_string") == "ee");
	assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["ww"])));
	return;
}

#[test]
fn test_a012() {
	let loads = r#"        {
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
    let ws = r#"{"dep":{"list" : ["jsonval1","jsonval2"],"string" : "jsonstring"},"port":6000,"verbose":3}"#;
    let f = make_temp_file(ws);
    let jsonfile = format!("{}",f.path().display());
	let params :Vec<String> = format_string_array(vec!["-p", "9000", "--json", &jsonfile, "dep", "--dep-string", "ee", "ww"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	before_parser();
	extargs_load_commandline!(parser,loads).unwrap();
	let p :ParserTest11 = ParserTest11::new();
	let pi :Arc<RefCell<ParserTest11>> = Arc::new(RefCell::new(p));
	extargs_log_trace!(" ");
	let ns = parser.parse_commandline_ex(Some(params.clone()),None,Some(pi.clone()),None).unwrap();
	assert!(ns.get_int("verbose") == 3);
	assert!(ns.get_int("port")== 9000);
	assert!(ns.get_string("subcommand") == "dep" );
	assert!(check_array_equal(ns.get_array("dep_list"), format_string_array(vec!["jsonval1", "jsonval2"])) );
	assert!(ns.get_string("dep_string") == "ee");
	assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["ww"])));
	return;
}

#[test]
fn test_a013() {
	let loads = r#"        {
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
    let ws = r#"{"dep":{"list" : ["jsonval1","jsonval2"],"string" : "jsonstring"},"port":6000,"verbose":3}"#;
    let f = make_temp_file(ws);
    let jsonfile = format!("{}",f.path().display());
	let params :Vec<String> = format_string_array(vec!["-p", "9000", "dep", "--dep-string", "ee", "ww"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	before_parser();
	extargs_load_commandline!(parser,loads).unwrap();
	let p :ParserTest11 = ParserTest11::new();
	let pi :Arc<RefCell<ParserTest11>> = Arc::new(RefCell::new(p));

	extargs_log_trace!(" ");
	set_env_var("EXTARGSPARSE_JSON",&jsonfile);
	let ns = parser.parse_commandline_ex(Some(params.clone()),None,Some(pi.clone()),None).unwrap();
	assert!(ns.get_int("verbose") == 3);
	assert!(ns.get_int("port")== 9000);
	assert!(ns.get_string("subcommand") == "dep" );
	assert!(check_array_equal(ns.get_array("dep_list"), format_string_array(vec!["jsonval1", "jsonval2"])) );
	assert!(ns.get_string("dep_string") == "ee");
	assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["ww"])));
	return;
}

#[test]
fn test_a014() {
	let loads = r#"        {
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
	before_parser();
    let ws = r#"{"dep":{"list" : ["jsonval1","jsonval2"],"string" : "jsonstring"},"port":6000,"verbose":3}"#;
    let depws = r#"{"list":["depjson1","depjson2"]}"#;
    let f = make_temp_file(ws);
    let depf = make_temp_file(depws);
    let jsonfile = format!("{}",f.path().display());
    let depjsonfile = format!("{}",depf.path().display());
	let params :Vec<String> = format_string_array(vec!["-p", "9000", "dep", "--dep-string", "ee", "ww"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	extargs_load_commandline!(parser,loads).unwrap();
	let p :ParserTest11 = ParserTest11::new();
	let pi :Arc<RefCell<ParserTest11>> = Arc::new(RefCell::new(p));

	extargs_log_trace!(" ");
    set_env_var("EXTARGSPARSE_JSON",&jsonfile);
    set_env_var("DEP_JSON",&depjsonfile);
	let ns = parser.parse_commandline_ex(Some(params.clone()),None,Some(pi.clone()),None).unwrap();
	assert!(ns.get_int("verbose") == 3);
	assert!(ns.get_int("port")== 9000);
	assert!(ns.get_string("subcommand") == "dep" );
	assert!(check_array_equal(ns.get_array("dep_list"), format_string_array(vec!["depjson1", "depjson2"])) );
	assert!(ns.get_string("dep_string") == "ee");
	assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["ww"])));
	return;
}

#[test]
fn test_a015() {
	let loads = r#"        {
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
	before_parser();
    let ws = r#"{"dep":{"list" : ["jsonval1","jsonval2"],"string" : "jsonstring"},"port":6000,"verbose":3}"#;
    let depws = r#"{"list":["depjson1","depjson2"]}"#;
    let f = make_temp_file(ws);
    let depf = make_temp_file(depws);
    let jsonfile = format!("{}",f.path().display());
    let depjsonfile = format!("{}",depf.path().display());
	let params :Vec<String> = format_string_array(vec!["-p", "9000", "--json", &jsonfile, "dep", "--dep-string", "ee", "ww"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	extargs_load_commandline!(parser,loads).unwrap();
	let p :ParserTest11 = ParserTest11::new();
	let pi :Arc<RefCell<ParserTest11>> = Arc::new(RefCell::new(p));

	extargs_log_trace!(" ");
    set_env_var("DEP_JSON",&depjsonfile);
	let ns = parser.parse_commandline_ex(Some(params.clone()),None,Some(pi.clone()),None).unwrap();
	assert!(ns.get_int("verbose") == 3);
	assert!(ns.get_int("port")== 9000);
	assert!(ns.get_string("subcommand") == "dep" );
	assert!(check_array_equal(ns.get_array("dep_list"), format_string_array(vec!["jsonval1", "jsonval2"])) );
	assert!(ns.get_string("dep_string") == "ee");
	assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["ww"])));
	return;
}

#[test]
fn test_a016() {
	let loads = r#"        {
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
	before_parser();
	let depstrval = r#"newval"#;
	let depliststrval = r#"["depenv1","depenv2"]"#;
    let ws = r#"{"dep":{"list" : ["jsonval1","jsonval2"],"string" : "jsonstring"},"port":6000,"verbose":3}"#;
    let depws = r#"{"list":["depjson1","depjson2"]}"#;
    let f = make_temp_file(ws);
    let depf = make_temp_file(depws);
    let jsonfile = format!("{}",f.path().display());
    let depjsonfile = format!("{}",depf.path().display());
	let params :Vec<String> = format_string_array(vec!["-p", "9000", "dep", "--dep-string", "ee", "ww"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	extargs_load_commandline!(parser,loads).unwrap();
	let p :ParserTest11 = ParserTest11::new();
	let pi :Arc<RefCell<ParserTest11>> = Arc::new(RefCell::new(p));

	extargs_log_trace!(" ");
	set_env_var("EXTARGSPARSE_JSON",&jsonfile);
    set_env_var("DEP_JSON",&depjsonfile);
    set_env_var("DEP_STRING",depstrval);
    set_env_var("DEP_LIST",depliststrval);
	let ns = parser.parse_commandline_ex(Some(params.clone()),None,Some(pi.clone()),None).unwrap();
	assert!(ns.get_int("verbose") == 3);
	assert!(ns.get_int("port")== 9000);
	assert!(ns.get_string("subcommand") == "dep" );
	assert!(check_array_equal(ns.get_array("dep_list"), format_string_array(vec!["depenv1", "depenv2"])) );
	assert!(ns.get_string("dep_string") == "ee");
	assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["ww"])));
	return;
}

#[test]
fn test_a017() {
	let loads = r#"        {
            "+dpkg" : {
                "dpkg" : "dpkg"
            },
            "verbose|v" : "+",
            "$port|p" : {
                "value" : 3000,
                "type" : "int",
                "nargs" : 1 ,
                "helpinfo" : "port to connect"
            }
        }"#;
	before_parser();
	let params :Vec<String> = format_string_array(vec![]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	extargs_load_commandline!(parser,loads).unwrap();

	extargs_log_trace!(" ");
	let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
	assert!(ns.get_int("verbose") == 0);
	assert!(ns.get_int("port")== 3000);
	assert!(ns.get_string("dpkg_dpkg") == "dpkg");
	assert!(check_array_equal(ns.get_array("args"), format_string_array(vec![])));
	return;
}

#[test]
fn test_a018() {
	let loads = r#"        {
            "+dpkg" : {
                "dpkg" : "dpkg"
            },
            "verbose|v" : "+",
            "rollback|r": true,
            "$port|p" : {
                "value" : 3000,
                "type" : "int",
                "nargs" : 1 ,
                "helpinfo" : "port to connect"
            }
        }"#;
	before_parser();
	let params :Vec<String> = format_string_array(vec!["-vvrvv"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	extargs_load_commandline!(parser,loads).unwrap();

	extargs_log_trace!(" ");
	let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
	assert!(ns.get_int("verbose") == 4);
	assert!(ns.get_bool("rollback") == false);
	assert!(ns.get_int("port")== 3000);
	assert!(ns.get_string("dpkg_dpkg") == "dpkg");
	assert!(check_array_equal(ns.get_array("args"), format_string_array(vec![])));
	return;
}

#[test]
fn test_a019() {
	let loads = r#"        {
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
	before_parser();
	let depstrval = "newval";
	let depliststr = r#"["depenv1","depenv2"]"#;
	let ws = r#"{"dep":{"list" : ["jsonval1","jsonval2"],"string" : "jsonstring"},"port":6000,"verbose":3}"#;
	let depws = r#"{"list":["depjson1","depjson2"]}"#;
	let f = make_temp_file(ws);
	let depf = make_temp_file(depws);
	let depjsonfile = format!("{}",depf.path().display());
	let jsonfile = format!("{}",f.path().display());

	let prioropt = Some(vec![ENV_COMMAND_JSON_SET, ENVIRONMENT_SET, ENV_SUB_COMMAND_JSON_SET]);

	let params :Vec<String> = format_string_array(vec!["-p", "9000", "dep", "--dep-string", "ee", "ww"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,prioropt).unwrap();
	extargs_load_commandline!(parser,loads).unwrap();
	set_env_var("EXTARGSPARSE_JSON", &jsonfile);
	set_env_var("DEP_JSON", &depjsonfile);
	set_env_var("DEP_STRING", depstrval);
	set_env_var("DEP_LIST", depliststr);
	extargs_log_trace!(" ");
	let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
	assert!(ns.get_int("verbose") == 3);
	assert!(ns.get_int("port") == 9000);
	assert!(ns.get_string("subcommand")== "dep");
	assert!(check_array_equal(ns.get_array("dep_list"), format_string_array(vec!["jsonval1", "jsonval2"])));
	assert!(ns.get_string("dep_string")== "ee");
	assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["ww"])));
	return;
}

#[test]
fn test_a020() {
	let loads = r#"        {
            "verbose|v" : "+",
            "rollback|R" : true,
            "$port|P" : {
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
	before_parser();

	let params :Vec<String> = format_string_array(vec!["-P", "9000", "--no-rollback", "dep", "--dep-string", "ee", "ww"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	extargs_load_commandline!(parser,loads).unwrap();
	let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
	assert!(ns.get_int("verbose") == 0);
	assert!(ns.get_int("port") == 9000);
	assert!(ns.get_bool("rollback") == false);
	assert!(ns.get_string("subcommand")== "dep");
	assert!(check_array_equal(ns.get_array("dep_list"), format_string_array(vec![])));
	assert!(ns.get_string("dep_string")== "ee");
	assert!(check_array_equal(ns.get_array("args"), format_string_array(vec![])));
	return;
}

#[test]
fn test_a021() {
	let loads = r#"        {
            "maxval|m" : 392244922
        }"#;
	before_parser();

	let params :Vec<String> = format_string_array(vec!["-m", "0xffcc"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	extargs_load_commandline!(parser,loads).unwrap();
	let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
	assert!(ns.get_int("maxval") == 0xffcc);
	return;
}

fn get_assert_opt(opts :Vec<ExtKeyParse>, kname :&str) -> Option<ExtKeyParse> {
	let mut retv :Option<ExtKeyParse> = None;
	for o in opts.iter() {
		if ! o.is_flag() {
			continue;
		}
		if o.flag_name() == KEYWORD_DOLLAR_SIGN && kname == KEYWORD_DOLLAR_SIGN {
			retv = Some(o.clone());
			break;
		}

		if o.flag_name() == KEYWORD_DOLLAR_SIGN {
			continue;
		}

		if o.opt_dest() == kname {
			retv = Some(o.clone());
			break;
		}
	}

	return retv;
}

#[test]
fn test_a022() {
	let loads = r#"        {
            "verbose|v" : "+"
        }"#;
	before_parser();

	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	extargs_load_commandline!(parser,loads).unwrap();
	let cmds :Vec<String> = parser.get_sub_commands_ex("").unwrap();
	assert!(check_array_equal(cmds.clone(),format_string_array(vec![])));
	let opts :Vec<ExtKeyParse> = parser.get_cmd_opts_ex("").unwrap();
	assert!(opts.len() == 4);
	let curopt = get_assert_opt(opts.clone(),"verbose").unwrap();
	assert!(curopt.opt_dest() == "verbose");
	assert!(curopt.long_opt() == "--verbose");
	assert!(curopt.short_opt() == "-v");
	assert!(get_assert_opt(opts.clone(),"noflag") == None);
	let curopt = get_assert_opt(opts.clone(),"json").unwrap();
	assert!(curopt.value() == Value::Null);
	let curopt = get_assert_opt(opts.clone(),"help").unwrap();
	assert!(curopt.type_name() == "help");
	assert!(curopt.long_opt() == "--help");
	assert!(curopt.short_opt() == "-h");

	return;
}

#[test]
fn test_a023() {
	let loads = r#"        {
            "verbose|v" : "+",
            "dep" : {
                "new|n" : false,
                "$<NARGS>" : "+"
            },
            "rdep" : {
                "new|n" : true,
                "$<NARGS>" : "?"
            }
        }"#;
	before_parser();

	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	extargs_load_commandline!(parser,loads).unwrap();
	let cmds :Vec<String> = parser.get_sub_commands_ex("").unwrap();
	assert!(check_array_equal(cmds.clone(),format_string_array(vec!["dep", "rdep"])));
	let opts :Vec<ExtKeyParse> = parser.get_cmd_opts_ex("").unwrap();
	assert!(opts.len() == 4);
	let curopt = get_assert_opt(opts.clone(),"$").unwrap();
	assert!(curopt.get_nargs_v() == Nargs::Argtype("*".to_string()));
	let curopt = get_assert_opt(opts.clone(),"verbose").unwrap();
	assert!(curopt.type_name() == KEYWORD_COUNT);
	let curopt = get_assert_opt(opts.clone(),"json").unwrap();
	assert!(curopt.type_name() == KEYWORD_JSONFILE);
	let curopt = get_assert_opt(opts.clone(),"help").unwrap();
	assert!(curopt.type_name() == KEYWORD_HELP);
	let opts = parser.get_cmd_opts_ex("dep").unwrap();
	assert!(opts.len() == 4);
	let curopt = get_assert_opt(opts.clone(),"$").unwrap();
	assert!(curopt.var_name() == "NARGS");
	let curopt = get_assert_opt(opts.clone(),"help").unwrap();
	assert!(curopt.type_name() == KEYWORD_HELP);
	let curopt = get_assert_opt(opts.clone(),"dep_json").unwrap();
	assert!(curopt.type_name() == KEYWORD_JSONFILE);
	let curopt = get_assert_opt(opts.clone(),"dep_new").unwrap();
	assert!(curopt.type_name() == KEYWORD_BOOL);
	return;
}

#[test]
fn test_a024() {
	let loads = r#"        {
            "rdep" : {
                "ip" : {
                    "modules" : [],
                    "called" : true,
                    "setname" : null,
                    "$" : 2
                }
            },
            "dep" : {
                "port" : 5000,
                "cc|C" : true
            },
            "verbose|v" : "+"
        }"#;
	before_parser();

	let params :Vec<String> = format_string_array(vec!["rdep", "ip", "--verbose", "--rdep-ip-modules", "cc", "--rdep-ip-setname", "bb", "xx", "bb"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	extargs_load_commandline!(parser,loads).unwrap();
	let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
	assert!(ns.get_string("subcommand") == "rdep.ip");
	assert!(ns.get_int("verbose") == 1);
	assert!(ns.get_array("rdep_ip_modules") == format_string_array(vec!["cc"]));
	assert!(ns.get_string("rdep_ip_setname") == "bb");
	assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["xx", "bb"])));
	let params :Vec<String> = format_string_array(vec!["dep", "--verbose", "--verbose", "-vvC"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	extargs_load_commandline!(parser,loads).unwrap();
	let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
	assert!(ns.get_string("subcommand") == "dep");
	assert!(ns.get_int("verbose") == 4);
	assert!(ns.get_int("dep_port") == 5000);
	assert!(ns.get_bool("dep_cc") == false);
	assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec![])));
	return;
}

#[test]
fn test_a025() {
	let loads = r#"        {
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
	before_parser();

	let ws = r#"{ "http" : { "url" : "http://www.github.com"} ,"dep":{"list" : ["jsonval1","jsonval2"],"string" : "jsonstring"},"port":6000,"verbose":3}"#;
	let depws = r#"{"list":["depjson1","depjson2"]}"#;
	let rdepws = r#"{"ip": {"list":["rdepjson1","rdepjson3"],"verbose": 5}}"#;
	let f = make_temp_file(ws);
	let depf = make_temp_file(depws);
	let rdepf = make_temp_file(rdepws);
	let depjsonfile = format!("{}",depf.path().display());
	let jsonfile = format!("{}",f.path().display());
	let rdepjsonfile = format!("{}",rdepf.path().display());

	set_env_var("EXTARGSPARSE_JSON",&jsonfile);
	set_env_var("DEP_JSON",&depjsonfile);
	set_env_var("RDEP_JSON", &rdepjsonfile);

	let prioropt = Some(vec![ENV_COMMAND_JSON_SET, ENVIRONMENT_SET, ENV_SUB_COMMAND_JSON_SET]);

	let params :Vec<String> = format_string_array(vec!["-p", "9000", "rdep", "ip", "--rdep-ip-verbose", "--rdep-ip-cc", "ee", "ww"]);
	let parser :ExtArgsParser = ExtArgsParser::new(None,prioropt).unwrap();
	extargs_load_commandline!(parser,loads).unwrap();
	extargs_log_trace!(" ");
	let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
	assert!(ns.get_int("verbose") == 3);
	assert!(ns.get_int("port") == 9000);
	assert!(ns.get_string("dep_string")== "jsonstring");
	assert!(check_array_equal(ns.get_array("dep_list"), format_string_array(vec!["jsonval1", "jsonval2"])));
	assert!(ns.get_bool("http_visual_mode") == false);
	assert!(ns.get_string("http_url")== "http://www.github.com");
	assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["ww"])));
	assert!(ns.get_string("subcommand") == "rdep.ip");
	assert!(ns.get_int("rdep_ip_verbose") == 1);
	assert!(check_array_equal(ns.get_array("rdep_ip_cc"), format_string_array(vec!["ee"])));
	assert!(check_array_equal(ns.get_array("rdep_ip_list"), format_string_array(vec!["rdepjson1", "rdepjson3"])));
	return;
}

fn get_cmd_help(parser :ExtArgsParser, cmdname :&str) -> Vec<String> {
	let mut buf = vec![];
	{
		let mut wstr = BufWriter::new(&mut buf);
		let _ = parser.print_help_ex(&mut wstr , cmdname).unwrap();


	}
	let s = std::str::from_utf8(&buf).unwrap();
	extargs_log_trace!("cmd[{}]help\n{}",cmdname,s);
	let sp :Vec<&str> = s.split("\n").collect();
	let mut retv :Vec<String> = Vec::new();
	for c in sp.iter() {
		retv.push(format!("{}",c));
	}
	return retv;
}

fn get_opt_ok(sarr :Vec<String>, opt :ExtKeyParse) -> bool {
	if opt.flag_name() == KEYWORD_DOLLAR_SIGN {
		return true;
	}
	let mut exprstr :String = "".to_string();
	let mut morethanone :i32 = 0;
	exprstr.push_str(&format!("^\\s+{}",opt.long_opt()));
	if opt.short_opt().len() > 0 {
		exprstr.push_str(&format!("\\|{}",opt.short_opt()));
	}

	match opt.get_nargs_v() {
		Nargs::Argnum(n) => {
			if n > 0 {
				morethanone = 1;
			}
		},
		_ => {}
	}

	if morethanone > 0 {
		exprstr.push_str(&format!("\\s+{}\\s+.*$",opt.opt_dest()));
	} else {
		exprstr.push_str(&format!("\\s+.*$"));
	}

	extargs_log_trace!("exprstr {}",exprstr);
	let ex = Regex::new(&exprstr).unwrap();
	for l in sarr.iter() {
		if ex.is_match(l) {
			return true;
		}
	}
	return false;
}

fn check_all_opts_help(sarr : Vec<String>, opts :Vec<ExtKeyParse>) -> bool {
	for opt in opts.iter() {
		let b = get_opt_ok(sarr.clone(),opt.clone());
		if !b {
			return false;
		}
	}
	return true;
}

#[test]
fn test_a026() {
	let loads = r#"        {
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
	before_parser();
	let s :String = format!("{{ \"{}\" : \"cmd1\" }}", OPT_PROG);
	let opt = ExtArgsOptions::new(&s).unwrap();


	let parser :ExtArgsParser = ExtArgsParser::new(Some(opt.clone()),None).unwrap();
	extargs_load_commandline!(parser,loads).unwrap();
	let sarr = get_cmd_help(parser.clone(),"");
	let opts = parser.get_cmd_opts_ex("").unwrap();
	assert!(check_all_opts_help(sarr.clone(),opts.clone()) == true);

	let sarr = get_cmd_help(parser.clone(),"rdep");
	let opts = parser.get_cmd_opts_ex("rdep").unwrap();
	assert!(check_all_opts_help(sarr.clone(),opts.clone()) == true);

	let sarr = get_cmd_help(parser.clone(),"rdep.ip");
	let opts = parser.get_cmd_opts_ex("rdep.ip").unwrap();
	assert!(check_all_opts_help(sarr.clone(),opts.clone()) == true);

	return;
}