use std::{fmt, fs::File, io::Read};

use serde::{Serialize, Deserialize};

use crate::{error::Error, result::Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    title: String,
    description: String,
    notes: Vec<String>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Ok(json) = self.to_json() {
            return write!(f, "{}", json)
        }
        write!(f, "{}", "<ERORR>")
    }
}

impl Task {
    pub fn new(title: &str) -> Self {
        Task {
            title: String::from(title),
            description: String::new(),
            notes: vec![]
        }
    }

    pub fn from_file(file: &mut File) -> Result<Self> {
        let mut bytes: Vec<u8> = Vec::new();
        file.read_to_end(&mut bytes)?;

        Self::from_bytes(bytes.as_slice())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(|e| Error::from(e))
    }

    pub fn set_description(&mut self, description: &str) -> &mut Self {
        self.description = String::from(description);
        self
    }

    pub fn add_note(&mut self, note: &str) -> &mut Self {
        self.notes.push(String::from(note));
        self
    }

    /// Removes message num. `i` -- not the index!
    pub fn remove_note(&mut self, i: usize) -> &mut Self {
        if i == 0 || i >= self.notes.len() {
            return self;
        }

        self.notes.remove(i - 1);
        self
    }

    pub fn to_md(&self) -> Result<String> {
        Ok(format!(r#"
# {}
## Description
{}
## Notes
{}
                "#, self.title, self.description, self.notes.join("\n")))
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(|e| Error::from(e))
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self).map_err(|e| Error::from(e))
    }
}
