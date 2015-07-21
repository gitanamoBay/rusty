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


some commands
    add     Add a new note, optional args for status
    list    Lists all notes
    update  Update a note with id to a new status
    remove  Remve note with id

");

//    rusty add <thing>
//    rusty list [<criteria>]
//    rusty update <id> <newStatus>
//    rusty remove <id>
//    rusty --help
#[derive(Debug)]
struct Args {
    arg_command: Option<Command>,
    arg_args: Vec<String>,
    flag_list: bool,
    flag_verbose: bool,
}

#[derive(Debug)]
enum Command{
    Add,List,Update,Remove
}
pub struct Collection{
    data: Vec<Entry>
}

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
      
  //  if(args.cmd_add)
  //  {
  //      println!("{}",args.arg_thing);
  //  }
  //  
  //  if(args.cmd_remove)
  //  {
  //  }


    
//    let ent = Entry{
//        name: "entry x".to_string(),
//        progress: 2,
//        id: 131
//    };
        
  //  let encoded = json::encode(&ent).unwrap();

//    let decoded: Entry = json::decode(&encoded[..]).unwrap();
     
}



