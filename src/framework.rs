
use configuration::Cluster;
use hyper::Client;
use hyper::header;
use hyper::status::StatusCode;
use hyper::client::response::Response;
use hyper::mime::Mime;

pub trait Framework {

    fn deploy(&self, content: &String, cluster: &Cluster);

}


impl Framework {

    pub fn of(descripor: &String) -> Option<Box<Framework>> {
        if descripor.ends_with(".job") {
            Some(Box::new(Chronos))
        } else if descripor.ends_with(".srv") {
            Some(Box::new(Marathon))
        } else {
            None
        }
    }

}

struct Chronos;

impl Framework for Chronos {

    fn deploy(&self, content: &String, cluster: &Cluster){
        match post(content, &cluster.chronos, "/scheduler/iso8601") {
            Some(response) => {
                if !response.status.is_success() {
                    println!("Error deploing, status code: {}", response.status);
                }
            },
            None => {}
        
        }
    }

}

struct Marathon;

impl Framework for Marathon {

    fn deploy(&self, content: &String, cluster: &Cluster){
        match post(content, &cluster.marathon, "/v2/apps"){
            Some(response) => {
                if response.status == StatusCode::Conflict {
                    match read("id", &content) {
                        Some(id) => {
                            let srv_address = "/v2/apps/".to_string() + &id;
                            put(content, &cluster.marathon, &srv_address);
                        },
                        None => {
                            println!("Impossible to update, missing id field");
                        }
                    }
                } else if !response.status.is_success() {
                    println!("Error deploing, status code: {}", response.status);
                }
            },
            _ => {}
        }
    }
}

fn post(content: &str, address: &str, uri: &str) -> Option<Response> {
    let path = address.to_owned() + uri;
    let mime: Mime = "application/json".parse().unwrap();
    let mut client = Client::new();
    match client.post(&path)
        .header(header::Connection::close())
        .header(header::ContentType(mime))
        .body(content).send() {
        Ok(response) => {
            Some(response)
        }, 
        Err(why) => {
            println!("error send data to {}", address); 
            return None
        }
    }
}

fn put(content: &str, address: &str, uri: &str) -> Option<Response> {
    let path = address.to_owned() + uri;
    let mime: Mime = "application/json".parse().unwrap();
    let mut client = Client::new();
    match client.put(&path)
        .header(header::Connection::close())
        .header(header::ContentType(mime))
        .body(content).send() {
        Ok(response) => {
            Some(response)
        }, 
        Err(why) => {
            println!("error send data to {}", address); 
            return None
        }
    }
}

fn read(field: &str, content: &str) -> Option<String> {
    use regex::Regex;
    let regex = "\"".to_string() + field + "\" : \"(.*)\"";
    let re = match Regex::new(&regex) {
        Ok(re) => {
            for cap in re.captures_iter(content) {
                return Some(cap.at(1).unwrap().to_string());
            }
            return None;
        },
        Err(err) => panic!("{}", err),
    };

}


