use serde_json::Value;
use std::collections::HashMap;
use regex::Regex;


#[allow(dead_code)]
enum Nargs {	
	Argtype(String),
	Argnum(i32),
	None,
}

struct KeyAttr {
	__splitchar :char,
	__obj :HashMap<String,String>,
}

#[allow(dead_code)]
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
			let mut i :usize;
			let sarr :Vec<&str>;
			let mut carr :Vec<&str>;
			let re ;
			let rec ;
			re = Regex::new(&(format!("{}",kattr.__splitchar)[..])).unwrap();
			rec = Regex::new("=").unwrap();
			sarr = re.split(_attr).into_iter().collect();
			i = 0;
			while i < sarr.len() {
				carr = rec.split(sarr[i]).into_iter().collect();
				if carr.len()  > 1 {
					if carr[0] != "split" {
						kattr.__obj.insert(format!("{}",carr[0]),format!("{}",carr[1]));	
					}					
				}
				i = i + 1;
			}
		}
		return kattr;
	}

	fn string(&self) -> String {
		let mut retstr :String;
		let mut v:Vec<_> = (&(self.__obj)).into_iter().collect();
		let mut i:usize;
		v.sort_by(|x,y|x.0.cmp(&y.0));

		retstr = String::from("{");
		i = 0 ;
		while i < v.len() {
			retstr.push_str(&(format!("{}={}", v[i].0,v[i].1)[..]));
			i = i + 1;
		}
		retstr.push_str("}");
		return retstr;
	}

	fn get_attr(&self,name :&str) -> String {
		match self.__obj.get(name) {
			Some(v) => { return v.to_string();},
			None => {return String::from("");}
		}
	}
}

struct TypeClass {
	typeval : String,
}

#[allow(dead_code)]
impl TypeClass {
	fn new(v :&Value) -> TypeClass {
		let tv :String;
		match v {
			Value::String(_)  => {tv = String::from("string");},
			Value::Object(_) => {tv = String::from("dict");},
			Value::Array(_) => { tv = String::from("list");},
			Value::Bool(_) => {tv = String::from("bool");},
			Value::Number(n) => {
				if n.is_i64() || n.is_u64() {
					tv = String::from("int");
				} else {
					tv = String::from("float");
				}
			},
			Value::Null => {tv = String::from("string");},
		}
		return TypeClass{typeval : tv,};
	}

	fn get_type(&self) -> String {
		return format!("{}",self.typeval);
	}

	fn string(&self) -> String {
		return format!("{}",self.typeval);
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