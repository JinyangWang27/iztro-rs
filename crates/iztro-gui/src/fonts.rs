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

    /// Guards against shipping a broken font: `CJK_FONT_BYTES` is embedded at
    /// compile time via `include_bytes!`, so if the asset is an un-smudged Git
    /// LFS pointer (no `git lfs` installed / no `lfs: true` checkout) the GUI
    /// silently embeds pointer text instead of a font and renders tofu boxes.
    /// This test fails fast in that situation instead of letting it ship.
    #[test]
    fn embedded_cjk_font_is_a_real_font_not_an_lfs_pointer() {
        assert!(
            !CJK_FONT_BYTES.starts_with(b"version https://git-lfs"),
            "CJK_FONT_BYTES is a Git LFS pointer, not a real font. \
             Install git-lfs and run `git lfs pull` (CI must use `lfs: true`)."
        );

        // sfnt magic for the supported font containers. Source Han Serif SC is a
        // CFF-flavoured OpenType font, so its magic is `OTTO`; the others are
        // accepted so a future TrueType/collection swap still validates.
        const SFNT_MAGICS: [&[u8; 4]; 4] = [b"OTTO", &[0x00, 0x01, 0x00, 0x00], b"true", b"ttcf"];
        let magic = CJK_FONT_BYTES.get(0..4).unwrap_or(&[]);
        assert!(
            SFNT_MAGICS.iter().any(|m| magic == m.as_slice()),
            "CJK_FONT_BYTES does not begin with a known sfnt signature (got {magic:02x?}); \
             the embedded font asset looks corrupt or missing."
        );
    }
}
