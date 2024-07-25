use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

pub fn open(&str database){
      let conn = Connection::open("cats.db")?;

}
