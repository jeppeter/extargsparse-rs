use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::namespace::NameSpaceEx;
use extargsparse_worker::funccall::ExtArgsParseFunc;
use extargsparse_worker::parser::ExtArgsParser;


use std::error::Error;
use lazy_static::lazy_static;
use std::sync::Arc;
use std::cell::RefCell;
use std::any::Any;
use std::collections::HashMap;


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
            "float1|f" : 3.633 ,
            "float2" : 6422.22,
            "float3" : 44463.23,
            "verbose|v" : "+",
            "dep<dep_handler>" : {
                "float3" : 3332.233
            },
            "rdep<rdep_handler>" : {
                "ip" : {
                    "float4" : 3377.33,
                    "float6" : 33.22,
                    "float7" : 0.333
                }
            }

        }"#;
    let parser :ExtArgsParser = ExtArgsParser::new(None,None)?;
    extargs_load_commandline!(parser,cmdline)?;
    let ns :NameSpaceEx = parser.parse_commandline_ex(None,None,None,None)?;
    println!("ns.float1 {}", ns.get_float("float1"));
    println!("ns.float2 {}",ns.get_float("float2") );
    println!("subcommand {}",ns.get_string("subcommand"));
    println!("args {:?}", ns.get_array("args"));
    println!("subnargs {:?}", ns.get_array("subnargs"));
    println!("ns.float3 {}", ns.get_float("float3"));
    println!("ns.dep_float3 {}", ns.get_float("dep_float3"));
    println!("ns.rdep_ip_float4 {}", ns.get_float("rdep_ip_float4"));
    println!("ns.rdep_ip_float6 {}", ns.get_float("rdep_ip_float6"));
    println!("ns.rdep_ip_float7 {}", ns.get_float("rdep_ip_float7"));
    Ok(())
}
