use crate::scribe_database::{NoteData, NotebookCoverData};
use colored::Colorize;
use std::io;
use std::io::*;
use std::io::{stdin, stdout, Write};
use std::string::String;
use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

//function to display the notes vector to screen.
pub fn display_notes(notes: Option<Vec<NoteData>>) {
    match notes {
        Some(note_data) => {
            for note in note_data.iter() {
                display_note(note);
            }
        }
        None => {
            println!("No recent notes returned");
        }
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
pub fn display_note_raw(note: &NoteData) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut pinned_status = String::new();
    match note.pinned {
        0 => {
            pinned_status.push_str("No");
        }
        1 => {
            pinned_status.push_str("Yes");
        }
        _ => {
            panic!("Invalid pinned status!");
        }
    }
    write!(stdout, "{}", "<----------\r\n".cyan()).unwrap();
    write!(
        stdout,
        "| From Notebook: {}\r\n",
        note.notebook.green().bold()
    )
    .unwrap();
    write!(
        stdout,
        "| Pinned: {}  Created: {}  Modified: {}\r\n",
        pinned_status.green().bold(),
        &note.created[..16].green().bold(),
        &note.modified[..16].green().bold()
    )
    .unwrap();
    write!(stdout, "{}", "-----------\r\n".cyan()).unwrap();
    write!(stdout, "{}\n\r", note.content.replace("\n", "\n\r").trim()).unwrap();
    write!(stdout, "{}", "---------->\n\r".cyan()).unwrap();
    stdout.flush().unwrap();
}

//functiom to display a single note to screen
pub fn display_note(note: &NoteData) {
    let mut pinned_status = String::new();
    match note.pinned {
        0 => {
            pinned_status.push_str("No");
        }
        1 => {
            pinned_status.push_str("Yes");
        }
        _ => {
            panic!("Invalid pinned status!");
        }
    }
    println!("{}", "<----------".cyan());
    println!("| From Notebook: {}", note.notebook.green().bold());
    println!(
        "| Pinned: {}  Created: {}  Modified: {}",
        pinned_status.green().bold(),
        &note.created[..16].green().bold(),
        &note.modified[..16].green().bold()
    );
    println!("{}", "-----------".cyan());
    println!("{}", note.content.trim());
    println!("{}", "---------->".cyan());
}

//function to show notebook book pages one page at a time
//User can navogate with keyboard
pub fn pages_view(pages: &Vec<NoteData>) {
    let num_pages = pages.len() - 1;
    let mut current_page = 0; //using zero index for pages
    let stdin = stdin();
    let mut stdout_raw = stdout()
        .into_raw_mode()
        .expect("termion - error -  into_raw_mode"); //this messes up formatting is display_note()

    write!(stdout_raw, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    display_note_raw(&pages[current_page]);
    write!(stdout_raw, "{}", "n = next;  p = previous  q = quit").unwrap();
    stdout_raw.flush().unwrap();
    for c in stdin.keys() {
        //clearing the screen and going to top left corner
        //Process key presses
        match c.unwrap() {
            Key::Char('j') => {
                if current_page < num_pages {
                    current_page += 1;
                    write!(stdout_raw, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
                    display_note_raw(&pages[current_page]);
                    write!(stdout_raw, "{}", "j = next;  h = previous  q = quit").unwrap();
                }
            }
            Key::Char('h') => {
                if current_page > 0 {
                    current_page -= 1;
                    write!(stdout_raw, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
                    display_note_raw(&pages[current_page]);
                    write!(stdout_raw, "{}", "j = next;  h = previous  q = quit").unwrap();
                }
            }
            Key::Char('q') => {
                write!(stdout_raw, "\r\n").unwrap();
                break;
            }
            Key::Char('e') => {
                write!(stdout_raw, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
                write!(stdout_raw, "Edit is not supported yet (coming soon!)\r\n").unwrap();
                stdout_raw.flush().unwrap();
                break;
            }
            _ => (),
        }

        stdout_raw.flush().unwrap();
    }
}

pub fn get_user_input(msg: &str) -> Result<String> {
    println!("{msg}");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?; //.expect("error: unable to read user input");
                                        //println!("{}",input);
    Ok(input)
}

pub fn display_notebook_names(notebooks: &Vec<NotebookCoverData>) {
    if notebooks.len() > 0 {
        println!("{}", "--Notebooks--".green().bold());
    }

    let mut nb_index = 1;
    for notebook_name in notebooks.iter() {
        println!("   {}. {}", nb_index, notebook_name.notebook);
        nb_index += 1;
    }
}

pub fn display_error(msg: &str) {
    println!("{}: {}", "Error".red(), msg.cyan());
}

pub fn display_warning(msg: &str) {
    println!("{}: {}", "Warning".blue(), msg.cyan());
}

pub fn display_help() {
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
    println!("    list - Displays a list of available notebook names");
    println!("         option : <None>");
}
