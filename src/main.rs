use rusqlite::{Connection};
use colored::Colorize;

mod config;
mod database;


fn main() {
    //let command = std::env::args().nth(1).expect("no command given");
    let command = std::env::args().nth(1);
    let options = std::env::args().nth(2);
    let param = std::env::args().nth(3);
    let mut conf = config::ConfigFile::default();
    
    //println!("---------- Scribe cli 1.0 -------------");
    //println!("Command: {}", command);
    match options{
        Some(s) => println!("options: {}", s),
        None => println!("options <none>"),
    };

    match param{
        Some(s) => println!("params: {}", s),
        None => println!("params <none>"),
    }

    /*match command.as_str(){
        "recent" => {recent_notes_cmd(&command, options.as_ref(), param.as_ref(), &conf);}, 
        _ => {println!("No command!");}
    }*/
    
    conf.get_config(); // this will read the scribe config and populate the struct with the values

}
/*
fn recent_notes_cmd(command: &str, option: Option<&String>, parmam: Option<&String>, conf: &config::ConfigFile){
    let conn = database::open(conf.database_file.as_str());

    let notes = database::get_recent_notes(&conn, 4);

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
}*/