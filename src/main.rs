use std::{
    env,
    io::{self, Read, Write},
    net::TcpStream,
};

#[derive(Debug)]
enum ArgError {
    ArgNotGiven,
    ArgLenthError,
}

#[allow(unused)]
enum Command {
    USER,
    PASS,
    SYST,
    PORT,
    PASV,
    CWD,
    PWD,
    CDUP,
    MKD,
    RMD,
    STAT,
    DELE,
    LIST,
    RETR,
    QUIT,
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

fn command_parsing(mut stream: &TcpStream) {
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

fn handle_authentication(mut stream: &TcpStream) {
    print!("Enter the UserName: ");
    io::stdout().flush().unwrap();
    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name).unwrap();
    let user_response = format!("USER {}", user_name);
    stream.write_all(user_response.as_bytes()).unwrap();
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let response = String::from_utf8_lossy(&buffer);
    let response_part: Vec<&str> = response.split_whitespace().collect();
    if response_part[0] == "331" {
        print!("Enter the Password: ");
        io::stdout().flush().unwrap();
        let mut password = String::new();
        io::stdin().read_line(&mut password).unwrap();
        let password_response = format!("PASS {}", password);
        stream.write_all(password_response.as_bytes()).unwrap();
    } else if response_part[0] == "230" {
        println!("{}", response);
    } else {
        eprintln!("Login Failed");
    }
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
    handle_authentication(&stream);
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let response = String::from_utf8_lossy(&buffer);
    println!("{response}");
    command_parsing(&stream);
    Ok(())
}
