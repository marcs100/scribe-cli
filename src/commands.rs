use crate::scribe_database::{
    write_note,
    get_recent_notes,
    opendb,
    get_pinned_notes,
    NoteData,
    Notebook
};

use crate::console_display::{display_error, display_notes};
use crate::config::ConfigFile;
use std::string::String;
use chrono::Local;


pub fn notebook_cmd(value: &str, conf: ConfigFile){
    let conn = opendb(conf.database_file.as_str());
    let mut nb = Notebook::default();

    nb.get(&conn, value); //populate notebook struucture

    //For now just display all pages
    //but in future we want to display one page at a time
    if nb.pages.is_some(){
         display_notes(nb.pages);
    }
    else {
        display_error("Notebook not found");
    }

}

pub fn recent_notes_cmd(option: &str, value: &str, conf: ConfigFile){

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
            if value.len()>0{
                num_notes = value.parse().expect("invalid option given");
            }
            else{
                display_error("expecting a value for count!");
                return;
            }
        },
        _ => {conf.recent_notes_count;}
    }

    let notes = get_recent_notes(&conn, num_notes);

    display_notes(notes);

    conn.close().expect("error closing db connection");
}

//writes one line of user input to the defualt note book
pub fn quick_note_cmd(option: &str, value: &str, conf: ConfigFile){
    let notebook: String  = conf.default_notebook;
    let note_content = String::from(value);
    let tag = String::from("None"); // this field is not used any more!
    let bg = conf.default_note_background;
    let conn = opendb(conf.database_file.as_str());

    let mut pin = 0;
    match option{
        "--pin"| "-p" => {pin = 1;}
     _=> {}
    }

    if value.len() == 0 {
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
        pinned: pin,
        back_colour: bg,
    };

    write_note(&conn,note_details).expect("quick_note_cmd: error writing note!");

    //Now lets show the note we have just created
    let notes = get_recent_notes(&conn,1);
    display_notes(notes);

    conn.close().expect("error closing db connection!");
}


pub fn pinned_notes_cmd(option: &str, value: &str, conf: ConfigFile){
    if option.len() > 0{panic!("No options currently supported for this command!");}

    let conn = opendb(conf.database_file.as_str());
    let notes = get_pinned_notes(&conn);
    display_notes(notes);


}

