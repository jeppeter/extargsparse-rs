
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
	static ref HELP_SIZE_KEYWORDS :Vec<String> = {
		vec![format!("optnamesize"), format!("optexprsize"), format!("opthelpsize"), format!("cmdnamesize"),format!("cmdhelpsize")]
	};
}

pub (crate) struct HelpSize {
	intvalue :HashMap<String,i32>,
}

pub (crate) fn new() -> HelpSize {
	let mut retv :HelpSize = HelpSize {
		intvalue :HashMap::new(),
	};
	for k in HELP_SIZE_KEYWORDS.iter() {
		retv.intvalue.insert(k.to_string(),0);
	}
	retv
}
impl HelpSize {
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
