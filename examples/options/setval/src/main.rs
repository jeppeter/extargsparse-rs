use extargsparse_worker::options::{ExtArgsOptions,OPT_SCREEN_WIDTH};
use std::error::Error;


fn main() -> Result<(),Box<dyn Error>> {


    let optstr = format!("{{ \"{}\" : 60}}",OPT_SCREEN_WIDTH);
    let options = ExtArgsOptions::new(&optstr)?;
    println!("screenwidth={}", options.get_int(OPT_SCREEN_WIDTH));
    let optstr = format!("{{ \"{}\" : 100}}",OPT_SCREEN_WIDTH);
    let options = ExtArgsOptions::new(&optstr)?;
    println!("screenwidth={}", options.get_int(OPT_SCREEN_WIDTH));
    Ok(())
}