use colored::Colorize;
use crate::scribe_database::NoteData;
use std::io::{stdin, stdout, Write};
//use termion::event::Key;
//use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

//function to display the notes vector to screen.
pub fn display_notes(notes: Option<Vec<NoteData>>){
    match notes{
        Some(note_data) => {
            for note in note_data.iter(){
               display_note(note);
            };
        },
        None => {println!("No recent notes returned");}
    }
}


//functiom to display a single note to screen in raw tty mode
/*
 Notes on raw mode (termion):

 1) It disables the line b*uffering: As you might notice, your command-line application tends to behave like the command-line. The programs will first get the input when the user types            \n. Raw mode makes the program get the input after every key stroke.

 2) It disables displaying the input: Without raw mode, the things you type appear on the screen, making it insufficient for most interactive TTY applications, where keys can represent controls and not textual input.

 3) It disables canonicalization of the output: For example, \n represents “go one cell down” not “break the line”, for line breaks \n\r is needed.

 4) It disables scrolling.
*/
pub fn display_note_raw(note: &NoteData, ){
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut pinned_status = String::new();
    match note.pinned{
        0 => {pinned_status.push_str("No");}
        1 => {pinned_status.push_str("Yes");}
        _=> {panic!("Invalid pinned status!");}
    }
    write!(stdout,"{}","<----------\r\n".cyan()).unwrap();
    write!(stdout,"| From Notebook: {}\r\n",note.notebook.green().bold()).unwrap();
    write!(stdout,"| Pinned: {}  Created: {}  Modified: {}\r\n",pinned_status.green().bold(), &note.created[..16].green().bold(), &note.modified[..16].green().bold()).unwrap();
    write!(stdout,"{}","-----------\r\n".cyan()).unwrap();
    write!(stdout,"{}\n\r", note.content.replace("\n","\n\r").trim()).unwrap();
    write!(stdout,"{}","---------->\n\r".cyan()).unwrap();
    stdout.flush().unwrap();
}

//functiom to display a single note to screen
pub fn display_note(note: &NoteData){
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

