use serde_json::Value;
use std::collections::HashMap;
use regex::Regex;
use std::fmt::{Debug};


use std::fmt;
use std::error::Error;
use std::boxed::Box;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

use super::{extargs_error_class,extargs_new_error};
//use super::logger::{extargs_debug_out,extargs_log_get_timestamp};
//use super::{extargs_log_trace};




extargs_error_class!{KeyAttrError}
extargs_error_class!{KeyError}

pub enum Nargs {	
	Argtype(String),
	Argnum(i32),
}

impl Nargs {
	fn string(&self) -> String {
		let retstr:String ;
		match self {
			Nargs::Argtype(s) => {
				retstr = format!("{}",s);
			},
			Nargs::Argnum(i) => {
				retstr = format!("{}", i);
			},
		}

		return retstr;
	}
}

impl Debug for Nargs {
	fn fmt(&self,f :&mut fmt::Formatter) -> fmt::Result {
		write!(f,"{}",self.string())
	}
}

impl PartialEq for Nargs {
	fn eq (&self, other :&Self) -> bool {
		let mut retval :bool = false;
		match self {
			Nargs::Argtype(s) => {
				match other {
					Nargs::Argtype(os) => {
						if s == os {
							retval = true;
						}
					},
					_ => {},
				}
			},
			Nargs::Argnum(i) => {
				match other {
					Nargs::Argnum(oi) => {
						if i == oi {
							retval = true;
						}
					},
					_ => {},
				}
			}
		}
		return retval;
	}
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

pub struct KeyAttr {
	__splitchar :char,
	__obj :HashMap<String,String>,
}


impl KeyAttr {
	fn new_json(val :&Value) -> Result<KeyAttr, Box<dyn Error>> {
		let mut kattr = KeyAttr {
			__splitchar  : ';',
			__obj : HashMap::new(),
		};
		match val {
			Value::Object(_v) => {
				for (k,v) in val.as_object().unwrap() {
					match v {
						Value::String(sv) => {
							kattr.__obj.insert(format!("{}",k), format!("{}",sv));
						},
						_ => {
							extargs_new_error!{KeyError,"[{}] not string type", v}
						}
					}
				}
			},
			_ => {
				extargs_new_error!(KeyError,"{:?} not valid object", val)
			},
		}

		Ok(kattr)
	}

	fn new(_attr :&str) -> Result<KeyAttr,Box<dyn Error>> {
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
					extargs_new_error!{KeyAttrError,"not support char [{}]", c}
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
		return Ok(kattr);
	}

	pub fn get_keys(&self) -> Vec<String> {
		let mut retvec :Vec<String> = Vec::new();
		for (k,_) in &(self.__obj) {
			retvec.push(String::from(k));
		}
		return retvec;
	}

	pub fn string(&self) -> String {
		let mut retstr :String;
		let mut v:Vec<_> = (&(self.__obj)).into_iter().collect();
		let mut i:usize;
		v.sort_by(|x,y|x.0.cmp(&y.0));

		retstr = "".to_string();
		i = 0 ;
		while i < v.len() {
			if i > 0 {
				retstr.push_str("\n");
			}
			retstr.push_str(&format!("[{}]=[{}]", v[i].0,v[i].1));
			i = i + 1;
		}
		return retstr;
	}

	pub fn get_attr(&self,name :&str) -> String {
		match self.__obj.get(name) {
			Some(v) => { return v.to_string();},
			None => {return String::from("");}
		}
	}
}

impl PartialEq for KeyAttr {
	fn eq(&self,other :&Self) -> bool {
		let mut retval :bool = true;
		let sks :Vec<String> = self.get_keys();
		let oks :Vec<String> = other.get_keys();
		let mut sv :String;
		let mut ov :String;
		for v in sks {
			sv = self.get_attr(&v);
			ov = other.get_attr(&v);
			if sv != ov {
				retval = false;
			}
		}
		for v in oks {
			sv = self.get_attr(&v);
			ov = other.get_attr(&v);
			if sv != ov {
				retval = false;
			}			
		}

		return retval;
	}
}

impl Clone for KeyAttr {
	fn clone(&self) -> KeyAttr {
		let mut retattr :KeyAttr = KeyAttr{__splitchar : self.__splitchar, __obj : HashMap::new(),};
		for (k,v) in &(self.__obj) {
			retattr.__obj.insert(String::from(k),String::from(v));
		}
		return retattr;
	}
}

impl Debug for KeyAttr {
	fn fmt(&self,f :&mut fmt::Formatter) -> fmt::Result {
		write!(f,"{}",self.string())
	}
}


pub const KEYWORD_STRING :&str = "string";
pub const KEYWORD_DICT :&str = "dict";
pub const KEYWORD_LIST :&str = "list";
pub const KEYWORD_BOOL :&str = "bool";
pub const KEYWORD_INT :&str = "int";
pub const KEYWORD_FLOAT :&str = "float";
//const KEYWORD_LONG :&str = "long";
pub const KEYWORD_ARGS :&str = "args";
pub const KEYWORD_HELP :&str = "help";
pub const KEYWORD_JSONFILE :&str = "jsonfile";
pub const KEYWORD_COUNT :&str = "count";
pub const KEYWORD_DOLLAR_SIGN :&str = "$";
pub const KEYWORD_SUBNARGS :&str = "subnargs";
pub const KEYWORD_SUBCOMMAND :&str = "subcommand";
pub const KEYWORD_PLUS_SIGN :&str = "+";
pub const KEYWORD_STAR_SIGN :&str = "*";
pub const KEYWORD_QUESTION_SIGN :&str = "?";


#[derive(Clone)]
pub struct TypeClass {
	typeval : String,
}

impl TypeClass {
	fn new(v :&Value) -> TypeClass {
		let tv :String;
		match v {
			Value::String(_)  => {tv = String::from(KEYWORD_STRING);},
			Value::Object(_) => {tv = String::from(KEYWORD_DICT);},
			Value::Array(_) => { tv = String::from(KEYWORD_LIST);},
			Value::Bool(_) => {tv = String::from(KEYWORD_BOOL);},
			Value::Number(n) => {
				if n.is_i64() || n.is_u64() {
					tv = String::from(KEYWORD_INT);
				} else {
					tv = String::from(KEYWORD_FLOAT);
				}
			},
			Value::Null => {tv = String::from(KEYWORD_STRING);},
		}
		return TypeClass{typeval : tv,};
	}


	fn set_type(&mut self,val :&str) {
		self.typeval = String::from(val);
		return;
	}

	fn get_type(&self) -> String {
		return format!("{}",self.typeval);
	}

	fn string(&self) -> String {
		return format!("{}",self.typeval);
	}

}

pub const KEYWORD_VALUE :&str = "value";
pub const KEYWORD_PREFIX :&str = "prefix";
const KEYWORD_FLAGNAME :&str = "flagname";
const KEYWORD_HELPINFO :&str = "helpinfo";
const KEYWORD_SHORTFLAG :&str = "shortflag";
pub const KEYWORD_NARGS :&str = "nargs";
pub const KEYWORD_VARNAME :&str = "varname";
const KEYWORD_CMDNAME :&str = "cmdname";
pub const KEYWORD_COMMAND :&str = "command";
const KEYWORD_FUNCTION :&str = "function";
const KEYWORD_ORIGKEY :&str = "origkey";
const KEYWORD_ISCMD :&str = "iscmd";
const KEYWORD_ISFLAG :&str = "isflag";
const KEYWORD_TYPE :&str = "type";
pub const KEYWORD_ATTR :&str = "attr";
const KEYWORD_LONGPREFIX :&str = "longprefix";
const KEYWORD_SHORTPREFIX :&str = "shortprefix";
pub const KEYWORD_LONGOPT :&str = "longopt";
pub const KEYWORD_SHORTOPT :&str = "shortopt";
const KEYWORD_OPTDEST :&str = "optdest";
const KEYWORD_NEEDARG :&str = "needarg";
const KEYWORD_BLANK :&str  = "";

const KEYWORD_NOCHANGE :&str  = "nochange";

pub const FLAGSPECIAL : &'static [&'static str] = &[KEYWORD_VALUE,KEYWORD_PREFIX];
pub const FLAGWORDS :&'static [&'static str] = &[KEYWORD_FLAGNAME,KEYWORD_HELPINFO,KEYWORD_SHORTFLAG,KEYWORD_NARGS,KEYWORD_VARNAME];
pub const CMDWORDS :&'static [&'static str] = &[KEYWORD_CMDNAME,KEYWORD_FUNCTION,KEYWORD_HELPINFO];
pub const OTHERWORDS :&'static [&'static str] = &[KEYWORD_ORIGKEY,KEYWORD_ISCMD,KEYWORD_ISFLAG,KEYWORD_TYPE,KEYWORD_ATTR,KEYWORD_LONGPREFIX,KEYWORD_SHORTPREFIX];
pub const FORMWORDS :&'static [&'static str] = &[KEYWORD_LONGOPT,KEYWORD_SHORTOPT,KEYWORD_OPTDEST,KEYWORD_NEEDARG];


fn in_array_word( key :&str, sarr :&[&str]) -> bool {
	let mut retval :bool = false;

	for k in sarr {
		if (*k) == key {
			retval = true;
			break;
		}
	}
	retval
}

#[derive(Clone)]
pub enum KeyVal {
	StrVal(Option<String>),
	BoolVal(Option<bool>),
	JsonVal(Option<Value>),
	KeyAttrVal(Option<KeyAttr>),
	NArgVal(Option<Nargs>),
	TypeVal(Option<TypeClass>),
}

impl KeyVal {
	pub fn string(&self) -> String {
		let mut rets :String = String::from("");
		match self {
			KeyVal::StrVal(Some(v)) => {
				rets.push_str(&(format!("strval({})",v)[..]));
			},
			KeyVal::BoolVal(Some(v)) => {
				rets.push_str(&(format!("boolval({:?})",v)[..]));
			},
			KeyVal::JsonVal(Some(v)) => {
				rets.push_str(&(format!("jsonval({:?})",v)[..]));
			},
			KeyVal::KeyAttrVal(Some(v)) => {
				rets.push_str(&(format!("keyattr({})",v.string())[..]));
			},
			KeyVal::NArgVal(Some(v)) => {
				rets.push_str(&(format!("nargval({})",v.string())[..]));
			},
			KeyVal::TypeVal(Some(v)) => {
				rets.push_str(&(format!("typeval({})",v.string())[..]));
			},
			_ => {
				rets.push_str(&(format!("none")[..]));
			}
		}
		return rets;
	}
}

#[derive(Clone)]
struct KeyData {
	data :HashMap<String,KeyVal>,
}

impl KeyData {
	pub fn reset(&mut self) {
		let typeval = TypeClass::new(&(Value::Null));
		self.data.clear();
		self.data.insert(String::from(KEYWORD_VALUE),KeyVal::JsonVal(None));
		self.data.insert(String::from(KEYWORD_PREFIX),KeyVal::StrVal(None));
		self.data.insert(String::from(KEYWORD_FLAGNAME),KeyVal::StrVal(None));
		self.data.insert(String::from(KEYWORD_HELPINFO),KeyVal::StrVal(None));
		self.data.insert(String::from(KEYWORD_SHORTFLAG),KeyVal::StrVal(None));
		self.data.insert(String::from(KEYWORD_NARGS),KeyVal::NArgVal(None));
		self.data.insert(String::from(KEYWORD_VARNAME),KeyVal::StrVal(None));
		self.data.insert(String::from(KEYWORD_CMDNAME),KeyVal::StrVal(None));
		self.data.insert(String::from(KEYWORD_FUNCTION),KeyVal::StrVal(None));
		self.data.insert(String::from(KEYWORD_ORIGKEY),KeyVal::StrVal(None));
		self.data.insert(String::from(KEYWORD_ISCMD),KeyVal::BoolVal(None));
		self.data.insert(String::from(KEYWORD_ISFLAG),KeyVal::BoolVal(None));
		self.data.insert(String::from(KEYWORD_TYPE),KeyVal::TypeVal(Some(typeval)));
		self.data.insert(String::from(KEYWORD_ATTR),KeyVal::KeyAttrVal(None));
		self.data.insert(String::from(KEYWORD_NOCHANGE),KeyVal::BoolVal(Some(false)));
		self.data.insert(String::from(KEYWORD_LONGPREFIX),KeyVal::StrVal(None));
		self.data.insert(String::from(KEYWORD_SHORTPREFIX),KeyVal::StrVal(None));
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

	pub fn get_type(&self,key :&str) -> String {
		let ks :String = String::from(key);
		let mut retstr :String = format!("{}",KEYWORD_STRING);
		match self.data.get(&ks) {
			Some(v) => {
				match v {
					KeyVal::TypeVal(v2) => {
						match v2 {
							Some(v3) => {
								retstr = v3.get_type();
							},
							_ => {
								
							}
						}
					},
					_ => {
					},
				}
			},
			_ => {
			},
		}
		return retstr;
	}

	pub fn set_type(&mut self,key :&str,c :&str)  {
		let ks :String = String::from(key);

		match self.data.get_mut(&ks) {
			Some(v) => {
				match v {
					KeyVal::TypeVal(v2) => {
						match v2 {
							Some(v3) => {
								v3.set_type(c)
							},
							_ => {
							}
						}
					},
					_ => {
					},
				}
			},
			_ => {
			},
		}
		return;		
	}

	pub fn get_string(&self, key :&str) -> Option<String> {
		let ks :String = String::from(key);

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

	pub fn get_string_value(&self, key :&str) -> String {
		let ks :String = String::from(key);
		let mut rets :String = String::from("");

		match self.data.get(&ks) {
			Some(v) => {
				match v {
					KeyVal::StrVal(kv2) => {
						match kv2 {
							Some(sv) => {
								rets = sv.clone();
							},
							_ => {

							},
						}						
						
					},
					_ => {},
				}
			},
			_ =>  {
			}
		}
		return rets;		
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

	pub fn get_bool_value(&self, key :&str) -> bool {
		let ks :String = String::from(key);
		let mut retb :bool = false;
		match self.data.get(&ks) {
			Some(v) => {
				match v {
					KeyVal::BoolVal(kv2) => {
						match kv2 {
							Some(sv) => {								
								retb = *sv;
							},
							_ => {},
						}						
						
					},
					KeyVal::JsonVal(kv3) => {
						match kv3 {
							Some(kv4) => {
								match kv4 {
									Value::Bool(kv5) => {										
										retb = *kv5;
									},
									_ => {										
									}
								}
								
							},
							_ => {								
							}
						}
					},
					_ => {						
					},
				}
			},
			_ =>  {
			}
		}		
		return retb;
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
	pub fn get_nargs_value(&self, key :&str) -> Nargs {
		let ks :String = String::from(key);
		let mut retargs :Nargs = Nargs::Argnum(0);
		match self.data.get(&ks) {
			Some(v) => {
				match v {
					KeyVal::NArgVal(kv2) => {
						match kv2 {
							Some(kv3) => {
								retargs = kv3.clone();
							},
							_ => {
							}
						}						
					},
					_ => {},
				}
			},
			_ =>  {
			}
		}
		return retargs;
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

	pub fn get_jsonval_value(&self, key :&str) -> Value {
		let ks :String = String::from(key);
		let mut retv :Value = Value::Null;
		match self.data.get(&ks) {
			Some(v) => {
				match v {
					KeyVal::JsonVal(kv2) => {
						match kv2 {
							Some(kv3) => {
								retv = kv3.clone();
							},
							_ => {
							}
						}						
					},
					_ => {},
				}
			},
			_ =>  {
			}
		}
		return retv;
	}

	pub fn set_keyattr(&mut self,key :&str,val :&KeyAttr)  -> bool {
		let mut retval :bool = true;
		let ks :String = String::from(key);


		if self.data.contains_key(&ks) {
			retval = false;
			self.data.remove(&ks);
		}
		self.data.insert(ks,KeyVal::KeyAttrVal(Some(val.clone())));
		
		return retval;
	}

	pub fn get_keyattr(&self,key :&str) -> Option<KeyAttr> {
		let ks :String = String::from(key);

		match self.data.get(&ks) {
			Some(v) => {
				match v {
					KeyVal::KeyAttrVal(kv2) => {
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

struct InnerExtKeyParse {
	keydata : KeyData,
	__helpexpr :Regex,
	__cmdexpr : Regex,
	__prefixexpr : Regex,
	__funcexpr : Regex,
	__flagexpr : Regex,
	__mustflagexpr : Regex,
	__attrexpr : Regex,
}

fn compile_regex(expr :&str) -> Result<Regex,Box<dyn Error>> {
	match Regex::new(expr) {
		Err(e) => {
			extargs_new_error!(KeyError,"compile [{}] error[{:?}]",expr,e)
		},
		Ok(v) => {
			Ok(v)
		},
	}

}

impl InnerExtKeyParse {
	fn __reset(&mut self) {
		self.keydata = KeyData::new();
		return;
	}

	fn __compare_value_type(&self,types :&str) -> bool {
		let mut retval :bool = false;
		match self.get_value_v() {
			Value::Null => {
				if types == KEYWORD_STRING {
					retval = true;
				}
			},
			Value::String(_v) => {
				if types == KEYWORD_STRING {
					retval = true;
				}
			},
			Value::Number(v) => {
				if (v.is_u64() || v.is_i64() ) && types == KEYWORD_INT {
					retval = true;
				} else if (!v.is_u64() && !v.is_i64() ) && types == KEYWORD_FLOAT {
					retval = true;
				}
			},
			Value::Object(_v) => {
				if types == KEYWORD_DICT {
					retval = true;
				}
			},
			Value::Array(_v) => {
				if types == KEYWORD_LIST {
					retval = true;
				}
			},
			Value::Bool(_v) => {
				if types == KEYWORD_BOOL {
					retval = true;
				}
			},
		}

		return retval;
	}

	fn __form_word_num(&self,key :&str) -> i32 {
		let bval :bool;
		let bopt :Option<bool>;
		let sval :String;
		let mut retval :i32 = 0; 
		if key == KEYWORD_NEEDARG {
			bopt = self.keydata.get_bool(KEYWORD_ISFLAG);
			match bopt {
				None => {
					return retval;
				},
				Some(v) => {
					bval = v;
				},
			}

			if !bval {
				return retval;
			}

			sval = self.keydata.get_type(KEYWORD_TYPE);

			if sval == KEYWORD_INT || sval == KEYWORD_LIST || 
			  //sval == KEYWORD_LONG || sval == KEYWORD_FLOAT ||
			  sval == KEYWORD_FLOAT ||
			  sval == KEYWORD_STRING || sval == KEYWORD_JSONFILE {
			  	retval = 1;
			  }
			}
			return retval;
		}

		fn __form_word_str(&self,key :&str) -> Result<String,Box<dyn Error>> {
			let bval :bool;
			let sval :String;
			let mut retval :String = String::from("");

			if key == KEYWORD_LONGOPT {
				if !self.keydata.get_bool_value(KEYWORD_ISFLAG) ||  
				self.keydata.get_string_value(KEYWORD_FLAGNAME) == KEYWORD_BLANK ||
				self.keydata.get_type(KEYWORD_TYPE) == KEYWORD_ARGS	{
					extargs_new_error!{KeyError,"can not set ({}) longopt",self.keydata.get_string_value(KEYWORD_ORIGKEY)}
				}
				retval = format!("{}",self.keydata.get_string_value(KEYWORD_LONGPREFIX));
				if self.keydata.get_type(KEYWORD_TYPE) == KEYWORD_BOOL {
					match self.keydata.get_jsonval_value(KEYWORD_VALUE){
						Value::Bool(v) => {
							bval = v;
						},
						_ => {
							bval = false;
						}
					}
					if bval {
						retval.push_str("no-");
					}				
				}

				sval = self.keydata.get_string_value(KEYWORD_PREFIX);
				if sval.len() > 0 && 
				self.keydata.get_type(KEYWORD_TYPE) != KEYWORD_HELP {
					retval.push_str(&(format!("{}_",sval)[..]));
				}
				retval.push_str(&(format!("{}",self.keydata.get_string_value(KEYWORD_FLAGNAME))[..]));
				if !self.keydata.get_bool_value(KEYWORD_NOCHANGE) {
					retval = retval.to_lowercase();
					retval = retval.replace("_","-");
				}
			} else if key == KEYWORD_SHORTOPT {
				if ! self.keydata.get_bool_value(KEYWORD_ISFLAG) || 
				self.keydata.get_string_value(KEYWORD_FLAGNAME) == KEYWORD_BLANK || 
				self.keydata.get_type(KEYWORD_TYPE)  == KEYWORD_ARGS {
					extargs_new_error!{KeyError,"can not set ({}) shortopt",self.keydata.get_string_value(KEYWORD_ORIGKEY)}
				}
				if self.keydata.get_string_value(KEYWORD_SHORTFLAG).len() > 0 {
					retval = format!("{}{}",self.keydata.get_string_value(KEYWORD_SHORTPREFIX),
						self.keydata.get_string_value(KEYWORD_SHORTFLAG));
				}
			} else if key == KEYWORD_OPTDEST {
				if ! self.keydata.get_bool_value(KEYWORD_ISFLAG) || 
				self.keydata.get_string_value(KEYWORD_FLAGNAME) == KEYWORD_BLANK || 
				self.keydata.get_type(KEYWORD_TYPE)  == KEYWORD_ARGS {
					extargs_new_error!{KeyError,"can not set ({}) optdest",self.keydata.get_string_value(KEYWORD_ORIGKEY)}
				}
				if self.keydata.get_string_value(KEYWORD_PREFIX).len() > 0 {
					retval.push_str(&(format!("{}_",self.keydata.get_string_value(KEYWORD_PREFIX))[..]));
				}
				retval.push_str(&(format!("{}",self.keydata.get_string_value(KEYWORD_FLAGNAME))[..]));
				if !self.keydata.get_bool_value(KEYWORD_NOCHANGE) {
					retval = retval.to_lowercase();
				}
				retval = retval.replace("-","_");
			}
			return Ok(retval);
		}

		fn __eq_name__(&self,other :&InnerExtKeyParse,name :&str) -> bool {
			let mut ret :bool = false;
			let sjval  :Value;
			let ojval :Value;
			let sjopt :Option<Value>;
			let ojopt :Option<Value>;
			let ssval :String;
			let osval :String;
			let ssopt :Option<String>;
			let osopt :Option<String>;
			let snval :Nargs;
			let onval :Nargs;
			let snopt :Option<Nargs>;
			let onopt :Option<Nargs>;
			let sbval :bool;
			let obval :bool;
			let sbopt :Option<bool>;
			let obopt :Option<bool>;
			let skval :KeyAttr;
			let okval :KeyAttr;
			let skopt :Option<KeyAttr>;
			let okopt :Option<KeyAttr>;
			if name == KEYWORD_VALUE {
				sjopt = self.keydata.get_jsonval(name);
				ojopt = other.keydata.get_jsonval(name);
				match sjopt {
					Some(v) => {sjval = v;},
					None => {
						match ojopt {
							None => {  return true; },
							_ => {
								return false;
							}
						}
					}
				}
				match ojopt {
					Some(v) => {ojval = v;},
					_ => {
						return false;
					}
				}

				if ojval == sjval {
					ret = true;
				}
			} else if name == KEYWORD_PREFIX || name == KEYWORD_FLAGNAME || 
			name == KEYWORD_HELPINFO || name == KEYWORD_SHORTFLAG || 
			name == KEYWORD_VARNAME || name == KEYWORD_CMDNAME ||
			name == KEYWORD_FUNCTION || name == KEYWORD_ORIGKEY || 
			name == KEYWORD_LONGPREFIX || name == KEYWORD_SHORTPREFIX || name == KEYWORD_LONGOPT || 
			name == KEYWORD_SHORTOPT || name == KEYWORD_OPTDEST {
				ssopt = self.keydata.get_string(name);
				osopt = other.keydata.get_string(name);
				match ssopt {
					Some(v) => {ssval = v;},
					None => {
						match osopt {
							None => {  return true; },
							_ => {
								return false;
							}
						}
					}
				}
				match osopt {
					Some(v) => {osval = v;},
					_ => {
						return false;
					}
				}

				if osval == ssval {
					ret = true;
				}

			} else if name == KEYWORD_TYPE {
				ssval = self.keydata.get_type(name);
				osval = self.keydata.get_type(name);
				if osval == ssval {
					ret = true;
				}

			}else if name == KEYWORD_NARGS {
				snopt = self.keydata.get_nargs(name);
				onopt = other.keydata.get_nargs(name);
				match snopt {
					Some(v) => {snval = v;},
					None => {
						match onopt {
							None => {  return true; },
							_ => {
								return false;
							}
						}
					}
				}
				match onopt {
					Some(v) => {onval = v;},
					_ => {
						return false;
					}
				}

				if onval == snval {
					ret = true;
				}
			}  else if name == KEYWORD_ISCMD || name == KEYWORD_ISFLAG {
				sbopt = self.keydata.get_bool(name);
				obopt = other.keydata.get_bool(name);
				match sbopt {
					Some(v) => {sbval = v;},
					None => {
						match obopt {
							None => {  return true; },
							_ => {
								return false;
							}
						}
					}
				}
				match obopt {
					Some(v) => {obval = v;},
					_ => {
						return false;
					}
				}

				if obval == sbval {
					ret = true;
				}
			} else if name == KEYWORD_ATTR {
				skopt = self.keydata.get_keyattr(name);
				okopt = other.keydata.get_keyattr(name);
				match skopt {
					Some(v) => {skval = v;},
					None => {
						match okopt {
							None => {  return true; },
							_ => {
								return false;
							}
						}
					}
				}
				match okopt {
					Some(v) => {okval = v;},
					_ => {
						return false;
					}
				}

				if okval == skval {
					ret = true;
				}
			}

			return ret;
		}

		pub fn string(&self) -> String {
			let mut retstr :String;
			let mut s :String;
			retstr = String::from("{");
			retstr.push_str(&(format!("<{}:{}>",KEYWORD_TYPE, self.keydata.get_type(KEYWORD_TYPE))[..]));
			retstr.push_str(&(format!("<{}:{}>",KEYWORD_ORIGKEY,self.keydata.get_string_value(KEYWORD_ORIGKEY))[..]));
			if self.keydata.get_bool_value(KEYWORD_ISCMD) {
				retstr.push_str(&(format!("<cmdname:{}>",self.keydata.get_string_value(KEYWORD_CMDNAME))[..]));
				s = self.keydata.get_string_value(KEYWORD_FUNCTION);
				if s.len() > 0 {
					retstr.push_str(&(format!("<{}:{}>",KEYWORD_FUNCTION,s)[..]));
				}

				s = self.keydata.get_string_value(KEYWORD_HELPINFO);
				if s.len() > 0 {
					retstr.push_str(&(format!("<{}:{}>", KEYWORD_HELPINFO,s)[..]));
				}

				s = self.keydata.get_string_value(KEYWORD_PREFIX);
				if s.len() > 0 {
					retstr.push_str(&(format!("<{}:{}>", KEYWORD_PREFIX,s)[..]));
				}
			}

			if self.keydata.get_bool_value(KEYWORD_ISFLAG) {
				s = self.keydata.get_string_value(KEYWORD_FLAGNAME);
				if s.len() > 0 {
					retstr.push_str(&(format!("<{}:{}>", KEYWORD_FLAGNAME,s)[..]));
				}

				s = self.keydata.get_string_value(KEYWORD_SHORTFLAG);
				if s.len() > 0 {
					retstr.push_str(&(format!("<{}:{}>",KEYWORD_SHORTFLAG,s)[..]));
				}

				s = self.keydata.get_string_value(KEYWORD_PREFIX);
				if s.len() > 0 {
					retstr.push_str(&(format!("<{}:{}>", KEYWORD_PREFIX,s)[..]));
				}

				match self.keydata.get_nargs(KEYWORD_NARGS) {
					Some(v) => {
						retstr.push_str(&(format!("<{}:{}>",KEYWORD_NARGS, v.string())[..]));
					},
					_ => {

					},
				}

				s = self.keydata.get_string_value(KEYWORD_VARNAME);
				if s.len() > 0 {
					retstr.push_str(&(format!("<{}:{}>", KEYWORD_VARNAME,s)[..]));
				}

				match self.keydata.get_jsonval(KEYWORD_VALUE) {
					Some(v) => {
						match v {
							Value::Null => {

							},
							_ => {
								retstr.push_str(&(format!("<{}:{:?}>", KEYWORD_VALUE,v)[..]));		
							}
						}

					},
					_ => {

					},
				}

				s = self.keydata.get_string_value(KEYWORD_LONGPREFIX);
				retstr.push_str(&(format!("<{}:{}>", KEYWORD_LONGPREFIX,s)[..]));
				s = self.keydata.get_string_value(KEYWORD_SHORTPREFIX);
				retstr.push_str(&(format!("<{}:{}>", KEYWORD_SHORTPREFIX,s)[..]));
			}

			match self.keydata.get_keyattr(KEYWORD_ATTR) {
				Some(v) => {
					retstr.push_str(&(format!("<{}:{}>",KEYWORD_ATTR,v.string())[..]));
				},
				_ => {
					retstr.push_str(&(format!("<{}:>",KEYWORD_ATTR)));
				},
			}

			return retstr;
		}

		fn __validate(&mut self) -> Result<bool,Box<dyn Error>>{
			let mut s:String;
			let mut s2 :String;
			let mut bnone :bool;
			let origkey :String = self.keydata.get_string_value(KEYWORD_ORIGKEY);
			if self.keydata.get_bool_value(KEYWORD_ISFLAG) {
				assert!(!self.keydata.get_bool_value(KEYWORD_ISCMD));
				s = self.keydata.get_string_value(KEYWORD_FUNCTION);
				if s.len() > 0 {
					extargs_new_error!{KeyError,"({}) can not accept function", origkey}
				}

				bnone = false;
				match self.keydata.get_string(KEYWORD_FLAGNAME) {
					Some(_v) => {
					},
					None => {
						bnone = true;
					}
				}

				if self.keydata.get_type(KEYWORD_TYPE) == KEYWORD_DICT && !bnone {
					extargs_new_error!{KeyError,"({}) flag can not accept dict",origkey}
				}

				s = self.keydata.get_type(KEYWORD_TYPE);
				if !self.__compare_value_type(&(s[..])) && s != KEYWORD_COUNT && s != KEYWORD_HELP && 
				s != KEYWORD_JSONFILE {
					extargs_new_error!{KeyError,"({}) value ({:?}) not match type ({})",origkey,self.keydata.get_jsonval_value(KEYWORD_VALUE),s}
				}
				s = self.keydata.get_string_value(KEYWORD_FLAGNAME);
				if s.len() == 0 {
					s = self.keydata.get_string_value(KEYWORD_PREFIX);
					if s.len() == 0{
						extargs_new_error!{KeyError,"({}) should at least for prefix", origkey}
					}
					self.keydata.set_type(KEYWORD_TYPE,KEYWORD_PREFIX);
					match self.keydata.get_jsonval_value(KEYWORD_VALUE) {
						Value::Object(_v) => {},
						_ => {
							extargs_new_error!{KeyError,"({}) should used dict to make prefix",origkey}
						},
					}
					s = self.keydata.get_string_value(KEYWORD_HELPINFO);
					if s.len() > 0 {
						extargs_new_error!{KeyError,"({}) should not have help info",origkey}
					}
					s = self.keydata.get_string_value(KEYWORD_SHORTFLAG);
					if s.len() > 0 {
						extargs_new_error!{KeyError,"({}) should not set shortflag",origkey}
					}
				} else if s == KEYWORD_DOLLAR_SIGN {
					self.keydata.set_type(KEYWORD_TYPE,KEYWORD_ARGS);
					s = self.keydata.get_string_value(KEYWORD_SHORTFLAG);
					if s.len() > 0 {
						extargs_new_error!{KeyError,"({}) can not set shortflag for args",origkey}
					}
				}

				s = self.keydata.get_string_value(KEYWORD_SHORTFLAG);
				if s.len() > 1 {
					extargs_new_error!{KeyError,"({}) can not accept ({}) for shortflag",origkey,s}
				}

				s = self.keydata.get_type(KEYWORD_TYPE);
				if s == KEYWORD_BOOL {
					match self.keydata.get_nargs(KEYWORD_NARGS) {
						Some(Nargs::Argnum(iv)) => {
							if iv != 0 {
								extargs_new_error!{KeyError,"bool type ({}) can not accept not 0 nargs",origkey}
							}
						},
						_ => {},
					}
					self.keydata.set_nargs(KEYWORD_NARGS,&(Nargs::Argnum(0)));
				} else if s == KEYWORD_HELP {
					match self.keydata.get_nargs(KEYWORD_NARGS) {
						Some(Nargs::Argnum(iv)) => {
							if iv != 0 {
								extargs_new_error!{KeyError,"help type ({}) can not accept not 0 nargs",origkey}
							}
						},
						_ => {},
					}
					self.keydata.set_nargs(KEYWORD_NARGS,&(Nargs::Argnum(0)));
				} else if s != KEYWORD_PREFIX && s != KEYWORD_COUNT && self.keydata.get_string_value(KEYWORD_FLAGNAME) != KEYWORD_DOLLAR_SIGN {
					if self.keydata.get_string_value(KEYWORD_FLAGNAME) != KEYWORD_DOLLAR_SIGN {
						match self.keydata.get_nargs(KEYWORD_NARGS) {
							Some(Nargs::Argnum(iv)) => {
								if iv != 1 {
									extargs_new_error!{KeyError,"({})only $ can accept nargs option [{}]",origkey,iv}
								}
							},
							_ => {},
						}
						self.keydata.set_nargs(KEYWORD_NARGS,&(Nargs::Argnum(1)));
					}
				} else {
					if self.keydata.get_string_value(KEYWORD_FLAGNAME) == KEYWORD_DOLLAR_SIGN {
						match self.keydata.get_nargs(KEYWORD_NARGS) {
							None => {
								self.keydata.set_nargs(KEYWORD_NARGS,&(Nargs::Argtype(String::from(KEYWORD_STAR_SIGN))));
							},
							_ => {},
						}
					}
				}
			} else {
				s = self.keydata.get_string_value(KEYWORD_CMDNAME);
				if s.len() == 0 {
					extargs_new_error!{KeyError,"({}) not set cmdname",origkey}
				}

				s = self.keydata.get_string_value(KEYWORD_SHORTFLAG);
				if s.len() > 0 {
					extargs_new_error!{KeyError,"({}) has shortflag ({})",origkey,s}
				}

				match self.keydata.get_nargs(KEYWORD_NARGS) {
					None => {},
					Some(e) => {
						extargs_new_error!{KeyError,"({}) has nargs ({})",origkey,e.string()}
					},
				}

				s = self.keydata.get_type(KEYWORD_TYPE);
				if s != KEYWORD_DICT {
					extargs_new_error!{KeyError,"({}) command must be dict",origkey}
				}

				s = self.keydata.get_string_value(KEYWORD_PREFIX);
				if s.len() == 0 {
					self.keydata.set_string(KEYWORD_PREFIX,KEYWORD_BLANK);
				}

				s = self.keydata.get_string_value(KEYWORD_PREFIX);
				if s.len() == 0 {
					s.push_str(self.keydata.get_string_value(KEYWORD_CMDNAME).as_str());
					self.keydata.set_string(KEYWORD_PREFIX,s.as_str());
				}

				self.keydata.set_type(KEYWORD_TYPE,KEYWORD_COMMAND);
			}

			s = self.keydata.get_string_value(KEYWORD_VARNAME);
			s2 = self.keydata.get_string_value(KEYWORD_FLAGNAME);
			if self.keydata.get_bool_value(KEYWORD_ISFLAG) && s.len() == 0 && 
			s2.len() > 0 {
				if s2 != KEYWORD_DOLLAR_SIGN {
					s2 = self.__form_word_str(KEYWORD_OPTDEST)?;
					self.keydata.set_string(KEYWORD_VARNAME,s2.as_str());
				} else {
					s2 = self.keydata.get_string_value(KEYWORD_PREFIX);
					if s2.len() > 0 {
						self.keydata.set_string(KEYWORD_VARNAME,KEYWORD_SUBNARGS);
					} else {
						self.keydata.set_string(KEYWORD_VARNAME,KEYWORD_ARGS);
					}
				}
			}

			return Ok(true);
		}

		fn __set_flag(&mut self, prefix :&str, key :&str,value :&Value) -> Result<bool,Box<dyn Error>> {
			let vtrue : bool = true;
			let vfalse :bool = false;
			let mut binvalue :bool = false;
			let mut s :String;
			self.keydata.set_bool(KEYWORD_ISFLAG,&vtrue);
			self.keydata.set_bool(KEYWORD_ISCMD,&vfalse);
			self.keydata.set_string(KEYWORD_ORIGKEY,key);

			match value {
				Value::Object(v2) => {
					match v2.get(&(format!("{}",KEYWORD_VALUE)[..])) {
						None => {

						},
						Some(v3) => {
							match v3 {
								Value::String(_v4) => {
									binvalue = true;
								},
								Value::Null => {
									binvalue = true;
								},
								_ => {
									binvalue =false;
								}
							}
						}
					}
				},
				_ => {
					binvalue = false;
				},
			}

			if !binvalue {
				self.keydata.set_jsonval(KEYWORD_VALUE,&(Value::Null));
				self.keydata.set_type(KEYWORD_TYPE,KEYWORD_STRING);
			}

			match value {
				Value::Object(v2) => {
					for (k,v) in v2 {
						if in_array_word(k,FLAGWORDS) {
							if k == KEYWORD_NARGS {
								let va :Nargs;
								match v {
									Value::String(vs) => {
										if vs != KEYWORD_PLUS_SIGN && vs != KEYWORD_STAR_SIGN && vs != KEYWORD_QUESTION_SIGN {
											extargs_new_error!{KeyError,"{} not in +?*",vs}
										}
										va = Nargs::Argtype(vs.to_string());
									},
									Value::Number(vi) => {
										if !vi.is_u64() {
											extargs_new_error!{KeyError,"{:?} not valid u64",v}
										}
										match vi.as_u64() {
											Some(v3) => {
												va = Nargs::Argnum(v3 as i32);		
											},
											None => {
												extargs_new_error!{KeyError,"{:?} not valid u64",v}
											}
										}

									},
									_ => {
										extargs_new_error!{KeyError,"{:?} not for int or string", v}
									}
								}
								self.keydata.set_nargs(KEYWORD_NARGS,&va);
							} else {
								match v {
									Value::String(vs) => {
										s = self.keydata.get_string_value(k);
										if s != &(vs[..]) && s.len() > 0 {
											extargs_new_error!{KeyError,"set ({}) for not equal value ({}) ({})",k,s,vs}
										}
										self.keydata.set_string(k,&(vs[..]));
									},
									_ => {
										extargs_new_error!{KeyError,"({})({})({:?}) can not take other than int or string ({})",key,k,v,TypeClass::new(v).get_type()}
									},
								}
							}
						} else if in_array_word(k,FLAGSPECIAL) {
							if k == KEYWORD_PREFIX {
								match v {
									Value::String(vs) => {
										self.keydata.set_string(KEYWORD_PREFIX,vs);
									},
									Value::Null =>  {
										self.keydata.set_string(KEYWORD_PREFIX,KEYWORD_BLANK);
									},
									_ => {
										extargs_new_error!{KeyError,"({}) prefix not string or none", k}
									}
								}
							} else if k == KEYWORD_VALUE {
								let vtype :TypeClass;
								match v {
									Value::Object(_v3) => {
										extargs_new_error!{KeyError,"{:?} object values",v}
									},
									_ => {

									},
								}
								self.keydata.set_jsonval(KEYWORD_VALUE,v);
								vtype = TypeClass::new(v);
								self.keydata.set_type(KEYWORD_TYPE,&(vtype.get_type()[..]));
							} else {
								extargs_new_error!{KeyError,"{} not valid key", k}
							}
						} else if k == KEYWORD_ATTR {
							match v {
								Value::String(vs) => {
									let vattr :KeyAttr = KeyAttr::new(vs)?;
									self.keydata.set_keyattr(KEYWORD_ATTR,&vattr);
								},
								Value::Object(_ov) => {
									let vattr :KeyAttr = KeyAttr::new_json(&v)?;
									self.keydata.set_keyattr(KEYWORD_ATTR,&vattr);
								}
								_ => {

								},
							}
						}
					}
				},
				_ => {

				},
			}

			if prefix.len() > 0 {
				self.keydata.set_string(KEYWORD_PREFIX,prefix);
			}
			Ok(true)
		}

		fn __parse(&mut self,prefix :&str, origkey :&str, value :&Value, isflag :bool,
			ishelp :bool, isjsonfile :bool) -> Result<bool,Box<dyn Error>> {
			let mut flagmode : bool;
			let mut cmdmode : bool;
			let mut flags :String;
			let mut s :String;
			let mut idx :usize;
			let mut sv ;
			let mut _splitre :Regex;
			let mut _sarr :Vec<&str>;
			let mut newprefix :String;
			let vtrue :bool = true;
			let vfalse :bool  = false;
			let mut bmatch : bool;

			flagmode = false;
			cmdmode = false;
			flags = format!("{}",KEYWORD_BLANK);
			self.keydata.set_string(KEYWORD_ORIGKEY,origkey);
			s = self.keydata.get_string_value(KEYWORD_ORIGKEY);
			sv = s.chars().as_str().bytes();
			if s.contains("$") {
				match sv.nth(0) {
					None => {
						extargs_new_error!{KeyError,"{} not get $",origkey}
					},
					Some(v) => {
						if v != ('$' as u8) {
							extargs_new_error!(KeyError,"({}) not right format for ($)",origkey)		
						}
					},
				}
				idx = 0;
				while idx < sv.len() {
					match sv.nth(idx) {
						None => {
							extargs_new_error!{KeyError,"{} can not get [{}]", origkey,idx}
						},
						Some(v) => {
							if v == ('$' as u8) {
								extargs_new_error!{KeyError,"({}) has ($) more than one",origkey}
							}
						}
					}
					idx += 1;
				}
			}

			if isflag || ishelp || isjsonfile {
				match self.__flagexpr.captures(origkey) {
					None => {
						flags = format!("");
					},
					Some(v) => {
						if v.len() > 1 {
							flags = format!("{}",v.get(1).map_or("", |m| m.as_str()));
						} else {
							flags = format!("");
						}
					},
				}

				if flags.len() == 0 {
					match self.__mustflagexpr.captures(origkey) {
						None => {
							flags = format!("");
						},
						Some(v) => {
							if v.len() > 1 {
								flags = format!("{}",v.get(1).map_or("", |m| m.as_str()));
							} else {
								flags = format!("");
							}
						}
					}
				}

				if flags.len() == 0  {
					s = format!("{}", origkey);
					sv = s.chars().as_str().bytes();
					match sv.nth(0) {
						None => {

						},
						Some(v) => {
							if v == ('$' as u8) {							
								self.keydata.set_string(KEYWORD_FLAGNAME,KEYWORD_DOLLAR_SIGN);
								flagmode = true;
							}
						}
					}
				}

				if flags.len() > 0 {
					if flags.contains("|") {
						_splitre = compile_regex("\\|")?;
						_sarr = _splitre.split(flags.as_str()).collect();
						if _sarr.len() > 2 || _sarr[1].len() != 1  || _sarr[0].len() <= 1 {
							extargs_new_error!{KeyError,"({}) ({})flag only accept (longop|l) format",origkey,flags}
						}
					//extargs_log_trace!("KEYWORD_FLAGNAME [{}]", _sarr[0]);
					self.keydata.set_string(KEYWORD_FLAGNAME,_sarr[0]);
					self.keydata.set_string(KEYWORD_SHORTFLAG,_sarr[1]);
				} else {
					//extargs_log_trace!("KEYWORD_FLAGNAME [{}]", flags.as_str());
					self.keydata.set_string(KEYWORD_FLAGNAME,flags.as_str());
				}
				flagmode = true;
			}
		} else {
			match self.__mustflagexpr.captures(origkey) {
				Some(m) => {
					if m.len() > 1 {
						flags = format!("");
						flags.push_str(&(m[1]));
						if flags.contains("|") {
							_splitre = compile_regex("\\|")?;
							_sarr = _splitre.split(flags.as_str()).collect();
							if _sarr.len() > 2 || _sarr[1].len() != 1 || _sarr[0].len() <= 1 {
								extargs_new_error!{KeyError,"({}) ({})flag only accept (longop|l) format",origkey,flags}
							}
							self.keydata.set_string(KEYWORD_FLAGNAME,_sarr[0]);
							self.keydata.set_string(KEYWORD_SHORTFLAG,_sarr[1]);
						} else {
							if flags.len() <= 1 {
								extargs_new_error!{KeyError,"({}) flag must have long opt",origkey}
							}
							//extargs_log_trace!("KEYWORD_FLAGNAME [{}]", flags.as_str());
							self.keydata.set_string(KEYWORD_FLAGNAME,flags.as_str());
						}
						flagmode = true;
					}
				},
				None => {
					s = self.keydata.get_string_value(KEYWORD_ORIGKEY);
					sv = s.chars().as_str().bytes();
					match sv.nth(0) {
						None => {
						},
						Some(v) => {
							if v == ('$' as u8) {
								//extargs_log_trace!("KEYWORD_FLAGNAME [{}]", KEYWORD_DOLLAR_SIGN);
								self.keydata.set_string(KEYWORD_FLAGNAME,KEYWORD_DOLLAR_SIGN);
								flagmode = true;
							}
						},
					}
				}
			}

			match self.__cmdexpr.captures(origkey) {
				Some(m) => {
					let mut cmds :String;
					assert!(flagmode == false);
					cmds = format!("");
					cmds.push_str(&(m[0]));
					if cmds.contains("|") {
						_splitre = compile_regex("\\|")?;
						_sarr = _splitre.split(cmds.as_str()).collect();
						if _sarr.len() > 2 || _sarr[1].len() != 1 || _sarr[0].len() <= 1 {
							extargs_new_error!{KeyError,"({}) ({})flag only accept (longop|l) format",origkey,flags}
						}
						//extargs_log_trace!("KEYWORD_FLAGNAME [{}]", _sarr[0]);
						self.keydata.set_string(KEYWORD_FLAGNAME,_sarr[0]);
						self.keydata.set_string(KEYWORD_SHORTFLAG,_sarr[1]);
						flagmode= true;
					} else {
						//extargs_log_trace!("KEYWORD_FLAGNAME [{}]", &(m[0]));
						self.keydata.set_string(KEYWORD_CMDNAME,&(m[0]));
						cmdmode = true;
					}
				},
				None => {

				},
			}
		}
		flagmode = flagmode;

		match self.__helpexpr.captures(origkey) {
			Some(m) => {
				if m.len() > 1 {
					self.keydata.set_string(KEYWORD_HELPINFO,&(m[1]));	
				}				
			},
			None => {

			},
		}

		newprefix = String::from("");

		if prefix.len() > 0 {
			newprefix = format!("{}",prefix);
		}

		match self.__prefixexpr.captures(origkey) {
			Some(m) => {
				if m.len() > 1 {
					if newprefix.len() > 0 {
						newprefix.push_str("_");
					}
					newprefix.push_str(&(m[1]));
					self.keydata.set_string(KEYWORD_PREFIX,newprefix.as_str());
				}
			},
			None => {
				if newprefix.len() > 0 {
					self.keydata.set_string(KEYWORD_PREFIX,newprefix.as_str());
				}
			},
		}

		if flagmode {
			self.keydata.set_bool(KEYWORD_ISFLAG,&vtrue);
			self.keydata.set_bool(KEYWORD_ISCMD,&vfalse);
		}

		if cmdmode {
			self.keydata.set_bool(KEYWORD_ISFLAG,&vfalse);
			self.keydata.set_bool(KEYWORD_ISCMD,&vtrue);
		}

		if !flagmode && !cmdmode {
			self.keydata.set_bool(KEYWORD_ISFLAG,&vtrue);
			self.keydata.set_bool(KEYWORD_ISCMD,&vfalse);
		}

		self.keydata.set_jsonval(KEYWORD_VALUE,value);

		if !ishelp && !isjsonfile {
			self.keydata.set_type(KEYWORD_TYPE,TypeClass::new(value).get_type().as_str());
		} else if ishelp {
			self.keydata.set_type(KEYWORD_TYPE,KEYWORD_HELP);
			self.keydata.set_nargs(KEYWORD_NARGS,&(Nargs::Argnum(0)));
		} else if isjsonfile {
			self.keydata.set_type(KEYWORD_TYPE,KEYWORD_JSONFILE);
			self.keydata.set_nargs(KEYWORD_NARGS,&(Nargs::Argnum(1)));
		}

		s = self.keydata.get_type(KEYWORD_TYPE);
		if s == KEYWORD_HELP && !value.is_null() {
			extargs_new_error!{KeyError,"help type must be value None"}
		}

		if cmdmode && s != KEYWORD_DICT {
			flagmode = true;
			//cmdmode = false;
			self.keydata.set_bool(KEYWORD_ISFLAG,&vtrue);
			self.keydata.set_bool(KEYWORD_ISCMD,&vfalse);
			s = self.keydata.get_string_value(KEYWORD_CMDNAME);
			//extargs_log_trace!("KEYWORD_FLAGNAME [{}]", s.as_str());
			self.keydata.set_string(KEYWORD_FLAGNAME,s.as_str());
			self.keydata.set_string(KEYWORD_CMDNAME,KEYWORD_BLANK);
		}


		if self.keydata.get_bool_value(KEYWORD_ISFLAG) && 
		self.keydata.get_type(KEYWORD_TYPE) == KEYWORD_STRING && 
		self.keydata.get_string_value(KEYWORD_FLAGNAME) != KEYWORD_DOLLAR_SIGN {
			match self.keydata.get_jsonval_value(KEYWORD_VALUE) {
				Value::String(v) => {
					if v == "+" {
						let tmpv :Value = serde_json::from_str("0").unwrap();
						self.keydata.set_jsonval(KEYWORD_VALUE,&tmpv);
						self.keydata.set_type(KEYWORD_TYPE,KEYWORD_COUNT);
						self.keydata.set_nargs(KEYWORD_NARGS,&(Nargs::Argnum(0)));
					}
				},
				_ => {

				},
			}
		}

		if self.keydata.get_bool_value(KEYWORD_ISFLAG) && 
		self.keydata.get_string_value(KEYWORD_FLAGNAME) == KEYWORD_DOLLAR_SIGN &&
		self.keydata.get_type(KEYWORD_TYPE) != KEYWORD_DICT {
			let mut nval :Nargs = Nargs::Argnum(0);
			let jval :Value = serde_json::from_str("null").unwrap();
			bmatch = false;
			s = self.keydata.get_type(KEYWORD_TYPE);
			match self.keydata.get_jsonval_value(KEYWORD_VALUE) {
				Value::String(sval) => {
					if sval == KEYWORD_PLUS_SIGN || sval == KEYWORD_QUESTION_SIGN || sval == KEYWORD_STAR_SIGN {
						bmatch = true;
						nval = Nargs::Argtype(sval);
					}
				},
				Value::Number(ref ival) => {
					bmatch = false;
					match ival.as_i64() {
						Some(vv) => {
							nval = Nargs::Argnum( vv as i32 );		
						},
						None => {
							nval = Nargs::Argnum( 0 as i32);
						},
					}						
				},
				_ => {
				},
			}

			if !((s == KEYWORD_STRING && bmatch )|| s == KEYWORD_INT) {
				extargs_new_error!{KeyError,"({})({})({:?}) for $ should option dict set opt or +?* specialcase or type int",prefix,origkey,self.keydata.get_jsonval_value(KEYWORD_VALUE)}
			} else {
				self.keydata.set_nargs(KEYWORD_NARGS,&nval);
				self.keydata.set_jsonval(KEYWORD_VALUE,&jval);
				self.keydata.set_type(KEYWORD_TYPE,KEYWORD_STRING);
			}
		}

		if self.keydata.get_bool_value(KEYWORD_ISFLAG) && 
		self.keydata.get_type(KEYWORD_TYPE) == KEYWORD_DICT && 
		self.keydata.get_string_value(KEYWORD_FLAGNAME).len() > 0 {
			_ = self.__set_flag(prefix,origkey,value)?;
		}

		match self.__attrexpr.captures(origkey) {
			Some(m) => {
				if m.len() > 1 {
					let attr :KeyAttr = KeyAttr::new(&(m[1]))?;
					self.keydata.set_keyattr(KEYWORD_ATTR,&attr);
				}
			},
			None => {

			},
		}

		match self.__funcexpr.captures(origkey) {
			Some(m) => {
				if m.len() > 1 {
					if flagmode {
						self.keydata.set_string(KEYWORD_VARNAME,&(m[1]));
					} else {
						self.keydata.set_string(KEYWORD_FUNCTION,&(m[1]));
					}
				}
			},
			None => {

			},
		}

		self.__validate()
	}


	pub fn new(prefix :&str, key1 :&str,
		value :&Value,isflag :bool,
		ishelp :bool,isjsonfile :bool,
		longprefix :&str,shortprefix :&str,
		nochange :bool) -> Result<InnerExtKeyParse,Box<dyn Error>> {
		let mut key :InnerExtKeyParse;
		key = InnerExtKeyParse {
			keydata : KeyData::new(),
			__helpexpr : compile_regex("##([^#]+)##$")?,
			__cmdexpr : compile_regex("^([^#<>\\+\\$!]+)")?,
			__prefixexpr : compile_regex("\\+([a-zA-Z]+[a-zA-Z0-9]*)")?,
			__funcexpr : compile_regex("<([^<>\\$| \t!\\+]+)>")?,
			__flagexpr : compile_regex("^([a-zA-Z]+[a-zA-Z0-9|\\?\\-]*)")?,
			__mustflagexpr : compile_regex("^\\$([a-zA-Z]+[a-zA-Z0-9_|\\?\\-]*)")?,
			__attrexpr : compile_regex("!([^<>\\$!#|]+)!")?,
		};

		key.__reset();
		key.keydata.set_string(KEYWORD_ORIGKEY,key1);
		key.keydata.set_string(KEYWORD_LONGPREFIX,longprefix);
		key.keydata.set_string(KEYWORD_SHORTPREFIX,shortprefix);
		key.keydata.set_bool(KEYWORD_NOCHANGE,&nochange);
		key.__parse(prefix,key1,value,isflag,ishelp,isjsonfile)?;

		Ok(key)
	}

	pub fn get_string_v(&self,key :&str) -> String {
		if in_array_word(key, FORMWORDS) {
			match self.__form_word_str(key) {
				Ok(v) => {
					return v;
				},
				Err(_e) => {
					//extargs_log_error!("can not get [{}] [{}]",key,e);
					return String::from(KEYWORD_BLANK);
				}
			}
		}  else if key == KEYWORD_TYPE {
			return self.keydata.get_type(key);
		}
		return self.keydata.get_string_value(key);
	}

	pub fn get_bool_v(&self, key :&str) -> bool {
		return self.keydata.get_bool_value(key);
	}

	pub fn get_value_v(&self) -> Value {
		let val :Value = self.keydata.get_jsonval_value(KEYWORD_VALUE);
		return val;
	}

	pub fn get_nargs_v(&self) -> Nargs {
		return self.keydata.get_nargs_value(KEYWORD_NARGS);
	}

	pub fn get_keyattr(&self, key :&str) -> Option<KeyAttr> {
		return self.keydata.get_keyattr(key);
	}

	pub fn is_cmd(&self) -> bool {
		return self.get_bool_v(KEYWORD_ISCMD);
	}

	pub fn cmd_name(&self) -> String {
		return self.get_string_v(KEYWORD_CMDNAME);
	}

	pub fn help_info(&self) -> String {
		return self.get_string_v(KEYWORD_HELPINFO);
	}

	pub fn func_name(&self) -> String {
		return self.get_string_v(KEYWORD_FUNCTION);
	}

	pub fn type_name(&self) -> String {
		return self.get_string_v(KEYWORD_TYPE);
	}

	pub fn opt_dest(&self) -> String {
		return self.get_string_v(KEYWORD_OPTDEST);
	}

	pub fn is_flag(&self) -> bool {
		return self.get_bool_v(KEYWORD_ISFLAG);
	}

	pub	 fn value(&self) -> Value {
		return self.keydata.get_jsonval_value(KEYWORD_VALUE)
	}

	pub fn long_opt(&self) -> String {
		return self.get_string_v(KEYWORD_LONGOPT);
	}

	pub fn long_prefix(&self) -> String {
		return self.get_string_v(KEYWORD_LONGPREFIX);
	}

	pub fn short_opt(&self) -> String {
		return self.get_string_v(KEYWORD_SHORTOPT);
	}

	pub fn var_name(&self) -> String {
		return self.get_string_v(KEYWORD_VARNAME);
	}

	pub fn flag_name(&self) -> String {
		return self.get_string_v(KEYWORD_FLAGNAME);
	}

	pub fn short_flag(&self) -> String {
		return self.get_string_v(KEYWORD_SHORTFLAG);
	}

	pub fn prefix(&self) -> String {
		return self.get_string_v(KEYWORD_PREFIX);
	}

}

impl PartialEq for InnerExtKeyParse {
	fn eq(&self, other :&Self) -> bool {
		if !self.__eq_name__(other,KEYWORD_TYPE) {
			return false;
		}
		if !self.__eq_name__(other,KEYWORD_ORIGKEY) {
			return false;
		}
		if !self.__eq_name__(other,KEYWORD_PREFIX) {
			return false;
		}

		if !self.__eq_name__(other,KEYWORD_VALUE) {
			return false;
		}

		if !self.__eq_name__(other,KEYWORD_ATTR) {
			return false;
		}

		if !self.__eq_name__(other,KEYWORD_LONGPREFIX) {
			return false;
		}

		if !self.__eq_name__(other,KEYWORD_SHORTPREFIX) {
			return false;
		}
		return true;
	}

	fn ne(&self,other :&Self) -> bool {
		return ! self.eq(other);
	}
}

impl Eq for InnerExtKeyParse {}

impl PartialOrd for InnerExtKeyParse {
	fn partial_cmp(&self,other :&Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for InnerExtKeyParse {
	fn cmp(&self, other :&Self) -> Ordering {
		if self.type_name() == KEYWORD_ARGS {
			return Ordering::Less;
		}
		if other.type_name() == KEYWORD_ARGS {
			return Ordering::Greater;
		}

		return self.flag_name().cmp(&(other.flag_name()));
	}
}


#[derive(Clone)]
pub struct ExtKeyParse {
	innerrc :Rc<RefCell<InnerExtKeyParse>>,
}

impl ExtKeyParse {
	pub fn new(prefix :&str, key1 :&str,
		value :&Value,isflag :bool,
		ishelp :bool,isjsonfile :bool,
		longprefix :&str,shortprefix :&str,
		nochange :bool) -> Result<ExtKeyParse,Box<dyn Error>>{
		let k = InnerExtKeyParse::new(prefix,key1,value,isflag,ishelp,isjsonfile,longprefix,shortprefix,nochange)?;
		let b = ExtKeyParse {
			innerrc : Rc::new(RefCell::new(k)),
		};
		Ok(b)
	}

	pub fn is_cmd(&self) -> bool {
		return self.innerrc.borrow().is_cmd();
	}
	pub fn string(&self) -> String {
		return self.innerrc.borrow().string();
	}

	pub fn cmd_name(&self) -> String {
		return self.innerrc.borrow().cmd_name();
	}

	pub fn help_info(&self) -> String {
		return self.innerrc.borrow().help_info();
	}

	pub fn func_name(&self) -> String {
		return self.innerrc.borrow().func_name();
	}

	pub fn get_keyattr(&self,key :&str) -> Option<KeyAttr> {
		return self.innerrc.borrow().get_keyattr(key);
	}

	pub fn type_name(&self) -> String {
		return self.innerrc.borrow().type_name();
	}

	pub fn get_bool_v(&self, k :&str) -> bool {
		return self.innerrc.borrow().get_bool_v(k);
	}

	pub fn opt_dest(&self) -> String {
		return self.innerrc.borrow().opt_dest();
	}

	pub fn get_string_v(&self, k :&str) -> String {
		return self.innerrc.borrow().get_string_v(k);
	}

	pub fn is_flag(&self) -> bool {
		return self.innerrc.borrow().is_flag();
	}

	pub	 fn value(&self) -> Value {
		return self.innerrc.borrow().value();
	}

	pub fn get_value_v(&self) -> Value {
		return self.innerrc.borrow().get_value_v();
	}

	pub fn long_opt(&self) -> String {
		return self.innerrc.borrow().long_opt();
	}

	pub fn long_prefix(&self) -> String {
		return self.innerrc.borrow().long_prefix();	
	}

	pub fn short_opt(&self) -> String {
		return self.innerrc.borrow().short_opt();
	}

	pub fn var_name(&self) -> String {
		return self.innerrc.borrow().var_name();
	}

	pub fn get_nargs_v(&self) -> Nargs {
		return self.innerrc.borrow().get_nargs_v();
	}

	pub fn flag_name(&self) -> String {
		return self.innerrc.borrow().flag_name();
	}

	pub fn short_flag(&self) -> String {
		return self.innerrc.borrow().short_flag();
	}

	pub fn prefix(&self) -> String {
		return self.innerrc.borrow().prefix();
	}
}

impl PartialEq for ExtKeyParse {
	fn eq(&self, other :&Self) -> bool {
		return self.innerrc.borrow().eq(&(other.innerrc.borrow()));
	}

	fn ne(&self,other :&Self) -> bool {
		return ! self.eq(other);
	}
}


impl Eq for ExtKeyParse {}

impl PartialOrd for ExtKeyParse {
	fn partial_cmp(&self,other :&Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for ExtKeyParse {
	fn cmp(&self, other :&Self) -> Ordering {
		return self.innerrc.borrow().cmp(&(other.innerrc.borrow()));
	}
}


#[cfg(test)]
mod debug_key_test_case {
	use super::*;
	use serde_json::{Value};

	fn __opt_fail_check(flags :&ExtKeyParse) {
		assert!(flags.get_string_v(KEYWORD_LONGOPT) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_OPTDEST) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_SHORTOPT) == KEYWORD_BLANK);
		return;
	}

	#[test]
	fn test_a001() {
		let data = "\"string\"";
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","$flag|f+type",&jsonv,false,false,false,"--","-",false).unwrap();

		assert!(flags.flag_name() == "flag");
		assert!(flags.long_opt() == "--type-flag");
		assert!(flags.short_opt() == "-f");
		assert!(flags.opt_dest() == "type_flag");
		assert!(flags.get_value_v() == Value::String(String::from("string")));
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_STRING);
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == "f");
		assert!(flags.get_string_v(KEYWORD_PREFIX) == "type");
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_bool_v(KEYWORD_ISFLAG));
		assert!(!flags.get_bool_v(KEYWORD_ISCMD));
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "type_flag");
		return;
	}

	#[test]
	fn test_a002() {
		let data = "[]";
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","$flag|f+type",&jsonv,true,false,false,"--","-",false).unwrap();

		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == "flag");
		assert!(flags.get_string_v(KEYWORD_LONGOPT) == "--type-flag");
		assert!(flags.get_string_v(KEYWORD_SHORTOPT) == "-f");
		assert!(flags.get_string_v(KEYWORD_OPTDEST) == "type_flag");
		assert!(flags.get_value_v() == jsonv);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_LIST);
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_bool_v(KEYWORD_ISFLAG));
		assert!(!flags.get_bool_v(KEYWORD_ISCMD));
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "type_flag");
		return;
	}

	#[test]
	fn test_a003() {
		let data = "false";
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","$flag|f+type",&jsonv,false,false,false,"--","-",false).unwrap();

		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == "flag");
		assert!(flags.get_string_v(KEYWORD_LONGOPT) == "--type-flag");
		assert!(flags.get_string_v(KEYWORD_SHORTOPT) == "-f");
		assert!(flags.get_string_v(KEYWORD_OPTDEST) == "type_flag");
		assert!(flags.get_value_v() == jsonv);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_BOOL);
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_bool_v(KEYWORD_ISFLAG));
		assert!(!flags.get_bool_v(KEYWORD_ISCMD));
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "type_flag");
		return;
	}

	#[test]
	fn test_a004() {
		let data = "{}";
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("newtype","flag<flag.main>##help for flag##",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == "flag");
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == "flag.main");
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_COMMAND);
		assert!(flags.get_string_v(KEYWORD_PREFIX) == "newtype");
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == "help for flag");
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == KEYWORD_BLANK);
		assert!(flags.get_value_v() == jsonv);
		assert!(!flags.get_bool_v(KEYWORD_ISFLAG));
		assert!(flags.get_bool_v(KEYWORD_ISCMD));
		assert!(flags.get_string_v(KEYWORD_VARNAME) == KEYWORD_BLANK);
		__opt_fail_check(&flags);
		return;
	}	

	#[test]
	fn test_a005() {
		let data = "\"\"";
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","flag<flag.main>##help for flag##",&jsonv,true,false,false,"--","-",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_STRING);
		assert!(flags.get_string_v(KEYWORD_PREFIX) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == "flag");
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == "help for flag");
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == KEYWORD_BLANK);
		assert!(flags.get_value_v() == jsonv);
		assert!(flags.get_bool_v(KEYWORD_ISFLAG));
		assert!(!flags.get_bool_v(KEYWORD_ISCMD));
		assert!(flags.get_string_v(KEYWORD_LONGOPT) == "--flag");
		assert!(flags.get_string_v(KEYWORD_SHORTOPT) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_OPTDEST) == "flag");
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "flag.main");
		return;

	}	

	#[test]
	fn test_a006() {
		let data = r#"{ "new" : false}"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","flag+type<flag.main>##main",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == "flag");
		assert!(flags.get_string_v(KEYWORD_PREFIX) == "type");
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == "flag.main");
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == KEYWORD_BLANK);
		assert!(!flags.get_bool_v(KEYWORD_ISFLAG));
		assert!(flags.get_bool_v(KEYWORD_ISCMD));
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_COMMAND);
		assert!(flags.get_value_v() == jsonv);
		assert!(flags.get_string_v(KEYWORD_VARNAME) == KEYWORD_BLANK);
		__opt_fail_check(&flags);
		return;
	}

	#[test]
	fn test_a007() {
		let data = r#"{}"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","+flag",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_PREFIX) == "flag");
		assert!(flags.get_value_v() == jsonv);
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == KEYWORD_BLANK);
		assert!(flags.get_bool_v(KEYWORD_ISFLAG));
		assert!(!flags.get_bool_v(KEYWORD_ISCMD));
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_PREFIX);
		assert!(flags.get_string_v(KEYWORD_VARNAME) == KEYWORD_BLANK);
		__opt_fail_check(&flags);
		return;
	}

	#[test]
	fn test_a008() {
		let data = r#"null"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let mut ok :i32 = 0;
		match ExtKeyParse::new("","+flag## help ##",&jsonv,false,false,false,"--","-",false) {
			Ok(_v) => {

			},
			Err(_e) => {
				ok = 1;
			},
		}
		assert!(ok > 0);
		return;
	}


	#[test]
	fn test_a009() {
		let data = r#"null"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let mut ok :i32 = 0;
		match ExtKeyParse::new("","+flag<flag.main>",&jsonv,false,false,false,"--","-",false) {
			Ok(_v) => {

			},
			Err(_e) => {
				ok = 1;
			},
		}
		assert!(ok > 0);
		return;
	}

	#[test]
	fn test_a010() {
		let data = r#""""#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let mut ok :i32 = 0;
		match ExtKeyParse::new("","flag|f2",&jsonv,false,false,false,"--","-",false) {
			Ok(_v) => {

			},
			Err(_e) => {
				ok = 1;
			},
		}
		assert!(ok > 0);
		return;
	}

	#[test]
	fn test_a011() {
		let data = r#"null"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let mut ok :i32 = 0;
		match ExtKeyParse::new("","f|f2",&jsonv,false,false,false,"--","-",false) {
			Ok(_v) => {

			},
			Err(_e) => {
				ok = 1;
			},
		}
		assert!(ok > 0);
		return;
	}

	#[test]
	fn test_a012() {
		let data = r#"{}"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","$flag|f<flag.main>",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_PREFIX) == KEYWORD_BLANK);
		assert!(flags.get_value_v() == Value::Null);
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == "f");
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == "flag");
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == KEYWORD_BLANK);
		assert!(flags.get_bool_v(KEYWORD_ISFLAG));
		assert!(!flags.get_bool_v(KEYWORD_ISCMD));
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_STRING);
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "flag.main");
		assert!(flags.get_string_v(KEYWORD_LONGOPT) == "--flag");
		assert!(flags.get_string_v(KEYWORD_SHORTOPT) == "-f");
		assert!(flags.get_string_v(KEYWORD_OPTDEST) == "flag");
		return;
	}


	#[test]
	fn test_a013() {
		let data = r#"null"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","$flag|f+cc<flag.main>",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_PREFIX) == "cc");
		assert!(flags.get_value_v() == jsonv);
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == "f");
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == "flag");
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == KEYWORD_BLANK);
		assert!(flags.get_bool_v(KEYWORD_ISFLAG));
		assert!(!flags.get_bool_v(KEYWORD_ISCMD));
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_STRING);
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "flag.main");
		assert!(flags.get_string_v(KEYWORD_LONGOPT) == "--cc-flag");
		assert!(flags.get_string_v(KEYWORD_SHORTOPT) == "-f");
		assert!(flags.get_string_v(KEYWORD_OPTDEST) == "cc_flag");
		return;
	}

	#[test]
	fn test_a014() {
		let data = r#""""#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let mut ok :i32 = 0;
		match ExtKeyParse::new("","c$",&jsonv,false,false,false,"--","-",false) {
			Ok(_v) => {

			},
			Err(_e) => {
				ok = 1;
			},
		}
		assert!(ok > 0);
		return;
	}

	#[test]
	fn test_a015() {
		let data = r#"null"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let mut ok :i32 = 0;
		match ExtKeyParse::new("","$$",&jsonv,false,false,false,"--","-",false) {
			Ok(_v) => {

			},
			Err(_e) => {
				ok = 1;
			},
		}
		assert!(ok > 0);
		return;
	}

	#[test]
	fn test_a016() {
		let data = r#"{ "nargs" : "+" }"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","$",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == KEYWORD_DOLLAR_SIGN);
		assert!(flags.get_string_v(KEYWORD_PREFIX) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_ARGS);
		assert!(flags.get_string_v(KEYWORD_VARNAME) == KEYWORD_ARGS);
		assert!(flags.get_value_v() == Value::Null);
		assert!(flags.get_nargs_v() == Nargs::Argtype(format!("{}",KEYWORD_PLUS_SIGN)));
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == KEYWORD_BLANK);
		assert!(flags.get_bool_v(KEYWORD_ISFLAG));
		assert!(!flags.get_bool_v(KEYWORD_ISCMD));
		__opt_fail_check(&flags);
		return;
	}

	#[test]
	fn test_a017() {
		let data = r#"3.3"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("type","flag+app## flag help ##",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == "flag");
		assert!(flags.get_string_v(KEYWORD_PREFIX) == "type_app");
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_FLOAT);
		assert!(flags.get_value_v() == jsonv);
		assert!(flags.get_string_v(KEYWORD_LONGOPT) == "--type-app-flag");
		assert!(flags.get_string_v(KEYWORD_SHORTOPT) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_OPTDEST) == "type_app_flag");
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == " flag help ");
		assert!(flags.get_bool_v(KEYWORD_ISFLAG));
		assert!(!flags.get_bool_v(KEYWORD_ISCMD));
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "type_app_flag");
		return;
	}

	#[test]
	fn test_a018() {
		let data = r#"{}"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","flag+app<flag.main>## flag help ##",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_PREFIX) == "app");
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == "flag");
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_VARNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_COMMAND);
		assert!(flags.get_value_v() == jsonv);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == "flag.main");
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == " flag help ");
		assert!(!flags.get_bool_v(KEYWORD_ISFLAG));
		assert!(flags.get_bool_v(KEYWORD_ISCMD));
		__opt_fail_check(&flags);
		return ;
	}

	#[test]
	fn test_a019() {
		let data = r#"{ "prefix" : "good" , "value" : false }"#;
		let falses = r#"false"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let cmpjsonv :Value = serde_json::from_str(falses).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","$flag## flag help ##",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == "flag");
		assert!(flags.get_string_v(KEYWORD_PREFIX) == "good");
		assert!(flags.get_value_v() == cmpjsonv);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_BOOL);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == " flag help ");
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "good_flag");
		assert!(flags.get_nargs_v() == Nargs::Argnum(0));
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_LONGOPT) == "--good-flag");
		assert!(flags.get_string_v(KEYWORD_SHORTOPT) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_OPTDEST) == "good_flag");
		return;
	}

	#[test]
	fn test_a020() {
		let data = r#"null"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let mut ok :i32 = 0;
		match ExtKeyParse::new("","$",&jsonv,false,false,false,"--","-",false) {
			Ok(_v) => {

			},
			Err(_e) => {
				ok = 1;
			},
		}
		assert!(ok > 0);
		return;
	}

	#[test]
	fn test_a021() {
		let data = r#"{ "nargs" : "?" , "value" : null }"#;
		let nulls = r#"null"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let cmpjsonv :Value = serde_json::from_str(nulls).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("command","$## self define ##",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(!flags.get_bool_v(KEYWORD_ISCMD));
		assert!(flags.get_bool_v(KEYWORD_ISFLAG));
		assert!(flags.get_string_v(KEYWORD_PREFIX) == "command");
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "subnargs");
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == KEYWORD_DOLLAR_SIGN);
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == KEYWORD_BLANK);
		assert!(flags.get_value_v() == cmpjsonv);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_ARGS);
		assert!(flags.get_nargs_v() == Nargs::Argtype(String::from(KEYWORD_QUESTION_SIGN)));
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == " self define ");
		__opt_fail_check(&flags);
		return;
	}

	#[test]
	fn test_a022() {
		let data = r#"{}"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("command","+flag",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_PREFIX) == "command_flag");
		assert!(flags.get_value_v() == jsonv);
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_VARNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == KEYWORD_BLANK);
		assert!(flags.get_bool_v(KEYWORD_ISFLAG));
		assert!(!flags.get_bool_v(KEYWORD_ISCMD));
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_PREFIX);
		__opt_fail_check(&flags);
		return;
	}

	#[test]
	fn test_a023() {
		let data = r#"{ "prefix" : "good" , "value" : 3.9, "nargs" : 1 }"#;
		let floats = r#"3.9"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let cmpjsonv :Value = serde_json::from_str(floats).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","$flag## flag help ##",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == "flag");
		assert!(flags.get_string_v(KEYWORD_PREFIX) == "good");
		assert!(flags.get_value_v() == cmpjsonv);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_FLOAT);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == " flag help ");
		assert!(flags.get_nargs_v() == Nargs::Argnum(1));
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_LONGOPT) == "--good-flag");
		assert!(flags.get_string_v(KEYWORD_SHORTOPT) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_OPTDEST) == "good_flag");
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "good_flag");
		return;
	}

	#[test]
	fn test_a024() {
		let data = r#"{ "prefix" : "good" , "value" : false, "nargs" : 2 }"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let mut ok :i32 = 0;
		match ExtKeyParse::new("","$flag## flag help ##",&jsonv,false,false,false,"--","-",false) {
			Ok(_v) => {

			},
			Err(_e) => {
				ok = 1;
			},
		}
		assert!(ok > 0);
		return;
	}


	#[test]
	fn test_a026() {
		let data = r#""+""#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let cmpjsonv :Value = serde_json::from_str("null").unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("dep","$",&jsonv,true,false,false,"--","-",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == KEYWORD_DOLLAR_SIGN);
		assert!(flags.get_string_v(KEYWORD_PREFIX) == "dep");
		assert!(flags.get_value_v() == cmpjsonv);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_ARGS);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == KEYWORD_BLANK);
		assert!(flags.get_nargs_v() == Nargs::Argtype(String::from("+")));
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_VARNAME) == KEYWORD_SUBNARGS);
		__opt_fail_check(&flags);
		return;
	}

	#[test]
	fn test_a027() {
		let data = r#""+""#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let cmpjsonv :Value = serde_json::from_str("0").unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("dep","verbose|v",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == "verbose");
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == "v");
		assert!(flags.get_string_v(KEYWORD_PREFIX) == "dep");
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_COUNT);
		assert!(flags.get_value_v() == cmpjsonv);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == KEYWORD_BLANK);
		assert!(flags.get_nargs_v() == Nargs::Argnum(0));
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_OPTDEST) == "dep_verbose");
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "dep_verbose");
		assert!(flags.get_string_v(KEYWORD_LONGOPT) == "--dep-verbose");
		assert!(flags.get_string_v(KEYWORD_SHORTOPT) == "-v");
		return;
	}

	#[test]
	fn test_a028() {
		let data = r#""+""#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let cmpjsonv :Value = serde_json::from_str("0").unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","verbose|v## new help info ##",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == "verbose");
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == "v");
		assert!(flags.get_string_v(KEYWORD_PREFIX) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_COUNT);
		assert!(flags.get_value_v() == cmpjsonv);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == " new help info ");
		assert!(flags.get_nargs_v() == Nargs::Argnum(0));
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_OPTDEST) == "verbose");
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "verbose");
		assert!(flags.get_string_v(KEYWORD_LONGOPT) == "--verbose");
		assert!(flags.get_string_v(KEYWORD_SHORTOPT) == "-v");
		return;
	}

	#[test]
	fn test_a029() {
		let data = r#"true"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","rollback|R## rollback not set ##",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == "rollback");
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == "R");
		assert!(flags.get_string_v(KEYWORD_PREFIX) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_BOOL);
		assert!(flags.get_value_v() == jsonv);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == " rollback not set ");
		assert!(flags.get_nargs_v() == Nargs::Argnum(0));
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_OPTDEST) == "rollback");
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "rollback");
		assert!(flags.get_string_v(KEYWORD_LONGOPT) == "--no-rollback");
		assert!(flags.get_string_v(KEYWORD_SHORTOPT) == "-R");
		return;
	}

	#[test]
	fn test_a030() {
		let iv :i64 = 0xffffffff;
		let data = format!("{}",iv);
		let jsonv :Value = serde_json::from_str(&(data[..])).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","maxval|m##max value set ##",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == "maxval");
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == "m");
		assert!(flags.get_string_v(KEYWORD_PREFIX) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_INT);
		assert!(flags.get_value_v() == jsonv);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == "max value set ");
		assert!(flags.get_nargs_v() == Nargs::Argnum(1));
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_OPTDEST) == "maxval");
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "maxval");
		assert!(flags.get_string_v(KEYWORD_LONGOPT) == "--maxval");
		assert!(flags.get_string_v(KEYWORD_SHORTOPT) == "-m");
		return;
	}

	#[test]
	fn test_a032() {
		let data = r#""+""#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let cmpjsonv :Value = serde_json::from_str("null").unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","$<numargs>",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == KEYWORD_DOLLAR_SIGN);
		assert!(flags.get_string_v(KEYWORD_PREFIX) == KEYWORD_BLANK);
		assert!(flags.get_value_v() == cmpjsonv);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_ARGS);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == KEYWORD_BLANK);
		assert!(flags.get_nargs_v() == Nargs::Argtype(String::from(KEYWORD_PLUS_SIGN)));
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "numargs");
		__opt_fail_check(&flags);
		return;
	}

	#[test]
	fn test_a033() {
		let data = r#""+""#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let cmpjsonv :Value = serde_json::from_str("null").unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","$",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == KEYWORD_DOLLAR_SIGN);
		assert!(flags.get_string_v(KEYWORD_PREFIX) == KEYWORD_BLANK);
		assert!(flags.get_value_v() == cmpjsonv);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_ARGS);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == KEYWORD_BLANK);
		assert!(flags.get_nargs_v() == Nargs::Argtype(String::from(KEYWORD_PLUS_SIGN)));
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_VARNAME) == KEYWORD_ARGS);
		__opt_fail_check(&flags);
		return;
	}

	#[test]
	fn test_a034() {
		let data = r#""+""#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let cmpjsonv :Value = serde_json::from_str("null").unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("prefix","$",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == KEYWORD_DOLLAR_SIGN);
		assert!(flags.get_string_v(KEYWORD_PREFIX) == "prefix");
		assert!(flags.get_value_v() == cmpjsonv);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_ARGS);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == KEYWORD_BLANK);
		assert!(flags.get_nargs_v() == Nargs::Argtype(String::from(KEYWORD_PLUS_SIGN)));
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_VARNAME) == KEYWORD_SUBNARGS);
		__opt_fail_check(&flags);
		return;
	}

	#[test]
	fn test_a035() {
		let data = r#""+""#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let cmpjsonv :Value = serde_json::from_str("null").unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("prefix","$<newargs>",&jsonv,false,false,false,"--","-",false).unwrap();
		let val :i32;
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == KEYWORD_DOLLAR_SIGN);
		assert!(flags.get_string_v(KEYWORD_PREFIX) == "prefix");
		assert!(flags.get_value_v() == cmpjsonv);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_ARGS);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == KEYWORD_BLANK);
		assert!(flags.get_nargs_v() == Nargs::Argtype(String::from(KEYWORD_PLUS_SIGN)));
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "newargs");
		match flags.get_keyattr(KEYWORD_ATTR) {
			None => {
				val = 1;
			},
			_ => {
				val = 0;
			}
		}
		assert!(val > 0);
		__opt_fail_check(&flags);
		return;
	}

	#[test]
	fn test_a036() {
		let data = r#""+""#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let cmpjsonv :Value = serde_json::from_str("null").unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("prefix","$<newargs>!func=args_opt_func;wait=cc!",&jsonv,false,false,false,"--","-",false).unwrap();
		let val :i32;
		let  mut attr :KeyAttr = KeyAttr::new(KEYWORD_BLANK).unwrap();
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == KEYWORD_DOLLAR_SIGN);
		assert!(flags.get_string_v(KEYWORD_PREFIX) == "prefix");
		assert!(flags.get_value_v() == cmpjsonv);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_ARGS);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == KEYWORD_BLANK);
		assert!(flags.get_nargs_v() == Nargs::Argtype(String::from(KEYWORD_PLUS_SIGN)));
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "newargs");
		match flags.get_keyattr(KEYWORD_ATTR) {
			None => {
				val = 0;
			},
			Some(v) => {
				val = 1;
				attr = v.clone();
			}
		}
		assert!(val > 0);
		assert!(attr.get_attr("func") == "args_opt_func");
		assert!(attr.get_attr("wait") == "cc");
		__opt_fail_check(&flags);
		return;
	}

	#[test]
	fn test_a037() {
		let data = r#"null"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let cmpjsonv :Value = serde_json::from_str("null").unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("prefix","help|h!func=args_opt_func;wait=cc!",&jsonv,false,true,false,"--","-",false).unwrap();
		let val :i32;
		let  mut attr :KeyAttr = KeyAttr::new(KEYWORD_BLANK).unwrap();
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == "help");
		assert!(flags.get_string_v(KEYWORD_PREFIX) == "prefix");
		assert!(flags.get_value_v() == cmpjsonv);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_HELP);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == KEYWORD_BLANK);
		assert!(flags.get_nargs_v() == Nargs::Argnum(0));
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == "h");
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "prefix_help");
		match flags.get_keyattr(KEYWORD_ATTR) {
			None => {
				val = 0;
			},
			Some(v) => {
				val = 1;
				attr = v.clone();
			}
		}
		assert!(val > 0);
		assert!(attr.get_attr("func") == "args_opt_func");
		assert!(attr.get_attr("wait") == "cc");
		assert!(flags.get_string_v(KEYWORD_LONGOPT) == "--help");
		assert!(flags.get_string_v(KEYWORD_SHORTOPT) == "-h");
		assert!(flags.get_string_v(KEYWORD_OPTDEST) == "prefix_help");
		return;
	}

	#[test]
	fn test_a038() {
		let data = r#"null"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let mut flag1 :ExtKeyParse = ExtKeyParse::new("prefix","help|h!func=args_opt_func;wait=cc!",&jsonv,false,true,false,"--","-",false).unwrap();
		let mut flag2 :ExtKeyParse = ExtKeyParse::new("prefix","help|h!func=args_opt_func;wait=cc!",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(flag1 == flag2);
		flag1 = ExtKeyParse::new("prefix","help|h!func=args_opt_func;wait=cc!",&jsonv,false,true,false,"--","-",false).unwrap();
		flag2 = ExtKeyParse::new("prefix","help|h!func=args_opt_func;wait=cc!",&jsonv,false,true,false,"--","-",false).unwrap();
		assert!(flag1 == flag2);
		flag1 = ExtKeyParse::new("prefix","help|h!func=args_opt_func!",&jsonv,false,true,false,"--","-",false).unwrap();
		flag2 = ExtKeyParse::new("prefix","help|h!func=args_opt_func;wait=cc!",&jsonv,false,true,false,"--","-",false).unwrap();
		assert!(flag1 != flag2);
		return;
	}

	#[test]
	fn test_a039() {
		let mut data = r#"{ "modules" : [], "$<NARGS>" : "+" }"#;
		let mut jsonv :Value = serde_json::from_str(data).unwrap();
		let mut flags :ExtKeyParse = ExtKeyParse::new("rdep","ip",&jsonv,false,false,false,"--","-",false).unwrap();

		assert!(flags.get_bool_v(KEYWORD_ISCMD));
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == "ip");
		assert!(flags.get_string_v(KEYWORD_PREFIX) == "rdep");

		data = r#"[]"#;
		jsonv = serde_json::from_str(data).unwrap();
		flags = ExtKeyParse::new("rdep_ip","modules",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(flags.get_bool_v(KEYWORD_ISFLAG));
		assert!(flags.get_value_v() == jsonv);
		assert!(flags.get_string_v(KEYWORD_PREFIX) == "rdep_ip");
		assert!(flags.get_string_v(KEYWORD_LONGOPT) == "--rdep-ip-modules");
		assert!(flags.get_string_v(KEYWORD_SHORTOPT) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_OPTDEST) == "rdep_ip_modules");
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "rdep_ip_modules");
		return;
	}

	#[test]
	fn test_a040() {
		let data = r#"null"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let mut flag1 :ExtKeyParse = ExtKeyParse::new("prefix","json!func=args_opt_func;wait=cc!",&jsonv,false,false,true,"--","-",false).unwrap();
		let mut flag2 :ExtKeyParse = ExtKeyParse::new("prefix","json!func=args_opt_func;wait=cc!",&jsonv,false,false,false,"--","-",false).unwrap();
		assert!(flag1 == flag2);
		flag1 = ExtKeyParse::new("prefix","json!func=args_opt_func;wait=cc!",&jsonv,false,false,true,"--","-",false).unwrap();
		flag2 = ExtKeyParse::new("prefix","json!func=args_opt_func;wait=cc!",&jsonv,false,false,true,"--","-",false).unwrap();
		assert!(flag1 == flag2);
		assert!(flag1.get_string_v(KEYWORD_OPTDEST) == "prefix_json");
		assert!(flag1.get_string_v(KEYWORD_LONGOPT) == "--prefix-json");
		return;
	}

	#[test]
	fn test_a041() {
		let data = r#"{ "nargs" : 1,"attr" : {"func" : "args_opt_func" , "wait" : "cc"} }"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("prefix","$json",&jsonv,false,false,false,"--","-",false).unwrap();
		let  mut attr :KeyAttr = KeyAttr::new(KEYWORD_BLANK).unwrap();
		assert!(flags.get_string_v(KEYWORD_PREFIX) == "prefix");
		assert!(flags.get_bool_v(KEYWORD_ISFLAG));
		match flags.get_keyattr(KEYWORD_ATTR) {
			None => {
			},
			Some(v) => {
				attr = v.clone();
			}
		}
		assert!(attr.get_attr("func") == "args_opt_func");
		assert!(attr.get_attr("wait") == "cc");
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == "json");
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_LONGOPT) == "--prefix-json");
		assert!(flags.get_string_v(KEYWORD_SHORTOPT) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_OPTDEST) == "prefix_json");
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "prefix_json");
		return;
	}

	#[test]
	fn test_a042() {
		let data = r#"{}"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","main",&jsonv,false,false,false,"--","-",false).unwrap();
		let mut ok :i32 = 0;
		assert!(flags.get_string_v(KEYWORD_PREFIX) == "main");
		assert!(!flags.get_bool_v(KEYWORD_ISFLAG));
		assert!(flags.get_bool_v(KEYWORD_ISCMD));
		match flags.get_keyattr(KEYWORD_ATTR) {
			None => {
				ok = 1;
			},
			_ => {

			}
		}
		assert!(ok > 0);
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == "main");
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		__opt_fail_check(&flags);
		return;
	}

	#[test]
	fn test_a043() {
		let data = r#"true"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","rollback|R## rollback not set ##",&jsonv,true,false,false,"++","+",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == "rollback");
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == "R");
		assert!(flags.get_string_v(KEYWORD_PREFIX) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_BOOL);
		assert!(flags.get_value_v() == jsonv);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == " rollback not set ");
		assert!(flags.get_nargs_v() == Nargs::Argnum(0));
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_OPTDEST) == "rollback");
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "rollback");
		assert!(flags.get_string_v(KEYWORD_LONGOPT) == "++no-rollback");
		assert!(flags.get_string_v(KEYWORD_SHORTOPT) == "+R");
		return;
	}

	#[test]
	fn test_a044() {
		let data = r#"true"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","rollback|R## rollback not set ##",&jsonv,true,false,false,"++","+",false).unwrap();
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == "rollback");
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == "R");
		assert!(flags.get_string_v(KEYWORD_PREFIX) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_BOOL);
		assert!(flags.get_value_v() == jsonv);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == " rollback not set ");
		assert!(flags.get_nargs_v() == Nargs::Argnum(0));
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_OPTDEST) == "rollback");
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "rollback");
		assert!(flags.get_string_v(KEYWORD_LONGOPT) == "++no-rollback");
		assert!(flags.get_string_v(KEYWORD_SHORTOPT) == "+R");
		assert!(flags.get_string_v(KEYWORD_LONGPREFIX) == "++");
		assert!(flags.get_string_v(KEYWORD_SHORTPREFIX) == "+");
		return;
	}

	#[test]
	fn test_a045() {
		let data = r#"false"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","crl_CA_compromise",&jsonv,false,false,false,"++","+",true).unwrap();
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == "crl_CA_compromise");
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_PREFIX) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_BOOL);
		assert!(flags.get_value_v() == jsonv);
		assert!(flags.get_string_v(KEYWORD_HELPINFO) == KEYWORD_BLANK);
		assert!(flags.get_nargs_v() == Nargs::Argnum(0));
		assert!(flags.get_string_v(KEYWORD_CMDNAME) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_FUNCTION) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_OPTDEST) == "crl_CA_compromise");
		assert!(flags.get_string_v(KEYWORD_VARNAME) == "crl_CA_compromise");
		assert!(flags.get_string_v(KEYWORD_LONGOPT) == "++crl_CA_compromise");
		assert!(flags.get_string_v(KEYWORD_SHORTOPT) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_LONGPREFIX) == "++");
		assert!(flags.get_string_v(KEYWORD_SHORTPREFIX) == "+");
		return;
	}

	#[test]
	fn test_a046() {
		let data = r#"true"#;
		let jsonv :Value = serde_json::from_str(data).unwrap();
		let flags :ExtKeyParse = ExtKeyParse::new("","panicenable",&jsonv,false,false,false,"--","-",true).unwrap();
		assert!(flags.get_string_v(KEYWORD_FLAGNAME) == "panicenable");
		assert!(flags.get_string_v(KEYWORD_SHORTFLAG) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_PREFIX) == KEYWORD_BLANK);
		assert!(flags.get_string_v(KEYWORD_TYPE) == KEYWORD_BOOL);
		assert!(flags.get_value_v() == jsonv);
		assert!(flags.get_bool_v(KEYWORD_VALUE) == true);
		return;
	}
}
