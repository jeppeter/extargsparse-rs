use serde_json::{Value};
use std::collections::HashMap;

#[allow(dead_code)]
enum Nargs {	
	Argtype(String),
	Argnum(i32),
	None,
}

struct KeyAttr {
	__splitchar :char,
	__obj :HashMap<String,Value>,
}

impl KeyAttr {
	fn new(_attr :&str) -> KeyAttr {
		let mut kattr = KeyAttr {
			__splitchar  : ';',
			__obj : HashMap::new(),
		};

		if _attr.len() > 0 {
			if _attr.starts_with("split=") && _attr.len() >= 7 {
				let c = _attr.as_bytes()[6] as char;
				if c == '.' {
					kattr.__splitchar = '.';
				} else if c == '\\' {
					kattr.__splitchar = '\\';
				} else if c == '/' {
					kattr.__splitchar = '/';
				} else if c == ':' {
					kattr.__splitchar = ':';
				} else if c == '@' {
					kattr.__splitchar = '@';
				} else if c == '+' {
					kattr.__splitchar = '+';
				} else {
					panic!("not support char [{}]", c);
				}
			}
		}
		return kattr;
	}

}

pub struct Key {
	__value :Value,
	__prefix :String,
	__flagname :String,
	__helpinfo :String,
	__shortflag :String,
	__nargs :Nargs,
	__varname :String,
	__cmdname :String,
	__function :String,
	__origkey :String,
	__iscmd :bool,
	__isflag :bool,
	__type :String,
	__attr :KeyAttr,
	__nochange :bool,
	__longprefix :String,
	__shortprefix :String,
}