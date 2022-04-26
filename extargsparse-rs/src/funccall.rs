
use super::key::{ExtKeyParse};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;


pub type ExtArgsParseHelpFunc = fn(&ExtKeyParse) -> String;

#[derive(Clone)]
pub enum ExtArgsParseFunc {
	HelpFunc(ExtArgsParseHelpFunc),
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct InnerExtArgsMatchFuncMap {
	data :HashMap<String,ExtArgsParseFunc>,
}


#[allow(dead_code)]
impl InnerExtArgsMatchFuncMap {
	pub fn new() -> InnerExtArgsMatchFuncMap {
		InnerExtArgsMatchFuncMap {
			data : HashMap::new(),
		}
	}

	pub (crate) fn get_help_func(&self,k :&str) -> Option<ExtArgsParseHelpFunc> {
		let mut retv :Option<ExtArgsParseHelpFunc> = None;
		match self.data.get(k) {
			Some(v1) => {
				match v1 {
					ExtArgsParseFunc::HelpFunc(f1) => {
						retv = Some(*f1);
					}
				}
			},
			_ => {}
		}
		retv
	}
}

#[derive(Clone)]
pub struct ExtArgsMatchFuncMap {
	innerrc : Rc<RefCell<InnerExtArgsMatchFuncMap>>,
}

impl ExtArgsMatchFuncMap {
	pub fn new() -> ExtArgsMatchFuncMap {
		ExtArgsMatchFuncMap {
			innerrc : Rc::new(RefCell::new(InnerExtArgsMatchFuncMap::new())),
		}
	}

	pub (crate) fn get_help_func(&self,k :&str) -> Option<ExtArgsParseHelpFunc> {
		return self.innerrc.borrow().get_help_func(k);
	}
}
