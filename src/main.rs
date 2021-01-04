mod taras;

extern crate tiny_http;

use tiny_http::{Server, Response, Method};
use crate::taras::{TarasNabadImpl, TarasNabad};
use uuid::Uuid;
use url::Url;
use std::str::FromStr;


fn is_json(request: &tiny_http::Request) -> bool {
    let headers = request.headers();
    for header in headers {
        if header.value == "application/json" {
            return true;
        }
    }
    false
}


fn main() {
    let mut db = taras::TarasNabad::new("d.db");
    let server = Server::http("127.0.0.1:8000").unwrap();
    println!("Welcome home, great slayer.");
    println!("Serving on 127.0.0.1:8000");
    for mut request in server.incoming_requests() {
        match request.method() {
            &Method::Get => {
                let uri = request.url().trim();
                let path = &uri[1..];
                match Uuid::parse_str(path) {
                    Ok(value) => {
                        println!("received get {}", path);
                        match db.get(&value) {
                            Ok(Some(data)) => {
                                request.respond(Response::from_string(String::from_utf8(data).unwrap()));
                            }
                            Ok(None) => {
                                let rep = tiny_http::Response::new_empty(tiny_http::StatusCode(404));
                                let _ = request.respond(rep);
                            }
                            Err(_e) => {
                                let rep = tiny_http::Response::new_empty(tiny_http::StatusCode(500));
                                let _ = request.respond(rep);
                            }
                        }
                    }
                    Err(e) => {
                        let rep = tiny_http::Response::new_empty(tiny_http::StatusCode(404));
                        let _ = request.respond(rep);
                    }
                }
            }
            &Method::Put => {
                println!("received put");
                if !is_json(&request) {
                    let rep = tiny_http::Response::new_empty(tiny_http::StatusCode(400));
                    let _ = request.respond(rep);
                } else {
                    let mut content = String::new();
                    request.as_reader().read_to_string(&mut content).unwrap();
                    let data: String = content.parse().unwrap();
                    let flag = db.put(data).to_string();
                    let rep = tiny_http::Response::new_empty(tiny_http::StatusCode(201));
                    request.respond(Response::from_string(flag));
                }
            }
            &Method::Delete => {
                println!("received delete");
            }
            _ => {
                request.respond(Response::from_string("Unsupported Method"));
            }
        }
    }
}
