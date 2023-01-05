#![allow(unused)]

use std::{path::PathBuf, str::FromStr, env, io};
use dotenvy::dotenv;

use snippeter::find_path;
use filebase::Filebase;
use snippet::Snippet;

mod filebase;
mod snippet;

fn main() {
    let filebase_path = find_path();

    let mut filebase = Filebase::load(&filebase_path).unwrap_or_default();
    
    // match filebase.search("comp") {
    //     Some(v) => for i in v { println!("{}: {} \n\t {}", i.name, i.desc.unwrap(), i.value)},
    //     None => println!("nada")
    // }

    // filebase.save(&mut filebase_path);

}
