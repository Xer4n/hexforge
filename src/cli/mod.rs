use clap::Parser;

/// Hexforge CLI - Simple low-level tool for crafting HTTP packets.


#[derive(Parser, Debug)]
#[command(author, version, about)]

pub struct Cli {

    /// Target IP address or domain name
    pub target: String,

    /// Target port
    #[arg(short, long, default_value_t = 80)]
    pub port: u16,

    /// HTTP path
    #[arg(short = 'P', long, default_value = "/")]
    pub path: String,

    /// HTTP method (GET, POST, PUT, DELETE, HEAD)
    #[arg(short = 'X', long, default_value = "GET")]
    pub method: String,

    /// Custom headers (repeatable)
    #[arg(short = 'H', long = "header")]
    pub headers: Vec<String>,

    /// Verbose mode
    #[arg(long)]
    pub verbose: bool,

    /// Test mode (No requests sent)
    #[arg(long)]
    pub test: bool,

    ///Disable host header
    #[arg(long)]
    pub no_host: bool,

    ///Disable user agent header
    #[arg(long)]
    pub no_ua: bool,
}
