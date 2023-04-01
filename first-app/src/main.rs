// first : creating a message handler and an enum for the different tupes of messages
// second : manage user input (arrows)


#[macro_use]
extern crate crossterm;

use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

use std::io::{stdout, Write};
use rand::{self, Rng};
use std::process::Command;

enum Input {
    Up,
    Down,
    Right,
    Left
} impl Input {
    fn input_handler(&self){
        match self {
            Input::Up => println!("Up"), 
            Input::Down => println!("Down"),
            Input::Right => println!("Right"),
            Input::Left => println!("Left")

        }
    }
}


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
    location : (u32 , u32)
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
    let screen = [['_';10]; 10];
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

fn input_manager() {
    let mut stdout = stdout();
    //going into raw mode
    enable_raw_mode().unwrap();

    //clearing the screen, going to top left corner and printing welcoming message
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0), Print(r#"ctrl + q to exit, ctrl + h to print "Hello world", alt + t to print "crossterm is cool""#))
            .unwrap();

    //key detection
    loop {
        //going to top left corner
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();

        //matching the key
        match read().unwrap() {
            //i think this speaks for itself
            Event::Key(KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: KeyModifiers::CONTROL,
                //clearing the screen and printing our message
            }) => execute!(stdout, Clear(ClearType::All), Print("Hello world!")).unwrap(),
            Event::Key(KeyEvent {
                code: KeyCode::Char('t'),
                modifiers: KeyModifiers::ALT,
            }) => execute!(stdout, Clear(ClearType::All), Print("crossterm is cool")).unwrap(),
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            }) => break,
            _ => (),
        }
    }

    //disabling raw mode
    disable_raw_mode().unwrap();
}

fn main() {
    // first we will simulate a laoding
    simulate_loading();
    let (window, player) = init_game(); 

}
