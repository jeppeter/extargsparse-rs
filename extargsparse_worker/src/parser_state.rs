

use super::parser_compat::{ParserCompat};
use super::options::{ExtArgsOptions,OPT_LONG_PREFIX,OPT_SHORT_PREFIX,OPT_PARSE_ALL};
use super::key::{ExtKeyParse,KEYWORD_DOLLAR_SIGN};
use super::{extargs_error_class,extargs_new_error,extargs_assert,extargs_log_info,extargs_log_trace};
use super::logger::{extargs_debug_out,extargs_log_get_timestamp};

use std::error::Error;
use std::boxed::Box;
use std::rc::Rc;
use std::cell::RefCell;

extargs_error_class!{ParseStateError}

#[derive(Debug)]
#[derive(Clone)]
pub (crate) enum StateOptVal {
	LeftArgs(Vec<String>),
	OptDest(String),
	CmdPaths(String),
}

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
				extargs_new_error!{ParseStateError,"[{}] already set args",self.args[self.curidx as usize]}
			}

			if self.shortcharargs < 0 {
				self.shortcharargs = 0;
			}
			self.shortcharargs += nargs;
		} else {
			if self.longargs > 0 {
				extargs_new_error!{ParseStateError,"[{}] not handled", self.args[self.curidx as usize]}
			}
			if self.longargs < 0 {
				self.longargs = 0;
			}
			self.longargs += nargs;
		}
		Ok(())
	}

	fn find_key_cls(&mut self) -> Result<Option<ExtKeyParse>,Box<dyn Error>> {
		let mut retv :Option<ExtKeyParse> = None; 
		let mut oldcharidx :i32;
		let mut oldidx :i32;
		let mut idx :i32;
		if self.ended > 0 {
			return Ok(retv);
		}

		if self.longargs >= 0 {
			extargs_assert!(self.curcharidx < 0 , "curcharidx[{}]", self.curcharidx);
			self.curidx += self.longargs;
			extargs_assert!(self.args.len() as i32 >= self.curidx , "len[{}] < [{}]", self.args.len(), self.curidx);
			self.longargs = -1;
			self.validx = -1;
			self.keyidx = -1;
		}

		oldcharidx = self.curcharidx;
		oldidx = self.curidx;
		extargs_log_info!("oldcharidx [{}] oldidx [{}]", oldcharidx, oldidx);

		if oldidx >= self.args.len() as i32 {
			self.curidx = oldidx;
			self.curcharidx = -1;
			self.shortcharargs = -1;
			self.longargs = -1;
			self.keyidx = -1;
			self.validx = -1;
			self.ended = 1;
			return Ok(retv);
		}

		if oldcharidx >= 0 {
			let c = format!("{}",self.args[oldidx as usize]);
			if c.len() as i32 <= oldcharidx {
				oldidx += 1;
				extargs_log_trace!("oldidx [{}] [{}] [{}]", oldidx, c, oldcharidx);
				if self.shortcharargs > 0 {
					oldidx += self.shortcharargs;
				}
				extargs_log_trace!("oldidx [{}] __shortcharargs [{}]", oldidx, self.shortcharargs);
				self.curidx = oldidx;
				self.curcharidx = -1;
				self.shortcharargs = -1;
				self.keyidx = -1;
				self.validx = -1;
				self.longargs = -1;
				/*for next args parse*/
				return self.find_key_cls();
			}
			let cb = c.as_bytes();
			let curch = cb[oldcharidx as usize] as char;
			extargs_log_trace!("argv[{}][{}] {}", oldidx, oldcharidx, curch);
			idx = (self.cmdpaths.len() - 1) as i32;
			while idx >= 0 {
				let cmd = self.cmdpaths[idx as usize].clone();
				for opt in cmd.get_cmdopts().iter() {
					if !opt.is_flag() {
						continue;
					}
					if opt.flag_name() == KEYWORD_DOLLAR_SIGN {
						continue;
					}

					if opt.short_flag().len() > 0 {
						if opt.short_flag().eq(&format!("{}",curch)) {
							self.keyidx = oldidx;
							self.validx = oldidx + 1;
							self.curidx = oldidx;
							self.curcharidx = oldcharidx + 1;
							extargs_log_info!("{} validx [{}]", opt.string(), self.validx);
							retv=Some(opt.clone());
							return Ok(retv);
						}
					}
				}
				idx -= 1;
			}
			extargs_new_error!{ParseStateError,"can not parse ({})", self.args[oldidx as usize]}
		} else {
			if !self.bundlemode {
				let curarg = format!("{}",self.args[oldidx as usize]);
				if self.longprefix.len() > 0 && curarg.starts_with(&self.longprefix) {
					if curarg.eq(&self.longprefix) {
						/*it is the end to parse*/
						self.keyidx = -1;
						self.curidx = oldidx + 1;
						self.curcharidx = -1;
						self.validx = oldidx + 1;
						self.shortcharargs = -1;
						self.longargs = -1;
						self.ended = 1;
						idx  = self.curidx;
						while idx < self.args.len() as i32 {
							self.leftargs.push(format!("{}",self.args[idx as usize]));
							idx += 1;
						}
						retv = None;
						return Ok(retv);
					}
					idx = (self.cmdpaths.len() - 1 ) as i32;
					while idx >= 0 {
						let cmd = self.cmdpaths[idx as usize].clone();
						for opt in cmd.get_cmdopts().iter() {
							if !opt.is_flag() {
								continue;
							}
							if opt.flag_name() == KEYWORD_DOLLAR_SIGN {
								continue;
							}
							extargs_log_info!("[{}]longopt {} curarg {}", idx, opt.long_opt(), curarg);
							if opt.long_opt().eq(&curarg) {
								self.keyidx = oldidx;
								oldidx += 1;
								self.validx = oldidx;
								self.shortcharargs = -1;
								self.longargs = -1;
								extargs_log_info!("oldidx {} (len {})", oldidx, self.args.len());
								self.curidx = oldidx;
								self.curcharidx = -1;
								retv = Some(opt.clone());
								return Ok(retv);
							}
						}
						idx -= 1;
					}
					extargs_new_error!{ParseStateError,"can not parse ({})", self.args[oldidx as usize]}
				} else if self.shortprefix.len() > 0 && curarg.starts_with(&self.shortprefix) {
					if curarg.eq(&self.shortprefix) {
						if self.parseall {
							self.leftargs.push(format!("{}",curarg));
							oldidx += 1;
							self.curidx = oldidx;
							self.curcharidx = -1;
							self.longargs = -1;
							self.shortcharargs = -1;
							self.keyidx = -1;
							self.validx = -1;
							return self.find_key_cls();
						} else {
							self.ended = 1;
							idx = oldidx;
							while idx < self.args.len() as i32 {
								self.leftargs.push(format!("{}",self.args[idx as usize]));
								idx += 1;
							}
							self.validx = oldidx;
							self.keyidx = -1;
							self.curidx = oldidx;
							self.curcharidx = -1;
							self.shortcharargs = -1;
							self.longargs = -1;
							retv = None;
							return Ok(retv);
						}
					}
					oldcharidx = self.shortprefix.len() as i32;
					self.curidx = oldidx;
					self.curcharidx = oldcharidx;
					return self.find_key_cls();
				}
			} else {
				//
				//	not bundle mode ,it means that the long prefix and short prefix are the same
				//	so we should test one by one
				//	first to check for the long opt
				//	
				let curarg = format!("{}",self.args[oldidx as usize]);
				idx = (self.cmdpaths.len() - 1 ) as i32;
				while idx >= 0 {
					let cmd = self.cmdpaths[ idx as usize].clone();
					for opt in cmd.get_cmdopts().iter() {
						if !opt.is_flag() {
							continue;
						}
						if opt.flag_name() == KEYWORD_DOLLAR_SIGN {
							continue;
						}

						extargs_log_info!("[{}]({}) curarg [{}]", idx, opt.long_opt(), curarg);
						if opt.long_opt().eq(&curarg) {
							self.keyidx = oldidx;
							self.validx = oldidx + 1;
							self.shortcharargs = -1;
							self.longargs = -1;
							extargs_log_info!("oldidx {} (len {})", oldidx, self.args.len());
							self.curidx = oldidx + 1;
							self.curcharidx = -1;
							retv = Some(opt.clone());
							return Ok(retv);
						}
					}
					idx -= 1;
				}

				idx = (self.cmdpaths.len() - 1) as i32;
				while idx >= 0 {
					let cmd = self.cmdpaths[ idx as usize].clone();
					for opt in cmd.get_cmdopts().iter() {
						if !opt.is_flag() {
							continue;
						}
						if opt.flag_name() == KEYWORD_DOLLAR_SIGN {
							continue;
						}

						extargs_log_info!("[{}]({}) curarg [{}]", idx, opt.short_opt(), curarg);
						if opt.short_opt().eq(&curarg) {
							self.keyidx = oldidx;
							self.validx = oldidx + 1;
							self.shortcharargs = -1;
							self.longargs = -1;
							self.curidx = oldidx;
							self.curcharidx = opt.short_opt().len() as i32;
							retv = Some(opt.clone());
							return Ok(retv);
						}
					}
					idx -= 1;
				}
			}
		}

		let cname = format!("{}", self.args[oldidx as usize]);
		let okv = self.find_sub_command(&cname);
		if okv.is_some() {
			let c = okv.unwrap();
			extargs_log_info!("find {}", self.args[oldidx as usize]);
			self.keyidx = oldidx;
			self.curidx = oldidx + 1;
			self.validx = oldidx + 1;
			self.curcharidx = -1;
			self.shortcharargs = -1;
			self.longargs = -1;
			retv = Some(c.clone());
			return Ok(retv);
		}

		if self.parseall {
			self.leftargs.push(format!("{}",self.args[oldidx as usize]));
			oldidx += 1;
			self.keyidx = -1;
			self.validx = -1;
			self.curidx = oldidx;
			self.curcharidx = -1;
			self.shortcharargs = -1;
			self.longargs = -1;
			return self.find_key_cls();
		} else {
			self.ended = 1;
			idx = oldidx;
			while idx < self.args.len() as i32 {
				self.leftargs.push(format!("{}",self.args[idx as usize]));
				idx += 1;
			}
			self.keyidx = -1;
			self.curidx = oldidx;
			self.curcharidx = -1;
			self.shortcharargs = -1;
			self.longargs = -1;
		}

		return Ok(retv);
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
		validx = self.validx;
		return Ok((validx,optval,keycls));
	}

	pub (crate) fn get_cmd_paths(&self) -> Vec<ParserCompat> {
		return self.cmdpaths.clone();
	}
}


#[derive(Clone)]
pub struct ParserState {
	innerrc : Rc<RefCell<InnerParserState>>,
}

impl ParserState {
	pub (crate) fn new(args :Vec<String>,maincmd :ParserCompat,optattr :ExtArgsOptions) -> ParserState {
		ParserState {
			innerrc : Rc::new(RefCell::new(InnerParserState::new(args,maincmd,optattr))),
		}
	}

	pub (crate) fn get_cmd_paths(&self) -> Vec<ParserCompat> {
		return self.innerrc.borrow().get_cmd_paths();
	}

	pub (crate) fn step_one(&self) -> Result<(i32,Option<StateOptVal>,Option<ExtKeyParse>),Box<dyn Error>> {
		return self.innerrc.borrow_mut().step_one();
	}

	pub (crate) fn add_parse_args(&self,nargs :i32) -> Result<(),Box<dyn Error>> {
		return self.innerrc.borrow_mut().add_parse_args(nargs);
	}
}