//! Compile-time Fluent bundle loading, per-thread caching, and fallback lookup.
//!
//! All `.ftl` resources are embedded with `include_str!`, so translation never
//! touches the filesystem at runtime. Bundles are parsed once per thread and
//! shared by `Rc`, keeping [`I18n::new`] cheap enough to call on every GUI frame.

use std::rc::Rc;

use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};

use crate::locale::Locale;

/// The embedded resource files for one locale.
macro_rules! locale_resources {
    ($dir:literal) => {
        [
            include_str!(concat!("../locales/", $dir, "/ui.ftl")),
            include_str!(concat!("../locales/", $dir, "/chart.ftl")),
            include_str!(concat!("../locales/", $dir, "/stars.ftl")),
            include_str!(concat!("../locales/", $dir, "/temporal.ftl")),
            include_str!(concat!("../locales/", $dir, "/claims.ftl")),
        ]
    };
}

const EN_US_FTL: [&str; 5] = locale_resources!("en-US");
const ZH_HANS_FTL: [&str; 5] = locale_resources!("zh-Hans");

/// Builds one Fluent bundle for a locale from its embedded resources.
///
/// Bidirectional isolation marks are disabled so interpolated values render as
/// plain text (the GUI does no RTL layout). Resource parse errors would only
/// arise from a malformed committed `.ftl`; they are surfaced by the bundle
/// tests rather than panicking in production.
fn build_bundle(locale: Locale) -> FluentBundle<FluentResource> {
    let sources = match locale {
        Locale::EnUs => EN_US_FTL,
        Locale::ZhHans => ZH_HANS_FTL,
    };
    let mut bundle = FluentBundle::new(vec![locale.langid()]);
    bundle.set_use_isolating(false);
    for source in sources {
        let resource = FluentResource::try_new(source.to_owned())
            .unwrap_or_else(|(resource, _errors)| resource);
        // Duplicate-key collisions across files would be reported here; our keys
        // are partitioned by file, so this is expected to succeed.
        let _ = bundle.add_resource(resource);
    }
    bundle
}

/// Both locale bundles, parsed once and shared across [`I18n`] instances.
struct Bundles {
    en_us: FluentBundle<FluentResource>,
    zh_hans: FluentBundle<FluentResource>,
}

impl Bundles {
    fn build() -> Self {
        Self {
            en_us: build_bundle(Locale::EnUs),
            zh_hans: build_bundle(Locale::ZhHans),
        }
    }

    fn bundle(&self, locale: Locale) -> &FluentBundle<FluentResource> {
        match locale {
            Locale::EnUs => &self.en_us,
            Locale::ZhHans => &self.zh_hans,
        }
    }
}

thread_local! {
    static BUNDLES: Rc<Bundles> = Rc::new(Bundles::build());
}

/// Looks a key up in one bundle, returning `None` when it is absent so the
/// caller can fall back.
fn lookup(
    bundle: &FluentBundle<FluentResource>,
    key: &str,
    args: Option<&FluentArgs>,
) -> Option<String> {
    let message = bundle.get_message(key)?;
    let pattern = message.value()?;
    let mut errors = Vec::new();
    let value = bundle.format_pattern(pattern, args, &mut errors);
    Some(value.into_owned())
}

/// A locale-bound localizer over the embedded Fluent resources.
///
/// Lookups resolve in the selected locale, then fall back to English, then to a
/// visible `!missing.key!` placeholder. Lookups never panic.
#[derive(Clone)]
pub struct I18n {
    locale: Locale,
    bundles: Rc<Bundles>,
}

impl I18n {
    /// Builds a localizer for `locale` (sharing this thread's cached bundles).
    pub fn new(locale: Locale) -> Self {
        Self {
            locale,
            bundles: BUNDLES.with(Rc::clone),
        }
    }

    /// The active locale.
    pub fn locale(&self) -> Locale {
        self.locale
    }

    /// Resolves `key` in the active locale, falling back to English, then to a
    /// visible `!key!` placeholder. Never panics.
    pub fn text(&self, key: &str) -> String {
        self.resolve(key, None)
    }

    /// Resolves an interpolated `key` (for example `age-label` with `$n`).
    pub fn text_args(&self, key: &str, args: &FluentArgs) -> String {
        self.resolve(key, Some(args))
    }

    fn resolve(&self, key: &str, args: Option<&FluentArgs>) -> String {
        lookup(self.bundles.bundle(self.locale), key, args)
            .or_else(|| {
                (self.locale != Locale::EnUs)
                    .then(|| lookup(self.bundles.bundle(Locale::EnUs), key, args))
                    .flatten()
            })
            .unwrap_or_else(|| format!("!{key}!"))
    }
}
