use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    UNKNOWN,
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::UNKNOWN,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    UNKNOWN,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::UNKNOWN,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    PATH(String),
}

pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    fn parse_req_line(line: &str) -> (Method, Resource, Version) {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let m: Method = parts[0].into();
        let r = Resource::PATH(parts[1].to_string());
        let v: Version = parts[2].into();
        (m, r, v)
    }
}

impl From<String> for HttpRequest {
    fn from(value: String) -> Self {
        let mut method = Method::UNKNOWN;
        let mut version = Version::UNKNOWN;
        let mut resource = Resource::PATH("".to_string());
        let mut headers = HashMap::new();
        let mut body = String::from("");

        let lines = value.lines().collect::<Vec<&str>>();
        println!("=== start parse http request ===");
        // 请求行
        let req_line = lines[0];
        let (m, r, v) = HttpRequest::parse_req_line(req_line);
        println!("method: {:?}, resource: {:?}, version: {:?}", m, r, v);
        method = m;
        version = v;
        resource = r;

        // header
        let mut i = 1;
        while lines[i].len() != 0 {
            let mut iter = lines[i].split(":");
            let key = iter.next().unwrap();
            let value = iter.next().unwrap().trim();
            println!("key: {:?}, value: {:?}", key, value);
            headers.insert(key.to_string(), value.to_string());
            i += 1;
        }

        i += 1;
        // body
        if i < lines.len() {
            body = lines[i].to_string();
        }
        println!("body: {:?}", body);
        
        Self {
            method,
            version,
            resource,
            headers,
            body,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::http_request::{HttpRequest, Method, Resource, Version};

    #[test]
    fn test_get_http_request() {
        let req = 
        "GET /text.html HTTP/1.1
Accept: */*q
Accept-Language: zh-cn
Accept-Encoding: gzip, deflate
User-Agent: Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1; .NET CLR 2.0.50727; .NET CLR 3.0.04506.648; .NET CLR 3.5.21022)
Host: 127.0.0.1
Connection: Keep-Alive

";

        println!("{}", req);

        let http_req = HttpRequest::from(req.to_string());

        let mut expected_headers: HashMap<String, String> = HashMap::new();
        expected_headers.insert("Accept".into(), "*/*q".into());
        expected_headers.insert("Accept-Language".into(), "zh-cn".into());
        expected_headers.insert("Accept-Encoding".into(), "gzip, deflate".into());
        expected_headers.insert("User-Agent".into(), "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1; .NET CLR 2.0.50727; .NET CLR 3.0.04506.648; .NET CLR 3.5.21022)".into());
        expected_headers.insert("Host".into(), "127.0.0.1".into());
        expected_headers.insert("Connection".into(), "Keep-Alive".into());

        assert_eq!(http_req.method, Method::GET);
        assert_eq!(http_req.resource, Resource::PATH("/text.html".to_string()));
        assert_eq!(http_req.version, Version::V1_1);
        assert_eq!(http_req.headers, expected_headers);

    }
    

    #[test]
    fn test_post_http_request() {
        let req = 
        "POST /text.html HTTP/1.1
Host:www.wrox.com
User-Agent:Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1; .NET CLR 2.0.50727; .NET CLR 3.0.04506.648; .NET CLR 3.5.21022)
Content-Type:application/x-www-form-urlencoded
Content-Length:40
Connection: Keep-Alive

name=Professional%20Ajax&publisher=Wiley";

        println!("{}", req);

        let http_req = HttpRequest::from(req.to_string());

        let mut expected_headers: HashMap<String, String> = HashMap::new();
        expected_headers.insert("Content-Type".into(), "application/x-www-form-urlencoded".into());
        expected_headers.insert("Content-Length".into(), "40".into());
        expected_headers.insert("User-Agent".into(), "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1; .NET CLR 2.0.50727; .NET CLR 3.0.04506.648; .NET CLR 3.5.21022)".into());
        expected_headers.insert("Host".into(), "www.wrox.com".into());
        expected_headers.insert("Connection".into(), "Keep-Alive".into());

        assert_eq!(http_req.method, Method::POST);
        assert_eq!(http_req.resource, Resource::PATH("/text.html".to_string()));
        assert_eq!(http_req.version, Version::V1_1);
        assert_eq!(http_req.headers, expected_headers);
        assert_eq!(http_req.body, "name=Professional%20Ajax&publisher=Wiley".to_string());

    }
}