use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Snippet {
    pub name: String,
    pub desc: Option<String>,
    pub value: String,
    pub times_used: u32,
}

impl Snippet {
    pub fn new(name: String) -> Self {
        Snippet {
            name,
            desc: None,
            value: String::new(),
            times_used: 0,
        }
    }

    pub fn desc(&mut self, desc: impl Into<String>) -> &mut Self {
        self.desc = Some(desc.into());
        self
    }

    pub fn value(&mut self, value: impl Into<String>) -> &mut Self {
        self.value = value.into();
        self
    }
}