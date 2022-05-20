
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::rc::Rc;
use std::cell::RefCell;


pub const OPT_NAME_SIZE :&str = "optnamesize";
pub const OPT_EXPR_SIZE :&str = "optexprsize";
pub const OPT_HELP_SIZE :&str = "opthelpsize";
pub const CMD_NAME_SIZE :&str = "cmdnamesize";
pub const CMD_HELP_SIZE :&str = "cmdhelpsize";

lazy_static! {
	static ref HELP_SIZE_KEYWORDS :Vec<String> = {
		vec![format!("{}",OPT_NAME_SIZE), format!("{}",OPT_EXPR_SIZE), format!("{}",OPT_HELP_SIZE), format!("{}",CMD_NAME_SIZE),format!("{}",CMD_HELP_SIZE)]
	};
}

struct InnerHelpSize {
	intvalue :HashMap<String,i32>,
}

impl InnerHelpSize {
	pub (crate) fn new() -> InnerHelpSize {
		let mut retv :InnerHelpSize = InnerHelpSize {
			intvalue :HashMap::new(),
		};
		for k in HELP_SIZE_KEYWORDS.iter() {
			retv.intvalue.insert(k.to_string(),0);
		}
		retv
	}


	pub (crate) fn get_value(&self,k :&str) -> i32 {
		let mut retv :i32 = 0;
		let bv = self.intvalue.get(k);
		if bv.is_some() {
			retv = *(bv.unwrap());
		}
		retv
	}

	pub (crate) fn set_value(&mut self,k :&str, v :i32) {
		for kv in HELP_SIZE_KEYWORDS.iter() {
			if kv == k {
				let cmpv :i32 = *(self.intvalue.get(k).unwrap());
				if cmpv < v {
					self.intvalue.insert(k.to_string(),v);
				}
			}
		}
	}

	pub (crate) fn string(&self) -> String {
		let mut rets :String = "".to_string();
		let mut i :i32 = 0;

		for k in HELP_SIZE_KEYWORDS.iter() {
			if i > 0 {
				rets.push_str(",");
			}
			rets.push_str(&(format!("{}",k)));
			i += 1;
		}

		rets
	}
}

#[derive(Clone)]
pub struct HelpSize {
	innerrc : Rc<RefCell<InnerHelpSize>>,
}

impl HelpSize {
	pub (crate) fn new() -> HelpSize {
		HelpSize {
			innerrc : Rc::new(RefCell::new(InnerHelpSize::new())),
		}
	}

	pub (crate) fn set_value(&self,k :&str, v :i32) {
		self.innerrc.borrow_mut().set_value(k,v);
		return;
	}

	pub (crate) fn get_value(&self,k :&str) -> i32 {
		return self.innerrc.borrow().get_value(k);
	}

	pub (crate) fn string(&self) -> String {
		return self.innerrc.borrow().string();
	}

}
