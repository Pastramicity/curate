use dirs::home_dir;

pub const PORT: u32 = 888;
pub const MAX_SEND_RETRIES: u64 = 100;
pub const MAX_SERVER_CONNECTIONS: usize = 100;
pub const CURATE_TEMPLATE_URL: &str = "raw.github.com/pastramicity/curate/master/curate_template.md";
pub const CURATE_DIR: &str = "$HOME/.config/curate";
pub const CURATE_FILE: &str = "$HOME/.config/curate/curate.md";
pub const SUBS_FILE: &str = "$HOME/.config/curate/subs.txt";



pub fn replace_home(path: &str) -> String{
    let home = home_dir();
    let home = home.expect("No home directory?");
    let home = home.to_str().expect("No home directory?");
    path.to_string().replace("$HOME", home)
}


pub fn curate_dir() -> String{
    replace_home(CURATE_DIR)
}

pub fn curate_file() -> String{
    replace_home(CURATE_FILE)
}

pub fn subs_file() -> String{
    replace_home(SUBS_FILE)
}
