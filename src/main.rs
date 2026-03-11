use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

fn command_parsing(mut stream: TcpStream) {
    loop {
        print!("ftp> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Expect an input..");

        stream.write_all(input.as_bytes()).unwrap();

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let response = String::from_utf8_lossy(&buffer);
        println!("{response}");

        let command: Vec<&str> = input.trim().split_whitespace().collect();

        match command[0] {
            "quit" => break,
            _ => continue,
        }
    }
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:2020").expect("connection is failed");
    println!("Connection is established...");
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let response = String::from_utf8_lossy(&buffer);
    println!("{response}");
    command_parsing(stream);
}
