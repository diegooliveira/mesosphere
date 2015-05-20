
use std::path::Path;
use std::process::Command as ProcessCommand;
use hyper::client::*;
use hyper::header::{Authorization, Basic};
use hyper::method::Method;

use console;
use super::Command;
use help::HelpPrinter;
use arguments::{Arguments, ValueArgument};
use configuration::{Configuration, CONFIGURATION_OPTIONS_HELP_TEXT};


pub struct Upload;

impl Command for Upload {

	fn execute(&self, mut args: Arguments) {

        let prefix = match args.get_option("--folder") {
		    ValueArgument::Supplied(prefix) => prefix,
		    ValueArgument::MissingValue => {
		        console::erro(format!("Error - Missing folder value, use --folder name"));
		        return;
		    },
		    _ => { ".".to_string() }
		};

        // Reading the cluster configuration
		if let Some(cluster) = Configuration::load(&mut args){
            let binaries = args.get_arguments();
            if binaries.len() == 0 {
                console::erro(format!("Error - Inform at least one file"));
            } else {
                for binary in binaries {
                    console::info(format!("Sending {}", binary));
                    let path = Path::new(&binary);
                    
                    match path.file_name() {
                    
                        Some(name) => {
                        
                            use std::path::Component;
                        
                            let mut client = Client::new();
                            let prefix_path = Path::new(&prefix);
                            let mut path_to_create = cluster.binary.address.clone();
                            for component in prefix_path.components() {
                                
                                match component {
                                    Component::Normal(name) => {
                                        path_to_create = format!("{}/{}", path_to_create,  name.to_string_lossy() );
                                        let mkcol = Method::Extension("MKCOL".to_owned());
                                        if let Some(ref user) = cluster.binary.user {
                                            client.request(mkcol, &path_to_create)
                                                .header(Authorization(Basic { username: user.clone(), password: cluster.binary.passwd.clone() }))
                                                .send().unwrap();
                                        } else {
                                            client.request(mkcol, &path_to_create)
                                                .send().unwrap();
                                        }
                                    },
                                    _ => {}
                                }
                            }
                            let binary_name = name.to_string_lossy();
                            let target_url = format!("{}/{}/{}", cluster.binary.address, prefix, binary_name);
                            let mut commands = vec!["-s".to_string()];
                            
                            if let Some(ref user) = cluster.binary.user {
                                commands.push("-u".to_string());
                                
                                let user_auth = if let Some(ref passwd) = cluster.binary.passwd {
                                    format!("{}:{}", user, passwd )
                                } else {
                                    user.clone()
                                };
                                commands.push(user_auth);
                                
                            }
                            
                            commands.push("-T".to_string());
                            commands.push(binary.clone());
                            commands.push(target_url.clone());
                            
                            match ProcessCommand::new("curl") 
                                 .args(&commands)
                                 .output() {
                                 
                                Ok(output) => {
                                    console::success(format!("Success sending '{}', path: {}", binary_name, target_url));
                                    console::debug(format!("HttpStatus : {}", output.status));
                                    console::debug(format!("stdout: {}", String::from_utf8_lossy(&output.stdout)));
                                    console::debug(format!("stderr: {}", String::from_utf8_lossy(&output.stderr)));
                                },
                                Err(why) =>  println!("{}" , why),
                            };
                        },
                        None => console::erro(format!("Invalid file name {}", &binary)),
                    }
                }
            }
		
		};
		
	}
	
	fn show_short_help(&self, hp : &mut HelpPrinter){
		hp.short("upload", "Uploads a binary to the cluster binary repository");
	}
	
	fn show_long_help(&self, hp : &mut HelpPrinter){
		hp.long(UPLOAD_OPTIONS_HELP_TEXT);
		hp.long(CONFIGURATION_OPTIONS_HELP_TEXT);
	}
	
	fn is_called(&self, name: &String) -> bool {
		return "upload" == name;
	}	

}
pub const UPLOAD_OPTIONS_HELP_TEXT : &'static str = 
"Upload a guiven artifact to the binary strore of the target environment. This
command uses the curl internaly to handle large file upload. And asumes the
server is running WebDav.

Usage:
    psmesos upload [options] <files..>

Upload Options:
    --folder    The folder to save the file in the remote server";    



