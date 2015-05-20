

use arguments::{Arguments, ValueArgument}; 
use std::str::FromStr;
use term;

static mut log_level : LogLevel = LogLevel::Debug;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum LogLevel {
	Trace = 1,
	Debug,
	Info,
	Erro,
	Off
}

pub fn configure(args: &mut Arguments) {
	unsafe {
		log_level = match args.get_option("--log") {
			ValueArgument::Supplied(level) => {
				match LogLevel::from_str(&level) {
				    Ok(value) => value,
				    Err(_) => {
				        erro(format!("Invalid log level {}, fallback to debug", level));
				        LogLevel::Debug
				    }
				}
			}, 
            ValueArgument::MissingValue => {
			    LogLevel::Info
			},
			ValueArgument::NotSupplied => {
			    if args.has_flag("-v") {
			        LogLevel::Info
    	        } else { 
    			    LogLevel::Erro
    			}
			}
		}
	};
}

impl FromStr for LogLevel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "trace" => Ok(LogLevel::Trace),
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "erro" => Ok(LogLevel::Erro),
            "off" => Ok(LogLevel::Off),
            _ => Err(()),
        }
    }
}

pub fn erro(msg: String) {
	unsafe {
		if LogLevel::Erro >= log_level {
    		match term::stdout(){
    		    Some(mut term) => {
    		        term.fg(term::color::RED).unwrap();
    		        term.attr(term::Attr::Bold).unwrap();
			        println!("{}", msg);    
			        term.reset().unwrap();
    		    },
    		    None => {
    		        println!("{}", msg);    
    		    }
    		}
		}
	}
}

pub fn info(msg: String) {
	unsafe {   
		if LogLevel::Info >= log_level {
			println!("{}", msg);
		}
	}
}

pub fn debug(msg: String) {
	unsafe {   
		if LogLevel::Debug >= log_level {
			println!("{}", msg);
		}
	}
}

pub fn success(msg: String) {
	unsafe {   
		if LogLevel::Info >= log_level {
			match term::stdout(){
    		    Some(mut term) => {
    		        term.fg(term::color::GREEN).unwrap();
    		        term.attr(term::Attr::Bold).unwrap();
			        println!("{}", msg);    
			        term.reset().unwrap();
    		    },
    		    None => {
    		        println!("{}", msg);    
    		    }
    		}
		}
	}
}

pub const CONSOLE_OPTIONS_HELP_TEXT : &'static str = "
Console Options:
    -v             Turn on info log.
    --log LEVEL    One of trace, debug, info, erro or off";
    



