use std::net::UdpSocket;
use std::{io, str};
fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8000").expect("Could not bind client socket");
    socket
        .connect("127.0.0.1:8080")
        .expect("Could not connect to server");
    loop {
        let mut input = String::new();
        let mut buffer = [0u8; 1500];
        io::stdin()
            .read_line(&mut input)
            .expect("Faild to read from stdin");
        socket
            .send(input.as_bytes())
            .expect("Faild to write to server");
        let (amt, _) = socket
            .recv_from(&mut buffer)
            .expect("could nor read into buffer");
        println!(
            "{}",
            str::from_utf8(&buffer[..amt]).expect("could not write buffer as string")
        );
    }
}
