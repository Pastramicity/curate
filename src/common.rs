use std::io::stdin;
use std::process::Command;


pub fn input() -> String {
    let mut s = String::new();
    let res = stdin().read_line(&mut s);
    match res{
        Err(_) => {s = String::from("n");},
        _ => {}
    };
    s
}

pub fn cmd(command: &str){
    let command = command.to_string();
    let parts: Vec<&str> = command.split(' ').collect();
    let err_msg = format!("Could not run command: {}", command);

    Command::new(parts[0])
        .args(&parts[1..])
        .spawn()
        .expect(err_msg.as_str());
}
