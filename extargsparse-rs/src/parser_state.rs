

use super::parser_compat::{ParserCompat};
use super::options::{ExtArgsOptions,OPT_LONG_PREFIX,OPT_SHORT_PREFIX,OPT_PARSE_ALL};
use super::key::{ExtKeyParse};
use super::{error_class,new_error};
//use super::logger::{extargs_debug_out};

use std::error::Error;
use std::boxed::Box;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

error_class!{ParseStateError}

#[allow(dead_code)]
pub (crate) enum StateOptVal {
	LeftArgs(Vec<String>),
	OptDest(String),
	CmdPaths(String),
}

#[allow(dead_code)]
#[derive(Clone)]
struct InnerParserState {
	cmdpaths :Vec<ParserCompat>,
	curidx : i32,
	curcharidx :i32,
	shortcharargs :i32,
	longargs :i32,
	keyidx :i32,
	validx :i32,
	args :Vec<String>,
	ended : i32,
	longprefix :String,
	shortprefix :String,
	bundlemode :bool,
	parseall :bool,
	leftargs :Vec<String>,
}

#[allow(dead_code)]
impl InnerParserState {
	pub (crate) fn new(args :Vec<String>,maincmd :ParserCompat,optattr :ExtArgsOptions) -> InnerParserState {
		let mut retv :InnerParserState = InnerParserState {
			cmdpaths : Vec::new(),
			curidx : 0,
			curcharidx : -1,
			shortcharargs : -1,
			longargs : -1,
			keyidx : -1,
			validx : -1,
			args : Vec::new(),
			ended : 0,
			longprefix : "--".to_string(),
			shortprefix : "-".to_string(),
			bundlemode : false,
			parseall : true,
			leftargs : Vec::new(),
		};

		retv.cmdpaths.push(maincmd.clone());
		retv.longprefix = optattr.get_string(OPT_LONG_PREFIX);
		retv.shortprefix = optattr.get_string(OPT_SHORT_PREFIX);

		if retv.longprefix.len() == 0 || 
		retv.shortprefix.len() == 0 || 
		retv.longprefix == retv.shortprefix {
			retv.bundlemode = true;
		} else {
			retv.bundlemode = false;
		}
		retv.parseall = optattr.get_bool(OPT_PARSE_ALL);
		retv.args = args.clone();
		retv
	}

	pub (crate) fn format_cmd_name_path(&self,cmdpaths :Vec<ParserCompat>) -> String {
		let mut rets :String = "".to_string();
		let mut curparser :Vec<ParserCompat> = cmdpaths.clone();
		if curparser.len() > 0 {
			curparser = self.cmdpaths.clone()
		}
		for c in curparser.iter() {
			if rets.len() > 0 {
				rets.push_str(".");
			}
			rets.push_str(&(format!("{}",c.cmd_name())));
		}
		rets
	}

	fn find_sub_command(&mut self,name :&str) -> Option<ExtKeyParse> {
		let mut retv :Option<ExtKeyParse> = None;
		if self.cmdpaths.len() > 0 {
			let idx = self.cmdpaths.len() - 1;
			let cmdparent = self.cmdpaths[idx].clone();

			for c in cmdparent.sub_cmds().iter() {
				if c.cmd_name() == name {
					self.cmdpaths.push(c.clone());
					if c.get_keycls().is_some() {
						let r = c.get_keycls().unwrap();
						retv = Some(r.clone());	
					}					
					break;
				}
			}
		}
		retv
	}

	pub (crate) fn add_parse_args(&mut self, nargs :i32) -> Result<(),Box<dyn Error>> {
		if self.curcharidx >= 0 {
			if nargs > 0 && self.shortcharargs > 0 {
				new_error!{ParseStateError,"[{}] already set args",self.args[self.curidx as usize]}
			}

			if self.shortcharargs < 0 {
				self.shortcharargs = 0;
			}
			self.shortcharargs += nargs;
		} else {
			if self.longargs > 0 {
				new_error!{ParseStateError,"[{}] not handled", self.args[self.curidx as usize]}
			}
			if self.longargs < 0 {
				self.longargs = 0;
			}
			self.longargs += nargs;
		}
		Ok(())
	}

	fn find_key_cls(&mut self) -> Result<Option<ExtKeyParse>,Box<dyn Error>> {
		return Ok(None);
	}

	pub (crate) fn step_one(&mut self) -> Result<(i32,Option<StateOptVal>,Option<ExtKeyParse>),Box<dyn Error>> {
		let validx :i32;
		let mut optval :Option<StateOptVal> = None;
		let keycls : Option<ExtKeyParse>;
		if self.ended > 0 {
			validx = self.curidx;
			optval = Some(StateOptVal::LeftArgs(self.leftargs.clone()));
			keycls = None;
			return Ok((validx,optval,keycls));
		}

		keycls = self.find_key_cls()?;
		if keycls.is_none() {
			validx = self.curidx;
			optval = Some(StateOptVal::LeftArgs(self.leftargs.clone()));
			return Ok((validx,optval,keycls));
		}

		let kopt = keycls.as_ref().unwrap().clone();
		if !kopt.is_cmd() {
			optval = Some(StateOptVal::OptDest(format!("{}",kopt.opt_dest())));
		} else if kopt.is_cmd() {
			let cmdpaths = self.cmdpaths.clone();
			let r = self.format_cmd_name_path(cmdpaths);
			optval = Some(StateOptVal::CmdPaths(r));
		}
		validx = self.curidx;
		return Ok((validx,optval,keycls));
	}

	pub (crate) fn get_cmd_paths(&self) -> Vec<ParserCompat> {
		return self.cmdpaths.clone();
	}
}


#[allow(dead_code)]
#[derive(Clone)]
pub struct ParserState {
	innerrc : Rc<RefCell<InnerParserState>>,
}

#[allow(dead_code)]
impl ParserState {
	pub (crate) fn new(args :Vec<String>,maincmd :ParserCompat,optattr :ExtArgsOptions) -> ParserState {
		ParserState {
			innerrc : Rc::new(RefCell::new(InnerParserState::new(args,maincmd,optattr))),
		}
	}
}