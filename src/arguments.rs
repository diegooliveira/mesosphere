

#[derive(Debug)]
pub struct Arguments {
    args: Vec<ArgValue>,
    flags: Vec<String>,
    command: Option<String>,
    parameters: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ValueArgument {
    Supplied(String),
    MissingValue,
    NotSupplied
}

#[derive(Debug, Clone)]
pub struct ArgValue {
    pub name: String,
    value: ValueArgument
}

impl Arguments {

    pub fn new(args: &Vec<String>) -> Self {
        let command = if args.len() > 1 { Some(args[1].clone()) } else { None  };
        
        let mut parsed_args = Vec::new();
	    let mut parsed_params = Vec::new();
  	    let mut flags = Vec::new();
	    
	    let mut curr_idx = 2;
	    while curr_idx < args.len() {
	        let current = args[curr_idx].clone();
	        if is_paramenter(&current) {
	            
	            // Checking if it is the last element
	            if curr_idx < args.len() -1 {
	            
	            
	                // Take a look if the next element is also a paramiter or not
	                let next = args[curr_idx + 1].clone();
	                if !is_paramenter(&next) && !is_flag(&next) {
	                
	                    // Saving the paramenter value
	                    let arg_value = ArgValue {name: current, value: ValueArgument::Supplied(next) };
	                    parsed_args.push(arg_value);
	                    curr_idx = curr_idx + 1;
	                } else {
	                
	                    // Saving a paramiter without value
	                    let arg_value = ArgValue {name: current, value: ValueArgument::MissingValue };
	                    parsed_args.push(arg_value);	                
	                }
	                
	            } else {
    	            let arg_value = ArgValue {name: current, value: ValueArgument::MissingValue };
	                parsed_args.push(arg_value);	
	            }

	        } else if is_flag(&current) {
	            flags.push(current);
	        } else {
	            // Saving an argument
	            parsed_params.push(current);
	        }
    	    curr_idx = curr_idx + 1;
	    }
        
        return Arguments{args: parsed_args, parameters: parsed_params, command : command, flags: flags}
    }
	
	pub fn get_command(&self) -> Option<String> {
	    return self.command.clone();
	}
	
	/// Get the argument value and remove from the avealeble args
	pub fn get_option(&mut self, name : &str) -> ValueArgument {
	    
	    match self.args.iter().position(|arg| *arg.name == name.to_owned()) {
	        None => return ValueArgument::NotSupplied,
	        Some(idx) => {
	            let arg_value = self.args.remove(idx);
	            return arg_value.value;
	        }
	    }
	}
	
	pub fn has_flag(&mut self, name: &str) -> bool {
	     match self.flags.iter().position(|arg| arg == name) {
	        None => {
	            return false
	        },
	        Some(idx) => {
	            self.flags.remove(idx);
	            return true;
	        }
	    }
	}
	
	pub fn get_remmaning_params(&mut self) -> Option<Vec<ArgValue>> {
	
	    if self.args.is_empty() {
	        return None;
	    } else {
	        let params = self.args.clone();
	        self.args = Vec::new();
	        return Some(params);
	    }
	}
	
	pub fn get_arguments(&mut self) -> Vec<String> {
	    let parameters = self.parameters.clone();
	    self.parameters = Vec::new();
	    return parameters;
	}
}

fn is_paramenter(name: &String) -> bool {
    return name.starts_with("--");
}

fn is_flag(name: &String) -> bool {
    return name.starts_with("-");
}

#[cfg(test)]
mod test {

    #[test]
    fn should_parser_command_name() {
    
        let args = prepare(vec!["program-name", "command"]);
        
        match args.get_command() {
            Some(command) => {
                assert_eq!(command, "command");        
            },
            None => assert!(false), 
        };
    }
    
    #[test]
    fn should_parser_arguents() {
        let mut args = prepare(vec!["program-name", "command", "value1", "value2"]);
        let options = args.get_arguments();
        assert!(options.len() == 2, "should have two options but was {}", options.len());   
    }
    
    #[test]
    fn should_parser_flags() {
        let mut args = prepare(vec!["program-name", "command", "-a", "-b"]);
        assert!(args.has_flag("-b"), "Should have the flag");
        assert!(!args.has_flag("-b"), "Should have cleaned the flag");
    }
    
    fn prepare(params: Vec<&str>) -> super::Arguments {
        let args = super::Arguments::new(&params.iter().map(|x| x.to_string()).collect());
        return args;
    }
    


}


