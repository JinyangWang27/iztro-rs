//! Pure application state and logic for the static chart GUI.
//!
//! This module is renderer-agnostic: it depends only on `iztro` facade APIs and
//! read models, never on `iced`. It owns the birth-input form, builds charts
//! through the public `static_temporal_chart_view` facade, caches the resulting
//! [`StaticChartViewSnapshot`] values by `(input, selection)`, and exposes
//! deterministic, testable accessors. No astrology placement, rule evaluation,
//! temporal-overlay, 三方四正, mutagen, or 成格 derivation lives here — those
//! facts are read from prepared snapshots only.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::persistence::ChartStore;
use iztro::core::{
    BirthTime, ChartAlgorithmKind, ChartError, EarthlyBranch, Gender, MethodProfile,
    SolarChartRequest, SolarDay, SolarMonth, StaticChartCenterView, StaticChartViewSnapshot,
    StaticPalaceView, StaticTemporalNavigationSelection, static_temporal_chart_view,
};

/// Non-fatal notice shown when no local data directory is available, so saved
/// charts cannot be persisted this session.
pub const PERSISTENCE_UNAVAILABLE: &str =
    "Persistent storage unavailable; generated charts won't be saved this session.";

/// Side length of the fixed visual palace grid (4x4 perimeter layout).
pub const GRID_SIZE: u8 = 4;

/// The four center grid cells that hold the center panel, never a palace.
pub const CENTER_CELLS: [(u8, u8); 4] = [(1, 1), (1, 2), (2, 1), (2, 2)];

/// A sample birth input used to pre-fill the startup form for convenience.
///
/// Pre-filling the form does *not* generate a chart: the app starts on the
/// startup screen with no chart until the user explicitly generates one.
pub const SAMPLE_INPUT: BirthInput = BirthInput {
    year: 1990,
    month: 5,
    day: 17,
    time_index: 4, // 辰时
    gender: Gender::Female,
};

/// Which top-level screen the app is showing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Screen {
    /// Landing page: birth-input form plus the saved-charts list. No chart yet.
    Startup,
    /// A generated static chart is being displayed.
    Chart,
}

/// A clickable bottom temporal-navigation cell, identified by row and index.
///
/// This is the renderer-side identity of a navigation cell. Each cell maps to a
/// renderer-neutral [`StaticTemporalNavigationSelection`] that core turns into a
/// prepared snapshot; the GUI never derives the overlay itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TemporalCell {
    /// The 本命 (natal) cell.
    Natal,
    /// The 限前 (pre-decadal) cell.
    PreDecadal,
    /// A 大限 decadal cell at the given index.
    Decadal(usize),
    /// A 流年/小限 cell at the given index.
    YearlyAge(usize),
    /// A 流月 cell at the given index.
    Month(usize),
    /// A 流日 cell at `(row, index)`.
    Day(usize, usize),
    /// A 流时 cell at the given index.
    Hour(usize),
}

/// Extends or resets the current hierarchical temporal path for one clicked cell.
///
/// Child cells require their parent indices to exist. Selecting an ancestor
/// returns a shallower variant, which automatically clears invalid descendants.
fn next_temporal_selection(
    current: StaticTemporalNavigationSelection,
    clicked: TemporalCell,
) -> Option<StaticTemporalNavigationSelection> {
    match clicked {
        TemporalCell::Natal => Some(StaticTemporalNavigationSelection::Natal),
        TemporalCell::PreDecadal => Some(StaticTemporalNavigationSelection::PreDecadal),
        TemporalCell::Decadal(decadal_index) => {
            Some(StaticTemporalNavigationSelection::Decadal { decadal_index })
        }
        TemporalCell::YearlyAge(index) => Some(StaticTemporalNavigationSelection::Yearly {
            decadal_index: current.decadal_index()?,
            year_index: u8::try_from(index).ok()?,
        }),
        TemporalCell::Month(index) => Some(StaticTemporalNavigationSelection::Monthly {
            decadal_index: current.decadal_index()?,
            year_index: current.year_index()?,
            month_index: u8::try_from(index).ok()?,
        }),
        TemporalCell::Day(row, column) => Some(StaticTemporalNavigationSelection::Daily {
            decadal_index: current.decadal_index()?,
            year_index: current.year_index()?,
            month_index: current.month_index()?,
            day_index: u8::try_from(row.checked_mul(10)?.checked_add(column)?).ok()?,
        }),
        TemporalCell::Hour(index) => Some(StaticTemporalNavigationSelection::Hourly {
            decadal_index: current.decadal_index()?,
            year_index: current.year_index()?,
            month_index: current.month_index()?,
            day_index: current.day_index()?,
            hour_index: u8::try_from(index).ok()?,
        }),
    }
}

/// Normalized, hashable birth input. Doubles as the chart cache key and the
/// persisted record for a saved chart.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BirthInput {
    /// Solar (Gregorian) year.
    pub year: i32,
    /// Solar month (`1..=12`, validated downstream by the facade).
    pub month: u8,
    /// Solar day (`1..=31`, validated downstream by the facade).
    pub day: u8,
    /// Upstream `iztro` `timeIndex` (`0..=12`).
    pub time_index: u8,
    /// Gender marker.
    pub gender: Gender,
}

/// Editable, renderer-facing birth-input form (raw text plus typed choices).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BirthForm {
    /// Raw year text field.
    pub year: String,
    /// Raw month text field.
    pub month: String,
    /// Raw day text field.
    pub day: String,
    /// Selected birth-time index (`0..=12`).
    pub time_index: u8,
    /// Selected gender.
    pub gender: Gender,
}

impl BirthForm {
    /// Builds a form pre-filled from a normalized input.
    pub fn from_input(input: &BirthInput) -> Self {
        Self {
            year: input.year.to_string(),
            month: input.month.to_string(),
            day: input.day.to_string(),
            time_index: input.time_index,
            gender: input.gender,
        }
    }

    /// Parses and normalizes the form into a [`BirthInput`].
    ///
    /// Returns a user-facing message on a malformed numeric field. Deep calendar
    /// validity (e.g. 31 February) is deferred to the facade at build time.
    pub fn parse(&self) -> Result<BirthInput, String> {
        let year: i32 = self
            .year
            .trim()
            .parse()
            .map_err(|_| "Year must be a whole number".to_string())?;
        let month: u8 = self
            .month
            .trim()
            .parse()
            .map_err(|_| "Month must be a whole number".to_string())?;
        let day: u8 = self
            .day
            .trim()
            .parse()
            .map_err(|_| "Day must be a whole number".to_string())?;

        Ok(BirthInput {
            year,
            month,
            day,
            time_index: self.time_index,
            gender: self.gender,
        })
    }
}

impl Default for BirthForm {
    fn default() -> Self {
        Self::from_input(&SAMPLE_INPUT)
    }
}

/// Outcome of a [`StaticChartApp::generate`] call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GenerateOutcome {
    /// Built a fresh chart and stored it in the cache.
    Built,
    /// Served an existing chart from the cache.
    CacheHit,
    /// Input was invalid; the error state was set and the chart left unchanged.
    Invalid,
}

/// In-memory cache of static chart snapshots keyed by normalized birth input.
///
/// This caches view-model values only; it never caches rendered widgets and is
/// not persisted to disk.
#[derive(Clone, Debug, Default)]
pub struct ChartCache {
    entries: HashMap<(BirthInput, StaticTemporalNavigationSelection), StaticChartViewSnapshot>,
    hits: u64,
    misses: u64,
}

impl ChartCache {
    /// Returns the cached pre-decadal snapshot for `input`, building it on a miss.
    pub fn get_or_build(
        &mut self,
        input: &BirthInput,
    ) -> Result<(StaticChartViewSnapshot, bool), ChartError> {
        self.get_or_build_with(input, StaticTemporalNavigationSelection::PreDecadal)
    }

    /// Returns the cached snapshot for `(input, selection)`, building and storing
    /// it on a miss. The `bool` is `true` when the result came from the cache.
    ///
    /// The snapshot is prepared by core through the `static_temporal_chart_view`
    /// facade: the GUI never derives the overlay itself.
    pub fn get_or_build_with(
        &mut self,
        input: &BirthInput,
        selection: StaticTemporalNavigationSelection,
    ) -> Result<(StaticChartViewSnapshot, bool), ChartError> {
        let key = (*input, selection);
        if let Some(snapshot) = self.entries.get(&key) {
            self.hits += 1;
            return Ok((snapshot.clone(), true));
        }
        let snapshot = build_snapshot(input, selection)?;
        self.misses += 1;
        self.entries.insert(key, snapshot.clone());
        Ok((snapshot, false))
    }

    /// Number of cache hits observed so far.
    pub fn hits(&self) -> u64 {
        self.hits
    }

    /// Number of cache misses (fresh builds) observed so far.
    pub fn misses(&self) -> u64 {
        self.misses
    }

    /// Number of distinct cached inputs.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Whether the default pre-decadal snapshot for `input` is currently cached.
    pub fn contains(&self, input: &BirthInput) -> bool {
        self.entries
            .contains_key(&(*input, StaticTemporalNavigationSelection::PreDecadal))
    }
}

/// Messages the GUI emits into the pure update loop.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    /// The user clicked the palace cell identified by its branch.
    SelectPalace(EarthlyBranch),
    /// Year text field changed.
    YearChanged(String),
    /// Month text field changed.
    MonthChanged(String),
    /// Day text field changed.
    DayChanged(String),
    /// Birth-time index selected.
    TimeSelected(u8),
    /// Gender selected.
    GenderSelected(Gender),
    /// Generate-chart action triggered.
    Generate,
    /// A saved chart selected by index; opens it in the chart view.
    SelectSaved(usize),
    /// A bottom temporal-navigation cell was clicked.
    SelectTemporalCell(TemporalCell),
    /// The 三方四正 highlight mode was toggled.
    ToggleSanFang(bool),
    /// The pointer entered the palace cell identified by its branch.
    HoverPalace(EarthlyBranch),
    /// The pointer left the palace cell identified by its branch.
    ///
    /// Carries the branch so a stale exit (for a palace the pointer already
    /// left) cannot clear a newer hover.
    ClearHoveredPalace(EarthlyBranch),
    /// Return to the startup screen.
    BackToStartup,
}

/// Pure application state backing the static chart screen.
#[derive(Debug, Clone)]
pub struct StaticChartApp {
    screen: Screen,
    form: BirthForm,
    input: Option<BirthInput>,
    snapshot: Option<StaticChartViewSnapshot>,
    selected: Option<EarthlyBranch>,
    hovered_palace: Option<EarthlyBranch>,
    selected_temporal_selection: StaticTemporalNavigationSelection,
    highlight_san_fang: bool,
    error: Option<String>,
    cache: ChartCache,
    saved: Vec<BirthInput>,
    store: Option<ChartStore>,
}

impl StaticChartApp {
    /// Builds an app on the startup screen with no chart generated.
    ///
    /// The birth form is pre-filled from [`SAMPLE_INPUT`] for convenience, but no
    /// chart is built until the user generates one.
    pub fn new() -> Self {
        Self {
            screen: Screen::Startup,
            form: BirthForm::default(),
            input: None,
            snapshot: None,
            selected: None,
            hovered_palace: None,
            selected_temporal_selection: StaticTemporalNavigationSelection::PreDecadal,
            highlight_san_fang: true,
            error: None,
            cache: ChartCache::default(),
            saved: Vec::new(),
            store: None,
        }
    }

    /// Builds an app backed by a local [`ChartStore`], seeding the saved-charts
    /// list from disk. Still starts on the startup screen with no chart.
    pub fn with_store(store: ChartStore) -> Self {
        let saved = store.load();
        Self {
            saved,
            store: Some(store),
            ..Self::new()
        }
    }

    /// Builds an app from an optional store.
    ///
    /// `Some(store)` behaves like [`with_store`](Self::with_store). `None` starts
    /// the app without persistence and surfaces a non-fatal notice: chart
    /// generation still works, but saved charts are not written to disk.
    pub fn with_optional_store(store: Option<ChartStore>) -> Self {
        match store {
            Some(store) => Self::with_store(store),
            None => {
                let mut app = Self::new();
                app.error = Some(PERSISTENCE_UNAVAILABLE.to_owned());
                app
            }
        }
    }

    /// Replaces the saved-charts list (e.g. when seeding from persistence).
    pub fn set_saved(&mut self, saved: Vec<BirthInput>) {
        self.saved = saved;
    }

    /// The current top-level screen.
    pub fn screen(&self) -> Screen {
        self.screen
    }

    /// Returns the static chart snapshot driving the chart view, if any.
    pub fn snapshot(&self) -> Option<&StaticChartViewSnapshot> {
        self.snapshot.as_ref()
    }

    /// Returns the editable birth-input form.
    pub fn form(&self) -> &BirthForm {
        &self.form
    }

    /// Returns the normalized input that produced the current snapshot, if any.
    pub fn input(&self) -> Option<BirthInput> {
        self.input
    }

    /// Returns the current user-facing error, if any.
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    /// Returns the chart cache (read-only).
    pub fn cache(&self) -> &ChartCache {
        &self.cache
    }

    /// Returns the saved generated-chart inputs, most recent last.
    pub fn saved(&self) -> &[BirthInput] {
        &self.saved
    }

    /// Returns the twelve perimeter palaces of the current snapshot, if any.
    pub fn palaces(&self) -> &[StaticPalaceView] {
        self.snapshot
            .as_ref()
            .map(|snapshot| snapshot.palaces.as_slice())
            .unwrap_or(&[])
    }

    /// Returns the center-panel facts of the current snapshot, if any.
    pub fn center(&self) -> Option<&StaticChartCenterView> {
        self.snapshot.as_ref().map(|snapshot| &snapshot.center)
    }

    /// Returns the palace whose fixed grid position is `(row, column)`.
    ///
    /// Lookup is keyed by [`grid_position`], not by `Vec` order. Center cells and
    /// the empty-snapshot case return `None`.
    ///
    /// [`grid_position`]: iztro::core::StaticPalaceView::grid_position
    pub fn palace_at(&self, row: u8, column: u8) -> Option<&StaticPalaceView> {
        self.palaces().iter().find(|palace| {
            palace.grid_position.row() == row && palace.grid_position.column() == column
        })
    }

    /// Returns the branch of the currently selected palace, if any.
    pub fn selected_branch(&self) -> Option<EarthlyBranch> {
        self.selected
    }

    /// Returns the currently selected palace, if any.
    pub fn selected_palace(&self) -> Option<&StaticPalaceView> {
        let branch = self.selected?;
        self.palaces().iter().find(|palace| palace.branch == branch)
    }

    /// Returns the branch of the palace currently under the pointer, if any.
    pub fn hovered_palace(&self) -> Option<EarthlyBranch> {
        self.hovered_palace
    }

    /// The branch driving 三方四正 highlighting: hover takes priority over the
    /// sticky selection while the pointer is over a palace.
    pub fn active_branch(&self) -> Option<EarthlyBranch> {
        self.hovered_palace.or(self.selected)
    }

    /// Returns the palace driving highlighting (hovered, else selected), if any.
    pub fn active_palace(&self) -> Option<&StaticPalaceView> {
        let branch = self.active_branch()?;
        self.palaces().iter().find(|palace| palace.branch == branch)
    }

    /// Returns the authoritative hierarchical temporal selection path.
    pub fn selected_temporal_selection(&self) -> StaticTemporalNavigationSelection {
        self.selected_temporal_selection
    }

    /// Whether 三方四正 highlight mode is enabled.
    pub fn highlight_san_fang(&self) -> bool {
        self.highlight_san_fang
    }

    /// Whether `branch` is in the active palace's prepared 三方四正 set.
    ///
    /// The active palace is the hovered one, falling back to the sticky
    /// selection. Reads the prepared [`surround`] field; performs no branch
    /// arithmetic. Returns `false` while the toggle is off, so only the active
    /// palace itself is highlighted.
    ///
    /// [`surround`]: iztro::core::StaticPalaceView::surround
    pub fn is_in_san_fang(&self, branch: EarthlyBranch) -> bool {
        self.highlight_san_fang
            && self
                .active_palace()
                .is_some_and(|palace| palace.surround.involves(branch))
    }

    /// Whether the given temporal cell is enabled in the current snapshot.
    pub fn temporal_cell_enabled(&self, cell: TemporalCell) -> bool {
        let Some(panel) = self.snapshot.as_ref().map(|s| &s.temporal_panel) else {
            return false;
        };
        match cell {
            // The natal slice is always available once a chart is shown.
            TemporalCell::Natal => true,
            TemporalCell::PreDecadal => panel.pre_decadal_cell.enabled,
            TemporalCell::Decadal(i) => panel.decadal_cells.get(i).is_some_and(|c| c.enabled),
            TemporalCell::YearlyAge(i) => panel.yearly_age_cells.get(i).is_some_and(|c| c.enabled),
            TemporalCell::Month(i) => panel.month_cells.get(i).is_some_and(|c| c.enabled),
            TemporalCell::Day(row, i) => panel
                .day_rows
                .get(row)
                .and_then(|cells| cells.get(i))
                .is_some_and(|c| c.enabled),
            TemporalCell::Hour(i) => panel.hour_cells.get(i).is_some_and(|c| c.enabled),
        }
    }

    /// Generates a chart from the current form, switching to the chart view on
    /// success or setting the error state on invalid input. Never panics.
    pub fn generate(&mut self) -> GenerateOutcome {
        let input = match self.form.parse() {
            Ok(input) => input,
            Err(message) => {
                self.error = Some(message);
                return GenerateOutcome::Invalid;
            }
        };

        match self.cache.get_or_build(&input) {
            Ok((snapshot, hit)) => {
                self.snapshot = Some(snapshot);
                self.input = Some(input);
                self.selected = None;
                self.hovered_palace = None;
                self.selected_temporal_selection = StaticTemporalNavigationSelection::PreDecadal;
                self.error = None;
                self.screen = Screen::Chart;
                let newly_saved = !self.saved.contains(&input);
                if newly_saved {
                    self.saved.push(input);
                    self.persist_saved();
                }
                if hit {
                    GenerateOutcome::CacheHit
                } else {
                    GenerateOutcome::Built
                }
            }
            Err(error) => {
                self.error = Some(error.to_string());
                GenerateOutcome::Invalid
            }
        }
    }

    /// Persists the saved list to the backing store, if one is configured. A
    /// write failure is non-fatal: the in-memory list stays authoritative.
    fn persist_saved(&mut self) {
        if let Some(store) = &self.store {
            let _ = store.save(&self.saved);
        }
    }

    /// Applies a message to the state.
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SelectPalace(branch) => self.selected = Some(branch),
            Message::YearChanged(value) => self.form.year = value,
            Message::MonthChanged(value) => self.form.month = value,
            Message::DayChanged(value) => self.form.day = value,
            Message::TimeSelected(index) => self.form.time_index = index,
            Message::GenderSelected(gender) => self.form.gender = gender,
            Message::Generate => {
                self.generate();
            }
            Message::SelectSaved(index) => {
                if let Some(input) = self.saved.get(index).copied() {
                    self.form = BirthForm::from_input(&input);
                    self.generate();
                }
            }
            Message::SelectTemporalCell(cell) => {
                // Disabled cells stay inert: no selection, no snapshot change.
                if self.temporal_cell_enabled(cell) {
                    let Some(selection) =
                        next_temporal_selection(self.selected_temporal_selection, cell)
                    else {
                        return;
                    };
                    if let Some(input) = self.input {
                        match self.cache.get_or_build_with(&input, selection) {
                            Ok((snapshot, _)) => {
                                self.selected_temporal_selection = selection;
                                self.snapshot = Some(snapshot);
                                self.error = None;
                            }
                            Err(error) => self.error = Some(error.to_string()),
                        }
                    }
                }
            }
            Message::ToggleSanFang(enabled) => self.highlight_san_fang = enabled,
            Message::HoverPalace(branch) => self.hovered_palace = Some(branch),
            Message::ClearHoveredPalace(branch) => {
                // Ignore a stale exit so it cannot clear a newer hover.
                if self.hovered_palace == Some(branch) {
                    self.hovered_palace = None;
                }
            }
            Message::BackToStartup => {
                self.screen = Screen::Startup;
                self.selected = None;
                self.hovered_palace = None;
                self.selected_temporal_selection = StaticTemporalNavigationSelection::PreDecadal;
            }
        }
    }
}

impl Default for StaticChartApp {
    fn default() -> Self {
        Self::new()
    }
}

/// Builds a [`StaticChartViewSnapshot`] for `input` and `selection` through the
/// `static_temporal_chart_view` facade, so all temporal-overlay derivation stays
/// in core. Returns the facade error for invalid calendar input or selection.
fn build_snapshot(
    input: &BirthInput,
    selection: StaticTemporalNavigationSelection,
) -> Result<StaticChartViewSnapshot, ChartError> {
    let request = SolarChartRequest::builder()
        .solar_year(input.year)
        .solar_month(SolarMonth::new(input.month)?)
        .solar_day(SolarDay::new(input.day)?)
        .birth_time_variant(BirthTime::from_iztro_time_index(input.time_index)?)
        .gender(input.gender)
        .method_profile(MethodProfile::new(
            "iztro_gui",
            ChartAlgorithmKind::QuanShu,
            "iztro-gui static chart prototype",
        ))
        .build()?;

    static_temporal_chart_view(request, selection)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn production_source(raw: &str) -> String {
        raw.split("#[cfg(test)]")
            .next()
            .unwrap_or(raw)
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn app_starts_on_startup_without_generating_a_chart() {
        let app = StaticChartApp::new();
        assert_eq!(app.screen(), Screen::Startup);
        assert!(app.snapshot().is_none());
        assert!(app.input().is_none());
        assert!(app.palaces().is_empty());
        assert!(app.center().is_none());
        assert!(app.saved().is_empty());
        // The form is pre-filled for convenience.
        assert_eq!(app.form(), &BirthForm::from_input(&SAMPLE_INPUT));
        assert!(app.error().is_none());
    }

    #[test]
    fn generating_a_valid_chart_moves_to_chart_view() {
        let mut app = StaticChartApp::new();
        app.update(Message::YearChanged("1985".to_string()));
        app.update(Message::MonthChanged("3".to_string()));
        app.update(Message::DayChanged("8".to_string()));
        app.update(Message::TimeSelected(6));
        app.update(Message::GenderSelected(Gender::Male));

        let outcome = app.generate();

        assert_eq!(outcome, GenerateOutcome::Built);
        assert_eq!(app.screen(), Screen::Chart);
        assert!(app.snapshot().is_some());
        assert!(app.error().is_none());
        assert_eq!(
            app.input(),
            Some(BirthInput {
                year: 1985,
                month: 3,
                day: 8,
                time_index: 6,
                gender: Gender::Male,
            })
        );
        assert_eq!(app.palaces().len(), 12);
        // The generated chart is added to the saved list.
        assert_eq!(app.saved().len(), 1);
    }

    #[test]
    fn invalid_numeric_input_sets_error_and_stays_on_startup() {
        let mut app = StaticChartApp::new();
        app.update(Message::YearChanged("not-a-year".to_string()));

        let outcome = app.generate();

        assert_eq!(outcome, GenerateOutcome::Invalid);
        assert!(app.error().is_some());
        assert_eq!(app.screen(), Screen::Startup);
        assert!(app.snapshot().is_none());
        assert!(app.saved().is_empty());
    }

    #[test]
    fn invalid_calendar_input_sets_error_without_corrupting_saved() {
        let mut app = StaticChartApp::new();
        // Generate one valid chart first.
        app.generate();
        assert_eq!(app.saved().len(), 1);
        let saved_before = app.saved().to_vec();

        // Month 13 is numerically parseable but rejected by the facade.
        app.update(Message::MonthChanged("13".to_string()));
        let outcome = app.generate();

        assert_eq!(outcome, GenerateOutcome::Invalid);
        assert!(app.error().is_some());
        // The saved list is untouched by the invalid attempt.
        assert_eq!(app.saved(), saved_before.as_slice());
    }

    #[test]
    fn repeated_generation_with_same_input_hits_the_cache() {
        let mut app = StaticChartApp::new();
        let first = app.generate();
        let second = app.generate();

        assert_eq!(first, GenerateOutcome::Built);
        assert_eq!(second, GenerateOutcome::CacheHit);
        assert!(app.cache().hits() >= 1);
        // The same input is saved only once.
        assert_eq!(app.saved().len(), 1);
    }

    #[test]
    fn different_birth_input_creates_a_different_cache_key() {
        let mut app = StaticChartApp::new();
        app.generate();
        app.update(Message::YearChanged("2000".to_string()));
        app.generate();

        let other = BirthInput {
            year: 2000,
            ..SAMPLE_INPUT
        };
        assert!(app.cache().contains(&SAMPLE_INPUT));
        assert!(app.cache().contains(&other));
        assert_eq!(app.cache().len(), 2);
        assert_eq!(app.saved().len(), 2);
    }

    #[test]
    fn generated_charts_persist_and_reload_through_the_store() {
        let dir = tempfile::tempdir().expect("temp dir");
        let store = crate::persistence::ChartStore::new(dir.path().join("charts.json"));

        let mut app = StaticChartApp::with_store(store.clone());
        assert!(app.saved().is_empty());
        app.update(Message::YearChanged("1985".to_string()));
        app.generate();
        assert_eq!(app.saved().len(), 1);

        // A fresh app backed by the same store sees the persisted chart and can
        // open it without re-entering the form.
        let mut reloaded = StaticChartApp::with_store(store);
        assert_eq!(reloaded.saved().len(), 1);
        reloaded.update(Message::SelectSaved(0));
        assert_eq!(reloaded.screen(), Screen::Chart);
        assert_eq!(reloaded.input().map(|i| i.year), Some(1985));
    }

    #[test]
    fn no_store_starts_without_persistence_and_warns_but_still_generates() {
        let mut app = StaticChartApp::with_optional_store(None);
        assert!(app.saved().is_empty());
        assert_eq!(app.error(), Some(PERSISTENCE_UNAVAILABLE));

        // Generation still works; the chart is tracked in memory only.
        assert_eq!(app.generate(), GenerateOutcome::Built);
        assert_eq!(app.screen(), Screen::Chart);
        assert_eq!(app.saved(), &[SAMPLE_INPUT]);
        assert!(
            app.error().is_none(),
            "a successful build clears the notice"
        );
    }

    #[test]
    fn optional_store_some_behaves_like_with_store() {
        let dir = tempfile::tempdir().expect("temp dir");
        let store = crate::persistence::ChartStore::new(dir.path().join("charts.json"));
        let app = StaticChartApp::with_optional_store(Some(store));
        assert!(app.saved().is_empty());
        assert!(app.error().is_none());
    }

    #[test]
    fn invalid_input_does_not_corrupt_the_persisted_store() {
        let dir = tempfile::tempdir().expect("temp dir");
        let store = crate::persistence::ChartStore::new(dir.path().join("charts.json"));

        let mut app = StaticChartApp::with_store(store.clone());
        app.generate(); // one valid sample chart persisted
        app.update(Message::MonthChanged("13".to_string()));
        assert_eq!(app.generate(), GenerateOutcome::Invalid);

        // The on-disk store still parses and holds exactly the valid chart.
        let reloaded = store.load();
        assert_eq!(reloaded, vec![SAMPLE_INPUT]);
    }

    #[test]
    fn selecting_a_saved_chart_opens_it() {
        let mut app = StaticChartApp::new();
        app.set_saved(vec![SAMPLE_INPUT]);
        app.update(Message::SelectSaved(0));

        assert_eq!(app.screen(), Screen::Chart);
        assert_eq!(app.input(), Some(SAMPLE_INPUT));
        assert_eq!(app.palaces().len(), 12);
    }

    #[test]
    fn back_to_startup_returns_to_landing() {
        let mut app = StaticChartApp::new();
        app.generate();
        assert_eq!(app.screen(), Screen::Chart);

        app.update(Message::BackToStartup);
        assert_eq!(app.screen(), Screen::Startup);
        assert_eq!(app.selected_branch(), None);
        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::PreDecadal
        );
    }

    #[test]
    fn palace_layout_lookup_uses_grid_position() {
        let mut app = StaticChartApp::new();
        app.generate();
        for palace in app.palaces() {
            let pos = palace.grid_position;
            let found = app
                .palace_at(pos.row(), pos.column())
                .expect("palace reachable at its grid position");
            assert_eq!(found.branch, palace.branch);
        }
        for (row, column) in CENTER_CELLS {
            assert!(app.palace_at(row, column).is_none());
        }
        let by_grid = app.palace_at(1, 3).expect("cell (1,3) holds a palace");
        assert_eq!(by_grid.branch, app.palaces()[5].branch);
    }

    #[test]
    fn selecting_a_palace_updates_the_selected_branch() {
        let mut app = StaticChartApp::new();
        app.generate();
        assert_eq!(app.selected_branch(), None);
        let branch = app.palaces()[3].branch;
        app.update(Message::SelectPalace(branch));
        assert_eq!(app.selected_branch(), Some(branch));
        assert_eq!(app.selected_palace().expect("selected").branch, branch);
    }

    #[test]
    fn san_fang_highlight_reads_prepared_surround_only() {
        let mut app = StaticChartApp::new();
        app.generate();
        let palace = app.palaces()[0].clone();
        app.update(Message::SelectPalace(palace.branch));

        // Highlight membership matches the prepared surround set exactly.
        for related in palace.surround.branches() {
            assert!(app.is_in_san_fang(related));
        }
        assert!(!app.is_in_san_fang(palace.branch));

        // Toggling the mode off suppresses all highlight membership.
        app.update(Message::ToggleSanFang(false));
        for related in palace.surround.branches() {
            assert!(!app.is_in_san_fang(related));
        }
    }

    #[test]
    fn hovering_a_palace_sets_the_hovered_branch() {
        let mut app = StaticChartApp::new();
        app.generate();
        let branch = app.palaces()[2].branch;
        app.update(Message::HoverPalace(branch));
        assert_eq!(app.hovered_palace(), Some(branch));
        assert_eq!(app.active_branch(), Some(branch));
    }

    #[test]
    fn hover_takes_priority_then_clearing_restores_sticky_selection() {
        let mut app = StaticChartApp::new();
        app.generate();
        let selected = app.palaces()[0].branch;
        let hovered = app.palaces()[1].branch;
        app.update(Message::SelectPalace(selected));
        app.update(Message::HoverPalace(hovered));

        // Hover wins over the sticky selection while the pointer is over it.
        assert_eq!(app.active_branch(), Some(hovered));

        // Clearing the current hover restores the sticky selection.
        app.update(Message::ClearHoveredPalace(hovered));
        assert_eq!(app.hovered_palace(), None);
        assert_eq!(app.active_branch(), Some(selected));
    }

    #[test]
    fn a_stale_hover_exit_does_not_clear_a_newer_hover() {
        let mut app = StaticChartApp::new();
        app.generate();
        let first = app.palaces()[0].branch;
        let second = app.palaces()[1].branch;
        app.update(Message::HoverPalace(second));
        // A late exit for a palace already left must not clear the newer hover.
        app.update(Message::ClearHoveredPalace(first));
        assert_eq!(app.hovered_palace(), Some(second));
    }

    #[test]
    fn hover_driven_san_fang_reads_prepared_surround_only() {
        let mut app = StaticChartApp::new();
        app.generate();
        let palace = app.palaces()[4].clone();
        app.update(Message::HoverPalace(palace.branch));

        for related in palace.surround.branches() {
            assert!(app.is_in_san_fang(related));
        }
        assert!(!app.is_in_san_fang(palace.branch));

        // Toggling the mode off suppresses related-palace highlight; only the
        // active palace itself stays emphasized (handled by the renderer).
        app.update(Message::ToggleSanFang(false));
        for related in palace.surround.branches() {
            assert!(!app.is_in_san_fang(related));
        }
    }

    #[test]
    fn generated_chart_defaults_to_pre_decadal_selection() {
        let mut app = StaticChartApp::new();
        app.generate();
        let panel = &app.snapshot().expect("snapshot").temporal_panel;

        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::PreDecadal
        );
        assert!(panel.pre_decadal_cell.selected);
        assert!(panel.yearly_age_cells.iter().all(|cell| !cell.enabled));
        assert!(panel.month_cells.iter().all(|cell| !cell.enabled));
        assert!(panel.day_rows.iter().flatten().all(|cell| !cell.enabled));
        assert!(panel.hour_cells.iter().all(|cell| !cell.enabled));
    }

    #[test]
    fn clicking_a_decadal_cell_changes_the_effective_snapshot() {
        let mut app = StaticChartApp::new();
        app.generate();
        // The natal base carries no overlays.
        assert!(app.palaces().iter().all(|p| p.overlays.is_empty()));

        let cell = TemporalCell::Decadal(0);
        assert!(app.temporal_cell_enabled(cell));
        app.update(Message::SelectTemporalCell(cell));

        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::Decadal { decadal_index: 0 }
        );
        // The prepared snapshot now carries a decadal overlay — the click changed
        // the effective slice, not only the selection state.
        assert!(
            app.snapshot()
                .expect("snapshot")
                .active_scopes
                .contains(&iztro::core::Scope::Decadal)
        );
        assert!(app.palaces().iter().any(|p| !p.overlays.is_empty()));
    }

    #[test]
    fn child_clicks_without_the_required_parent_path_are_ignored() {
        let mut app = StaticChartApp::new();
        app.generate();
        let initial = app.snapshot().cloned();

        for cell in [
            TemporalCell::YearlyAge(0),
            TemporalCell::Month(0),
            TemporalCell::Day(0, 0),
            TemporalCell::Hour(0),
        ] {
            assert!(!app.temporal_cell_enabled(cell));
            app.update(Message::SelectTemporalCell(cell));
            assert_eq!(
                app.selected_temporal_selection(),
                StaticTemporalNavigationSelection::PreDecadal
            );
            assert_eq!(app.snapshot().cloned(), initial);
        }
    }

    #[test]
    fn pre_decadal_cell_is_an_enabled_first_row_navigation_cell() {
        let mut app = StaticChartApp::new();
        app.generate();
        let panel = &app.snapshot().expect("snapshot").temporal_panel;
        assert_eq!(panel.pre_decadal_cell.label_zh, "限前");
        assert!(panel.pre_decadal_cell.enabled);

        // 限前 is selectable and resolves to the natal base slice (no overlay).
        let cell = TemporalCell::PreDecadal;
        assert!(app.temporal_cell_enabled(cell));
        app.update(Message::SelectTemporalCell(cell));
        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::PreDecadal
        );
        assert!(app.palaces().iter().all(|p| p.overlays.is_empty()));
    }

    #[test]
    fn temporal_drill_down_unlocks_each_child_row() {
        let mut app = StaticChartApp::new();
        app.generate();

        app.update(Message::SelectTemporalCell(TemporalCell::Decadal(0)));
        assert_eq!(
            app.snapshot()
                .expect("decadal snapshot")
                .temporal_panel
                .yearly_age_cells
                .iter()
                .filter(|cell| cell.enabled)
                .count(),
            10
        );

        app.update(Message::SelectTemporalCell(TemporalCell::YearlyAge(0)));
        assert!(
            app.snapshot()
                .expect("yearly snapshot")
                .temporal_panel
                .month_cells
                .iter()
                .all(|cell| cell.enabled)
        );

        app.update(Message::SelectTemporalCell(TemporalCell::Month(0)));
        assert!(
            app.snapshot()
                .expect("monthly snapshot")
                .temporal_panel
                .day_rows
                .iter()
                .flatten()
                .any(|cell| cell.enabled)
        );

        app.update(Message::SelectTemporalCell(TemporalCell::Day(0, 0)));
        assert!(
            app.snapshot()
                .expect("daily snapshot")
                .temporal_panel
                .hour_cells
                .iter()
                .all(|cell| cell.enabled)
        );

        app.update(Message::SelectTemporalCell(TemporalCell::Hour(0)));
        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::Hourly {
                decadal_index: 0,
                year_index: 0,
                month_index: 0,
                day_index: 0,
                hour_index: 0,
            }
        );
    }

    #[test]
    fn selecting_a_new_parent_clears_descendant_path() {
        let mut app = StaticChartApp::new();
        app.generate();
        for cell in [
            TemporalCell::Decadal(0),
            TemporalCell::YearlyAge(0),
            TemporalCell::Month(0),
            TemporalCell::Day(0, 0),
            TemporalCell::Hour(0),
        ] {
            app.update(Message::SelectTemporalCell(cell));
        }

        app.update(Message::SelectTemporalCell(TemporalCell::Decadal(1)));

        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::Decadal { decadal_index: 1 }
        );
        let panel = &app.snapshot().expect("snapshot").temporal_panel;
        assert!(panel.decadal_cells[1].selected);
        assert!(panel.yearly_age_cells.iter().all(|cell| !cell.selected));
        assert!(panel.month_cells.iter().all(|cell| !cell.enabled));
        assert!(panel.day_rows.iter().flatten().all(|cell| !cell.enabled));
        assert!(panel.hour_cells.iter().all(|cell| !cell.enabled));
    }

    #[test]
    fn returning_to_a_decadal_then_natal_keeps_natal_facts_immutable() {
        let mut app = StaticChartApp::new();
        app.generate();
        let natal_palaces: Vec<_> = app.palaces().to_vec();

        app.update(Message::SelectTemporalCell(TemporalCell::Decadal(0)));
        app.update(Message::SelectTemporalCell(TemporalCell::Natal));

        // Selecting overlays then returning to 本命 leaves natal facts identical.
        for (before, after) in natal_palaces.iter().zip(app.palaces()) {
            assert_eq!(before.branch, after.branch);
            assert_eq!(before.surround, after.surround);
            assert_eq!(before.major_stars, after.major_stars);
        }
    }

    #[test]
    fn natal_snapshot_has_no_temporal_overlays() {
        let mut app = StaticChartApp::new();
        app.generate();
        for palace in app.palaces() {
            assert!(
                palace.overlays.is_empty(),
                "natal-only snapshot must have no overlays"
            );
        }
        let has_natal_stars = app.palaces().iter().any(|palace| {
            !palace.major_stars.is_empty()
                || !palace.minor_stars.is_empty()
                || !palace.adjective_stars.is_empty()
        });
        assert!(has_natal_stars, "natal star groups must be populated");
    }

    #[test]
    fn gui_source_does_not_derive_astrology_facts() {
        const FORBIDDEN: [&str; 20] = [
            "Placer",
            "palace_grid_position",
            "zi_wei_branch",
            "tian_fu_branch",
            "build_minimal_natal_chart",
            "build_natal_chart_with",
            "star_brightness",
            "PlacementInput",
            // 三方四正 / mutagen must be read from prepared snapshots, never derived.
            ".offset(",
            "StaticSurroundPalacesView::for_branch",
            "birth_year_star_mutagen",
            "birth_year_major_star_mutagen",
            // Temporal overlays must be prepared by the `static_temporal_chart_view`
            // facade; the GUI must never construct a horoscope, temporal layer,
            // or decadal frame itself.
            "build_decadal_horoscope_chart",
            "build_decadal_horoscope_layer",
            "build_full_horoscope_chart",
            "from_horoscope_chart_with",
            "HoroscopeChart",
            "TemporalLayer",
            "DecadalHoroscopeInput",
            "build_decadal_frame",
        ];

        let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut checked = 0;
        for entry in std::fs::read_dir(&src_dir).expect("src directory must exist") {
            let path = entry.expect("readable dir entry").path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
                continue;
            }
            let raw = std::fs::read_to_string(&path).expect("source file must read");
            // Scan production code only; tests and comments may name forbidden symbols.
            let source = production_source(&raw);
            for needle in FORBIDDEN {
                assert!(
                    !source.contains(needle),
                    "{} must not reference derivation symbol `{needle}`",
                    path.display()
                );
            }
            checked += 1;
        }
        assert!(checked >= 3, "expected to scan the GUI source files");

        let app_src = std::fs::read_to_string(src_dir.join("app.rs")).expect("app.rs must read");
        assert!(
            app_src.contains("static_temporal_chart_view"),
            "charts must be built through the static_temporal_chart_view facade"
        );
    }
}
