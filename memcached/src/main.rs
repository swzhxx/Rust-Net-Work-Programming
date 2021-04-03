use std::sync::{Arc, RwLock};
use std::thread;
use std::{borrow::BorrowMut, convert::TryInto};
use std::{boxed::Box, io::Read};
use std::{collections::HashMap, mem};
use std::{
    intrinsics::transmute,
    net::{SocketAddr, TcpListener, TcpStream},
};
use std::{io::Write, str};
use std::{
    io::{self, BufRead, BufReader, Result},
    usize,
};
type Store = HashMap<String, String>;

const get_faild: &str = "NOT FIND";
const delete_success: &str = "DELETE SUCCESS";
const delete_error: &str = "DELETE ERROR";
const set_success: &str = "SET SUCCESS";
const set_faild: &str = "SET FAILED";

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
    keyLength: u32,
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

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.store.remove(key)
    }
}

fn handle_read(mut stream: TcpStream, mut memcached: Arc<RwLock<Memcached>>) {
    // let len = std::mem::size_of(Message);
    let mut message: Message;
    let mut buf: Vec<u8> = vec![];
    let mut size: usize = 0;
    // let mut op: Operate = Operate::SET;

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
            // let op_raw_data:[u8;1] = [buf[0]];

            raw_data.clone_from_slice(buf[1..5].try_into().expect("try_into error"));
            keyLength = u32::from_be_bytes(raw_data);
            raw_data.clone_from_slice(buf[5..9].try_into().expect("try_into error"));
            valueLength = u32::from_be_bytes(raw_data);
            // println!("key length {}", keyLength);
            // println!("value length {}", valueLength);
            size = (keyLength + valueLength) as usize;
        }
        if size > 0 {
            // 判断buf是否已经完成了key长度的接受
            let len: usize = buf.len() - 9;
            if len >= size {
                let op = unsafe { mem::transmute_copy::<[u8; 1], Operate>(&[buf[0]]) };
                let keyend = 9 + keyLength as usize;
                let key = str::from_utf8(&buf[9..keyend]).unwrap().to_string();

                let value = str::from_utf8(&buf[keyend..keyend + valueLength as usize])
                    .unwrap()
                    .to_string();
                // println!("receive key {}", key);
                // println!("receive value {}", value);
                size = 0;
                let mut index = 0;
                buf.retain(|_i| {
                    index += 1;
                    (index - 1) < size
                });

                message = Message {
                    op: op,
                    keyLength,
                    valueLength,
                    key: Box::new(key),
                    value: Box::new(value),
                };

                //处理操作
                match message.op {
                    Operate::GET => {
                        match memcached.try_read() {
                            Ok(memcached) => {
                                match memcached.get(&message.key) {
                                    Some(value) => {
                                        stream.write(value.as_bytes());
                                    }
                                    _ => {
                                        stream.write_all(get_faild.as_bytes());
                                    }
                                };
                            }
                            _ => {
                                print!("a");
                            }
                        };
                    }
                    Operate::SET => {
                        match memcached.try_write() {
                            Ok(mut memcached) => {
                                memcached.set(
                                    message.key.to_owned().to_string(),
                                    message.key.to_owned().to_string(),
                                );
                                stream.write_all(set_success.as_bytes());
                            }
                            _ => {
                                stream.write_all(set_faild.as_bytes());
                            }
                        };
                    }
                    Operate::DELETE => {
                        match memcached.try_write() {
                            Ok(mut memcached) => {
                                memcached.delete(&message.key);
                                stream.write_all(delete_success.as_bytes());
                            }
                            _ => {
                                // stream.write(delete_error.as_bytes());
                            }
                        };
                    }
                    _ => {
                        stream.write_all("Unknown Operate".as_bytes());
                    }
                };

                println!("{:?}", memcached.read().unwrap().store);
            }
        }
        // println!("接受:{:?}", &buf[..bytes_read]);
        // let raw_data: &[u8; len] = &buf[..len].try_into().expect("");
        // message = unsafe { std::mem::transmute_copy::<[u8; len], Message>(raw_data) };
        // println!("{:?}", message);
    }
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
// TCP 测试工具
// 00 00 00 00 02 00 00 00 03 62 61 62 63 64
// 02 00 00 00 02 00 00 00 03 62 61 62 63 64
