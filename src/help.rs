
pub struct HelpPrinter {
	commands : Vec<String>,
	descriptions: Vec<String>,	
	command_size: u32,
}

impl HelpPrinter {
	
	pub fn new() -> Self {
		return HelpPrinter{
			commands : Vec::new(),
			descriptions : Vec::new(),
			command_size : 0,
		}
	}
	
	pub fn long(&mut self, help: String){
		println!("{}", help);
	}

	pub fn short(&mut self, name: String, desc: String){
	
		let command_size = name.len() as u32;
		if self.command_size < command_size {
			self.command_size = command_size;
		}

		self.commands.push(name);
		self.descriptions.push(desc);
	}

	pub fn print(&self) {
		println!("Commands:");
		for i in (0..self.commands.len()) {
			let command = &self.commands[i];
			let desc = &self.descriptions[i];
			
			print!("\t{}", command);
			let spaces = self.command_size - (command.len() as u32) + 4;
			for _ in (0..spaces){
			    print!(" ");
			}
			println!("{}", desc);
		}
	}
}
