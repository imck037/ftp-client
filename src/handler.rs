use crate::command::{Connection, read_response, write_response};

use std::{
    io::{self, BufReader, Read, Write},
    net::TcpStream,
};

pub fn handle_list(stream: &mut TcpStream, conn: &mut Connection) {
    if let Some(data_stream) = &conn.datastream {
        let mut buffer = String::new();
        let mut data_reader = BufReader::new(data_stream.try_clone().unwrap());
        data_reader.read_to_string(&mut buffer).unwrap();
        print!("{buffer}");
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let response = read_response(&mut reader).unwrap();
        write_response(&response);
        conn.datastream = None;
    } else {
        stream.write_all(b"PASV\r\n").unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let response = read_response(&mut reader).unwrap();
        conn.datastream = handle_pasv(stream, &response[0]);
        stream.write_all(b"LIST\r\n").unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let response = read_response(&mut reader).unwrap();
        write_response(&response);
        handle_list(stream, conn);
    }
}

pub fn handle_pasv(stream: &mut TcpStream, response_line: &str) -> Option<TcpStream> {
    let start = response_line.find("(").unwrap() + 1;
    let end = response_line.find(")").unwrap();
    let numbers: Vec<u16> = response_line[start..end]
        .split(',')
        .filter_map(|num| num.parse::<u16>().ok())
        .collect();
    if numbers.len() != 6 {
        println!("Error openning the passive mode connection.");
        return None;
    }
    let ip = stream.peer_addr().unwrap().ip();
    let port = numbers[4] * 256 + numbers[5];
    let addr = format!("{}:{}", ip, port);
    let data_connection = TcpStream::connect(addr).expect("Connection error.");
    Some(data_connection)
}

pub fn handle_authentication(stream: &mut TcpStream) {
    print!("Enter the UserName: ");
    io::stdout().flush().unwrap();

    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name).unwrap();

    let user_response = format!("USER {}", user_name);
    stream.write_all(user_response.as_bytes()).unwrap();
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut response = read_response(&mut reader).unwrap();

    for line in response {
        let code = line[..3].to_string();
        if code == "331".to_string() {
            print!("Enter the Password: ");
            io::stdout().flush().unwrap();
            let mut password = String::new();
            io::stdin().read_line(&mut password).unwrap();
            let password_response = format!("PASS {}", password);
            stream.write_all(password_response.as_bytes()).unwrap();
            response = read_response(&mut reader).unwrap();
            write_response(&response);
        } else if code == "230".to_string() {
            print!("{}", line);
        } else {
            eprintln!("Login Failed");
        }
    }
}
