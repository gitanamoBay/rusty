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
    
    println!("{:#?}", args);
    let filepath = Path::new("itsbroke.txt");
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
        //let status = if (args.arg_status.is_empty()) {
        //    "new" 
        //} else {
        //    args.arg_status
        //};


        let ent = Entry::new(1,"1","1");
        let ent1 = Entry::new(2,"2","2");
        let ent2 = Entry::new(3,"3","3");

        
        //let mut v = vec![json::encode(&ent).unwrap(),json::encode(&ent1).unwrap(),json::encode(&ent2).unwrap()];
        let mut v = vec![ent,ent1,ent2];
        
        let encoded = json::encode(&v).unwrap();
         
        match file.write_all(encoded.as_bytes()) {
            Ok(_) => println!("worked."),
            Err(why) => panic!("errored on write:  {}",why),
        }
    }


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

