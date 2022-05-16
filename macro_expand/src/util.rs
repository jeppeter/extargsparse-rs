

pub (crate) fn check_in_array(v :Vec<String>, cmpv :&str) -> bool {
	for s in v.iter() {
		let vs = format!("{}",s);
		if vs == cmpv {
			return true;
		}
	}
	return false;
}
