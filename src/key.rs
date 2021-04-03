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
		let kattr = KeyAttr {
			__splitchar  : ';',
			__obj : HashMap::new(),
		};

		if _attr.len() > 0 {
			
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