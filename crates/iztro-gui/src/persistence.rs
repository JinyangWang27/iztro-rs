//! Local persistence of generated birth charts.
//!
//! The GUI persists only named [`SavedChart`] records (a display name plus the
//! normalized [`BirthInput`]), never rendered widgets or derived astrology
//! facts: a saved chart is deterministically rebuilt from its input through the
//! `by_solar` facade. The on-disk format is a JSON array, and the persistence
//! boundary is an explicit, path-injectable [`ChartStore`] so tests never touch
//! a real home directory.
//!
//! Loads are backward compatible. The current on-disk schema is a tagged
//! [`BirthInput`] (one per input mode); older files are migrated in memory:
//! a bare flat array of pre-mode birth inputs becomes named records via
//! [`default_chart_name`], and named records with a flat `input` keep their
//! name. Both migrate the input to [`BirthInput::SolarKnownTimeBranch`].

use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::app::{BirthInput, SavedChart, SolarKnownTimeBranchBirthInput, default_chart_name};
use iztro::core::Gender;
use iztro_i18n::Locale;

/// The pre-mode legacy on-disk birth input: a flat solar date plus a known
/// 时辰. Kept only to migrate old saved files; new records use the tagged
/// [`BirthInput`] schema.
#[derive(Clone, Copy, Debug, Deserialize)]
struct LegacyBirthInput {
    year: i32,
    month: u8,
    day: u8,
    time_index: u8,
    gender: Gender,
}

impl LegacyBirthInput {
    /// Migrates a legacy flat record to the tagged known-time-branch variant.
    ///
    /// Old records carried only a date + 时辰 + gender, so they map cleanly onto
    /// [`BirthInput::SolarKnownTimeBranch`]. Clock time, UTC offset, and longitude
    /// are deliberately *not* invented for a record that never had them.
    fn migrate(self) -> BirthInput {
        BirthInput::SolarKnownTimeBranch(SolarKnownTimeBranchBirthInput {
            year: self.year,
            month: self.month,
            day: self.day,
            time_index: self.time_index,
            gender: self.gender,
        })
    }
}

/// A legacy named saved record whose `input` is the pre-mode flat schema.
#[derive(Clone, Debug, Deserialize)]
struct LegacySavedChart {
    name: String,
    input: LegacyBirthInput,
}

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
    /// The new tagged named format ([`Vec<SavedChart>`]) is tried first. On
    /// failure the pre-mode legacy formats are migrated in memory:
    ///
    /// 1. named records with a flat `input` ([`Vec<LegacySavedChart>`]) keep their
    ///    name and migrate the input to [`BirthInput::SolarKnownTimeBranch`];
    /// 2. a bare flat array ([`Vec<LegacyBirthInput>`]) migrates each record and
    ///    generates a default name via [`default_chart_name`].
    ///
    /// If nothing parses the list is empty. Migration never invents clock time,
    /// UTC offset, or longitude for a record that never carried them.
    pub fn load(&self) -> Vec<SavedChart> {
        let Ok(text) = fs::read_to_string(&self.path) else {
            return Vec::new();
        };
        if let Ok(charts) = serde_json::from_str::<Vec<SavedChart>>(&text) {
            return charts;
        }
        if let Ok(legacy) = serde_json::from_str::<Vec<LegacySavedChart>>(&text) {
            return legacy
                .into_iter()
                .map(|record| SavedChart {
                    name: record.name,
                    input: record.input.migrate(),
                })
                .collect();
        }
        match serde_json::from_str::<Vec<LegacyBirthInput>>(&text) {
            Ok(legacy) => legacy
                .into_iter()
                .map(|legacy| {
                    let input = legacy.migrate();
                    SavedChart {
                        name: default_chart_name(&input, Locale::EnUs),
                        input,
                    }
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
    use crate::app::{GuiSolarTimePolicy, SolarClockBirthInput};
    use iztro::core::Gender;

    /// A known-time-branch sample input (the default input mode).
    fn sample(year: i32) -> BirthInput {
        BirthInput::SolarKnownTimeBranch(SolarKnownTimeBranchBirthInput {
            year,
            month: 5,
            day: 17,
            time_index: 4,
            gender: Gender::Female,
        })
    }

    /// A clock-time sample input with apparent-solar-time correction enabled.
    fn clock_sample(year: i32) -> BirthInput {
        BirthInput::SolarClock(SolarClockBirthInput {
            year,
            month: 1,
            day: 1,
            clock_hour: 1,
            clock_minute: 5,
            utc_offset_minutes: 8 * 60,
            solar_time_policy: GuiSolarTimePolicy::ApparentSolarTime {
                longitude_micro_degrees: 105_000_000,
            },
            gender: Gender::Male,
        })
    }

    fn named(year: i32, name: &str) -> SavedChart {
        SavedChart {
            name: name.to_owned(),
            input: sample(year),
        }
    }

    /// The pre-mode legacy on-disk record JSON: a bare flat birth input object.
    fn legacy_flat_json(year: i32) -> String {
        format!(r#"{{"year":{year},"month":5,"day":17,"time_index":4,"gender":"female"}}"#)
    }

    /// The migration target for [`legacy_flat_json`]: a known-time-branch record.
    fn migrated(year: i32) -> BirthInput {
        BirthInput::SolarKnownTimeBranch(SolarKnownTimeBranchBirthInput {
            year,
            month: 5,
            day: 17,
            time_index: 4,
            gender: Gender::Female,
        })
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
    fn new_tagged_format_roundtrips_both_input_modes() {
        let dir = tempfile::tempdir().expect("temp dir");
        let store = ChartStore::new(dir.path().join("charts.json"));
        let charts = vec![
            SavedChart {
                name: "钟表".to_owned(),
                input: clock_sample(2000),
            },
            named(1990, "时辰"),
        ];

        store.save(&charts).expect("save should succeed");
        assert_eq!(store.load(), charts);
    }

    #[test]
    fn loading_a_bare_legacy_flat_array_migrates_to_named_known_time_branch() {
        // The oldest format was a bare array of flat birth inputs. Such a file
        // must load as named records whose input is the known-time-branch variant.
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("charts.json");
        let json = format!("[{},{}]", legacy_flat_json(1990), legacy_flat_json(2000));
        fs::write(&path, json).expect("write legacy file");
        let store = ChartStore::new(path);

        let loaded = store.load();
        let expected: Vec<SavedChart> = [1990, 2000]
            .into_iter()
            .map(|year| {
                let input = migrated(year);
                SavedChart {
                    name: default_chart_name(&input, Locale::EnUs),
                    input,
                }
            })
            .collect();
        assert_eq!(loaded, expected);
        assert!(matches!(
            loaded[0].input,
            BirthInput::SolarKnownTimeBranch(_)
        ));
    }

    #[test]
    fn loading_a_legacy_named_flat_record_migrates_to_known_time_branch() {
        // A named record whose `input` is the pre-mode flat schema keeps its name
        // and migrates the input to the known-time-branch variant.
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("charts.json");
        let json = format!(
            r#"[{{"name":"旧命盘","input":{}}}]"#,
            legacy_flat_json(1985)
        );
        fs::write(&path, json).expect("write legacy named file");
        let store = ChartStore::new(path);

        let loaded = store.load();
        assert_eq!(
            loaded,
            vec![SavedChart {
                name: "旧命盘".to_owned(),
                input: migrated(1985),
            }]
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
