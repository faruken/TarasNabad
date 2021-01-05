mod taras;

extern crate tiny_http;

use tiny_http::{Server, Response, Method};
use crate::taras::{TarasNabadImpl, TarasNabad};
use uuid::Uuid;
use rocksdb::Error;
use structopt::StructOpt;
use futures::executor::{ThreadPoolBuilder, ThreadPool};
use futures::task::SpawnExt;
use futures::future;


fn is_json(request: &tiny_http::Request) -> bool {
    let headers = request.headers();
    for header in headers {
        if header.value == "application/json" {
            return true;
        }
    }
    false
}


#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "p", long = "port", default_value = "8000")]
    port: u16,

    #[structopt(short = "t", long = "thread-pool-size", default_value = "32")]
    thread_pool_size: usize,
}


fn main() {
    let opt = Opt::from_args();
    let port = opt.port;
    let thread_pool_size = opt.thread_pool_size;
    let pool = ThreadPoolBuilder::new().pool_size(thread_pool_size).create().expect("Failed to create thread pool");
    let mut db = taras::TarasNabad::new("d.db").to_owned();
    let server = Server::http(("127.0.0.1", port)).unwrap();
    println!("Welcome home, great slayer.");
    println!("Serving on 127.0.0.1:{}", port);
    futures::executor::block_on(future::lazy(move |_| {
        for mut request in server.incoming_requests() {
            let mut db = db.clone();
            match request.method() {
                &Method::Get => {
                    let uri = request.url().trim();
                    let path = &uri[1..];
                    match Uuid::parse_str(path) {
                        Ok(value) => {
                            match db.get(&value) {
                                Ok(Some(data)) => {
                                    pool.spawn(future::lazy(move |_| {
                                        request.respond(Response::from_string(String::from_utf8(data).unwrap()));
                                    })).unwrap();
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
                    if !is_json(&request) {
                        let rep = tiny_http::Response::new_empty(tiny_http::StatusCode(400));
                        let _ = request.respond(rep);
                    } else {
                        pool.spawn(future::lazy(move |_| {
                            let mut content = String::new();
                            request.as_reader().read_to_string(&mut content).unwrap();
                            let data: String = content.parse().unwrap();
                            let flag = db.put(data).to_string();
                            let rep = tiny_http::Response::new_empty(tiny_http::StatusCode(201));
                            request.respond(Response::from_string(flag));
                        })).unwrap();
                    }
                }
                &Method::Delete => {
                    let uri = request.url().trim();
                    let path = &uri[1..];
                    match Uuid::parse_str(path) {
                        Ok(value) => {
                            match db.delete(&value) {
                                Ok(()) => {
                                    pool.spawn(future::lazy(move |_| {
                                        let rep = tiny_http::Response::new_empty(tiny_http::StatusCode(200));
                                        let _ = request.respond(rep);
                                    })).unwrap();
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
                _ => {
                    request.respond(Response::from_string("Unsupported Method"));
                }
            }
        }
    }));
}
