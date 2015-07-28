use std::fmt;

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

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n\n{0}\nStatus:{1} \nID:{2}", self.name,self.status,self.id)
    }
}
