use super::parser::{ExtArgsParser};
use super::logger::{extargs_debug_out,extargs_log_get_timestamp};
use super::argset::{ArgSetImpl};
use super::funccall::{ExtArgsParseFunc};
use super::{extargs_log_trace};
use super::{extargs_error_class};
use super::namespace::{NameSpaceEx};
use super::key::{ExtKeyParse,KEYWORD_DOLLAR_SIGN,Nargs,KEYWORD_COUNT,KEYWORD_JSONFILE,KEYWORD_HELP,KEYWORD_BOOL,KEYWORD_ARGS,KEYWORD_ATTR,KeyAttr,KEYWORD_LIST,KEYWORD_STRING};
use super::options::{ExtArgsOptions,OPT_PROG,OPT_ERROR_HANDLER,OPT_HELP_HANDLER,OPT_SHORT_PREFIX,OPT_LONG_PREFIX,OPT_PARSE_ALL,OPT_HELP_SHORT,OPT_HELP_LONG,OPT_JSON_LONG,OPT_SCREEN_WIDTH,OPT_NO_JSON_OPTION,OPT_NO_HELP_OPTION,OPT_CMD_PREFIX_ADDED,OPT_FLAG_NO_CHANGE};
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
use std::path::Path;
use std::fs;
use std::env;
use serde_json::Value;
use std::io::{Write,BufWriter};


use extargsparse_codegen::{extargs_load_commandline,ArgSet,extargs_map_function};

use super::util_test::{ExtArgsDir,FuncComposer};

extargs_error_class!{TestCaseError}

lazy_static!{
    static ref PATH_SPLIT_CHAR :char = {
        let mut retv :char = '/';
        if env::consts::OS == "windows" {
            retv = '\\';
        }
        retv
    };
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

fn assert_get_cmd(sarr :Vec<String>,cmdname :&str) -> bool {
    let restr :String = format!(r#"^\s+\[{}\]\s+.*"#, cmdname);
    let matchexpr = Regex::new(&restr).unwrap();
    for s in sarr {
        if matchexpr.is_match(&s) {
            return true;
        }
    }
    return false;
}

fn assert_ok_cmds(sarr :Vec<String>, parser :ExtArgsParser,cmdname :&str) -> Result<(),Box<dyn Error>> {
    let cmds = parser.get_sub_commands_ex(cmdname)?;
    for c in cmds.iter() {
        let ok = assert_get_cmd(sarr.clone(),c);
        if !ok {
            extargs_new_error!{TestCaseError,"cannot found [{}] in {:?}", c, cmds}
        }
    }
    Ok(())
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

fn split_string_array(s :&str) -> Vec<String> {
    let sp :Vec<&str> = s.split("\n").collect();
    let mut retv :Vec<String> = Vec::new();
    for c in sp.iter() {
        retv.push(format!("{}",c));
    }
    retv
}

fn get_cmd_help(parser :ExtArgsParser, cmdname :&str) -> Vec<String> {
    let mut buf = vec![];
    {
        let mut wstr = BufWriter::new(&mut buf);
        let _ = parser.print_help_ex(&mut wstr , cmdname).unwrap();
    }
    let s = std::str::from_utf8(&buf).unwrap();
    extargs_log_trace!("cmd[{}]help\n{}",cmdname,s);
    return split_string_array(&s);
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

fn check_opt_bool_help(sarr :Vec<String>, optname :&str, value : bool) -> bool {
    let mut exprstr :String = "".to_string();
    if value {
        exprstr.push_str(&format!("\\s+{}\\s+.*set false default\\(True\\)",optname));
    } else {
        exprstr.push_str(&format!("\\s+{}\\s+.*set true default\\(False\\)",optname));        
    }

    extargs_log_trace!("exprstr [{}]",exprstr);

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
        },
        "panicenable" : true,
        "panicbb" : false
    }"#;
    before_parser();
    let s :String = format!("{{ \"{}\" : \"cmd1\" }}", OPT_PROG);
    let opt = ExtArgsOptions::new(&s).unwrap();


    let parser :ExtArgsParser = ExtArgsParser::new(Some(opt.clone()),None).unwrap();
    extargs_load_commandline!(parser,loads).unwrap();
    let sarr = get_cmd_help(parser.clone(),"");
    let opts = parser.get_cmd_opts_ex("").unwrap();
    assert!(check_all_opts_help(sarr.clone(),opts.clone()) == true);
    assert!(check_opt_bool_help(sarr.clone(),"panicenable",true) == true);
    assert!(check_opt_bool_help(sarr.clone(),"panicbb",false) == true);

    let sarr = get_cmd_help(parser.clone(),"rdep");
    let opts = parser.get_cmd_opts_ex("rdep").unwrap();
    assert!(check_all_opts_help(sarr.clone(),opts.clone()) == true);

    let sarr = get_cmd_help(parser.clone(),"rdep.ip");
    let opts = parser.get_cmd_opts_ex("rdep.ip").unwrap();
    assert!(check_all_opts_help(sarr.clone(),opts.clone()) == true);

    return;
}

#[test]
fn test_a027() {
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
            "list|l!attr=cc;optfunc=list_opt_func!" : [],
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
    let opts = parser.get_cmd_opts_ex("dep").unwrap();
    let mut val :i32 = 0;
    let mut flago :Option<ExtKeyParse> = None;
    for f in opts.iter() {
        if f.type_name() == KEYWORD_ARGS {
            continue;
        }

        if f.flag_name() == "list" {
            val = 1;
            flago = Some(f.clone());
            break;
        }
    }
    assert!(val == 1);
    let flag = flago.unwrap().clone();
    let mut attro :Option<KeyAttr> = None;

    match flag.get_keyattr(KEYWORD_ATTR) {
        Some(v) => {
            attro = Some(v.clone());
        },
        None => {           
        }
    }
    let attr = attro.unwrap().clone();
    assert!(attr.get_attr("attr") == "cc");
    assert!(attr.get_attr("optfunc") == "list_opt_func");
    return;
}

#[test]
fn test_a028() {
    let loads = r#"        {
        "verbose<VAR1>|v" : "+",
        "+http" : {
            "url|u<VAR1>" : "http://www.google.com",
            "visual_mode|V": false
        },
        "$port|p" : {
            "value" : 3000,
            "type" : "int",
            "nargs" : 1 ,
            "helpinfo" : "port to connect"
        },
        "dep" : {
            "list|l!attr=cc;optfunc=list_opt_func!" : [],
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
    let s :String = format!("{{ \"{}\" : \"raise\" }}", OPT_ERROR_HANDLER);
    let opt = ExtArgsOptions::new(&s).unwrap();


    let parser :ExtArgsParser = ExtArgsParser::new(Some(opt.clone()),None).unwrap();
    let params :Vec<String> = format_string_array(vec!["dep", "cc"]);
    extargs_load_commandline!(parser,loads).unwrap();
    let oerr = parser.parse_commandline_ex(Some(params.clone()),None,None,None);
    assert!(oerr.is_err() == true);
    return;
}

#[test]
fn test_a029() {
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
            "list|l!attr=cc;optfunc=list_opt_func!" : [],
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
    let s :String = format!("{{ \"{}\" : \"nohelp\" }}", OPT_HELP_HANDLER);
    let opt = ExtArgsOptions::new(&s).unwrap();


    let parser :ExtArgsParser = ExtArgsParser::new(Some(opt.clone()),None).unwrap();
    extargs_load_commandline!(parser,loads).unwrap();
    let sarr = get_cmd_help(parser,"");
    assert!(check_array_equal(sarr.clone(),format_string_array(vec!["no help information"])));
    return;
}

#[test]
fn test_a030() {
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
        "dep<dep_handler>!opt=cc!" : {
            "list|l!attr=cc;optfunc=list_opt_func!" : [],
            "string|s" : "s_var",
            "$" : "+",
            "ip" : {
                "verbose" : "+",
                "list" : [],
                "cc" : []
            }
        },
        "rdep<rdep_handler>" : {
            "ip" : {
                "verbose" : "+",
                "list" : [],
                "cc" : []
            }
        }
    }"#;
    before_parser();
    let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
    extargs_load_commandline!(parser,loads).unwrap();
    let flag = parser.get_cmd_key_ex("").unwrap().unwrap();
    assert!(flag.cmd_name() == "main");
    assert!(flag.is_cmd() == true);
    assert!(flag.func_name() == "");
    let flag = parser.get_cmd_key_ex("dep").unwrap().unwrap();
    assert!(flag.cmd_name() == "dep");
    assert!(flag.func_name() == "dep_handler");
    let attr = flag.get_keyattr(KEYWORD_ATTR).unwrap();
    assert!(attr.get_attr("opt")== "cc");
    let flag = parser.get_cmd_key_ex("rdep").unwrap().unwrap();
    assert!(flag.cmd_name() == "rdep");
    assert!(flag.func_name() == "rdep_handler");
    let attro = flag.get_keyattr(KEYWORD_ATTR);
    assert!(attro == None);
    let oerr = parser.get_cmd_key_ex("nosuch").unwrap();
    assert!(oerr == None);
    return;
}

#[test]
fn test_a031() {
    let loads = r#"        {
        "verbose|v" : "+",
        "catch|C## to not catch the exception ##" : true,
        "input|i## to specify input default(stdin)##" : null,
        "$caption## set caption ##" : "runcommand",
        "test|t##to test mode##" : false,
        "release|R##to release test mode##" : false,
        "$" : "*"
    }"#;
    before_parser();


    let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
    let params :Vec<String> = format_string_array(vec!["--test"]);
    extargs_load_commandline!(parser,loads).unwrap();
    let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
    assert!(ns.get_bool("test") == true);
    assert!(check_array_equal(ns.get_array("args"), format_string_array(vec![])));
    return;
}

fn get_root_cargo_path() -> String {
    let cfile = format!("{}",file!());
    let cdir = Path::new(&cfile).parent().unwrap();
    let cdname1 = fs::canonicalize(&cdir).unwrap();
    let cdname = cdname1.parent().unwrap().parent().unwrap();
    let bname = cdname.display().to_string();
    return bname;

}

fn get_codegen_path() -> String {
    return format!("{}{}extargsparse_codegen",get_root_cargo_path(),*PATH_SPLIT_CHAR);
}

fn get_worker_path() -> String {
    return format!("{}{}extargsparse_worker",get_root_cargo_path(),*PATH_SPLIT_CHAR);
}

#[test]
fn test_a032() {
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
        "dep<dep_handler>!opt=cc!" : {
            "list|l!attr=cc;optfunc=list_opt_func!" : [],
            "string|s" : "s_var",
            "$" : "+",
            "ip" : {
                "verbose" : "+",
                "list" : [],
                "cc" : []
            }
        },
        "rdep<rdep_handler>" : {
            "ip" : {
                "verbose" : "+",
                "list" : [],
                "cc" : []
            }
        }
    }"#;      
    before_parser();
    let fcomposer : FuncComposer = FuncComposer::new();
    let workdir = get_worker_path();
    let gendir = get_codegen_path();
    let mut compiler :ExtArgsDir = ExtArgsDir::new("callextargs",&workdir,&gendir);
    let importlibs :HashMap<String,String> = HashMap::new();
    let setvars :HashMap<String,String> = HashMap::new();
    let delvars :Vec<String> = Vec::new();

    compiler.write_rust_code("{}",loads,importlibs.clone(),fcomposer.clone(),None,false,"ns","pp").unwrap();
    compiler.compile_command().unwrap();
    let s = compiler.run_command(setvars.clone(),delvars.clone(),format_string_array(vec!["-h"])).unwrap();
    let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
    extargs_load_commandline!(parser,loads).unwrap();
    let opts = parser.get_cmd_opts_ex("").unwrap();
    let sarr = split_string_array(&s);
    assert!(check_all_opts_help(sarr.clone(),opts.clone()) == true);
    let s = compiler.run_command(setvars.clone(),delvars.clone(),format_string_array(vec!["dep","-h"])).unwrap();
    let opts = parser.get_cmd_opts_ex("dep").unwrap();
    let sarr = split_string_array(&s);
    assert!(check_all_opts_help(sarr.clone(),opts.clone()) == true);

    let s = compiler.run_command(setvars.clone(),delvars.clone(),format_string_array(vec!["rdep","-h"])).unwrap();
    let opts = parser.get_cmd_opts_ex("rdep").unwrap();
    let sarr = split_string_array(&s);
    assert!(check_all_opts_help(sarr.clone(),opts.clone()) == true);
    return;
}


fn format_cmd1(k :&str) -> String {
    let mut rets :String = "".to_string();
    rets.push_str("{ \"");
    rets.push_str(k);
    rets.push_str("\" : true }");
    rets
}

fn format_cmd2(k :&str) -> String {
    let mut rets :String = "".to_string();
    rets.push_str("{ \"+");
    rets.push_str(k);
    rets.push_str("\" : { \"reserve\" : true } }");
    rets
}

fn format_cmd3(k :&str) -> String {
    let mut rets :String = "".to_string();
    rets.push_str("{ \"");
    rets.push_str(k);
    rets.push_str("\" : { \"function\" : 30 } }");
    rets
}

#[test]
fn test_a033() {
    let reserve_args = vec!["subcommand", "subnargs", "nargs", "extargs", "args"];
    for c in reserve_args.clone().iter() {
        let cmdline = format_cmd1(c);
        let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
        let berr = extargs_load_commandline!(parser,&cmdline);
        assert!(berr.is_err() == true);
        let cmdline = format_cmd2(c);
        let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
        let berr = extargs_load_commandline!(parser,&cmdline);
        assert!(berr.is_err() == true);
        let cmdline = format_cmd3(c);
        let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
        let berr = extargs_load_commandline!(parser,&cmdline);
        assert!(berr.is_err() == true);
    }
    return;
}

#[test]
fn test_a034() {
    let loads = r#"        {
        "dep" : {
            "string|S" : "stringval"
        }
    }"#;
    before_parser();

    let depws = r#"{"dep_string":null}"#;
    let depf = make_temp_file(depws);
    let depjson = format!("{}",depf.path().display());


    let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
    let params :Vec<String> = format_string_array(vec!["--json", &depjson, "dep"]);
    extargs_load_commandline!(parser,loads).unwrap();
    let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
    assert!(ns.get_string("dep_string") == "");
    assert!(ns.get_string("subcommand") == "dep");
    assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec![])));
    return;
}

#[test]
fn test_a035() {
    let loads = r#"        {
        "float1|f" : 3.633 ,
        "float2" : 6422.22,
        "float3" : 44463.23,
        "verbose|v" : "+",
        "dep" : {
            "float3" : 3332.233
        },
        "rdep" : {
            "ip" : {
                "float4" : 3377.33,
                "float6" : 33.22,
                "float7" : 0.333
            }
        }

    }"#;
    before_parser();

    let depws = r#"{"float3":33.221}"#;
    let depf = make_temp_file(depws);
    let depjson = format!("{}",depf.path().display());

    let rdepws = r#"{"ip" : { "float4" : 40.3}}"#;
    let rdepf = make_temp_file(rdepws);
    let rdepjson = format!("{}",rdepf.path().display());

    let ws = r#"{"verbose": 30,"float3": 77.1}"#;
    let f = make_temp_file(ws);
    let jsonfile = format!("{}",f.path().display());


    let rdepipws = r#"{"float7" : 11.22,"float4" : 779.2}"#;
    let rdepipf = make_temp_file(rdepipws);
    let rdepipjson = format!("{}",rdepipf.path().display());

    set_env_var("EXTARGSPARSE_JSON",&jsonfile);
    set_env_var("DEP_JSON",&depjson);
    set_env_var("RDEP_JSON",&rdepjson);
    set_env_var("DEP_FLOAT3","33.52");
    set_env_var("RDEP_IP_FLOAT7", "99.3");


    let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
    let params :Vec<String> = format_string_array(vec!["-vvfvv", "33.21", "rdep", "ip", "--json", &jsonfile, "--rdep-ip-json", &rdepipjson]);
    extargs_load_commandline!(parser,loads).unwrap();
    let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
    assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec![])));
    assert!(ns.get_string("subcommand") == "rdep.ip");
    assert!(ns.get_int("verbose") == 4);
    assert!(ns.get_float("float1") == 33.21);
    assert!(ns.get_float("dep_float3") == 33.52);
    assert!(ns.get_float("float2") == 6422.22);
    assert!(ns.get_float("float3") == 77.1);
    assert!(ns.get_float("rdep_ip_float4") == 779.2);
    assert!(ns.get_float("rdep_ip_float6") == 33.22);
    assert!(ns.get_float("rdep_ip_float7") == 11.22);
    return;
}

#[test]
fn test_a037() {
    let loads = r#"        {
        "jsoninput|j##input json default stdin##" : null,
        "input|i##input file to get default nothing - for stdin##" : null,
        "output|o##output c file##" : null,
        "verbose|v##verbose mode default(0)##" : "+",
        "cmdpattern|c" : "%EXTARGS_CMDSTRUCT%",
        "optpattern|O" : "%EXTARGS_STRUCT%",
        "structname|s" : "args_options_t",
        "funcname|F" : "debug_extargs_output",
        "releasename|R" : "release_extargs_output",
        "funcpattern" : "%EXTARGS_DEBUGFUNC%",
        "prefix|p" : "",
        "test" : {
            "$" : 0
        },
        "optstruct" : {
            "$" : 0
        },
        "cmdstruct" : {
            "$" : 0
        },
        "debugfunc" : {
            "$" : 0
        },
        "all" : {
            "$" : 0
        }
    }"#;
    before_parser();



    let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
    extargs_load_commandline!(parser,loads).unwrap();
    let subcmds = parser.get_sub_commands_ex("").unwrap();
    assert!(subcmds.len() == 5);
    assert!(subcmds[0] == "all");
    assert!(subcmds[1] == "cmdstruct");
    assert!(subcmds[2] == "debugfunc");
    assert!(subcmds[3] == "optstruct");
    assert!(subcmds[4] == "test");
    let opts = parser.get_cmd_opts_ex("").unwrap();
    assert!(opts.len() == 14);
    assert!(opts[0].flag_name() == "$");
    assert!(opts[1].long_opt() == "--cmdpattern");
    assert!(opts[2].opt_dest() == "funcname");
    assert!(opts[3].var_name() == "funcpattern");
    assert!(opts[4].type_name() == KEYWORD_HELP);
    return;
}

#[test]
fn test_a038() {
    let loads = r#"        {
        "verbose|v" : "+",
        "kernel|K" : "/boot/",
        "initrd|I" : "/boot/",
        "encryptfile|e" : null,
        "encryptkey|E" : null,
        "setupsectsoffset" : 0x1f1,
        "ipxe<ipxe_handler>" : {
            "$" : "+"
        }
    }"#;
    before_parser();


    let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
    let berr = extargs_load_commandline!(parser,loads);
    assert!(berr.is_err() == true);
    return;
}

#[test]
fn test_a039() {
    let loads = r#"        {
        "verbose|v" : "+",
        "kernel|K" : "/boot/",
        "initrd|I" : "/boot/",
        "encryptfile|e" : null,
        "encryptkey|E" : null,
        "setupsectsoffset" : 451
    }"#;
    before_parser();
    set_env_var("EXTARGS_VERBOSE", "4");
    set_env_var("EXTARGS_SETUPSECTSOFFSET", "0x612");


    let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
    let params :Vec<String> = format_string_array(vec![]);
    extargs_load_commandline!(parser,loads).unwrap();
    let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
    assert!(ns.get_int("verbose") == 4);
    assert!(ns.get_int("setupsectsoffset") == 0x612);
    return;
}

#[derive(ArgSet)]
struct ParserTest40 {
    tce_mirror       :String,
    tce_root         :String,
    tce_tceversion   :String,
    tce_wget         :String,
    tce_cat          :String,
    tce_rm           :String,
    tce_sudoprefix   :String,
    tce_optional_dir :String,
    tce_trymode      :bool,
    tce_platform     :String,
    tce_mount        :String,
    tce_umount       :String,
    tce_chroot       :String,
    tce_chown        :String,
    tce_mkdir        :String,
    tce_rollback     :bool,
    tce_cp           :String,
    tce_jsonfile     :String,
    tce_perspace     :i64,
    tce_depmapfile   :String,
    tce_timeout      :i64,
    tce_listsfile    :String,
    tce_maxtries     :i64,
    args             :Vec<String>,
}

#[test]
fn test_a040() {
   let loads = r#"        {
    "+tce": {
        "mirror": "http://repo.tinycorelinux.net",
        "root": "/",
        "tceversion": "7.x",
        "wget": "wget",
        "cat": "cat",
        "rm": "rm",
        "sudoprefix": "sudo",
        "optional_dir": "/cde",
        "trymode": false,
        "platform": "x86_64",
        "mount": "mount",
        "umount": "umount",
        "chroot": "chroot",
        "chown": "chown",
        "chmod": "chmod",
        "mkdir": "mkdir",
        "rollback": true,
        "cp": "cp",
        "jsonfile": null,
        "perspace": 3,
        "depmapfile": null,
        "timeout": 10,
        "listsfile": null,
        "maxtries": 5
    }
}"#;
before_parser();
let params :Vec<String> = format_string_array(vec!["--tce-root", "/home/"]);
let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
extargs_load_commandline!(parser,loads).unwrap();
let p :ParserTest40 = ParserTest40::new();
let pi :Arc<RefCell<ParserTest40>> = Arc::new(RefCell::new(p));
let _ns = parser.parse_commandline_ex(Some(params.clone()),None,Some(pi.clone()),None).unwrap();
assert!(pi.borrow().tce_mirror == "http://repo.tinycorelinux.net");
assert!(pi.borrow().tce_root == "/home/");
assert!(pi.borrow().tce_listsfile == "");
assert!(pi.borrow().tce_maxtries == 5);
assert!(pi.borrow().tce_timeout == 10);
return;
}

#[test]
fn test_a041() {
    let rootd = get_root_cargo_path();
    let mut fdir :String =format!("{}{}extargsparse_worker{}certs",rootd,*PATH_SPLIT_CHAR,*PATH_SPLIT_CHAR);
    if *PATH_SPLIT_CHAR == '\\' {
        fdir = fdir.replace("\\","\\\\");
    }
    let loads = format!(r#"        {{            "countryname|N" : "CN",
        "statename|S" : "ZJ",
        "localityname" : "HZ",
        "organizationname|O" : ["BT"],
        "organizationunitname" : "BT R&D",
        "commonname|C" : "bingte.com",
        "+ssl" : {{
            "chain" : true,
            "dir" : "{}",
            "bits" : 4096,
            "md" : "sha256",
            "utf8" : true,
            "name" : "ipxe",
            "days" : 3650,
            "crl-days": 365,
            "emailaddress" : "bt@bingte.com",
            "aia_url" : "http://bingte.com/sec/aia",
            "crl_url" : "http://bingte.com/sec/crl",
            "ocsp_url" : "http://bingte.com/sec/ocsp",
            "dns_url" : ["bingte.com"],
            "excluded_ip" : ["0.0.0.0/0.0.0.0","0:0:0:0:0:0:0:0/0:0:0:0:0:0:0:0"],
            "password|P" : null,
            "copy_extensions" : "none",
            "subca" : false,
            "comment": ""
        }}
    }}"#,fdir);
    before_parser();

    let ws = r#"{"emailaddress" : "unit@bingte.com","organizationname" : "BT RD","ssl" :{ "dir" : "./certs/bingte","name" : "bingte","subca" : true,"copy_extensions" : "copy","days" : 375,"crl_days" : 30,"bits" : 4096}}"#;
    let f = make_temp_file(ws);
    let jsonfile = format!("{}",f.path().display());

    let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
    let params :Vec<String> = format_string_array(vec!["--json", &jsonfile]);
    extargs_load_commandline!(parser,&loads).unwrap();
    let berr = parser.parse_commandline_ex(Some(params.clone()),None,None,None);
    assert!(berr.is_err() == true);
    return;
}


#[test]
fn test_a042() {
   let loads = r#"        {
    "verbose|v" : "+",
    "kernel|K" : "/boot/",
    "initrd|I" : "/boot/",
    "encryptfile|e" : null,
    "encryptkey|E" : null,
    "setupsectsoffset" : 663,
    "ipxe" : {
        "$" : "+"
    }
}"#;
before_parser();
let params :Vec<String> = format_string_array(vec!["-vvvK", "kernel", "--initrd", "initrd", "cc", "dd", "-E", "encryptkey", "-e", "encryptfile", "ipxe"]);
let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
extargs_load_commandline!(parser,loads).unwrap();
let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
assert!(ns.get_string("subcommand") == "ipxe");
assert!(check_array_equal(ns.get_array("subnargs"),format_string_array(vec!["cc", "dd"])));
return;
}

#[test]
fn test_a043() {
   let loads = r#"        {
    "verbose|v" : "+",
    "kernel|K" : "/boot/",
    "initrd|I" : "/boot/",
    "encryptfile|e" : null,
    "encryptkey|E" : null,
    "setupsectsoffset" : 663,
    "ipxe" : {
        "$" : "+"
    }
}"#;
before_parser();
let optstr :String = format!(r#"{{"{}": true,"{}" : "-", "{}" : "-"}}"#,OPT_PARSE_ALL,OPT_LONG_PREFIX,OPT_SHORT_PREFIX);
let optref :ExtArgsOptions = ExtArgsOptions::new(&optstr).unwrap();
let params :Vec<String> = format_string_array(vec!["-K", "kernel", "-initrd", "initrd", "cc", "dd", "-E", "encryptkey", "-e", "encryptfile", "ipxe"]);
let parser :ExtArgsParser = ExtArgsParser::new(Some(optref.clone()),None).unwrap();
extargs_load_commandline!(parser,loads).unwrap();
let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
assert!(ns.get_string("subcommand") == "ipxe");
assert!(check_array_equal(ns.get_array("subnargs"),format_string_array(vec!["cc", "dd"])));
return;
}

#[test]
fn test_a044() {
   let loads = r#"        {
    "verbose|v" : "+",
    "kernel|K" : "/boot/",
    "initrd|I" : "/boot/",
    "encryptfile|e" : null,
    "encryptkey|E" : null,
    "setupsectsoffset" : 663,
    "ipxe" : {
        "$" : "+"
    }
}"#;
before_parser();
let optstr :String = format!(r#"{{"{}": true,"{}" : "++", "{}" : "+"}}"#,OPT_PARSE_ALL,OPT_LONG_PREFIX,OPT_SHORT_PREFIX);
let optref :ExtArgsOptions = ExtArgsOptions::new(&optstr).unwrap();
let params :Vec<String> = format_string_array(vec!["+K", "kernel", "++initrd", "initrd", "cc", "dd", "+E", "encryptkey", "+e", "encryptfile", "ipxe"]);
let parser :ExtArgsParser = ExtArgsParser::new(Some(optref.clone()),None).unwrap();
extargs_load_commandline!(parser,loads).unwrap();
let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
assert!(ns.get_string("subcommand") == "ipxe");
assert!(check_array_equal(ns.get_array("subnargs"),format_string_array(vec!["cc", "dd"])));
return;
}

fn debug_set_2_args(ns :NameSpaceEx, validx :i32, keycls :ExtKeyParse, params :Vec<String>) -> Result<i32,Box<dyn Error>> {
    let mut sarr :Vec<String>;
    extargs_log_trace!("validx [{}]",validx);
    if (validx + 2) > params.len() as i32 {
        extargs_new_error!{TestCaseError,"[{}+2] > len({}) {:?}",validx,params.len(),params}
    }

    sarr = ns.get_array(&keycls.opt_dest());
    sarr.push(format!("{}",params[validx as usize]));
    sarr.push(format!("{}",params[(validx + 1) as usize]));
    extargs_log_trace!("set [{}] value {:?}", keycls.opt_dest(),sarr);
    ns.set_array(&keycls.opt_dest(),sarr)?;
    return Ok(2);
}

fn debug_set_2_args_upper(ns :NameSpaceEx, validx :i32, keycls :ExtKeyParse, params :Vec<String>) -> Result<i32,Box<dyn Error>> {
    let mut sarr :Vec<String>;
    extargs_log_trace!("validx [{}]",validx);
    if (validx + 2) > params.len() as i32 {
        extargs_new_error!{TestCaseError,"[{}+2] > len({}) {:?}",validx,params.len(),params}
    }

    sarr = ns.get_array(&keycls.opt_dest());
    sarr.push(format!("{}",params[validx as usize]).to_uppercase());
    sarr.push(format!("{}",params[(validx + 1) as usize]).to_uppercase());
    extargs_log_trace!("set [{}] value {:?}", keycls.opt_dest(),sarr);
    ns.set_array(&keycls.opt_dest(),sarr)?;
    return Ok(2);
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



#[test]
#[extargs_map_function(optparse=debug_set_2_args)]
fn test_a045() {
   let loads = r#"        {
    "verbose|v" : "+",
    "kernel|K" : "/boot/",
    "initrd|I" : "/boot/",
    "pair|P!optparse=debug_set_2_args!" : [],
    "encryptfile|e" : null,
    "encryptkey|E" : null,
    "setupsectsoffset" : 663,
    "ipxe" : {
        "$" : "+"
    }
}"#;
before_parser();
let optstr :String = format!(r#"{{"{}": true,"{}" : "++", "{}" : "+"}}"#,OPT_PARSE_ALL,OPT_LONG_PREFIX,OPT_SHORT_PREFIX);
let optref :ExtArgsOptions = ExtArgsOptions::new(&optstr).unwrap();
let params :Vec<String> = format_string_array(vec!["+K", "kernel", "++pair", "initrd", "cc", "dd", "+E", "encryptkey", "+e", "encryptfile", "ipxe"]);
let parser :ExtArgsParser = ExtArgsParser::new(Some(optref.clone()),None).unwrap();
extargs_load_commandline!(parser,loads).unwrap();
let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
assert!(ns.get_string("subcommand") == "ipxe");
assert!(check_array_equal(ns.get_array("pair"),format_string_array(vec!["initrd", "cc"])));
assert!(check_array_equal(ns.get_array("subnargs"),format_string_array(vec!["dd"])));
return;
}

#[test]
#[extargs_map_function(optparse=debug_set_2_args,opthelp=debug_opthelp_set)]
fn test_a046() {
   let loads = r#"        {
    "verbose|v" : "+",
    "kernel|K" : "/boot/",
    "initrd|I" : "/boot/",
    "pair|P!optparse=debug_set_2_args;opthelp=debug_opthelp_set!" : [],
    "encryptfile|e" : null,
    "encryptkey|E" : null,
    "setupsectsoffset" : 663,
    "ipxe" : {
        "$" : "+"
    }
}"#;
before_parser();
let optstr :String = format!(r#"{{"{}": true,"{}" : "++", "{}" : "+"}}"#,OPT_PARSE_ALL,OPT_LONG_PREFIX,OPT_SHORT_PREFIX);
let optref :ExtArgsOptions = ExtArgsOptions::new(&optstr).unwrap();
let parser :ExtArgsParser = ExtArgsParser::new(Some(optref.clone()),None).unwrap();
extargs_load_commandline!(parser,loads).unwrap();
let sarr = get_cmd_help(parser.clone(),"");
let expr = Regex::new(r#".*opthelp function set \[pair\].*"#).unwrap();
let mut bmatch :bool =false;
for l in sarr.iter() {
    if expr.is_match(l) {
        bmatch = true;
        break;
    }
}
assert!(bmatch == true);
return;
}

#[test]
fn test_a047() {
    let loads = r#"        {
        "verbose|v" : "+",
        "kernel|K" : "/boot/",
        "initrd|I" : "/boot/",
        "pair|P!optparse=debug_set_2_args;opthelp=debug_opthelp_set!" : [],
        "encryptfile|e" : null,
        "encryptkey|E" : null,
        "setupsectsoffset" : 663,
        "ipxe" : {
            "$" : "+"
        }
    }"#;
    before_parser();
    let optstr :String = format!(r#"{{"{}": true,"{}" : "++", "{}" : "+", "{}" : "?", "{}" : "usage" ,"{}" : "jsonfile"}}"#,OPT_PARSE_ALL,OPT_LONG_PREFIX,OPT_SHORT_PREFIX,OPT_HELP_SHORT,OPT_HELP_LONG,OPT_JSON_LONG);
    let codstr =r#"
    use serde_json::Value;
    use extargsparse_worker::key::ExtKeyParse;

    extargs_error_class!{TestCaseError}

    macro_rules! extargs_log_trace {
        ($($arg:tt)+) => {
            let mut _c :String= format!("[{}:{}] ",file!(),line!());
            _c.push_str(&(format!($($arg)+)[..]));
            println!("{}",_c);
        }
    }

    fn debug_set_2_args(ns :NameSpaceEx, validx :i32, keycls :ExtKeyParse, params :Vec<String>) -> Result<i32,Box<dyn Error>> {
        let mut sarr :Vec<String>;
        extargs_log_trace!("validx [{}]",validx);
        if (validx + 2) > params.len() as i32 {
            extargs_new_error!{TestCaseError,"[{}+2] > len({}) {:?}",validx,params.len(),params}
        }

        sarr = ns.get_array(&keycls.opt_dest());
        sarr.push(format!("{}",params[validx as usize]));
        sarr.push(format!("{}",params[(validx + 1) as usize]));
        extargs_log_trace!("set [{}] value {:?}", keycls.opt_dest(),sarr);
        ns.set_array(&keycls.opt_dest(),sarr)?;
        return Ok(2);
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

    "#;
    let mapstr = r#"optparse=debug_set_2_args,opthelp=debug_opthelp_set"#;
    let mut fcomposer : FuncComposer = FuncComposer::new();
    fcomposer.add_code(codstr);
    fcomposer.add_inner(mapstr);

    let workdir = get_worker_path();
    let gendir = get_codegen_path();
    let mut compiler :ExtArgsDir = ExtArgsDir::new("callextargs",&workdir,&gendir);
    let mut importlibs :HashMap<String,String> = HashMap::new();
    let setvars :HashMap<String,String> = HashMap::new();
    let delvars :Vec<String> = Vec::new();

    importlibs.insert(format!("serde_json"),format!("^1.0.42"));
    compiler.write_rust_code(&optstr,loads,importlibs.clone(),fcomposer.clone(),None,false,"args","ppc").unwrap();
    compiler.compile_command().unwrap();
    let s = compiler.run_command(setvars.clone(),delvars.clone(),format_string_array(vec!["++usage"])).unwrap();
    let sarr = split_string_array(&s);
    let expr = Regex::new(r#".*opthelp function set \[pair\].*"#).unwrap();
    let mut bmatch :bool =false;
    for l in sarr.iter() {
        if expr.is_match(l) {
            bmatch = true;
            break;
        }
    }
    assert!(bmatch == true);
    return;
}

#[test]
fn test_a048() {
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
    let optstr :String = format!(r#"{{ "{}" : "jsonfile" }}"#,OPT_JSON_LONG);
    let ws = r#"{"dep":{"list" : ["jsonval1","jsonval2"],"string" : "jsonstring"},"port":6000,"verbose":3}"#;
    let f = make_temp_file(ws);
    let jsonfile = format!("{}",f.path().display());
    let depws = r#"{"list":["depjson1","depjson2"]}"#;
    let depf = make_temp_file(depws);
    let depjsonfile = format!("{}",depf.path().display());
    let depstrval = "newval";
    let depliststr = r#"["depenv1","depenv2"]"#;
    set_env_var("EXTARGSPARSE_JSONFILE", &jsonfile);
    set_env_var("DEP_JSONFILE", &depjsonfile);
    let optref :ExtArgsOptions = ExtArgsOptions::new(&optstr).unwrap();
    let params :Vec<String> = format_string_array(vec!["-p", "9000", "dep", "--dep-string", "ee", "ww"]);
    let vint :Vec<i32> = vec![ENV_COMMAND_JSON_SET, ENVIRONMENT_SET, ENV_SUB_COMMAND_JSON_SET];
    let parser :ExtArgsParser = ExtArgsParser::new(Some(optref.clone()),Some(vint.clone())).unwrap();
    extargs_load_commandline!(parser,loads).unwrap();
    set_env_var("DEP_STRING",depstrval);
    set_env_var("DEP_LIST",depliststr);
    let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
    assert!(ns.get_int("verbose") == 3);
    assert!(ns.get_int("port") == 9000);
    assert!(ns.get_string("subcommand") == "dep");
    assert!(check_array_equal(ns.get_array("dep_list"), format_string_array(vec!["jsonval1", "jsonval2"])));
    assert!(ns.get_string("dep_string") == "ee");
    assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["ww"])));
    return;
}


#[test]
fn test_a049() {
    let loads = r#"        {
        "verbose|v##very long very long very long very long very long very long very long very long very long very long very long very long very long very long very long very long very long very long##" : "+",
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
    let optstr :String = format!(r#"{{ "{}" : 60 }}"#,OPT_SCREEN_WIDTH);
    let optref :ExtArgsOptions = ExtArgsOptions::new(&optstr).unwrap();
    let parser :ExtArgsParser = ExtArgsParser::new(Some(optref.clone()),None).unwrap();
    extargs_load_commandline!(parser,loads).unwrap();
    let sarr = get_cmd_help(parser,"");
    let mut overlength : bool = false;
    let mut idx :i32 = 0;
    for l in sarr.iter() {
        if l.len() > 64 &&  idx > 0 {
            overlength = true;
            break;
        }
        idx += 1;
    }
    assert!(overlength == false);
    let optstr :String = format!(r#"{{ "{}" : 80 }}"#,OPT_SCREEN_WIDTH);
    let optref :ExtArgsOptions = ExtArgsOptions::new(&optstr).unwrap();
    let parser :ExtArgsParser = ExtArgsParser::new(Some(optref.clone()),None).unwrap();
    extargs_load_commandline!(parser,loads).unwrap();
    let sarr = get_cmd_help(parser,"");
    let mut overlength : bool = false;
    let mut idx :i32 = 0;
    for l in sarr.iter() {
        if l.len() > 64 &&  idx > 0 {
            overlength = true;
            break;
        }
        idx += 1;
    }
    assert!(overlength == true);
    return;
}

#[test]
fn test_a050() {
    let loads = r#"        {
        "verbose|v" : "+",
        "dep" : {
            "list|l" : [],
            "string|s" : "s_var",
            "$" : "+"
        }
    }"#;
    before_parser();
    let optstr :String = format!(r#"{{ "{}" : "usage" , "{}" : "?" , "{}" : "++", "{}" : "+" }}"#,OPT_HELP_LONG,OPT_HELP_SHORT,OPT_LONG_PREFIX,OPT_SHORT_PREFIX);
    let optref :ExtArgsOptions = ExtArgsOptions::new(&optstr).unwrap();
    let parser :ExtArgsParser = ExtArgsParser::new(Some(optref.clone()),None).unwrap();
    extargs_load_commandline!(parser,loads).unwrap();
    let sarr = get_cmd_help(parser,"");
    let expr = Regex::new(r#"^\s+\+\+usage|\+\?\s+to display.*"#).unwrap();
    let mut bmatch :bool =false;
    for l in sarr.iter() {
        if expr.is_match(l) {
            bmatch = true;
            break;
        }
    }
    assert!(bmatch == true);
    return;
}

#[test]
fn test_a051() {
    let loads = r#"        {
        "verbose|v" : "+",
        "dep" : {
            "list|l" : [],
            "string|s" : "s_var",
            "$" : "+"
        }
    }"#;
    before_parser();
    let optstr :String = format!(r#"{{ "{}" : "usage" , 
        "{}" : null , 
        "{}" : "++", 
        "{}" : "+" }}"#,
        OPT_HELP_LONG,OPT_HELP_SHORT,OPT_LONG_PREFIX,OPT_SHORT_PREFIX);
    let optref :ExtArgsOptions = ExtArgsOptions::new(&optstr).unwrap();
    let parser :ExtArgsParser = ExtArgsParser::new(Some(optref.clone()),None).unwrap();
    extargs_load_commandline!(parser,loads).unwrap();
    let sarr = get_cmd_help(parser,"");
    let expr = Regex::new(r#"^\s+\+\+usage\|\+\?\s+to display.*"#).unwrap();
    let mut bmatch :bool =false;
    for l in sarr.iter() {
        if expr.is_match(l) {
            bmatch = true;
            break;
        }
    }
    assert!(bmatch == false);
    return;
}

#[test]
fn test_a052() {
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
    let optstr :String = format!(r#"{{ "{}" : true , 
        "{}" : true}}"#,
        OPT_NO_JSON_OPTION,OPT_NO_HELP_OPTION);
    let depstrval = "newval";
    let depliststr = r#"["depenv1","depenv2"]"#;
    let ws = r#"{"dep":{"list" : ["jsonval1","jsonval2"],"string" : "jsonstring"},"port":6000,"verbose":3}"#;
    let f = make_temp_file(ws);
    let jsonfile = format!("{}",f.path().display());
    let depws = r#"{"list":["depjson1","depjson2"]}"#;
    let depf = make_temp_file(depws);
    let depjsonfile = format!("{}", depf.path().display());
    set_env_var("EXTARGSPARSE_JSONFILE",&jsonfile);
    set_env_var("DEP_JSONFILE",&depjsonfile);
    let optref :ExtArgsOptions = ExtArgsOptions::new(&optstr).unwrap();
    let vint :Vec<i32> = vec![ENV_COMMAND_JSON_SET, ENVIRONMENT_SET, ENV_SUB_COMMAND_JSON_SET];
    let parser :ExtArgsParser = ExtArgsParser::new(Some(optref.clone()),Some(vint.clone())).unwrap();
    extargs_load_commandline!(parser,loads).unwrap();
    set_env_var("DEP_STRING",depstrval);
    set_env_var("DEP_LIST",depliststr);
    let sarr = get_cmd_help(parser.clone(),"");
    let helpexpr = Regex::new(r#"^\s+--help.*"#).unwrap();
    let jsonexpr = Regex::new(r#"^\s+--json.*"#).unwrap();
    let mut helpok :bool =false;
    let mut jsonok :bool = false;
    for l in sarr.iter() {
        if helpexpr.is_match(l) {
            helpok = true;
        }
        if jsonexpr.is_match(l) {
            jsonok = true;
        }
    }
    assert!(helpok == false);
    assert!(jsonok == false);
    let params :Vec<String> = format_string_array(vec!["-p", "9000", "dep", "--dep-string", "ee", "ww"]);
    let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
    assert!(ns.get_int("verbose") == 0);
    assert!(ns.get_int("port") == 9000);
    assert!(ns.get_string("subcommand") == "dep");
    assert!(check_array_equal(ns.get_array("dep_list"), format_string_array(vec!["depenv1", "depenv2"])));
    assert!(ns.get_string("dep_string") == "ee");
    assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["ww"])));

    return;
}

#[test]
fn test_a053() {
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
        },
        "rdep" : {
            "list|l" : [],
            "string|s" : "s_rdep",
            "$" : "+"
        }
    }"#;
    before_parser();
    let optstr :String = format!(r#"{{ "{}" : false }}"#,
        OPT_CMD_PREFIX_ADDED);
    let depstrval = "newval";
    let depliststr = r#"["depenv1","depenv2"]"#;
    let ws = r#"{"list" : ["jsonval1","jsonval2"],"string" : "jsonstring","port":6000,"verbose":3}"#;
    let f = make_temp_file(ws);
    let jsonfile = format!("{}",f.path().display());
    let depws = r#"{"list":["depjson1","depjson2"]}"#;
    let depf = make_temp_file(depws);
    let depjsonfile = format!("{}", depf.path().display());
    set_env_var("EXTARGSPARSE_JSON",&jsonfile);
    set_env_var("DEP_JSON",&depjsonfile);
    let optref :ExtArgsOptions = ExtArgsOptions::new(&optstr).unwrap();
    let vint :Vec<i32> = vec![ENV_COMMAND_JSON_SET, ENVIRONMENT_SET, ENV_SUB_COMMAND_JSON_SET];
    let parser :ExtArgsParser = ExtArgsParser::new(Some(optref.clone()),Some(vint.clone())).unwrap();
    extargs_load_commandline!(parser,loads).unwrap();
    set_env_var("DEP_STRING",depstrval);
    set_env_var("DEP_LIST",depliststr);
    let sarr = get_cmd_help(parser.clone(),"dep");
    let helpexpr = Regex::new(r#"^\s+--help.*"#).unwrap();
    let jsonexpr = Regex::new(r#"^\s+--dep-json.*"#).unwrap();
    let listexpr = Regex::new(r#"^\s+--list.*"#).unwrap();
    let stringexpr = Regex::new(r#"^\s+--string.*"#).unwrap();
    let mut helpok :bool =false;
    let mut jsonok :bool = false;
    let mut listok :bool =false;
    let mut stringok :bool = false;

    for l in sarr.iter() {
        if helpexpr.is_match(l) {
            helpok = true;
        }
        if jsonexpr.is_match(l) {
            jsonok = true;
        }

        if listexpr.is_match(l) {
            listok = true;
        }

        if stringexpr.is_match(l) {
            stringok = true;
        }
    }
    assert!(helpok == true);
    assert!(jsonok == true);
    assert!(listok == true);
    assert!(stringok == true); 

    let sarr = get_cmd_help(parser.clone(),"rdep");
    let jsonexpr = Regex::new(r#"^\s+--rdep-json.*"#).unwrap();
    helpok = false;
    jsonok = false;
    listok = false;
    stringok = false;

    for l in sarr.iter() {
        if helpexpr.is_match(l) {
            helpok = true;
        }
        if jsonexpr.is_match(l) {
            jsonok = true;
        }

        if listexpr.is_match(l) {
            listok = true;
        }

        if stringexpr.is_match(l) {
            stringok = true;
        }
    }
    assert!(helpok == true);
    assert!(jsonok == true);
    assert!(listok == true);
    assert!(stringok == true); 


    let params :Vec<String> = format_string_array(vec!["-p", "9000", "dep", "--string", "ee", "ww"]);
    let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
    assert!(ns.get_int("verbose") == 3);
    assert!(ns.get_int("port") == 9000);
    assert!(ns.get_string("subcommand") == "dep");
    assert!(check_array_equal(ns.get_array("list"), format_string_array(vec!["jsonval1", "jsonval2"])));
    assert!(ns.get_string("string") == "ee");
    assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["ww"])));

    return;
}

#[test]
fn test_a054() {
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
    let optstr :String = format!(r#"{{ "{}" : "jsonfile" }}"#,
        OPT_JSON_LONG);
    let depstrval = "newval";
    let depliststr = r#"["depenv1","depenv2"]"#;
    let ws = r#"{"dep":{"list" : ["jsonval1","jsonval2"],"string" : "jsonstring"},"port":6000,"verbose":3}"#;
    let f = make_temp_file(ws);
    let jsonfile = format!("{}",f.path().display());
    let depws = r#"{"list":["depjson1","depjson2"]}"#;
    let depf = make_temp_file(depws);
    let depjsonfile = format!("{}", depf.path().display());
    set_env_var("EXTARGSPARSE_JSONFILE",&jsonfile);
    set_env_var("DEP_JSONFILE",&depjsonfile);
    let optref :ExtArgsOptions = ExtArgsOptions::new(&optstr).unwrap();
    let vint :Vec<i32> = vec![ENV_COMMAND_JSON_SET, ENVIRONMENT_SET, ENV_SUB_COMMAND_JSON_SET];
    let parser :ExtArgsParser = ExtArgsParser::new(Some(optref.clone()),Some(vint.clone())).unwrap();
    extargs_load_commandline!(parser,loads).unwrap();
    set_env_var("DEP_STRING",depstrval);
    set_env_var("DEP_LIST",depliststr);

    let params :Vec<String> = format_string_array(vec!["--jsonfile", &jsonfile, "dep", "ww"]);
    let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
    assert!(ns.get_int("verbose") == 3);
    assert!(ns.get_int("port") == 6000);
    assert!(ns.get_string("subcommand") == "dep");
    assert!(check_array_equal(ns.get_array("dep_list"), format_string_array(vec!["jsonval1", "jsonval2"])));
    assert!(ns.get_string("dep_string") == "jsonstring");
    assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["ww"])));
    return;
}

#[allow(unused_assignments)]
fn debug_2_jsonfunc(ns :NameSpaceEx, keycls :ExtKeyParse, value :Value) -> Result<(),Box<dyn Error>> {
    let mut vs :Vec<String> = Vec::new();
    let mut setvs :Vec<String> = Vec::new();
    let mut idx :usize=0;

    if !keycls.is_flag() || keycls.type_name() != KEYWORD_LIST {
        extargs_new_error!{TestCaseError,"keycls [{}] not valid", keycls.string()}
    }

    match value {
        Value::Null => {
            vs = Vec::new();
        },
        Value::Array(_a) => {
            vs = Vec::new();
            idx = 0;
            for _i in _a.iter() {
                match _i {
                    Value::String(_s) => {
                        vs.push(format!("{}",_s));
                    },
                    _ => {
                        extargs_new_error!{TestCaseError,"at [{}] not valid [{:?}]", idx,_i}
                    }
                }
                idx += 1;
            }
        },
        _ => { extargs_new_error!{TestCaseError,"value [{:?}] not valid" , value } }
    }


    if (vs.len() % 2) != 0 {
        extargs_new_error!{TestCaseError,"{:?} not event size", vs}
    }

    setvs = Vec::new();
    idx = 0;
    while idx < vs.len() {
        setvs.push(format!("{}",vs[idx]));
        idx += 2;
    }

    return ns.set_array(&keycls.opt_dest(), setvs.clone());
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

#[test]
#[extargs_map_function(jsonfunc=debug_2_jsonfunc,jsonfunc=debug_upper_jsonfunc)]
fn test_a055() {
    let loads = r#"        {
        "verbose|v" : "+",
        "$port|p" : {
            "value" : 3000,
            "type" : "int",
            "nargs" : 1 ,
            "helpinfo" : "port to connect"
        },
        "dep" : {
            "list|l!jsonfunc=debug_2_jsonfunc!" : [],
            "string|s!jsonfunc=debug_upper_jsonfunc!" : "s_var",
            "$" : "+"
        }
    }"#;
    before_parser();
    let optstr :String = format!(r#"{{ "{}" : "jsonfile" }}"#,
        OPT_JSON_LONG);
    let depstrval = "newval";
    let depliststr = r#"["depenv1","depenv2"]"#;
    let ws = r#"{"dep":{"list" : ["jsonval1","jsonval2"],"string" : "jsonstring"},"port":6000,"verbose":3}"#;
    let f = make_temp_file(ws);
    let jsonfile = format!("{}",f.path().display());
    let depws = r#"{"list":["depjson1","depjson2"]}"#;
    let depf = make_temp_file(depws);
    let depjsonfile = format!("{}", depf.path().display());
    set_env_var("EXTARGSPARSE_JSONFILE",&jsonfile);
    set_env_var("DEP_JSONFILE",&depjsonfile);
    let optref :ExtArgsOptions = ExtArgsOptions::new(&optstr).unwrap();
    let vint :Vec<i32> = vec![ENV_COMMAND_JSON_SET, ENVIRONMENT_SET, ENV_SUB_COMMAND_JSON_SET];
    let parser :ExtArgsParser = ExtArgsParser::new(Some(optref.clone()),Some(vint.clone())).unwrap();
    extargs_load_commandline!(parser,loads).unwrap();
    set_env_var("DEP_STRING",depstrval);
    set_env_var("DEP_LIST",depliststr);

    let params :Vec<String> = format_string_array(vec!["--jsonfile", &jsonfile, "dep", "ww"]);
    let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
    assert!(ns.get_int("verbose") == 3);
    assert!(ns.get_int("port") == 6000);
    assert!(ns.get_string("subcommand") == "dep");
    assert!(check_array_equal(ns.get_array("dep_list"), format_string_array(vec!["jsonval1"])));
    assert!(ns.get_string("dep_string") == "JSONSTRING");
    assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["ww"])));
    return;
}


#[test]
fn test_a056() {
    let loads = r#"        {
            "asn1parse" : {
                "$" : 0,
                "$inform!optparse=inform_optparse;completefunc=inform_complete!" : null,
                "$in" : null,
                "$out" : null,
                "$noout" : false,
                "$offset" : 0,
                "$length" : -1,
                "$dump" : false,
                "$dlimit" : -1,
                "$oid" : null,
                "$strparse" : 0,
                "$genstr" : null,
                "$genconf" : null
            }
        }"#;
    before_parser();
    let optstr :String = format!(r#"{{ "{}" : "-",
        "{}" : "-",
        "{}" : true,
        "{}" : false }}"#,
        OPT_LONG_PREFIX,OPT_SHORT_PREFIX,OPT_NO_JSON_OPTION,OPT_CMD_PREFIX_ADDED);
    let optref :ExtArgsOptions = ExtArgsOptions::new(&optstr).unwrap();
    let parser :ExtArgsParser = ExtArgsParser::new(Some(optref.clone()),None).unwrap();
    extargs_load_commandline!(parser,loads).unwrap();
    let sarr = parser.get_sub_commands_ex("").unwrap();
    check_array_equal(sarr.clone(), format_string_array(vec!["asn1parse"]));
    let opts = parser.get_cmd_opts_ex("asn1parse").unwrap();
    let optnames = format_string_array(vec!["inform", "in", "out", "noout", "offset", "length", "dump", "dlimit", "oid", "strparse", "genstr", "genconf", "help"]);
    let mut ok :bool;
    for opt in opts.iter() {
        if !opt.is_flag()  || opt.type_name() == KEYWORD_ARGS {
            continue;
        }
        ok = false;
        if opt.type_name() == KEYWORD_HELP {
            if opt.long_opt() == "-help" && opt.short_opt() == "-h" {
                ok = true;
            }
        } else {
            for c in optnames.clone() {
                let clong = format!("-{}",c);
                if opt.opt_dest().eq(&c) && opt.long_opt().eq(&clong) {
                    ok = true;
                    break;
                }
            }
        }
        assert!(ok == true);
    }

    return;
}

#[test]
fn test_a057() {
    let loads = r#"        {
            "asn1parse" : {
                "$" : 0,
                "$inform!optparse=inform_optparse;completefunc=inform_complete!" : null,
                "$in" : null,
                "$out" : null,
                "$noout" : false,
                "$offset" : 0,
                "$length" : -1,
                "$dump" : false,
                "$dlimit" : -1,
                "$oid" : null,
                "$strparse" : 0,
                "$genstr" : null,
                "$genconf" : null
            },
            "ca" : {
                "$" : 0,
                "$config" : null,
                "$name" : null,
                "$in" : null,
                "$ss_cert" : null,
                "$spkac" : null,
                "$infiles" : null,
                "$out" : null,
                "$outdir" : null,
                "$cert" : null,
                "$keyfile" : null,
                "$keyform!optparse=inform_optparse;completefunc=inform_complete!" : null,
                "$key" : null,
                "$selfsign" : false,
                "$passin" : null,
                "$verbose" : "+",
                "$notext" : false,
                "$startdate" : null,
                "$enddate" : null,
                "$days" : 30,
                "$md" : null,
                "$policy" : null,
                "$preserveDN" : false,
                "$msie_hack" : false,
                "$noemailDN" : false,
                "$batch" : false,
                "$extensions" : null,
                "$extfile" : null,
                "$engine" : null,
                "$subj" : null,
                "$utf8" : false,
                "$multivalue-rdn" : false,
                "$gencrl" : false,
                "$crldays" : 30,
                "$crlhours" : -1,
                "$revoke" : null,
                "$status" : null,
                "$updatedb" : false,
                "$crl_reason" : null,
                "$crl_hold" : null,
                "$crl_compromise" : null,
                "$crl_CA_compromise" : null,
                "$crlexts" : null
            }
        }"#;
    before_parser();
    let optstr :String = format!(r#"{{ "{}" : "-",
        "{}" : "-",
        "{}" : true,
        "{}" : false ,
        "{}" : true }}"#,
        OPT_LONG_PREFIX,OPT_SHORT_PREFIX,OPT_NO_JSON_OPTION,OPT_CMD_PREFIX_ADDED,
        OPT_FLAG_NO_CHANGE);
    let optref :ExtArgsOptions = ExtArgsOptions::new(&optstr).unwrap();
    let parser :ExtArgsParser = ExtArgsParser::new(Some(optref.clone()),None).unwrap();
    extargs_load_commandline!(parser,loads).unwrap();
    let sarr = parser.get_sub_commands_ex("").unwrap();
    check_array_equal(sarr.clone(), format_string_array(vec!["asn1parse","ca"]));
    let opts = parser.get_cmd_opts_ex("ca").unwrap();
    let optnames = format_string_array(vec!["config", "name", "in", "ss_cert", "spkac", "infiles", "out", "outdir", "cert", "keyfile", "keyform", "key", "selfsign", "passin", "verbose", "notext", "startdate", "enddate", "days", "md", "policy", "preserveDN", "msie_hack", "noemailDN", "batch", "extensions", "extfile", "engine", "subj", "utf8", "gencrl", "crldays", "crlhours", "revoke", "status", "updatedb", "crl_reason", "crl_hold", "crl_compromise", "crl_CA_compromise", "crlexts",]);
    let mut ok :bool;
    for opt in opts.iter() {
        if !opt.is_flag()  || opt.type_name() == KEYWORD_ARGS {
            continue;
        }
        ok = false;
        if opt.type_name() == KEYWORD_HELP {
            if opt.long_opt() == "-help" && opt.short_opt() == "-h" {
                ok = true;
            }
        } else if opt.long_opt() == "-multivalue-rdn" && opt.opt_dest() == "multivalue_rdn" {
            ok = true;
        } else {
            for c in optnames.clone() {
                let clong = format!("-{}",c);
                if opt.opt_dest().eq(&c) && opt.long_opt().eq(&clong) {
                    ok = true;
                    break;
                }
            }
        }

        assert!(ok == true);
    }

    return;
}

#[test]
fn test_a058() {
    let loads = r#"        {
            "verbose" : "+",
            "dep" : {
                "$" : "*"
            },
            "rdep" : {
                "$" : "*"
            }
        }"#;
    before_parser();
    let optstr :String = format!("{{ }}");
    let optref :ExtArgsOptions = ExtArgsOptions::new(&optstr).unwrap();
    let parser :ExtArgsParser = ExtArgsParser::new(Some(optref.clone()),None).unwrap();
    extargs_load_commandline!(parser,loads).unwrap();
    let sarr = get_cmd_help(parser.clone(),"");
    let matchexpr = Regex::new(r#".*\[OPTIONS\]\s+\[SUBCOMMANDS\]\s+.*"#).unwrap();
    assert!(matchexpr.is_match(&(sarr[0])) == true);
    return;
}

#[test]
#[extargs_map_function(optparse=debug_set_2_args_upper)]
fn test_a059() {
    let loads = r#"        {
            "verbose|v" : "+",
            "kernel|K" : "/boot/",
            "initrd|I" : "/boot/",
            "pair|P!optparse=debug_set_2_args_upper!" : [],
            "encryptfile|e" : null,
            "encryptkey|E" : null,
            "setupsectsoffset" : 663,
            "ipxe" : {
                "$" : "+"
            }
        }"#;
    before_parser();
    let optstr :String = format!(r#"{{ "{}" : true, "{}" : "++" ,"{}" : "+" }}"#,
        OPT_PARSE_ALL,OPT_LONG_PREFIX,OPT_SHORT_PREFIX);
    let optref :ExtArgsOptions = ExtArgsOptions::new(&optstr).unwrap();
    let parser :ExtArgsParser = ExtArgsParser::new(Some(optref.clone()),None).unwrap();
    extargs_load_commandline!(parser,loads).unwrap();
    let params :Vec<String> = format_string_array(vec!["+K", "kernel", "++pair", "initrd", "cc", "dd", "+E", "encryptkey", "+e", "encryptfile", "ipxe"]);
    let ns = parser.parse_commandline_ex(Some(params.clone()),None,None,None).unwrap();
    assert!(ns.get_string("subcommand") == "ipxe");
    assert!(check_array_equal(ns.get_array("subnargs"), format_string_array(vec!["dd"])));
    assert!(check_array_equal(ns.get_array("pair"), format_string_array(vec!["INITRD","CC"])));
    return;
}

#[test]
fn test_a060() {
    let loads = r#"{
    "dep": {
        "$": "*",
        "ip": {
            "$": "*"
        }
    },
    "rdep" : {
        "$" : "*",
        "ip" : {
            "$" : "*"
        }
    }
}"#;
    before_parser();
    let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
    extargs_load_commandline!(parser,loads).unwrap();
    let sarr = get_cmd_help(parser.clone(),"");
    let _ = assert_ok_cmds(sarr.clone(),parser.clone(),"").unwrap();
    let sarr = get_cmd_help(parser.clone(),"dep");
    let _ = assert_ok_cmds(sarr.clone(),parser.clone(),"dep").unwrap();
    let sarr = get_cmd_help(parser.clone(),"rdep");
    let _ = assert_ok_cmds(sarr.clone(),parser.clone(),"rdep").unwrap();
    return;
}

#[test]
fn test_a061() {
    let loads = r#"        {
            "dep##[cc]... dep handler used##" : {
                "$" : "*",
                "ip" : {
                    "$" : "*"
                }
            },
            "rdep##[dd]... rdep handler used##" : {
                "$" : "*",
                "ip" : {
                    "$" : "*"
                }
            }
        }"#;
    before_parser();
    let parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
    extargs_load_commandline!(parser,loads).unwrap();
    let sarr = get_cmd_help(parser.clone(),"dep");
    let ex = Regex::new(r#"\[cc\]... dep handler used"#).unwrap();
    let mut ok :bool =false;
    if sarr.len() > 0 {
        if ex.is_match(&sarr[0]) {
            ok = true;
        }
    }
    assert!(ok == true);

    let sarr = get_cmd_help(parser.clone(),"rdep");
    let ex = Regex::new(r#"\[dd\]... rdep handler used"#).unwrap();
    ok = false;
    if sarr.len() > 0 {
        if ex.is_match(&sarr[0]) {
            ok = true;
        }
    }
    assert!(ok == true);

    return;
}

#[test]
fn test_a062() {
    let loads = r#"        {
            "dep##[cc]... dep handler used##" : {
                "$" : "*",
                "ip" : {
                    "$" : "*"
                }
            },
            "rdep##[dd]... rdep handler used##" : {
                "$" : "*",
                "ip" : {
                    "$" : "*"
                }
            }
        }"#;
    before_parser();
    let optstr :String = format!(r#"{{ "{}" : "cmd1" }}"#, OPT_PROG);
    let optref :ExtArgsOptions = ExtArgsOptions::new(&optstr).unwrap();
    let parser :ExtArgsParser = ExtArgsParser::new(Some(optref.clone()),None).unwrap();
    extargs_load_commandline!(parser,loads).unwrap();
    let sarr = get_cmd_help(parser.clone(),"dep");
    let ex = Regex::new(r#"\[cc\]... dep handler used"#).unwrap();
    let ex2 = Regex::new("cmd1").unwrap();
    let mut ok :bool =false;
    if sarr.len() > 0 {
        if ex.is_match(&sarr[0]) {
            ok = true;
        }
    }
    assert!(ok == true);
    ok = false;
    if sarr.len() > 0 {
        if ex2.is_match(&sarr[0]) {
            ok = true;
        }
    }
    assert!(ok == true);


    let sarr = get_cmd_help(parser.clone(),"rdep");
    let ex = Regex::new(r#"\[dd\]... rdep handler used"#).unwrap();
    ok = false;
    if sarr.len() > 0 {
        if ex.is_match(&sarr[0]) {
            ok = true;
        }
    }
    assert!(ok == true);

    ok = false;
    if sarr.len() > 0 {
        if ex2.is_match(&sarr[0]) {
            ok = true;
        }
    }
    assert!(ok == true);

    return;
}