use std::io::Write;
use http::http_request::{HttpRequest, Method, Resource};
use crate::handler::{Handler, ServiceHandler, StaticHandler};

pub struct Router;

impl Router {
    pub fn route<T: Write>(req: HttpRequest, stream: &mut T) {
        if req.method != Method::GET {
            stream.write("HTTP/1.1 404 Not Found".as_bytes()).unwrap();
            return;
        }

        match &req.resource {
            Resource::PATH(s) => {
                let route = s.split("/").collect::<Vec<&str>>();
                let path = route[1];
                if path != "api" {
                    let resp = StaticHandler::handle(&req);
                    resp.send_response(stream).unwrap();
                    return;
                }
                // api 逻辑
                ServiceHandler::handle(&req).send_response(stream).unwrap();
            }
        }
    }
}