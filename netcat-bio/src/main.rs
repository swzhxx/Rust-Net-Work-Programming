use std::env;
use std::io::{self, Read, Write};
use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;

fn write_n(buf: &mut [u8], len: usize) {
    io::stdout().write_all(&buf[..len]);
}

fn run(mut stream: TcpStream) {
    let mut osstream = stream.try_clone().unwrap();
    let thr;
    {
        // let stream = Arc::clone(&stream);
        thr = thread::spawn(move || {
            let mut buf = [0u8; 8192];
            let mut nr = 0;
            loop {
                nr = stream.read(&mut buf).unwrap();
                write_n(&mut buf, nr);
                // if nw < nr {
                //     break;
                // }
            }
            std::process::exit(1);
        })
    }

    //监听用户输入,然后由socket发送
    loop {
        let mut input = String::from("");
        let buf = [0u8; 8192];
        io::stdin()
            .read_line(&mut input)
            .expect("Faild to read from stdin");
        osstream
            .write_all(input.as_bytes())
            .expect("Faild to write to server");
    }
    osstream.shutdown(Shutdown::Write).expect("shutdown Failed");
    thr.join();
}

fn main() {
    let arg: Vec<_> = env::args().collect();
    let port = arg[2].to_string();
    if arg.len() != 3 {
        eprintln!("Usage:\n {} hostname port \n", arg[0]);
        std::process::exit(1);
    }
    if arg[1] == "-l" {
        let addr = format!("127.0.0.1:{}", port).parse::<SocketAddr>().unwrap();
        let listener = TcpListener::bind(addr).expect("Failed bind addr");
        for stream in listener.incoming() {
            match stream {
                Err(e) => {
                    eprintln!("{}", e);
                }
                Ok(stream) => run(stream),
            }
        }
    } else {
        //Client
        let hostname = arg[1].to_string();
        let socket_addr = format!("{}:{}", hostname, port)
            .parse::<SocketAddr>()
            .unwrap();

        if let Ok(stream) = TcpStream::connect(socket_addr) {
            run(stream)
        }
    }
}
