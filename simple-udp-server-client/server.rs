use std::net::UdpSocket;
use std::thread;
fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8080").expect("Could not bind socket");
    loop {
        let mut buf = [0u8; 1500];
        // let sock =  socket.try_clone().expect("Failed to clone socket");
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                println!("Handling connection from {}", src);
                let buf = &mut buf[..amt];
                buf.reverse();
                socket
                    .send_to(buf, &src)
                    .expect("Failed to send a response");
            }
            Err(e) => {
                eprintln!("couldn't recieve a datagram: {}", e);
            }
        }
    }
}
