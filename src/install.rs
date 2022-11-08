use std::fs;
use std::path::Path;

use crate::vars;
use crate::read;



pub fn install(){
    let curate_dir = Path::new(vars::CURATE_DIR);
    if curate_dir.exists(){
        println!("It looks like you've already installed Curate on your system. Do you want to reinstall?");
        let input = read::input();
        match input.to_lowercase().as_str(){
            "n"|"no" => {return;},
            _ => {}
        }
    }
    // install for real now
    fs::create_dir(vars::CURATE_DIR).expect("Failed to create {vars::CURATE_DIR}");

    fs::write(vars::SUBS_FILE, "0");
    

}
