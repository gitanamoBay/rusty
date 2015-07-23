#![feature(path_ext, plugin)]
#![plugin(docopt_macros)]

extern crate docopt;
extern crate rustc_serialize;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use docopt::Docopt;
use rustc_serialize::json;

use entry::Entry;

mod entry;

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
    
    let filepath = Path::new("foo.txt");
    let mut file = if filepath.exists() {
        match OpenOptions::new().read(true).write(true).open(filepath) {
            Ok(file) => file,
            Err(why) => panic!("failed to open file {}",why)
        }
    } else {
        match File::create(filepath) {
            Ok(file) => file,
            Err(why) => panic!("it couldn't make files {}",why),
        }
    };
    
    if args.cmd_add {
        //we love egyptian braces.
        
    }

    let ent = Entry::new(1,"test","test");

    let encoded = json::encode(&ent).unwrap();
    
    match file.write_all(encoded.as_bytes()) {
        Ok(_) => println!("worked."),
        Err(why) => panic!("errored on write:  {}",why),
    }

   // println!("{:#?}", args);

    //if(args.cmd_add)
    //{
     //  println!("{}",args.arg_name);
    
   // }    
   
    //if(args.cmd_remove)
    //{
    //}
        
  //  let encoded = json::encode(&ent).unwrap();

//    let decoded: Entry = json::decode(&encoded[..]).unwrap();
     
}

