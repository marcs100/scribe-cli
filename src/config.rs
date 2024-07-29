use std::fs;
//use std::fs::read_dir;
use std::path::PathBuf;
use std::path::Path;
use homedir::my_home;
use std::string::String;


#[derive(Default)]
pub struct ConfigFile{
    pub database_file: String,
    pub default_notebook: String,
    pub recent_notes_count: u32,
    pub backup_location: String,
    pub default_note_background: String,
}

impl ConfigFile{
    pub fn get_config(&mut self){
        let file_path = ConfigFile::get_config_file();
        let contents = ConfigFile::read_config_file(file_path);
        let lines = contents.split("\n");
        for line in lines{
            if line.starts_with("database ="){                
                self.database_file = ConfigFile::get_value_from_line(line).to_string();
            }
            if line.starts_with("recent notes count ="){                
                self.recent_notes_count = ConfigFile::get_value_from_line(line).to_string().trim().parse().unwrap();
            }
            if line.starts_with("default notebook ="){                
                self.default_notebook = ConfigFile::get_value_from_line(line).to_string();
            }
            if line.starts_with("backup location ="){                
                self.backup_location = ConfigFile::get_value_from_line(line).to_string();
            }
            if line.starts_with("default note bg ="){
                self.default_note_background = ConfigFile::get_value_from_line(line).to_string();
            }
        }

        //Self{database_file: db_file_path, default_notebook: def_notebook}
    } 

    fn get_value_from_line(line: &str) -> &str{
        let mut conf_line_parts = line.split("=");
        if conf_line_parts.clone().count() != 2{  //Note clone() to make acopy as count would consume conf_line_parts iterator
            panic!("Bad config file");
        }

        let value: &str = conf_line_parts.nth(1).unwrap(); // note .unwrap() returns value of 'Some' from Option type (Some and None).
        value
    }
    
    fn read_config_file(config_file: PathBuf) -> String{
        println!("Reading config file...");
        // As reading file line by line is not working, lets read it into a string
        let contents = fs::read_to_string(config_file).expect("Could not read config file (empty)");
        contents
    }

    fn get_config_file() -> PathBuf{
        //let home_dir: Result<Option<std::path::PathBuf>, homedir::GetHomeError> = my_home().expect("Could not get config file");
        let mut home_dir: PathBuf = my_home().unwrap().expect("Could not get config file");

        home_dir.push(Path::new(".config/scribe/scribe.config"));
        home_dir
    }
}

