
use command::Command;
use help::HelpPrinter;
use arguments::Arguments;
use framework::Framework;
use console;
use configuration::{Configuration, CONFIGURATION_OPTIONS_HELP_TEXT};
use file_walker;


pub struct Deploy;

impl Command for Deploy {

	fn execute(&self, mut args: Arguments){
		
		// Reading the cluster configuration
		let cluster = match Configuration::load(&mut args){
		    Some(cfg) => cfg,
		    None => {
		        return;
		    }
		};
		
		// Validating any aditional parameter
		match args.get_remmaning_params() {
		    Some(params) => {
		        console::erro(format!("Invalid parameters:"));
		        for param in params {
    		        console::erro(format!("\t{}", param.name));
		        }
		        return;     
		    },
		    None => { }
		}
		
	    let descriptors = args.get_arguments();
		if descriptors.is_empty() {
		   console::erro(format!("Error: Missing deployment descriptor")); 
		} else {
		    file_walker::walk(descriptors, |descriptor, content| {
		    
		    
		        match Framework::of(descriptor) {
		        
		            Some(framework) => {
    		            console::info(format!("Sending {}", &descriptor));
    		            framework.deploy(&content, &cluster);
		            },
		            None => {
		                console::erro(format!("Invalid file: {}", descriptor));
		            }
		        }
		    });
		}
	}
	
	fn show_short_help(&self, hp : &mut HelpPrinter){
		hp.short("deploy", "Deploys the service or job configuration");
	}
	
	fn show_long_help(&self, hp : &mut HelpPrinter){
		hp.long(LONG_HELP_TEXT);
        hp.long(CONFIGURATION_OPTIONS_HELP_TEXT);
	}
	
	fn is_called(&self, name: &String) -> bool {
		return "deploy" == name;
	}	
}

const LONG_HELP_TEXT : &'static str = 
"Deploy or update a configuration description to a framework. The target framework
is discovered by parameter or file extension. 

Usage:
    mesosphere deploy [options] [args...]
        
Arguments:
    args    Paths to the deployment descriptors";

