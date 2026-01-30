
mod cli;
mod http;
mod net;
mod output;

use clap::Parser;
use cli::Cli;
use net::tcp;
use http::request;
use http::HttpMethod;


fn main() -> std::io::Result<()> {

    let args = Cli::parse();

    let method = match args.method.to_uppercase().as_str() {
        "GET" => HttpMethod::GET,
        "POST" => HttpMethod::POST,
        "PUT" => HttpMethod::PUT,
        "DELETE" => HttpMethod::DELETE,
        "HEAD" => HttpMethod::HEAD,
        other => {
            eprintln!("Invalid HTTP method: {}", other);
            std::process::exit(1);
        }
    };

    let mut headers = Vec::new();
    for raw in &args.headers{
        match http::Header::parse(raw) {
            Ok(h) => headers.push(h),
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }

        }
    }

    println!("Hexforge bound to {}:{}", args.target, args.port);


    let request = request::build_get_request(&method, &args.target, &args.path, &headers);

    let response = tcp::send_raw(
        &args.target,
        args.port,
        &request,
    )?;

    println!("Server responded:");
    println!("{}", String::from_utf8_lossy(&response));

    Ok(())

}
