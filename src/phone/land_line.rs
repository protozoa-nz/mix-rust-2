use crate::phone::dial::Dial;

#[derive(Debug)]
pub struct LandLine {
    pub phone_number: Option<String>
}

impl Dial for LandLine {
    fn dial(&mut self, number: String)-> bool{
        println!("dialing from {:?} to {}", self.phone_number, number);
        true
    }
}


