use serde_json::Value;
use std::collections::HashMap;
use regex::Regex;


#[allow(dead_code)]
pub enum Nargs {	
	Argtype(String),
	Argnum(i32),
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

	fn get_keys(&self) -> Vec<String> {
		let mut retvec :Vec<String> = Vec::new();
		for (k,_) in &(self.__obj) {
			retvec.push(String::from(k));
		}
		return retvec;
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


#[allow(dead_code)]
pub struct Key {
	keydata : KeyData,
}

impl Key {
	fn __reset(&mut self) {
		self.keydata = KeyData::new();
		return;
	}

	fn __eq_name__(&self,other :&Key,name :&str) -> bool {
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
			name == KEYWORD_TYPE || name == KEYWORD_LONGPREFIX ||
			name == KEYWORD_SHORTPREFIX || name == KEYWORD_LONGOPT || 
			name == KEYWORD_SHORTOPT {
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

		} else if name == KEYWORD_NARGS {
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
}