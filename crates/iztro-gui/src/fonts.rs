//! Isolated CJK font setup for the GUI.
//!
//! The static chart renders Chinese palace and star labels, so the app must not
//! depend on platform default-font fallback (which renders unknown glyphs as
//! tofu boxes). A CJK-capable font is bundled and loaded explicitly; this module
//! holds the font bytes and the [`iced::Font`] handle. No astrology logic lives
//! here.

/// Raw bytes of the bundled CJK fallback font ("Droid Sans Fallback",
/// Apache-2.0). Loaded once at startup via [`iced::application`]`.font(...)`.
pub const CJK_FONT_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/fonts/DroidSansFallbackFull.ttf"
));

/// The bundled CJK font handle, referenced by its embedded family name.
pub const CJK_FONT: iced::Font = iced::Font::with_name("Droid Sans Fallback");
