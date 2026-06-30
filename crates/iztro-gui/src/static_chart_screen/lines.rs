//! Transparent canvas overlay that draws the 三方四正 connecting lines over the
//! palace grid, matching the original iztro chart.
//!
//! All astrology relationships come from the prepared `surround` field of the
//! active palace; this module only turns fixed grid positions into line
//! endpoints. The geometry is pure GUI layout math — no branch arithmetic.

use iced::widget::canvas::{self, Frame, Geometry, Path, Stroke};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Theme, mouse};
use iztro::core::PalaceGridPosition;

use crate::app::{Message, StaticChartApp};

use super::theme::GuiPalette;

/// Normalized center (`0.0..=1.0` in both axes) of a fixed 4x4 grid cell.
fn cell_center(position: PalaceGridPosition) -> Point {
    Point::new(
        (f32::from(position.column()) + 0.5) / 4.0,
        (f32::from(position.row()) + 0.5) / 4.0,
    )
}

/// Builds the transparent 三方四正 overlay for the active palace.
///
/// Lines run from the active palace to its three prepared related palaces. The
/// natal 命宫 default uses a passive tone; a clicked palace / 流 badge uses the
/// active tone. Returns an empty (still transparent) overlay when no palace is
/// active, keeping the stack layout stable.
pub(super) fn san_fang_overlay(app: &StaticChartApp, palette: GuiPalette) -> Element<'_, Message> {
    let mut segments = Vec::new();
    if let Some(active) = app.active_palace() {
        let from = cell_center(active.grid_position);
        for branch in active.surround.branches() {
            if let Some(related) = app.palaces().iter().find(|p| p.branch == branch) {
                segments.push((from, cell_center(related.grid_position)));
            }
        }
    }

    let color = if app.san_fang_is_default() {
        palette.line_passive
    } else {
        palette.line_active
    };

    canvas::Canvas::new(SanFangLines { segments, color })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// Canvas program drawing 三方四正 line segments given as normalized endpoints.
struct SanFangLines {
    segments: Vec<(Point, Point)>,
    color: Color,
}

impl canvas::Program<Message> for SanFangLines {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let stroke = Stroke::default().with_color(self.color).with_width(1.5);
        for (start, end) in &self.segments {
            let path = Path::line(
                Point::new(start.x * bounds.width, start.y * bounds.height),
                Point::new(end.x * bounds.width, end.y * bounds.height),
            );
            frame.stroke(&path, stroke);
        }
        vec![frame.into_geometry()]
    }
}
