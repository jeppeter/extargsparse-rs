

#[macro_export]
macro_rules! error_class {
	($type:ident) => {
	#[derive(Debug,Clone)]
	pub struct $type {
		msg :String,		
	}

	impl $type {
		fn create(c :&str) -> $type {
			$type {msg : format!("{}",c)}
		}
	}

	impl fmt::Display for $type {
		fn fmt(&self,f :&mut fmt::Formatter) -> fmt::Result {
			write!(f,"{}",self.msg)
		}
	}

	impl Error for $type {}
	};
}

#[macro_export]
macro_rules! new_error {
	($type:ty,$($a:expr),*) => {
		{
		let mut c :String= format!("[{}:{}][{}]",file!(),line!(),stringify!($type));
		c.push_str(&(format!($($a),*)[..]));
		return Err(Box::new(<$type>::create(c.as_str())));
	  }
	};
}