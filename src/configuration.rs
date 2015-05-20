

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use rustc_serialize::json;

use arguments::{Arguments, ValueArgument};
use console;


#[derive(RustcDecodable, Debug)]
pub struct Configuration {
    default_cluster: String,
    cluster: Vec<Cluster>,
    ext: Option<String>
}

#[derive(RustcDecodable, Debug)]
pub struct Cluster {
    name: String,
    pub marathon: String,
    pub chronos: String,
    pub binary: Binary
}

#[derive(RustcDecodable, Debug)]
pub struct Binary {
    pub address : String,
    pub user: Option<String>,
    pub passwd: Option<String>,
}

impl Configuration {

    pub fn load(arg: &mut Arguments) -> Option<Cluster> {
    
        // Checking if there is a custom configuration flag and if it is valid
        let path = match arg.get_option("--cfg") {
		    ValueArgument::NotSupplied => ".mesosphere".to_string(),
		    ValueArgument::Supplied(path) => path,
		    ValueArgument::MissingValue => {
		        println!("Invalid config file, use --cfg PAHT_TO_FILE");
		        return None
		    },
		};
		
		// Open the configuration file and reading it's content.
		match File::open(&path) {
		    Ok(mut file) =>{
		    
		        let mut content = String::new();
                match file.read_to_string(&mut content) {
                    Ok(_) => {},
                    Err(why) => {
                        console::erro(format!("Error reading {} : {}", &path, why));
                        return None;
                    }
                }
                
                match json::decode::<Configuration>(&content) {
                    Ok(config) => {
                        
                        let target = match arg.get_option("--env") {
		                    ValueArgument::NotSupplied => config.default_cluster.clone(),
		                    ValueArgument::Supplied(target) => target,
		                    ValueArgument::MissingValue => {
		                        println!("Invalid target env, use --env ENV_NAME");
		                        return None
		                    },
		                };
                        
                        for x in config.cluster {
                            if x.name == target {
                                return Some(x);
                            }
                        }
                          
                        console::erro(format!("Undefined target env '{}'", target));
                        return None 
                        
                    },
                    Err(why) => {
                    
                        // Error parsing the config file
                        console::erro(format!("Invalid content in '{}': {}", &path, why)); //
                        return None
                    }
                }
		    },
		    Err(cause) => {
		        //Error opening the file
		        console::erro(format!("Error reading configuration '{}': {}", &path, cause));
		        return None
		    }
		}
    }    
}

pub const CONFIGURATION_OPTIONS_HELP_TEXT : &'static str = "
Configuratio Options:
    --cfg PATH     Path to the configuration file, defaults to a file called 
                   \".mesosphere\" in the path.
    --env NAME     Name of the target cluster. If not supplied it uses the 
                   \"default_cluster\" attribute from the configuration.";

