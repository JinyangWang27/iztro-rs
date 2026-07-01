//! The right-side analysis inspector: 全书规则 / 格局 / 设置 tabs.
//!
//! The inspector is a collapsible side panel rendered *outside* the scrollable
//! chart canvas, so the fixed-size chart never shrinks to make room for it. It
//! reads only cached, structured analysis values prepared by core
//! ([`AnalysisLayerResult`]); it performs no rule evaluation, pattern detection,
//! or overlay derivation. Verbatim QuanShu source text is resolved once per rule
//! id through [`classical_rule_metadata`] rather than stored in GUI state.
//!
//! [`AnalysisLayerResult`]: iztro::analysis::AnalysisLayerResult

use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Element, Length};

use iztro::PatternStrength;
use iztro::analysis::AnalysisLayerKey;
use iztro::rules::classical::classical_rule_metadata;
use iztro_i18n::I18n;

use crate::analysis::{PatternHitExpansionKey, RuleHitExpansionKey};
use crate::app::{Message, StaticChartApp};
use crate::settings::{GuiThemeId, RightPanelMode, RightPanelTab};

use super::style::{
    input_panel_style, inspector_row_style, pill_badge, secondary_text_style, section_title_style,
    segmented_track_style, subtle_text_style,
};
use super::theme::{CHART_LAYOUT, GuiPalette, SPACING, TYPE};

/// Compact-mode inspector width.
const RIGHT_PANEL_COMPACT_WIDTH: f32 = CHART_LAYOUT.inspector_compact_width;
/// Expanded-mode inspector width.
const RIGHT_PANEL_EXPANDED_WIDTH: f32 = CHART_LAYOUT.inspector_expanded_width;

/// Renders the right inspector for the current mode, or `None` when hidden.
///
/// Returning `None` lets the caller drop the panel from the layout row entirely
/// rather than reserving a zero-width slot.
pub(super) fn right_inspector<'a>(
    app: &'a StaticChartApp,
    palette: GuiPalette,
    i18n: &I18n,
) -> Option<Element<'a, Message>> {
    let width = match app.right_panel_mode() {
        RightPanelMode::Hidden => return None,
        RightPanelMode::Compact => RIGHT_PANEL_COMPACT_WIDTH,
        RightPanelMode::Expanded => RIGHT_PANEL_EXPANDED_WIDTH,
    };

    let body = match app.right_panel_tab() {
        RightPanelTab::QuanShuRules => rules_tab(app, palette, i18n),
        RightPanelTab::Patterns => patterns_tab(app, palette, i18n),
        RightPanelTab::Settings => settings_tab(app, palette, i18n),
    };

    let panel = column![tab_bar(app, palette, i18n), body]
        .spacing(SPACING.lg)
        .padding(SPACING.xl)
        .width(Length::Fill);

    Some(
        container(panel)
            .style(input_panel_style(palette))
            .width(Length::Fixed(width))
            .height(Length::Fill)
            .into(),
    )
}

/// The three-tab selector, rendered as a segmented control sitting on a recessed
/// track so it reads as one grouped control rather than three loose buttons.
fn tab_bar<'a>(app: &'a StaticChartApp, palette: GuiPalette, i18n: &I18n) -> Element<'a, Message> {
    let active = app.right_panel_tab();
    let tab = |label: &str, value: RightPanelTab| {
        let style = if active == value {
            button::primary
        } else {
            button::text
        };
        button(
            text(i18n.text(label))
                .size(TYPE.label)
                .align_x(Alignment::Center),
        )
        .on_press(Message::SetRightPanelTab(value))
        .style(style)
        .padding([SPACING.sm, SPACING.lg])
        .width(Length::FillPortion(1))
    };
    let segments = row![
        tab(
            "right-panel-tab-quan-shu-rules",
            RightPanelTab::QuanShuRules
        ),
        tab("right-panel-tab-patterns", RightPanelTab::Patterns),
        tab("right-panel-tab-settings", RightPanelTab::Settings),
    ]
    .spacing(SPACING.xs);

    container(segments)
        .style(segmented_track_style(palette))
        .padding(SPACING.xs)
        .width(Length::Fill)
        .into()
}

/// The 全书规则 tab: cached QuanShu rule hits grouped by layer scope.
fn rules_tab<'a>(
    app: &'a StaticChartApp,
    palette: GuiPalette,
    i18n: &I18n,
) -> Element<'a, Message> {
    let mut content = column![].spacing(SPACING.lg);
    let mut any = false;

    for key in app.required_analysis_layers() {
        let Some(result) = app.analysis_cache().get(&key) else {
            continue;
        };
        if result.rule_hits.is_empty() {
            continue;
        }
        any = true;
        let scope_label = i18n.analysis_scope_label(key.scope());
        let mut group = column![
            text(scope_label)
                .size(TYPE.body)
                .style(section_title_style(palette))
        ]
        .spacing(SPACING.md);
        for hit in &result.rule_hits {
            group = group.push(rule_hit_line(app, &key, hit, palette, i18n));
        }
        content = content.push(group);
    }

    if any {
        scrollable(content).height(Length::Fill).into()
    } else {
        empty_notice(palette, i18n.text("rules-panel-empty"))
    }
}

/// One collapsed/expanded 全书规则 line. The collapsed line shows only the
/// scope-prefixed verbatim source text; expansion reveals localized claim text.
fn rule_hit_line<'a>(
    app: &StaticChartApp,
    key: &AnalysisLayerKey,
    hit: &iztro::rules::classical::ClassicalRuleHitRef,
    palette: GuiPalette,
    i18n: &I18n,
) -> Element<'a, Message> {
    let metadata = classical_rule_metadata(hit.rule_id.clone());
    let source_text = metadata
        .map(|m| m.source_text_zh_hans.to_owned())
        .unwrap_or_else(|| i18n.text("rules-panel-unknown-rule"));

    let expansion_key = RuleHitExpansionKey {
        layer: key.clone(),
        rule_id: hit.rule_id.clone(),
    };
    let expanded = app.is_rule_hit_expanded(&expansion_key);

    let header = row![text(source_text).size(TYPE.body)]
        .spacing(SPACING.md)
        .align_y(Alignment::Center);

    let mut block = column![
        button(header)
            .on_press(Message::ToggleRuleHit(expansion_key))
            .style(button::text)
            .padding(0)
            .width(Length::Fill),
    ]
    .spacing(SPACING.xs);

    if expanded {
        let claim = hit
            .claim_key
            .as_deref()
            .map(|claim_key| i18n.claim_text_by_key(claim_key))
            .unwrap_or_else(|| i18n.text("rules-panel-no-claim"));
        block = block.push(
            text(claim)
                .size(TYPE.label)
                .style(subtle_text_style(palette))
                .width(Length::Fill),
        );
    }

    inspector_card(palette, block)
}

/// Wraps inspector row content in a compact card surface.
fn inspector_card<'a>(
    palette: GuiPalette,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    container(content)
        .style(inspector_row_style(palette))
        .padding([SPACING.md, SPACING.lg])
        .width(Length::Fill)
        .into()
}

/// The 格局 tab: all detected patterns across all layers, each row showing
/// scope · name · polarity inline (no separate scope-group headers).
fn patterns_tab<'a>(
    app: &'a StaticChartApp,
    palette: GuiPalette,
    i18n: &I18n,
) -> Element<'a, Message> {
    let mut content = column![].spacing(SPACING.md);
    let mut any = false;

    for key in app.required_analysis_layers() {
        let Some(result) = app.analysis_cache().get(&key) else {
            continue;
        };
        for detection in &result.pattern_hits {
            any = true;
            content = content.push(pattern_hit_line(app, &key, detection, palette, i18n));
        }
    }

    if any {
        scrollable(content).height(Length::Fill).into()
    } else {
        empty_notice(palette, i18n.text("patterns-panel-empty"))
    }
}

/// One collapsed/expanded 格局 line, e.g. `本命· 紫府朝垣（成格）`.
fn pattern_hit_line<'a>(
    app: &StaticChartApp,
    key: &AnalysisLayerKey,
    detection: &iztro::PatternDetection,
    palette: GuiPalette,
    i18n: &I18n,
) -> Element<'a, Message> {
    let expansion_key = PatternHitExpansionKey {
        layer: key.clone(),
        pattern_id: detection.id,
    };
    let expanded = app.is_pattern_hit_expanded(&expansion_key);

    let scope_label = i18n.analysis_scope_label(key.scope());
    let polarity_label = i18n.pattern_polarity_label(detection.polarity);
    let header_text = format!(
        "{} · {} · {}",
        scope_label, detection.name_zh, polarity_label
    );

    let header = row![
        text(header_text).size(TYPE.body),
        iced::widget::horizontal_space(),
        pill_badge(palette, i18n.pattern_status_label(detection.status)),
    ]
    .spacing(SPACING.md)
    .align_y(Alignment::Center);

    let mut block = column![
        button(header)
            .on_press(Message::TogglePatternHit(expansion_key))
            .style(button::text)
            .padding(0)
            .width(Length::Fill),
    ]
    .spacing(SPACING.xs);

    if expanded {
        block = block.push(pattern_details(detection, palette, i18n));
    }

    inspector_card(palette, block)
}

/// Basic structured detail rows for an expanded pattern: strength plus the
/// involved stars / palaces / mutagens. No narrative prose is added.
fn pattern_details<'a>(
    detection: &iztro::PatternDetection,
    palette: GuiPalette,
    i18n: &I18n,
) -> Element<'a, Message> {
    let mut rows = column![
        detail_row(
            palette,
            &i18n.text("patterns-detail-polarity"),
            &i18n.pattern_polarity_label(detection.polarity),
        ),
        detail_row(
            palette,
            &i18n.text("patterns-detail-strength"),
            &pattern_strength_label(detection.strength, i18n),
        ),
    ]
    .spacing(2);

    if !detection.involved_stars.is_empty() {
        let stars = detection
            .involved_stars
            .iter()
            .map(|star| i18n.star_name(*star))
            .collect::<Vec<_>>()
            .join("、");
        rows = rows.push(detail_row(
            palette,
            &i18n.text("patterns-detail-stars"),
            &stars,
        ));
    }
    if !detection.involved_palaces.is_empty() {
        let palaces = detection
            .involved_palaces
            .iter()
            .map(|branch| i18n.branch(*branch))
            .collect::<Vec<_>>()
            .join("、");
        rows = rows.push(detail_row(
            palette,
            &i18n.text("patterns-detail-palaces"),
            &palaces,
        ));
    }
    if !detection.involved_mutagens.is_empty() {
        let mutagens = detection
            .involved_mutagens
            .iter()
            .map(|mutagen| i18n.mutagen(*mutagen))
            .collect::<Vec<_>>()
            .join("、");
        rows = rows.push(detail_row(
            palette,
            &i18n.text("patterns-detail-mutagens"),
            &mutagens,
        ));
    }

    rows.into()
}

/// A structured `label / value` detail row: a muted label over the ink value,
/// so expanded details read as a small key-value table rather than prose.
fn detail_row<'a>(palette: GuiPalette, label: &str, value: &str) -> Element<'a, Message> {
    row![
        text(label.to_owned())
            .size(TYPE.label)
            .style(secondary_text_style(palette))
            .width(Length::FillPortion(2)),
        text(value.to_owned())
            .size(TYPE.label)
            .width(Length::FillPortion(3)),
    ]
    .spacing(SPACING.md)
    .into()
}

/// Localized coarse pattern-strength label.
fn pattern_strength_label(strength: PatternStrength, i18n: &I18n) -> String {
    i18n.text(match strength {
        PatternStrength::Weak => "pattern-strength-weak",
        PatternStrength::Medium => "pattern-strength-medium",
        PatternStrength::Strong => "pattern-strength-strong",
    })
}

/// The 设置 tab: language selection and sidebar mode controls.
fn settings_tab<'a>(
    app: &'a StaticChartApp,
    palette: GuiPalette,
    i18n: &I18n,
) -> Element<'a, Message> {
    let language = {
        let choice = |label: &str, locale| {
            let selected = app.locale() == locale;
            let style = if selected {
                button::primary
            } else {
                button::secondary
            };
            button(text(i18n.text(label)).size(TYPE.label))
                .on_press(Message::SetLocale(locale))
                .style(style)
                .padding([SPACING.sm, SPACING.lg])
        };
        column![
            text(i18n.text("settings-language"))
                .size(TYPE.body)
                .style(section_title_style(palette)),
            row![
                choice("ui-english", iztro_i18n::Locale::EnUs),
                choice("ui-simplified-chinese", iztro_i18n::Locale::ZhHans),
            ]
            .spacing(SPACING.md),
        ]
        .spacing(SPACING.sm)
    };

    let theme = {
        let active = app.settings().theme;
        let choice = |label: &str, theme_id: GuiThemeId| {
            let style = if active == theme_id {
                button::primary
            } else {
                button::secondary
            };
            button(text(i18n.text(label)).size(TYPE.label))
                .on_press(Message::SetTheme(theme_id))
                .style(style)
                .padding([SPACING.sm, SPACING.lg])
        };
        column![
            text(i18n.text("settings-theme"))
                .size(TYPE.body)
                .style(section_title_style(palette)),
            row![
                choice("theme-ink-paper", GuiThemeId::InkPaper),
                choice("theme-jade-light", GuiThemeId::JadeLight),
                choice("theme-deep-ink", GuiThemeId::DeepInk),
            ]
            .spacing(SPACING.md),
        ]
        .spacing(SPACING.sm)
    };

    let sidebar = {
        let active = app.right_panel_mode();
        let choice = |label: &str, mode: RightPanelMode| {
            let style = if active == mode {
                button::primary
            } else {
                button::secondary
            };
            button(text(i18n.text(label)).size(TYPE.label))
                .on_press(Message::SetRightPanelMode(mode))
                .style(style)
                .padding([SPACING.sm, SPACING.lg])
        };
        column![
            text(i18n.text("settings-sidebar-mode"))
                .size(TYPE.body)
                .style(section_title_style(palette)),
            row![
                choice("settings-sidebar-hidden", RightPanelMode::Hidden),
                choice("settings-sidebar-compact", RightPanelMode::Compact),
                choice("settings-sidebar-expanded", RightPanelMode::Expanded),
            ]
            .spacing(SPACING.md),
        ]
        .spacing(SPACING.sm)
    };

    scrollable(
        column![
            text(i18n.text("settings-panel-title")).size(TYPE.heading),
            language,
            theme,
            sidebar,
        ]
        .spacing(SPACING.xxl),
    )
    .height(Length::Fill)
    .into()
}

/// A centered muted empty-state notice for a tab with no visible groups.
fn empty_notice<'a>(palette: GuiPalette, message: String) -> Element<'a, Message> {
    container(
        text(message)
            .size(TYPE.body)
            .style(subtle_text_style(palette)),
    )
    .padding(8)
    .width(Length::Fill)
    .align_x(Alignment::Center)
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{Message, StaticChartApp};
    use iztro_i18n::Locale;

    /// The active palette under test: the default (InkPaper) theme.
    fn test_palette() -> GuiPalette {
        *super::super::theme::palette(GuiThemeId::InkPaper)
    }

    #[test]
    fn hidden_mode_removes_the_inspector_from_the_layout() {
        let mut app = StaticChartApp::new();
        app.update(Message::SetRightPanelMode(RightPanelMode::Hidden));
        let i18n = I18n::new(Locale::EnUs);
        assert!(right_inspector(&app, test_palette(), &i18n).is_none());
    }

    #[test]
    fn patterns_tab_renders_with_generated_chart() {
        let mut app = StaticChartApp::new();
        app.generate();
        app.update(Message::SetRightPanelMode(RightPanelMode::Compact));
        app.update(Message::SetRightPanelTab(RightPanelTab::Patterns));
        let i18n = I18n::new(Locale::ZhHans);
        assert!(right_inspector(&app, test_palette(), &i18n).is_some());
    }

    #[test]
    fn settings_tab_renders_for_every_theme() {
        let i18n = I18n::new(Locale::EnUs);
        for theme in [
            GuiThemeId::InkPaper,
            GuiThemeId::JadeLight,
            GuiThemeId::DeepInk,
        ] {
            let mut app = StaticChartApp::new();
            app.update(Message::SetTheme(theme));
            app.update(Message::SetRightPanelTab(RightPanelTab::Settings));
            assert!(right_inspector(&app, test_palette(), &i18n).is_some());
        }
    }

    #[test]
    fn visible_modes_render_a_panel_for_every_tab() {
        let i18n = I18n::new(Locale::EnUs);
        for mode in [RightPanelMode::Compact, RightPanelMode::Expanded] {
            for tab in [
                RightPanelTab::QuanShuRules,
                RightPanelTab::Patterns,
                RightPanelTab::Settings,
            ] {
                let mut app = StaticChartApp::new();
                app.generate();
                app.update(Message::SetRightPanelMode(mode));
                app.update(Message::SetRightPanelTab(tab));
                assert!(
                    right_inspector(&app, test_palette(), &i18n).is_some(),
                    "{mode:?}/{tab:?} should render"
                );
            }
        }
    }
}
