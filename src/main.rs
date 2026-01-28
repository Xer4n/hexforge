mod cli;
mod http;
mod net;
mod output;

use clap::Parser;
use cli::Cli;

fn main() {

    let args = Cli::parse();

    println!("Hexforge bound to {}:{}", args.target, args.port);

}
