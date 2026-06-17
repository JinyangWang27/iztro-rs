//! Pure application state and selection logic for the static chart GUI.
//!
//! This module is renderer-agnostic: it depends only on `iztro` read models and
//! never on `iced`. It builds one hardcoded sample chart, converts it into a
//! [`StaticChartViewSnapshot`], and exposes deterministic, testable accessors
//! that the Iced layer renders. No astrology placement, rule evaluation, or
//! 成格 detection lives here.

use iztro::core::{
    ChartAlgorithmKind, EarthlyBranch, Gender, MethodProfile, SolarChartRequest, SolarDay,
    SolarMonth, StaticChartCenterView, StaticChartViewSnapshot, StaticPalaceView, by_solar,
};

/// Side length of the fixed visual palace grid (4x4 perimeter layout).
pub const GRID_SIZE: u8 = 4;

/// The four center grid cells that hold the center panel, never a palace.
pub const CENTER_CELLS: [(u8, u8); 4] = [(1, 1), (1, 2), (2, 1), (2, 2)];

/// Messages the GUI emits into the pure update loop.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    /// The user clicked the palace cell identified by its branch.
    SelectPalace(EarthlyBranch),
}

/// Pure application state backing the static chart screen.
#[derive(Debug, Clone)]
pub struct StaticChartApp {
    snapshot: StaticChartViewSnapshot,
    selected: Option<EarthlyBranch>,
}

impl StaticChartApp {
    /// Builds the app from the hardcoded sample snapshot, with no selection.
    pub fn new() -> Self {
        Self {
            snapshot: sample_snapshot(),
            selected: None,
        }
    }

    /// Returns the immutable static chart snapshot driving the view.
    pub fn snapshot(&self) -> &StaticChartViewSnapshot {
        &self.snapshot
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
    /// Lookup is keyed by [`grid_position`], not by `Vec` order, so the renderer
    /// places each palace in its conventional cell. Center cells return `None`.
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

    /// Applies a message to the state.
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SelectPalace(branch) => self.selected = Some(branch),
        }
    }
}

impl Default for StaticChartApp {
    fn default() -> Self {
        Self::new()
    }
}

/// Builds the single hardcoded sample [`StaticChartViewSnapshot`] from existing
/// `iztro` facade APIs. Deterministic: same input pillars every run.
pub fn sample_snapshot() -> StaticChartViewSnapshot {
    let request = SolarChartRequest::builder()
        .solar_year(1990)
        .solar_month(SolarMonth::new(5).expect("May is a valid month"))
        .solar_day(SolarDay::new(17).expect("day 17 is valid"))
        .birth_time(EarthlyBranch::Chen)
        .gender(Gender::Female)
        .method_profile(MethodProfile::new(
            "iztro_gui_sample",
            ChartAlgorithmKind::QuanShu,
            "iztro-gui static chart prototype sample",
        ))
        .build()
        .expect("hardcoded sample solar chart request must build");

    let chart = by_solar(request).expect("hardcoded sample chart must build via by_solar");
    StaticChartViewSnapshot::from_chart(&chart)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_app_holds_a_valid_static_chart_snapshot() {
        let app = StaticChartApp::new();
        // A valid snapshot has a populated center and the natal scope active.
        assert!(!app.center().birth_year_stem_zh.is_empty());
        assert!(!app.palaces().is_empty());
    }

    #[test]
    fn snapshot_has_exactly_twelve_palace_cells() {
        let app = StaticChartApp::new();
        assert_eq!(app.palaces().len(), 12);
    }

    #[test]
    fn palaces_are_mapped_by_grid_position_not_vec_order() {
        let app = StaticChartApp::new();

        // Every palace is reachable at its own declared grid position.
        for palace in app.palaces() {
            let pos = palace.grid_position;
            let found = app
                .palace_at(pos.row(), pos.column())
                .expect("palace must be reachable at its grid position");
            assert_eq!(found.branch, palace.branch);
        }

        // The four center cells never hold a palace.
        for (row, column) in CENTER_CELLS {
            assert!(
                app.palace_at(row, column).is_none(),
                "center cell ({row},{column}) must be empty"
            );
        }

        // Grid lookup must not coincide with naive Vec/row-major order: the
        // palace at vec index 5 lives at grid (1,3), while naive row-major index
        // 5 would be center cell (1,1).
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
        assert_eq!(
            app.selected_palace().expect("a palace is selected").branch,
            branch
        );
    }

    #[test]
    fn center_panel_data_is_present() {
        let app = StaticChartApp::new();
        let center = app.center();
        assert!(!center.birth_year_stem_zh.is_empty());
        assert!(!center.birth_year_branch_zh.is_empty());
        assert!(center.five_element_bureau.is_some());
        assert!(center.life_palace_branch.is_some());
    }

    #[test]
    fn natal_star_groups_stay_separate_from_overlays() {
        let app = StaticChartApp::new();
        // A natal-only snapshot carries no temporal overlays on any palace, so
        // natal star groups can never be merged with overlay facts.
        for palace in app.palaces() {
            assert!(
                palace.overlays.is_empty(),
                "natal-only snapshot must have no overlays"
            );
        }
        // At least one palace must carry natal typed stars, proving the natal
        // groups are populated independently of the (empty) overlays.
        let has_natal_stars = app.palaces().iter().any(|palace| {
            !palace.major_stars.is_empty()
                || !palace.minor_stars.is_empty()
                || !palace.adjective_stars.is_empty()
        });
        assert!(has_natal_stars, "natal star groups must be populated");
    }

    #[test]
    fn gui_source_does_not_call_placement_modules_directly() {
        // The GUI must consume snapshots through the facade only. Direct
        // placement / star-math symbols would mean the GUI re-implements
        // astrology logic, which the architecture forbids.
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

        // Charts must be built only through the public facade entry point.
        let app_src = std::fs::read_to_string(src_dir.join("app.rs")).expect("app.rs must read");
        assert!(
            app_src.contains("by_solar"),
            "the sample chart must be built through the by_solar facade"
        );
    }
}
