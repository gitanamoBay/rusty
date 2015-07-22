use rustc_serialize::json;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Entry {
    pub name: String,
    pub status: String,
    pub id: u32,
}

impl Entry {
    pub fn new(id: u32, name: &str, status: &str) -> Entry {
        return Entry {
            name: name.to_string(),
            status: status.to_string(),
            id: id
        };
    }
}
