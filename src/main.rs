use rusqlite::{Connection};
use colored::Colorize;
use std::string::String;

mod config;
mod database;

static VERSION: &str = "0.001 dev";

fn main() {
    let command = std::env::args().nth(1).expect("no command given");
    //let command = std::env::args().nth(1);
    let arg1 = std::env::args().nth(2);
    let arg2 = std::env::args().nth(3);
    let mut conf = config::ConfigFile::default();
    
    println!("---------- Scribe cli {} -------------", VERSION);
    //println!("Command: {}", command);

    let mut user_option: String = String::new();
    let mut user_param: String = String::new();

    if arg1.clone().is_some_and(|val| val.starts_with("--") ){
        user_option.push_str(&arg1.unwrap());
    }
    else{
        //arg1 must a parameter - check we only have one parameter given
        if arg2.is_some(){
            panic!("Too omany parameters or bad option");
        }
    }

     if arg2.is_some(){
        user_param.push_str(&arg2.unwrap());
    }

    conf.get_config(); // this will read the scribe config and populate the struct with the values

    match command.as_str(){
        "recent" => {recent_notes_cmd(&user_option, &user_param, conf);}, 
        _ => {println!("No command!");}
    }
    
 
}

fn recent_notes_cmd(option: &str, param: &str, conf: config::ConfigFile){
    let conn = database::open(conf.database_file.as_str());

    let mut num_notes = conf.recent_notes_count;
    let got_option = false;
    let got_param = false;

    match option{
        "--count" => {
            if(param.len()>0){
                num_notes = param.parse().expect("bad parameter {}");
            }
            else{
                println!("expecting parameter for count!");
                return;
            }
        },
        _ => {conf.recent_notes_count;}
    }

    let notes = database::get_recent_notes(&conn, num_notes);

    match notes{
        Some(note_data) => {
            for note in note_data.iter(){
                println!("{}","<----------".cyan());
                println!("| From Notebook: {}  Created: {}  Modified: {}",note.notebook.green().bold(), &note.created[..16].green().bold(), &note.modified[..16].green().bold());
                println!("{}","-----------".cyan());
                println!("{}", note.content.trim());
                println!("{}","---------->".cyan());
                println!("\n\n");
            };
        },
        None => {println!("No recent notes returned");}
    }
}
