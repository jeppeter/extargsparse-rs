
use super::key::{ExtKeyParse};
use std::collections::HashMap;

type ExtArgsParseHelpFunc = fn (&ExtKeyParse) -> String;


pub enum ExtArgsParseFunc {
	HelpFunc(ExtArgsParseHelpFunc),
}

pub struct ExtArgsMatchFuncMap {
	data :HashMap<String,ExtArgsParseFunc>,
}