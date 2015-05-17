
extern crate rustc_serialize;
extern crate getopts;
extern crate hyper;
extern crate term;
extern crate regex;

mod command;
mod help;
mod framework;
mod deploy;
mod console;
mod status;
mod configuration;
mod arguments;
mod file_walker;

use std::env;

fn main() {

	let args: Vec<String> = env::args().collect();
	
	let mut commands = command::CommandList::new();
	commands.register(Box::new(deploy::Deploy));
	commands.register(Box::new(status::Status));
	commands.process(args);
	
}


