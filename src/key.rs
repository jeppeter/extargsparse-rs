use serde_json::{Value};

enum Nargs {	
	String,
	i32,
}

pub struct Key {
	__value :Value,
	__prefix :String,
	__flagname :String,
	__helpinfo :String,
	__shortflag :String,
	__nargs :Nargs,
}