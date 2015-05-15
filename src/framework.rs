
use configuration::Cluster;

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
        println!("Chronos({}) -> \n{}", cluster.chronos, content);
    }

}

struct Marathon;

impl Framework for Marathon {

    fn deploy(&self, content: &String, cluster: &Cluster){
        println!("Marathon({}) -> \n{}", cluster.marathon, content);
    }
   
}
