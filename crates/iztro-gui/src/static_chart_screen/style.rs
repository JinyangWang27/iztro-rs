use iced::widget::{button, container, text};
use iced::{Border, Color, Element, Theme};
use iztro::core::Mutagen;

use crate::app::Message;

use super::palace::PalaceHighlight;
use super::theme::{CHART_LAYOUT, GuiPalette, RADIUS, TYPE};

// Custom widget styles are theme-aware: every helper takes the active
// [`GuiPalette`] (resolved once per frame from the persisted `GuiThemeId`) and
// derives its colors from it, rather than reading any one theme's constant.
// Adding a future theme therefore needs only a new palette + resolver arm, not
// edits to these widget styles. Layout/spacing/radius/typography tokens are
// theme-independent scales and stay as shared constants.

/// Vertical space reserved so variable-height temporal overlays cannot cover the
/// bottom footer layer: decorative-star lines plus the anchored palace/stem labels.
pub(super) const DECORATIVE_AREA_HEIGHT: f32 = CHART_LAYOUT.decorative_area_height;
/// Fixed height of a palace's period-badge row (流年/流月/流日/流时). Reserved even
/// when a palace has no badge so the 大限/小限 line below it keeps a constant
/// y-position across every palace.
pub(super) const PERIOD_BADGE_ROW_HEIGHT: f32 = CHART_LAYOUT.period_badge_row_height;
/// Fixed height of a palace's time-flow band (period badge row + 大限/小限 line).
/// The band is bottom-anchored a constant offset above the identity footer, so
/// the band — and therefore the 大限/小限 line — aligns across all palaces
/// regardless of star count.
pub(super) const PALACE_MIDDLE_BAND_HEIGHT: f32 = CHART_LAYOUT.middle_band_height;
/// Maximum star lines stacked in a single star-area column before wrapping into
/// the next column (grid-`auto-flow: column` equivalent). Iced resolves layout
/// after `view`, so this is a deliberate, documented cap rather than a value
/// measured from the cell's pixel height; the wrapping helper takes the row cap
/// as a parameter so a future `responsive`-based caller can compute it from the
/// real star-area height.
///
/// Sized to fit conservatively inside the clipped star area at the smallest
/// supported cell: `MIN_PALACE_CELL_HEIGHT` (190) − button padding (2 × 6) −
/// reserved metadata (`PALACE_MIDDLE_BAND_HEIGHT` 44 + `DECORATIVE_AREA_HEIGHT`
/// 46) ≈ 88 px of star area. A minor star line (size-12 text + 1 px column
/// spacing, taller still with a brightness/mutagen suffix) is ≈ 16–18 px, so
/// five rows (≈ 85–90 px) would sit on the clipping edge. Four rows (≈ 68–72 px)
/// keep the `+N` overflow marker appearing strictly before any visible clipping.
pub(super) const MAX_STAR_ROWS: usize = CHART_LAYOUT.max_star_rows;
/// Maximum star-area columns before remaining stars collapse into a `+N`
/// indicator, bounding horizontal growth so wrapped stars cannot crowd out the
/// protected metadata's horizontal space.
pub(super) const MAX_STAR_COLUMNS: usize = CHART_LAYOUT.max_star_columns;

pub(super) fn palace_cell_style(
    palette: GuiPalette,
    highlight: PalaceHighlight,
    analysis_emphasis: bool,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, _status| {
        // Palaces read as clean ivory cards on the warm chart surface: a thin
        // beige border by default, a soft accent fill when selected, and an
        // accent-toned border (no heavy fill) for 三方四正-related palaces, so
        // the selected palace stays the single strongest cell.
        let (background, text_color, border_color, width): (Color, Color, Color, f32) =
            match highlight {
                PalaceHighlight::Selected => {
                    (palette.accent_soft, palette.ink, palette.accent, 2.0)
                }
                PalaceHighlight::Related => (
                    palette.palace_surface,
                    palette.ink,
                    palette.accent_border,
                    1.25,
                ),
                PalaceHighlight::None => (
                    palette.palace_surface,
                    palette.ink,
                    palette.subtle_border,
                    1.0,
                ),
            };
        // Analysis emphasis is additive: it only nudges the border tone on top
        // of the structural highlight, so the selected/related visual identity
        // is preserved. An inactive palace gains a soft jade border to mark it as
        // an analysis target without overriding background or text.
        let (border_color, width) = if analysis_emphasis {
            (palette.jade, width.max(1.5))
        } else {
            (border_color, width)
        };
        button::Style {
            background: Some(background.into()),
            text_color,
            border: Border {
                color: border_color,
                width,
                radius: RADIUS.md.into(),
            },
            ..button::Style::default()
        }
    }
}

/// A calm card surface used for the right inspector and startup cards.
pub(super) fn input_panel_style(palette: GuiPalette) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(palette.panel_surface.into()),
        text_color: Some(palette.ink),
        border: Border {
            color: palette.subtle_border,
            width: 1.0,
            radius: RADIUS.lg.into(),
        },
        ..container::Style::default()
    }
}

/// The chart-canvas surface card sitting behind the fixed palace grid.
pub(super) fn chart_surface_style(palette: GuiPalette) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(palette.chart_surface.into()),
        border: Border {
            color: palette.subtle_border,
            width: 1.0,
            radius: RADIUS.lg.into(),
        },
        ..container::Style::default()
    }
}

/// The slim application header bar above the chart.
pub(super) fn header_bar_style(palette: GuiPalette) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(palette.panel_surface.into()),
        text_color: Some(palette.ink),
        border: Border {
            color: palette.subtle_border,
            width: 1.0,
            radius: RADIUS.lg.into(),
        },
        ..container::Style::default()
    }
}

pub(super) fn center_panel_style(palette: GuiPalette) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(palette.chart_surface.into()),
        text_color: Some(palette.ink),
        border: Border {
            color: palette.accent_border,
            width: 1.5,
            radius: RADIUS.lg.into(),
        },
        ..container::Style::default()
    }
}

/// The styled surface for an enabled/disabled temporal cell or inert stepper.
pub(super) fn temporal_cell_style(palette: GuiPalette, enabled: bool) -> container::Style {
    let background = if enabled {
        palette.palace_surface
    } else {
        palette.muted_surface
    };
    container::Style {
        background: Some(background.into()),
        text_color: Some(if enabled {
            palette.ink
        } else {
            palette.disabled_text
        }),
        border: Border {
            color: palette.subtle_border,
            width: 1.0,
            radius: RADIUS.sm.into(),
        },
        ..container::Style::default()
    }
}

/// A compact temporal-period badge (`流年·丁` …) in the accent tone. The selected
/// badge is filled; the rest are outlined.
pub(super) fn period_badge_button_style(palette: GuiPalette, selected: bool) -> button::Style {
    let (background, text_color) = if selected {
        (Some(palette.accent.into()), Color::WHITE)
    } else {
        (Some(palette.accent_soft.into()), palette.accent)
    };
    button::Style {
        background,
        text_color,
        border: Border {
            color: palette.accent,
            width: if selected { 0.0 } else { 1.0 },
            radius: RADIUS.sm.into(),
        },
        ..button::Style::default()
    }
}

/// Style for a compact temporal stepper button (`◀限`, `今`, `限▶`). Disabled
/// steps are rendered as inert containers elsewhere, so this is the enabled tone.
pub(super) fn stepper_button_style(
    palette: GuiPalette,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme, _status| button::Style {
        background: Some(palette.panel_surface.into()),
        text_color: palette.ink,
        border: Border {
            color: palette.subtle_border,
            width: 1.0,
            radius: RADIUS.sm.into(),
        },
        ..button::Style::default()
    }
}

/// A compact card row used for inspector rule/pattern lines, so they read as
/// discrete cards rather than a flat debug list.
pub(super) fn inspector_row_style(palette: GuiPalette) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(palette.panel_surface.into()),
        text_color: Some(palette.ink),
        border: Border {
            color: palette.subtle_border,
            width: 1.0,
            radius: RADIUS.md.into(),
        },
        ..container::Style::default()
    }
}

/// The recessed track behind the inspector's segmented tab control.
pub(super) fn segmented_track_style(palette: GuiPalette) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(palette.muted_surface.into()),
        border: Border {
            color: palette.subtle_border,
            width: 1.0,
            radius: RADIUS.md.into(),
        },
        ..container::Style::default()
    }
}

/// A compact status pill (accent-soft fill, accent ink) for inspector rows.
pub(super) fn pill_badge(palette: GuiPalette, label: String) -> Element<'static, Message> {
    container(text(label).size(TYPE.badge).color(palette.accent))
        .style(move |_theme| container::Style {
            background: Some(palette.accent_soft.into()),
            text_color: Some(palette.accent),
            border: Border {
                color: palette.accent_border,
                width: 1.0,
                radius: RADIUS.sm.into(),
            },
            ..container::Style::default()
        })
        .padding([1, 5])
        .into()
}

pub(super) fn subtle_text_style(palette: GuiPalette) -> impl Fn(&Theme) -> text::Style {
    move |_theme| text::Style {
        color: Some(palette.text_muted),
    }
}

/// Secondary text tone, a step stronger than [`subtle_text_style`].
pub(super) fn secondary_text_style(palette: GuiPalette) -> impl Fn(&Theme) -> text::Style {
    move |_theme| text::Style {
        color: Some(palette.text_secondary),
    }
}

pub(super) fn section_title_style(palette: GuiPalette) -> impl Fn(&Theme) -> text::Style {
    move |_theme| text::Style {
        color: Some(palette.accent),
    }
}

pub(super) fn error_text_style(palette: GuiPalette) -> impl Fn(&Theme) -> text::Style {
    move |_theme| text::Style {
        color: Some(palette.cinnabar),
    }
}

/// 科权禄忌 badge background color (禄 cinnabar / 权 blue / 科 jade / 忌 ink),
/// resolved from the active palette.
pub(super) fn mutagen_badge_color(palette: GuiPalette, mutagen: Mutagen) -> Color {
    match mutagen {
        Mutagen::Lu => palette.cinnabar,
        Mutagen::Quan => palette.tian_ma,
        Mutagen::Ke => palette.jade,
        Mutagen::Ji => palette.ink,
    }
}

/// A compact 科权禄忌 badge rendered inline after a star's brightness. The
/// mutagen char is the prepared `mutagen_zh`; the GUI derives no mutagens.
pub(super) fn mutagen_inline_badge(
    palette: GuiPalette,
    mutagen: Mutagen,
    label: &str,
) -> Element<'static, Message> {
    let background = mutagen_badge_color(palette, mutagen);
    container(
        text::Text::new(label.to_owned())
            .size(TYPE.badge)
            .color(Color::WHITE),
    )
    .style(move |_theme| container::Style {
        background: Some(background.into()),
        text_color: Some(Color::WHITE),
        border: Border {
            color: background,
            width: 1.0,
            radius: RADIUS.sm.into(),
        },
        ..container::Style::default()
    })
    .padding([0, 3])
    .into()
}
