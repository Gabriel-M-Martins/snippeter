use std::{path::PathBuf, io, str::FromStr};

use dotenvy::dotenv;

pub fn find_path() -> PathBuf {
    dotenv().ok();

    let filebase_path: PathBuf;
    
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

    filebase_path
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