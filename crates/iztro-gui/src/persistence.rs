//! Local persistence of generated birth charts.
//!
//! The GUI persists only the normalized [`BirthInput`] records that produced a
//! chart, never rendered widgets or derived astrology facts: a saved chart is
//! deterministically rebuilt from its input through the `by_solar` facade. The
//! on-disk format is a simple JSON array, and the persistence boundary is an
//! explicit, path-injectable [`ChartStore`] so tests never touch a real home
//! directory.

use std::fs;
use std::path::{Path, PathBuf};

use crate::app::BirthInput;

/// Default on-disk file name for saved charts under the data directory.
const STORE_DIR: &str = "iztro-gui";
const STORE_FILE: &str = "charts.json";

/// A file-backed store of saved [`BirthInput`] records.
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

    /// The conventional per-user store at `<data_dir>/iztro-gui/charts.json`,
    /// falling back to the current directory when no data directory is known.
    pub fn default_path() -> Self {
        let base = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
        Self::new(base.join(STORE_DIR).join(STORE_FILE))
    }

    /// The backing file path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Loads the saved charts, returning an empty list if the file is missing or
    /// cannot be parsed. Never panics.
    pub fn load(&self) -> Vec<BirthInput> {
        let Ok(text) = fs::read_to_string(&self.path) else {
            return Vec::new();
        };
        serde_json::from_str(&text).unwrap_or_default()
    }

    /// Persists the saved charts as pretty JSON, creating the parent directory
    /// if needed.
    pub fn save(&self, charts: &[BirthInput]) -> std::io::Result<()> {
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

    #[test]
    fn save_then_load_roundtrips() {
        let dir = tempfile::tempdir().expect("temp dir");
        let store = ChartStore::new(dir.path().join("nested").join("charts.json"));
        let charts = vec![sample(1990), sample(2000)];

        store.save(&charts).expect("save should succeed");
        assert_eq!(store.load(), charts);
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
    fn default_path_targets_the_conventional_store_file() {
        let store = ChartStore::default_path();
        assert!(store.path().ends_with("iztro-gui/charts.json"));
    }
}
