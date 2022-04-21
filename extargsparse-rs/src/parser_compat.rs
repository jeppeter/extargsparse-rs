
use super::key::{ExtKeyParse};

pub struct ParserCompat {
	pub keycls :ExtKeyParse,
	pub cmdname :String,
	pub cmdopts :Vec<ExtKeyParse>,
	pub subcmds :Vec<Box<ParserCompat>>,
	pub helpinfo :String,
}