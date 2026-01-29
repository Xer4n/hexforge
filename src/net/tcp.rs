
use std::net::TcpStream;
use std::io::{Read, Write};

pub fn send_raw(
    host: &str,
    port: u16,
    payload: &[u8],
) -> std::io::Result<Vec<u8>> {

    let address = format!("{}:{}", host, port);
    let mut stream = TcpStream::connect(address)?;

    stream.write_all(payload)?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;

    Ok(response)

}
