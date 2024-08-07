use rusqlite::{Connection, Result, Statement};
//use rusqlite::NO_PARAMS;

#[derive(Debug)]
pub struct NoteData {
      pub id: i32,
      pub notebook: String,
      pub tag: String,
      pub content: String,
      pub created: String,
      pub modified: String,
      pub pinned: i32,
      pub back_colour: String,
}


pub fn opendb(database_file: &str) -> Connection{
      
      let conn : Connection;
      let connection_result = Connection::open(database_file.trim());
      
      match connection_result{
            Err(e) => panic!("can't open database {} {}", database_file, e),
            Ok(v) => conn = v,
      };
      
      conn
}


pub fn get_recent_notes(conn: &Connection, num_notes: u32) -> Option<Vec<NoteData>>{

      let mut stmt: Statement = conn.prepare("SELECT * from marcnotes order by modified desc LIMIT :limit;").unwrap();
      //let row_iter = stmt.query_map([], |row| {
      let row_iter = stmt.query_map(&[(":limit", num_notes.to_string().as_str())], |row| {
            Ok(NoteData{
                  id: row.get(0)?,
               notebook: row.get(1)?,
               tag: row.get(2)?,
               content: row.get(3)?,
               created: row.get(4)?,
               modified: row.get(5)?,
               pinned: row.get(6)?,
               back_colour: row.get(7)?
            })
      }).expect("get_recent_notes: error getting row");

      let mut notes: Vec<NoteData> =  vec![];

      for row_data in row_iter {
            let note_data = row_data.expect("get_recent_notes: error getting row data");
            //println!("{}",print_out);
            //println!("\n");
            notes.push(note_data);
      }

      if notes.len() == 0 {return None;}

      Some(notes)
}


pub fn write_note(conn: &Connection, note_details: NoteData)-> Result<usize>{

      let result = conn.execute(
            "insert into marcnotes (notebook, tag, content, created, modified, pinned, BGColour) values (?,?,?,?,?,?,?)",
                                (note_details.notebook,
                                 note_details.tag,
                                 note_details.content.replace("\\n","\n"),
                                 note_details.created,
                                 note_details.modified,
                                 note_details.pinned,
                                 note_details.back_colour));  //.expect("write_note: error with sqlite query");

                                 result
}