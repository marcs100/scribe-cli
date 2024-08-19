use rusqlite::{Connection, Result, Statement};
//use rusqlite::NO_PARAMS;

#[derive(Default)]
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

#[derive(Default)]
pub struct Notebook{
      pub name: String,
      pub colour: String,
      pub number_of_pages: usize,
      pub current_page: i32,
      pub pages: Option<Vec<NoteData>>,
}

impl Notebook{
      pub fn get(&mut self, conn: &Connection, notebook_name: &str){
            //let conn: Connection  =  opendb("scribe");  //debug only ****************
            //let notebook_name = "General";  //debug only ****************
            self.name = String::from(notebook_name);
            self.pages = Notebook::get_notebook(&conn, notebook_name);
            match &self.pages {
                  Some(p) => {
                        self.number_of_pages = p.len();
                        //now we know notebook exists, get the colour fron the notebook covers table
                        self.colour = String::from("#FFFFFF"); //to do !!!!!!!!
                        self.current_page = 1;
                  },
                  None => {
                        self.number_of_pages = 0;
                        self.colour = String::new();
                        self.current_page=0;
                  },
            };
      }

      fn get_notebook(conn: &Connection, name: &str) -> Option<Vec<NoteData>>{
            let mut stmt: Statement = conn.prepare("SELECT * from marcnotes where notebook = :nb").unwrap();
            let row_iter = stmt.query_map(&[(":nb", name)], |row| {
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
            }).expect("get_notebook: error getting row");

            let mut notes: Vec<NoteData> =  vec![];

            for row_data in row_iter {
                  let note_data = row_data.expect("get_notebook: error getting row data");
                  notes.push(note_data);
            }

            if notes.len() == 0 {return None;}

            Some(notes)
      }

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


pub fn get_pinned_notes(conn: &Connection) -> Option<Vec<NoteData>>{
      let mut stmt: Statement = conn.prepare("SELECT * from marcnotes where pinned is 1 order by modified desc").unwrap();
      let row_iter = stmt.query_map([], |row|{
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
      }).expect("get_pinned_notes: error getting row");

      let mut notes: Vec<NoteData> =  vec![];

      for row_data in row_iter {
            let note_data = row_data.expect("get_pinned_notes: error getting row data");
            notes.push(note_data);
      }

      if notes.len() == 0 {return None;}

      Some(notes)
}
