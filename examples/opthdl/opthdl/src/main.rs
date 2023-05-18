use extargsparse_codegen::{ArgSet,extargs_load_commandline,extargs_map_function};
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::{extargs_error_class,extargs_new_error};
use extargsparse_worker::key::{ExtKeyParse};
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
use std::borrow::Borrow;
use std::borrow::BorrowMut;

extargs_error_class!{OptHdlError}

#[derive(ArgSet)]
struct DepSt {
	subnargs :Vec<String>,
	strv : String,
	list :Vec<String>,
}

#[derive(ArgSet)]
struct RdepSt {
	strv :String,
	subnargs : Vec<String>,
	list :Vec<String>,
}

#[derive(ArgSet)]
struct SubCmdStruct {
	verbose :i32,
	pair :Vec<String>,
	dep :DepSt,
	rdep :RdepSt,
	args :Vec<String>,
}


fn pair_key_handle(ns :NameSpaceEx, validx :i32, keycls :ExtKeyParse, params :Vec<String>) -> Result<i32,Box<dyn Error>> {
    println!("validx [{}]",validx);

    if params.len() < (validx + 2) as usize {
    	extargs_new_error!{OptHdlError,"need 2 args"}
    }
    let mut vc :Vec<String> = ns.get_array(&(keycls.opt_dest()));
    vc.push(format!("{}",params[validx as usize]));
    vc.push(format!("{}",params[(validx + 1) as usize]));
    let _ = ns.set_array(&(keycls.opt_dest()),vc)?;
    return Ok(2);
}


fn dep_handler(_ns :NameSpaceEx, _args :Option<Arc<RefCell<dyn ArgSetImpl>>>, _ctx : Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	println!("in dep_handler");
	if _args.is_some() {
		println!("some _args");
		let ctx = _args.as_ref().unwrap().clone();
        let c  = ctx.as_ptr() as *const RefCell<dyn ArgSetImpl>;
        let b = c.borrow();
        let cc = *b as *const SubCmdStruct;
        let bbcref :&SubCmdStruct = unsafe {cc.as_ref().unwrap()};
        println!("verbose {}", bbcref.verbose);
        println!("pair {:?}", bbcref.pair);
        println!("args {:?}", bbcref.args);
        println!("subnargs {:?}", bbcref.dep.subnargs);
        println!("strv {}", bbcref.dep.strv);
        println!("list {:?}",bbcref.dep.list);
	} else {
		println!("none of _args");
	}
	Ok(())
}

fn rdep_handler(_ns :NameSpaceEx, _args :Option<Arc<RefCell<dyn ArgSetImpl>>>, _ctx : Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
    println!("in rdep_handler");
    if _args.is_some() {
        println!("some _args");
        let ctx = _args.as_ref().unwrap().clone();
        let mut c  = ctx.as_ptr() as *mut RefCell<dyn ArgSetImpl>;
        let b = c.borrow_mut();
        let cc = *b as *mut SubCmdStruct;
        let bbcref :&mut SubCmdStruct = unsafe {cc.as_mut().unwrap()};
        println!("verbose {}", bbcref.verbose);
        println!("pair {:?}", bbcref.pair);
        println!("args {:?}", bbcref.args);
        println!("subnargs {:?}", bbcref.rdep.subnargs);
        println!("strv {}", bbcref.rdep.strv);
        bbcref.rdep.list.push(format!("rdep"));
        println!("list {:?}",bbcref.rdep.list);
    } else {
        println!("none of _args");
    }
    Ok(())
}


#[extargs_map_function(optparse=pair_key_handle,dep_handler,rdep_handler)]
fn main() -> Result<(),Box<dyn Error>> {
    let loads = r#"{
		"verbose" : "+",
		"pair|p!optparse=pair_key_handle!##to set pair parameters##" : [],
		"dep<dep_handler>" : {
			"$" : "*",
			"list|L" :  [],
			"str|S<strv>" : ""
		},
		"rdep<rdep_handler>" : {
			"$" : "*",
			"list|l" : [],
			"str|s<strv>" : ""
		}
		}"#;
    let parser :ExtArgsParser = ExtArgsParser::new(None,None)?;
    extargs_load_commandline!(parser,loads)?;
    let v :SubCmdStruct = SubCmdStruct::new();
    let argv :Arc<RefCell<SubCmdStruct>> = Arc::new(RefCell::new(v));
    let _ = parser.parse_commandline_ex(None,None,Some(argv.clone()),None)?;
    Ok(())
}