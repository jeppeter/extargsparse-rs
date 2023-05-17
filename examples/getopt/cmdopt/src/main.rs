use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
use extargsparse_worker::key::{ExtKeyParse,KEYWORD_ATTR};
use extargsparse_worker::options::{ExtArgsOptions,OPT_PROG};
use extargsparse_worker::funccall::ExtArgsParseFunc;
use extargsparse_worker::parser::ExtArgsParser;


use std::error::Error;
use lazy_static::lazy_static;
use std::collections::HashMap;





#[extargs_map_function()]
fn main() -> Result<(),Box<dyn Error>> {
    let loads = r#"{
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
    let opts :String = format!("{{
    	\"{}\" : \"cmd1\"
    }}",OPT_PROG);
    let options = ExtArgsOptions::new(&opts)?;
    let parser :ExtArgsParser = ExtArgsParser::new(Some(options.clone()),None)?;
    extargs_load_commandline!(parser,loads)?;
    let mut flags :Vec<ExtKeyParse>;
    let mut i :usize;
    flags = parser.get_cmd_opts_ex("")?;
    i = 0;
    println!("main cmd opts:");
    for f in flags.iter() {
    	if f.type_name() == "args" {
    		println!("[{}].type=args",i);
    	} else {
    		let ores = f.get_keyattr(KEYWORD_ATTR);
    		let mut attrs :String = "".to_string();
    		if ores.is_some() {
    			let v = ores.unwrap();
    			attrs = v.string();
    		}
    		println!("[{}].Longopt={};.Shortopt={};Optdest={};attr={}", i, f.long_opt(), f.short_opt(), f.opt_dest(), attrs);
    	}
    	i += 1;
    }


    flags = parser.get_cmd_opts_ex("dep")?;
    i = 0;
    println!("dep cmd opts:");
    for f in flags.iter() {
    	if f.type_name() == "args" {
    		println!("[{}].type=args",i);
    	} else {
    		let ores = f.get_keyattr(KEYWORD_ATTR);
    		let mut attrs :String = "".to_string();
    		if ores.is_some() {
    			let v = ores.unwrap();
    			attrs = v.string();
    		}
    		println!("[{}].Longopt={};.Shortopt={};Optdest={};attr={}", i, f.long_opt(), f.short_opt(), f.opt_dest(), attrs);
    	}
    	i += 1;
    }


    flags = parser.get_cmd_opts_ex("rdep.ip")?;
    i = 0;
    println!("rdep.ip cmd opts:");
    for f in flags.iter() {
    	if f.type_name() == "args" {
    		println!("[{}].type=args",i);
    	} else {
    		let ores = f.get_keyattr(KEYWORD_ATTR);
    		let mut attrs :String = "".to_string();
    		if ores.is_some() {
    			let v = ores.unwrap();
    			attrs = v.string();
    		}
    		println!("[{}].Longopt={};.Shortopt={};Optdest={};attr={}", i, f.long_opt(), f.short_opt(), f.opt_dest(), attrs);
    	}
    	i += 1;
    }

    Ok(())
}
/*
output:
main cmd opts:
[0].type=args
[1].Longopt=--help;.Shortopt=-h;Optdest=help;attr=
[2].Longopt=--json;.Shortopt=;Optdest=json;attr=
[3].Longopt=--port;.Shortopt=-p;Optdest=port;attr=
[4].Longopt=--http-url;.Shortopt=-u;Optdest=http_url;attr=
[5].Longopt=--verbose;.Shortopt=-v;Optdest=verbose;attr=
[6].Longopt=--http-visual-mode;.Shortopt=-V;Optdest=http_visual_mode;attr=
dep cmd opts:
[0].type=args
[1].Longopt=--help;.Shortopt=-h;Optdest=help;attr=
[2].Longopt=--dep-json;.Shortopt=;Optdest=dep_json;attr=
[3].Longopt=--dep-list;.Shortopt=-l;Optdest=dep_list;attr=[attr]=[cc]
[optfunc]=[list_opt_func]
[4].Longopt=--dep-string;.Shortopt=-s;Optdest=dep_string;attr=
rdep.ip cmd opts:
[0].type=args
[1].Longopt=--rdep-ip-cc;.Shortopt=;Optdest=rdep_ip_cc;attr=
[2].Longopt=--help;.Shortopt=-h;Optdest=help;attr=
[3].Longopt=--rdep-ip-json;.Shortopt=;Optdest=rdep_ip_json;attr=
[4].Longopt=--rdep-ip-list;.Shortopt=;Optdest=rdep_ip_list;attr=
[5].Longopt=--rdep-ip-verbose;.Shortopt=;Optdest=rdep_ip_verbose;attr=    
*/
