
use help::HelpPrinter;
use arguments::Arguments;
use command::Command;
use command::deploy::Deploy;
use command::status::Status;
use console;

pub struct PsMesos {
	commands : Vec<Box<Command>>,
}


impl PsMesos {

	/// Create a new CommandList
	pub fn new() -> Self {
	
	    let mut commands : Vec<Box<Command>> = Vec::new();
		
		commands.push(Box::new(Deploy));
	    commands.push(Box::new(Status));
	
		return PsMesos {commands : commands}
	}

	/// Show help for all commands
	pub fn show_help(&self){		
		let mut help_printer = HelpPrinter::new();
		for command in &self.commands {
			command.show_short_help(&mut help_printer);
		}
		help_printer.short("help", "Show this help or for a specific command");
		help_printer.print();
	}

	/// Process the supplied program arguments with the registereds commands
	pub fn process(self, args: Vec<String>) {
		let mut arguments = Arguments::new(&args);

		console::configure(&mut arguments);

		match arguments.get_command() {
			Some(command_name) => {

				// Taking care of the help command
				if command_name == "help" {
					let values = arguments.get_arguments();
					if values.len() == 1 {
						let command_name = &values[0];
						match self.commands.iter().filter( |c| c.is_called(command_name) ).last() {
							None => {
								console::erro(format!("Invalid help: {}\n", command_name) );
								self.show_help();
							},
							Some(c) => {
								let mut help_printer = HelpPrinter::new();
								c.show_long_help(&mut help_printer);
								help_printer.long(console::CONSOLE_OPTIONS_HELP_TEXT);
							},
						} 
					} else {
						self.show_help();
					}

				} else {
					//let arguments = Arguments::new(params);
					self.execute(&command_name, arguments);			
				}
			},
			None => {
				console::erro(format!("Select a command"));
				self.show_help();
			}
		}
	}

	pub fn execute(&self, name : &String, arguments: Arguments) {

		match self.commands
			.iter()
			.filter( |c| c.is_called(name) )
			.last() {
				None => {
					println!("Invalid command: {}\n", name);
					self.show_help();
				},
				Some(c) => c.execute(arguments),
			} 
	}
}
