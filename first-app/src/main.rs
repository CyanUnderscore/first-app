// first : creating a message handler and an enum for the different tupes of messages
// #[derive(Debug)]

use rand::{self, Rng};
use std::process::Command;

enum Message {
    Sucess,
    Error(i32), // return a Error code
    Warning(Box<str>), // send a Warning
    Message(Box<str>) // send a message to the user
}

fn message_handler(message : Message) { //will handle the messages
    match message {
        Message::Sucess => println!("Sucess"),
        Message::Error(error_code) => println!("Error Code : {} refer to the documentation for more information", error_code),
        Message::Warning(str) => println!("Warning : {}", str),
        Message::Message(str) => println!("System Message : {}", str)
    }
}

fn simulate_loading() {
    let mut charge_bar : [char; 12] = ['[', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', ']'];
    message_handler(Message::Message("loading the data ...".into()));
    message_handler(Message::Warning("watch out the current version is unstable".into()));
    'outer : for percent in 1..11 {
        if !load(percent) {
            println!("the program ran into an error while loading component {} please restart the program", percent);
            break 'outer;
        }else {
            charge_bar[usize::try_from(percent).unwrap()] = '#';
            println!("{} {}%", String::from_iter(&charge_bar), percent * 10 );  
        }    
    } 
}

fn load(feature_id : u32) -> bool  {
    println!("geting the data of object {} :", feature_id);
    let mut child = Command::new("sleep").arg("0.5").spawn().unwrap();
    let _result = child.wait().unwrap();
    let random : f32 = rand::thread_rng().gen();
        if random > 0.9 {
        message_handler(Message::Error(1));
        return false
    } else {
        message_handler(Message::Sucess);
        return true
    }
}

fn main() {
    // first we will simulate a laoding
    simulate_loading();
}
