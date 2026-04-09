use std::{
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
};

use crate::handler::{handle_list, handle_pasv};

pub struct Connection {
    pub datastream: Option<TcpStream>,
}

pub fn command_parsing(stream: &mut TcpStream) {
    let mut conn = Connection { datastream: None };
    loop {
        print!("ftp> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Expect an input..");

        let command: Vec<&str> = input.trim().split_whitespace().collect();
        if command.len() == 0 {
            continue;
        }

        stream.write_all(input.as_bytes()).unwrap();

        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let response = read_response(&mut reader).unwrap();
        write_response(&response);

        match command[0].to_uppercase().as_str() {
            "PASV" => {
                conn.datastream = handle_pasv(stream, &response[0]);
            }
            "LIST" => {
                handle_list(stream, &mut conn);
            }
            "QUIT" => break,
            _ => continue,
        };
    }
}

pub fn read_response(reader: &mut BufReader<TcpStream>) -> std::io::Result<Vec<String>> {
    let mut lines = Vec::new();
    let mut line = String::new();
    reader.read_line(&mut line)?;
    lines.push(line.clone());

    if line.as_bytes().get(3) == Some(&b'-') {
        loop {
            line.clear();
            reader.read_line(&mut line)?;
            lines.push(line.clone());
            if line.as_bytes().get(3) == Some(&b' ') {
                break;
            }
        }
    }
    Ok(lines)
}

pub fn write_response(response: &Vec<String>) {
    for line in response {
        print!("{line}");
    }
}
