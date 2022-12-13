use std::env;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let server_address = retrieve_server_address();
    if let Ok(_stream) = TcpStream::connect(server_address) {
        println!("Successfully connected");
    } else { println!("Connection failed") };
    Ok(())
}

fn retrieve_server_address() -> String {
    match retrieve_args().get(1) {
        Some(address) => String::from(address),
        None => String::from("127.0.0.1:80")
    }
}

fn retrieve_args() -> Vec<String> {
    return env::args().collect();
}