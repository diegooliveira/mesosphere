
use command::Command;
use help::HelpPrinter;
use arguments::Arguments;
use framework::Framework;
use configuration::Configuration;
use file_walker;


pub struct Deploy;

impl Command for Deploy {

	fn execute(&self, mut args: Arguments){
		
		let cluster = match Configuration::load(&mut args){
		    Some(cfg) => cfg,
		    None => {
		        return;
		    }
		};
		
		match args.get_remmaning_params() {
		    Some(params) => {
		        println!("Invalid parameters:");
		        for param in params {
    		        println!("\t{}", param.name);
		        }
		        return;     
		    },
		    None => { }
		}
		
	    let descriptors = args.get_arguments();
		if descriptors.is_empty() {
		   println!("Error: Missing deployment descriptor"); 
		} else {
		    file_walker::walk(descriptors, |descriptor, content| {
		    
		    
		        match Framework::of(descriptor) {
		        
		            Some(framework) => {
    		            framework.deploy(&content, &cluster);
		            },
		            None => {
		                println!("Invalid file: {}", descriptor);
		            }
		        }
		    });
		}
		
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
