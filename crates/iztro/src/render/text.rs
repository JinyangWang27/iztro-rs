//! Plain text chart-stack rendering.

use crate::core::{
    ChartLayerKind, ChartStackSnapshot, DecorativeStarSnapshot, MutagenActivationSnapshot,
    PalaceLayerCellSnapshot, ScopedStarSnapshot, TypedStarSnapshot,
};

/// Renders a chart stack snapshot with default plain text options.
pub fn render_chart_stack_text(snapshot: &ChartStackSnapshot) -> String {
    PlainTextChartRenderer::new(PlainTextRenderOptions::default()).render(snapshot)
}

/// Options controlling deterministic plain text chart-stack rendering.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlainTextRenderOptions {
    /// Whether non-natal temporal layers are included.
    pub show_temporal_layers: bool,
    /// Whether decorative natal star lines are included.
    pub show_decorative_stars: bool,
    /// Maximum typed natal stars shown per cell before a truncation marker.
    pub max_typed_stars_per_cell: usize,
    /// Maximum decorative natal stars shown per cell before a truncation marker.
    pub max_decorative_stars_per_cell: usize,
}

impl Default for PlainTextRenderOptions {
    fn default() -> Self {
        Self {
            show_temporal_layers: true,
            show_decorative_stars: true,
            max_typed_stars_per_cell: 8,
            max_decorative_stars_per_cell: 8,
        }
    }
}

/// Deterministic ASCII renderer for chart stack snapshots.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlainTextChartRenderer {
    options: PlainTextRenderOptions,
}

impl PlainTextChartRenderer {
    /// Creates a renderer with explicit plain text options.
    pub const fn new(options: PlainTextRenderOptions) -> Self {
        Self { options }
    }

    /// Renders the provided chart stack snapshot into deterministic plain text.
    pub fn render(&self, snapshot: &ChartStackSnapshot) -> String {
        let mut output = String::new();
        self.render_header(snapshot, &mut output);

        for layer in snapshot.layers().iter().filter(|layer| {
            self.options.show_temporal_layers || layer.kind() == ChartLayerKind::Natal
        }) {
            output.push('\n');
            output.push_str(&format!("Layer {}: {:?}\n", layer.z_index(), layer.kind()));
            if let Some(context) = layer.context() {
                output.push_str(&format!("context: {context:?}\n"));
            }

            for cell in layer.cells() {
                self.render_cell(cell, &mut output);
            }
        }

        output
    }

    fn render_header(&self, snapshot: &ChartStackSnapshot, output: &mut String) {
        let birth_context = snapshot.birth_context();
        let birth_date = birth_context.date();
        let method_profile = snapshot.method_profile();

        output.push_str("Chart Stack\n");
        output.push_str(&format!(
            "birth: {:?} {}-{}-{}, time {:?}, gender {:?}\n",
            birth_date.kind(),
            birth_date.year(),
            birth_date.month(),
            birth_date.day(),
            birth_context.birth_time(),
            birth_context.gender()
        ));
        output.push_str(&format!(
            "method: {} / {:?}\n",
            method_profile.id(),
            method_profile.algorithm_kind()
        ));
        output.push_str(&format!(
            "life_palace_branch: {}\n",
            format_optional_debug(snapshot.life_palace_branch())
        ));
        output.push_str(&format!(
            "body_palace_branch: {}\n",
            format_optional_debug(snapshot.body_palace_branch())
        ));
        output.push_str(&format!(
            "five_element_bureau: {}\n",
            format_optional_debug(snapshot.five_element_bureau())
        ));
    }

    fn render_cell(&self, cell: &PalaceLayerCellSnapshot, output: &mut String) {
        output.push_str(&format!(
            "[{:?}] {} / {}\n",
            cell.branch(),
            format_optional_debug(cell.natal_palace_name()),
            format_optional_debug(cell.natal_palace_stem())
        ));

        if !cell.roles().is_empty() {
            let roles = cell
                .roles()
                .iter()
                .map(|role| format!("{:?}", role.kind()))
                .collect::<Vec<_>>()
                .join(", ");
            output.push_str(&format!("roles: {roles}\n"));
        }

        if !cell.typed_stars().is_empty() {
            output.push_str(&format!(
                "typed: {}\n",
                format_typed_stars(cell.typed_stars(), self.options.max_typed_stars_per_cell)
            ));
        } else if cell.scoped_stars().is_empty() && cell.mutagen_activations().is_empty() {
            output.push_str("typed: none\n");
        }

        if self.options.show_decorative_stars && !cell.decorative_stars().is_empty() {
            output.push_str(&format!(
                "decorative: {}\n",
                format_decorative_stars(
                    cell.decorative_stars(),
                    self.options.max_decorative_stars_per_cell
                )
            ));
        }

        if !cell.scoped_stars().is_empty() {
            let scoped_stars = cell
                .scoped_stars()
                .iter()
                .map(format_scoped_star)
                .collect::<Vec<_>>()
                .join(", ");
            output.push_str(&format!("scoped: {scoped_stars}\n"));
        }

        if !cell.mutagen_activations().is_empty() {
            let mutagens = cell
                .mutagen_activations()
                .iter()
                .map(format_mutagen_activation)
                .collect::<Vec<_>>()
                .join(", ");
            output.push_str(&format!("mutagens: {mutagens}\n"));
        }
    }
}

fn format_optional_debug<T: std::fmt::Debug>(value: Option<T>) -> String {
    value.map_or_else(|| "none".to_string(), |value| format!("{value:?}"))
}

fn format_typed_stars(stars: &[TypedStarSnapshot], limit: usize) -> String {
    format_limited(
        stars.len(),
        limit,
        stars.iter().map(|star| format!("{:?}", star.name())),
    )
}

fn format_decorative_stars(stars: &[DecorativeStarSnapshot], limit: usize) -> String {
    format_limited(
        stars.len(),
        limit,
        stars.iter().map(|star| format!("{:?}", star.name())),
    )
}

fn format_limited(names_len: usize, limit: usize, names: impl Iterator<Item = String>) -> String {
    let mut parts = names.take(limit).collect::<Vec<_>>();
    if names_len > limit {
        parts.push(format!("... (+{} more)", names_len - limit));
    }

    parts.join(", ")
}

fn format_scoped_star(star: &ScopedStarSnapshot) -> String {
    format!("{:?}", star.name())
}

fn format_mutagen_activation(activation: &MutagenActivationSnapshot) -> String {
    format!(
        "{:?} {:?} {:?}",
        activation.source_scope(),
        activation.target_star(),
        activation.mutagen()
    )
}
