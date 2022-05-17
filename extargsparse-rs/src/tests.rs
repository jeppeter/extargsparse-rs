use super::parser::{ExtArgsParser};
use super::logger::{extargs_debug_out};
use super::argset::{ArgSetImpl};
use super::{extargs_log_trace};
use super::{error_class};
use super::namespace::{NameSpaceEx};
use std::cell::RefCell;
use std::sync::Arc;
use std::error::Error;
use std::boxed::Box;
use regex::Regex;

use extargsparse_codegen::{extargs_load_commandline,ArgSet};


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
					std::env::remove_var(k);
					cont = 1;
					break;
			}
		}
	}
	return;
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
	assert!(check_array_equal(pi.borrow().dep.list.clone(), format_string_array(vec!["arg1", "arg2"])) );
	return;
}