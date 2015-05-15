

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use rustc_serialize::json;
use arguments::{Arguments, ValueArgument};

#[derive(RustcDecodable, Debug)]
pub struct Configuration {
    default: String,
    cluster: Vec<Cluster>,
    ext: Option<String>
}

#[derive(RustcDecodable, Debug)]
pub struct Cluster {
    name: String,
    pub marathon: String,
    pub chronos: String,
    pub binary: String
}


impl Configuration {

    pub fn load(arg: &mut Arguments) -> Option<Cluster> {
    
        // Checking if there is a custom configuration flag and if it is valid
        let path = match arg.get_param("--cfg") {
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
                        println!("Error reading {} : {}", &path, why);
                    }
                }
                
                match json::decode::<Configuration>(&content) {
                    Ok(config) => {
                        
                        let target = match arg.get_param("--env") {
		                    ValueArgument::NotSupplied => config.default.clone(),
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
                          
                        println!("Undefined target env {}", target);
                        return None 
                        
                    },
                    Err(why) => {
                    
                        // Error parsing the config file
                        println!("Invalid content in \"{}\": {}", &path, why); //
                        return None
                    }
                }
		    },
		    Err(cause) => {
		        //Error opening the file
		        println!("Error reading configuration \"{}\": {}", &path, cause);
		        return None
		    }
		    
		}
    }    
}

pub const CONFIGURATION_ARGS_OPTIONS : &'static str = "
Configuratio Options:
    --cfg PATH     Path to the configuration file
    --env NAME     Name of the target cluster
";

