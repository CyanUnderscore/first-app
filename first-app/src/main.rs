// first : creating a message handler and an enum for the different tupes of messages
// #[derive(Debug)]

use rand::{self, Rng};

enum Message {
    Sucess,
    Error(i32), // return a Error code
    Warning(Box<str>), // send a Warning
    Message(Box<str>) // send a message to the user
}

fn MessageHandler(message : Message) { //will handle the messages
    match message {
        Message::Sucess => println!("Sucess"),
        Message::Error(errCode) => println!("Error Code : {} refer to the documentation for more information", errCode),
        Message::Warning(str) => println!("{}", str),
        Message::Message(str) => println!("{}", str)
    }
}

fn simulateLaoding() {
    let mut charge_bar : [char; 12] = ['[', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', ']'];
    for percent in 1..11 {
        if !load(percent) {
            continue;
        }else {
            charge_bar[usize::try_from(percent).unwrap()] = '#';
            println!("{}", String::from_iter(&charge_bar));  
        }    
    } 
}

fn load(featureID : u32) -> bool  {
    let random : f32 = rand::thread_rng().gen();
    println!("geting the data of object {} :", featureID);
    if random > 0.9 {
        MessageHandler(Message::Error(1));
        return false
    } else {
        MessageHandler(Message::Sucess);
        return true
    }
}

fn main() {
    // first we will simulate a laoding
    simulateLaoding();
}
