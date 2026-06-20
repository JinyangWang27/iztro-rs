//! Local persistence of generated birth charts.
//!
//! The GUI persists only named [`SavedChart`] records (a display name plus the
//! normalized [`BirthInput`]), never rendered widgets or derived astrology
//! facts: a saved chart is deterministically rebuilt from its input through the
//! `by_solar` facade. The on-disk format is a JSON array, and the persistence
//! boundary is an explicit, path-injectable [`ChartStore`] so tests never touch
//! a real home directory.
//!
//! Loads are backward compatible: the legacy format was a plain array of
//! [`BirthInput`]; such files are migrated in memory to named records using
//! [`default_chart_name`].

use std::fs;
use std::path::{Path, PathBuf};

use crate::app::{BirthInput, SavedChart, default_chart_name};
use iztro_i18n::Locale;

/// Default on-disk file name for saved charts under the data directory.
const STORE_DIR: &str = "iztro-gui";
const STORE_FILE: &str = "charts.json";

/// A file-backed store of saved [`SavedChart`] records.
///
/// Reads are tolerant: a missing or corrupt file loads as an empty list rather
/// than panicking, so a damaged store never blocks startup.
#[derive(Clone, Debug)]
pub struct ChartStore {
    path: PathBuf,
}

impl ChartStore {
    /// Builds a store backed by an explicit file path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// The conventional per-user store at
    /// `<data_local_dir>/iztro-gui/charts.json`, or `None` when no local data
    /// directory is known.
    ///
    /// There is deliberately no current-directory fallback: silently writing a
    /// user's saved charts into whatever working directory the GUI happened to
    /// launch from would scatter and leak them. When this returns `None` the GUI
    /// runs without persistence instead.
    pub fn default_store() -> Option<Self> {
        let base = dirs::data_local_dir()?;
        Some(Self::new(base.join(STORE_DIR).join(STORE_FILE)))
    }

    /// The backing file path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Loads the saved charts, returning an empty list if the file is missing or
    /// cannot be parsed. Never panics.
    ///
    /// The new named format ([`Vec<SavedChart>`]) is tried first; on failure the
    /// legacy [`Vec<BirthInput>`] format is parsed and migrated in memory to
    /// named records via [`default_chart_name`]; if both fail the list is empty.
    pub fn load(&self) -> Vec<SavedChart> {
        let Ok(text) = fs::read_to_string(&self.path) else {
            return Vec::new();
        };
        if let Ok(charts) = serde_json::from_str::<Vec<SavedChart>>(&text) {
            return charts;
        }
        match serde_json::from_str::<Vec<BirthInput>>(&text) {
            Ok(legacy) => legacy
                .into_iter()
                .map(|input| SavedChart {
                    name: default_chart_name(&input, Locale::EnUs),
                    input,
                })
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    /// Persists the saved charts as pretty JSON in the named format, creating the
    /// parent directory if needed.
    pub fn save(&self, charts: &[SavedChart]) -> std::io::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut text = serde_json::to_string_pretty(charts)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
        text.push('\n');
        fs::write(&self.path, text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iztro::core::Gender;

    fn sample(year: i32) -> BirthInput {
        BirthInput {
            year,
            month: 5,
            day: 17,
            time_index: 4,
            gender: Gender::Female,
        }
    }

    fn named(year: i32, name: &str) -> SavedChart {
        SavedChart {
            name: name.to_owned(),
            input: sample(year),
        }
    }

    #[test]
    fn save_then_load_roundtrips() {
        let dir = tempfile::tempdir().expect("temp dir");
        let store = ChartStore::new(dir.path().join("nested").join("charts.json"));
        let charts = vec![named(1990, "甲"), named(2000, "乙")];

        store.save(&charts).expect("save should succeed");
        assert_eq!(store.load(), charts);
    }

    #[test]
    fn loading_the_legacy_birth_input_array_migrates_to_named_records() {
        // The legacy format was a plain array of `BirthInput`. Such a file must
        // load as named records using the generated default name.
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("charts.json");
        let legacy = vec![sample(1990), sample(2000)];
        fs::write(&path, serde_json::to_string_pretty(&legacy).unwrap())
            .expect("write legacy file");
        let store = ChartStore::new(path);

        let loaded = store.load();
        assert_eq!(
            loaded,
            legacy
                .into_iter()
                .map(|input| SavedChart {
                    name: default_chart_name(&input, Locale::EnUs),
                    input,
                })
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn saving_always_writes_the_new_named_format() {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("charts.json");
        let store = ChartStore::new(path.clone());
        store.save(&[named(1990, "甲")]).expect("save");

        let text = fs::read_to_string(&path).expect("read back");
        // The named format serializes name + input objects, not a bare array of
        // birth inputs.
        assert!(text.contains("\"name\""));
        assert!(text.contains("\"input\""));
    }

    #[test]
    fn loading_a_missing_file_returns_empty() {
        let dir = tempfile::tempdir().expect("temp dir");
        let store = ChartStore::new(dir.path().join("does-not-exist.json"));
        assert!(store.load().is_empty());
    }

    #[test]
    fn loading_corrupt_json_returns_empty_without_panicking() {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("charts.json");
        fs::write(&path, "{ not valid json ]").expect("write corrupt file");
        let store = ChartStore::new(path);
        assert!(store.load().is_empty());
    }

    #[test]
    fn default_store_targets_the_conventional_store_file_when_available() {
        // On a host with a local data directory the store targets the
        // conventional file; on a host without one it is `None`. Never the cwd.
        if let Some(store) = ChartStore::default_store() {
            assert!(store.path().ends_with("iztro-gui/charts.json"));
            assert_ne!(store.path(), Path::new("iztro-gui/charts.json"));
        }
    }
}
