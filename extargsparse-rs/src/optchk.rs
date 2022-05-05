
use std::rc::Rc;
use std::cell::RefCell;

use super::key::{KEYWORD_LONGOPT,KEYWORD_SHORTOPT,KEYWORD_VARNAME};
use super::util::{check_in_array};

struct InnerOptChk {
	long_opt :Vec<String>,
	short_opt :Vec<String>,
	var_name :Vec<String>,
}

impl InnerOptChk {
	pub (crate) fn new() -> InnerOptChk {
		InnerOptChk {
			long_opt: Vec::new(),
			short_opt : Vec::new(),
			var_name : Vec::new(),
		}
	}

	fn reset(&mut self) {
		self.long_opt = Vec::new();
		self.short_opt = Vec::new();
		self.var_name = Vec::new();
	}

	pub (crate) fn copy(&mut self, other :&InnerOptChk) {
		self.reset();
		for n in other.long_opt.iter() {
			self.long_opt.push(format!("{}",n));
		}
		for n in other.short_opt.iter() {
			self.short_opt.push(format!("{}",n));
		}

		for n in other.var_name.iter() {
			self.var_name.push(format!("{}",n));
		}
		return;
	}

	pub (crate) fn add_and_check(&mut self, typename :String, v :&str) -> bool {
		if typename == KEYWORD_LONGOPT {
			if check_in_array(self.long_opt.clone(),v) {
				return false;
			}
			self.long_opt.push(format!("{}",v));
			return true;
		} else if typename == KEYWORD_SHORTOPT {
			if check_in_array(self.short_opt.clone(),v) {
				return false;
			}
			self.short_opt.push(format!("{}",v));
			return true;
		} else if typename == KEYWORD_VARNAME {
			if check_in_array(self.var_name.clone(),v) {
				return false;
			}
			self.var_name.push(format!("{}",v));
			return true;
		}
		return false;
	}
}

#[allow(dead_code)]
pub (crate) struct OptChk {
	innerrc : Rc<RefCell<InnerOptChk>>,
}

#[allow(dead_code)]
impl  OptChk {
	pub (crate) fn new() -> OptChk {
		OptChk {
			innerrc : Rc::new(RefCell::new(InnerOptChk::new())),
		}
	}

	pub (crate) fn copy(&self,other :&OptChk)  {
		self.innerrc.borrow_mut().copy(&(other.innerrc.borrow()));
		return;
	}

	pub (crate) fn add_and_check(&self, k :String, v :&str) -> bool {
		return self.innerrc.borrow_mut().add_and_check(k,v);
	}
}