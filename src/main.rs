use std::io;
use std::io::prelude::*;
use std::net::{IpAddr, TcpStream, SocketAddr};
use std::env;

fn session(stream: &mut TcpStream){
    let _ = stream.write(b"ENTER\r\n");

    let mut banner = [0;128];
    let _ = stream.read(&mut banner);

    let mut buffer = String::new();
    loop {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                if buffer != "\r\n"{
                    stream.write(buffer.as_bytes());
                } else {
                    break;
                }
            },
            Err(why) => panic!("{:?}", why),
        }
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // 引数が無ければぺけちょんぱする
        1 => panic!("hoge"),
        // 引数が一つの場合, その引数をアドレスとする
        2 => {
            let ip_addr = match args[1].parse() {
                Ok(ipv4addr) => ipv4addr,
                Err(why) => panic!("{:?}", why),
            };
            let port = 8000;
            let mut stream = TcpStream::connect(SocketAddr::new(IpAddr::V4(ip_addr), port)).unwrap();
            println!("{:p}", &stream);
            session(&mut stream);
        },
        // 三つ目以降の引数は無視
        3 | _ => {
            let ip_addr = match args[1].parse() {
                Ok(ipv4addr) => ipv4addr,
                Err(why) => panic!("{:?}", why),
            };
            let port = match args[2].parse() {
                Ok(port_number) => port_number,
                Err(why) => panic!("{:?}", why),
            };
            let mut stream = TcpStream::connect(SocketAddr::new(IpAddr::V4(ip_addr), port)).unwrap();
            session(&mut stream);
        },
    }
}
