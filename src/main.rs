#![feature(plugin)]
#![plugin(docopt_macros)]
extern crate rustc_serialize;
extern crate docopt;

use rustc_serialize::json;
use docopt::Docopt;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Entry{
    name: String,
    progress: u16,
    id: u64,
}

docopt!(Args derive Debug, "
rusty app is a simple note keeping app

Usage:
    rusty <command> [<args>]

Options:
    -h --help  Show this help message
");

//    rusty add <thing>
//    rusty list [<criteria>]
//    rusty update <id> <newStatus>
//    rusty remove <id>
//    rusty --help


#[derive(Debug)]
enum Command{
    Add,List,Update,Remove
}
pub struct Collection{
    data: Vec<Entry>
}

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
      
    if(args.cmd_add)
    {
        println!("{}",args.arg_thing);
    }
    
    if(args.cmd_remove)
    {
    }

    
//    let ent = Entry{
//        name: "entry x".to_string(),
//        progress: 2,
//        id: 131
//    };
        
  //  let encoded = json::encode(&ent).unwrap();

//    let decoded: Entry = json::decode(&encoded[..]).unwrap();
     
}



