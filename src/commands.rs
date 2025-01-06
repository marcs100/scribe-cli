use crate::config::ConfigFile;
use crate::console::{self, display_error, display_notebook_names, display_notes, pages_view};
use crate::scribe_database::{
    get_notebook_names, get_pinned_notes, get_recent_notes, opendb, write_note, NoteData, Notebook,
    NotebookCoverData,
};
use chrono::Local;
use std::string::String;

pub fn notebook_cmd(value: &str, conf: ConfigFile) {
    let conn = opendb(conf.database_file.as_str());
    let mut nb = Notebook::default();

    nb.get(&conn, value); //populate notebook struucture
    if nb.pages.is_none() {
        display_error("Notebook not found");
        return;
    }

    let pages = &nb.pages.unwrap();
    pages_view(&pages);
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

    //display_notes(notes);
    match notes {
        Some(pages) => pages_view(&pages),
        None => (),
    }

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
        //panic!("No note contents to write!");
        display_error("No note contents to write!");
        return;
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
        //panic!("No options currently supported for this command!");
        display_error("No options currently supported for this command");
        return;
    }

    if value.len() > 0 {
        display_error("No value allowed for this command!");
        return;
    }

    let conn = opendb(conf.database_file.as_str());
    let notes = get_pinned_notes(&conn);
    //display_notes(notes);

    match notes {
        Some(pages) => pages_view(&pages),
        None => (),
    }
}

pub fn list_cmd(option: &str, value: &str, conf: ConfigFile) {
    if option.len() > 0 {
        //panic!("No options currently supported for this command!");
        display_error("No options currently supported for this command");
        return;
    }

    if value.len() > 0 {
        //panic!("No value allowed for this command!")
        display_error("No value allowed for this command");
        return;
    }

    let conn = opendb(conf.database_file.as_str());
    let notebooks = get_notebook_names(&conn);
    //display_notes(notes);

    match notebooks {
        Some(notebook_names) => {
            display_notebook_names(&notebook_names);
            let result =
                console::get_user_input("\nEnter notebook number to browse or just enter to quit");
            match result {
                Ok(s) => {
                    let input_val = s.trim();
                    if input_val.len() == 0 {
                        println!("quitting");
                        return;
                    }
                    else {
                        //do we have a number?
                        let notebook_number = input_val.parse().unwrap_or_else(|_| 0);
                        if notebook_number == 0 {
                            display_error("non-numeric value entered");
                            return;
                        } else if notebook_number > notebook_names.len() {
                            display_error("notebook number is out of range");
                            return;
                        } else {
                            let notebook_name =
                                notebook_names[notebook_number - 1].notebook.as_str(); //populate selected notebook name

                            let mut nb = Notebook::default();

                            nb.get(&conn, notebook_name); //populate notebook struucture
                            if nb.pages.is_none() {
                                display_error("Notebook not found");
                                return;
                            }

                            let pages = &nb.pages.expect("error processing notebook pages");
                            pages_view(&pages);
                        }
                    }
                }
                Err(error) => panic!("Invalid input: {error:?}"),
            }
        }

        None => return,
    }
}
