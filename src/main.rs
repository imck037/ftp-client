use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:2020").expect("connection is failed");
    println!("Connection is established...");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Expect an input..");

        stream.write_all(input.as_bytes()).unwrap();

        let mut buffer = String::new();
        stream.read_to_string(&mut buffer).unwrap();
        println!("{buffer}");
    }
}
