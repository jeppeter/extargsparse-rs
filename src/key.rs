use serde_json::Value;
use std::collections::HashMap;
use regex::Regex;


#[allow(dead_code)]
pub enum Nargs {	
	Argtype(String),
	Argnum(i32),
}

impl Nargs {
	fn string(&self) -> String {
		let mut retstr:String = String::from("");
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

const KEYWORD_STRING :&str = "string";
const KEYWORD_DICT :&str = "dict";
const KEYWORD_LIST :&str = "list";
const KEYWORD_BOOL :&str = "bool";
const KEYWORD_INT :&str = "int";
const KEYWORD_FLOAT :&str = "float";
const KEYWORD_LONG :&str = "long";
const KEYWORD_ARGS :&str = "args";
const KEYWORD_HELP :&str = "help";
const KEYWORD_JSONFILE :&str = "jsonfile";
const KEYWORD_COUNT :&str = "count";


struct TypeClass {
	typeval : String,
}

#[allow(dead_code)]
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
					_ => {},
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

	pub fn get_keyattr_value(&self,key :&str) -> KeyAttr {
		let ks :String = String::from(key);
		let mut retattr :KeyAttr = KeyAttr::new(KEYWORD_BLANK);

		match self.data.get(&ks) {
			Some(v) => {
				match v {
					KeyVal::KeyAttrVal(kv2) => {
						match kv2 {
							Some(kv3) => {
								retattr = kv3.clone();
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
		return retattr;
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

	fn __form_word_num(&self,key :&str) -> i32 {
		let bval :bool;
		let bopt :Option<bool>;
		let sval :String;
		let sopt :Option<String>;
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

			sopt = self.keydata.get_string(KEYWORD_TYPE);
			match sopt {
				None => {
					return retval;
				},
				Some(v) => {
					sval = v;
				},
			}

			if sval == KEYWORD_INT || sval == KEYWORD_LIST || 
			  sval == KEYWORD_LONG || sval == KEYWORD_FLOAT ||
			  sval == KEYWORD_STRING || sval == KEYWORD_JSONFILE {
			  	retval = 1;
			}
		}
		return retval;
	}

	fn __form_word_str(&self,key :&str) -> String {
		let bval :bool;
		let sval :String;
		let mut retval :String = String::from("");

		if key == KEYWORD_LONGOPT {
			if !self.keydata.get_bool_value(KEYWORD_ISFLAG) ||  
			    self.keydata.get_string_value(KEYWORD_FLAGNAME) == KEYWORD_BLANK ||
			    self.keydata.get_string_value(KEYWORD_TYPE) == KEYWORD_ARGS	{
				panic!("can not set ({}) longopt",self.keydata.get_string_value(KEYWORD_ORIGKEY));
			}
			retval = format!("{}",self.keydata.get_string_value(KEYWORD_LONGPREFIX));
			if self.keydata.get_string_value(KEYWORD_TYPE) == KEYWORD_BOOL {
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
				self.keydata.get_string_value(KEYWORD_TYPE) != KEYWORD_HELP {
				retval.push_str(&(format!("{}_",sval)[..]));
			}
			retval.push_str(&(format!("{}",self.keydata.get_string_value(KEYWORD_FLAGNAME))[..]));
			if !self.keydata.get_bool_value(KEYWORD_NOCHANGE) {
				retval = retval.to_lowercase();
				retval = retval.replace("-","_");
			}
		} else if key == KEYWORD_SHORTOPT {
			if ! self.keydata.get_bool_value(KEYWORD_ISFLAG) || 
			    self.keydata.get_string_value(KEYWORD_FLAGNAME) == KEYWORD_BLANK || 
			    self.keydata.get_string_value(KEYWORD_TYPE)  == KEYWORD_ARGS {
			    panic!("can not set ({}) shortopt",self.keydata.get_string_value(KEYWORD_ORIGKEY));	
			}
			if self.keydata.get_string_value(KEYWORD_SHORTFLAG).len() > 0 {
				retval = format!("{}{}",self.keydata.get_string_value(KEYWORD_SHORTPREFIX),
					self.keydata.get_string_value(KEYWORD_SHORTFLAG));
			}
		} else if key == KEYWORD_OPTDEST {
			if ! self.keydata.get_bool_value(KEYWORD_ISFLAG) || 
			    self.keydata.get_string_value(KEYWORD_FLAGNAME) == KEYWORD_BLANK || 
			    self.keydata.get_string_value(KEYWORD_TYPE)  == KEYWORD_ARGS {
			    panic!("can not set ({}) optdest",self.keydata.get_string_value(KEYWORD_ORIGKEY));	
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
		return retval;
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

	pub fn string(&self) -> String {
		let mut retstr :String;
		let mut s :String;
		retstr = String::from("{");
		retstr.push_str(&(format!("<{}:{}>",KEYWORD_TYPE, self.keydata.get_string_value(KEYWORD_TYPE))[..]));
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
					retstr.push_str(&(format!("{}", v.string())[..]));
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
					retstr.push_str(&(format!("<{}:{:?}>", KEYWORD_VALUE,v)[..]));
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

			},
		}

		return retstr;
	}

	fn __validate(&self) {
		let mut s:String;
		if self.keydata.get_bool_value(KEYWORD_ISFLAG) {
			s = self.keydata.get_string_value(KEYWORD_FUNCTION);
			if s.len() > 0 {
				panic!("({}) can not accept function", self.keydata.get_string_value(KEYWORD_ORIGKEY));
			}

			s = self.keydata.get_string_value(KEYWORD_FLAGNAME);
			if self.keydata.get_string_value(KEYWORD_TYPE) == KEYWORD_DICT && s.len() > 0 {
				panic!("({}) flag can not accept dict",self.keydata.get_string_value(KEYWORD_ORIGKEY));
			}

			s = self.keydata.get_string_value(KEYWORD_TYPE);
			if s != KEYWORD_STRING && s != KEYWORD_INT && s != KEYWORD_FLOAT && 
				s != KEYWORD_LIST && s != KEYWORD_DICT && s != KEYWORD_COUNT && 
				s != KEYWORD_HELP && s != KEYWORD_JSONFILE {
				panic!("({}) value ({:?}) not match type ({})",self.keydata.get_string_value(KEYWORD_ORIGKEY),
						self.keydata.get_jsonval_value(KEYWORD_VALUE),s);
			}
		} else {

		}


	}
}