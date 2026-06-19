//! Stable Fluent key derivation for typed `iztro` domain values.
//!
//! Keys are derived deterministically from each enum variant's `Debug` name so
//! the Rust mapping and the `.ftl` resources can never drift in shape: a variant
//! `StarName::ZiWei` maps to `star-zi-wei`, `PalaceName::Life` to `palace-life`,
//! `Mutagen::Lu` to `mutagen-lu`, and so on. The `.ftl` files are authored (and,
//! for stars, generated) against exactly these keys.

use iztro::core::{
    Brightness, EarthlyBranch, FiveElementBureau, Gender, HeavenlyStem, Mutagen, PalaceName, Scope,
    StarName, WesternZodiac,
};

/// Converts a `CamelCase` / `PascalCase` identifier to `kebab-case`.
///
/// A hyphen is inserted before each interior uppercase letter; digits stay
/// attached to the preceding token (`Water2` → `water2`).
fn kebab(name: &str) -> String {
    let mut out = String::with_capacity(name.len() + 4);
    for (index, ch) in name.char_indices() {
        if ch.is_ascii_uppercase() && index != 0 {
            out.push('-');
        }
        out.push(ch.to_ascii_lowercase());
    }
    out
}

/// The kebab-case key suffix for a `Debug`-printable enum variant.
fn variant_kebab<T: std::fmt::Debug>(value: &T) -> String {
    kebab(&format!("{value:?}"))
}

/// Fluent key for a natal palace name (`palace-life` …).
pub fn palace_key(name: PalaceName) -> String {
    format!("palace-{}", variant_kebab(&name))
}

/// Fluent key for a star name (`star-zi-wei` …).
pub fn star_key(name: StarName) -> String {
    format!("star-{}", variant_kebab(&name))
}

/// Fluent key for a mutagen / four-transformation (`mutagen-lu` …).
pub fn mutagen_key(mutagen: Mutagen) -> String {
    format!("mutagen-{}", variant_kebab(&mutagen))
}

/// Fluent key for a horoscope scope / temporal layer (`temporal-decadal` …).
pub fn scope_key(scope: Scope) -> String {
    format!("temporal-{}", variant_kebab(&scope))
}

/// Fluent key for a star brightness (`brightness-temple` …).
pub fn brightness_key(brightness: Brightness) -> String {
    format!("brightness-{}", variant_kebab(&brightness))
}

/// Fluent key for a gender marker (`gender-male` / `gender-female`).
pub fn gender_key(gender: Gender) -> String {
    format!("gender-{}", variant_kebab(&gender))
}

/// Fluent key for a Heavenly Stem (`stem-jia` …).
pub fn stem_key(stem: HeavenlyStem) -> String {
    format!("stem-{}", variant_kebab(&stem))
}

/// Fluent key for an Earthly Branch (`branch-zi` …).
pub fn branch_key(branch: EarthlyBranch) -> String {
    format!("branch-{}", variant_kebab(&branch))
}

/// Fluent key for a Chinese zodiac animal, keyed by Earthly Branch (`zodiac-zi` …).
pub fn zodiac_key(branch: EarthlyBranch) -> String {
    format!("zodiac-{}", variant_kebab(&branch))
}

/// Fluent key for a five-element bureau (`bureau-wood3` …).
pub fn bureau_key(bureau: FiveElementBureau) -> String {
    format!("bureau-{}", variant_kebab(&bureau))
}

/// Fluent key for a Western zodiac sign (`constellation-gemini` …).
pub fn constellation_key(sign: WesternZodiac) -> String {
    format!("constellation-{}", variant_kebab(&sign))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_match_expected_kebab_shape() {
        assert_eq!(star_key(StarName::ZiWei), "star-zi-wei");
        assert_eq!(star_key(StarName::TianYueAdj), "star-tian-yue-adj");
        assert_eq!(palace_key(PalaceName::Life), "palace-life");
        assert_eq!(mutagen_key(Mutagen::Lu), "mutagen-lu");
        assert_eq!(scope_key(Scope::Decadal), "temporal-decadal");
        assert_eq!(bureau_key(FiveElementBureau::Wood3), "bureau-wood3");
        assert_eq!(constellation_key(WesternZodiac::Gemini), "constellation-gemini");
    }
}
