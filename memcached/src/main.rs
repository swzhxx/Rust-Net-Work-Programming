use std::net::{SocketAddr, TcpListener, TcpStream};
use std::str;
use std::sync::{Arc, RwLock};
use std::thread;
use std::{borrow::BorrowMut, convert::TryInto};
use std::{boxed::Box, io::Read};
use std::{collections::HashMap, mem};
use std::{
    io::{self, BufRead, BufReader, Result},
    usize,
};
type Store = HashMap<String, String>;

#[derive(Debug)]
enum Operate {
    SET,
    GET,
    DELETE,
}

#[derive(Debug)]
#[repr(packed)]
struct Message {
    op: Operate,
    keyLenght: u32,
    valueLength: u32,
    key: Box<String>,
    value: Box<String>,
}
struct Memcached {
    store: Store,
}

impl Memcached {
    pub fn new() -> Self {
        let mut store = HashMap::new();
        Memcached { store: store }
    }

    pub fn get(&self, key: &str) {
        self.store.get(key);
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn delete(&mut self, key: &str) {
        self.store.remove(key);
    }
}

fn handle_read(mut stream: TcpStream, mut memcahced: Arc<RwLock<Memcached>>) {
    // let len = std::mem::size_of(Message);
    let mut message: Message;
    let mut buf: Vec<u8> = vec![];
    let mut size: usize = 0;
    let op: Operate = Operate::SET;

    let mut keyLength: u32 = 0;
    let mut valueLength: u32 = 0;
    loop {
        let mut buf_arr = [0u8; 8192];
        let bytes_read = stream.read(&mut buf_arr).unwrap();
        if bytes_read > 0 {
            // buf.concat(&buf_arr[..bytes_read])
            buf.extend_from_slice(&buf_arr[..bytes_read]);
        }
        // buf.len > mem::size_of::<Operate>() + 2*mem::size_of::<u32>()
        if buf.len() >= 9 && size == 0 {
            let mut raw_data = [0u8; 4];
            raw_data.clone_from_slice(buf[1..5].try_into().expect("try_into error"));
            keyLength = u32::from_be_bytes(raw_data);
            raw_data.clone_from_slice(buf[5..9].try_into().expect("try_into error"));
            valueLength = u32::from_be_bytes(raw_data);
            println!("key length {}", keyLength);
            println!("value length {}", valueLength);
            size = (keyLength + valueLength) as usize;
        }
        if size > 0 {
            // 判断buf是否已经完成了key长度的接受
            let len: usize = buf.len() - 9;
            if len >= size {
                let keyend = 9 + keyLength as usize;
                let key = str::from_utf8(&buf[9..keyend]).unwrap();
                let value = str::from_utf8(&buf[keyend..keyend + valueLength as usize]).unwrap();
                println!("receive key {}", key);
                println!("receive value {}", value);
                size = 0;
                let mut index = 0;
                buf.retain(|_i| {
                    index += 1;
                    (index - 1) < size
                });
            }
        }
        // println!("接受:{:?}", &buf[..bytes_read]);
        // let raw_data: &[u8; len] = &buf[..len].try_into().expect("");
        // message = unsafe { std::mem::transmute_copy::<[u8; len], Message>(raw_data) };
        // println!("{:?}", message);
    }
    //读写锁
    // memcahced.write().unwrap().set("a".to_string(), "b".to_string());
}

fn main() -> io::Result<()> {
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(addr)?;
    let memcached = Memcached::new();
    let arc_memcached = Arc::new(RwLock::new(memcached));
    for stream in listener.incoming() {
        println!("incoming");
        match stream {
            Err(e) => {
                eprintln!("{}", e);
            }
            Ok(stream) => {
                let mut memcached_clone = Arc::clone(&arc_memcached);
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
        let v = vec![1, 2, 3, 4, 5];
        // let s = v[2..4];
        let str = String::from("Hello World");
        assert_eq!(1, std::mem::size_of::<Operate>());
        assert_eq!(4, std::mem::size_of::<u32>());
        assert_eq!(8, std::mem::size_of::<Box<String>>());
        assert_eq!(24, std::mem::size_of_val(&str));
        assert_eq!(24, std::mem::size_of::<String>());
        assert_eq!(25, std::mem::size_of::<Message>());
    }
}
