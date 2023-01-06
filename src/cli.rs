use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct CLI {
    #[command(subcommand)]
    cmd: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add new snippet with Name and Value. Description field is optional.
    Add {
        name: String,
        value: String,
        desc: Option<String>
    },
    /// Edit an already existing snippet.
    Edit {
        id: Option<u32>,
        name: Option<String>,
        new_name: String,
        new_value: String,
        desc: Option<String>
    },
    /// Delete a snippet or the filebase altogether. Deleting a snippet requires an id OR a name.
    Delete {
        filebase: bool,
        id: Option<u32>,
        name: Option<String>
    },
}