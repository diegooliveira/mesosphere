
extern crate rustc_serialize;
extern crate getopts;
extern crate hyper;
extern crate term;
extern crate regex;

mod command;
mod help;
mod framework;
mod console;
mod configuration;
mod arguments;
mod file_walker;
mod psmesos;

use std::env;
use psmesos::PsMesos;

fn main() {

	let args: Vec<String> = env::args().collect();
	
	let psmesos = PsMesos::new();
	psmesos.process(args);
	
}


