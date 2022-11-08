use std::env;
use std::fs;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::sync::broadcast;

mod vars;
mod read;
mod client;
mod server;
mod host;
mod install;

#[tokio::main]
async fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.len() == 0 {
        println!("Please specify either server or client as an argument to run the program.");
        return;
    }

    match &args[0][..] {
        "server" => {
            server::server().await;
        }
        "client" => {
            client::client().await;
        }
        "install" => {

        }
        "-h" | "--help" => {
            println!("curate [server|host|client]");
        }
        _ => {
            println!("Please specify either server or client as an argument to run the program");
            return;
        }
    }
}



fn latest_curation(curation: &String) -> Result<String, &str> {
    // push line nums of where new entries are to vec
    let lines = curation.lines();
    let mut entry_idx = Vec::new();
    for (i, line) in lines.enumerate() {
        if &line[..2] == "# " {
            // is top level heading
            entry_idx.push(i);
        }
    }

    // err if no entries
    if entry_idx.len() == 0 {
        return Err("Couldn't split the curation into entries.");
    }

    // get all lines after the last entry mark
    let lines: Vec<_> = curation.lines().collect();
    let lines = &lines[*entry_idx.last().unwrap()..];
    let mut entry = String::new();

    // smush lines into entry
    for line in lines {
        entry.push_str(line);
        entry.push('\n');
    }

    // return
    Ok(entry)
}

