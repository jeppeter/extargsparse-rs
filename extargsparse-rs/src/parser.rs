
use super::options::{ExtArgsOptions};
use super::parser_compat::{ParserCompat};


pub struct ExtArgsParser {
	options :Option<ExtArgsOptions>,
	maincmd :Option<ParserCompat>,
}