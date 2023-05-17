use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
use extargsparse_worker::key::{ExtKeyParse};
use extargsparse_worker::options::{ExtArgsOptions,OPT_NO_HELP_OPTION,OPT_NO_JSON_OPTION};
use extargsparse_worker::funccall::ExtArgsParseFunc;
use extargsparse_worker::parser::ExtArgsParser;


use std::error::Error;
use lazy_static::lazy_static;
use std::collections::HashMap;




#[extargs_map_function()]
fn main() -> Result<(),Box<dyn Error>> {
    let loads = r#"{
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
    let opts :String = format!("{{
    	\"{}\" : true,
    	\"{}\" : true
    }}",OPT_NO_JSON_OPTION,OPT_NO_HELP_OPTION);
    let options = ExtArgsOptions::new(&opts)?;
    let parser :ExtArgsParser = ExtArgsParser::new(Some(options.clone()),None)?;
    extargs_load_commandline!(parser,loads)?;
    let mut flag :ExtKeyParse;
    let mut name :String;
    let ores = parser.get_cmd_key_ex("")?;
    assert!(ores.is_some());
    flag = ores.unwrap();
    name = flag.cmd_name();
    println!("cmdname={}", name);

    let ores = parser.get_cmd_key_ex("dep")?;
    assert!(ores.is_some());
    flag = ores.unwrap();
    name = flag.cmd_name();
    println!("cmdname={}", name);

    let ores = parser.get_cmd_key_ex("rdep.ip")?;
    assert!(ores.is_some());
    flag = ores.unwrap();
    name = flag.cmd_name();
    println!("cmdname={}", name);

    Ok(())
}
/*
output:
cmdname=main
cmdname=dep
cmdname=ip
*/