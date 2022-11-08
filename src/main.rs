use std::env;

mod vars;
mod common;
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
            if args.len() != 1{
                panic!("Too many arguments. Try just running with flag 'server'")
            }
            server::server().await;
        }
        "client" => {
            client::client().await;
        }
        "install" => {
            if args.len() != 2{
                panic!("Too many arguments. Try running with 'server -h', or 'server --help' if you're having trouble.");
            }
            match &args[1][..]{
                "server" => {install::install_server::install_server();}
                "client" => {todo!();}
                "host" => {todo!();}
                "-h" | "--help" => {todo!();}
                _ => {panic!("Sorry, don't know that command. Try running with 'server -h', or 'server --help' if you're having trouble.")}
            }
        }
        "-h" | "--help" => {
            todo!();
        }
        _ => {
            println!("New? Run the program with '-h', '--help', 'install -h', or 'install --help' to get started.");
            return;
        }
    }
}





