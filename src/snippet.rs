use fuse_rust::{Fuseable, FuseProperty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Snippet {
    pub name: String,
    pub desc: Option<String>,
    pub value: String,
    pub times_used: u32,
}

impl Snippet {
    pub fn default() -> Self {
        Snippet {
            name: String::new(),
            desc: None,
            value: String::new(),
            times_used: 0,
        }
    }

    pub fn name(&mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self.to_owned()
    }

    pub fn desc(&mut self, desc: impl Into<String>) -> Self {
        self.desc = Some(desc.into());
        self.to_owned()
    }

    pub fn value(&mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self.to_owned()
    }
}

impl Fuseable for Snippet {
    fn properties(&self) -> Vec<fuse_rust::FuseProperty> {
        return vec![
            FuseProperty{value: String::from("name"), weight: 0.6},
            FuseProperty{value: String::from("desc"), weight: 0.3},
            FuseProperty{value: String::from("value"), weight: 0.1}
        ]
    }

    fn lookup(&self, key: &str) -> Option<&str> {
        return match key {
            "name" => Some(&self.name),
            "desc" => {
                match &self.desc {
                    Some(desc) => return Some(desc),
                    None => return None

                }
            },
            "value" => Some(&self.value),
            _ => None
        };
    }
}