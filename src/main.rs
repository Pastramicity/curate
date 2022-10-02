use std::env;
use std::fs;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::sync::broadcast;

const PORT: u32 = 888;
const CURATE_FILE: &str = "curate.md";
const SUBS_FILE: &str = "subs.txt";
const MAX_SEND_RETRIES: u64 = 100;
const MAX_SERVER_CONNECTIONS: usize = 100;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        println!("Please specify either server or client as an argument to run the program.");
        return;
    }

    match &args[0][..] {
        "server" => {
            server().await;
        }
        "client" => {
            client().await;
        }
        _ => {
            println!("Please specify either server or client as an argument to run the program");
            return;
        }
    }
}

async fn server() {
    // get num subs
    let mut subs: u64;
    let subs_file_res = fs::read_to_string("subs.txt");
    match subs_file_res {
        Ok(num_str) => match num_str.parse::<u64>() {
            Ok(num) => {
                subs = num;
            }
            Err(_) => {
                println!("Make sure you have a positive integer in your subs file");
                subs = 0;
            }
        },
        Err(_) => {
            println!("Couldn't read subs.txt, creating now. This file should contain a number of how many subscribers you have so you can track it");
            subs = 0;
            match fs::write("subs.txt", format!("{}", subs)) {
                Ok(_) => {}
                Err(_) => {
                    println!("Failed to write to subs.txt");
                }
            }
        }
    }

    // get curation
    let curation: String;
    let curation_file_res = fs::read_to_string(CURATE_FILE);
    match curation_file_res {
        Ok(msg) => match msg.as_str() {
            "" => {
                println!(
                    "Please enter at least one curation in {} to run your server.",
                    CURATE_FILE
                );
                return;
            }
            msg => {
                curation = msg.to_string();
            }
        },
        Err(_) => {
            println!(
                "To start curating, place a curation entry into the {} file.",
                CURATE_FILE
            );
            match fs::write(CURATE_FILE, "") {
                Ok(_) => {}
                Err(_) => {
                    println!("Failed to create {} file, do it yourself.", CURATE_FILE);
                }
            }
            return;
        }
    }

    // run actual server code
    let this_addr = format!("localhost:{}", PORT);
    let listener = TcpListener::bind(this_addr)
        .await
        .expect(format!("Couldn't set up network listener on port {}", PORT).as_str());
    let (tx, rx) = broadcast::channel::<String>(MAX_SERVER_CONNECTIONS);
    loop {
        let curation = curation.clone();
        let (mut socket, addr);
        let listener_res = listener.accept().await;
        match listener_res {
            Ok((s, a)) => {
                socket = s;
                addr = a;
            }
            Err(_) => {
                println!("Failed to accept connection.");
                continue;
            }
        }
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            // split socket

            let (reader, mut writer) = socket.split();
            let result = match rx.recv().await {
                Ok(res) => res,
                Err(_) => {
                    println!("Failed to receive message.");
                    return;
                }
            };

            // check which command is being received
            let response: String = match result.as_str() {
                "all" => curation,
                "latest" => match latest_curation(&curation) {
                    Ok(c) => c,
                    Err(_) => String::from("Failed to find latest curation"),
                },
                "sub" => {
                    // received a new subscriber
                    subs += 1;
                    match fs::write(SUBS_FILE, subs.to_string()) {
                        Ok(_) => String::from("OK"),
                        Err(_) => {
                            println!("Failed to update {}", SUBS_FILE);
                            String::from("Failed to update sub count")
                        }
                    }
                }
                "unsub" => {
                    // lost a subscriber
                    subs -= 1;
                    match fs::write(SUBS_FILE, subs.to_string()) {
                        Ok(_) => String::from("OK"),
                        Err(_) => {
                            println!("Failed to update {}", SUBS_FILE);
                            String::from("Failed to update sub count")
                        }
                    }
                }
                _ => {
                    // inform requester this is an invalid command
                    String::from("This is not a valid command")
                }
            };

            let mut retries = 0;
            let response_bytes = response.as_bytes();
            while retries < MAX_SEND_RETRIES {
                if let Err(_) = writer.write_all(response.as_bytes()).await {
                    println!("Failed to send response, trying again");
                    retries += 1;
                } else {
                    break;
                }
            }
        });
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

async fn client() {}
