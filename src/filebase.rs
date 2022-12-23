use std::{fs, io, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::snippet::Snippet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Filebase {
    snippets: Vec<Snippet>,
    size: u32,
}

impl Filebase {
    pub fn load(mut path: &PathBuf) -> Result<Self, io::Error> {
        match fs::read(path) {
            Ok(mut file_data) => {
                let filebase: Result<Filebase, Box<bincode::ErrorKind>> =
                    bincode::deserialize(&file_data);

                match filebase {
                    Ok(filebase) => return Ok(filebase),
                    Err(e) => {
                        // TODO: retornar que nao conseguiu deserializar o filebase
                        todo!()
                    }
                }
            }
            Err(e) => {
                // TODO: retornar que nao conseguiu ler o arquivo do filebase
                todo!()
            }
        }
    }

    pub fn save(&self, path: &PathBuf) -> Result<(), io::Error> {
        if let Err(e) = fs::create_dir_all(path) {
            if e.kind() != io::ErrorKind::AlreadyExists {
                return Err(e);
            }
        }

        let file_data = bincode::serialize(self);
        match file_data {
            Ok(data) => {
                return fs::write(path, data);
            }
            Err(e) => {
                // TODO: retornar erro de serialização dos dados
                todo!()
            }
        }
        
        todo!()
    }

    pub fn search(&self, pattern: impl Into<String>) -> Option<Vec<Snippet>>{
        todo!()
    }
}
