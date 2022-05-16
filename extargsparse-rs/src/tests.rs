use super::parser::{ExtArgsParser};
use extargsparse_codegen::{extargs_load_commandline};


fn before_parser() {
	let mut cont : i32= 1;
	while cont > 0 {
		cont = 0;
		for (k,v) in std::env::vars() {
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
	let params :Vec<String> = vec!["-vvvv".to_string(), "-f".to_string(), "-n".to_string(), "30".to_string(), "-l".to_string(), "bar1".to_string(), "-l".to_string(), "bar2".to_string(), "var1".to_string(), "var2".to_string()];
	let mut parser :ExtArgsParser = ExtArgsParser::new(None,None).unwrap();
	before_parser();
	extargs_load_commandline!(parser,loads).unwrap();

	return;
}