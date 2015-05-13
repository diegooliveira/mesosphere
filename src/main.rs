
extern crate rustc_serialize;
extern crate getopts;

mod command;
mod help;
mod deploy;
mod status;

use std::env;

fn main() {

	let args: Vec<String> = env::args().collect();

	let mut commands = command::CommandList::new();
	commands.register(Box::new(deploy::Deploy));
	commands.register(Box::new(status::Status));
	commands.process(args);
	
}


