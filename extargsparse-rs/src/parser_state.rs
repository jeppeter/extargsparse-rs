

use super::parser_compat::{ParserCompat};
use super::options::{ExtArgsOptions,OPT_LONG_PREFIX,OPT_SHORT_PREFIX,OPT_PARSE_ALL};
use super::key::{ExtKeyParse};
use super::{error_class,new_error,extargs_assert,extargs_log_trace};
use super::logger::{extargs_debug_out};

use std::error::Error;
use std::boxed::Box;
use std::fmt;

error_class!{ParseStateError}

#[derive(Clone)]
pub (crate) struct ParserState {
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

pub (crate) fn new(args :Vec<String>,maincmd :ParserCompat,optattr :ExtArgsOptions) -> ParserState {
	let mut retv :ParserState = ParserState {
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

impl ParserState {
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
			rets.push_str(&(format!("{}",c.cmdname)));
		}
		rets
	}

	fn find_sub_command(&mut self,name :&str) -> Option<ExtKeyParse> {
		let mut retv :Option<ExtKeyParse> = None;
		if self.cmdpaths.len() > 0 {
			let idx = self.cmdpaths.len() - 1;
			let cmdparent = self.cmdpaths[idx].clone();

			for c in cmdparent.subcmds.iter() {
				if c.cmdname == name {
					self.cmdpaths.push(c.clone());
					if c.keycls.is_some() {
						let r = c.keycls.as_ref().unwrap();
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
		let retv :Option<ExtKeyParse> = None;
		let  oldcharidx :i32;
		let oldidx :i32;

		if self.ended > 0 {
			return Ok(retv);
		}

		if self.longargs > 0 {
			extargs_assert!{self.curcharidx < 0 , "curcharidx [{}]", self.curcharidx};
			self.curcharidx += self.longargs ;
			extargs_assert!{self.args.len() as i32 >= self.curidx ,"[{}] < [{}]", self.args.len(), self.curidx};
			self.longargs = -1;
			self.validx = -1;
			self.keyidx = -1;
		}

		oldcharidx = self.curcharidx;
		oldidx = self.curidx;

		extargs_log_trace!("oldcharidx [{}] oldidx[{}]",oldcharidx,oldidx);

		Ok(retv)
	}
}