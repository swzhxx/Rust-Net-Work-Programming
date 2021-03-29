use std::boxed::Box;
use std::collections::HashMap;
use std::io::Result;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

type Store = HashMap<Box<String>, Mutex<String>>;

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

fn handle_read(stream: TcpStream, memcahced: Arc<Memcached>) {}

fn main() -> io::Result<()> {
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(addr)?;
    let memcached = Memcached::new();
    let arc_memcached = Arc::new(memcached);
    for stream in listener.incoming() {
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
