//! Supported display locales and their BCP-47 / `unic-langid` identities.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use unic_langid::LanguageIdentifier;

/// A supported user-interface locale.
///
/// The default is [`Locale::EnUs`]; English is the fallback for every key, so a
/// missing translation in another locale degrades to English rather than to a
/// raw key.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Default)]
pub enum Locale {
    /// English (United States). The default and fallback locale.
    #[default]
    EnUs,
    /// Simplified Chinese.
    ZhHans,
}

impl Locale {
    /// Every supported locale, in a stable order.
    pub const ALL: [Locale; 2] = [Locale::EnUs, Locale::ZhHans];

    /// The BCP-47 tag for this locale (`"en-US"` / `"zh-Hans"`).
    pub const fn as_bcp47(self) -> &'static str {
        match self {
            Locale::EnUs => "en-US",
            Locale::ZhHans => "zh-Hans",
        }
    }

    /// The `unic-langid` identity for this locale, used to build Fluent bundles.
    ///
    /// Parsed from the canonical [`as_bcp47`](Self::as_bcp47) tag, which is a
    /// well-formed language identifier, so the parse never fails.
    pub fn langid(self) -> LanguageIdentifier {
        self.as_bcp47()
            .parse()
            .expect("canonical BCP-47 tag is a valid language identifier")
    }

    /// Parses a BCP-47 tag, falling back to the default locale ([`Locale::EnUs`])
    /// for anything unrecognized. Never fails, so callers can lossily coerce.
    pub fn parse_or_default(tag: &str) -> Locale {
        tag.parse().unwrap_or_default()
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_bcp47())
    }
}

/// Serializes as the stable BCP-47 tag (`"en-US"` / `"zh-Hans"`) rather than the
/// Rust variant name, so persisted settings stay legible and decoupled from
/// internal enum naming.
impl Serialize for Locale {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_bcp47())
    }
}

/// Deserializes a BCP-47 tag, lossily coercing an unrecognized value to the
/// default locale via [`Locale::parse_or_default`] so a hand-edited or
/// forward-versioned settings file never fails to load on the locale field.
impl<'de> Deserialize<'de> for Locale {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let tag = String::deserialize(deserializer)?;
        Ok(Locale::parse_or_default(&tag))
    }
}

/// The error returned when a BCP-47 tag does not match a supported [`Locale`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnsupportedLocale;

impl fmt::Display for UnsupportedLocale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unsupported locale")
    }
}

impl std::error::Error for UnsupportedLocale {}

impl FromStr for Locale {
    type Err = UnsupportedLocale;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en-US" | "en-us" | "en" => Ok(Locale::EnUs),
            "zh-Hans" | "zh-hans" | "zh-CN" | "zh-cn" | "zh" => Ok(Locale::ZhHans),
            _ => Err(UnsupportedLocale),
        }
    }
}
