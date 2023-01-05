use std::{fs, io, path::PathBuf};

use fuse_rust::Fuse;
use serde::{Deserialize, Serialize};

use crate::snippet::Snippet;
use snippeter::find_path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Filebase {
    snippets: Vec<Snippet>,
    size: u32,
    path: PathBuf
}

impl Default for Filebase {
    fn default() -> Self {
        Filebase { snippets: vec![], size: 0, path: find_path()}
    }
}

impl Filebase {
    pub fn load(path: &PathBuf) -> io::Result<Self> {
        match fs::read(path) {
            Ok(mut file_data) => {
                let filebase: Result<Filebase, Box<bincode::ErrorKind>> =
                    bincode::deserialize(&file_data);

                match filebase {
                    Ok(filebase) => return Ok(filebase),
                    Err(e) => {
                        // todo: retornar que nao conseguiu deserializar o filebase
                        todo!()
                    }
                }
            }
            Err(e) => {
                // todo: retornar que nao conseguiu ler o arquivo do filebase
                
                match e.kind() {
                    io::ErrorKind::NotFound => { return Ok(Filebase::default()) }
                    _ => { todo!() }
                }
            }
        }
    }

    pub fn save(&self) -> io::Result<()> {
        let mut dir_path = self.path.clone();
        dir_path.pop();
        if let Err(e) = fs::create_dir_all(dir_path) {
            if e.kind() != io::ErrorKind::AlreadyExists {
                return Err(e);
            }
        }

        let file_data = bincode::serialize(self);
        match file_data {
            Ok(data) => {
                return fs::write(&self.path, data);
            }
            Err(e) => {
                // todo: retornar erro de serialização dos dados
                todo!()
            }
        }
    }

    pub fn delete_all(&self) -> io::Result<()> {
        fs::remove_file(&self.path)
    }

    pub fn search(&self, pattern: impl Into<String>) -> Option<Vec<Snippet>>{
        let mut fuse = Fuse::default();
        fuse.threshold = 0.4;
        
        let mut search_results = fuse.search_text_in_fuse_list(&pattern.into(), &self.snippets);

        if search_results.is_empty() {
            return None;
        }

        let mut output: Vec<Snippet> = vec![];
        search_results.sort_unstable_by(|a,b| a.score.total_cmp(&b.score));
        
        for i in search_results {
            output.push(self.snippets[i.index].clone());
        }

        Some(output)
    }

    pub fn add_snippet(&mut self, snippet: Snippet) {
        self.snippets.push(snippet);
        self.size += 1;
    }
}
