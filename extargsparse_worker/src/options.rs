use std::collections::HashMap;
use serde_json::{Value};
use lazy_static::lazy_static;
use super::logger::{extargs_debug_out,extargs_log_get_timestamp};
//use super::{extargs_log_error,extargs_log_info,extargs_log_trace};
use super::{extargs_log_error,extargs_log_info};

use std::fmt::{Debug};


use std::error::Error;
use std::boxed::Box;
use std::rc::Rc;
use std::cell::RefCell;


use super::{extargs_error_class,extargs_new_error};

extargs_error_class!{ExtArgsOptionParseError}



pub const OPT_PROG :&str = "prog";
pub const OPT_USAGE :&str = "usage";
pub const OPT_DESCRIPTION :&str= "description";
pub const OPT_EPILOG :&str = "epilog";
pub const OPT_VERSION :&str = "version";
pub const OPT_ERROR_HANDLER :&str = "errorhandler";
pub const OPT_HELP_HANDLER :&str = "helphandler";
pub const OPT_LONG_PREFIX :&str = "longprefix";
pub const OPT_SHORT_PREFIX :&str = "shortprefix";
pub const OPT_NO_HELP_OPTION :&str = "nohelpoption";
pub const OPT_NO_JSON_OPTION :&str = "nojsonoption";
pub const OPT_HELP_LONG :&str = "helplong";
pub const OPT_HELP_SHORT :&str = "helpshort";
pub const OPT_JSON_LONG :&str = "jsonlong";
pub const OPT_CMD_PREFIX_ADDED :&str = "cmdprefixadded";
pub const OPT_PARSE_ALL :&str = "parseall";
pub const OPT_SCREEN_WIDTH :&str = "screenwidth";
pub const OPT_FLAG_NO_CHANGE :&str = "flagnochange";
pub const OPT_VAR_UPPER_CASE :&str = "varuppercase";
pub const OPT_FUNC_UPPER_CASE :&str = "funcuppercase";


macro_rules! OPT_DEFAULT_S {
	() => {
		format!(r#"
			{{
				"{}" : "",
				"{}" : "",
				"{}" : "",
				"{}" : "",
				"{}" : "0.0.1",
				"{}" : "exit",
				"{}" : null,
				"{}" : "--",
				"{}" : "-",
				"{}" : false,
				"{}" : false,
				"{}" : "help",
				"{}" : "h",
				"{}" : "json",
				"{}" : true ,
				"{}" : true,
				"{}" : 80,
				"{}" : false,
				"{}" : true,
				"{}" : true
			}}
			"#,OPT_PROG,OPT_USAGE,OPT_DESCRIPTION,OPT_EPILOG,OPT_VERSION,
			OPT_ERROR_HANDLER,OPT_HELP_HANDLER,OPT_LONG_PREFIX,OPT_SHORT_PREFIX, OPT_NO_HELP_OPTION,
			OPT_NO_JSON_OPTION, OPT_HELP_LONG,OPT_HELP_SHORT, OPT_JSON_LONG,OPT_CMD_PREFIX_ADDED,
			OPT_PARSE_ALL, OPT_SCREEN_WIDTH,OPT_FLAG_NO_CHANGE, OPT_VAR_UPPER_CASE,OPT_FUNC_UPPER_CASE)
	}
}

lazy_static! {
	static ref OPT_KEYS :Vec<String>= {
		let mut retv :Vec<String>= Vec::new();
		retv.push(OPT_PROG.to_string());
		retv.push(OPT_USAGE.to_string());
		retv.push(OPT_DESCRIPTION.to_string());
		retv.push(OPT_EPILOG.to_string());
		retv.push(OPT_VERSION.to_string());
		retv.push(OPT_ERROR_HANDLER.to_string());
		retv.push(OPT_HELP_HANDLER.to_string());
		retv.push(OPT_LONG_PREFIX.to_string());
		retv.push(OPT_SHORT_PREFIX.to_string());
		retv.push(OPT_NO_HELP_OPTION.to_string());
		retv.push(OPT_NO_JSON_OPTION.to_string());
		retv.push(OPT_HELP_LONG.to_string());
		retv.push(OPT_HELP_SHORT.to_string());
		retv.push(OPT_JSON_LONG.to_string());
		retv.push(OPT_CMD_PREFIX_ADDED.to_string());
		retv.push(OPT_PARSE_ALL.to_string());
		retv.push(OPT_SCREEN_WIDTH.to_string());
		retv.push(OPT_FLAG_NO_CHANGE.to_string());
		retv.push(OPT_VAR_UPPER_CASE.to_string());
		retv.push(OPT_FUNC_UPPER_CASE.to_string());
		retv
	};
	static ref OPT_DEFAULT_VALUE :HashMap<String,Value> = {
		let defs = OPT_DEFAULT_S!();
		let tmpv :HashMap<String,Value>;
		extargs_log_info!("parse opt default\n{}",defs);
		match serde_json::from_str(&defs) {
			Ok(d) => {
				tmpv = d;
			},
			Err(e) => {
				extargs_log_error!("can not parse error[{}]\n{}",e,defs);
				panic!("exit");
			}
		}
		tmpv	
	};
}

#[derive(Clone)]
struct InnerExtArgsOptions {
	values :HashMap<String,Value>,
}


impl InnerExtArgsOptions {
	pub (crate) fn new(s :&str) -> Result<InnerExtArgsOptions,Box<dyn Error>> {
		let mut retv :InnerExtArgsOptions = InnerExtArgsOptions {
			values :HashMap::new(),
		};

		for (k,v) in OPT_DEFAULT_VALUE.clone() {
			//extargs_log_trace!("[{}]=[{:?}]", k,v);
			retv.values.insert(k,v);
		}
		let err = serde_json::from_str(s);
		if err.is_err() {
			extargs_new_error!{ExtArgsOptionParseError,"parse error[{:?}]\n{}", err, s}
		}

		let f :Value = err.unwrap();
		if !f.is_object() {
			extargs_new_error!{ExtArgsOptionParseError,"{} not object", s}
		}
		for (k,v) in f.as_object().unwrap() {
			retv.values.insert(k.to_string(),v.clone());
		}


		Ok(retv)
	}


	pub (crate) fn set_value( & mut self,k :&str, v :Value) {
		self.values.insert(k.to_string(),v);
	}


	pub (crate) fn string(&self) -> String {
		let mut rets :String;
		let mut idx :i32 = 0;
		rets = "".to_string();
		for (k,v) in self.values.clone() {
			if idx > 0 {
				rets.push_str(&format!(","));
			}
			rets.push_str(&format!("[{}]=[{:?}]",k,v));
			idx += 1;
		}
		rets
	}
	pub (crate) fn get_value(&self, k :&str) -> Option<Value> {
		match self.values.get(k) {
			Some(v) => {
				return Some(v.clone());
			},
			None => {
				return None;
			}
		}
	}

	pub (crate) fn get_string(&self,k :&str) -> String {
		let mut rets :String = "".to_string();

		match self.values.get(k) {
			Some(v) => {
				match v {
					Value::String(ref _v)	=> {
						rets = format!("{}",_v);	
					},
					Value::Bool(ref _b) => {
						if *_b {
							rets = format!("true");
						} else {
							rets = format!("false");
						}
					},
					Value::Null => {
						rets = "".to_string();
					},
					Value::Number(ref _n) => {
						if _n.is_i64()  {
							rets = format!("{}",_n.as_i64().unwrap());
						} else if _n.is_u64() {
							rets = format!("{}",_n.as_u64().unwrap());
						} else if _n.is_f64() {
							rets = format!("{}",_n.as_f64().unwrap());
						}
					},
					Value::Array(ref _a) => {
						rets = format!("{:?}", _a);
					},
					Value::Object(ref _o) => {
						rets = format!("{:?}", _o);
					}
				}
				
			},
			None => {
				
			}
		}
		rets
	}

	pub (crate) fn get_int(&self,k :&str) -> i32 {
		let mut reti :i32 = 0;
		match self.values.get(k) {
			Some(v1) => {
				match v1 {
					Value::Number(n) => {
						if n.is_i64()  {
							match n.as_i64() {
								Some(ic) => {
									reti = ic as i32;
								},
								_ => {

								}
							}
						} else if n.is_u64() {
							match n.as_u64() {
								Some(ic) => {
									reti = ic as i32;
								},
								_ => {

								}
							}
						}
					},
					_ => {

					}
				}
			},
			None => {

			}
		}
		reti
	}

	pub (crate) fn get_bool(&self,k :&str) -> bool {
		let mut retb :bool = false;
		match self.values.get(k) {
			Some(v1) => {
				match v1 {
					Value::Bool(v) => {
						retb = *v;
					},
					_ => {

					}
				}
			},
			None => {

			}
		}
		retb
	}
}

#[derive(Clone)]
pub struct ExtArgsOptions {
	innerrc :Rc<RefCell<InnerExtArgsOptions>>,
}

impl ExtArgsOptions {
	pub fn new(s :&str) -> Result<ExtArgsOptions,Box<dyn Error>> {
		let k :InnerExtArgsOptions = InnerExtArgsOptions::new(s)?;
		Ok(ExtArgsOptions {
			innerrc : Rc::new(RefCell::new(k)),
		})
	}

	pub (crate) fn set_string(&mut self, k :&str, v :&str) -> Result<(),Box<dyn Error>> {
		let ns :String = format!("\"{}\"", v);
		/*for parse will not make this ok*/
		let s :String = ns.replace("\\","\\\\");
		match serde_json::from_str(&s) {
			Ok(v) => {
				self.innerrc.borrow_mut().set_value(k,v);
			},
			Err(e) => {
				extargs_new_error!{ExtArgsOptionParseError,"can not parse [{}] error[{:?}]", s,e}
			}
		}
		Ok(())		
	}

	pub fn get_value(&self, k :&str) -> Option<Value> {
		return self.innerrc.borrow().get_value(k);
	}

	pub fn get_int(&self,k :&str) -> i32 {
		return self.innerrc.borrow().get_int(k);
	}

	pub fn get_string(&self,k :&str) -> String {
		return self.innerrc.borrow().get_string(k);
	}

	pub fn get_bool(&self,k :&str) -> bool {
		return self.innerrc.borrow().get_bool(k);
	}

	pub fn string(&self) -> String {
		return self.innerrc.borrow().string();
	}
}