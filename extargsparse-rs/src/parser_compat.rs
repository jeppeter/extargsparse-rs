
use super::key::{ExtKeyParse,KEYWORD_BOOL,KEYWORD_VALUE,KEYWORD_STRING,KEYWORD_HELP};
use super::options::{ExtArgsOptions,OPT_SCREEN_WIDTH,OPT_EPILOG,OPT_DESCRIPTION,OPT_PROG,OPT_USAGE,OPT_VERSION};
use super::logger::{extargs_debug_out};
use super::{extargs_assert,extargs_log_warn};
use super::funccall::{ExtArgsMatchFuncMap};

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
	let mut isopt :bool = false;

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

	if _opt.is_some() {
		isopt = true;
	}

	if isopt  {
		let optc = _opt.as_ref().unwrap();
		if optc.get_value(OPT_SCREEN_WIDTH).is_some() {
			retc.screenwidth = optc.get_int(OPT_SCREEN_WIDTH);	
		}
		retc.epilog = optc.get_string(OPT_EPILOG);
		retc.description = optc.get_string(OPT_DESCRIPTION);
		retc.prog = optc.get_string(OPT_PROG);
		retc.usage = optc.get_string(OPT_USAGE);
		retc.version = optc.get_string(OPT_VERSION);		
	}

	if retc.screenwidth < 40 {
		retc.screenwidth = 40;
	}

	retc
}

impl ParserCompat {
	fn get_help_info(keycls :&ExtKeyParse,mapv :&ExtArgsMatchFuncMap) -> String {
		let hlp = keycls.get_keyattr("opthelp");
		let mut rets :String = "".to_string();
		if hlp.is_some() {
			let hlpfunc = hlp.unwrap().string();
			let funchelp = mapv.get_help_func(&hlpfunc);
			if funchelp.is_some() {
				let callf = funchelp.unwrap();
				return callf(keycls);
			}
			extargs_log_warn!("can not find function [{}] for opthelp", hlpfunc);
		}

		if keycls.type_name() == KEYWORD_BOOL {
			if keycls.get_bool_v(KEYWORD_VALUE) == true {
				rets.push_str(&(format!("{} set false default(True)", keycls.opt_dest())));
			} else {
				rets.push_str(&(format!("{} set true default(False)", keycls.opt_dest())));
			}
		} else if keycls.type_name() == KEYWORD_STRING && keycls.get_string_v(KEYWORD_VALUE) == "+" {
			if keycls.is_flag() == true {
				rets.push_str(&(format!("{} inc", keycls.opt_dest())));
			} else {
				extargs_assert!(false == true,"cmd({}) can not set value({:?})", keycls.cmd_name(), keycls.get_string_v(KEYWORD_STRING));
			}
		} else if keycls.type_name() == KEYWORD_HELP {
			rets.push_str(&(format!("to display this help information")));
		} else {
			if keycls.is_flag() == true {
				rets.push_str(&(format!("{} set default {:?}",keycls.opt_dest(),keycls.value())));
			} else {
				rets.push_str(&(format!("{} command exec", keycls.cmd_name())));
			}
		}

		rets
	}
}