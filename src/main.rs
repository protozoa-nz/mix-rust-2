
mod phone;

use phone::dial::Dial;
use phone::cell_phone::CellPhone;
use phone::land_line::LandLine;
use phone::dial_state::DialState;
//TODO: percent type
//TODO: generics

type Point = (i32,i32); // Type alias

//Option type, either Some<Type> or None

fn dial_all(phones: Vec<&mut Dial>) {
   phones
       .into_iter() //
       .for_each(|phone| {
           phone.dial("123".to_string());
       })
}

fn main() {
    let battery_level_percent = 100;
    let mut cell = CellPhone{
        dial_state: DialState::OnHook,
        battery_level_percent,
        phone_number: Some("02742432323".to_string())
    };

    let mut land = LandLine{
        phone_number: Some("34343434".to_string())
    };

    let phones: Vec<&mut Dial> = vec![&mut cell, &mut land];
    dial_all(phones);


    match cell.dial_state {
        DialState::Connected(number) => {
            println!("cell was connected to number: {}", number);
        }

        DialState::OnHook => {
            println!("cell was on hook");
        }
        DialState::Disconnecting(_) => {
        
        }
        DialState::Dialing(_) => {
        
        }
    }
}
