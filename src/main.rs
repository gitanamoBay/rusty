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
    
    let filepath = Path::new("rustynotes.txt");

    let mut file = if filepath.exists() {
        match OpenOptions::new().read(true).open(filepath) {
            Ok(file) => file,
            Err(why) => panic!("failed to open file {}",why)
        }
    } else {
        match File::create(filepath) {
            Ok(file) => file,
            Err(why) => panic!("it couldn't make files {}",why),
        }
    };

    let mut stringdata = String::new();

    if let Err(why) = file.read_to_string(&mut stringdata) {
        println!("couldn't read contents {}",why);
    }
    
    let mut entries: Vec<Entry> = if stringdata.len() != 0 {
        json::decode(&stringdata).unwrap()
    } else {
        Vec::new()
    };
     
    if args.cmd_add {
        let status = if args.arg_status.len()==0 {
            "new".to_string()
        } else {
            args.arg_status.to_string()
        };
        
        let mut done = false;
        let mut value:u32 = 0;
        let mut cindex = 0;
        
        while !done {
            if cindex == entries.len() {
                done = true;
            } else {
               if entries[cindex].id != value{
                    done = true;
                } else {
                    value += 1;
                    cindex += 1;
                }
            }
        }

        let new_ent = Entry::new(value,&args.arg_name,&status);
        
        entries.insert(cindex,new_ent);
    }

    if args.cmd_remove || args.cmd_update {
            let number: u32 = match args.arg_id.parse() {
                Ok(n) => n,
                Err(_) => panic!("error: second argument not an integer"),
            }; 


        match entries.binary_search_by(|probe| probe.id.cmp(&number)) {
            Ok(found) => {
                if args.cmd_remove{
                    entries.remove(found);
                } else {
                    entries[found].status = args.arg_status;
                }
            },
            Err(why) => panic!("id not found {}",why),
        };
    }

    if args.cmd_list {
        for entry in entries.iter(){
            println!("{}",entry);
        }
        return;
    }
    
    let encoded = json::encode(&entries).unwrap();
    match File::create(filepath) {
        Ok(mut swap_file) => {
            if let Err(why) = swap_file.write_all(encoded.as_bytes()) {
                panic!("errored on write:  {}",why);
            }
        },
        Err(why) => panic!("failed to open file {}",why)
    }
}
