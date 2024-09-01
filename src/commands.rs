use crate::scribe_database::{
    get_pinned_notes, get_recent_notes, opendb, write_note, NoteData, Notebook,
};
use crate::config::ConfigFile;
use crate::console::{display_error, display_notes, display_note};
use chrono::Local;
use std::io::Read;
use std::string::String;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;


pub fn notebook_cmd(value: &str, conf: ConfigFile) {
    let conn = opendb(conf.database_file.as_str());
    let mut nb = Notebook::default();

    nb.get(&conn, value); //populate notebook struucture
    if nb.pages.is_none() {
        display_error("Notebbok not found");
        return;
    }


    let pages = &nb.pages.unwrap();
    // **** debug only ********
    //display_note(&pages[0]);
    //return;
    //*************************
    let num_pages = pages.len()-1;
    let mut current_page = 0; //using zero index for pages
    let mut stdin = std::io::stdin();
    let mut stdout = stdout().into_raw_mode().unwrap(); //this messes up formatting is display_note()
    display_note(&pages[current_page]);
    for c in stdin.keys() {
        //clearing the screen and going to top left corner
        /*write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),s
               termion::clear::All
        )
        .unwrap();*/

        //Process key presses
        match c.unwrap() {
            Key::Char('n') => {
                if current_page < num_pages{
                    current_page += 1;
                    let cont: &str = pages[current_page].content.as_str();
                    display_note(&pages[current_page]);
                }

            },
            Key::Char('p') => {
                if current_page > 0{
                    current_page -= 1;
                    display_note(&pages[current_page]);
                }
            },
            Key::Char('q') => break,
            Key::Char('e') => println!("Edit is not supported yet (coming soon!)"),
            _ => (),
        }

        stdout.flush().unwrap();
    }
}

pub fn recent_notes_cmd(option: &str, value: &str, conf: ConfigFile) {
    // ************* debug only *****************************
    //let mut s1: String = String::new();
    //s1.push_str(conf.database_file.as_str());
    //s1.push_str("_test");
    //let conn = database::open(s1.as_str()); // debug_only
    // ********** end debug only ****************************
    let conn = opendb(conf.database_file.as_str());
    let mut num_notes = conf.recent_notes_count;

    match option {
        "--count" | "-c" => {
            if value.len() > 0 {
                num_notes = value.parse().expect("invalid option given");
            } else {
                display_error("expecting a value for count!");
                return;
            }
        }
        _ => {
            conf.recent_notes_count;
        }
    }

    let notes = get_recent_notes(&conn, num_notes);

    display_notes(notes);

    conn.close().expect("error closing db connection");
}

//writes one line of user input to the defualt note book
pub fn quick_note_cmd(option: &str, value: &str, conf: ConfigFile) {
    let notebook: String = conf.default_notebook;
    let note_content = String::from(value);
    let tag = String::from("None"); // this field is not used any more!
    let bg = conf.default_note_background;
    let conn = opendb(conf.database_file.as_str());

    let mut pin = 0;
    match option {
        "--pin" | "-p" => {
            pin = 1;
        }
        _ => {}
    }

    if value.len() == 0 {
        panic!("No note contents to write!");
    }

    let dt = Local::now();
    let date_time_cr: String = dt.to_string();
    let date_time_formatted = date_time_cr[..19].to_string();

    let note_details: NoteData = NoteData {
        id: 1,
        notebook: notebook,
        tag: tag,
        content: note_content,
        created: date_time_formatted.clone(),
        modified: date_time_formatted.clone(),
        pinned: pin,
        back_colour: bg,
    };

    write_note(&conn, note_details).expect("quick_note_cmd: error writing note!");

    //Now lets show the note we have just created
    let notes = get_recent_notes(&conn, 1);
    display_notes(notes);

    conn.close().expect("error closing db connection!");
}

pub fn pinned_notes_cmd(option: &str, value: &str, conf: ConfigFile) {
    if option.len() > 0 {
        panic!("No options currently supported for this command!");
    }

    let conn = opendb(conf.database_file.as_str());
    let notes = get_pinned_notes(&conn);
    display_notes(notes);
}
