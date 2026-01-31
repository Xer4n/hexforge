
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

    let request = request::build_get_request(
        &method,
        &args.target,
        &args.path,
        &headers,
        !args.no_host,
        !args.no_ua
    );

    if args.verbose {
        println!("\x1b[33m[INFO]\x1b[0m - VERBOSE mode enabled.");

        if args.no_host {
            println!("\x1b[33m[INFO]\x1b[0m - Host header not included.");
        }

        if args.no_ua {
            println!("\x1b[33m[INFO]\x1b[0m - User-Agent header not included.");
        }
        println!("\x1b[32m\nConstructed request:\x1b[0m\n");
        println!("{}", String::from_utf8_lossy(&request));
    }

    // If test mode is enabled, exit without sending requests
    if args.test {
        println!("\x1b[31m[TEST MODE]\x1b[0m - No requests sent.");
        std::process::exit(0);
    }

    let response = tcp::send_raw(
        &args.target,
        args.port,
        &request,
    )?;

    println!("\n\n\x1b[32mServer response:\x1b[0m\n");
    println!("{}", String::from_utf8_lossy(&response));

    Ok(())

}
