//! Pure application state and logic for the static chart GUI.
//!
//! This module is renderer-agnostic: it depends only on `iztro` facade APIs and
//! read models, never on `iced`. It owns the birth-input form, builds charts
//! through the public `static_temporal_chart_view` facade, caches the resulting
//! [`StaticChartProjection`] values by `(input, selection)`, and exposes
//! deterministic, testable accessors. No astrology placement, rule evaluation,
//! pattern detection, temporal-overlay, 三方四正, mutagen, or 成格 derivation is
//! computed here — chart facts are read from prepared snapshots, and the right
//! inspector's rule/pattern data is **requested** from the core analysis API
//! (`iztro::analysis::detect_static_temporal_analysis_layers_from_chart`) per
//! layer and cached. This module
//! decides *which* layers to request and holds the results; the derivation
//! itself stays in core.

use std::collections::{BTreeSet, HashMap};

use serde::{Deserialize, Serialize};

use crate::analysis::{
    ActiveAnalysisSelection, AnalysisCache, ChartHighlightView, PatternHitExpansionKey,
    RuleHitExpansionKey, highlight_for_pattern_detection, highlight_for_rule_hit,
    missing_analysis_layers,
};
use crate::persistence::ChartStore;
use crate::settings::{AppSettings, GuiThemeId, RightPanelMode, RightPanelTab, SettingsStore};
use iztro::analysis::{
    AnalysisLayerKey, AnalysisLayerRequest, analysis_layers_for_selection,
    detect_static_temporal_analysis_layers_from_chart,
};
use iztro::core::{
    BirthTime, Chart, ChartAlgorithmKind, ChartError, EarthlyBranch, Gender, MethodProfile,
    SolarChartRequest, SolarDay, SolarMonth, by_solar,
};
use iztro::{
    StaticChartCenterProjection, StaticChartProjection, StaticPalaceProjection,
    StaticTemporalNavigationSelection, static_temporal_chart_view,
    temporal_selection_for_solar_moment,
};
use iztro_i18n::{I18n, Locale};

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

/// Palace selection mode for 三方四正 highlighting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PalaceSelection {
    /// Follow the current projection's active-frame Life palace.
    DefaultActiveLife,
    /// User-clicked palace branch, sticky across temporal navigation.
    UserSelectedBranch(EarthlyBranch),
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

/// A temporal unit the compact stepper controls can move by.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TemporalUnit {
    /// 大限 (decadal period).
    Decadal,
    /// 流年 (flowing year).
    Year,
    /// 流月 (flowing month).
    Month,
    /// 流日 (flowing day).
    Day,
    /// 流时 (flowing double-hour).
    Hour,
}

/// Direction a temporal stepper moves a unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepDirection {
    /// Step to an earlier index (◀).
    Backward,
    /// Step to a later index (▶).
    Forward,
}

/// Plain current-moment facts the renderer reads from the system clock for the
/// `今` control. All calendar/age mapping happens in core; the GUI never
/// performs date math itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LocalSolarMoment {
    /// Solar (Gregorian) year.
    pub year: i32,
    /// Solar month (`1..=12`).
    pub month: u8,
    /// Solar day (`1..=31`).
    pub day: u8,
    /// Clock hour (`0..=23`).
    pub hour: u8,
    /// Clock minute (`0..=59`).
    pub minute: u8,
}

/// Carries a `(decadal_index, year_index)` pair one 流年 step in `direction`,
/// rolling across the 大限 boundary: forward past year 9 enters the next 大限 at
/// year 0, backward past year 0 enters the previous 大限 at year 9.
///
/// Returns `None` at the absolute first/last year of the supported navigation
/// range so the caller can stay on the current valid selection. `max_decadal`
/// is the last enabled 大限 index.
fn carry_year_pair(
    decadal: usize,
    year: u8,
    direction: StepDirection,
    max_decadal: usize,
) -> Option<(usize, u8)> {
    match direction {
        StepDirection::Forward => {
            if year < 9 {
                Some((decadal, year + 1))
            } else if decadal < max_decadal {
                Some((decadal + 1, 0))
            } else {
                None
            }
        }
        StepDirection::Backward => {
            if year > 0 {
                Some((decadal, year - 1))
            } else if decadal > 0 {
                Some((decadal - 1, 9))
            } else {
                None
            }
        }
    }
}

/// Carries a `(decadal, year, month)` tuple one 流月 step in `direction`, rolling
/// across both the 流年 and 大限 boundaries (forward past month 11 enters the next
/// 流年 month 0, backward past month 0 enters the previous 流年 month 11).
///
/// Returns `None` at the absolute boundary of the supported navigation range.
fn carry_month_tuple(
    decadal: usize,
    year: u8,
    month: u8,
    direction: StepDirection,
    max_decadal: usize,
) -> Option<(usize, u8, u8)> {
    match direction {
        StepDirection::Forward => {
            if month < 11 {
                Some((decadal, year, month + 1))
            } else {
                carry_year_pair(decadal, year, direction, max_decadal).map(|(d, y)| (d, y, 0))
            }
        }
        StepDirection::Backward => {
            if month > 0 {
                Some((decadal, year, month - 1))
            } else {
                carry_year_pair(decadal, year, direction, max_decadal).map(|(d, y)| (d, y, 11))
            }
        }
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

/// A saved chart record: a user-provided display name plus the normalized birth
/// input that deterministically rebuilds the chart. The name is metadata only;
/// the chart cache is keyed by [`BirthInput`], never the name.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedChart {
    /// User-provided display name shown in the saved-charts list.
    pub name: String,
    /// The normalized birth input that produced the chart.
    pub input: BirthInput,
}

/// A default display name for a saved chart, used to pre-fill the form and to
/// migrate legacy unnamed records, e.g. `1990-05-17 女 辰时`.
///
/// This is plain label formatting, not astrology derivation; it stays in the
/// renderer-agnostic layer so persistence migration can reuse it.
pub fn default_chart_name(input: &BirthInput, locale: Locale) -> String {
    let i18n = I18n::new(locale);
    format!(
        "{}-{:02}-{:02} {} {}",
        input.year,
        input.month,
        input.day,
        i18n.gender(input.gender),
        i18n.hour_branch(input.time_index),
    )
}

/// Editable, renderer-facing birth-input form (raw text plus typed choices).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BirthForm {
    /// User-provided chart name.
    pub name: String,
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
    /// Builds a form pre-filled from a normalized input, defaulting the name to
    /// [`default_chart_name`] in `locale`.
    pub fn from_input(input: &BirthInput, locale: Locale) -> Self {
        Self {
            name: default_chart_name(input, locale),
            year: input.year.to_string(),
            month: input.month.to_string(),
            day: input.day.to_string(),
            time_index: input.time_index,
            gender: input.gender,
        }
    }

    /// Builds a form pre-filled from a saved chart, preserving its display name.
    ///
    /// The saved name is kept verbatim, so the seeded default name (and therefore
    /// the locale) is irrelevant here.
    pub fn from_saved(saved: &SavedChart) -> Self {
        Self {
            name: saved.name.clone(),
            ..Self::from_input(&saved.input, Locale::EnUs)
        }
    }

    /// Parses and normalizes the form into a [`BirthInput`].
    ///
    /// Returns a typed [`FormError`] on a malformed numeric field so the renderer
    /// localizes it. Deep calendar validity (e.g. 31 February) is deferred to the
    /// facade at build time.
    pub fn parse(&self) -> Result<BirthInput, FormError> {
        let year: i32 = self
            .year
            .trim()
            .parse()
            .map_err(|_| FormError::YearInvalid)?;
        let month: u8 = self
            .month
            .trim()
            .parse()
            .map_err(|_| FormError::MonthInvalid)?;
        let day: u8 = self.day.trim().parse().map_err(|_| FormError::DayInvalid)?;

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
        Self::from_input(&SAMPLE_INPUT, Locale::EnUs)
    }
}

/// A user-facing form/generation error, kept as a typed value so the renderer
/// resolves the localized message at the presentation boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormError {
    /// The chart name was left blank.
    NameRequired,
    /// The year field was not a whole number.
    YearInvalid,
    /// The month field was not a whole number.
    MonthInvalid,
    /// The day field was not a whole number.
    DayInvalid,
    /// The date does not exist on the calendar (e.g. 31 February).
    InvalidCalendarDate,
    /// The selected birth time is out of the supported range.
    InvalidBirthTime,
    /// The selected temporal period/navigation is out of range.
    InvalidTemporalSelection,
    /// Chart generation failed for an otherwise-unspecified reason.
    ChartGenerationFailed,
    /// No local data directory is available, so charts can't be persisted.
    PersistenceUnavailable,
}

impl FormError {
    /// The stable Fluent key for this error, resolved by the renderer at the
    /// presentation boundary. Keeping errors typed (rather than carrying a raw
    /// core error string) is what lets the GUI localize them.
    pub fn fluent_key(self) -> &'static str {
        match self {
            FormError::NameRequired => "name-required",
            FormError::YearInvalid => "error-year",
            FormError::MonthInvalid => "error-month",
            FormError::DayInvalid => "error-day",
            FormError::InvalidCalendarDate => "error-invalid-calendar-date",
            FormError::InvalidBirthTime => "error-invalid-birth-time",
            FormError::InvalidTemporalSelection => "error-invalid-temporal-selection",
            FormError::ChartGenerationFailed => "error-chart-generation-failed",
            FormError::PersistenceUnavailable => "persistence-unavailable",
        }
    }
}

/// Maps a core [`ChartError`] to the most specific GUI [`FormError`], keeping the
/// error typed (never stringly-typed) across the GUI boundary so it localizes.
fn form_error_from_chart_error(error: ChartError) -> FormError {
    match error {
        ChartError::InvalidSolarMonth { .. } | ChartError::InvalidLunarMonth { .. } => {
            FormError::MonthInvalid
        }
        ChartError::InvalidSolarDay { .. } | ChartError::InvalidLunarDay { .. } => {
            FormError::DayInvalid
        }
        ChartError::InvalidBirthTimeIndex { .. } => FormError::InvalidBirthTime,
        ChartError::InvalidSolarDate { .. }
        | ChartError::UnsupportedCalendarDate { .. }
        | ChartError::CalendarConversionFailed { .. }
        | ChartError::UnsupportedLeapMonthCombination { .. }
        | ChartError::UnresolvableLunarDate { .. } => FormError::InvalidCalendarDate,
        ChartError::InvalidTemporalSelectionIndex { .. }
        | ChartError::InvalidDecadalPeriodIndex { .. }
        | ChartError::AnalysisLayerNotVisibleForSelection { .. }
        | ChartError::NominalAgeOutsideDecadalFrame { .. } => FormError::InvalidTemporalSelection,
        _ => FormError::ChartGenerationFailed,
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
    entries: HashMap<(BirthInput, StaticTemporalNavigationSelection), StaticChartProjection>,
    hits: u64,
    misses: u64,
}

impl ChartCache {
    /// Returns the cached pre-decadal snapshot for `input`, building it on a miss.
    pub fn get_or_build(
        &mut self,
        input: &BirthInput,
    ) -> Result<(StaticChartProjection, bool), ChartError> {
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
    ) -> Result<(StaticChartProjection, bool), ChartError> {
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
    /// Chart-name text field changed.
    NameChanged(String),
    /// Generate-chart action triggered.
    Generate,
    /// A saved chart selected by index; opens it in the chart view.
    SelectSaved(usize),
    /// A saved chart selected for editing; loads it into the form without
    /// generating, entering update mode for that record.
    EditSaved(usize),
    /// A saved chart removed from the list and persisted.
    DeleteSaved(usize),
    /// Leaves edit mode and resets the form to its default sample input.
    CancelEditSaved,
    /// A bottom temporal-navigation cell was clicked.
    SelectTemporalCell(TemporalCell),
    /// A compact stepper moved a temporal unit one step.
    StepTemporal(TemporalUnit, StepDirection),
    /// The `今` control was pressed. The Iced boundary reads the clock and
    /// dispatches [`SelectToday`](Self::SelectToday) with explicit facts.
    TodayPressed,
    /// The `今` control jumped to the supplied current local moment.
    SelectToday(LocalSolarMoment),
    /// The pointer entered the palace cell identified by its branch.
    HoverPalace(EarthlyBranch),
    /// The pointer left the palace cell identified by its branch.
    ///
    /// Carries the branch so a stale exit (for a palace the pointer already
    /// left) cannot clear a newer hover.
    ClearHoveredPalace(EarthlyBranch),
    /// Return to the startup screen.
    BackToStartup,
    /// Switch the active display locale.
    SetLocale(Locale),
    /// Switch the active GUI visual theme.
    SetTheme(GuiThemeId),
    /// Toggle the right inspector between hidden and visible (compact).
    ToggleRightPanel,
    /// Set the right inspector visibility/width mode.
    SetRightPanelMode(RightPanelMode),
    /// Switch the active right inspector tab.
    SetRightPanelTab(RightPanelTab),
    /// Toggle the expansion of one 全书规则 rule hit line.
    ToggleRuleHit(RuleHitExpansionKey),
    /// Toggle the expansion of one 格局 pattern hit line.
    TogglePatternHit(PatternHitExpansionKey),
}

/// Pure application state backing the static chart screen.
#[derive(Debug, Clone)]
pub struct StaticChartApp {
    screen: Screen,
    form: BirthForm,
    input: Option<BirthInput>,
    snapshot: Option<StaticChartProjection>,
    selected: PalaceSelection,
    hovered_palace: Option<EarthlyBranch>,
    selected_temporal_selection: StaticTemporalNavigationSelection,
    error: Option<FormError>,
    cache: ChartCache,
    saved: Vec<SavedChart>,
    /// Index into `saved` currently being edited, if any. When set, generating
    /// updates that record in place instead of appending a new one.
    editing_saved_index: Option<usize>,
    store: Option<ChartStore>,
    /// Persisted user preferences (locale + right-inspector layout). The locale
    /// lives here, not as a standalone field, so a locale change is one settings
    /// mutation that is persisted alongside the panel state.
    settings: AppSettings,
    settings_store: Option<SettingsStore>,
    /// In-memory, per-layer analysis cache for the current chart. Cleared when a
    /// new birth input is generated; never persisted.
    analysis_cache: AnalysisCache,
    /// The natal chart backing analysis for the current input, built once per
    /// input through the `by_solar` facade. `None` until a chart is generated.
    natal_chart: Option<Chart>,
    /// Which 全书规则 lines are expanded in the inspector.
    expanded_rule_hits: BTreeSet<RuleHitExpansionKey>,
    /// Which 格局 lines are expanded in the inspector.
    expanded_pattern_hits: BTreeSet<PatternHitExpansionKey>,
    /// The currently selected inspector hit driving chart highlighting, if any.
    /// Renderer-local only: never persisted, cleared when the chart input changes
    /// or temporal navigation invalidates its layer.
    active_analysis_selection: Option<ActiveAnalysisSelection>,
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
            selected: PalaceSelection::DefaultActiveLife,
            hovered_palace: None,
            selected_temporal_selection: StaticTemporalNavigationSelection::PreDecadal,
            error: None,
            cache: ChartCache::default(),
            saved: Vec::new(),
            editing_saved_index: None,
            store: None,
            settings: AppSettings::default(),
            settings_store: None,
            analysis_cache: AnalysisCache::default(),
            natal_chart: None,
            expanded_rule_hits: BTreeSet::new(),
            expanded_pattern_hits: BTreeSet::new(),
            active_analysis_selection: None,
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
                app.error = Some(FormError::PersistenceUnavailable);
                app
            }
        }
    }

    /// Builds an app from optional chart and settings stores, seeding both saved
    /// charts and persisted settings from disk where available.
    ///
    /// The chart store drives the same non-fatal persistence notice as
    /// [`with_optional_store`](Self::with_optional_store). The settings store is
    /// independent: when present, persisted preferences (locale + inspector
    /// layout) are loaded; when absent, in-memory defaults are used and settings
    /// changes simply are not written to disk.
    pub fn with_optional_stores(
        chart_store: Option<ChartStore>,
        settings_store: Option<SettingsStore>,
    ) -> Self {
        let settings = settings_store
            .as_ref()
            .map(SettingsStore::load)
            .unwrap_or_default();
        let mut app = Self::with_optional_store(chart_store);
        app.settings = settings;
        app.settings_store = settings_store;
        app
    }

    /// Replaces the saved-charts list (e.g. when seeding from persistence).
    pub fn set_saved(&mut self, saved: Vec<SavedChart>) {
        self.saved = saved;
    }

    /// The current top-level screen.
    pub fn screen(&self) -> Screen {
        self.screen
    }

    /// Returns the static chart snapshot driving the chart view, if any.
    pub fn snapshot(&self) -> Option<&StaticChartProjection> {
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
    pub fn error(&self) -> Option<&FormError> {
        self.error.as_ref()
    }

    /// The active display locale (read from persisted settings).
    pub fn locale(&self) -> Locale {
        self.settings.locale
    }

    /// The persisted user preferences (locale + right-inspector layout).
    pub fn settings(&self) -> &AppSettings {
        &self.settings
    }

    /// The current right inspector visibility/width mode.
    pub fn right_panel_mode(&self) -> RightPanelMode {
        self.settings.right_panel_mode
    }

    /// The active right inspector tab.
    pub fn right_panel_tab(&self) -> RightPanelTab {
        self.settings.right_panel_tab
    }

    /// The per-layer analysis cache backing the inspector (read-only).
    pub fn analysis_cache(&self) -> &AnalysisCache {
        &self.analysis_cache
    }

    /// The analysis layers the current temporal selection makes visible, in
    /// natal-outward order. The inspector reads cached results for these keys and
    /// hides any whose group is empty.
    pub fn required_analysis_layers(&self) -> Vec<AnalysisLayerKey> {
        analysis_layers_for_selection(self.selected_temporal_selection)
    }

    /// Whether the given 全书规则 line is currently expanded.
    pub fn is_rule_hit_expanded(&self, key: &RuleHitExpansionKey) -> bool {
        self.expanded_rule_hits.contains(key)
    }

    /// Whether the given 格局 line is currently expanded.
    pub fn is_pattern_hit_expanded(&self, key: &PatternHitExpansionKey) -> bool {
        self.expanded_pattern_hits.contains(key)
    }

    /// The currently selected inspector hit driving chart highlighting, if any.
    pub fn active_analysis_selection(&self) -> Option<&ActiveAnalysisSelection> {
        self.active_analysis_selection.as_ref()
    }

    /// Projects the current active analysis selection into a [`ChartHighlightView`]
    /// by reading the cached structured analysis result for its layer.
    ///
    /// Returns `None` when there is no active selection or the cached layer no
    /// longer contains the referenced hit (e.g. a temporal change just dropped
    /// the layer from the visible set). An empty highlight is still returned as
    /// `Some` so the caller can distinguish "no selection" from "selected but
    /// nothing safely projectable".
    pub fn active_chart_highlight(&self) -> Option<ChartHighlightView> {
        let selection = self.active_analysis_selection.as_ref()?;
        let result = self.analysis_cache.get(selection.layer())?;
        match selection {
            ActiveAnalysisSelection::Rule(key) => result
                .rule_hits
                .iter()
                .find(|hit| hit.rule_id == key.rule_id)
                .map(highlight_for_rule_hit),
            ActiveAnalysisSelection::Pattern(key) => result
                .pattern_hits
                .iter()
                .find(|hit| hit.id == key.pattern_id)
                .map(highlight_for_pattern_detection),
        }
    }

    /// Returns the chart cache (read-only).
    pub fn cache(&self) -> &ChartCache {
        &self.cache
    }

    /// Returns the saved charts, most recent last.
    pub fn saved(&self) -> &[SavedChart] {
        &self.saved
    }

    /// The index of the saved chart currently being edited, if any. When set,
    /// the next [`generate`](Self::generate) updates that record in place and
    /// the primary action should read as an update.
    pub fn editing_saved_index(&self) -> Option<usize> {
        self.editing_saved_index
    }

    /// Returns the twelve perimeter palaces of the current snapshot, if any.
    pub fn palaces(&self) -> &[StaticPalaceProjection] {
        self.snapshot
            .as_ref()
            .map(|snapshot| snapshot.palaces.as_slice())
            .unwrap_or(&[])
    }

    /// Returns the center-panel facts of the current snapshot, if any.
    pub fn center(&self) -> Option<&StaticChartCenterProjection> {
        self.snapshot.as_ref().map(|snapshot| &snapshot.center)
    }

    /// Returns the palace whose fixed grid position is `(row, column)`.
    ///
    /// Lookup is keyed by [`grid_position`], not by `Vec` order. Center cells and
    /// the empty-snapshot case return `None`.
    ///
    /// [`grid_position`]: iztro::StaticPalaceProjection::grid_position
    pub fn palace_at(&self, row: u8, column: u8) -> Option<&StaticPalaceProjection> {
        self.palaces().iter().find(|palace| {
            palace.grid_position.row() == row && palace.grid_position.column() == column
        })
    }

    /// Returns the branch of the currently selected palace, if any.
    pub fn selected_branch(&self) -> Option<EarthlyBranch> {
        if self.screen != Screen::Chart {
            return None;
        }
        match self.selected {
            PalaceSelection::DefaultActiveLife => self
                .snapshot
                .as_ref()
                .map(StaticChartProjection::active_life_branch),
            PalaceSelection::UserSelectedBranch(branch) => Some(branch),
        }
    }

    /// Returns the currently selected palace, if any.
    pub fn selected_palace(&self) -> Option<&StaticPalaceProjection> {
        let branch = self.selected_branch()?;
        self.palaces().iter().find(|palace| palace.branch == branch)
    }

    /// Returns the branch of the palace currently under the pointer, if any.
    pub fn hovered_palace(&self) -> Option<EarthlyBranch> {
        self.hovered_palace
    }

    /// The branch driving 三方四正 highlighting: hover takes priority over the
    /// sticky selection while the pointer is over a palace.
    pub fn active_branch(&self) -> Option<EarthlyBranch> {
        self.hovered_palace.or_else(|| self.selected_branch())
    }

    /// Returns the palace driving highlighting (hovered, else selected), if any.
    pub fn active_palace(&self) -> Option<&StaticPalaceProjection> {
        let branch = self.active_branch()?;
        self.palaces().iter().find(|palace| palace.branch == branch)
    }

    /// Returns the authoritative hierarchical temporal selection path.
    pub fn selected_temporal_selection(&self) -> StaticTemporalNavigationSelection {
        self.selected_temporal_selection
    }

    /// Whether `branch` is in the active palace's prepared 三方四正 set.
    ///
    /// The active palace is the hovered one, falling back to the sticky
    /// selection (which defaults to the active-frame 命宫 after generating).
    /// Reads the prepared [`surround`] field; performs no branch arithmetic.
    /// 三方四正 is always shown, matching the original iztro chart.
    ///
    /// [`surround`]: iztro::StaticPalaceProjection::surround
    pub fn is_in_san_fang(&self, branch: EarthlyBranch) -> bool {
        self.active_palace()
            .is_some_and(|palace| palace.surround.involves(branch))
    }

    /// Whether the active 三方四正 source is the active-frame 命宫 default
    /// (passive lines) rather than another user-clicked/hovered palace.
    pub fn san_fang_is_default(&self) -> bool {
        match (self.active_branch(), self.snapshot.as_ref()) {
            (Some(active), Some(snapshot)) => active == snapshot.active_life_branch(),
            // No active palace yet behaves like the default state.
            (None, _) => true,
            _ => false,
        }
    }

    /// Number of enabled 大限 cells in the current snapshot, used to roll/clamp
    /// decadal navigation at the final available period.
    fn enabled_decadal_count(&self) -> usize {
        self.snapshot
            .as_ref()
            .map(|snapshot| {
                snapshot
                    .temporal_panel
                    .decadal_cells
                    .iter()
                    .filter(|cell| cell.enabled)
                    .count()
            })
            .unwrap_or(0)
    }

    /// Number of enabled 流日 cells for the lunar month named by
    /// `(decadal, year, month)`, read from that month's prepared snapshot through
    /// the cache. Used to find a month's last valid day when rolling day/hour
    /// navigation across a month boundary (29- vs 30-day lunar months). Returns
    /// `0` when no chart is loaded or the month snapshot cannot be built.
    fn enabled_day_count_for(&mut self, decadal: usize, year: u8, month: u8) -> usize {
        let Some(input) = self.input else {
            return 0;
        };
        let selection = StaticTemporalNavigationSelection::Monthly {
            decadal_index: decadal,
            year_index: year,
            month_index: month,
        };
        match self.cache.get_or_build_with(&input, selection) {
            Ok((snapshot, _)) => snapshot
                .temporal_panel
                .day_rows
                .iter()
                .flatten()
                .filter(|cell| cell.enabled)
                .count(),
            Err(_) => 0,
        }
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
    ///
    /// A non-empty trimmed name is required; a blank name is reported without
    /// building. On success the named record is saved: editing an existing chart
    /// updates that record in place, otherwise a same-named record is updated and
    /// a new name is appended, then edit mode is cleared.
    pub fn generate(&mut self) -> GenerateOutcome {
        let name = self.form.name.trim().to_owned();
        if name.is_empty() {
            self.error = Some(FormError::NameRequired);
            return GenerateOutcome::Invalid;
        }

        let input = match self.form.parse() {
            Ok(input) => input,
            Err(error) => {
                self.error = Some(error);
                return GenerateOutcome::Invalid;
            }
        };

        match self.cache.get_or_build(&input) {
            Ok((snapshot, hit)) => {
                // Default the active 三方四正 source to the active-frame 命宫.
                self.selected = PalaceSelection::DefaultActiveLife;
                self.snapshot = Some(snapshot);
                // A new birth input invalidates the analysis cache and the natal
                // chart it was built from; the same input reuses them.
                if self.input != Some(input) {
                    self.analysis_cache.clear();
                    self.expanded_rule_hits.clear();
                    self.expanded_pattern_hits.clear();
                    self.natal_chart = build_request(&input).and_then(by_solar).ok();
                }
                // generate() always resets selected_temporal_selection to
                // PreDecadal, so any active selection on a deeper layer would
                // become stale. Clear it unconditionally on every successful
                // generate regardless of whether the input changed.
                self.active_analysis_selection = None;
                self.input = Some(input);
                self.hovered_palace = None;
                self.selected_temporal_selection = StaticTemporalNavigationSelection::PreDecadal;
                self.error = None;
                self.screen = Screen::Chart;
                self.refresh_analysis();
                self.save_record(SavedChart { name, input });
                if hit {
                    GenerateOutcome::CacheHit
                } else {
                    GenerateOutcome::Built
                }
            }
            Err(error) => {
                self.error = Some(form_error_from_chart_error(error));
                GenerateOutcome::Invalid
            }
        }
    }

    /// Stores a freshly generated record, then persists. Editing an existing
    /// chart updates that row in place; otherwise an existing same-named record
    /// is updated and a new name is appended, avoiding duplicate rows. Edit mode
    /// is cleared either way.
    fn save_record(&mut self, record: SavedChart) {
        match self.editing_saved_index.take() {
            Some(index) if index < self.saved.len() => self.saved[index] = record,
            _ => match self.saved.iter_mut().find(|s| s.name == record.name) {
                Some(existing) => *existing = record,
                None => self.saved.push(record),
            },
        }
        self.persist_saved();
    }

    /// Rebuilds the snapshot for a new temporal selection through the cache, so
    /// all overlay derivation stays in core. Errors are surfaced without
    /// disturbing the existing snapshot.
    fn apply_temporal_selection(&mut self, selection: StaticTemporalNavigationSelection) {
        let Some(input) = self.input else {
            return;
        };
        match self.cache.get_or_build_with(&input, selection) {
            Ok((snapshot, _)) => {
                self.selected_temporal_selection = selection;
                self.snapshot = Some(snapshot);
                self.error = None;
                // Request only the layers the new selection adds; cached ancestor
                // layers (本命/大限/…) are reused untouched.
                self.refresh_analysis();
                // A highlight pinned to a layer the new selection no longer makes
                // visible is stale and would point at an invisible hit; clear it.
                self.clear_stale_active_selection();
            }
            Err(error) => self.error = Some(form_error_from_chart_error(error)),
        }
    }

    /// Drops [`active_analysis_selection`] when its layer is no longer in the
    /// visible set for the current temporal selection.
    ///
    /// Layers that remain visible (e.g. `Yearly` while drilling deeper into the
    /// same year) keep their highlight; layers dropped by the navigation step
    /// (e.g. switching to a different `Yearly`) are released.
    ///
    /// [`active_analysis_selection`]: Self::active_analysis_selection
    fn clear_stale_active_selection(&mut self) {
        let Some(selection) = self.active_analysis_selection.as_ref() else {
            return;
        };
        let visible = analysis_layers_for_selection(self.selected_temporal_selection);
        if !visible.iter().any(|layer| layer == selection.layer()) {
            self.active_analysis_selection = None;
        }
    }

    /// Fills the analysis cache for the layers the current temporal selection
    /// makes visible, requesting detection only for the layers still missing.
    ///
    /// Detection goes through the core selected-view batch facade
    /// ([`detect_static_temporal_analysis_layers_from_chart`]): the GUI passes the
    /// natal [`Chart`], the current selection, and the missing layer keys, and core
    /// builds the temporal context and returns one compact result per requested
    /// layer. The GUI stays a cache/render layer and never constructs horoscope
    /// overlays itself. On failure the user-facing [`FormError`] is set the same
    /// way temporal selection failures are reported.
    fn refresh_analysis(&mut self) {
        let Some(natal) = self.natal_chart.as_ref() else {
            return;
        };
        let required = analysis_layers_for_selection(self.selected_temporal_selection);
        let missing = missing_analysis_layers(&required, &self.analysis_cache);
        if missing.is_empty() {
            return;
        }
        let request = AnalysisLayerRequest::user_facing();
        match detect_static_temporal_analysis_layers_from_chart(
            natal.clone(),
            self.selected_temporal_selection,
            &missing,
            &request,
        ) {
            Ok(results) => {
                for result in results {
                    self.analysis_cache.insert(result.key.clone(), result);
                }
            }
            Err(error) => self.error = Some(form_error_from_chart_error(error)),
        }
    }

    /// Carries a `(decadal, year, month, day)` tuple one 流日 step in `direction`,
    /// rolling across month/年/大限 boundaries. A month's last valid day is read
    /// from the prepared month snapshot, so 29- vs 30-day lunar months are
    /// handled by core, not the GUI. Returns `None` at the absolute boundary.
    fn carry_day_tuple(
        &mut self,
        decadal: usize,
        year: u8,
        month: u8,
        day: u8,
        direction: StepDirection,
        max_decadal: usize,
    ) -> Option<(usize, u8, u8, u8)> {
        match direction {
            StepDirection::Forward => {
                let count = self.enabled_day_count_for(decadal, year, month);
                if usize::from(day) + 1 < count {
                    Some((decadal, year, month, day + 1))
                } else {
                    carry_month_tuple(decadal, year, month, direction, max_decadal)
                        .map(|(d, y, m)| (d, y, m, 0))
                }
            }
            StepDirection::Backward => {
                if day > 0 {
                    Some((decadal, year, month, day - 1))
                } else {
                    carry_month_tuple(decadal, year, month, direction, max_decadal).map(
                        |(d, y, m)| {
                            let last = self.enabled_day_count_for(d, y, m).saturating_sub(1) as u8;
                            (d, y, m, last)
                        },
                    )
                }
            }
        }
    }

    /// Computes the temporal selection one `unit` step from the current selection
    /// in `direction`, rolling across period boundaries instead of clamping
    /// inside the current parent. Decadal navigation still clamps at the
    /// first/last 大限.
    ///
    /// Returns `None` when the step needs a parent index the current path lacks
    /// (keeping that control inert), and the current selection unchanged when
    /// already at the absolute first/last navigable moment.
    fn carry_stepped_selection(
        &mut self,
        unit: TemporalUnit,
        direction: StepDirection,
    ) -> Option<StaticTemporalNavigationSelection> {
        use StaticTemporalNavigationSelection as Sel;
        let current = self.selected_temporal_selection;
        let max_decadal = self.enabled_decadal_count().saturating_sub(1);
        match unit {
            TemporalUnit::Decadal => match (current.decadal_index(), direction) {
                (Some(index), StepDirection::Forward) => Some(Sel::Decadal {
                    decadal_index: (index + 1).min(max_decadal),
                }),
                (Some(index), StepDirection::Backward) => Some(Sel::Decadal {
                    decadal_index: index.saturating_sub(1),
                }),
                (None, StepDirection::Forward) => Some(Sel::Decadal { decadal_index: 0 }),
                (None, StepDirection::Backward) => None,
            },
            TemporalUnit::Year => {
                let decadal = current.decadal_index()?;
                match (current.year_index(), direction) {
                    (None, StepDirection::Forward) => Some(Sel::Yearly {
                        decadal_index: decadal,
                        year_index: 0,
                    }),
                    (None, StepDirection::Backward) => None,
                    (Some(year), _) => Some(
                        carry_year_pair(decadal, year, direction, max_decadal)
                            .map(|(d, y)| Sel::Yearly {
                                decadal_index: d,
                                year_index: y,
                            })
                            .unwrap_or(current),
                    ),
                }
            }
            TemporalUnit::Month => {
                let decadal = current.decadal_index()?;
                let year = current.year_index()?;
                match (current.month_index(), direction) {
                    (None, StepDirection::Forward) => Some(Sel::Monthly {
                        decadal_index: decadal,
                        year_index: year,
                        month_index: 0,
                    }),
                    (None, StepDirection::Backward) => None,
                    (Some(month), _) => Some(
                        carry_month_tuple(decadal, year, month, direction, max_decadal)
                            .map(|(d, y, m)| Sel::Monthly {
                                decadal_index: d,
                                year_index: y,
                                month_index: m,
                            })
                            .unwrap_or(current),
                    ),
                }
            }
            TemporalUnit::Day => {
                let decadal = current.decadal_index()?;
                let year = current.year_index()?;
                let month = current.month_index()?;
                match (current.day_index(), direction) {
                    (None, StepDirection::Forward) => Some(Sel::Daily {
                        decadal_index: decadal,
                        year_index: year,
                        month_index: month,
                        day_index: 0,
                    }),
                    (None, StepDirection::Backward) => None,
                    (Some(day), _) => Some(
                        self.carry_day_tuple(decadal, year, month, day, direction, max_decadal)
                            .map(|(d, y, m, dy)| Sel::Daily {
                                decadal_index: d,
                                year_index: y,
                                month_index: m,
                                day_index: dy,
                            })
                            .unwrap_or(current),
                    ),
                }
            }
            TemporalUnit::Hour => {
                let decadal = current.decadal_index()?;
                let year = current.year_index()?;
                let month = current.month_index()?;
                let day = current.day_index()?;
                match (current.hour_index(), direction) {
                    (None, StepDirection::Forward) => Some(Sel::Hourly {
                        decadal_index: decadal,
                        year_index: year,
                        month_index: month,
                        day_index: day,
                        hour_index: 0,
                    }),
                    (None, StepDirection::Backward) => None,
                    // The authoritative selection keeps 13 hour slots (0..=12:
                    // early 子..亥, late 子); forward past late 子 rolls to the next day.
                    (Some(hour), StepDirection::Forward) if hour < 12 => Some(Sel::Hourly {
                        decadal_index: decadal,
                        year_index: year,
                        month_index: month,
                        day_index: day,
                        hour_index: hour + 1,
                    }),
                    (Some(_), StepDirection::Forward) => Some(
                        self.carry_day_tuple(decadal, year, month, day, direction, max_decadal)
                            .map(|(d, y, m, dy)| Sel::Hourly {
                                decadal_index: d,
                                year_index: y,
                                month_index: m,
                                day_index: dy,
                                hour_index: 0,
                            })
                            .unwrap_or(current),
                    ),
                    (Some(hour), StepDirection::Backward) if hour > 0 => Some(Sel::Hourly {
                        decadal_index: decadal,
                        year_index: year,
                        month_index: month,
                        day_index: day,
                        hour_index: hour - 1,
                    }),
                    (Some(_), StepDirection::Backward) => Some(
                        self.carry_day_tuple(decadal, year, month, day, direction, max_decadal)
                            .map(|(d, y, m, dy)| Sel::Hourly {
                                decadal_index: d,
                                year_index: y,
                                month_index: m,
                                day_index: dy,
                                hour_index: 12,
                            })
                            .unwrap_or(current),
                    ),
                }
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
            Message::SelectPalace(branch) => {
                self.selected = PalaceSelection::UserSelectedBranch(branch)
            }
            Message::YearChanged(value) => self.form.year = value,
            Message::MonthChanged(value) => self.form.month = value,
            Message::DayChanged(value) => self.form.day = value,
            Message::TimeSelected(index) => self.form.time_index = index,
            Message::GenderSelected(gender) => self.form.gender = gender,
            Message::NameChanged(value) => self.form.name = value,
            Message::Generate => {
                self.generate();
            }
            Message::SelectSaved(index) => {
                if let Some(saved) = self.saved.get(index).cloned() {
                    // Opening a saved chart is not an edit: the same-named record
                    // is left untouched by the generate that follows.
                    self.editing_saved_index = None;
                    self.form = BirthForm::from_saved(&saved);
                    self.generate();
                }
            }
            Message::EditSaved(index) => {
                if let Some(saved) = self.saved.get(index) {
                    self.form = BirthForm::from_saved(saved);
                    self.editing_saved_index = Some(index);
                    self.error = None;
                }
            }
            Message::DeleteSaved(index) => {
                if index < self.saved.len() {
                    self.saved.remove(index);
                    // Keep the edit cursor valid: clear it if the edited row was
                    // removed, or shift it left if an earlier row was removed.
                    self.editing_saved_index = match self.editing_saved_index {
                        Some(editing) if editing == index => None,
                        Some(editing) if editing > index => Some(editing - 1),
                        other => other,
                    };
                    self.persist_saved();
                }
            }
            Message::CancelEditSaved => {
                self.editing_saved_index = None;
                self.form = BirthForm::default();
                self.error = None;
            }
            Message::SelectTemporalCell(cell) => {
                // Disabled cells stay inert: no selection, no snapshot change.
                if self.temporal_cell_enabled(cell) {
                    if let Some(selection) =
                        next_temporal_selection(self.selected_temporal_selection, cell)
                    {
                        self.apply_temporal_selection(selection);
                    }
                }
            }
            Message::StepTemporal(unit, direction) => {
                // A step that needs a missing parent index stays inert; a step at
                // the absolute boundary returns the current selection unchanged.
                if let Some(selection) = self.carry_stepped_selection(unit, direction) {
                    if selection != self.selected_temporal_selection {
                        self.apply_temporal_selection(selection);
                    }
                }
            }
            // The Iced boundary replaces this with SelectToday(moment). Keeping
            // the pure app branch inert preserves deterministic direct tests.
            Message::TodayPressed => {}
            Message::SelectToday(moment) => {
                if let Some(input) = self.input {
                    match resolve_today_selection(&input, moment) {
                        Ok(selection) => self.apply_temporal_selection(selection),
                        Err(error) => self.error = Some(form_error_from_chart_error(error)),
                    }
                }
            }
            Message::HoverPalace(branch) => self.hovered_palace = Some(branch),
            Message::ClearHoveredPalace(branch) => {
                // Ignore a stale exit so it cannot clear a newer hover.
                if self.hovered_palace == Some(branch) {
                    self.hovered_palace = None;
                }
            }
            Message::BackToStartup => {
                self.screen = Screen::Startup;
                self.selected = PalaceSelection::DefaultActiveLife;
                self.hovered_palace = None;
                self.selected_temporal_selection = StaticTemporalNavigationSelection::PreDecadal;
                self.clear_stale_active_selection();
            }
            Message::SetLocale(locale) => {
                let previous = self.settings.locale;
                if previous == locale {
                    return;
                }
                self.settings.locale = locale;
                // If the user has not customized the (auto-seeded) chart name,
                // re-seed it in the new locale so the default stays localized.
                if self.editing_saved_index.is_none() {
                    if let Ok(input) = self.form.parse() {
                        if self.form.name == default_chart_name(&input, previous) {
                            self.form.name = default_chart_name(&input, locale);
                        }
                    }
                }
                self.persist_settings();
            }
            Message::SetTheme(theme_id) => {
                self.settings.theme = theme_id;
                self.persist_settings();
            }
            Message::ToggleRightPanel => {
                // Toggle hides a visible panel and restores the compact panel from
                // hidden; the explicit mode controls live in the 设置 tab.
                let mode = match self.settings.right_panel_mode {
                    RightPanelMode::Hidden => RightPanelMode::Compact,
                    RightPanelMode::Compact | RightPanelMode::Expanded => RightPanelMode::Hidden,
                };
                self.set_right_panel_mode(mode);
            }
            Message::SetRightPanelMode(mode) => self.set_right_panel_mode(mode),
            Message::SetRightPanelTab(tab) => {
                if self.settings.right_panel_tab != tab {
                    self.settings.right_panel_tab = tab;
                    self.persist_settings();
                }
            }
            Message::ToggleRuleHit(key) => {
                let was_expanded = self.expanded_rule_hits.remove(&key);
                if !was_expanded {
                    self.expanded_rule_hits.insert(key.clone());
                    self.active_analysis_selection = Some(ActiveAnalysisSelection::Rule(key));
                } else if self.active_analysis_selection == Some(ActiveAnalysisSelection::Rule(key))
                {
                    // Collapsing the active row releases the highlight; another
                    // expanded row is still expanded but no longer the source.
                    self.active_analysis_selection = None;
                }
            }
            Message::TogglePatternHit(key) => {
                let was_expanded = self.expanded_pattern_hits.remove(&key);
                if !was_expanded {
                    self.expanded_pattern_hits.insert(key.clone());
                    self.active_analysis_selection = Some(ActiveAnalysisSelection::Pattern(key));
                } else if self.active_analysis_selection
                    == Some(ActiveAnalysisSelection::Pattern(key))
                {
                    self.active_analysis_selection = None;
                }
            }
        }
    }

    /// Sets the right inspector mode and persists it when it changes.
    fn set_right_panel_mode(&mut self, mode: RightPanelMode) {
        if self.settings.right_panel_mode != mode {
            self.settings.right_panel_mode = mode;
            self.persist_settings();
        }
    }

    /// Persists the settings to the backing store, if one is configured. A write
    /// failure is non-fatal: the in-memory settings stay authoritative.
    fn persist_settings(&mut self) {
        if let Some(store) = &self.settings_store {
            let _ = store.save(&self.settings);
        }
    }
}

impl Default for StaticChartApp {
    fn default() -> Self {
        Self::new()
    }
}

/// Builds a [`StaticChartProjection`] for `input` and `selection` through the
/// `static_temporal_chart_view` facade, so all temporal-overlay derivation stays
/// in core. Returns the facade error for invalid calendar input or selection.
fn build_snapshot(
    input: &BirthInput,
    selection: StaticTemporalNavigationSelection,
) -> Result<StaticChartProjection, ChartError> {
    static_temporal_chart_view(build_request(input)?, selection)
}

/// Builds the typed solar chart request for an input. Shared by snapshot
/// building and the `今` selection resolver.
fn build_request(input: &BirthInput) -> Result<SolarChartRequest, ChartError> {
    SolarChartRequest::builder()
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
        .build()
}

/// Resolves the temporal selection pointing at `moment` ("today") for `input`,
/// delegating all calendar/age mapping to the core facade.
fn resolve_today_selection(
    input: &BirthInput,
    moment: LocalSolarMoment,
) -> Result<StaticTemporalNavigationSelection, ChartError> {
    temporal_selection_for_solar_moment(
        build_request(input)?,
        moment.year,
        moment.month,
        moment.day,
        moment.hour,
        moment.minute,
    )
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

    /// A saved record for `input` carrying its default chart name.
    fn saved_default(input: BirthInput) -> SavedChart {
        SavedChart {
            name: default_chart_name(&input, Locale::EnUs),
            input,
        }
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
        assert_eq!(
            app.form(),
            &BirthForm::from_input(&SAMPLE_INPUT, Locale::EnUs)
        );
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
        // A distinct name keeps this a separate saved row (saves dedupe by name).
        app.update(Message::NameChanged("第二张命盘".to_string()));
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
        assert_eq!(app.error(), Some(&FormError::PersistenceUnavailable));

        // Generation still works; the chart is tracked in memory only.
        assert_eq!(app.generate(), GenerateOutcome::Built);
        assert_eq!(app.screen(), Screen::Chart);
        assert_eq!(app.saved(), &[saved_default(SAMPLE_INPUT)]);
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
        assert_eq!(reloaded, vec![saved_default(SAMPLE_INPUT)]);
    }

    #[test]
    fn selecting_a_saved_chart_opens_it() {
        let mut app = StaticChartApp::new();
        app.set_saved(vec![saved_default(SAMPLE_INPUT)]);
        app.update(Message::SelectSaved(0));

        assert_eq!(app.screen(), Screen::Chart);
        assert_eq!(app.input(), Some(SAMPLE_INPUT));
        assert_eq!(app.palaces().len(), 12);
        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::PreDecadal
        );
        assert!(
            app.snapshot()
                .expect("saved chart snapshot")
                .temporal_panel
                .pre_decadal_cell
                .selected
        );
    }

    #[test]
    fn generating_requires_a_non_empty_name() {
        let mut app = StaticChartApp::new();
        app.update(Message::NameChanged("   ".to_string()));

        let outcome = app.generate();

        assert_eq!(outcome, GenerateOutcome::Invalid);
        assert!(app.error().is_some());
        assert_eq!(app.screen(), Screen::Startup);
        assert!(app.saved().is_empty());
    }

    #[test]
    fn generating_a_named_chart_saves_a_named_record() {
        let mut app = StaticChartApp::new();
        app.update(Message::NameChanged("我的命盘".to_string()));

        assert_eq!(app.generate(), GenerateOutcome::Built);
        assert_eq!(
            app.saved(),
            &[SavedChart {
                name: "我的命盘".to_string(),
                input: SAMPLE_INPUT,
            }]
        );
    }

    #[test]
    fn generating_trims_whitespace_around_the_name() {
        let mut app = StaticChartApp::new();
        app.update(Message::NameChanged("  命盘甲  ".to_string()));
        app.generate();
        assert_eq!(app.saved()[0].name, "命盘甲");
    }

    #[test]
    fn selecting_a_saved_chart_fills_the_form_including_its_name() {
        let mut app = StaticChartApp::new();
        let record = SavedChart {
            name: "命名命盘".to_string(),
            input: BirthInput {
                year: 1985,
                ..SAMPLE_INPUT
            },
        };
        app.set_saved(vec![record.clone()]);

        app.update(Message::SelectSaved(0));

        assert_eq!(app.form().name, "命名命盘");
        assert_eq!(app.form().year, "1985");
        assert_eq!(app.input(), Some(record.input));
        // Opening is not editing, and it leaves the single row untouched.
        assert_eq!(app.editing_saved_index(), None);
        assert_eq!(app.saved(), &[record]);
    }

    #[test]
    fn modifying_a_saved_chart_loads_it_and_updates_the_same_row() {
        let mut app = StaticChartApp::new();
        app.set_saved(vec![
            saved_default(SAMPLE_INPUT),
            saved_default(BirthInput {
                year: 2001,
                ..SAMPLE_INPUT
            }),
        ]);

        // Editing the second row loads it into the form in update mode.
        app.update(Message::EditSaved(1));
        assert_eq!(app.editing_saved_index(), Some(1));
        assert_eq!(app.form().year, "2001");
        assert_eq!(app.screen(), Screen::Startup);

        // Change the name and regenerate: the same row is updated in place, the
        // list length is unchanged, and edit mode is cleared.
        app.update(Message::NameChanged("改名命盘".to_string()));
        app.update(Message::YearChanged("2002".to_string()));
        app.update(Message::Generate);

        assert_eq!(app.saved().len(), 2);
        assert_eq!(app.saved()[1].name, "改名命盘");
        assert_eq!(app.saved()[1].input.year, 2002);
        assert_eq!(app.editing_saved_index(), None);
        assert_eq!(app.screen(), Screen::Chart);
    }

    #[test]
    fn deleting_a_saved_chart_removes_it_and_persists() {
        let dir = tempfile::tempdir().expect("temp dir");
        let store = crate::persistence::ChartStore::new(dir.path().join("charts.json"));

        let mut app = StaticChartApp::with_store(store.clone());
        let keep = saved_default(SAMPLE_INPUT);
        let drop = saved_default(BirthInput {
            year: 2001,
            ..SAMPLE_INPUT
        });
        app.set_saved(vec![keep.clone(), drop]);
        app.persist_saved();

        app.update(Message::DeleteSaved(1));

        assert_eq!(app.saved(), std::slice::from_ref(&keep));
        // The deletion is durable: a fresh app over the same store agrees.
        assert_eq!(store.load(), vec![keep]);
    }

    #[test]
    fn cancelling_edit_clears_edit_mode_and_resets_the_form() {
        let mut app = StaticChartApp::new();
        app.set_saved(vec![saved_default(BirthInput {
            year: 1985,
            ..SAMPLE_INPUT
        })]);
        app.update(Message::EditSaved(0));
        assert_eq!(app.editing_saved_index(), Some(0));

        app.update(Message::CancelEditSaved);

        assert_eq!(app.editing_saved_index(), None);
        assert_eq!(app.form(), &BirthForm::default());
        assert!(app.error().is_none());
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
    fn generated_chart_defaults_active_branch_to_natal_life_palace() {
        let mut app = StaticChartApp::new();
        app.generate();
        let life = app.snapshot().expect("snapshot").active_life_branch();
        // The default selection resolves through the active frame's 命宫.
        assert_eq!(app.selected_branch(), Some(life));
        assert_eq!(app.active_branch(), Some(life));
        assert!(app.san_fang_is_default());
    }

    #[test]
    fn default_selection_follows_temporal_active_life_palace() {
        let mut app = StaticChartApp::new();
        app.generate();
        let generated_default = app.selected_branch();

        app.update(Message::SelectTemporalCell(TemporalCell::Decadal(1)));

        let decadal_life = app
            .snapshot()
            .expect("decadal snapshot")
            .active_life_branch();
        assert_eq!(app.selected_branch(), Some(decadal_life));
        assert_eq!(app.active_branch(), Some(decadal_life));
        assert!(app.san_fang_is_default());
        assert_ne!(
            Some(decadal_life),
            generated_default,
            "sample decadal frame should move the active Life palace"
        );
    }

    #[test]
    fn user_selected_branch_stays_sticky_across_temporal_navigation() {
        let mut app = StaticChartApp::new();
        app.generate();
        let clicked = app
            .palaces()
            .iter()
            .map(|palace| palace.branch)
            .find(|branch| Some(*branch) != app.selected_branch())
            .expect("a non-default branch");

        app.update(Message::SelectPalace(clicked));
        app.update(Message::SelectTemporalCell(TemporalCell::Decadal(1)));

        assert_eq!(app.selected_branch(), Some(clicked));
        assert_eq!(app.active_branch(), Some(clicked));
        assert!(!app.san_fang_is_default());
    }

    #[test]
    fn selecting_a_palace_changes_the_active_branch_off_default() {
        let mut app = StaticChartApp::new();
        app.generate();
        // Pick a palace that is not the default 命宫.
        let life = app.center().and_then(|c| c.life_palace_branch).unwrap();
        let branch = app
            .palaces()
            .iter()
            .map(|p| p.branch)
            .find(|b| *b != life)
            .expect("a non-life palace");
        app.update(Message::SelectPalace(branch));
        assert_eq!(app.selected_branch(), Some(branch));
        assert_eq!(app.selected_palace().expect("selected").branch, branch);
        // Clicking a non-命宫 palace switches lines to the active (non-default) tone.
        assert!(!app.san_fang_is_default());
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
    fn hover_still_wins_over_default_active_life_selection() {
        let mut app = StaticChartApp::new();
        app.generate();
        app.update(Message::SelectTemporalCell(TemporalCell::Decadal(1)));
        let default = app.selected_branch().expect("default branch");
        let hovered = app
            .palaces()
            .iter()
            .map(|palace| palace.branch)
            .find(|branch| *branch != default)
            .expect("non-default branch");

        app.update(Message::HoverPalace(hovered));
        assert_eq!(app.active_branch(), Some(hovered));

        app.update(Message::ClearHoveredPalace(hovered));
        assert_eq!(app.active_branch(), Some(default));
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
    fn stepping_decadal_forward_enters_the_first_period_and_rebuilds_through_cache() {
        let mut app = StaticChartApp::new();
        app.generate();
        let misses_before = app.cache().misses();

        app.update(Message::StepTemporal(
            TemporalUnit::Decadal,
            StepDirection::Forward,
        ));

        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::Decadal { decadal_index: 0 }
        );
        // A fresh selection is built through the cache facade.
        assert!(app.cache().misses() > misses_before);
        assert!(
            app.snapshot()
                .expect("snapshot")
                .active_scopes
                .contains(&iztro::core::Scope::Decadal)
        );
    }

    #[test]
    fn stepping_a_unit_without_its_parent_path_is_inert() {
        let mut app = StaticChartApp::new();
        app.generate();
        let before = app.snapshot().cloned();

        // From 限前 there is no 大限, so stepping 流年 cannot resolve a parent.
        app.update(Message::StepTemporal(
            TemporalUnit::Year,
            StepDirection::Forward,
        ));

        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::PreDecadal
        );
        assert_eq!(app.snapshot().cloned(), before);
    }

    #[test]
    fn stepping_backward_without_a_selected_child_is_inert() {
        let mut app = StaticChartApp::new();
        app.generate();
        let before = app.snapshot().cloned();

        app.update(Message::StepTemporal(
            TemporalUnit::Decadal,
            StepDirection::Backward,
        ));

        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::PreDecadal
        );
        assert_eq!(app.snapshot().cloned(), before);
    }

    #[test]
    fn carry_year_pair_rolls_across_the_decadal_boundary() {
        // Within a 大限 the year just steps by one.
        assert_eq!(
            carry_year_pair(2, 4, StepDirection::Forward, 11),
            Some((2, 5))
        );
        assert_eq!(
            carry_year_pair(2, 4, StepDirection::Backward, 11),
            Some((2, 3))
        );
        // Year 9 forward enters the next 大限 at year 0.
        assert_eq!(
            carry_year_pair(2, 9, StepDirection::Forward, 11),
            Some((3, 0))
        );
        // Year 0 backward enters the previous 大限 at year 9.
        assert_eq!(
            carry_year_pair(2, 0, StepDirection::Backward, 11),
            Some((1, 9))
        );
        // Final 大限 last year forward / first 大限 first year backward: no roll.
        assert_eq!(carry_year_pair(11, 9, StepDirection::Forward, 11), None);
        assert_eq!(carry_year_pair(0, 0, StepDirection::Backward, 11), None);
    }

    #[test]
    fn carry_month_tuple_rolls_across_year_and_decadal_boundaries() {
        // Within a 流年 the month steps by one.
        assert_eq!(
            carry_month_tuple(2, 4, 5, StepDirection::Forward, 11),
            Some((2, 4, 6))
        );
        assert_eq!(
            carry_month_tuple(2, 4, 5, StepDirection::Backward, 11),
            Some((2, 4, 4))
        );
        // Month 11 forward enters the next 流年 month 0.
        assert_eq!(
            carry_month_tuple(2, 4, 11, StepDirection::Forward, 11),
            Some((2, 5, 0))
        );
        // Month 11 forward at year 9 carries through to the next 大限.
        assert_eq!(
            carry_month_tuple(2, 9, 11, StepDirection::Forward, 11),
            Some((3, 0, 0))
        );
        // Month 0 backward enters the previous 流年 month 11.
        assert_eq!(
            carry_month_tuple(2, 4, 0, StepDirection::Backward, 11),
            Some((2, 3, 11))
        );
        // Month 0 backward at year 0 carries to the previous 大限 year 9.
        assert_eq!(
            carry_month_tuple(2, 0, 0, StepDirection::Backward, 11),
            Some((1, 9, 11))
        );
        // Absolute boundary: no roll.
        assert_eq!(
            carry_month_tuple(11, 9, 11, StepDirection::Forward, 11),
            None
        );
        assert_eq!(
            carry_month_tuple(0, 0, 0, StepDirection::Backward, 11),
            None
        );
    }

    /// The last enabled decadal index of the freshly generated sample chart.
    fn last_decadal_index(app: &StaticChartApp) -> usize {
        app.snapshot()
            .expect("snapshot")
            .temporal_panel
            .decadal_cells
            .iter()
            .filter(|cell| cell.enabled)
            .count()
            - 1
    }

    /// Number of enabled 流日 cells of the month named by `(decadal, year, month)`.
    fn month_day_count(app: &mut StaticChartApp, decadal: usize, year: u8, month: u8) -> u8 {
        app.apply_temporal_selection(StaticTemporalNavigationSelection::Monthly {
            decadal_index: decadal,
            year_index: year,
            month_index: month,
        });
        app.snapshot()
            .expect("month snapshot")
            .temporal_panel
            .day_rows
            .iter()
            .flatten()
            .filter(|cell| cell.enabled)
            .count() as u8
    }

    #[test]
    fn decadal_stepping_still_clamps_at_the_first_and_last_period() {
        let mut app = StaticChartApp::new();
        app.generate();
        let last = last_decadal_index(&app);

        app.apply_temporal_selection(StaticTemporalNavigationSelection::Decadal {
            decadal_index: last,
        });
        app.update(Message::StepTemporal(
            TemporalUnit::Decadal,
            StepDirection::Forward,
        ));
        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::Decadal {
                decadal_index: last
            }
        );

        app.apply_temporal_selection(StaticTemporalNavigationSelection::Decadal {
            decadal_index: 0,
        });
        app.update(Message::StepTemporal(
            TemporalUnit::Decadal,
            StepDirection::Backward,
        ));
        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::Decadal { decadal_index: 0 }
        );
    }

    #[test]
    fn year_stepping_rolls_across_the_decadal_boundary() {
        let mut app = StaticChartApp::new();
        app.generate();

        // 年▶ from the last 流年 of a 大限 advances into the next 大限 year 0.
        app.apply_temporal_selection(StaticTemporalNavigationSelection::Yearly {
            decadal_index: 0,
            year_index: 9,
        });
        app.update(Message::StepTemporal(
            TemporalUnit::Year,
            StepDirection::Forward,
        ));
        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::Yearly {
                decadal_index: 1,
                year_index: 0,
            }
        );

        // ◀年 from the first 流年 of a 大限 steps back into the previous 大限 year 9.
        app.apply_temporal_selection(StaticTemporalNavigationSelection::Yearly {
            decadal_index: 1,
            year_index: 0,
        });
        app.update(Message::StepTemporal(
            TemporalUnit::Year,
            StepDirection::Backward,
        ));
        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::Yearly {
                decadal_index: 0,
                year_index: 9,
            }
        );
    }

    #[test]
    fn year_stepping_stays_at_the_absolute_first_year() {
        // The upper-boundary "stay" semantics are covered by the pure
        // `carry_year_pair` test; the topmost 大限's final 流年 (ages 111-120) is
        // beyond the chart's supported lunar range and cannot be selected anyway.
        let mut app = StaticChartApp::new();
        app.generate();

        app.apply_temporal_selection(StaticTemporalNavigationSelection::Yearly {
            decadal_index: 0,
            year_index: 0,
        });
        app.update(Message::StepTemporal(
            TemporalUnit::Year,
            StepDirection::Backward,
        ));
        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::Yearly {
                decadal_index: 0,
                year_index: 0,
            }
        );
    }

    #[test]
    fn month_stepping_rolls_across_year_and_decadal_boundaries() {
        let mut app = StaticChartApp::new();
        app.generate();

        // 月▶ from month 11 carries into the next 流年 month 0.
        app.apply_temporal_selection(StaticTemporalNavigationSelection::Monthly {
            decadal_index: 0,
            year_index: 0,
            month_index: 11,
        });
        app.update(Message::StepTemporal(
            TemporalUnit::Month,
            StepDirection::Forward,
        ));
        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::Monthly {
                decadal_index: 0,
                year_index: 1,
                month_index: 0,
            }
        );

        // 月▶ from month 11 of year 9 carries through to the next 大限.
        app.apply_temporal_selection(StaticTemporalNavigationSelection::Monthly {
            decadal_index: 0,
            year_index: 9,
            month_index: 11,
        });
        app.update(Message::StepTemporal(
            TemporalUnit::Month,
            StepDirection::Forward,
        ));
        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::Monthly {
                decadal_index: 1,
                year_index: 0,
                month_index: 0,
            }
        );

        // ◀月 from month 0 carries into the previous 流年 month 11.
        app.apply_temporal_selection(StaticTemporalNavigationSelection::Monthly {
            decadal_index: 0,
            year_index: 1,
            month_index: 0,
        });
        app.update(Message::StepTemporal(
            TemporalUnit::Month,
            StepDirection::Backward,
        ));
        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::Monthly {
                decadal_index: 0,
                year_index: 0,
                month_index: 11,
            }
        );
    }

    #[test]
    fn day_stepping_rolls_across_the_month_boundary_using_prepared_day_counts() {
        let mut app = StaticChartApp::new();
        app.generate();

        // 日▶ from the last enabled day of month 0 carries into month 1 day 0.
        let count = month_day_count(&mut app, 0, 0, 0);
        app.apply_temporal_selection(StaticTemporalNavigationSelection::Daily {
            decadal_index: 0,
            year_index: 0,
            month_index: 0,
            day_index: count - 1,
        });
        app.update(Message::StepTemporal(
            TemporalUnit::Day,
            StepDirection::Forward,
        ));
        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::Daily {
                decadal_index: 0,
                year_index: 0,
                month_index: 1,
                day_index: 0,
            }
        );

        // ◀日 from day 0 of month 1 carries back to month 0's last valid day,
        // whose length (29 vs 30) comes from the prepared month snapshot.
        let prev_count = month_day_count(&mut app, 0, 0, 0);
        app.apply_temporal_selection(StaticTemporalNavigationSelection::Daily {
            decadal_index: 0,
            year_index: 0,
            month_index: 1,
            day_index: 0,
        });
        app.update(Message::StepTemporal(
            TemporalUnit::Day,
            StepDirection::Backward,
        ));
        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::Daily {
                decadal_index: 0,
                year_index: 0,
                month_index: 0,
                day_index: prev_count - 1,
            }
        );
    }

    #[test]
    fn hour_stepping_rolls_across_the_day_boundary() {
        let mut app = StaticChartApp::new();
        app.generate();

        // 时▶ from the late-子 slot (index 12) carries into the next day hour 0.
        app.apply_temporal_selection(StaticTemporalNavigationSelection::Hourly {
            decadal_index: 0,
            year_index: 0,
            month_index: 0,
            day_index: 0,
            hour_index: 12,
        });
        app.update(Message::StepTemporal(
            TemporalUnit::Hour,
            StepDirection::Forward,
        ));
        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::Hourly {
                decadal_index: 0,
                year_index: 0,
                month_index: 0,
                day_index: 1,
                hour_index: 0,
            }
        );

        // ◀时 from hour 0 carries back to the previous day's late-子 slot (12).
        app.apply_temporal_selection(StaticTemporalNavigationSelection::Hourly {
            decadal_index: 0,
            year_index: 0,
            month_index: 0,
            day_index: 1,
            hour_index: 0,
        });
        app.update(Message::StepTemporal(
            TemporalUnit::Hour,
            StepDirection::Backward,
        ));
        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::Hourly {
                decadal_index: 0,
                year_index: 0,
                month_index: 0,
                day_index: 0,
                hour_index: 12,
            }
        );
    }

    #[test]
    fn selecting_today_jumps_to_a_dated_selection_with_nominal_age() {
        let mut app = StaticChartApp::new();
        // Spec birth data: solar 1993-05-27, 酉时 (index 9), male.
        app.update(Message::YearChanged("1993".to_string()));
        app.update(Message::MonthChanged("5".to_string()));
        app.update(Message::DayChanged("27".to_string()));
        app.update(Message::TimeSelected(9));
        app.update(Message::GenderSelected(Gender::Male));
        app.generate();

        app.update(Message::SelectToday(LocalSolarMoment {
            year: 2008,
            month: 2,
            day: 10,
            hour: 10,
            minute: 0,
        }));

        // 2008 is the 16th nominal year (虚岁) for a 1993 birth.
        assert_eq!(
            app.center().and_then(|c| c.nominal_age_label.as_deref()),
            Some("16 岁")
        );
        assert!(matches!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::Hourly { .. }
        ));
        assert_eq!(
            app.center().and_then(|c| c.temporal_solar_label.as_deref()),
            Some("2008-2-10")
        );
    }

    #[test]
    fn gui_source_does_not_derive_astrology_facts() {
        const FORBIDDEN: [&str; 27] = [
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
            "StaticSurroundProjection::for_branch",
            "birth_year_star_mutagen",
            "birth_year_major_star_mutagen",
            // Temporal overlays must be prepared by the `static_temporal_chart_view`
            // facade; the GUI must never construct a horoscope, temporal layer,
            // or decadal frame itself.
            "build_decadal_horoscope_chart",
            "build_partial_horoscope_chart",
            "build_decadal_horoscope_layer",
            "build_full_horoscope_chart",
            "build_yearly_period",
            "build_monthly_period",
            "build_daily_period",
            "build_hourly_period",
            "resolve_non_leap_lunar",
            "target_lunar_date",
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

        // Analysis must go through the selected-view batch facade, not the
        // natal-only workaround removed in this PR.
        let app_production = production_source(&app_src);
        assert!(
            !app_production.contains("TemporalAnalysisContext::natal"),
            "refresh_analysis must not build a natal-only analysis context"
        );
        assert!(
            app_production.contains("detect_static_temporal_analysis_layers_from_chart"),
            "analysis must request layers through the selected-view batch facade"
        );
    }

    #[test]
    fn generating_runs_only_the_natal_analysis_layer() {
        let mut app = StaticChartApp::new();
        app.generate();
        let cache = app.analysis_cache();
        assert!(cache.contains(&AnalysisLayerKey::Natal));
        assert_eq!(cache.len(), 1);
        // The natal view needs no further layers.
        assert!(missing_analysis_layers(&app.required_analysis_layers(), cache).is_empty());
    }

    #[test]
    fn moving_from_natal_to_decadal_does_not_re_request_natal() {
        let mut app = StaticChartApp::new();
        app.generate();

        // Planning the decadal view leaves only the decadal layer missing; the
        // cached natal layer is reused.
        let required = analysis_layers_for_selection(StaticTemporalNavigationSelection::Decadal {
            decadal_index: 0,
        });
        assert_eq!(
            missing_analysis_layers(&required, app.analysis_cache()),
            vec![AnalysisLayerKey::Decadal { decadal_index: 0 }]
        );

        app.update(Message::SelectTemporalCell(TemporalCell::Decadal(0)));
        assert!(app.analysis_cache().contains(&AnalysisLayerKey::Natal));
        assert!(
            app.analysis_cache()
                .contains(&AnalysisLayerKey::Decadal { decadal_index: 0 })
        );
    }

    #[test]
    fn moving_within_the_same_year_requests_only_the_new_monthly_layer() {
        let mut app = StaticChartApp::new();
        app.generate();
        app.update(Message::SelectTemporalCell(TemporalCell::Decadal(0)));
        app.update(Message::SelectTemporalCell(TemporalCell::YearlyAge(0)));

        // Cache holds the full yearly chain.
        for key in app.required_analysis_layers() {
            assert!(
                app.analysis_cache().contains(&key),
                "{key:?} should be cached"
            );
        }

        // A monthly view under the same 流年 leaves only the 流月 layer missing —
        // 本命/大限/小限/流年 are reused, never re-requested.
        let monthly = StaticTemporalNavigationSelection::Monthly {
            decadal_index: 0,
            year_index: 0,
            month_index: 0,
        };
        assert_eq!(
            missing_analysis_layers(
                &analysis_layers_for_selection(monthly),
                app.analysis_cache()
            ),
            vec![AnalysisLayerKey::Monthly {
                decadal_index: 0,
                year_index: 0,
                month_index: 0,
            }]
        );
    }

    #[test]
    fn moving_from_monthly_to_daily_requests_only_daily() {
        let mut app = StaticChartApp::new();
        app.generate();
        app.update(Message::SelectTemporalCell(TemporalCell::Decadal(0)));
        app.update(Message::SelectTemporalCell(TemporalCell::YearlyAge(0)));
        app.update(Message::SelectTemporalCell(TemporalCell::Month(0)));

        let daily = StaticTemporalNavigationSelection::Daily {
            decadal_index: 0,
            year_index: 0,
            month_index: 0,
            day_index: 0,
        };
        assert_eq!(
            missing_analysis_layers(&analysis_layers_for_selection(daily), app.analysis_cache()),
            vec![AnalysisLayerKey::Daily {
                decadal_index: 0,
                year_index: 0,
                month_index: 0,
                day_index: 0,
            }]
        );
    }

    #[test]
    fn a_new_birth_input_clears_the_analysis_cache() {
        let mut app = StaticChartApp::new();
        app.generate();
        app.update(Message::SelectTemporalCell(TemporalCell::Decadal(0)));
        app.update(Message::SelectTemporalCell(TemporalCell::YearlyAge(0)));
        assert!(app.analysis_cache().len() > 1);

        // A different birth input resets analysis to just the new natal layer.
        app.update(Message::YearChanged("2000".to_string()));
        app.update(Message::NameChanged("第二张命盘".to_string()));
        app.generate();

        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::PreDecadal
        );
        assert_eq!(app.analysis_cache().len(), 1);
        assert!(app.analysis_cache().contains(&AnalysisLayerKey::Natal));
    }

    #[test]
    fn regenerating_same_birth_input_keeps_the_analysis_cache() {
        let mut app = StaticChartApp::new();
        app.generate();
        app.update(Message::SelectTemporalCell(TemporalCell::Decadal(0)));
        app.update(Message::SelectTemporalCell(TemporalCell::YearlyAge(0)));

        let decadal = AnalysisLayerKey::Decadal { decadal_index: 0 };
        let yearly = AnalysisLayerKey::Yearly {
            decadal_index: 0,
            year_index: 0,
        };
        assert!(app.analysis_cache().contains(&decadal));
        assert!(app.analysis_cache().contains(&yearly));
        let cached_layers = app.analysis_cache().len();

        app.generate();

        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::PreDecadal
        );
        assert_eq!(app.analysis_cache().len(), cached_layers);
        assert!(app.analysis_cache().contains(&decadal));
        assert!(app.analysis_cache().contains(&yearly));
    }

    #[test]
    fn toggling_the_right_panel_hides_then_restores_compact() {
        let mut app = StaticChartApp::new();
        assert_eq!(app.right_panel_mode(), RightPanelMode::Compact);
        app.update(Message::ToggleRightPanel);
        assert_eq!(app.right_panel_mode(), RightPanelMode::Hidden);
        app.update(Message::ToggleRightPanel);
        assert_eq!(app.right_panel_mode(), RightPanelMode::Compact);
    }

    #[test]
    fn changing_locale_mode_and_tab_persists_through_the_settings_store() {
        let dir = tempfile::tempdir().expect("temp dir");
        let chart_store = crate::persistence::ChartStore::new(dir.path().join("charts.json"));
        let settings_store = crate::settings::SettingsStore::new(dir.path().join("settings.json"));
        let mut app =
            StaticChartApp::with_optional_stores(Some(chart_store), Some(settings_store.clone()));

        app.update(Message::SetLocale(Locale::ZhHans));
        app.update(Message::SetRightPanelMode(RightPanelMode::Expanded));
        app.update(Message::SetRightPanelTab(RightPanelTab::Patterns));

        // A fresh load from the same store sees every persisted preference.
        let reloaded = settings_store.load();
        assert_eq!(reloaded.locale, Locale::ZhHans);
        assert_eq!(reloaded.right_panel_mode, RightPanelMode::Expanded);
        assert_eq!(reloaded.right_panel_tab, RightPanelTab::Patterns);
    }

    #[test]
    fn settings_default_when_no_store_is_configured() {
        let app = StaticChartApp::new();
        assert_eq!(app.settings(), &AppSettings::default());
        assert_eq!(app.right_panel_tab(), RightPanelTab::QuanShuRules);
    }

    fn rule_key(layer: AnalysisLayerKey, id: &str) -> RuleHitExpansionKey {
        RuleHitExpansionKey {
            layer,
            rule_id: iztro::rules::classical::ClassicalRuleId::new(id),
        }
    }

    fn pattern_key(
        layer: AnalysisLayerKey,
        pattern_id: iztro::core::PatternId,
    ) -> PatternHitExpansionKey {
        PatternHitExpansionKey { layer, pattern_id }
    }

    fn different_key_same_scope(key: &AnalysisLayerKey) -> AnalysisLayerKey {
        match key {
            AnalysisLayerKey::Decadal { decadal_index } => AnalysisLayerKey::Decadal {
                decadal_index: if *decadal_index == 0 { 1 } else { 0 },
            },
            AnalysisLayerKey::Age {
                decadal_index,
                year_index,
            } => AnalysisLayerKey::Age {
                decadal_index: *decadal_index,
                year_index: if *year_index == 0 { 1 } else { 0 },
            },
            AnalysisLayerKey::Yearly {
                decadal_index,
                year_index,
            } => AnalysisLayerKey::Yearly {
                decadal_index: *decadal_index,
                year_index: if *year_index == 0 { 1 } else { 0 },
            },
            AnalysisLayerKey::Monthly {
                decadal_index,
                year_index,
                month_index,
            } => AnalysisLayerKey::Monthly {
                decadal_index: *decadal_index,
                year_index: *year_index,
                month_index: if *month_index == 0 { 1 } else { 0 },
            },
            AnalysisLayerKey::Daily {
                decadal_index,
                year_index,
                month_index,
                day_index,
            } => AnalysisLayerKey::Daily {
                decadal_index: *decadal_index,
                year_index: *year_index,
                month_index: *month_index,
                day_index: if *day_index == 0 { 1 } else { 0 },
            },
            AnalysisLayerKey::Hourly {
                decadal_index,
                year_index,
                month_index,
                day_index,
                hour_index,
            } => AnalysisLayerKey::Hourly {
                decadal_index: *decadal_index,
                year_index: *year_index,
                month_index: *month_index,
                day_index: *day_index,
                hour_index: if *hour_index == 0 { 1 } else { 0 },
            },
            AnalysisLayerKey::Natal => AnalysisLayerKey::Decadal { decadal_index: 0 },
        }
    }

    fn first_cached_non_natal_pattern_hit(
        app: &mut StaticChartApp,
    ) -> (AnalysisLayerKey, iztro::core::PatternId) {
        app.generate();
        app.update(Message::SelectTemporalCell(TemporalCell::Decadal(0)));

        for year_index in 0..10 {
            app.update(Message::SelectTemporalCell(TemporalCell::YearlyAge(
                year_index,
            )));
            let yearly = AnalysisLayerKey::Yearly {
                decadal_index: 0,
                year_index,
            };
            if let Some(pattern_id) = app
                .analysis_cache()
                .get(&yearly)
                .and_then(|result| result.pattern_hits.first())
                .map(|hit| hit.id)
            {
                return (yearly, pattern_id);
            }

            for month_index in 0..12 {
                app.update(Message::SelectTemporalCell(TemporalCell::Month(
                    month_index,
                )));
                let monthly = AnalysisLayerKey::Monthly {
                    decadal_index: 0,
                    year_index,
                    month_index,
                };
                if let Some(pattern_id) = app
                    .analysis_cache()
                    .get(&monthly)
                    .and_then(|result| result.pattern_hits.first())
                    .map(|hit| hit.id)
                {
                    return (monthly, pattern_id);
                }
            }
        }

        panic!("expected at least one cached non-natal pattern hit");
    }

    #[test]
    fn clicking_a_rule_hit_sets_the_active_analysis_selection() {
        let mut app = StaticChartApp::new();
        app.generate();
        let key = rule_key(AnalysisLayerKey::Natal, "test.rule");
        app.update(Message::ToggleRuleHit(key.clone()));
        assert_eq!(
            app.active_analysis_selection(),
            Some(&ActiveAnalysisSelection::Rule(key.clone()))
        );
        // The row is also expanded so the structured detail is visible alongside
        // the highlight.
        assert!(app.is_rule_hit_expanded(&key));
    }

    #[test]
    fn clicking_a_pattern_hit_sets_the_active_analysis_selection() {
        let mut app = StaticChartApp::new();
        app.generate();
        let key = pattern_key(
            AnalysisLayerKey::Natal,
            iztro::core::PatternId::ZiFuChaoYuan,
        );
        app.update(Message::TogglePatternHit(key.clone()));
        assert_eq!(
            app.active_analysis_selection(),
            Some(&ActiveAnalysisSelection::Pattern(key.clone()))
        );
        assert!(app.is_pattern_hit_expanded(&key));
    }

    #[test]
    fn pattern_expansion_is_keyed_by_layer_and_pattern_id() {
        let mut app = StaticChartApp::new();
        app.generate();
        let natal = pattern_key(
            AnalysisLayerKey::Natal,
            iztro::core::PatternId::ChangQuJiaMing,
        );
        let decadal = pattern_key(
            AnalysisLayerKey::Decadal { decadal_index: 0 },
            iztro::core::PatternId::ChangQuJiaMing,
        );

        app.update(Message::TogglePatternHit(natal.clone()));
        assert!(app.is_pattern_hit_expanded(&natal));
        assert!(!app.is_pattern_hit_expanded(&decadal));

        app.update(Message::TogglePatternHit(decadal.clone()));
        assert!(app.is_pattern_hit_expanded(&natal));
        assert!(app.is_pattern_hit_expanded(&decadal));

        app.update(Message::TogglePatternHit(natal.clone()));
        assert!(!app.is_pattern_hit_expanded(&natal));
        assert!(app.is_pattern_hit_expanded(&decadal));
    }

    #[test]
    fn cached_non_natal_pattern_hit_uses_exact_layer_key_for_expansion() {
        let mut app = StaticChartApp::new();
        let (layer, pattern_id) = first_cached_non_natal_pattern_hit(&mut app);
        assert_ne!(layer, AnalysisLayerKey::Natal);

        let result = app
            .analysis_cache()
            .get(&layer)
            .expect("non-natal layer should be cached by exact key");
        assert_eq!(result.key, layer);
        assert!(!result.pattern_hits.is_empty());
        assert!(
            result
                .pattern_hits
                .iter()
                .all(|hit| hit.scope == layer.pattern_scope())
        );

        let exact = pattern_key(layer.clone(), pattern_id);
        let same_scope_other_key = pattern_key(different_key_same_scope(&layer), pattern_id);
        app.update(Message::TogglePatternHit(exact.clone()));

        assert!(app.is_pattern_hit_expanded(&exact));
        assert!(!app.is_pattern_hit_expanded(&same_scope_other_key));
        assert_eq!(
            app.active_analysis_selection(),
            Some(&ActiveAnalysisSelection::Pattern(exact))
        );
    }

    #[test]
    fn collapsing_the_active_rule_hit_releases_the_selection() {
        let mut app = StaticChartApp::new();
        app.generate();
        let key = rule_key(AnalysisLayerKey::Natal, "test.rule");
        app.update(Message::ToggleRuleHit(key.clone()));
        app.update(Message::ToggleRuleHit(key.clone()));
        assert!(app.active_analysis_selection().is_none());
        assert!(!app.is_rule_hit_expanded(&key));
    }

    #[test]
    fn generating_a_new_chart_clears_the_active_analysis_selection() {
        let mut app = StaticChartApp::new();
        app.generate();
        let key = rule_key(AnalysisLayerKey::Natal, "test.rule");
        app.update(Message::ToggleRuleHit(key));
        assert!(app.active_analysis_selection().is_some());

        // A different birth input must clear the analysis cache *and* the
        // highlight that points at a now-invisible hit.
        app.update(Message::YearChanged("2000".to_string()));
        app.update(Message::NameChanged("第二张命盘".to_string()));
        app.generate();
        assert!(app.active_analysis_selection().is_none());
    }

    #[test]
    fn temporal_navigation_keeps_active_selection_when_layer_stays_visible() {
        let mut app = StaticChartApp::new();
        app.generate();
        app.update(Message::SelectTemporalCell(TemporalCell::Decadal(0)));
        let key = rule_key(AnalysisLayerKey::Decadal { decadal_index: 0 }, "test.rule");
        app.update(Message::ToggleRuleHit(key.clone()));
        // Drilling deeper into the same 大限 keeps the 大限 layer visible, so the
        // 大限-anchored highlight survives.
        app.update(Message::SelectTemporalCell(TemporalCell::YearlyAge(0)));
        assert_eq!(
            app.active_analysis_selection(),
            Some(&ActiveAnalysisSelection::Rule(key))
        );
    }

    #[test]
    fn temporal_navigation_clears_active_selection_when_layer_drops_off() {
        let mut app = StaticChartApp::new();
        app.generate();
        app.update(Message::SelectTemporalCell(TemporalCell::Decadal(0)));
        let key = rule_key(AnalysisLayerKey::Decadal { decadal_index: 0 }, "test.rule");
        app.update(Message::ToggleRuleHit(key));
        // Switching to a different 大限 drops the previously selected 大限 layer
        // from the visible set; the highlight pinned to it is released.
        app.update(Message::SelectTemporalCell(TemporalCell::Decadal(1)));
        assert!(app.active_analysis_selection().is_none());
    }

    #[test]
    fn active_chart_highlight_reads_the_cached_layer_result_for_a_pattern() {
        let mut app = StaticChartApp::new();
        app.generate();
        // Drive a real pattern detection by selecting whatever pattern hit shows
        // up on the seeded sample chart's natal layer, if any. If the sample
        // chart happens to produce no pattern hits, the assertion is vacuous;
        // the structural projection is still covered by analysis.rs unit tests.
        if let Some(detection) = app
            .analysis_cache()
            .get(&AnalysisLayerKey::Natal)
            .and_then(|result| result.pattern_hits.first())
            .cloned()
        {
            let key = pattern_key(AnalysisLayerKey::Natal, detection.id);
            app.update(Message::TogglePatternHit(key));
            let view = app.active_chart_highlight().expect("active highlight");
            // The view is whatever the cached detection's involved_* fields
            // project to; an entirely empty detection is allowed but rare.
            for branch in &detection.involved_palaces {
                assert!(view.highlights_palace(*branch));
            }
            for star in &detection.involved_stars {
                assert!(view.star_names.contains(star));
            }
            for mutagen in &detection.involved_mutagens {
                assert!(view.mutagens.contains(mutagen));
            }
        }
    }

    #[test]
    fn active_chart_highlight_is_none_when_nothing_is_selected() {
        let mut app = StaticChartApp::new();
        app.generate();
        assert!(app.active_chart_highlight().is_none());
    }

    #[test]
    fn regenerating_same_input_clears_the_active_analysis_selection() {
        // generate() always resets selected_temporal_selection to PreDecadal,
        // so any active selection anchored to a deeper layer (e.g. Decadal)
        // would point at a layer that is no longer visible. generate() must
        // clear active_analysis_selection unconditionally, not just when the
        // birth input changes.
        let mut app = StaticChartApp::new();
        app.generate();
        app.update(Message::SelectTemporalCell(TemporalCell::Decadal(0)));
        let key = rule_key(AnalysisLayerKey::Decadal { decadal_index: 0 }, "test.rule");
        app.update(Message::ToggleRuleHit(key));
        assert!(app.active_analysis_selection().is_some());

        // Regenerate with the same input: selected_temporal_selection resets to
        // PreDecadal and the Decadal-anchored active selection must be released.
        app.generate();
        assert_eq!(
            app.selected_temporal_selection(),
            StaticTemporalNavigationSelection::PreDecadal
        );
        assert!(
            app.active_analysis_selection().is_none(),
            "active selection must be cleared on every successful generate()"
        );
    }
}
