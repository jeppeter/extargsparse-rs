use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
use extargsparse_worker::funccall::ExtArgsParseFunc;
use extargsparse_worker::parser::ExtArgsParser;


use std::error::Error;
use lazy_static::lazy_static;
use std::collections::HashMap;



#[extargs_map_function()]
fn main() -> Result<(),Box<dyn Error>> {
    let cmdline = r#"{}"#;
    let parser = ExtArgsParser::new(None,None)?;
    extargs_load_commandline!(parser,cmdline)?;
    let params = vec!["-h".to_string()];
    parser.parse_commandline(Some(params.clone()),None)?;

    Ok(())
}
/*
output:
noopt.exe  [OPTIONS] [args...]

 [OPTIONS]
    --json     json  json input file to get the value set 
    --help|-h        to display this help information     

*/