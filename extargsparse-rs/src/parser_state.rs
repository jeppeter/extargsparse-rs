

use super::parser_compat::{ParserCompat};

pub (crate) struct parserState {
	cmdpaths :Vec<Box<ParserCompat>>,
	curidx : i32,
	curcharidx :i32,
	shortcharargs :i32,
	longargs :i32,
	keyidx :i32,
	
}