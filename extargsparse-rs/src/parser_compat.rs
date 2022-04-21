
use super::key::{ExtKeyParse};
use super::options::{ExtArgsOptions};

pub struct ParserCompat {
	pub keycls :Option<ExtKeyParse>,
	pub cmdname :String,
	pub cmdopts :Vec<ExtKeyParse>,
	pub subcmds :Vec<Box<ParserCompat>>,
	pub helpinfo :String,
	pub callfunction :String,
	pub screenwidht :i32,
	pub epilog :String,
	pub description :String,
	pub prog :String,
	pub usage :String,
	pub version :String,
}

pub (crate) fn new(_cls :Option<Box<ExtKeyParse>> , _opt :Option<Box<ExtArgsOptions>>) -> ParserCompat {
	let retc :ParserCompat = ParserCompat {
		keycls : None,
		cmdname : "".to_string(),
		cmdopts : Vec::new(),
		subcmds : Vec::new(),
		helpinfo : "".to_string(),
		callfunction : "".to_string(),
		screenwidht : 80,
		epilog : "".to_string(),
		description : "".to_string(),
		prog : "".to_string(),
		usage : "".to_string(),
		version : "".to_string(),
	};

	retc
}