/*
  Problem 35: Mini Phonebook

  Build a Phonebook struct backed by a HashMap<String, String> (name -> phone).
  Implement methods new, add, lookup, and remove.
  Also implement the Display trait to print all entries sorted by name, one per line
  as "Name: Phone".

  Run the tests for this problem with:
    cargo test --test mini_phonebook_test
*/

use std::collections::HashMap;
use std::fmt::{self, Debug};

pub struct Phonebook {
    pub entries: HashMap<String, String>,
}

impl Phonebook {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, phone: &str) {
        self.entries.insert(name.to_string(), phone.to_string());
    }

    pub fn lookup(&self, name: &str) -> Option<&String> {
        self.entries.get(name)
    }

    pub fn remove(&mut self, name: &str) -> bool {
        self.entries.remove(name).is_some()
    }
}

impl fmt::Display for Phonebook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut vec: Vec<_> = self.entries.iter().collect();
        vec.sort_by(|a, b| a.0.cmp(b.0));
        for (i, (name, phone)) in vec.iter().enumerate() {
            if i > 0 {
                writeln!(f);
            }
            write!(f, "{}: {}", name, phone);
            
        }
        Ok(())
    }
}
