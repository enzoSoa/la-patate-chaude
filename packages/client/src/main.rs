use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    if let Ok(stream) = TcpStream::connect("127.0.0.1:80") {
        println!("{:#?}", stream);
    } else { println!("Connection failed") };

    Ok(())
}