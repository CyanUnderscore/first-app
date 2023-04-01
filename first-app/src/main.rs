// first : creating a message handler and an enum for the different tupes of messages
// second : manage user input (zqsd)



use rand::{self, Rng};
use std::process::Command;
use std::process::exit;
use console::Term;

enum Message {
    Sucess,
    Error(i32), // return a Error code
    Warning(Box<str>), // send a Warning
    Message(Box<str>) // send a message to the user
} impl Message {
    fn message_handler(&self) { //will handle the messages
        match self {
            Message::Sucess => println!("Sucess"),
            Message::Error(error_code) => println!("Error Code : {} refer to the documentation for more information", error_code),
            Message::Warning(str) => println!("Warning : {}", str),
            Message::Message(str) => println!("System Message : {}", str)
        }
    }   
}

struct Player {
    life : u32,
    lenght : u32,
    color : (u32, u32, u32),
    location : (usize , usize)
}

struct Coin {
    location : (u32, u32)
}

fn simulate_loading() {
    let mut charge_bar : [char; 12] = ['[', '-', '-', '-', '-', '-', '-', '-', '-', '-', '-', ']'];
    Message::Message("loading the data ...".into()).message_handler(); 
    Message::Warning("watch out the current version is unstable".into()).message_handler();
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
    let mut child = Command::new("sleep").arg("0.2").spawn().unwrap();
    let _result = child.wait().unwrap();
    let random : f32 = rand::thread_rng().gen();
        if random > 0.9 {
        Message::Error(1).message_handler();
        return false
    } else {
        Message::Sucess.message_handler();
        return true
    }
}

fn init_game() -> ([[char; 10]; 10] , Player) {
    
    Message::Message("initialisation of the game ...".into()).message_handler();
    
    print!("loading screen ... : ");
    const rows: usize = 10;
    const lines: usize = 10;
    let screen = [['_';rows];lines];
    Message::Sucess.message_handler();
    println!("{:?}", screen);
    
    print!("loading player ... : ");
    let player = Player{
        life: 3,
        lenght: 2,
        color: (255, 0, 0),
        location: (5, 5)
    };
    Message::Sucess.message_handler();
    return (screen, player);
}

fn restart() {
    println!("restarting ... : ");
    Message::Sucess.message_handler();
    main();
}

fn main() {
    // first we will simulate a laoding
    simulate_loading();
    let (mut window, mut player) = init_game(); 
    let stdout = Term::buffered_stdout();

    'game_loop: loop {
        let mut child = Command::new("sleep").arg("0.1").spawn().unwrap();
        let _result = child.wait().unwrap();
        window[player.location.1][player.location.0] = '#';
        println!("{:?}", window);
        window[player.location.1][player.location.0] = '_';
        if let Ok(character) = stdout.read_char() {
            match character {
                'z' => player.location.1 -= 1,
                's' => player.location.1 += 1,
                'q' => player.location.0 -= 1,
                'd' => player.location.0 += 1,
                'r' => main(),
                _ => ()
            }
        }
    }
}

