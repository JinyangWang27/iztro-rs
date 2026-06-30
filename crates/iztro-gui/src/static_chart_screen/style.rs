use iced::widget::{button, container, text};
use iced::{Border, Color, Element, Theme};
use iztro::core::Mutagen;

use crate::app::Message;

use super::palace::PalaceHighlight;
use super::theme::{CHART_LAYOUT, INK_PAPER, RADIUS, TYPE};

use super::theme::GuiPalette;

/// The active InkPaper palette. All widget colors derive from these semantic
/// tokens rather than scattered hex constants, so a future theme swaps one
/// palette instead of editing every widget.
const P: GuiPalette = INK_PAPER.palette;

/// Major stars (主星) and the auspicious soft minor pair stars.
pub(super) const MAJOR_PURPLE: Color = P.accent;
/// Brightness suffix (庙旺得利平陷不), independent of star category.
pub(super) const BRIGHTNESS_GRAY: Color = P.brightness_suffix;
/// Six malefics / 六煞 (擎羊陀罗火星铃星地空地劫).
pub(super) const MINOR_MALEFIC: Color = P.malefic;
/// 禄存.
pub(super) const LU_CUN_ORANGE: Color = P.cinnabar;
/// 天马.
pub(super) const TIAN_MA_BLUE: Color = P.tian_ma;
/// Ordinary adjective / miscellaneous stars (杂曜).
pub(super) const ADJ_GRAY: Color = P.text_muted;
/// 桃花 / festive relationship stars (红鸾咸池天姚天喜, and flow variants).
pub(super) const PEACH_MAGENTA: Color = P.peach;
/// 长生十二神 / 博士十二神 decorative gods (bottom-left).
pub(super) const DECOR_GOD_OLIVE: Color = P.decorative_olive;
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
/// Passive 三方四正 connecting-line tone, used for the natal 命宫 default lines.
pub(super) const SAN_FANG_PASSIVE: Color = P.line_passive;
/// Active 三方四正 connecting-line tone, used after a 流 badge / palace click.
pub(super) const SAN_FANG_ACTIVE: Color = P.line_active;
/// 大限 / 小限 limit text shown in the palace center.
pub(super) const LIMIT_GRAY: Color = P.text_muted;
/// Highlight tone for the active 大限 palace's limit text.
pub(super) const LIMIT_ACTIVE: Color = P.cinnabar;

/// 化禄 badge background.
const MUTAGEN_LU: Color = P.cinnabar;
/// 化权 badge background.
const MUTAGEN_QUAN: Color = P.tian_ma;
/// 化科 badge background.
const MUTAGEN_KE: Color = P.jade;
/// 化忌 badge background.
const MUTAGEN_JI: Color = P.ink;

pub(super) fn palace_cell_style(
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
                PalaceHighlight::Selected => (P.accent_soft, P.ink, P.accent, 2.0),
                PalaceHighlight::Related => (P.palace_surface, P.ink, P.accent_border, 1.25),
                PalaceHighlight::None => (P.palace_surface, P.ink, P.subtle_border, 1.0),
            };
        // Analysis emphasis is additive: it only nudges the border tone on top
        // of the structural highlight, so the selected/related visual identity
        // is preserved. An inactive palace gains a soft jade border to mark it as
        // an analysis target without overriding background or text.
        let (border_color, width) = if analysis_emphasis {
            (P.jade, width.max(1.5))
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
pub(super) fn input_panel_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(P.panel_surface.into()),
        text_color: Some(P.ink),
        border: Border {
            color: P.subtle_border,
            width: 1.0,
            radius: RADIUS.lg.into(),
        },
        ..container::Style::default()
    }
}

/// The chart-canvas surface card sitting behind the fixed palace grid.
pub(super) fn chart_surface_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(P.chart_surface.into()),
        border: Border {
            color: P.subtle_border,
            width: 1.0,
            radius: RADIUS.lg.into(),
        },
        ..container::Style::default()
    }
}

/// The slim application header bar above the chart.
pub(super) fn header_bar_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(P.panel_surface.into()),
        text_color: Some(P.ink),
        border: Border {
            color: P.subtle_border,
            width: 1.0,
            radius: RADIUS.lg.into(),
        },
        ..container::Style::default()
    }
}

pub(super) fn center_panel_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(P.chart_surface.into()),
        text_color: Some(P.ink),
        border: Border {
            color: P.accent_border,
            width: 1.5,
            radius: RADIUS.lg.into(),
        },
        ..container::Style::default()
    }
}

pub(super) fn temporal_cell_style(_theme: &Theme, enabled: bool) -> container::Style {
    let background = if enabled {
        P.palace_surface
    } else {
        P.muted_surface
    };
    container::Style {
        background: Some(background.into()),
        text_color: Some(if enabled { P.ink } else { P.disabled_text }),
        border: Border {
            color: P.subtle_border,
            width: 1.0,
            radius: RADIUS.sm.into(),
        },
        ..container::Style::default()
    }
}

/// A compact temporal-period badge (`流年·丁` …) in the accent tone. The selected
/// badge is filled; the rest are outlined.
pub(super) fn period_badge_button_style(selected: bool) -> button::Style {
    let (background, text_color) = if selected {
        (Some(P.accent.into()), Color::WHITE)
    } else {
        (Some(P.accent_soft.into()), P.accent)
    };
    button::Style {
        background,
        text_color,
        border: Border {
            color: P.accent,
            width: if selected { 0.0 } else { 1.0 },
            radius: RADIUS.sm.into(),
        },
        ..button::Style::default()
    }
}

/// Style for a compact temporal stepper button (`◀限`, `今`, `限▶`). Disabled
/// steps are rendered as inert containers elsewhere, so this is the enabled tone.
pub(super) fn stepper_button_style(_theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: Some(P.panel_surface.into()),
        text_color: P.ink,
        border: Border {
            color: P.subtle_border,
            width: 1.0,
            radius: RADIUS.sm.into(),
        },
        ..button::Style::default()
    }
}

pub(super) fn subtle_text_style(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(P.text_muted),
    }
}

/// Secondary text tone, a step stronger than [`subtle_text_style`].
pub(super) fn secondary_text_style(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(P.text_secondary),
    }
}

pub(super) fn section_title_style(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(P.accent),
    }
}

pub(super) fn error_text_style(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(P.cinnabar),
    }
}

/// 科权禄忌 badge background color (禄 cinnabar / 权 blue / 科 jade / 忌 ink).
pub(super) fn mutagen_badge_color(mutagen: Mutagen) -> Color {
    match mutagen {
        Mutagen::Lu => MUTAGEN_LU,
        Mutagen::Quan => MUTAGEN_QUAN,
        Mutagen::Ke => MUTAGEN_KE,
        Mutagen::Ji => MUTAGEN_JI,
    }
}

/// A compact 科权禄忌 badge rendered inline after a star's brightness. The
/// mutagen char is the prepared `mutagen_zh`; the GUI derives no mutagens.
pub(super) fn mutagen_inline_badge(mutagen: Mutagen, label: &str) -> Element<'static, Message> {
    let background = mutagen_badge_color(mutagen);
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
