use crate::scribe_database::{write_note, get_recent_notes,opendb, NoteData};
use crate::config::ConfigFile;
use colored::Colorize;
use std::string::String;
use chrono::Local;


pub fn recent_notes_cmd(option: &str, param: &str, conf: ConfigFile){

    // ************* debug only *****************************
    //let mut s1: String = String::new();
    //s1.push_str(conf.database_file.as_str());
    //s1.push_str("_test");
    //let conn = database::open(s1.as_str()); // debug_only
    // ********** end debug only ****************************
    let conn = opendb(conf.database_file.as_str());
    let mut num_notes = conf.recent_notes_count;

    match option{
        "--count" | "-c" => {
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

    let notes = get_recent_notes(&conn, num_notes);

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
pub fn quick_note_cmd(option: &str, param: &str, conf: ConfigFile){
    let notebook: String  = conf.default_notebook;
    let note_content = String::from(param);
    let tag = String::from("None");
    let bg = conf.default_note_background;
    let conn = opendb(conf.database_file.as_str());

    //println!("option = {}", option); //debug on;y **********
    //println!("param = {}", param); //debug on;y **********

    if option.len() > 0{
        panic!("no options currently supported for this command!");
    }

    if param.len() == 0 {
        panic!("No note contents to write!");
    }

    let dt = Local::now();
    let date_time_cr: String = dt.to_string();
    let date_time_formatted = date_time_cr[..19].to_string();


    let note_details: NoteData = NoteData{
        id: 1,
        notebook: notebook,
        tag: tag,
        content: note_content,
        created: date_time_formatted.clone(),
        modified: date_time_formatted.clone(),
        pinned: 0,
        back_colour: bg,
    };

    write_note(&conn,note_details).expect("quick_note_cmd: error writing note");

    conn.close().expect("error closing db connection");
}
