
use super::key::{ExtKeyParse};
use super::options::{ExtArgsOptions,OPT_SCREEN_WIDTH};
use super::logger::{extargs_debug_out};
use super::{extargs_assert};
use std::rc::Rc;
use serde_json::{Value};


pub struct ParserCompat {
	pub keycls :Option<Rc<ExtKeyParse>>,
	pub cmdname :String,
	pub cmdopts :Vec<Box<ExtKeyParse>>,
	pub subcmds :Vec<Box<ParserCompat>>,
	pub helpinfo :String,
	pub callfunction :String,
	pub screenwidth :i32,
	pub epilog :String,
	pub description :String,
	pub prog :String,
	pub usage :String,
	pub version :String,
}

pub (crate) fn new(_cls :Option<Rc<ExtKeyParse>> , _opt :Option<Rc<ExtArgsOptions>>) -> ParserCompat {
	let mut retc :ParserCompat = ParserCompat {
		keycls : None,
		cmdname : "".to_string(),
		cmdopts : Vec::new(),
		subcmds : Vec::new(),
		helpinfo : "".to_string(),
		callfunction : "".to_string(),
		screenwidth : 80,
		epilog : "".to_string(),
		description : "".to_string(),
		prog : "".to_string(),
		usage : "".to_string(),
		version : "".to_string(),
	};
	let mut tmps :String;
	let mut jsonv :Value;

	if _cls.is_some() {
		let c :Rc<ExtKeyParse> = _cls.unwrap();
		extargs_assert!(c.is_cmd(),"{} must be cmd", c.string());
		retc.keycls = Some(c.clone());
		retc.cmdname = c.cmd_name();
		/*no cmdopts no subcommands*/		
		retc.helpinfo = format!("{} handler", retc.cmdname);
		tmps = c.help_info();
		if tmps.len() > 0 {
			retc.helpinfo = tmps;
		}
		tmps = c.func_name();
		if tmps.len() > 0 {
			retc.callfunction = tmps;
		}
	} else {
		tmps = r#"{{}}"#.to_string();
		jsonv = serde_json::from_str(&tmps).unwrap();
		match ExtKeyParse::new("","main",&jsonv,false,false,false,"--","-",false) {
			Ok(_cv) => {
				retc.keycls = Some(Rc::new(_cv));
			},
			Err(_e) => {
				panic!("can not parse [{}] error[{:?}]", tmps,_e);
			}
		}
	}
	retc.screenwidth = 80;

	if _opt.is_some() && _opt.as_ref().unwrap().get_value(OPT_SCREEN_WIDTH).is_some()  {
		retc.screenwidth = _opt.as_ref().unwrap().get_int(OPT_SCREEN_WIDTH);
	}

	retc
}