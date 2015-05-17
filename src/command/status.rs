
use super::Command;
use help::HelpPrinter;
use arguments::{Arguments, ValueArgument};
use framework::Framework;
use configuration::{Configuration, CONFIGURATION_OPTIONS_HELP_TEXT};
use file_walker;

pub struct Status;


impl Command for Status {

	fn execute(&self, mut args: Arguments){
		
		let cluster = match Configuration::load(&mut args){
		    Some(cfg) => cfg,
		    None => {
		        return;
		    }
		};

		match args.get_option("--srv") {
		    ValueArgument::Supplied(srv_id) => {
		        // List the content of the supplied directory
		        // and deploy everething 
		        //list_status( || vec![id]);
		        println!("from srv {}", srv_id)
		    },
		    ValueArgument::MissingValue => {
		        println!("Error: missing srv name, use --srv service-name");
		    },
		    _ => { }
		}
		
		match args.get_option("--job") {
		    ValueArgument::Supplied(job_id) => {
		        // List the content of the supplied directory
		        // and deploy everething 
		        //list_status( || vec![id]);
		        println!("from job {}", job_id)
		    },
		    ValueArgument::MissingValue => {
		        println!("Error: missing job name, use --job service-name");
		    },
		    _ => { }
		}
		
		match args.get_option("-r") {
		    ValueArgument::Supplied(directory) => {
		        // List the content of the supplied directory
		        // and deploy everething 
		        //list_status( || vec![""]);
		        println!("from recursive directory {}", directory)
		    },
		    ValueArgument::MissingValue => {
		        // Show a error message asking for the 
		        println!("Error: missing -r value, use -r path");
		    },
		    _ => {}
		}
		
		match args.get_remmaning_params() {
		    Some(params) => {
		        println!("Invalid parameters:");
		        for param in params {
    		        println!("\t{}", param.name);
		        }
		    },
		    None => {}
		}
		
		let descriptors = args.get_arguments();
		if !descriptors.is_empty() {
		    file_walker::walk(descriptors, |descriptor, content| {
		    
		        match Framework::of(descriptor) {
		        
		            Some(framework) => {
    		            framework.status_by_content(&content, &cluster);
		            },
		            None => {
		                println!("Invalid file: {}", descriptor);
		            }
		        }
		    });
		}
	}
	
	fn show_short_help(&self, hp : &mut HelpPrinter){
		hp.short("status", "Show the service or job statys");
	}
	
	fn show_long_help(&self, hp : &mut HelpPrinter){
		hp.long(HELP_TEXT);
		hp.long(CONFIGURATION_OPTIONS_HELP_TEXT);
	}
	
	fn is_called(&self, name: &String) -> bool  {
		return "status" == name;
	}
}

const HELP_TEXT : &'static str = 
"Show the status of all the supplied configurations. It is possible to pass the
configuration name in two forms: the path of an especification file or the 
job/service name.

Usage:
    mesosphere status [options] [args...]
 
Status Options:
    --srv SRV_ID     The service id
    --job JOB_NAME   The job name";

