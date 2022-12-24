#![allow(unused)]

use std::{path::PathBuf, str::FromStr};

use filebase::Filebase;
use snippet::Snippet;

mod filebase;
mod snippet;

fn main() {
    dotenv::dotenv().ok();
    let mut vars = dotenv::vars();

    let filebase_path: String;
    match vars.find(|(key, value)| key == "FILEBASE_PATH") {
        Some((_, value)) => filebase_path = value,
        None => {
            // todo: (tentar) usar um caminho default. por enquanto é hardcoded
            
            filebase_path = String::from("C:\\Users\\Gabriel\\Documents\\snippets");
        }
    }

    let mut filebase: Filebase;
    match PathBuf::from_str(filebase_path.as_str()) {
        Ok(mut path) => {
            path.push("snippets.snp");
            match Filebase::load(&path) {
                Ok(value) => {
                    filebase = value.unwrap_or_default();
                }
                Err(e) => {
                    // todo: avisar que não conseguiu carregar o snippeter e perguntar se deveria criar um novo
                    todo!()
                }
            }
        },
            
        Err(e) => {
            // todo: (tentar) usar um caminho default
            todo!()
        }
    }
}
