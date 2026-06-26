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

use iztro::analysis::AnalysisLayerKey;
use iztro::core::PatternStrength;
use iztro::rules::classical::classical_rule_metadata;
use iztro_i18n::I18n;

use crate::analysis::{PatternHitExpansionKey, RuleHitExpansionKey};
use crate::app::{Message, StaticChartApp};
use crate::settings::{RightPanelMode, RightPanelTab};

use super::style::{input_panel_style, section_title_style, subtle_text_style};

/// Compact-mode inspector width.
const RIGHT_PANEL_COMPACT_WIDTH: f32 = 280.0;
/// Expanded-mode inspector width.
const RIGHT_PANEL_EXPANDED_WIDTH: f32 = 360.0;

/// Renders the right inspector for the current mode, or `None` when hidden.
///
/// Returning `None` lets the caller drop the panel from the layout row entirely
/// rather than reserving a zero-width slot.
pub(super) fn right_inspector<'a>(
    app: &'a StaticChartApp,
    i18n: &I18n,
) -> Option<Element<'a, Message>> {
    let width = match app.right_panel_mode() {
        RightPanelMode::Hidden => return None,
        RightPanelMode::Compact => RIGHT_PANEL_COMPACT_WIDTH,
        RightPanelMode::Expanded => RIGHT_PANEL_EXPANDED_WIDTH,
    };

    let body = match app.right_panel_tab() {
        RightPanelTab::QuanShuRules => rules_tab(app, i18n),
        RightPanelTab::Patterns => patterns_tab(app, i18n),
        RightPanelTab::Settings => settings_tab(app, i18n),
    };

    let panel = column![tab_bar(app, i18n), body]
        .spacing(10)
        .padding(12)
        .width(Length::Fill);

    Some(
        container(panel)
            .style(input_panel_style)
            .width(Length::Fixed(width))
            .height(Length::Fill)
            .into(),
    )
}

/// The three-tab selector row.
fn tab_bar<'a>(app: &'a StaticChartApp, i18n: &I18n) -> Element<'a, Message> {
    let active = app.right_panel_tab();
    let tab = |label: &str, value: RightPanelTab| {
        let style = if active == value {
            button::primary
        } else {
            button::secondary
        };
        button(text(i18n.text(label)).size(12))
            .on_press(Message::SetRightPanelTab(value))
            .style(style)
            .padding([4, 8])
    };
    row![
        tab("right-panel-tab-quan-shu-rules", RightPanelTab::QuanShuRules),
        tab("right-panel-tab-patterns", RightPanelTab::Patterns),
        tab("right-panel-tab-settings", RightPanelTab::Settings),
    ]
    .spacing(6)
    .into()
}

/// The 全书规则 tab: cached QuanShu rule hits grouped by layer scope.
fn rules_tab<'a>(app: &'a StaticChartApp, i18n: &I18n) -> Element<'a, Message> {
    let mut content = column![].spacing(10);
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
        let mut group = column![text(scope_label).size(13).style(section_title_style)].spacing(4);
        for hit in &result.rule_hits {
            group = group.push(rule_hit_line(app, &key, hit, i18n));
        }
        content = content.push(group);
    }

    if any {
        scrollable(content).height(Length::Fill).into()
    } else {
        empty_notice(i18n.text("rules-panel-empty"))
    }
}

/// One collapsed/expanded 全书规则 line. The collapsed line shows only the
/// scope-prefixed verbatim source text; expansion reveals localized claim text.
fn rule_hit_line<'a>(
    app: &StaticChartApp,
    key: &AnalysisLayerKey,
    hit: &iztro::rules::classical::ClassicalRuleHitRef,
    i18n: &I18n,
) -> Element<'a, Message> {
    let metadata = classical_rule_metadata(hit.rule_id.clone());
    let source_text = metadata
        .map(|m| m.source_text_zh_hans.to_owned())
        .unwrap_or_else(|| i18n.text("rules-panel-unknown-rule"));
    let line = format!("{}· {}", i18n.analysis_scope_label(key.scope()), source_text);

    let expansion_key = RuleHitExpansionKey {
        layer: key.clone(),
        rule_id: hit.rule_id.clone(),
    };
    let expanded = app.is_rule_hit_expanded(&expansion_key);

    let mut block = column![
        button(text(line).size(13))
            .on_press(Message::ToggleRuleHit(expansion_key))
            .style(button::text)
            .width(Length::Fill),
    ]
    .spacing(2);

    if expanded {
        let claim = hit
            .claim_key
            .as_deref()
            .map(|claim_key| i18n.claim_text_by_key(claim_key))
            .unwrap_or_else(|| i18n.text("rules-panel-no-claim"));
        block = block.push(
            text(claim)
                .size(12)
                .style(subtle_text_style)
                .width(Length::Fill),
        );
    }

    block.into()
}

/// The 格局 tab: cached pattern detections grouped by layer scope.
fn patterns_tab<'a>(app: &'a StaticChartApp, i18n: &I18n) -> Element<'a, Message> {
    let mut content = column![].spacing(10);
    let mut any = false;

    for key in app.required_analysis_layers() {
        let Some(result) = app.analysis_cache().get(&key) else {
            continue;
        };
        if result.pattern_hits.is_empty() {
            continue;
        }
        any = true;
        let scope_label = i18n.analysis_scope_label(key.scope());
        let mut group = column![text(scope_label).size(13).style(section_title_style)].spacing(4);
        for detection in &result.pattern_hits {
            group = group.push(pattern_hit_line(app, &key, detection, i18n));
        }
        content = content.push(group);
    }

    if any {
        scrollable(content).height(Length::Fill).into()
    } else {
        empty_notice(i18n.text("patterns-panel-empty"))
    }
}

/// One collapsed/expanded 格局 line, e.g. `本命· 紫府朝垣（成格）`.
fn pattern_hit_line<'a>(
    app: &StaticChartApp,
    key: &AnalysisLayerKey,
    detection: &iztro::core::PatternDetection,
    i18n: &I18n,
) -> Element<'a, Message> {
    let line = format!(
        "{}· {}（{}）",
        i18n.analysis_scope_label(key.scope()),
        detection.name_zh,
        i18n.pattern_status_label(detection.status),
    );

    let expansion_key = PatternHitExpansionKey {
        layer: key.clone(),
        pattern_id: detection.id,
    };
    let expanded = app.is_pattern_hit_expanded(&expansion_key);

    let mut block = column![
        button(text(line).size(13))
            .on_press(Message::TogglePatternHit(expansion_key))
            .style(button::text)
            .width(Length::Fill),
    ]
    .spacing(2);

    if expanded {
        block = block.push(pattern_details(detection, i18n));
    }

    block.into()
}

/// Basic structured detail rows for an expanded pattern: strength plus the
/// involved stars / palaces / mutagens. No narrative prose is added.
fn pattern_details<'a>(
    detection: &iztro::core::PatternDetection,
    i18n: &I18n,
) -> Element<'a, Message> {
    let mut rows = column![detail_row(
        &i18n.text("patterns-detail-strength"),
        &pattern_strength_label(detection.strength, i18n),
    )]
    .spacing(2);

    if !detection.involved_stars.is_empty() {
        let stars = detection
            .involved_stars
            .iter()
            .map(|star| i18n.star_name(*star))
            .collect::<Vec<_>>()
            .join("、");
        rows = rows.push(detail_row(&i18n.text("patterns-detail-stars"), &stars));
    }
    if !detection.involved_palaces.is_empty() {
        let palaces = detection
            .involved_palaces
            .iter()
            .map(|branch| i18n.branch(*branch))
            .collect::<Vec<_>>()
            .join("、");
        rows = rows.push(detail_row(&i18n.text("patterns-detail-palaces"), &palaces));
    }
    if !detection.involved_mutagens.is_empty() {
        let mutagens = detection
            .involved_mutagens
            .iter()
            .map(|mutagen| i18n.mutagen(*mutagen))
            .collect::<Vec<_>>()
            .join("、");
        rows = rows.push(detail_row(&i18n.text("patterns-detail-mutagens"), &mutagens));
    }

    rows.into()
}

/// A `label: value` detail row in the muted detail style.
fn detail_row<'a>(label: &str, value: &str) -> Element<'a, Message> {
    text(format!("{label}: {value}"))
        .size(12)
        .style(subtle_text_style)
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
fn settings_tab<'a>(app: &'a StaticChartApp, i18n: &I18n) -> Element<'a, Message> {
    let language = {
        let choice = |label: &str, locale| {
            let selected = app.locale() == locale;
            let style = if selected {
                button::primary
            } else {
                button::secondary
            };
            button(text(i18n.text(label)).size(12))
                .on_press(Message::SetLocale(locale))
                .style(style)
                .padding([4, 10])
        };
        column![
            text(i18n.text("settings-language")).size(13).style(section_title_style),
            row![
                choice("ui-english", iztro_i18n::Locale::EnUs),
                choice("ui-simplified-chinese", iztro_i18n::Locale::ZhHans),
            ]
            .spacing(6),
        ]
        .spacing(4)
    };

    let sidebar = {
        let active = app.right_panel_mode();
        let choice = |label: &str, mode: RightPanelMode| {
            let style = if active == mode {
                button::primary
            } else {
                button::secondary
            };
            button(text(i18n.text(label)).size(12))
                .on_press(Message::SetRightPanelMode(mode))
                .style(style)
                .padding([4, 10])
        };
        column![
            text(i18n.text("settings-sidebar-mode")).size(13).style(section_title_style),
            row![
                choice("settings-sidebar-hidden", RightPanelMode::Hidden),
                choice("settings-sidebar-compact", RightPanelMode::Compact),
                choice("settings-sidebar-expanded", RightPanelMode::Expanded),
            ]
            .spacing(6),
        ]
        .spacing(4)
    };

    scrollable(
        column![
            text(i18n.text("settings-panel-title")).size(15),
            language,
            sidebar,
        ]
        .spacing(14),
    )
    .height(Length::Fill)
    .into()
}

/// A centered muted empty-state notice for a tab with no visible groups.
fn empty_notice<'a>(message: String) -> Element<'a, Message> {
    container(text(message).size(13).style(subtle_text_style))
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

    #[test]
    fn hidden_mode_removes_the_inspector_from_the_layout() {
        let mut app = StaticChartApp::new();
        app.update(Message::SetRightPanelMode(RightPanelMode::Hidden));
        let i18n = I18n::new(Locale::EnUs);
        assert!(right_inspector(&app, &i18n).is_none());
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
                    right_inspector(&app, &i18n).is_some(),
                    "{mode:?}/{tab:?} should render"
                );
            }
        }
    }
}
