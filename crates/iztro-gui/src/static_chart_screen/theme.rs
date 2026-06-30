//! GUI design tokens and the InkPaper theme.
//!
//! This module is the single source of truth for the GUI's visual language:
//! semantic colors, spacing, radius, typography, and the fixed chart-layout
//! dimensions. Widget code reads *semantic* tokens (`palette().accent`,
//! `SPACING.md`, `CHART_LAYOUT.palace_cell_width`) instead of scattering raw
//! constants, so a future theme is added by extending [`GuiThemeId`] and
//! supplying another [`GuiTheme`] — not by rewriting every widget.
//!
//! Only `InkPaper` exists today. Its direction is a warm, paper-like reading
//! surface: ivory palace cards on a warm background, thin beige borders, deep
//! purple primary accents, and restrained semantic star tones — never a bright
//! SaaS dashboard, parchment, or dark manuscript.
//!
//! This is presentation-only. No astrology, overlay, rule, or pattern logic
//! lives here.

use iced::{Color, Theme};

use crate::settings::GuiThemeId;

/// `const`-friendly sRGB8 color (iced's `Color::from_rgb8` is not `const`).
pub(super) const fn rgb8(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: 1.0,
    }
}

/// Semantic color tokens for one theme.
///
/// Fields are intentionally semantic (surfaces, text levels, accents, star
/// tones) rather than named after raw hex values, so the same widget code reads
/// the right color under any future theme.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct GuiPalette {
    /// Outer application background.
    pub(super) app_background: Color,
    /// Surface behind the fixed chart canvas.
    pub(super) chart_surface: Color,
    /// Panel / card surface (right inspector, startup cards).
    pub(super) panel_surface: Color,
    /// Palace cell surface.
    pub(super) palace_surface: Color,
    /// Muted, recessed surface (disabled cells, segmented-control track).
    pub(super) muted_surface: Color,
    /// Thin, default border.
    pub(super) subtle_border: Color,
    /// Stronger border for emphasis.
    pub(super) strong_border: Color,
    /// Primary text / ink.
    pub(super) ink: Color,
    /// Secondary text.
    pub(super) text_secondary: Color,
    /// Muted text (captions, low-emphasis labels).
    pub(super) text_muted: Color,
    /// Primary accent / 紫微 (selected state, major stars, active controls).
    pub(super) accent: Color,
    /// Soft accent fill (selected palace background).
    pub(super) accent_soft: Color,
    /// Accent border.
    pub(super) accent_border: Color,
    /// Cinnabar secondary accent (禄 / 禄存 / important secondary markers).
    pub(super) cinnabar: Color,
    /// Cinnabar soft fill.
    pub(super) cinnabar_soft: Color,
    /// Jade / success / analysis-emphasis accent.
    pub(super) jade: Color,
    /// Jade soft fill.
    pub(super) jade_soft: Color,
    /// Malefic / wine star tone (六煞).
    pub(super) malefic: Color,
    /// 桃花 / peach relationship-star tone.
    pub(super) peach: Color,
    /// 天马 blue tone.
    pub(super) tian_ma: Color,
    /// Decorative-god olive tone (长生/博士 十二神).
    pub(super) decorative_olive: Color,
    /// Brightness suffix (庙旺得利平陷不) tone.
    pub(super) brightness_suffix: Color,
    /// Disabled text/control tone.
    pub(super) disabled_text: Color,
    /// Passive 三方四正 connecting-line tone (natal 命宫 default).
    pub(super) line_passive: Color,
    /// Active 三方四正 connecting-line tone (after a click / 流 badge).
    pub(super) line_active: Color,
}

/// Spacing scale, in logical pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct GuiSpacing {
    pub(super) xs: f32,
    pub(super) sm: f32,
    pub(super) md: f32,
    pub(super) lg: f32,
    pub(super) xl: f32,
    pub(super) xxl: f32,
}

/// Corner-radius scale, in logical pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct GuiRadius {
    pub(super) sm: f32,
    pub(super) md: f32,
    pub(super) lg: f32,
}

/// Typography sizes, in logical pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct GuiTypography {
    /// Page title (startup heading).
    pub(super) title: u16,
    /// Section heading.
    pub(super) heading: u16,
    /// Primary body / fact rows.
    pub(super) body: u16,
    /// Secondary label.
    pub(super) label: u16,
    /// Small caption / compact controls.
    pub(super) small: u16,
    /// Major star name.
    pub(super) star_major: u16,
    /// Minor / adjective star name.
    pub(super) star_minor: u16,
    /// Decorative-god names.
    pub(super) caption: u16,
    /// Compact mutagen badge glyph.
    pub(super) badge: u16,
    /// Smallest annotation (小限 age list).
    pub(super) micro: u16,
}

/// Fixed chart-layout dimensions, in logical pixels.
///
/// The chart is a fixed 4×4 canvas that scrolls when the window is smaller;
/// these are the named dimensions behind that layout. Scaling is intentionally
/// not implemented yet — moving the magic numbers here makes a future
/// `UiScale` straightforward without another styling rewrite.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct ChartLayoutTokens {
    /// Width of one palace cell.
    pub(super) palace_cell_width: f32,
    /// Height of one palace cell.
    pub(super) palace_cell_height: f32,
    /// Palace columns across the full chart.
    pub(super) grid_columns: f32,
    /// Palace rows down the full chart.
    pub(super) grid_rows: f32,
    /// Gutter reserved for the scrollable's floating scrollbars.
    pub(super) scrollbar_gutter: f32,
    /// Reserved height of the identity footer (gods + 宫名/干支).
    pub(super) decorative_area_height: f32,
    /// Reserved height of the period-badge row.
    pub(super) period_badge_row_height: f32,
    /// Reserved height of the time-flow band (badge row + 大限/小限 line).
    pub(super) middle_band_height: f32,
    /// Maximum star rows stacked in one wrapped column before wrapping.
    pub(super) max_star_rows: usize,
    /// Maximum star columns before remaining stars collapse into `+N`.
    pub(super) max_star_columns: usize,
    /// Right inspector width in compact mode.
    pub(super) inspector_compact_width: f32,
    /// Right inspector width in expanded mode.
    pub(super) inspector_expanded_width: f32,
}

/// A complete theme: a palette plus the shared, theme-independent token scales.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct GuiTheme {
    pub(super) palette: GuiPalette,
}

impl GuiTheme {
    /// Resolves the token set for a persisted [`GuiThemeId`].
    pub(super) const fn resolve(id: GuiThemeId) -> &'static GuiTheme {
        match id {
            GuiThemeId::InkPaper => &INK_PAPER,
        }
    }
}

/// The InkPaper palette — the project's only implemented theme.
pub(super) const INK_PAPER: GuiTheme = GuiTheme {
    palette: GuiPalette {
        app_background: rgb8(0xF7, 0xF1, 0xE6),
        chart_surface: rgb8(0xFB, 0xF7, 0xEF),
        panel_surface: rgb8(0xFF, 0xFD, 0xF8),
        palace_surface: rgb8(0xFF, 0xFD, 0xF8),
        muted_surface: rgb8(0xF1, 0xE8, 0xDA),
        subtle_border: rgb8(0xD8, 0xCD, 0xBB),
        strong_border: rgb8(0xB8, 0xA9, 0x92),
        ink: rgb8(0x26, 0x23, 0x1F),
        text_secondary: rgb8(0x6F, 0x66, 0x5B),
        text_muted: rgb8(0x9A, 0x90, 0x84),
        accent: rgb8(0x6F, 0x3A, 0x7C),
        accent_soft: rgb8(0xEF, 0xE3, 0xF1),
        accent_border: rgb8(0x9B, 0x73, 0xA6),
        cinnabar: rgb8(0xB6, 0x42, 0x2C),
        cinnabar_soft: rgb8(0xF5, 0xDE, 0xD8),
        jade: rgb8(0x5F, 0x7F, 0x64),
        jade_soft: rgb8(0xE5, 0xEE, 0xE3),
        malefic: rgb8(0x8A, 0x3F, 0x55),
        peach: rgb8(0xB4, 0x5A, 0x83),
        tian_ma: rgb8(0x3F, 0x6F, 0x99),
        decorative_olive: rgb8(0x7C, 0x7B, 0x4A),
        brightness_suffix: rgb8(0xB4, 0xAA, 0xA0),
        disabled_text: rgb8(0xBD, 0xB3, 0xA7),
        line_passive: rgb8(0xC9, 0xBD, 0xAA),
        line_active: rgb8(0x7B, 0x4A, 0x86),
    },
};

/// The shared spacing scale.
pub(super) const SPACING: GuiSpacing = GuiSpacing {
    xs: 2.0,
    sm: 4.0,
    md: 6.0,
    lg: 8.0,
    xl: 12.0,
    xxl: 16.0,
};

/// The shared radius scale.
pub(super) const RADIUS: GuiRadius = GuiRadius {
    sm: 3.0,
    md: 5.0,
    lg: 8.0,
};

/// The shared typography scale.
pub(super) const TYPE: GuiTypography = GuiTypography {
    title: 24,
    heading: 15,
    body: 13,
    label: 12,
    small: 11,
    star_major: 15,
    star_minor: 12,
    caption: 10,
    badge: 9,
    micro: 8,
};

/// The fixed chart-layout dimensions.
pub(super) const CHART_LAYOUT: ChartLayoutTokens = ChartLayoutTokens {
    palace_cell_width: 275.0,
    palace_cell_height: 190.0,
    grid_columns: 4.0,
    grid_rows: 4.0,
    scrollbar_gutter: 16.0,
    decorative_area_height: 46.0,
    period_badge_row_height: 18.0,
    middle_band_height: 44.0,
    max_star_rows: 4,
    max_star_columns: 2,
    inspector_compact_width: 280.0,
    inspector_expanded_width: 360.0,
};

/// The active palette for the current theme id.
pub(super) const fn palette(id: GuiThemeId) -> &'static GuiPalette {
    &GuiTheme::resolve(id).palette
}

/// Builds the iced [`Theme`] backing a [`GuiThemeId`], so iced's built-in widget
/// styles (`button::primary`, `button::secondary`, text inputs, pick lists,
/// scrollbars) inherit the InkPaper palette. Chart-specific surfaces are styled
/// explicitly from the same tokens elsewhere; this keeps the framework defaults
/// on-theme without rewriting every widget.
pub fn iced_theme(id: GuiThemeId) -> Theme {
    let p = palette(id);
    Theme::custom(
        "InkPaper".to_owned(),
        iced::theme::Palette {
            background: p.app_background,
            text: p.ink,
            primary: p.accent,
            success: p.jade,
            danger: p.cinnabar,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ink_paper_uses_the_documented_warm_surfaces() {
        let p = palette(GuiThemeId::InkPaper);
        assert_eq!(p.app_background, rgb8(0xF7, 0xF1, 0xE6));
        assert_eq!(p.palace_surface, rgb8(0xFF, 0xFD, 0xF8));
        assert_eq!(p.accent, rgb8(0x6F, 0x3A, 0x7C));
    }

    #[test]
    fn every_theme_id_resolves_to_a_palette() {
        // Exhaustive over the (currently single) theme id: resolution is total.
        for id in [GuiThemeId::InkPaper] {
            let _ = iced_theme(id);
        }
    }
}
