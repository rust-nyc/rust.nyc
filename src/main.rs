extern crate iron;

use iron::prelude::*;
use iron::mime;
use iron::status;
use std::io::Read;
use std::fs;
use std::path;

fn static_file_response<P: AsRef<path::Path>>(path: P) -> Response {
    let file = fs::File::open(path.as_ref()).expect("could not read from file");
    let bytes = file.bytes().map(|b| b.unwrap()).collect::<Vec<_>>();
    let mut response = Response::with((status::Ok, bytes));
    response.headers.set(iron::headers::ContentType(mime::Mime(mime::TopLevel::Text, mime::SubLevel::Html, vec![])));
    response
}

fn respond(req_: &mut Request) -> Result<Response, IronError> {
    let response = static_file_response("index.html");
    Ok(response)
}

fn main() {
    Iron::new(respond).http("localhost:3000").unwrap();
}