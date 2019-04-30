use std::cmp::Ordering;

use chrono::{DateTime, Utc};
use libnest::package::PackageManifest;
use serde_derive::{Deserialize, Serialize};

static HISTORY_SIZE: usize = 15;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct HistoryEntry {
    date: DateTime<Utc>,
    manifest: PackageManifest,
}

impl HistoryEntry {
    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }

    pub fn manifest(&self) -> &PackageManifest {
        &self.manifest
    }
}

impl Ord for HistoryEntry {
    fn cmp(&self, other: &HistoryEntry) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialOrd for HistoryEntry {
    fn partial_cmp(&self, other: &HistoryEntry) -> Option<Ordering> {
        Some(self.date.cmp(&other.date))
    }
}

/// A wrapper around a sorted vector of [`HistoryEntry`].
///
/// Note: there is currently no way to remove an entry from the history.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct History {
    entries: Vec<HistoryEntry>,
}

impl History {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn entries(&self) -> &Vec<HistoryEntry> {
        &self.entries
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn add_manifest(&mut self, manifest: PackageManifest) {
        let last_version = manifest
            .versions()
            .iter()
            .max_by(|(_, a), (_, b)| a.wrap_date().cmp(b.wrap_date()));

        if let Some((_, version_data)) = last_version {
            let full_name = manifest.full_name();

            // Remove any entries with the same name than the given package
            self.entries
                .retain(|entry| entry.manifest().full_name() != full_name);

            let entry = HistoryEntry {
                date: version_data.wrap_date().clone(),
                manifest: manifest.clone(),
            };

            match self
                .entries
                .binary_search_by(|e| version_data.wrap_date().cmp(e.date()))
            {
                Ok(idx) => self.entries.insert(idx, entry),
                Err(idx) => self.entries.insert(idx, entry),
            }

            // Remove any extra entry
            if self.entries.len() > HISTORY_SIZE {
                self.entries.resize_with(HISTORY_SIZE, || unreachable!());
            }
        }
    }
}
