use rustc_serialize::json;
use std::fmt;
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
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "\n\n{0}\nStatus:{1} \nID:{2}", self.name,self.status,self.id)
    }
}
