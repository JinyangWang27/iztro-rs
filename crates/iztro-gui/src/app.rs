//! Pure application state and logic for the static chart GUI.
//!
//! This module is renderer-agnostic: it depends only on `iztro` facade APIs and
//! read models, never on `iced`. It owns the birth-input form, builds charts
//! through the public [`by_solar`] facade, caches the resulting
//! [`StaticChartViewSnapshot`] values by normalized input, and exposes
//! deterministic, testable accessors. No astrology placement, rule evaluation,
//! or 成格 detection lives here.

use std::collections::HashMap;

use iztro::core::{
    ChartAlgorithmKind, ChartError, EarthlyBranch, Gender, MethodProfile, SolarChartRequest,
    SolarDay, SolarMonth, StaticChartCenterView, StaticChartViewSnapshot, StaticPalaceView,
    by_solar,
};

/// Side length of the fixed visual palace grid (4x4 perimeter layout).
pub const GRID_SIZE: u8 = 4;

/// The four center grid cells that hold the center panel, never a palace.
pub const CENTER_CELLS: [(u8, u8); 4] = [(1, 1), (1, 2), (2, 1), (2, 2)];

/// The hardcoded sample birth input used as the default chart.
pub const SAMPLE_INPUT: BirthInput = BirthInput {
    year: 1990,
    month: 5,
    day: 17,
    time_index: 4, // 辰时
    gender: Gender::Female,
};

/// Normalized, hashable birth input. Doubles as the chart cache key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    entries: HashMap<BirthInput, StaticChartViewSnapshot>,
    hits: u64,
    misses: u64,
}

impl ChartCache {
    /// Returns the cached snapshot for `input`, building and storing it on a
    /// miss. The `bool` is `true` when the result came from the cache.
    pub fn get_or_build(
        &mut self,
        input: &BirthInput,
    ) -> Result<(StaticChartViewSnapshot, bool), ChartError> {
        if let Some(snapshot) = self.entries.get(input) {
            self.hits += 1;
            return Ok((snapshot.clone(), true));
        }
        let snapshot = build_snapshot(input)?;
        self.misses += 1;
        self.entries.insert(*input, snapshot.clone());
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

    /// Whether a snapshot for `input` is currently cached.
    pub fn contains(&self, input: &BirthInput) -> bool {
        self.entries.contains_key(input)
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
    /// A history entry selected by index.
    SelectHistory(usize),
}

/// Pure application state backing the static chart screen.
#[derive(Debug, Clone)]
pub struct StaticChartApp {
    form: BirthForm,
    input: BirthInput,
    snapshot: StaticChartViewSnapshot,
    selected: Option<EarthlyBranch>,
    error: Option<String>,
    cache: ChartCache,
    history: Vec<BirthInput>,
}

impl StaticChartApp {
    /// Builds the app from the hardcoded sample input, generating its chart.
    pub fn new() -> Self {
        let mut cache = ChartCache::default();
        let (snapshot, _) = cache
            .get_or_build(&SAMPLE_INPUT)
            .expect("hardcoded sample input must build a chart");
        Self {
            form: BirthForm::from_input(&SAMPLE_INPUT),
            input: SAMPLE_INPUT,
            snapshot,
            selected: None,
            error: None,
            cache,
            history: vec![SAMPLE_INPUT],
        }
    }

    /// Returns the immutable static chart snapshot driving the view.
    pub fn snapshot(&self) -> &StaticChartViewSnapshot {
        &self.snapshot
    }

    /// Returns the editable birth-input form.
    pub fn form(&self) -> &BirthForm {
        &self.form
    }

    /// Returns the normalized input that produced the current snapshot.
    pub fn input(&self) -> BirthInput {
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

    /// Returns the generated-chart history, most recent last.
    pub fn history(&self) -> &[BirthInput] {
        &self.history
    }

    /// Returns the twelve perimeter palaces.
    pub fn palaces(&self) -> &[StaticPalaceView] {
        &self.snapshot.palaces
    }

    /// Returns the center-panel facts.
    pub fn center(&self) -> &StaticChartCenterView {
        &self.snapshot.center
    }

    /// Returns the palace whose fixed grid position is `(row, column)`.
    ///
    /// Lookup is keyed by [`grid_position`], not by `Vec` order. Center cells
    /// return `None`.
    ///
    /// [`grid_position`]: iztro::core::StaticPalaceView::grid_position
    pub fn palace_at(&self, row: u8, column: u8) -> Option<&StaticPalaceView> {
        self.snapshot.palaces.iter().find(|palace| {
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
        self.snapshot
            .palaces
            .iter()
            .find(|palace| palace.branch == branch)
    }

    /// Generates a chart from the current form, updating the displayed chart on
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
                self.snapshot = snapshot;
                self.input = input;
                self.selected = None;
                self.error = None;
                if self.history.last() != Some(&input) && !self.history.contains(&input) {
                    self.history.push(input);
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
            Message::SelectHistory(index) => {
                if let Some(input) = self.history.get(index).copied() {
                    self.form = BirthForm::from_input(&input);
                    self.generate();
                }
            }
        }
    }
}

impl Default for StaticChartApp {
    fn default() -> Self {
        Self::new()
    }
}

/// Builds a [`StaticChartViewSnapshot`] for `input` through the `by_solar`
/// facade. Returns the facade error for invalid calendar input.
fn build_snapshot(input: &BirthInput) -> Result<StaticChartViewSnapshot, ChartError> {
    let request = SolarChartRequest::builder()
        .solar_year(input.year)
        .solar_month(SolarMonth::new(input.month)?)
        .solar_day(SolarDay::new(input.day)?)
        .birth_time_variant(iztro::core::BirthTime::from_iztro_time_index(
            input.time_index,
        )?)
        .gender(input.gender)
        .method_profile(MethodProfile::new(
            "iztro_gui",
            ChartAlgorithmKind::QuanShu,
            "iztro-gui static chart prototype",
        ))
        .build()?;

    let chart = by_solar(request)?;
    Ok(StaticChartViewSnapshot::from_chart(&chart))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_app_initializes_with_sample_input_and_valid_snapshot() {
        let app = StaticChartApp::new();
        assert_eq!(app.input(), SAMPLE_INPUT);
        assert_eq!(app.form(), &BirthForm::from_input(&SAMPLE_INPUT));
        assert_eq!(app.palaces().len(), 12);
        assert!(!app.center().birth_year_stem_zh.is_empty());
        let pillars = app
            .center()
            .four_pillars
            .as_ref()
            .expect("default by_solar chart should expose factual four pillars");
        assert!(!pillars.yearly_zh.is_empty());
        assert!(!pillars.monthly_zh.is_empty());
        assert!(!pillars.daily_zh.is_empty());
        assert!(!pillars.hourly_zh.is_empty());
        assert!(app.error().is_none());
    }

    #[test]
    fn valid_input_generates_a_chart() {
        let mut app = StaticChartApp::new();
        app.update(Message::YearChanged("1985".to_string()));
        app.update(Message::MonthChanged("3".to_string()));
        app.update(Message::DayChanged("8".to_string()));
        app.update(Message::TimeSelected(6));
        app.update(Message::GenderSelected(Gender::Male));

        let outcome = app.generate();

        assert_eq!(outcome, GenerateOutcome::Built);
        assert!(app.error().is_none());
        assert_eq!(
            app.input(),
            BirthInput {
                year: 1985,
                month: 3,
                day: 8,
                time_index: 6,
                gender: Gender::Male,
            }
        );
        assert_eq!(app.palaces().len(), 12);
    }

    #[test]
    fn invalid_numeric_input_sets_error_state_without_panicking() {
        let mut app = StaticChartApp::new();
        let before = app.snapshot().clone();
        app.update(Message::YearChanged("not-a-year".to_string()));

        let outcome = app.generate();

        assert_eq!(outcome, GenerateOutcome::Invalid);
        assert!(app.error().is_some());
        // Chart is unchanged on invalid input.
        assert_eq!(app.snapshot(), &before);
    }

    #[test]
    fn invalid_calendar_input_sets_error_state_without_panicking() {
        let mut app = StaticChartApp::new();
        let before = app.snapshot().clone();
        // Month 13 is numerically parseable but rejected by the facade.
        app.update(Message::MonthChanged("13".to_string()));

        let outcome = app.generate();

        assert_eq!(outcome, GenerateOutcome::Invalid);
        assert!(app.error().is_some());
        assert_eq!(app.snapshot(), &before);
    }

    #[test]
    fn repeated_generation_with_same_input_hits_the_cache() {
        let mut app = StaticChartApp::new();
        // The sample build on `new()` already counts as one miss.
        let misses_before = app.cache().misses();

        app.update(Message::TimeSelected(SAMPLE_INPUT.time_index));
        let first = app.generate();
        let second = app.generate();

        assert_eq!(first, GenerateOutcome::CacheHit);
        assert_eq!(second, GenerateOutcome::CacheHit);
        assert!(app.cache().hits() >= 2);
        // No new distinct input was built.
        assert_eq!(app.cache().misses(), misses_before);
    }

    #[test]
    fn different_birth_input_creates_a_different_cache_key() {
        let mut app = StaticChartApp::new();
        app.update(Message::YearChanged("2000".to_string()));
        app.generate();

        let other = BirthInput {
            year: 2000,
            ..SAMPLE_INPUT
        };
        assert_ne!(SAMPLE_INPUT, other);
        assert!(app.cache().contains(&SAMPLE_INPUT));
        assert!(app.cache().contains(&other));
        assert_eq!(app.cache().len(), 2);
    }

    #[test]
    fn palace_layout_lookup_uses_grid_position() {
        let app = StaticChartApp::new();
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
        // Grid lookup must not coincide with naive Vec/row-major order.
        let by_grid = app.palace_at(1, 3).expect("cell (1,3) holds a palace");
        assert_eq!(by_grid.branch, app.palaces()[5].branch);
    }

    #[test]
    fn selecting_a_palace_updates_the_selected_branch() {
        let mut app = StaticChartApp::new();
        assert_eq!(app.selected_branch(), None);
        let branch = app.palaces()[3].branch;
        app.update(Message::SelectPalace(branch));
        assert_eq!(app.selected_branch(), Some(branch));
        assert_eq!(app.selected_palace().expect("selected").branch, branch);
    }

    #[test]
    fn natal_facts_stay_separate_from_temporal_overlays() {
        let app = StaticChartApp::new();
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
    fn gui_source_does_not_call_placement_modules_directly() {
        const FORBIDDEN: [&str; 8] = [
            "Placer",
            "palace_grid_position",
            "zi_wei_branch",
            "tian_fu_branch",
            "build_minimal_natal_chart",
            "build_natal_chart_with",
            "star_brightness",
            "PlacementInput",
        ];

        let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut checked = 0;
        for entry in std::fs::read_dir(&src_dir).expect("src directory must exist") {
            let path = entry.expect("readable dir entry").path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
                continue;
            }
            let raw = std::fs::read_to_string(&path).expect("source file must read");
            // Only scan production code; this very test names forbidden symbols.
            let source = raw.split("#[cfg(test)]").next().unwrap_or(&raw);
            for needle in FORBIDDEN {
                assert!(
                    !source.contains(needle),
                    "{} must not reference placement symbol `{needle}`",
                    path.display()
                );
            }
            checked += 1;
        }
        assert!(checked >= 3, "expected to scan the GUI source files");

        let app_src = std::fs::read_to_string(src_dir.join("app.rs")).expect("app.rs must read");
        assert!(
            app_src.contains("by_solar"),
            "the sample chart must be built through the by_solar facade"
        );
    }
}
