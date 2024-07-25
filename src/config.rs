use std::fs;
//use std::fs::read_dir;
use std::path::PathBuf;
use std::path::Path;
use homedir::my_home;
use std::string::String;
use core::str::Split;

#[derive(Default)]
pub struct ConfigFile{
    pub database_file: String,
    pub default_notebook: String
}

impl ConfigFile{
    pub fn get_config(&mut self){
        let file_path = ConfigFile::get_config_file();
        let contents = ConfigFile::read_config_file(file_path);
        let lines = contents.split("\n");
        let mut conf_line_parts: Split<&str>;
        let mut counter = 0;
        for line in lines{
            println!("{}", line);
            if line.starts_with("database ="){
                println!("Found database line");
                conf_line_parts = line.split("=");
                if conf_line_parts.clone().count() != 2{  //Note clone() to make acopy as count would consume conf_line_parts iterator
                    panic!("Bad config file");
                }
                self.database_file = conf_line_parts.nth(1).unwrap().to_string(); // note .unwrap() returns value of 'Some' from Option type (Some and None).
                counter += 1;
                if counter > 1 {break;}
            }
            else if line.starts_with("default notebook = "){
                println!("Found default notebook");
                conf_line_parts = line.split("=");
                if conf_line_parts.clone().count() != 2{  //Note clone() to make acopy as count would consume conf_line_parts iterator
                    panic!("Bad config file");
                }
                self.default_notebook = conf_line_parts.nth(1).unwrap().to_string();
                counter += 1;
                if counter > 1{break;}
            }

            
        }
        //Self{database_file: db_file_path, default_notebook: def_notebook}
    } 

    fn read_config_file(config_file: PathBuf) -> String{
        println!("Reading config file...");
        // As reading file line by line is not working, lets read it into a string
        let contents = fs::read_to_string(config_file).expect("Could not read config file (empty)");
        contents
    }

    fn get_config_file() -> PathBuf{
        let home_dir_result: Result<Option<std::path::PathBuf>, homedir::GetHomeError> = my_home();
        //This is convoluted there must be a better way!!
        let mut home_dir = match home_dir_result {
            Ok(option) => match option{
                Some(pb) => pb,
                None => panic!("Could not find scribe configutaion file!")//PathBuf::new()
            },
            Err(..) => panic!("Could not get scribe configuation file!")//PathBuf::new()
        };
        home_dir.push(Path::new(".config/scribe/scribe.config"));
        home_dir
    }
}

