
//use super::logger::{extargs_debug_out};
//use super::{extargs_log_error,extargs_log_info,extargs_log_trace};
use std::collections::HashMap;
use serde_json::{Value};
use std::rc::Rc;
use std::cell::RefCell;

use std::error::Error;
use std::boxed::Box;

use super::{extargs_error_class,extargs_new_error};

extargs_error_class!{NameSpaceError}

#[derive(Clone)]
struct InnerNameSpaceEx {
	values :HashMap<String,Value>,
}


impl InnerNameSpaceEx {
	pub (crate) fn new() -> InnerNameSpaceEx {
		InnerNameSpaceEx {
			values : HashMap::new(),
		}
	}
	pub (crate) fn get_bool(&self, k :&str) -> bool {
		let mut retb :bool = false;
		match self.values.get(k) {
			Some(v1) => {
				match v1 {
					Value::Bool(v) => {
						retb = *v;
					},
					_ => {}
				}
			},
			None => {}
		}
		retb
	}

	pub (crate) fn is_accessed(&self,k :&str) -> bool {
		let mut retb :bool = false;
		match self.values.get(k) {
			Some(_v1) => {
				retb = true;
			},
			None => {

			}
		}
		retb
	}

	pub (crate) fn set_value( & mut self,k :&str, v :Value) {
		self.values.insert(k.to_string(),v);
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
						rets = "null".to_string();
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
			None => {}
		}
		rets
	}

	pub (crate) fn get_int(&self,k :&str) -> i64 {
		let mut reti :i64 = 0;
		match self.values.get(k) {
			Some(v1) => {
				match v1 {
					Value::Number(n) => {
						if n.is_i64()  {
							match n.as_i64() {
								Some(ic) => {
									reti = ic as i64;
								},
								_ => {}
							}
						} else if n.is_u64() {
							match n.as_u64() {
								Some(ic) => {
									reti = ic as i64;
								},
								_ => {}
							}
						} else if n.is_f64() {
							match n.as_f64() {
								Some(ic) => {
									reti = ic as i64;
								},
								_ => {}
							}
						}
					},
					_ => {}
				}
			},
			None => {

			}
		}
		reti
	}

	pub (crate) fn get_float(&self,k :&str) -> f64 {
		let mut retf :f64 = 0.0;
		match self.values.get(k) {
			Some(v1) => {
				match v1 {
					Value::Number(n) => {
						if n.is_f64()  {
							match n.as_f64() {
								Some(ic) => {
									retf = ic as f64;
								},
								_ => {}
							}
						} else if n.is_u64() {
							match n.as_u64() {
								Some(ic) => {
									retf = ic as f64;
								},
								_ => {}
							}
						} else if n.is_i64() {
							match n.as_i64() {
								Some(ic) => {
									retf = ic as f64;
								},
								_ => {}
							}
						}
					},
					_ => {}
				}
			},
			None => {

			}
		}
		retf
	}

	pub (crate) fn get_array(&self, k :&str) -> Vec<String> {
		let mut retv :Vec<String> = Vec::new();
		let mut errorval :i32 = 0;
		match self.values.get(k) {
			Some(v2) => {
				match v2 {
					Value::Array(v1) => {
						for curv in v1 {
							match curv {
								Value::String(v) => {
									retv.push(v.to_string());
								},
								_ => {
									errorval = 1;
								}
							}
						}
					},
					_ => {

					}
				}
				if errorval > 0 {
					/*has something no in string ,so just clear*/
					retv = Vec::new();
				}				
			},
			_ => {}
		}
		retv
	}

	pub (crate) fn get_keys(&self) -> Vec<String> {
		let mut retv :Vec<String> = Vec::new();
		for (k,_) in self.values.clone() {
			retv.push(k.to_string());
		}
		retv
	}

	pub (crate) fn string(&self) -> String {
		let mut rets :String = "".to_string();
		let mut i :i32 = 0;

		for (k,v) in self.values.clone() {
			if i>0 {
				rets.push_str(";");
			}
			rets.push_str(&format!("{}={:?}",k,v));
			i += 1;
		}
		rets
	}
}

#[derive(Clone)]
pub struct NameSpaceEx {
	innerrc : Rc<RefCell<InnerNameSpaceEx>>,
}

impl NameSpaceEx {
	pub (crate) fn new() -> NameSpaceEx {
		NameSpaceEx {
			innerrc : Rc::new(RefCell::new(InnerNameSpaceEx::new())),
		}
	}

	pub fn get_array(&self, k :&str) -> Vec<String> {
		return self.innerrc.borrow().get_array(k);
	}

	pub fn get_int(&self,k :&str) -> i64 {
		return self.innerrc.borrow().get_int(k);
	}

	pub fn get_string(&self, k :&str) -> String {
		return self.innerrc.borrow().get_string(k);
	}

	pub fn get_bool(&self, k :&str) -> bool {
		return self.innerrc.borrow().get_bool(k);
	}

	pub fn get_float(&self, k :&str) -> f64 {
		return self.innerrc.borrow().get_float(k);
	}

	pub fn get_keys(&self) -> Vec<String> {
		return self.innerrc.borrow().get_keys();
	}

	pub fn string(&self) -> String {
		return self.innerrc.borrow().string();
	}
	
	pub (crate) fn set_string(&self,k :&str, v :String) -> Result<(),Box<dyn Error>> {
		let ns :String = format!("\"{}\"", v);
		/*for parse will not make this ok*/
		let s :String = ns.replace("\\","\\\\");
		match serde_json::from_str(&s) {
			Ok(v) => {
				self.innerrc.borrow_mut().set_value(k,v);
			},
			Err(e) => {
				extargs_new_error!{NameSpaceError,"can not parse [{}] error[{:?}]", s,e}
			}
		}
		Ok(())
	}

	pub (crate) fn set_bool(&self,k :String, b :bool) -> Result<(),Box<dyn Error>> {
		let setv :Value;
		if b {
			setv = serde_json::from_str("true").unwrap();
		} else {
			setv = serde_json::from_str("false").unwrap();
		}
		self.innerrc.borrow_mut().set_value(&k,setv);
		Ok(())
	}

	pub (crate) fn set_int(&self,k :&str, v :i64) -> Result<(),Box<dyn Error>> {
		let s :String = format!("{}", v);
		match serde_json::from_str(&s) {
			Ok(iv) => {
				self.innerrc.borrow_mut().set_value(k,iv);
			},
			Err(e) => {
				extargs_new_error!{NameSpaceError,"can not parse [{}] error[{:?}]",s,e}
			}
		}
		Ok(())
	}

	pub (crate) fn set_array(&self, k :&str, narr :Vec<String>) -> Result<(),Box<dyn Error>> {
		let mut s :String = "[".to_string();
		let mut idx :i32 = 0;
		for c in narr {
			if idx > 0 {
				s.push_str(",");
			}
			s.push_str(&(format!("\"{}\"",c.replace("\\","\\\\"))));
			idx += 1;
		}
		s.push_str("]");
		match serde_json::from_str(&s) {
			Ok(v) => {
				self.innerrc.borrow_mut().set_value(k,v);
			},
			Err(e) => {
				extargs_new_error!{NameSpaceError,"can not parse [{}] error[{:?}]", s,e}
			}
		}
		Ok(())
	}

	pub (crate) fn set_float(&self, k :&str, fv :f64) -> Result<(),Box<dyn Error>> {
		let s :String = format!("{}",fv);
		match serde_json::from_str(&s) {
			Ok(v) => {
				self.innerrc.borrow_mut().set_value(k,v);
			},
			Err(e) => {
				extargs_new_error!{NameSpaceError,"can not parse [{}] error[{:?}]", s, e}
			}
		}
		Ok(())
	}

	pub (crate) fn is_accessed(&self,k :&str) -> bool {
		return self.innerrc.borrow().is_accessed(k);
	}

	pub (crate) fn set_value(&self,k :&str,v :Value)  {
		return self.innerrc.borrow_mut().set_value(k,v);
	}
}