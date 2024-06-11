use std::fmt;

use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    title: String,
    description: String,
    notes: Vec<String>,
}

fn write_title(task: &Task, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "    Title: {}\n\n", task.title)?;
    Ok(())
}

fn write_description(task: &Task, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if task.description.len() != 0 {
        return write!(f, "    Description:\n    {}\n\n", task.description);
    }
    Ok(())
}

fn write_notes(task: &Task, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "    Notes:\n")?;
    for i in 0..task.notes.len() {
        write!(f, "        {}) {}\n", i + 1, task.notes[i])?;
    }
    Ok(())
}

impl fmt::Display for Task {
    /*
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_title(self, f)?;
        write_description(self, f)?;
        write_notes(self, f)?;
        Ok(())
    }
    */

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

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
    }
}
