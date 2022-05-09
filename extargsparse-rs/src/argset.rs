
use super::namespace::{NameSpaceEx};
use std::error::Error;


pub trait ArgSetImpl {
	fn new() -> Self;
	fn set_value(&mut self, k :&str, ns :NameSpaceEx) -> Result<(),Box<dyn Error>>;
}