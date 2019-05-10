use crate::phone::dial::Dial;
use crate::phone::dial_state::DialState;

#[derive(Debug)]
pub struct CellPhone {
    pub dial_state: DialState,
    pub battery_level_percent: u8,
    pub phone_number: Option<String>
}

impl Dial for CellPhone {
    fn dial(&mut self, number: String)-> bool{
        println!("dialing from {:?} to {}", self.phone_number, number);
        self.dial_state = DialState::Connected(number);
        true
    }
}
