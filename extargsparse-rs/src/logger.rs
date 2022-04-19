
use std::env;

use log::{LevelFilter};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root,RootBuilder,ConfigBuilder};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;


fn _extargs_get_environ_var(envname :&str) -> String {
	match env::var(envname) {
		Ok(v) => {
			format!("{}",v)
		},
		Err(_e) => {
			String::from("")
		}
	}
}

const DEFAULT_MSG_FMT :&str = "{d(%Y-%m-%d %H:%M:%S)}[{l}][{f}:{L}] {m}\n";

pub (crate) fn extargs_proc_log_init(prefix :&str) -> i32 {
		let mut msgfmt :String = String::from(DEFAULT_MSG_FMT);
		let mut getv :String;
		let mut retv :i32 = 0;
		let mut level :LevelFilter  = log::LevelFilter::Error;
		let mut rbuiler :RootBuilder;
		let mut cbuild :ConfigBuilder;
		let mut key :String;
		let wfile :String ;
		key = format!("{}_MSGFMT", prefix);
		getv = _extargs_get_environ_var(&key);
		if getv.len() > 0 {
			msgfmt = format!("{}",getv);
		}
		let stderr =ConsoleAppender::builder().encoder(Box::new(PatternEncoder::new(&msgfmt))).target(Target::Stderr).build();

        key = format!("{}_LEVEL", prefix);
        getv = _extargs_get_environ_var(&key);
        if getv.len() > 0 {
        	match getv.parse::<i32>() {
        		Ok(v) => {
        			retv = v;
        		},
        		Err(e) => {
        			retv = 0;
        			eprintln!("can not parse [{}] error[{}]", getv,e);
        		}
        	}
        }

        if retv >= 40 {
        	level = log::LevelFilter::Trace;
        } else if retv >= 30 {
        	level = log::LevelFilter::Debug;
        } else if retv >= 20 {
        	level = log::LevelFilter::Info;
        } else if retv >= 10 {
        	level = log::LevelFilter::Warn;
        }

	    cbuild = Config::builder()
	        .appender(
	            Appender::builder()
	                .filter(Box::new(ThresholdFilter::new(level)))
	                .build("stderr", Box::new(stderr)),
	        );
	    rbuiler =  Root::builder().appender("stderr");

	    key = format!("{}_LOGFILE",prefix);
	    wfile = _extargs_get_environ_var(&key);
	    if wfile.len() > 0 {
	    	let logfile = FileAppender::builder().encoder(Box::new(PatternEncoder::new(&msgfmt))).build(&wfile).unwrap();

	        cbuild = cbuild.appender(Appender::builder().build("logfile", Box::new(logfile)));
	        rbuiler = rbuiler.appender("logfile");
	    }

	    let config = cbuild.build(rbuiler.build(level)).unwrap();
	    let _handle = log4rs::init_config(config).unwrap();
		retv	
}


