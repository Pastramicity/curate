use std::fs;
use std::path::Path;

use crate::vars;
use crate::common;



pub fn install_server(){
    let curate_dir = Path::new(vars::CURATE_DIR);
    let dir_exists = curate_dir.exists();
    if dir_exists{
        println!("It looks like you've already installed Curate on your system. Do you want to reinstall?");
        let input = common::input();
        match input.to_lowercase().as_str(){
            "n"|"no" => {return;},
            _ => {}
        }
    }
    // install for real now
    if !dir_exists {
        let cmd = format!("mkdir {}", vars::curate_dir());
        common::cmd(cmd.as_str());
    }

    if !Path::new(vars::SUBS_FILE).exists(){
        fs::write(vars::subs_file(), "0").expect("Couldn't write subs file");
    }

    if !Path::new(vars::CURATE_FILE).exists(){
        let cmd = format!("wget {} -o /dev/null -O {}", vars::CURATE_TEMPLATE_URL, vars::curate_file());
        common::cmd(cmd.as_str());

    }

    println!("Ready to use curate server!");
}
