//!  Defined [`ArgSetImpl`]
use super::namespace::{NameSpaceEx};
use std::error::Error;


pub trait ArgSetImpl {
	fn set_value(&mut self,prefix :&str,k :&str, nsname :&str, ns :NameSpaceEx) -> Result<(),Box<dyn Error>>;
	fn new() -> Self where Self :Sized;
}
