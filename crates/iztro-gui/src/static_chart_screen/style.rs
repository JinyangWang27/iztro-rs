use iced::widget::{button, container, text};
use iced::{Border, Color, Element, Theme};
use iztro::core::Mutagen;

use crate::app::Message;

use super::palace::PalaceHighlight;

/// `const`-friendly sRGB8 color (iced's `Color::from_rgb8` is not `const`).
pub(super) const fn rgb8(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: 1.0,
    }
}

/// Major stars (主星) and the auspicious soft minor pair stars.
pub(super) const MAJOR_PURPLE: Color = rgb8(0x53, 0x1d, 0xab);
/// Brightness suffix (庙旺得利平陷不), independent of star category.
pub(super) const BRIGHTNESS_GRAY: Color = rgb8(0xc5, 0xcb, 0xd0);
/// Six malefics / 六煞 (擎羊陀罗火星铃星地空地劫).
pub(super) const MINOR_MALEFIC: Color = rgb8(0x81, 0x33, 0x59);
/// 禄存.
pub(super) const LU_CUN_ORANGE: Color = rgb8(0xd4, 0x38, 0x0d);
/// 天马.
pub(super) const TIAN_MA_BLUE: Color = rgb8(0x18, 0x90, 0xff);
/// Ordinary adjective / miscellaneous stars (杂曜).
pub(super) const ADJ_GRAY: Color = rgb8(0x8c, 0x8c, 0x8c);
/// 桃花 / festive relationship stars (红鸾咸池天姚天喜, and flow variants).
pub(super) const PEACH_MAGENTA: Color = rgb8(0xc3, 0x1d, 0x7f);
/// 长生十二神 / 博士十二神 decorative gods (bottom-left).
pub(super) const DECOR_GOD_OLIVE: Color = rgb8(0x90, 0x98, 0x3c);
/// Vertical space reserved so variable-height temporal overlays cannot cover the
/// bottom footer layer: decorative-star lines plus the anchored palace/stem labels.
pub(super) const DECORATIVE_AREA_HEIGHT: f32 = 46.0;
/// Fixed height of a palace's period-badge row (流年/流月/流日/流时). Reserved even
/// when a palace has no badge so the 大限/小限 line below it keeps a constant
/// y-position across every palace.
pub(super) const PERIOD_BADGE_ROW_HEIGHT: f32 = 18.0;
/// Fixed height of a palace's middle band (period badge row + 大限/小限 line).
/// The band is centered vertically in the cell so the band — and therefore the
/// 大限/小限 line — aligns across all palaces regardless of star count.
pub(super) const PALACE_MIDDLE_BAND_HEIGHT: f32 = 44.0;
/// Passive 三方四正 connecting-line tone, used for the natal 命宫 default lines.
pub(super) const SAN_FANG_PASSIVE: Color = rgb8(0xb0, 0xb8, 0xc4);
/// Active 三方四正 connecting-line tone, used after a 流 badge / palace click.
pub(super) const SAN_FANG_ACTIVE: Color = MAJOR_PURPLE;
/// 大限 / 小限 limit text shown in the palace center.
pub(super) const LIMIT_GRAY: Color = rgb8(0x9a, 0x9a, 0x9a);
/// Highlight tone for the active 大限 palace's limit text.
pub(super) const LIMIT_ACTIVE: Color = LU_CUN_ORANGE;

/// 化禄 badge background.
const MUTAGEN_LU: Color = rgb8(0xd4, 0x38, 0x0d);
/// 化权 badge background.
const MUTAGEN_QUAN: Color = rgb8(0x2f, 0x54, 0xeb);
/// 化科 badge background.
const MUTAGEN_KE: Color = rgb8(0x23, 0x78, 0x04);
/// 化忌 badge background.
const MUTAGEN_JI: Color = rgb8(0x00, 0x00, 0x00);

pub(super) fn palace_cell_style(
    highlight: PalaceHighlight,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |theme, _status| {
        let palette = theme.extended_palette();
        let (background, text_color, border_color, width) = match highlight {
            PalaceHighlight::Selected => (
                palette.primary.weak.color,
                palette.primary.weak.text,
                palette.primary.strong.color,
                2.0,
            ),
            // 三方四正 related palaces get a subtle filled background, weaker
            // than the active palace above (a soft fill rather than only a
            // border), matching the iztro highlight feel.
            PalaceHighlight::Related => (
                palette.background.weak.color,
                palette.background.weak.text,
                palette.primary.base.color,
                1.5,
            ),
            PalaceHighlight::None => (
                palette.background.base.color,
                palette.background.base.text,
                palette.background.strong.color,
                1.0,
            ),
        };
        button::Style {
            background: Some(background.into()),
            text_color,
            border: Border {
                color: border_color,
                width,
                radius: 4.0.into(),
            },
            ..button::Style::default()
        }
    }
}

pub(super) fn input_panel_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(palette.background.weak.color.into()),
        border: Border {
            color: palette.background.strong.color,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..container::Style::default()
    }
}

pub(super) fn center_panel_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(palette.background.weak.color.into()),
        text_color: Some(palette.background.weak.text),
        border: Border {
            color: palette.primary.strong.color,
            width: 2.0,
            radius: 6.0.into(),
        },
        ..container::Style::default()
    }
}

pub(super) fn temporal_cell_style(theme: &Theme, enabled: bool) -> container::Style {
    let palette = theme.extended_palette();
    let background = if enabled {
        palette.background.base.color
    } else {
        palette.background.weak.color
    };
    container::Style {
        background: Some(background.into()),
        border: Border {
            color: palette.background.strong.color,
            width: 1.0,
            radius: 3.0.into(),
        },
        ..container::Style::default()
    }
}

/// A compact temporal-period badge (`流年·丁` …) in `MAJOR_PURPLE`. The selected
/// badge is filled; the rest are outlined.
pub(super) fn period_badge_button_style(selected: bool) -> button::Style {
    let (background, text_color) = if selected {
        (Some(MAJOR_PURPLE.into()), Color::WHITE)
    } else {
        (None, MAJOR_PURPLE)
    };
    button::Style {
        background,
        text_color,
        border: Border {
            color: MAJOR_PURPLE,
            width: if selected { 0.0 } else { 1.0 },
            radius: 3.0.into(),
        },
        ..button::Style::default()
    }
}

/// Style for a compact temporal stepper button (`◀限`, `今`, `限▶`). Disabled
/// steps are rendered as inert containers elsewhere, so this is the enabled tone.
pub(super) fn stepper_button_style(theme: &Theme, _status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    button::Style {
        background: Some(palette.background.base.color.into()),
        text_color: palette.background.base.text,
        border: Border {
            color: palette.background.strong.color,
            width: 1.0,
            radius: 3.0.into(),
        },
        ..button::Style::default()
    }
}

pub(super) fn subtle_text_style(theme: &Theme) -> text::Style {
    text::Style {
        color: Some(theme.extended_palette().background.strong.color),
    }
}

pub(super) fn section_title_style(theme: &Theme) -> text::Style {
    text::Style {
        color: Some(theme.extended_palette().primary.strong.color),
    }
}

pub(super) fn error_text_style(theme: &Theme) -> text::Style {
    text::Style {
        color: Some(theme.extended_palette().danger.base.color),
    }
}

/// 科权禄忌 badge background color (禄 #d4380d / 权 #2f54eb / 科 #237804 / 忌 #000000).
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
            .size(9)
            .color(Color::WHITE),
    )
    .style(move |_theme| container::Style {
        background: Some(background.into()),
        text_color: Some(Color::WHITE),
        border: Border {
            color: background,
            width: 1.0,
            radius: 3.0.into(),
        },
        ..container::Style::default()
    })
    .padding([0, 3])
    .into()
}
