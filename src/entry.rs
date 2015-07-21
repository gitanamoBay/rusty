extern crate rustc_serialize;
use rustc_serialize::json;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Entry{
    name: String,
    status: String,
    id: u16,
}

