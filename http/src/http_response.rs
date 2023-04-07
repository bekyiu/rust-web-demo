use std::{collections::HashMap, hash::Hash, io::Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut resp = HttpResponse::default();
        resp.headers = match &headers {
            Some(_) => headers,
            None => {
                let mut map = HashMap::new();
                map.insert("Content-Type", "text/html");
                Some(map)
            }
        };
        if status_code != "200" {
            resp.status_code = status_code;
            resp.status_text = match status_code {
                "400" => "Bad Request",
                "404" => "Not Found",
                "500" => "Internal Server Error",
                _ => "Unknown",
            }
        }

        resp.body = body;
        resp
    }

    pub fn send_response<T: Write>(&self, write_stream: &mut T) -> std::io::Result<()> {
        // 这里clone时, 会递归调用每个字段的clone
        // &str clone 就是复制一份栈上的地址
        // 不可变引用 实现了copy trait
        // rust 规定 当T: Copy时, T.clone()也必须是简单的内存拷贝
        let data = self.clone();
        let data = String::from(data);
        write!(write_stream, "{}", data)?;
        Ok(())
    }

    pub fn version(&self) -> &str {
        self.version
    }

    pub fn status_code(&self) -> &str {
        self.status_code
    }

    pub fn status_text(&self) -> &str {
        self.status_text
    }

    pub fn headers(&self) -> String {
        let map = self.headers.clone().unwrap();
        let mut header_str = "".to_string();
        for (k, v) in map.iter() {
            let s = format!("{}{}:{}\r\n", header_str, k, v);
            header_str = s;
        }
        header_str
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(value: HttpResponse<'a>) -> Self {
        let body = value.body();
        format!(
            "{} {} {}\r\n{}Content-Length:{}\r\n\r\n{}",
            value.version(),
            value.status_code(),
            value.status_text(),
            value.headers(),
            body.len(),
            body,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::HttpResponse;

    #[test]
    fn test_new() {
        let actual = HttpResponse::new("404", None, Some("abc".to_string()));

        let expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut expected_header = HashMap::new();
                expected_header.insert("Content-Type", "text/html");
                Some(expected_header)
            },
            body: Some("abc".to_string()),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from() {
        let resp = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut expected_header = HashMap::new();
                expected_header.insert("Content-Type", "text/html");
                Some(expected_header)
            },
            body: Some("nana".to_string()),
        };

        let actual: String = resp.into();
        let expected = "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length:4\r\n\r\nnana";
        assert_eq!(actual, expected);
    }
}
