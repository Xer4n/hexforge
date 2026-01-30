use crate::http::{HttpMethod, Header};

pub fn build_get_request(
    method: &HttpMethod,
    host: &str,
    path: &str,
    headers: &[Header]
)-> Vec<u8>  {
    let mut request = format!(
        "{} {} HTTP/1.1\r\n
        Host: {}\r\n
        User-Agent: Hexforge\r\n
        \r\n",
        method.as_str(),
        path,
        host
    );

    for header in headers {
        request.push_str(&format!("{}: {}\r\n", header.name, header.value));
    }

    request.push_str("Connection: close\r\n\r\n");
    request.into_bytes()
}
