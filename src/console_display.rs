use colored::Colorize;
use crate::scribe_database::NoteData;

//function to display the notes vector to screen.
pub fn display_notes(notes: Option<Vec<NoteData>>){
    match notes{
        Some(note_data) => {
            for note in note_data.iter(){
                let mut pinned_status = String::new();
                match note.pinned{
                    0 => {pinned_status.push_str("No");}
                    1 => {pinned_status.push_str("Yes");}
                    _=> {panic!("Invalid pinned status!");}
                }
                println!("{}","<----------".cyan());
                println!("| From Notebook: {}",note.notebook.green().bold());
                println!("| Pinned: {}  Created: {}  Modified: {}",pinned_status.green().bold(), &note.created[..16].green().bold(), &note.modified[..16].green().bold());
                println!("{}","-----------".cyan());
                println!("{}", note.content.trim());
                println!("{}","---------->".cyan());
            };
        },
        None => {println!("No recent notes returned");}
    }
}

pub fn display_error(msg: &str){
    println!("{}: {}","Error".red(), msg.cyan());
}

pub fn display_warning(msg: &str){
    println!("{}: {}","Warning".blue(), msg.cyan());
}

pub fn display_help(){
    println!("scribe-cli <command> <options>");
    println!("commands:");
    println!("    recent - Displays recent notes (number of notes to display is in scribe.config)");
    println!("         option : [--count -c] number of recent notes to display (overrides scribe.config)");
    println!("    quick <content> - Write a quick note (incase note in quotes)");
    println!("         option : [--pin -p] pin the note");
    println!("    pinned - Display all pinned notes");
    println!("         option : <None>");
    println!("    notebook <notebook name> - Display an entire notbbook");
    println!("         option : <None>");
}

