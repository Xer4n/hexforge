pub mod request;

#[derive(Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
}

#[derive(Debug, Clone)]
pub struct Header {
    pub name: String,
    pub value: String,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT=> "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::HEAD => "HEAD",
        }
    }
}

impl Header {
    pub fn parse(raw: &str) -> Result<Self, String>{
        let parts: Vec<&str> = raw.splitn(2, ':').collect();

        if parts.len() != 2 {
            return Err(format!("Invalid header format: {}", raw));
        }

        Ok(Header {
            name: parts[0].trim().to_string(),
            value: parts[1].trim().to_string()
        })
    }

}
