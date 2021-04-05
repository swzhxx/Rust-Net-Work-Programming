use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::thread;

fn main() {
    let socket_addr = "127.0.0.1:2000".parse::<SocketAddr>().unwrap();
    let socket = TcpListener::bind(socket_addr).expect("tcp relay server bind failed");

    loop {
        match socket.accept() {
            Ok((stream, addr)) => {
                let client_stream =
                    TcpStream::connect("127.0.0.1:8080".parse::<SocketAddr>().unwrap())
                        .expect("connect failed");

                //开启线程 转发
                let mut stream_clone_dst = stream.try_clone().unwrap();
                let mut stream_clone_src = stream.try_clone().unwrap();

                let mut client_stream_dst = client_stream.try_clone().unwrap();
                let mut client_stream_src = client_stream.try_clone().unwrap();
                thread::spawn(move || loop {
                    let mut buf = [0u8; 4092];
                    let bytes_read = stream_clone_src.read(&mut buf).unwrap();
                    if bytes_read >= 0 {
                        client_stream_dst.write_all(&buf);
                    } else {
                        client_stream_dst.shutdown(Shutdown::Both);
                    }
                });
                thread::spawn(move || {
                    let mut buf = [0u8; 4092];
                    let bytes_read = client_stream_src.read(&mut buf).unwrap();
                    if bytes_read >= 0 {
                        stream_clone_dst.write_all(&buf);
                    } else {
                        stream_clone_dst.shutdown(Shutdown::Both);
                    }
                });
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}
