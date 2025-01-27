mod commands;
mod config;
mod console;
mod scribe_database;

use console::{display_error, display_help, display_warning};

use crate::commands::{list_cmd, notebook_cmd, pinned_notes_cmd, quick_note_cmd, recent_notes_cmd};

//use std::env;  //currently only being used for rust baccktrace

static VERSION: &str = "0.1.1";

fn main() {
    let command = std::env::args().nth(1).expect("no command given");

    //postfix match is experimental - won't compile'
    /*let command = std::env::args().nth(1).match {
        Some(s) => s,
        None => {display_error("Missing command"); return}
    };*/

    //let command = std::env::args().nth(1);
    let arg1 = std::env::args().nth(2);
    let arg2 = std::env::args().nth(3);
    let mut conf = config::ConfigFile::default();
    let valid_short_options = ['c', 'p', 'l']; //room to add more in future!!!!!!!!
    let valid_long_options = ["--count", "--pin", "--list"]; //room for more in future!!!

    //env::set_var("RUST_BACKTRACE", "1"); //this should only be in the dubug version

    println!("---------- Scribe cli {} -------------", VERSION);

    let mut user_option: String = String::new();
    let mut user_value: String = String::new();
    let mut got_value = false;

    //check command is not help request
    if command == "-h" || command == "--help" {
        display_help();
        return;
    }

    match arg1 {
        //if arg1 is not an 'option' then it will be considered a value
        Some(s) => {
            if s.len() == 2 {
                if s.starts_with("-") && valid_short_options.contains(&s.chars().nth(1).unwrap()) {
                    //need to get 2nd char from String!!!!!!!!!!!!!!!
                    user_option.push_str(&s);
                } else {
                    display_error("invalid option given");
                    panic!();
                }
            } else if valid_long_options.contains(&s.as_str()) {
                user_option.push_str(&s);
            } else {
                if arg2.is_some() {
                    display_error("Too many values or bad option!");
                    panic!(); //can't have 2 values and no option given'
                }
                user_value.push_str(&s); //arg1 is a valueeter not an option
                got_value = true;
            }
        }
        None => (),
    }

    if arg2.is_some() && !got_value {
        user_value.push_str(&arg2.unwrap());
    }

    conf.get_config(); // this will read the scribe config and populate the struct with the values

    match command.as_str() {
        "recent" => {
            recent_notes_cmd(&user_option, &user_value, &conf);
        }
        "note" => {
            quick_note_cmd(&user_option, &user_value, &conf);
        }
        "pinned" => {
            pinned_notes_cmd(&user_option, &user_value, &conf);
        }
        "notebook" => {
            notebook_cmd(&user_value, &conf);
        }
        "list" => {
            list_cmd(&user_value, &user_value, &conf);
        }
        _ => {
            display_warning("No command matched!");
        }
    }
}
