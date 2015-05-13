
use command::{Command, Arguments, ValueArgument};
use help::HelpPrinter;


pub struct Status;

const HELP_TEXT : &'static str = "
List the status of all the supplied configurations. It is possible to pass the
configuration name in two forms: the path of an especification file or the 
job/service name.

Sample usage:

    mesosphere status *
        - Show the status for all especifications in the current dir

    mesosphere status hourely.job site.srv
        - Show the status for the job espcification \"hourely.job\" and the
        service especification \"site.srv\".
        
    mesosphere status --job my-test-job --job other-job-name --service my-app
        - Show the status 
";

impl Command for Status {

	fn execute(&self, args: Arguments){
		
		/*
		let cluster = match Cluster::fromArgs(args) {
		    Some(c) => c,
		    Err(why) => {
		    
		    }
		};
		
		*/
		
		match args.get_value("--srv") {
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
		
		match args.get_value("--job") {
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
		
		match args.get_value("-r") {
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
		
		
		//list_status( || args.get_arguments());
	}
	
	fn show_short_help(&self, hp : &mut HelpPrinter){
		hp.short("status".to_string(), "Show the service or job statys".to_string());
	}
	
	fn show_long_help(&self, hp : &mut HelpPrinter){
		hp.long(HELP_TEXT.to_string());
	}
	
	fn is_called(&self, name: &String) -> bool  {
		return "status" == name;
	}
}

