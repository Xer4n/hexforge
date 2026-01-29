use std::net::TcpStream;
use std::io::{Read, Write};

mod cli;
mod http;
mod net;
mod output;

use clap::Parser;
use cli::Cli;

fn main() -> std::io::Result<()> {

    let args = Cli::parse();

    println!("Hexforge bound to {}:{}", args.target, args.port);


    let address = format!("{}:{}", args.target, args.port);

    let mut stream = TcpStream::connect(address)?;


    let request = format!(
        "GET / HTTP/1.1\r\n\
        Host: {}\r\n\
        User-Agent: Hexforge\r\n
        Connection: close\r\n\
        \r\n",
        args.target
    );

    stream.write_all(request.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    println!("Server responded:");
    println!("{}", response);

    Ok(())

}
