use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        println!(" bytes_read {}", bytes_read);
        if bytes_read == 0 {
            return Ok(());
        }
        let s = String::from_utf8_lossy(&buf[..bytes_read]);
        println!("{}", s);
        stream.write("get_it".as_bytes())?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        println!("new comeing");
        let stream = stream.expect("failed");
        handle_client(stream).unwrap_or_else(|error| eprint!("{:?}", error));
    }
    Ok(())
}
