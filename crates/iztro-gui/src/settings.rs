//! Local persistence of GUI application settings.
//!
//! Settings are kept deliberately separate from saved charts: charts live in
//! [`ChartStore`](crate::persistence::ChartStore) (`charts.json`), while user
//! preferences — display locale and right-inspector layout — live in a distinct
//! [`SettingsStore`] (`settings.json`) under the same data directory. Mixing the
//! two would couple unrelated concerns and risk a charts migration clobbering
//! preferences (or vice versa).
//!
//! The persistence policy mirrors [`ChartStore`]: the path is injectable for
//! tests, there is no current-directory fallback, and a missing or corrupt file
//! loads as [`AppSettings::default`] rather than panicking. New fields can be
//! added safely because every field carries a serde default.

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use iztro_i18n::Locale;

/// Data sub-directory shared with the chart store.
const STORE_DIR: &str = "iztro-gui";
/// On-disk file name for persisted settings.
const STORE_FILE: &str = "settings.json";

/// Visibility/width mode of the right-side inspector panel.
///
/// Defaults to [`RightPanelMode::Compact`] — the inspector starts visible but
/// narrow, not hidden, so the analysis surface is discoverable on first run.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Default)]
pub enum RightPanelMode {
    /// The inspector is removed from the layout entirely.
    Hidden,
    /// A narrow inspector showing compact, collapsed lines.
    #[default]
    Compact,
    /// A wider inspector with more room for expanded details.
    Expanded,
}

/// Selects the GUI visual theme (palette + design tokens).
///
/// Only [`GuiThemeId::InkPaper`] is implemented today; the enum is the extension
/// point for future themes (JadeLight, DeepInk, …). It is a stable internal key,
/// never a localized display string, so settings files round-trip safely. A
/// missing or unknown theme field deserializes to the default via `#[serde(other)]`
/// / serde defaults, so older settings files keep loading.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Default)]
pub enum GuiThemeId {
    /// Warm paper background, ivory palace cards, deep-purple primary accents.
    #[default]
    InkPaper,
}

/// Which inspector tab is active. Defaults to [`RightPanelTab::QuanShuRules`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Default)]
pub enum RightPanelTab {
    /// 全书规则 — classical QuanShu rule hits.
    #[default]
    QuanShuRules,
    /// 格局 — detected patterns.
    Patterns,
    /// 设置 — application settings.
    Settings,
}

/// Persisted user preferences for the GUI.
///
/// Every field carries `#[serde(default)]` so a settings file written by an
/// older build (missing newer fields) still loads, with absent fields filling
/// in from [`Default`].
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    /// Display locale.
    #[serde(default)]
    pub locale: Locale,
    /// Right inspector visibility/width mode.
    #[serde(default)]
    pub right_panel_mode: RightPanelMode,
    /// Active right inspector tab.
    #[serde(default)]
    pub right_panel_tab: RightPanelTab,
    /// Active GUI visual theme.
    #[serde(default)]
    pub theme: GuiThemeId,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            locale: Locale::default(),
            right_panel_mode: RightPanelMode::Compact,
            right_panel_tab: RightPanelTab::QuanShuRules,
            theme: GuiThemeId::InkPaper,
        }
    }
}

/// A file-backed store of [`AppSettings`].
///
/// Reads are tolerant: a missing or corrupt file loads as
/// [`AppSettings::default`] rather than panicking, so damaged settings never
/// block startup.
#[derive(Clone, Debug)]
pub struct SettingsStore {
    path: PathBuf,
}

impl SettingsStore {
    /// Builds a store backed by an explicit file path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// The conventional per-user store at
    /// `<data_local_dir>/iztro-gui/settings.json`, or `None` when no local data
    /// directory is known.
    ///
    /// As with [`ChartStore`](crate::persistence::ChartStore) there is no
    /// current-directory fallback: settings should never be scattered into
    /// whatever working directory the GUI launched from. When this returns
    /// `None` the GUI runs with in-memory defaults instead.
    pub fn default_store() -> Option<Self> {
        let base = dirs::data_local_dir()?;
        Some(Self::new(base.join(STORE_DIR).join(STORE_FILE)))
    }

    /// The backing file path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Loads the settings, returning [`AppSettings::default`] if the file is
    /// missing or cannot be parsed. Never panics.
    pub fn load(&self) -> AppSettings {
        let Ok(text) = fs::read_to_string(&self.path) else {
            return AppSettings::default();
        };
        serde_json::from_str(&text).unwrap_or_default()
    }

    /// Persists the settings as pretty JSON, creating the parent directory if
    /// needed.
    pub fn save(&self, settings: &AppSettings) -> std::io::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut text = serde_json::to_string_pretty(settings)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
        text.push('\n');
        fs::write(&self.path, text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loading_a_missing_file_returns_default() {
        let dir = tempfile::tempdir().expect("temp dir");
        let store = SettingsStore::new(dir.path().join("does-not-exist.json"));
        assert_eq!(store.load(), AppSettings::default());
    }

    #[test]
    fn loading_corrupt_json_returns_default_without_panicking() {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("settings.json");
        fs::write(&path, "{ not valid json ]").expect("write corrupt file");
        let store = SettingsStore::new(path);
        assert_eq!(store.load(), AppSettings::default());
    }

    #[test]
    fn save_then_load_roundtrips() {
        let dir = tempfile::tempdir().expect("temp dir");
        let store = SettingsStore::new(dir.path().join("nested").join("settings.json"));
        let settings = AppSettings {
            locale: Locale::ZhHans,
            right_panel_mode: RightPanelMode::Expanded,
            right_panel_tab: RightPanelTab::Patterns,
            theme: GuiThemeId::InkPaper,
        };

        store.save(&settings).expect("save should succeed");
        assert_eq!(store.load(), settings);
    }

    #[test]
    fn default_settings_use_ink_paper_theme() {
        assert_eq!(AppSettings::default().theme, GuiThemeId::InkPaper);
    }

    #[test]
    fn missing_theme_field_deserializes_to_ink_paper() {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("settings.json");
        // A settings file written before the theme field existed must still load,
        // filling the theme in from its serde default.
        fs::write(
            &path,
            r#"{ "locale": "zh-Hans", "right_panel_mode": "Expanded" }"#,
        )
        .expect("write file");
        let store = SettingsStore::new(path);
        let loaded = store.load();
        assert_eq!(loaded.theme, GuiThemeId::InkPaper);
        assert_eq!(loaded.locale, Locale::ZhHans);
    }

    #[test]
    fn theme_setting_roundtrips_through_the_store() {
        let dir = tempfile::tempdir().expect("temp dir");
        let store = SettingsStore::new(dir.path().join("settings.json"));
        let settings = AppSettings {
            theme: GuiThemeId::InkPaper,
            ..AppSettings::default()
        };
        store.save(&settings).expect("save");
        assert_eq!(store.load().theme, GuiThemeId::InkPaper);
    }

    #[test]
    fn missing_fields_deserialize_through_serde_defaults() {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("settings.json");
        // A minimal object missing every field must fill in from `Default`.
        fs::write(&path, "{}").expect("write partial file");
        let store = SettingsStore::new(path);
        assert_eq!(store.load(), AppSettings::default());
    }

    #[test]
    fn missing_one_field_keeps_the_others() {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("settings.json");
        // Locale present, panel fields absent: the present field is honored and
        // the absent ones fall back to defaults.
        fs::write(&path, r#"{ "locale": "zh-Hans" }"#).expect("write file");
        let store = SettingsStore::new(path);
        let loaded = store.load();
        assert_eq!(loaded.locale, Locale::ZhHans);
        assert_eq!(loaded.right_panel_mode, RightPanelMode::default());
        assert_eq!(loaded.right_panel_tab, RightPanelTab::default());
    }

    #[test]
    fn locale_serializes_as_its_bcp47_tag() {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("settings.json");
        let store = SettingsStore::new(path.clone());
        store
            .save(&AppSettings {
                locale: Locale::ZhHans,
                ..AppSettings::default()
            })
            .expect("save");

        let text = fs::read_to_string(&path).expect("read back");
        // The persisted locale is the stable tag, not the Rust variant name.
        assert!(text.contains("zh-Hans"));
        assert!(!text.contains("ZhHans"));
    }

    #[test]
    fn default_store_targets_the_conventional_settings_file_when_available() {
        // On a host with a local data directory the store targets the
        // conventional file; on a host without one it is `None`. Never the cwd.
        if let Some(store) = SettingsStore::default_store() {
            assert!(store.path().ends_with("iztro-gui/settings.json"));
            assert_ne!(store.path(), Path::new("iztro-gui/settings.json"));
        }
    }

    #[test]
    fn defaults_match_the_documented_panel_state() {
        let settings = AppSettings::default();
        assert_eq!(settings.locale, Locale::EnUs);
        assert_eq!(settings.right_panel_mode, RightPanelMode::Compact);
        assert_eq!(settings.right_panel_tab, RightPanelTab::QuanShuRules);
        assert_eq!(settings.theme, GuiThemeId::InkPaper);
    }
}
