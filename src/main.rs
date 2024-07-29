mod config;
mod scribe_database;

use rusqlite::{Connection};
use colored::Colorize;
use std::string::String;
use chrono::{DateTime, Local};
use scribe_database::NoteData;


static VERSION: &str = "0.001 dev";

fn main() {
    let command = std::env::args().nth(1).expect("no command given");
    //let command = std::env::args().nth(1);
    let arg1 = std::env::args().nth(2);
    let arg2 = std::env::args().nth(3);
    let mut conf = config::ConfigFile::default();
    
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


fn recent_notes_cmd(option: &str, param: &str, conf: config::ConfigFile){

    // ************* debug only *****************************
    //let mut s1: String = String::new();
    //s1.push_str(conf.database_file.as_str());
    //s1.push_str("_test");
    //let conn = database::open(s1.as_str()); // debug_only
    // ********** end debug only ****************************
    let conn = scribe_database::open(conf.database_file.as_str());
    let mut num_notes = conf.recent_notes_count;

    match option{
        "--count" => {
            if param.len()>0{
                num_notes = param.parse().expect("bad parameter {}");
            }
            else{
                println!("expecting parameter for count!");
                return;
            }
        },
        _ => {conf.recent_notes_count;}
    }

    let notes = scribe_database::get_recent_notes(&conn, num_notes);

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

    conn.close().expect("error closing db connection");
}

//writes one line of user input to the defualt note book
fn quick_note_cmd(option: &str, param: &str, conf: config::ConfigFile){
    let notebook: String  = conf.default_notebook;
    let note_content = String::from(param);
    let tag = String::new();
    let bg = conf.default_note_background;
    let conn = scribe_database::open(conf.database_file.as_str());

    println!("option = {}", option); //debug on;y **********
    println!("param = {}", param); //debug on;y **********

    if option.len() > 0{
        panic!("no options currently supported for this command!");
    }

    if param.len() == 0 {
        panic!("No note contents to write!");
    }

    let dt = Local::now();
    let date_time_cr: String = dt.to_string();
    let date_time_mod = date_time_cr.clone();


    let note_details: NoteData = NoteData{
        id: 1,
        notebook: notebook,
        tag: tag,
        content: note_content,
        created: date_time_cr,
        modified: date_time_mod,
        pinned: 0,
        back_colour: bg,
    };

    scribe_database::write_note(&conn,note_details).expect("quick_note_cmd: error writing note");

    conn.close().expect("error closing db connection");
}
