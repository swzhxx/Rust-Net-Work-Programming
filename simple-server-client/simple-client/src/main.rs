use std::io::{self, prelude::*, BufRead, Write};
use std::net::TcpStream;
use std::str;
// extern crate rand;
use rand::{self, thread_rng, Rng};

fn main() -> io::Result<()> {
    let chars = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
    ];
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    for _ in 0..10000 {
        let nums: u8 = rand::random();
        let mut writer = String::from("");
        let mut rng = thread_rng();
        for i in 0..nums {
            let r = rng.gen_range(0..15);
            writer = writer + chars[r];
        }
        stream.write(writer.as_bytes());
    }
    
    Ok(())
}
