use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
//use extargsparse_worker::argset::{ArgSetImpl};
//use extargsparse_worker::{extargs_error_class,extargs_new_error};
//use extargsparse_worker::{extargs_log_trace};
use extargsparse_worker::key::{ExtKeyParse,KEYWORD_ATTR};
use extargsparse_worker::options::ExtArgsOptions;
use extargsparse_worker::namespace::NameSpaceEx;
use extargsparse_worker::funccall::ExtArgsParseFunc;
use extargsparse_worker::parser::ExtArgsParser;


use std::error::Error;
use lazy_static::lazy_static;
//use std::sync::Arc;
//use std::cell::RefCell;
//use std::any::Any;
use std::collections::HashMap;

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
struct SubcmdStruct {
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
    //println!("Attr={:?}",attr);
    let mut vc :Vec<String> = ns.get_array(&(keycls.opt_dest()));
    vc.push(format!("{}",params[idx as usize]));
    vc.push(format!("{}",params[(idx + 1) as usize]));
    let _ = ns.set_array(&(keycls.opt_dest()),vc)?;
    return Ok(2);
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
	Ok(())
}

fn rdep_handler(_ns :NameSpaceEx, _args :Option<Arc<RefCell<dyn ArgSetImpl>>>, _ctx : Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	println!("call rdep_handler");
	Ok(())
}


#[extargs_map_function(actfunc=flag_parse,opthelp=flag_help)]
fn main() -> Result<(),Box<dyn Error>> {
    let loads = r#"{
		"verbose" : "+",
		"pair|p!optparse=pair_key_handle!##to set pair parameters##" : [],
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
    extargs_load_commandline!(parser,loads)?;
    //parser.load_commandline_string(cmdline,Some(ST_FUNCTIONS_MFHGDTXIBZ9MXQY.clone()))?;
    let ns :NameSpaceEx = parser.parse_commandline_ex(None,None,None,None)?;
    println!("flag = {:?}", ns.get_array("flag"));
    Ok(())
}