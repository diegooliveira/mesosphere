

use std::fs::File;
use console;
use std::io::prelude::*;

pub fn walk<T>(descriptors : Vec<String>, handler : T) -> ()
    where T : Fn(&String, String) -> () {

    for desc in descriptors {
    
        match File::open(&desc) {
        
            Ok(mut file) => {
            
                let mut content = String::new();
                match file.read_to_string(&mut content) {
                    Ok(_) => {
                        handler(&desc, content);
                    },
                    Err(why) => {
                        console::erro(format!("Error reading {} - cause: {}", &desc, why));
                    }
                }
            },
            Err(why) => {
                console::erro(format!("Error opening \"{}\" - cause: {}", &desc, why));
            }
        }
    }

}

