
mod cli;
mod http;
mod net;
mod output;

use clap::Parser;
use cli::Cli;
use net::tcp;
use http::request;


fn main() -> std::io::Result<()> {

    let args = Cli::parse();

    println!("Hexforge bound to {}:{}", args.target, args.port);


    let request = request::build_get_request(&args.target, &args.path);

    let response = tcp::send_raw(
        &args.target,
        args.port,
        &request,
    )?;

    println!("Server responded:");
    println!("{}", String::from_utf8_lossy(&response));

    Ok(())

}
