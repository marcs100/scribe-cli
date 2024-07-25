//extern crate rusqlite;

//use std::result::Result::*;
use rusqlite::{params, Connection, Result, Statement, Error};
//use rusqlite::NO_PARAMS;


pub fn open(database_file: &str) -> Connection{
      
      let conn : Connection;
      let connection_result = Connection::open(database_file.trim());
      
      match connection_result{
            Err(e) => panic!("can't open database {} {}", database_file, e),
            Ok(v) => conn = v,
      };
      
      conn
}

pub fn get_recent_notes(conn: &Connection, num_notes: u32)->Result<()>{

      let mut stmt: Statement = conn.prepare("SELECT * from marcnotes order by modified desc LIMIT 4").unwrap();
      let contents: String = String::new();

      #[derive(Debug)]
      struct NoteData {
            id: i32,
            notebook: String,
            tag: String,
            content: String,
            created: String,
            modified: String,
            pinned: i32,
            back_colour: String            
}
      
      let row_iter = stmt.query_map([], |row| {
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
        })?;
    
        for row_data in row_iter {
            println!("{:?}", row_data.unwrap().content);
            println!("\n");
        }

        Ok(())
    
}
