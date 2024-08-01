mod config;
mod commands;
mod scribe_database;

use crate::commands::{quick_note_cmd,recent_notes_cmd};

//use std::env;  //currently only being used for rust baccktrace


static VERSION: &str = "0.001 dev";

fn main() {
    let command = std::env::args().nth(1).expect("no command given");
    //let command = std::env::args().nth(1);
    let arg1 = std::env::args().nth(2);
    let arg2 = std::env::args().nth(3);
    let mut conf = config::ConfigFile::default();

    //env::set_var("RUST_BACKTRACE", "1"); //this should only be in the dubug version
    
    println!("---------- Scribe cli {} -------------", VERSION);

    let mut user_option: String = String::new();
    let mut user_param: String = String::new();
    let mut got_param = false;

    match arg1{
        Some(s) => {
            if s.starts_with("--"){ //options should always start with -- else it wil be considered a parameter
                user_option.push_str(&s);
            }
            else{
                if arg2.is_some(){
                    panic!("Too many parameters or bad option!"); //can't have 2 parameters and no option given'
                }
                user_param.push_str(&s); //arg1 is a parameter not an option
                got_param = true;
            }
        },
        None => ()
    }

    if arg2.is_some() && !got_param{
        user_param.push_str(&arg2.unwrap());
    }


    conf.get_config(); // this will read the scribe config and populate the struct with the values

    match command.as_str(){
        "recent" => {recent_notes_cmd(&user_option, &user_param, conf);}, 
        "quick" => {quick_note_cmd(&user_option, &user_param, conf);},
         _ => {println!("No command!");},
    }
}


