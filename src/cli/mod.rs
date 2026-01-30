use clap::Parser;

/// Hexforge CLI - Simple low-level tool for crafting HTTP packets.


#[derive(Parser, Debug)]
#[command(author, version, about)]

pub struct Cli {

    // Target IP address
    pub target: String,

    // Target port
    #[arg(short, long, default_value_t = 80)] //Default port is 80
    pub port: u16,

    // HTTP path (default: "/")
    #[arg(short = 'P', long, default_value = "/")]
    pub path: String,
}
