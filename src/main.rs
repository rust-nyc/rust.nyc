extern crate iron;

use iron::prelude::*;
use iron::status;
use std::io::Read;
use std::fs;

fn respond(req: &mut Request) -> Result<Response, IronError> {
    let file = fs::File::open("index.html").expect("could not read from file");
    let bytes = file.bytes().map(|b| b.unwrap()).collect::<Vec<_>>();
    let mut response = Response::with((status::Ok, bytes));
    response.headers.set(iron::headers::ContentType(iron::mime::Mime(iron::mime::TopLevel::Text, iron::mime::SubLevel::Html, vec![])));
    Ok(response)
}

fn main() {
    Iron::new(respond).http("localhost:3000").unwrap();
}