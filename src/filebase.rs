use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::snippet::Snippet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Filebase {
    snippets: Vec<Snippet>,
    size: u32,
}

impl Filebase {
    pub fn load(mut path: PathBuf) -> Self {
        todo!()
    }
}
