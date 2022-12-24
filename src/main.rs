#![allow(unused)]

use std::{path::PathBuf, str::FromStr, env, io};

use filebase::Filebase;
use snippet::Snippet;

use dotenvy::dotenv;

mod filebase;
mod snippet;

fn main() {
    dotenv().ok();

    let mut filebase_path: PathBuf;
    
    match dotenvy::vars().find(|(key, _)| key == "FILEBASE_PATH") {
        Some((_, value)) => {
            match PathBuf::from_str(&value) {
                Ok(v) => filebase_path = v,
                Err(_) => filebase_path = default_path().unwrap()
            }
        }
        None => {
            filebase_path = default_path().unwrap();
        }
    }

    let mut filebase = Filebase::load(&filebase_path).unwrap_or_default();
    
    // match filebase.search("comp") {
    //     Some(v) => for i in v { println!("{}: {} \n\t {}", i.name, i.desc.unwrap(), i.value)},
    //     None => println!("nada")
    // }

    // filebase.save(&mut filebase_path);

}

fn default_path() -> io::Result<PathBuf> {
    match dirs::document_dir() {
        Some(mut p) => {
            p.push("snippets1/snippets.snp");
            Ok(p)
        },
        None => Err(io::Error::new(io::ErrorKind::NotFound, "There is something wrong with .env 'FILEBASE_PATH' and can't find default document directory."))
    }
}