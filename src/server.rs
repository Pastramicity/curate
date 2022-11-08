use std::fs;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::sync::broadcast;

use crate::vars;
use crate::common;

pub async fn server() {
    // get num subs
    let mut subs: u64;
    let subs_file_res = fs::read_to_string(vars::SUBS_FILE);
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
            println!("Couldn't read $HOME/.config/curate/subs.txt, creating it now. This file should contain a number of how many subscribers you have so you can track it. This program will modify your subs count as needed.");
            subs = 0;
            match fs::write(vars::SUBS_FILE, format!("{}", subs)) {
                Ok(_) => {}
                Err(_) => {
                    println!("Failed to write to $HOME/.config/curate/subs.txt");
                }
            }
        }
    }

    // get curation
    let curation: String;
    let curation_file_res = fs::read_to_string(vars::CURATE_FILE);
    match curation_file_res {
        Ok(msg) => match msg.as_str() {
            "" => {
                println!(
                    "Please enter at least one curation in {} to run your server.",
                    vars::CURATE_FILE
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
                vars::CURATE_FILE
            );
            match fs::write(vars::CURATE_FILE, "") {
                Ok(_) => {}
                Err(_) => {
                    println!("Failed to create {} file, do it yourself.", vars::CURATE_FILE);
                }
            }
            return;
        }
    }

    // run actual server code
    let this_addr = format!("localhost:{}", vars::PORT);
    let listener = TcpListener::bind(this_addr)
        .await
        .expect(format!("Couldn't set up network listener on port {}", vars::PORT).as_str());
    let (tx, rx) = broadcast::channel::<String>(vars::MAX_SERVER_CONNECTIONS);
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
                "latest" => {todo!();}
                "sub" => {
                    // received a new subscriber
                    subs += 1;
                    match fs::write(vars::SUBS_FILE, subs.to_string()) {
                        Ok(_) => String::from("OK"),
                        Err(_) => {
                            println!("Failed to update {}", vars::SUBS_FILE);
                            String::from("Failed to update sub count")
                        }
                    }
                }
                "unsub" => {
                    // lost a subscriber
                    subs -= 1;
                    match fs::write(vars::SUBS_FILE, subs.to_string()) {
                        Ok(_) => String::from("OK"),
                        Err(_) => {
                            println!("Failed to update {}", vars::SUBS_FILE);
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
            while retries < vars::MAX_SEND_RETRIES {
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
