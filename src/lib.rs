// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-bazel: bazel component model.
//!
//! Real implementation: bazel_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BazelItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BazelError {
    EmptyBazelItemName,
    DuplicateBazelItem(String),
    EmptyEventSubject,
}

impl fmt::Display for BazelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyBazelItemName => write!(f, "bazel_item name must not be empty"),
            Self::DuplicateBazelItem(id) => write!(f, "duplicate bazel_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for BazelError {}

pub struct Bazel {
    pub name: String,
    bazel_items: BTreeMap<String, BazelItem>,
    events: BTreeMap<String, Event>,
}

impl Bazel {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            bazel_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_bazel_item(&mut self, item: BazelItem) -> Result<(), BazelError> {
        if item.name.trim().is_empty() {
            return Err(BazelError::EmptyBazelItemName);
        }
        let id = item.id.clone();
        if self.bazel_items.contains_key(&id) {
            return Err(BazelError::DuplicateBazelItem(id.clone()));
        }
        self.bazel_items.insert(id, item);
        Ok(())
    }

    pub fn bazel_item_count(&self) -> usize {
        self.bazel_items.len()
    }

    pub fn get_bazel_item(&self, id: &str) -> Option<&BazelItem> {
        self.bazel_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), BazelError> {
        if event.subject.trim().is_empty() {
            return Err(BazelError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_bazel_item(id: &str, name: &str) -> BazelItem {
        BazelItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Bazel::new("Bazel");
        assert_eq!(m.name, "Bazel");
        assert_eq!(m.bazel_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_bazel_item() {
        let mut m = Bazel::new("Bazel");
        m.add_bazel_item(sample_bazel_item("a1", "Alpha")).unwrap();
        assert_eq!(m.bazel_item_count(), 1);
        assert!(m.get_bazel_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Bazel::new("Bazel");
        assert!(m.add_bazel_item(sample_bazel_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Bazel::new("Bazel");
        m.add_bazel_item(sample_bazel_item("x", "One")).unwrap();
        assert!(m.add_bazel_item(sample_bazel_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Bazel::new("Bazel");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
