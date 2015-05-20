use super::Command;
use help::HelpPrinter;
use arguments::Arguments;
use console;
use configuration::{Configuration, CONFIGURATION_OPTIONS_HELP_TEXT};

pub struct List;

impl Command for List {

    fn execute(&self, mut args: Arguments){
        
        // Reading the cluster configuration
		let cluster = match Configuration::load(&mut args){
		    Some(cfg) => cfg,
		    None => {
		        return;
		    }
		};
		console::info("Success".to_string());
		
    }
    
    fn show_short_help(&self, hp : &mut HelpPrinter){
		hp.short(COMMAND_NAME, "List all deployed configurations");
	}
	
	fn show_long_help(&self, hp : &mut HelpPrinter){
		hp.long(LONG_HELP_TEXT);
        hp.long(CONFIGURATION_OPTIONS_HELP_TEXT);
	}
	
	fn is_called(&self, name: &String) -> bool {
		return COMMAND_NAME == name;
	}	

}

const COMMAND_NAME : &'static str = "list";
const LONG_HELP_TEXT : &'static str = 
"";
