
use command::Command;
use help::HelpPrinter;
use arguments::Arguments;

pub struct Deploy;

impl Command for Deploy {

	fn execute(&self, args: Arguments){
		println!("deploy {:?}", args);
	}
	
	fn show_short_help(&self, hp : &mut HelpPrinter){
		hp.short("deploy".to_string(), "Deploys the service or job configuration".to_string()	);
	}
	
	fn show_long_help(&self, hp : &mut HelpPrinter){
		hp.long("
Teste de help longo		
		
		".to_string());
	}
	
	fn is_called(&self, name: &String) -> bool {
		return "deploy" == name;
	}
	
}
