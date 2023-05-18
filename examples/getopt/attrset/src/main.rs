use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
use extargsparse_worker::key::{ExtKeyParse,KEYWORD_ATTR};
use extargsparse_worker::options::ExtArgsOptions;
use extargsparse_worker::namespace::NameSpaceEx;
use extargsparse_worker::funccall::ExtArgsParseFunc;
use extargsparse_worker::parser::ExtArgsParser;


use std::error::Error;
use lazy_static::lazy_static;
use std::collections::HashMap;



fn flag_parse(ns :NameSpaceEx, validx :i32, keycls :ExtKeyParse, params :Vec<String>) -> Result<i32,Box<dyn Error>> {
    println!("validx [{}]",validx);
    let attr = keycls.get_keyattr(KEYWORD_ATTR).unwrap();
    let mut vc :Vec<String> = Vec::new();
    println!("opthelp={}",attr.get_attr("opthelp"));
    println!("optparse={}",attr.get_attr("optparse"));
    vc.push(format!("{}",params[validx as usize]));
    let _ = ns.set_array(&(keycls.opt_dest()),vc)?;
    return Ok(1);
}

fn flag_help(_keycls :&ExtKeyParse) -> String {
	return format!("flag special set []");
}


#[extargs_map_function(optparse=flag_parse,opthelp=flag_help)]
fn main() -> Result<(),Box<dyn Error>> {
    let loads = r#"{
    	"flag|f!optparse=flag_parse;opthelp=flag_help!" : []
    }"#;
    let opts :String = format!("{{}}");
    let options : ExtArgsOptions = ExtArgsOptions::new(&opts)?;
    let parser :ExtArgsParser = ExtArgsParser::new(Some(options.clone()),None)?;
    extargs_load_commandline!(parser,loads)?;
    let ns :NameSpaceEx = parser.parse_commandline_ex(None,None,None,None)?;
    println!("flag = {:?}", ns.get_array("flag"));
    Ok(())
}
