//! Isolated CJK font setup for the GUI.
//!
//! The static chart renders Chinese palace and star labels, so the app must not
//! depend on platform default-font fallback (which renders unknown glyphs as
//! tofu boxes). A CJK-capable font is bundled and loaded explicitly; this module
//! holds the font bytes and the [`iced::Font`] handle. No astrology logic lives
//! here.

/// Bundled Source Han Serif SC Regular OTF asset.
pub const CJK_FONT_ASSET: &str = "SourceHanSerifSC-Regular.otf";

/// Embedded family name reported by the bundled font.
pub const CJK_FONT_FAMILY_NAME: &str = "Source Han Serif SC";

/// Raw bytes of the bundled CJK font. Loaded once at startup via
/// [`iced::application`]`.font(...)`.
pub const CJK_FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/SourceHanSerifSC-Regular.otf");

/// The bundled CJK font handle, referenced by its embedded family name.
pub const CJK_FONT: iced::Font = iced::Font::with_name(CJK_FONT_FAMILY_NAME);

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn bundled_cjk_font_uses_source_han_serif_sc_regular_otf() {
        assert_eq!(CJK_FONT_ASSET, "SourceHanSerifSC-Regular.otf");
        assert_eq!(CJK_FONT_FAMILY_NAME, "Source Han Serif SC");

        let asset_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("assets/fonts")
            .join(CJK_FONT_ASSET);
        assert!(asset_path.is_file());

        let font_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/fonts");
        let bundled_fonts = std::fs::read_dir(font_dir)
            .expect("font asset directory must read")
            .map(|entry| {
                entry
                    .expect("font directory entry must read")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .filter(|name| name.ends_with(".otf") || name.ends_with(".ttf"))
            .collect::<Vec<_>>();
        assert_eq!(bundled_fonts, vec![CJK_FONT_ASSET.to_string()]);
    }
}
