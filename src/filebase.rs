use std::{fs, io, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::snippet::Snippet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Filebase {
    snippets: Vec<Snippet>,
    size: u32,
}

impl Filebase {
    pub fn load(mut path: PathBuf) -> Result<Self, io::Error> {
        path.push("snippets.sn");
        match fs::read(path) {
            Ok(mut file_data) => {
                let filebase: Result<Filebase, Box<bincode::ErrorKind>> =
                    bincode::deserialize(&file_data);

                if let Ok(filebase) = filebase {
                    return Ok(filebase);
                }
            }
            Err(e) => {}
        }

        todo!()
    }
}
