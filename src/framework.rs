

#[derive(Debug)]
pub enum Framework {

    Marathon,
    Chronos

}

impl Framework {

    pub fn of(descripor: &String) -> Option<Framework> {
        if descripor.ends_with(".job") {
            Some(Framework::Chronos)
        } else if descripor.ends_with(".srv") {
            Some(Framework::Marathon)
        } else {
            None
        }
    }

}


