use std::convert::TryInto;
use std::io::{self, BufRead, BufReader, Result};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{boxed::Box, io::Read};
use std::{collections::HashMap, mem};
type Store = HashMap<Box<String>, Mutex<String>>;

#[derive(Debug)]
#[repr(packed)]
struct Message {
    key: u32,
    value: u32,
}
struct Memcached {
    store: Store,
}

impl Memcached {
    pub fn new() -> Self {
        let mut store = HashMap::new();
        Memcached { store: store }
    }

    pub fn get(&self, key: &Box<String>) {
        self.store.get(key);
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(Box::new(key), Mutex::new(value));
    }

    pub fn delete(&mut self, key: &Box<String>) {
        self.store.remove(key);
    }
}

fn handle_read(mut stream: TcpStream, memcahced: Arc<Memcached>) {
    // let len = std::mem::size_of(Message);
    let mut message: Message;
    const len: usize = std::mem::size_of::<Message>();
    loop {
        // let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut buf = [0u8; 8192];
        let bytes_read = stream.read(&mut buf).unwrap();
        println!("接受:{:?}", &buf[..bytes_read]);
        let raw_data: &[u8; len] = &buf[..len].try_into().expect("");
        message = unsafe { std::mem::transmute_copy::<[u8; len], Message>(raw_data) };
        println!("{:?}", message);
    }
}

fn main() -> io::Result<()> {
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(addr)?;
    let memcached = Memcached::new();
    let arc_memcached = Arc::new(memcached);
    for stream in listener.incoming() {
        println!("incoming");
        match stream {
            Err(e) => {
                eprintln!("{}", e);
            }
            Ok(stream) => {
                let memcached_clone = Arc::clone(&arc_memcached);
                thread::spawn(move || {
                    handle_read(stream, memcached_clone);
                });
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    ///
    /// test mem size of
    fn test() {
        let str = String::from("Hello World");
        assert_eq!(24, std::mem::size_of_val(&str));
        assert_eq!(1, std::mem::size_of::<Operater>());
        assert_eq!(24, std::mem::size_of::<String>());
        assert_eq!(49, std::mem::size_of::<Message>());
    }
}
