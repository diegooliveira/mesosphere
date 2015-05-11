
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::env;



static USAGE: &'static str = "
Usage:
    mesosphere [--env=NAME... --cfg=FILE] <command> [<args>...]
    mesosphere --help
Options:
    --env=NAME...    The target environment name.
    --cfg=FILE       The configuration file path
    -h, --help       Show this help
Commands:
    deploy     Deploy the configuration
    undeploy   Undeploy the configuration
    status     Get the status";
    

#[derive(Debug,RustcDecodable)]
struct Args {
    arg_command: Option<Command>,
    flag_env: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
    					.and_then(|d| d.options_first(true)
                                           .version(Some("1.1.1".to_string()))
                                           .decode())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
	match args.arg_command {
		Some(command) => {
			command.run();
		},
		None => {
			println!("Installed commands: {}", USAGE);
		}
	}
}

#[derive(Debug, RustcDecodable)]
enum Command {
	Deploy,
	Undeploy
}

impl Command {

	fn run(self) {
		let argv: Vec<_> = env::args().map(|v| v.to_string()).collect();
        let argv: Vec<_> = argv.iter().map(|s| &**s).collect();
        let argv = &*argv;
        
        println!("{:?} - {:?}", self, argv);
	} 
	

}


