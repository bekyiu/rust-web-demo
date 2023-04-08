use std::{env, fs};
use std::collections::HashMap;
use http::http_request::HttpRequest;
use http::http_response::HttpResponse;
use serde::{Serialize, Deserialize};

pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;

    fn load_file(filename: &str) -> Option<String> {
        // crate的根目录
        let default = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        // 项目的根目录
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default);
        let full_path = format!("{}/{}", public_path, filename);

        fs::read_to_string(full_path).ok()
    }
}

pub struct StaticHandler;

impl Handler for StaticHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        // 完全限定无歧义调用
        let page = <StaticHandler as Handler>::load_file("index.html");
        HttpResponse::new("200", None, page)
    }
}

pub struct ServiceHandler;

// 要被json序列化反序列化 必须要实现这两个trait
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    age: i32,
}

impl Handler for ServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let json = "{\"name\": \"asuka\", \"age\": 16}";
        let mut obj = serde_json::from_str::<User>(json).ok().unwrap();
        obj.name.push_str(" haha");
        obj.age += 1;

        let ret = serde_json::to_string(&obj);
        let mut header = HashMap::new();
        header.insert("Content-Type", "application/json");
        HttpResponse::new("200", Some(header), ret.ok())
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    // #[test]
    fn sb() {
        let default = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        println!("{:?}", default);
        // 项目的根目录
        let public_path = env::var("PUBLIC_PATH").unwrap();
        println!("{:?}", public_path);
    }
}