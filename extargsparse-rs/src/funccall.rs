
use super::key::{ExtKeyParse};
use std::collections::HashMap;

type ExtArgsParseHelpFunc = fn(&ExtKeyParse) -> String;

#[derive(Clone)]
pub enum ExtArgsParseFunc {
	HelpFunc(ExtArgsParseHelpFunc),
}

#[derive(Clone)]
pub struct ExtArgsMatchFuncMap {
	data :HashMap<String,ExtArgsParseFunc>,
}

pub fn new() -> ExtArgsMatchFuncMap {
	ExtArgsMatchFuncMap {
		data : HashMap::new(),
	}
}

impl ExtArgsMatchFuncMap {
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