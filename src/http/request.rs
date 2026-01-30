

pub fn build_get_request(host: &str, path: &str) -> Vec<u8>  {
    let request = format!(
        "GET {} HTTP/1.1\r\n
        Host: {}\r\n
        User-Agent: Hexforge\r\n
        Connection: Close\r\n
        \r\n",
        path,
        host
    );

    request.into_bytes()
}
