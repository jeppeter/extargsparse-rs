use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
use extargsparse_worker::key::{ExtKeyParse};
use extargsparse_worker::options::{ExtArgsOptions,OPT_PROG};
use extargsparse_worker::funccall::ExtArgsParseFunc;
use extargsparse_worker::parser::ExtArgsParser;

use std::error::Error;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[extargs_map_function()]
fn main() -> Result<(),Box<dyn Error>> {
    let loads = r#"{
            "dep" : {
                "ip" : {
                	"$" : "*"
                },
                "mip" : {
                	"$" : "*"
                }
            },
            "rdep" : {
                "ip" : {
                },
                "rmip" : {                	
                }
            }
        }"#;
    let optstr = format!("{{ \"{}\" : \"cmd1\"}}", OPT_PROG);
    let options = ExtArgsOptions::new(&optstr)?;
    let parser :ExtArgsParser = ExtArgsParser::new(Some(options.clone()),None)?;
    extargs_load_commandline!(parser,loads)?;
    let mut cmds = parser.get_sub_commands_ex("")?;
    println!("main cmd subcmds:{:?}", cmds);

    cmds = parser.get_sub_commands_ex("dep")?;
    println!("dep cmd subcmds:{:?}", cmds);


    cmds = parser.get_sub_commands_ex("rdep.ip")?;
    println!("rdep.ip cmd subcmds:{:?}", cmds);

    Ok(())
}
/*
output:
main cmd subcmds:["dep", "rdep"]
dep cmd subcmds:["ip", "mip"]
rdep.ip cmd subcmds:[]
*/