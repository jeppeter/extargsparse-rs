
use super::logger::{extargs_debug_out};
use super::{extargs_log_error,extargs_log_info,extargs_log_trace};
use std::collections::HashMap;
use serde_json::{Value};

#[derive(Clone)]
pub struct NameSpaceEx {
	values :HashMap<String,Value>,
}

pub (crate) fn new() -> NameSpaceEx {
	NameSpaceEx {
		values : HashMap::new(),
	}
}

impl NameSpaceEx {
	pub fn get_bool(&self, k :&str) -> bool {
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

	pub fn is_accessed(&self,k :&str) -> bool {
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

	pub fn get_string(&self,k :&str) -> String {
		let mut rets :String = "".to_string();

		match self.values.get(k) {
			Some(v) => {
				rets = v.to_string();
			},
			None => {}
		}
		rets
	}

	pub fn get_int(&self,k :&str) -> i64 {
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

	pub fn get_float(&self,k :&str) -> f64 {
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

	pub fn get_array(&self, k :&str) -> Vec<String> {
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

	pub fn get_keys(&self) -> Vec<String> {
		let mut retv :Vec<String> = Vec::new();
		for (k,_) in self.values.clone() {
			retv.push(k.to_string());
		}
		retv
	}

	pub fn string(&self) -> String {
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