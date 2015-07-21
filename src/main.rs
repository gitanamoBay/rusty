#![feature(plugin)]
#![plugin(docopt_macros)]
extern crate rustc_serialize;
extern crate docopt;

mod entry;

use rustc_serialize::json;
use docopt::Docopt;
use entry::Entry;

docopt!(Args derive Debug, "
rusty app is a simple note keeping app

Usage:
    rusty add <name> [<status>]
    rusty list [<criteria>]
    rusty update <id> <status>
    rusty remove <id>
    rusty --help

Options:
    -h --help  Show this help message
");

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    
    println!("{:#?}", args);

    if(args.cmd_add)
    {
       println!("{}",args.arg_name);
    
    }
    
    let ent = Entry{
        name: "entry".to_string(),
        id: 1,
        status: "".to_string()
    };
    
   
    //if(args.cmd_remove)
    //{
    //}
        
  //  let encoded = json::encode(&ent).unwrap();

//    let decoded: Entry = json::decode(&encoded[..]).unwrap();
     
}



