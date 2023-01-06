#![allow(unused)]

use std::{path::PathBuf, str::FromStr, env, io};
use dotenvy::dotenv;
use arboard::Clipboard;

use snippeter::find_path;
use filebase::Filebase;
use snippet::Snippet;

mod filebase;
mod snippet;
mod cli;

fn main() {
    // let filebase_path = find_path();
    // let mut filebase = Filebase::load(&filebase_path).unwrap_or_default();
    // match filebase.search("comp") {
    //     Some(v) => for i in v { println!("{}: {} \n\t {}", i.name, i.desc.unwrap(), i.value)},
    //     None => println!("nada")
    // }
    // filebase.save(&mut filebase_path);
}


//     let mut clipboard = Clipboard::new().unwrap();
//     println!("Clipboard text was: {}", clipboard.get_text().unwrap());

//     let the_string = "Hello, world!";
//     clipboard.set_text(the_string).unwrap();
//     println!("But now the clipboard text should be: \"{}\"", the_string);