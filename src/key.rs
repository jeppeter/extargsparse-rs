use serde_json::Value;
use std::collections::HashMap;
use regex::Regex;


#[allow(dead_code)]
pub enum Nargs {	
	Argtype(String),
	Argnum(i32),
}

impl Clone for Nargs {
	fn clone(&self) -> Nargs {
		match self {
			Nargs::Argtype(s) => {
				return Nargs::Argtype(s.clone());
			},
			Nargs::Argnum(iv) => {
				return Nargs::Argnum(*iv);
			}
		}
	}
}


#[allow(dead_code)]
enum BoolNone {
	BoolVal(bool),
	None,
}

pub struct KeyAttr {
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

const KEYWORD_VALUE :&str = "value";
const KEYWORD_PREFIX :&str = "prefix";
const KEYWORD_FLAGNAME :&str = "flagname";
const KEYWORD_HELPINFO :&str = "helpinfo";
const KEYWORD_SHORTFLAG :&str = "shortflag";
const KEYWORD_NARGS :&str = "nargs";
const KEYWORD_VARNAME :&str = "varname";
const KEYWORD_CMDNAME :&str = "cmdname";
const KEYWORD_FUNCTION :&str = "function";
const KEYWORD_ORIGKEY :&str = "origkey";
const KEYWORD_ISCMD :&str = "iscmd";
const KEYWORD_ISFLAG :&str = "isflag";
const KEYWORD_TYPE :&str = "type";
const KEYWORD_ATTR :&str = "attr";
const KEYWORD_LONGPREFIX :&str = "longprefix";
const KEYWORD_SHORTPREFIX :&str = "shortprefix";
const KEYWORD_LONGOPT :&str = "longopt";
const KEYWORD_SHORTOPT :&str = "shortopt";
const KEYWORD_OPTDEST :&str = "optdest";
const KEYWORD_NEEDARG :&str = "needarg";
const KEYWORD_BLANK :&str  = "";

const KEYWORD_NOCHANGE :&str  = "nochange";

#[allow(dead_code)]
const FLAGSPECIAL : &'static [&'static str] = &[KEYWORD_VALUE,KEYWORD_PREFIX];
#[allow(dead_code)]
const FLAGWORDS :&'static [&'static str] = &[KEYWORD_FLAGNAME,KEYWORD_HELPINFO,KEYWORD_SHORTFLAG,KEYWORD_NARGS,KEYWORD_VARNAME];
#[allow(dead_code)]
const CMDWORDS :&'static [&'static str] = &[KEYWORD_CMDNAME,KEYWORD_FUNCTION,KEYWORD_HELPINFO];
#[allow(dead_code)]
const OTHERWORDS :&'static [&'static str] = &[KEYWORD_ORIGKEY,KEYWORD_ISCMD,KEYWORD_ISFLAG,KEYWORD_TYPE,KEYWORD_ATTR,KEYWORD_LONGPREFIX,KEYWORD_SHORTPREFIX];
#[allow(dead_code)]
const FORMWORDS :&'static [&'static str] = &[KEYWORD_LONGOPT,KEYWORD_SHORTOPT,KEYWORD_OPTDEST,KEYWORD_NEEDARG];


pub enum KeyVal {
	StrVal(Option<String>),
	BoolVal(Option<bool>),
	JsonVal(Option<Value>),
	KeyAttrVal(Option<KeyAttr>),
	NArgVal(Option<Nargs>),
}

pub struct KeyData {
	data :HashMap<String,KeyVal>,
}

impl KeyData {

	pub fn reset(&mut self) {
		self.data.clear();
		self.data.insert(String::from(KEYWORD_VALUE),KeyVal::JsonVal(None));
		self.data.insert(String::from(KEYWORD_PREFIX),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		self.data.insert(String::from(KEYWORD_FLAGNAME),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		self.data.insert(String::from(KEYWORD_HELPINFO),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		self.data.insert(String::from(KEYWORD_SHORTFLAG),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		self.data.insert(String::from(KEYWORD_NARGS),KeyVal::NArgVal(None));
		self.data.insert(String::from(KEYWORD_VARNAME),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		self.data.insert(String::from(KEYWORD_CMDNAME),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		self.data.insert(String::from(KEYWORD_FUNCTION),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		self.data.insert(String::from(KEYWORD_ORIGKEY),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		self.data.insert(String::from(KEYWORD_ISCMD),KeyVal::BoolVal(None));
		self.data.insert(String::from(KEYWORD_ISFLAG),KeyVal::BoolVal(None));
		self.data.insert(String::from(KEYWORD_TYPE),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		self.data.insert(String::from(KEYWORD_ATTR),KeyVal::KeyAttrVal(None));
		self.data.insert(String::from(KEYWORD_NOCHANGE),KeyVal::BoolVal(Some(false)));
		self.data.insert(String::from(KEYWORD_LONGPREFIX),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		self.data.insert(String::from(KEYWORD_SHORTPREFIX),KeyVal::StrVal(Some(String::from(KEYWORD_BLANK))));
		return;
	}

	pub fn set_string(&mut self,key :&str, val :&str) -> bool {
		let mut retval :bool = true;
		let ks :String = String::from(key);
		let vs :String = String::from(val);
		if self.data.contains_key(&ks) {
			retval = false;
			self.data.remove(&ks);
		}
		self.data.insert(ks,KeyVal::StrVal(Some(vs)));
		
		return retval;
	}

	pub fn get_string(&self, key :&str) -> Option<String> {
		let ks :String = String::from(key);
		if !self.data.contains_key(&ks) {
			return None;
		}

		match self.data.get(&ks) {
			Some(v) => {
				match v {
					KeyVal::StrVal(kv2) => {
						match kv2 {
							Some(sv) => {
								return Some(sv.clone());
							},
							_ => {return None;},
						}						
						
					},
					_ => {return None;},
				}
			},
			_ =>  {
				return None;
			}
		}		
	}	

	pub fn set_bool(&mut self, key :&str, val :&bool) -> bool {
		let mut retval :bool = true;
		let ks :String = String::from(key);
		let vb :bool = *val;

		if self.data.contains_key(&ks) {
			retval = false;
			self.data.remove(&ks);
		}
		self.data.insert(ks,KeyVal::BoolVal(Some(vb)));
		
		return retval;		
	}

	pub fn get_bool(&self, key :&str) -> Option<bool> {
		let ks :String = String::from(key);
		if !self.data.contains_key(&ks) {
			return None;
		}

		match self.data.get(&ks) {
			Some(v) => {
				match v {
					KeyVal::BoolVal(kv2) => {
						match kv2 {
							Some(sv) => {
								return Some(*sv);
							},
							_ => {return None;},
						}						
						
					},
					_ => {return None;},
				}
			},
			_ =>  {
				return None;
			}
		}		
	}


	pub fn set_nargs(&mut self, key :&str, val :&Nargs) -> bool {
		let mut retval :bool = true;
		let ks :String = String::from(key);
		let vb :Nargs;

		match val {
			Nargs::Argtype(s) => {vb = Nargs::Argtype(s.clone());},
			Nargs::Argnum(v) => {vb = Nargs::Argnum(*v);},
		}

		if self.data.contains_key(&ks) {
			retval = false;
			self.data.remove(&ks);
		}
		self.data.insert(ks,KeyVal::NArgVal(Some(vb)));
		
		return retval;		
	}

	pub fn get_nargs(&self, key :&str) -> Option<Nargs> {
		let ks :String = String::from(key);
		if !self.data.contains_key(&ks) {
			return None;
		}

		match self.data.get(&ks) {
			Some(v) => {
				match v {
					KeyVal::NArgVal(kv2) => {
						match kv2 {
							Some(kv3) => {
								return Some(kv3.clone());
							},
							_ => {
								return None;
							}
						}						
					},
					_ => {return None;},
				}
			},
			_ =>  {
				return None;
			}
		}		
	}

	pub fn set_jsonval(&mut self, key :&str,val :&Value) -> bool {
		let mut retval :bool = true;
		let ks :String = String::from(key);
		let vb :Value = val.clone();
		if self.data.contains_key(&ks) {
			retval = false;
			self.data.remove(&ks);
		}
		self.data.insert(ks,KeyVal::JsonVal(Some(vb)));
		return retval;
	}

	pub fn get_jsonval(&self, key :&str) -> Option<Value> {
		let ks :String = String::from(key);
		if !self.data.contains_key(&ks) {
			return None;
		}

		match self.data.get(&ks) {
			Some(v) => {
				match v {
					KeyVal::JsonVal(kv2) => {
						match kv2 {
							Some(kv3) => {
								return Some(kv3.clone());
							},
							_ => {
								return None;
							}
						}						
					},
					_ => {return None;},
				}
			},
			_ =>  {
				return None;
			}
		}
	}


	pub fn new() -> KeyData {
		let mut retval = KeyData{ data : HashMap::new() };
		retval.reset();
		return retval;
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
	__iscmd :BoolNone,
	__isflag :BoolNone,
	__type :String,
	__attr :KeyAttr,
	__nochange :bool,
	__longprefix :String,
	__shortprefix :String,
}

impl Key {
	fn __reset(&mut self) {
		self.__value = Value::Null;
		self.__prefix = String::from("");
		self.__flagname = String::from("");
		self.__helpinfo = String::from("");
		self.__shortflag = String::from("");
		//self.__nargs = Nargs::None;
		self.__varname = String::from("");
		self.__cmdname = String::from("");
		self.__function = String::from("");
		self.__origkey = String::from("");
		self.__iscmd = BoolNone::None;
		self.__isflag = BoolNone::None; 
		self.__type = String::from("");
		self.__attr = KeyAttr::new("");
		self.__nochange = false;
		self.__longprefix = String::from("");
		self.__shortprefix = String::from("");
		return;
	}
}