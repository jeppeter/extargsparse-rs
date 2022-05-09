
use super::key::{ExtKeyParse};
use super::namespace::{NameSpaceEx};
use super::argset::{ArgSetImpl};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use serde_json::Value;
use std::any::Any;


use std::error::Error;
use std::boxed::Box;




pub type ExtArgsParseHelpFunc = fn(&ExtKeyParse) -> String;
pub type ExtArgsJsonFunc = fn(NameSpaceEx,ExtKeyParse,Value)  -> Result<(),Box<dyn Error>> ;
pub type ExtArgsActionFunc = fn(NameSpaceEx,i32,ExtKeyParse,Vec<String>) -> Result<i32,Box<dyn Error>>;
pub type ExtArgsCallbackFunc = fn(NameSpaceEx,Option<Arc<RefCell<dyn ArgSetImpl>>>,Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>>;

#[derive(Clone)]
pub enum ExtArgsParseFunc {
	HelpFunc(ExtArgsParseHelpFunc),
	JsonFunc(ExtArgsJsonFunc),
	ActionFunc(ExtArgsActionFunc),
	CallbackFunc(ExtArgsCallbackFunc),
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
					},
					_ => {}
				}
			},
			_ => {}
		}
		retv
	}

	pub (crate) fn get_json_func(&self,k :&str) -> Option<ExtArgsJsonFunc> {
		let mut retv :Option<ExtArgsJsonFunc> = None;
		match self.data.get(k) {
			Some(v1) => {
				match v1 {
					ExtArgsParseFunc::JsonFunc(f1) => {
						retv = Some(*f1);
					},
					_ => {}
				}
			},
			_ => {}
		}
		retv
	}

	pub (crate) fn get_action_func(&self,k :&str) -> Option<ExtArgsActionFunc> {
		let mut retv :Option<ExtArgsActionFunc> = None;
		match self.data.get(k) {
			Some(v1) => {
				match v1 {
					ExtArgsParseFunc::ActionFunc(f1) => {
						retv = Some(*f1);
					},
					_ => {}
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

	pub (crate) fn get_json_func(&self, k :&str) -> Option<ExtArgsJsonFunc> {
		return self.innerrc.borrow().get_json_func(k);
	}
	pub (crate) fn get_action_func(&self, k :&str) -> Option<ExtArgsActionFunc> {
		return self.innerrc.borrow().get_action_func(k);
	}

}
