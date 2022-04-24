

use super::parser_compat::{ParserCompat};
use super::options::{ExtArgsOptions,OPT_LONG_PREFIX,OPT_SHORT_PREFIX};

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

	if retv.longprefix.len() == 0 || retv.shortprefix.len() == 0 || 
		retv.longprefix == retv.shortprefix {
			retv.bundlemode = true;
		} else {
			retv.bundlemode = false;
		}

	retv
}