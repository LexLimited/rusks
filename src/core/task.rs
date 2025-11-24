use std::{
    fmt::{self, Display},
    fs::File,
    io::Read,
};

use serde::{Deserialize, Serialize};

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
            return write!(f, "{}", json);
        }
        write!(f, "{}", "<ERORR>")
    }
}

impl Task {
    pub fn new(title: &str) -> Self {
        Task {
            title: String::from(title),
            description: String::new(),
            notes: vec![],
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
        let mut md_string = String::new();
        md_string.push_str(&format!("# {}\n", self.title));
        md_string.push_str(&format!("* **Description**\n{}\n", self.description));

        if self.notes.len() > 0 {
            md_string.push_str("* **Notes**\n\n");

            for (i, note) in self.notes.iter().enumerate() {
                md_string.push_str(&format!("  {}) {}\n", i + 1, note));
            }
        }

        Ok(md_string)
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(|e| Error::from(e))
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self).map_err(|e| Error::from(e))
    }
}

#[derive(Clone)]
pub struct TaskId(u64);

impl Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TaskId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != size_of::<u64>() {
            Err(Error::Reason {
                reason: "Invalid slice length".into(),
            })
        } else {
            unsafe {
                let ptr = std::ptr::from_ref(bytes) as *const u64;
                Ok(Self(*ptr))
            }
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(&self.0 as *const u64 as *const u8, size_of::<u64>()) }
    }
}
