//!  
//!  This crate is plugin for extargsparse it should be used
//!  directly with extargsparse_worker 
//!  ```rust
//!  use extargsparse_codegen::{ArgSet,extargs_load_commandline,extargs_map_function};
//!  use extargsparse_worker::argset::{ArgSetImpl};
//!  use extargsparse_worker::{extargs_error_class,extargs_new_error};
//!  use extargsparse_worker::key::{ExtKeyParse};
//!  use extargsparse_worker::namespace::NameSpaceEx;
//!  use extargsparse_worker::funccall::ExtArgsParseFunc;
//!  use extargsparse_worker::parser::ExtArgsParser;
//!  
//!  
//!  use std::error::Error;
//!  use lazy_static::lazy_static;
//!  use regex::Regex;
//!  use std::sync::Arc;
//!  use std::cell::RefCell;
//!  use std::any::Any;
//!  use std::collections::HashMap;
//!  use std::borrow::Borrow;
//!  use std::borrow::BorrowMut;
//!  
//!  extargs_error_class!{OptHdlError}
//!  
//!  #[derive(ArgSet)]
//!  struct DepSt {
//!  	subnargs :Vec<String>,
//!  	strv : String,
//!  	list :Vec<String>,
//!  }
//!  
//!  #[derive(ArgSet)]
//!  struct RdepSt {
//!  	strv :String,
//!  	subnargs : Vec<String>,
//!  	list :Vec<String>,
//!  }
//!  
//!  #[derive(ArgSet)]
//!  struct SubCmdStruct {
//!  	verbose :i32,
//!  	pair :Vec<String>,
//!  	dep :DepSt,
//!  	rdep :RdepSt,
//!  	args :Vec<String>,
//!  }
//!  
//!  
//!  fn pair_key_handle(ns :NameSpaceEx, validx :i32, keycls :ExtKeyParse, params :Vec<String>) -> Result<i32,Box<dyn Error>> {
//!      println!("validx [{}]",validx);
//!  
//!      if params.len() < (validx + 2) as usize {
//!      	extargs_new_error!{OptHdlError,"need 2 args"}
//!      }
//!      let mut vc :Vec<String> = ns.get_array(&(keycls.opt_dest()));
//!      vc.push(format!("{}",params[validx as usize]));
//!      vc.push(format!("{}",params[(validx + 1) as usize]));
//!      let _ = ns.set_array(&(keycls.opt_dest()),vc)?;
//!      return Ok(2);
//!  }
//!  
//!  /*
//!  this is the example call the mut pointer change the value
//!  */
//!  fn dep_handler(_ns :NameSpaceEx, _args :Option<Arc<RefCell<dyn ArgSetImpl>>>, _ctx : Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
//!  	println!("in dep_handler");
//!  	if _args.is_some() {
//!  		println!("some _args");
//!  		let ctx = _args.as_ref().unwrap().clone();
//!          let c  = ctx.as_ptr() as *const RefCell<dyn ArgSetImpl>;
//!          let b = c.borrow();
//!          let cc = *b as *const SubCmdStruct;
//!          let bbcref :&SubCmdStruct = unsafe {cc.as_ref().unwrap()};
//!          println!("verbose {}", bbcref.verbose);
//!          println!("pair {:?}", bbcref.pair);
//!          println!("args {:?}", bbcref.args);
//!          println!("subnargs {:?}", bbcref.dep.subnargs);
//!          println!("strv {}", bbcref.dep.strv);
//!          println!("list {:?}",bbcref.dep.list);
//!  	} else {
//!  		println!("none of _args");
//!  	}
//!  	Ok(())
//!  }
//!  
//!  /*
//!  this is the example call the const pointer
//!  */
//!  fn rdep_handler(_ns :NameSpaceEx, _args :Option<Arc<RefCell<dyn ArgSetImpl>>>, _ctx : Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
//!      println!("in rdep_handler");
//!      if _args.is_some() {
//!          println!("some _args");
//!          let ctx = _args.as_ref().unwrap().clone();
//!          let mut c  = ctx.as_ptr() as *mut RefCell<dyn ArgSetImpl>;
//!          let b = c.borrow_mut();
//!          let cc = *b as *mut SubCmdStruct;
//!          let bbcref :&mut SubCmdStruct = unsafe {cc.as_mut().unwrap()};
//!          println!("verbose {}", bbcref.verbose);
//!          println!("pair {:?}", bbcref.pair);
//!          println!("args {:?}", bbcref.args);
//!          println!("subnargs {:?}", bbcref.rdep.subnargs);
//!          println!("strv {}", bbcref.rdep.strv);
//!          bbcref.rdep.list.push(format!("rdep"));
//!          println!("list {:?}",bbcref.rdep.list);
//!      } else {
//!          println!("none of _args");
//!      }
//!      Ok(())
//!  }
//!  
//!  
//!  #[extargs_map_function(optparse=pair_key_handle,dep_handler,rdep_handler)]
//!  fn main() -> Result<(),Box<dyn Error>> {
//!      let loads = r#"{
//!  		"verbose" : "+",
//!  		"pair|p!optparse=pair_key_handle!##to set pair parameters##" : [],
//!  		"dep<dep_handler>" : {
//!  			"$" : "*",
//!  			"list|L" :  [],
//!  			"str|S<strv>" : ""
//!  		},
//!  		"rdep<rdep_handler>" : {
//!  			"$" : "*",
//!  			"list|l" : [],
//!  			"str|s<strv>" : ""
//!  		}
//!  		}"#;
//!      let parser :ExtArgsParser = ExtArgsParser::new(None,None)?;
//!      extargs_load_commandline!(parser,loads)?;
//!      let v :SubCmdStruct = SubCmdStruct::new();
//!      let argv :Arc<RefCell<SubCmdStruct>> = Arc::new(RefCell::new(v));
//!      let _ = parser.parse_commandline_ex(None,None,Some(argv.clone()),None)?;
//!      Ok(())
//!  }
//!  ```
//!  


use proc_macro::TokenStream;
//use proc_macro2::Span;
use syn;
use std::sync::{Mutex,Arc};
use lazy_static::lazy_static;
use std::collections::HashMap;
use regex::Regex;
//use std::cmp::Ordering;

use rand::Rng;
use bytes::{BytesMut,BufMut};


use std::fmt::{Debug};
use std::fmt;
use std::error::Error;
use std::boxed::Box;

mod consts;
#[macro_use]
mod errors;
#[macro_use]
mod logger;
mod util;

use consts::*;
use logger::{em_debug_out,em_log_get_timestamp};
use util::{check_in_array};


//use std::cell::RefCell;
//use std::rc::Rc;



const RAND_NAME_STRING :[u8; 62]= *b"abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";




lazy_static! {
	static ref LINK_NAMES :Arc<Mutex<HashMap<String,String>>> = Arc::new(Mutex::new(HashMap::new()));
	static ref EXTARGS_FUNC_MAP_NAME : Arc<Mutex<String>> = Arc::new(Mutex::new(String::from("")));

	static ref ARGSET_KEYWORDS :Vec<String> = {
		let mut retv :Vec<String> = Vec::new();
		retv.push(format!("{}",KEYWORD_U64));
		retv.push(format!("{}",KEYWORD_I64));
		retv.push(format!("{}",KEYWORD_F64));
		retv.push(format!("{}",KEYWORD_U32));
		retv.push(format!("{}",KEYWORD_I32));
		retv.push(format!("{}",KEYWORD_F32));
		retv.push(format!("{}",KEYWORD_TYPE_STRING));
		retv.push(format!("{}",KEYWORD_TYPE_BOOL));
		retv.push(format!("{}",KEYWORD_VEC_STRING));
		retv
	};
}


fn get_random_bytes(num :u32, basevec :&[u8]) -> String {
	let mut retm = BytesMut::with_capacity(num as usize);
	let mut rng = rand::thread_rng();
	let mut curi :usize;

	for _i in 0..num {
		curi = rng.gen_range(0..basevec.len());
		retm.put_u8(basevec[curi]);
	}
	let a = retm.freeze();
	String::from_utf8_lossy(&a).to_string()
}


#[derive(Debug,Clone)]
struct FuncAttrs {
	helpfuncs :Vec<String>,
	jsonfuncs :Vec<String>,
	optparsefuncs : Vec<String>,
	callbackfuncs : Vec<String>,
}

impl FuncAttrs {
	fn set_funcnames(&mut self,k :&str, v :&str, input : syn::parse::ParseStream) -> Result<(),syn::Error> {
		if k.len() == 0 && v.len() == 0 {
			return Ok(());
		} else if v.len() == 0 {
			self.callbackfuncs.push(format!("{}",k));
		} else {
			if k == FUNC_CALLBACK {
				self.callbackfuncs.push(format!("{}",v));
			} else if k == FUNC_OPTHELP {
				self.helpfuncs.push(format!("{}",v));
			} else if k == FUNC_JSONFUNC {
				self.jsonfuncs.push(format!("{}",v));
			} else if k == FUNC_ACTFUNC {
				self.optparsefuncs.push(format!("{}",v));
			} else {
				let c = format!("we not accept [{}] keyword only accept {}|{}|{}|{}", k,
					FUNC_ACTFUNC,FUNC_OPTHELP,FUNC_JSONFUNC,FUNC_CALLBACK);
				return Err(syn::Error::new(input.span(),&c));
			}
		}
		return Ok(());
	}	
	fn format_code(&self) -> String {
		let mut rets :String = "".to_string();
		let mut fname :String = "st_functions".to_string();

		fname.push_str("_");
		fname.push_str(&(get_random_bytes(15,&RAND_NAME_STRING)));
		fname = fname.to_uppercase();
		rets.push_str("lazy_static ! {\n");
		rets.push_str(&format_tab_space(1));
		rets.push_str(&format!("static ref {} :HashMap<String,ExtArgsParseFunc> = {{\n",fname));
		rets.push_str(&format_tab_space(2));
		if self.helpfuncs.len() > 0 || self.jsonfuncs.len() > 0 || self.optparsefuncs.len() > 0 || self.callbackfuncs.len() > 0 {
			rets.push_str(&format!("let mut retv :HashMap<String,ExtArgsParseFunc> = HashMap::new();\n"));	
		} else {
			rets.push_str(&format!("let retv :HashMap<String,ExtArgsParseFunc> = HashMap::new();\n"));
		}
		
		if self.jsonfuncs.len() > 0 {
			for f in self.jsonfuncs.iter() {
				rets.push_str(&format_tab_space(2));
				rets.push_str(&format!("retv.insert(format!(\"{}\"), ExtArgsParseFunc::JsonFunc({}));\n",f,f));
			}
		}

		if self.optparsefuncs.len() > 0 {
			for f in self.optparsefuncs.iter() {
				rets.push_str(&format_tab_space(2));
				rets.push_str(&format!("retv.insert(format!(\"{}\"), ExtArgsParseFunc::ActionFunc({}));\n",f,f));
			}
		}

		if self.helpfuncs.len() > 0 {
			for f in self.helpfuncs.iter() {
				rets.push_str(&format_tab_space(2));
				rets.push_str(&format!("retv.insert(format!(\"{}\"), ExtArgsParseFunc::HelpFunc({}));\n",f,f));
			}
		}

		if self.callbackfuncs.len() > 0 {
			for f in self.callbackfuncs.iter() {
				rets.push_str(&format_tab_space(2));
				rets.push_str(&format!("retv.insert(format!(\"{}\"), ExtArgsParseFunc::CallbackFunc({}));\n",f,f));
			}
		}

		rets.push_str(&format_tab_space(2));
		rets.push_str(&format!("retv\n"));

		rets.push_str(&format_tab_space(1));
		rets.push_str("};\n");

		rets.push_str("}\n");
		{
			let mut scb = EXTARGS_FUNC_MAP_NAME.lock().unwrap();
			if self.helpfuncs.len() > 0 || self.jsonfuncs.len() > 0 || self.optparsefuncs.len() > 0 || self.callbackfuncs.len() > 0 {
				*scb = format!("{}",fname);	
			} else {
				*scb = format!("");
			}
			em_log_trace!("EXTARGS_FUNC_MAP_NAME [{}]",scb);
		}

		em_log_trace!("rets\n{}",rets);
		return rets;
	}
}

impl syn::parse::Parse for FuncAttrs {
	#[allow(unused_assignments)]
	fn parse(input : syn::parse::ParseStream) -> syn::parse::Result<Self> {
		let mut retv :FuncAttrs = FuncAttrs {
			helpfuncs : Vec::new(),
			jsonfuncs : Vec::new(),
			optparsefuncs : Vec::new(),
			callbackfuncs : Vec::new(),
		};
		let mut k :String = "".to_string();
		let mut v :String = "".to_string();
		loop {
			if input.peek(syn::Ident) {
				let c : syn::Ident = input.parse()?;
				em_log_trace!("ident [{}]", c);
				if k.len() == 0 {
					k = format!("{}", c);
				} else if v.len() == 0 {
					v = format!("{}", c);
				} else {
					let c = format!("we acept  xx=xx or xx format");
					return Err(syn::Error::new(input.span(),&c));					
				}
			} else if input.peek(syn::Token![=]) {
				let _c : syn::token::Eq = input.parse()?;
				em_log_trace!("=");
			} else if input.peek(syn::Token![,]) {
				let _c : syn::token::Comma = input.parse()?;
				em_log_trace!(",");
				retv.set_funcnames(&k,&v,input.clone())?;
				k = "".to_string();
				v = "".to_string();
			} else {
				if input.is_empty() {
					break;
				}
				let c = format!("not valid token [{}]",input.to_string());
				return Err(syn::Error::new(input.span(),&c));
			}			
		}
		retv.set_funcnames(&k,&v,input.clone())?;
		k = "".to_string();
		v = "".to_string();
		return Ok(retv);
	}
}

fn format_tab_space(isize :usize) -> String {
	let mut rets :String = "".to_string();
	while rets.len() < (isize * 4) {
		rets.push_str("    ");
	}
	return rets;
}

macro_rules! syn_error_fmt {
	($($a:expr),*) => {
		let cerr = format!($($a),*);
		eprintln!("{}",cerr);
		em_log_error!("{}",cerr);
		return cerr.parse().unwrap();
		//return syn::Error::new(
        //            Span::call_site(),
        //            $cerr,
        //        ).to_compile_error().to_string().parse().unwrap();
    }
}


///   this is function of functions map expand
#[proc_macro_attribute]
pub fn extargs_map_function(_args :TokenStream , input :TokenStream) -> TokenStream {
	let mut code :String = "".to_string();
	let nargs = _args.clone();
	let attrs  = syn::parse_macro_input!(nargs as FuncAttrs);
	em_log_trace!("attrs [{:?}]",attrs);

	/**/
	code.push_str(&attrs.format_code());
	code.push_str(&(input.to_string()));
	em_log_trace!("code \n{}",code);	
	code.parse().unwrap()
}


extargs_error_class!{TypeError}

fn get_name_type(n : syn::Field) -> Result<(String,String), Box<dyn Error>> {
	let name :String ;
	let mut typename :String = "".to_string();
	match n.ident {
		Some(ref _i) => {
			name = format!("{}",_i);
		},
		None => {
			extargs_new_error!{TypeError,"can not get"}
		}
	}

	match n.ty {
		syn::Type::Path(ref _p) => {
			let mut pidx :i32 = 0;
			if _p.path.leading_colon.is_some() {
				typename.push_str("::");
			}
			for _s in _p.path.segments.iter() {
				if pidx > 0 {
					typename.push_str("::");
				}
				typename.push_str(&(format!("{}",_s.ident)));
				//em_log_trace!("f [{}]",typename);
				match _s.arguments {
					syn::PathArguments::None => {},
					syn::PathArguments::AngleBracketed(ref _an) => {
						typename.push_str("<");
						let mut idx :i32 = 0;
						for _ii in _an.args.iter() {
							match _ii {
								syn::GenericArgument::Type(ref _pi) => {
									match _pi {
										syn::Type::Path(ref _pt) => {
											let mut jdx : i32 = 0;
											if idx > 0 {
												typename.push_str(",");
											}
											for _tt in _pt.path.segments.iter() {
												if jdx > 0 {
													typename.push_str("::");
												}
												typename.push_str(&(format!("{}", _tt.ident)));
												jdx += 1;
											}
										},
										_ => { extargs_new_error!{TypeError, "not "}}
									}
								},
								_ => {
									extargs_new_error!{TypeError,"no args type"}
								}
							}
							idx += 1;
						}
						typename.push_str(">");
					},
					syn::PathArguments::Parenthesized(ref _pn) => {
						extargs_new_error!{TypeError,"Parenthesized"}
					}
				}
				pidx += 1;
			}
		},
		_ => {
			extargs_new_error!{TypeError,"ty not support for"}
		}
	}
	em_log_trace!("name [{}] typename [{}]",name,typename);
	Ok((name,typename))
}

fn format_code(ident :&str,names :HashMap<String,String>, structnames :Vec<String>) -> String {
	let mut rets :String = "".to_string();
	let mut typeerrname :String = format!("{}_typeerror",ident);
	if structnames.len() > 0 {
		for i in structnames.clone() {
			/*to make the type check for ArgSetImpl*/
			rets.push_str(&format!("const _ :fn() = || {{\n"));
			rets.push_str(&(format_tab_space(1)));
			rets.push_str(&format!("fn assert_impl_all<T : ?Sized + ArgSetImpl>() {{}}\n"));
			rets.push_str(&(format_tab_space(1)));
			rets.push_str(&format!("assert_impl_all::<{}>();\n", i));
			rets.push_str(&format!("}};\n"));
		}
	}

	typeerrname.push_str("_");
	typeerrname.push_str(&(get_random_bytes(15,&RAND_NAME_STRING)));



	rets.push_str(&format!("extargs_error_class!{{{}}}\n",typeerrname));

	rets.push_str(&format!("impl ArgSetImpl for {} {{\n",ident));
	rets.push_str(&(format_tab_space(1)));
	rets.push_str(&format!("fn new() -> Self {{\n"));
	rets.push_str(&(format_tab_space(2)));
	rets.push_str(&format!("{} {{\n",ident));
	for (k,v) in names.clone().iter() {
		rets.push_str(&format_tab_space(3));
		if v == KEYWORD_TYPE_STRING {
			rets.push_str(&format!("{} : \"\".to_string(),\n", k));
		} else if v == KEYWORD_U32 || v == KEYWORD_I32 || v == KEYWORD_U64 || v == KEYWORD_I64 {
			rets.push_str(&format!("{} : 0,\n",k));
		} else if v == KEYWORD_F32  || v == KEYWORD_F64 {
			rets.push_str(&format!("{} : 0.0,\n",k));
		} else if v == KEYWORD_VEC_STRING {
			rets.push_str(&format!("{} : Vec::new(),\n",k));
		} else if v == KEYWORD_TYPE_BOOL {
			rets.push_str(&format!("{} : false,\n",k));
		}else  {
			/*to make new type*/
			rets.push_str(&format!("{} : {}::new(),\n",k,v));
		}
	}
	rets.push_str(&format_tab_space(2));
	rets.push_str(&format!("}}\n"));
	rets.push_str(&format_tab_space(1));
	rets.push_str(&format!("}}\n"));


	rets.push_str(&format_tab_space(1));
	rets.push_str(&format!("\n"));

	rets.push_str(&format_tab_space(1));
	rets.push_str(&format!("fn set_value(&mut self, prefix :&str,k :&str, nsname :&str, ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {{\n"));
	rets.push_str(&format_tab_space(2));
	rets.push_str(&format!("let mut extk :String = \"\".to_string();\n"));
	let mut i :i32 = 0;
	for (k,v) in names.clone().iter() {
		if !check_in_array(ARGSET_KEYWORDS.clone(), v) {
			continue;
		}


		rets.push_str(&format_tab_space(2));
		if i > 0 {
			rets.push_str(&format!("}} else if "));
		} else {
			rets.push_str(&format!("if "));
		}
		rets.push_str(&format!("k == \"{}\" {{\n", k));
		rets.push_str(&format_tab_space(3));
		rets.push_str(&(format!("extk = \"\".to_string();\n")));
		rets.push_str(&format_tab_space(3));
		rets.push_str(&(format!("extk.push_str(prefix);\n")));
		rets.push_str(&format_tab_space(3));
		rets.push_str(&(format!("if extk.len() > 0 {{\n")));
		rets.push_str(&format_tab_space(4));
		rets.push_str(&(format!("extk.push_str(\"_\");\n")));
		rets.push_str(&format_tab_space(3));
		rets.push_str(&(format!("}}\n")));
		rets.push_str(&format_tab_space(3));
		rets.push_str(&(format!("extk.push_str(\"{}\");\n",k)));
		//rets.push_str(&format_tab_space(3));
		//rets.push_str(&(format!("println!(\"will get [{{}}]\", nsname);\n")));
		rets.push_str(&format_tab_space(3));
		if v == KEYWORD_TYPE_STRING {
			rets.push_str(&format!("self.{} = ns.get_string(nsname);\n", k));
		} else if v == KEYWORD_I32 {
			rets.push_str(&format!("self.{} = ns.get_int(nsname) as i32;\n",k));
		} else if v == KEYWORD_U32 {
			rets.push_str(&format!("self.{} = ns.get_int(nsname) as u32;\n",k));
		} else if v == KEYWORD_F32 {
			rets.push_str(&format!("self.{} = ns.get_float(nsname) as f32;\n",k));
		} else if v == KEYWORD_I64 {
			rets.push_str(&format!("self.{} = ns.get_int(nsname);\n",k));
		} else if v == KEYWORD_U64 {
			rets.push_str(&format!("self.{} = ns.get_int(nsname) as u64;\n",k));
		} else if v == KEYWORD_F64 {
			rets.push_str(&format!("self.{} = ns.get_float(nsname);\n",k));
		} else if v == KEYWORD_TYPE_BOOL {
			rets.push_str(&format!("self.{} = ns.get_bool(nsname);\n",k));
		} else if v == KEYWORD_VEC_STRING {
			let mut bsubnargs : bool= false;
			let resubnargs;
			match Regex::new(".*_subnargs$") {
				Ok(v) => {
					resubnargs = v;
					bsubnargs = resubnargs.is_match(k);
					em_log_trace!("match [{}] [{:?}]",k,bsubnargs);
				},
				_ => {}
			}
			em_log_trace!("match [{}] [{:?}]",k,bsubnargs);

			if k == KEYWORD_SUBNARGS || k == KEYWORD_ARGS  {
				//rets.push_str(&(format!("println!(\"will get args[{}]\");\n",k)));
				//rets.push_str(&format_tab_space(3));				
				rets.push_str(&format!("self.{} = ns.get_array(nsname);\n",k));
			} else if  bsubnargs {
				//rets.push_str(&(format!("println!(\"will get args[{}] [{{}}]\",nsname);\n",k)));
				//rets.push_str(&format_tab_space(3));
				rets.push_str(&format!("self.{} = ns.get_array(nsname);\n",k));
			} else {
				rets.push_str(&format!("self.{} = ns.get_array(nsname);\n",k));
			}
		} 
		i += 1;
	}

	if structnames.len() > 0 {
		for s in structnames.clone() {
			for (k,v) in names.clone().iter() {
				if s.eq(v) {
					rets.push_str(&format_tab_space(2));
					if i > 0 {
						rets.push_str(&format!("}} else if "));
					} else {
						rets.push_str(&format!("if "));
					}
					rets.push_str(&format!("k.starts_with(&format!(\"{}_\")) {{\n",k));
					rets.push_str(&format_tab_space(3));
					rets.push_str(&format!("let nk = format!(\"{{}}\",k);\n"));
					rets.push_str(&format_tab_space(3));
					rets.push_str(&format!("let re = Regex::new(r\"^{}_\").unwrap();\n",k));
					rets.push_str(&format_tab_space(3));
					rets.push_str(&format!("let kn = re.replace_all(&nk,\"\").to_string();\n"));
					rets.push_str(&format_tab_space(3));
					rets.push_str(&(format!("extk = \"\".to_string();\n")));
					rets.push_str(&format_tab_space(3));
					rets.push_str(&(format!("extk.push_str(prefix);\n")));
					rets.push_str(&format_tab_space(3));
					rets.push_str(&(format!("if extk.len() > 0 {{\n")));
					rets.push_str(&format_tab_space(4));
					rets.push_str(&(format!("extk.push_str(\"_\");\n")));
					rets.push_str(&format_tab_space(3));
					rets.push_str(&(format!("}}\n")));
					rets.push_str(&format_tab_space(3));
					rets.push_str(&(format!("extk.push_str(\"{}\");\n",k)));
					rets.push_str(&format_tab_space(3));
					//rets.push_str(&(format!("println!(\"will down [{{}}]{{}}\",extk,kn);\n")));
					//rets.push_str(&format_tab_space(3));
					rets.push_str(&format!("self.{}.set_value(&extk,&kn,nsname,ns.clone())?;\n",k));
					/*no break just for next search*/
				}
			}
			i += 1;
		}
	}


	if i > 0 {
		rets.push_str(&format_tab_space(2));

		rets.push_str(&format!("}} else {{\n"));
		rets.push_str(&format_tab_space(3));

		rets.push_str(&format!("extargs_new_error!{{ {},\"[{{}}].{{}} not valid\" , prefix , k}}\n", typeerrname));
		rets.push_str(&format_tab_space(2));

		rets.push_str(&format!("}}\n"));
	}
	rets.push_str(&format_tab_space(2));
	rets.push_str(&format!("Ok(())\n"));
	rets.push_str(&format_tab_space(1));
	rets.push_str(&format!("}}\n"));
	rets.push_str(&format!("}}\n"));

	rets
}


#[proc_macro_derive(ArgSet)]
pub fn argset_impl(item :TokenStream) -> TokenStream {
	em_log_trace!("item\n{}",item.to_string());
	let co :syn::DeriveInput;
	let sname :String;
	let mut names :HashMap<String,String> = HashMap::new();
	let mut structnames :Vec<String> = Vec::new();

	match syn::parse::<syn::DeriveInput>(item.clone()) {
		Ok(v) => {
			co = v.clone();
		},
		Err(_e) => {
			syn_error_fmt!("not parse \n{}",item.to_string());
          	//return syn::Error::new(
            //        Span::call_site(),
            //        &format!(
            //            "not parse \n{}",
            //            item.to_string()
            //        ),
            //    ).to_compile_error().to_string().parse().unwrap();

        }
    }

    sname = format!("{}",co.ident);
    em_log_trace!("sname [{}]",sname);


    match co.data {
    	syn::Data::Struct(ref _vv) => {
    		match _vv.fields {
    			syn::Fields::Named(ref _n) => {
    				for _v in _n.named.iter() {
    					let res = get_name_type(_v.clone());
    					if res.is_err() {
    						syn_error_fmt!("{:?}",res.err().unwrap());
    					}
    					let (n,tn) = res.unwrap();
    					if tn.contains(KEYWORD_LEFT_ARROW) && tn != KEYWORD_VEC_STRING {
    						syn_error_fmt!("tn [{}] not valid",tn);
    					}
    					if names.get(&n).is_some() {
    						syn_error_fmt!("n [{}] has already in",n);
    					}

    					if !check_in_array(ARGSET_KEYWORDS.clone(),&tn) {
    						if !check_in_array(structnames.clone(), &tn) {
	    						em_log_trace!("input typename [{}]",tn);
	    						structnames.push(format!("{}",tn));    							
    						}
    					}

    					names.insert(format!("{}",n),format!("{}",tn));
    				}
    			},
    			_ => {
    				syn_error_fmt!("not Named structure\n{}",item.to_string());
    			}
    		}
    	},
    	_ => {
    		syn_error_fmt!("not struct format\n{}",item.to_string());
    	}
    }

    /*now to compile ok*/
    let cc = format_code(&sname,names.clone(),structnames.clone());
    em_log_trace!("cc\n{}",cc);

    cc.parse().unwrap()
}

#[allow(dead_code)]
#[derive(Clone,Debug)]
struct LoadParserAttr {
	parserident :String,
	strident :String,
}

#[allow(dead_code)]
impl LoadParserAttr {
	fn format_code(&self) -> String {
		let mut rets :String = "".to_string();
		{
			let c = EXTARGS_FUNC_MAP_NAME.clone();
			let sb = c.lock().unwrap();
			if sb.len() > 0 {
				rets.push_str(&format!("{}.load_commandline_string({},Some({}.clone()))\n",self.parserident,self.strident,sb));	
			} else {
				rets.push_str(&format!("{}.load_commandline_string({},None)\n",self.parserident,self.strident));
			}
			
		}

		em_log_trace!("rets [{}]", rets);		
		return rets;
	}
}

impl syn::parse::Parse for LoadParserAttr {
	#[allow(unused_assignments)]
	fn parse(input : syn::parse::ParseStream) -> syn::parse::Result<Self> {
		let mut retv = LoadParserAttr{
			parserident : "".to_string(),
			strident : "".to_string(),
		};

		let mut k :String = "".to_string();
		loop {
			if input.is_empty() {
				break;
			}
			if input.peek(syn::Ident) {
				let c :syn::Ident = input.parse()?;
				k.push_str(&format!("{}",c));
			} else if input.peek(syn::token::And) {
				let _c :syn::token::And = input.parse()?;
				k.push_str("&");
			} else if input.peek(syn::token::Paren) {
				let con ;
				{
					em_log_trace!("start parenthesized");
					let _c = syn::parenthesized!(con in input);
					em_log_trace!("end parenthesized [{}]", con.to_string());
					k.push_str(&format!("({})",con.to_string()));
					let _fields :Box<syn::Expr> = con.parse()?;
				}
			} else if input.peek(syn::token::Comma) {
				let _c : syn::token::Comma = input.parse()?;
				if retv.parserident.len() == 0 {
					retv.parserident = format!("{}",k);
				} else if retv.strident.len() == 0 {
					retv.strident = format!("{}",k);
				} else {
					let c = format!("we only accept two params");
					return Err(syn::Error::new(input.span(),&c));					
				}
				k = "".to_string();
			} else {
				let c = format!("not valid macro expand parameters\n{}",input.to_string());
				return Err(syn::Error::new(input.span(),&c));					
			}
		}

		if k.len() > 0 {
			if retv.parserident.len() == 0 {
				retv.parserident = format!("{}",k);
			} else if retv.strident.len() == 0 {
				retv.strident = format!("{}",k);
			} else {
				let c = format!("we only accept two params");
				return Err(syn::Error::new(input.span(),&c));					
			}
			k = "".to_string();
		}

		if retv.parserident.len() == 0 || retv.strident.len() == 0 {
			let c = format!("need two params\n{}",input.to_string());
			return Err(syn::Error::new(input.span(),&c));
		}


		em_log_trace!("{:?}",retv);
		return Ok(retv);
	}
}

/// expand extargs_map_function random make functions name
/// to make this function to expand  
/// ```
/// extargs_load_commandline!(parser,loads)?;
/// ```
/// to expand
/// ```
/// parser.load_command_line(loads,st_functions_xxww222)?;
/// ```
/// which st_functions_xxww222 is generated by extargs_map_function macros

#[proc_macro]
pub fn extargs_load_commandline(input :TokenStream) -> TokenStream {
	let mut code :String = "".to_string();
	em_log_trace!("input \n{}",input.to_string());
	let nargs = input.clone();
	let pattr :LoadParserAttr = syn::parse_macro_input!(nargs as LoadParserAttr);
	code.push_str(&pattr.format_code());
	em_log_trace!("code \n{}",code);
	return code.parse().unwrap();
}
