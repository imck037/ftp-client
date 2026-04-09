mod command;
mod handler;
use command::{command_parsing, read_response, write_response};
use handler::handle_authentication;
use std::{env, io::BufReader, net::TcpStream};

#[derive(Debug)]
enum ArgError {
    ArgNotGiven,
    ArgLenthError,
}

fn args_parsing(args: env::Args) -> Result<Vec<String>, ArgError> {
    let args: Vec<String> = args.collect();
    if args.len() == 1 {
        return Err(ArgError::ArgNotGiven);
    } else if args.len() > 3 {
        return Err(ArgError::ArgLenthError);
    }

    Ok(args)
}

fn main() -> Result<(), ArgError> {
    let args: Vec<String> = args_parsing(env::args())?;
    let port: u16;
    if args.len() == 2 {
        port = 21;
    } else {
        port = args[2].parse().unwrap();
    }
    let host = format!("{}:{}", args[1], port);
    let mut stream = TcpStream::connect(host).expect("connection is failed");
    println!(
        "Connection to server {} established...",
        stream.peer_addr().unwrap()
    );
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let response = read_response(&mut reader).unwrap();
    write_response(&response);
    handle_authentication(&mut stream);
    command_parsing(&mut stream);
    Ok(())
}
