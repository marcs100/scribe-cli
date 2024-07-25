use rusqlite::{Connection};

mod config;
mod database;


fn main() {
    let command = std::env::args().nth(1).expect("no command given");
    let options = std::env::args().nth(2);
    let param = std::env::args().nth(3);
    let mut conf = config::ConfigFile::default();
    
    println!("---------- Scribe cli 1.0 -------------");
    println!("Command: {}", command);
    match options{
        Some(s) => println!("options: {}", s),
        None => println!("options <none>"),
    };

    match param{
        Some(s) => println!("params: {}", s),
        None => println!("params <none>"),
    };
    
    conf.get_config(); // this will read the scribe config and populate the struct with the values

    let conn = database::open(conf.database_file.as_str());

    database::get_recent_notes(&conn, 4);



    //println!("command: {:?}  options: {:?} ",command,options);    
    
    
    //println!("database file: {}", conf.database_file);
    //println!("default notebook: {}", conf.default_notebook);


    
    
    //testing ***********************
    /*
    let mut params = vec![
        "database", 
        "recent notes count", 
        "default notebook", 
        "screen scale", 
        "backup location"
    ];
    
    let params_size = params.len();
    println!("params size is {}", params_size);
    if params.contains(&"database") {
        println!("list contains database!");
        println!("removing database from params");
        let index = params.iter().position(|param| *param == "database" );
        if index.is_some(){
            println!("removing database from list");
            params.remove(index.unwrap());
            if params.contains(&"database") == false {
                println!("remove successful");
            }
            else {
                println!("remove did not work!");
            }
        }
        else{
            println!("Error index not found");                        
        }
        
    }*/


    // end of testing****************

   /*
   //Note I cannot get this to work, even though the code looks correct
   //   I was using the following imports
   //       use std::io::{BufReader, BufRead};
   //       use std::fs::File;

   let file = File::open(config_file)?;
   let reader = BufReader::new(file);

   for line in reader.lines{
        println("{}", line);
   } */

   

}


/*fn change_string(s1: &mut String)
{
    s1.push_str(" cruel world!");
}*/

