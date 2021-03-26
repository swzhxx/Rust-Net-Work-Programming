extern crate mio;
use mio::tcp::TcpListener;
use mio::*;
use std::env;
use std::net::SocketAddr;

// 这件稍后用于事件服务器的标识
const SERVER: Token = Token(0);

struct TcpServer {
    address: SocketAddr,
}

impl TcpServer {
    fn new(port: u32) -> Self {
        let address = format!("0.0.0.0:{}", port).parse::<SocketAddr>().unwrap();
        TcpServer { address }
    }
    /**
     * 将服务器绑定到一个地址并运行它，此函数还设置了分派事件的循环
     * 稍后我们将对事件上的令牌进行匹配确定它是否属于服务器
     */
    fn run(&mut self) {
        let server: TcpListener = TcpListener::bind(&self.address).expect("无法绑定");
        let poll = Poll::new().unwrap();
        poll.register(&server, SERVER, Ready::readable(), PollOpt::edge())
            .unwrap();
        let mut events = Events::with_capacity(1024);
        loop {
            poll.poll(&mut events, None).unwrap();
            for event in events.iter() {
                match event.token() {
                    SERVER => {
                        let (_stream, remote) = server.accept().unwrap();
                        println!("连接来自{}", remote);
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
        }
    }
}

fn main() {
    let mut server = TcpServer::new(18080);
    server.run();
}
