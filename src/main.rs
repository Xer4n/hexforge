
mod cli;
mod http;
mod net;
mod output;

use clap::Parser;
use cli::Cli;
use net::tcp;


fn main() -> std::io::Result<()> {

    let args = Cli::parse();

    println!("Hexforge bound to {}:{}", args.target, args.port);


    let request = format!(
        "GET / HTTP/1.1\r\n\
        Host: {}\r\n\
        User-Agent: Hexforge\r\n
        Connection: close\r\n\
        \r\n",
        args.target
    );

    let response = tcp::send_raw(
        &args.target,
        args.port,
        request.as_bytes()
    )?;

    println!("Server responded:");
    println!("{}", String::from_utf8_lossy(&response));

    Ok(())

}
