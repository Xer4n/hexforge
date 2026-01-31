use crate::http::{HttpMethod, Header};

pub fn build_get_request(
    method: &HttpMethod,
    host: &str,
    path: &str,
    headers: &[Header],
    include_host: bool,
    include_ua: bool
)-> Vec<u8>  {
    let mut request = format!(
        "{} {} HTTP/1.1\r\n",
        method.as_str(),
        path,
    );

    if include_host {
        request.push_str(&format!("Host: {}\r\n", host));
    }

    if include_ua {
        request.push_str("User-Agent: Hexforge\r\n");
    }

    for header in headers {
        request.push_str(&format!("{}: {}\r\n", header.name, header.value));
    }

    request.push_str("Connection: close\r\n\r\n");
    request.into_bytes()
}
