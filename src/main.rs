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

fn args_parsing(args: env::Args) -> Result<Vec<String>, ArgError> {
    let args: Vec<String> = args.collect();
    if args.len() == 1 {
        return Err(ArgError::ArgNotGiven);
    } else if args.len() > 3 {
        return Err(ArgError::ArgLenthError);
    }

    Ok(args)
}

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

fn main() -> Result<(), ArgError> {
    let args: Vec<String> = args_parsing(env::args())?;
    let host = format!("{}:{}", args[1], args[2]);
    let mut stream = TcpStream::connect(host).expect("connection is failed");
    println!("Connection is established...");
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let response = String::from_utf8_lossy(&buffer);
    println!("{response}");
    command_parsing(stream);
    Ok(())
}
