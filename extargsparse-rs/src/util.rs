
#[allow(dead_code)]
pub (crate) fn check_in_array(v :Vec<String>, cmpv :&str) -> bool {
	for s in v.iter() {
		let vs = format!("{}",s);
		if vs == cmpv {
			return true;
		}
	}
	return false;
}

#[allow(dead_code)]
pub (crate) fn format_array_string(v :Vec<String>) -> String {
	let mut rets :String = "".to_string();
	let mut i :i32=0;
	for s in v.iter() {
		if i > 0 {
			rets.push_str(",");
		}
		rets.push_str(s);
		i += 1;
	}
	rets
}
