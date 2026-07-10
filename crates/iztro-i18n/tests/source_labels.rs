//! Localized labels for classical source citations (典籍 / 卷 / 节).

use iztro::rules::source::ClassicalWork;
use iztro_i18n::{I18n, Locale};

#[test]
fn classical_work_label_localizes_both_works() {
    let en = I18n::new(Locale::EnUs);
    let zh = I18n::new(Locale::ZhHans);

    assert_eq!(
        en.classical_work_label(ClassicalWork::ZiWeiDouShuQuanShu),
        "Zi Wei Dou Shu Quan Shu"
    );
    assert_eq!(
        zh.classical_work_label(ClassicalWork::ZiWeiDouShuQuanShu),
        "《紫微斗数全书》"
    );

    assert_eq!(
        en.classical_work_label(ClassicalWork::IztroPatternCatalog),
        "iztro Pattern Catalog"
    );
    assert_eq!(
        zh.classical_work_label(ClassicalWork::IztroPatternCatalog),
        "iztro 格局目录"
    );
}

#[test]
fn source_location_label_formats_volume_and_section() {
    let en = I18n::new(Locale::EnUs);
    let zh = I18n::new(Locale::ZhHans);

    // Section headings stay in Chinese in every locale: they are canonical
    // Zi Wei Dou Shu terminology, not translatable UI copy.
    assert_eq!(en.source_location_label(1, "太微赋"), "Volume 1 · 太微赋");
    assert_eq!(zh.source_location_label(1, "太微赋"), "卷一 · 太微赋");
    assert_eq!(zh.source_location_label(3, "定杂局"), "卷三 · 定杂局");
    // Volumes beyond the numeral map fall back to the digit form.
    assert_eq!(zh.source_location_label(9, "某节"), "卷9 · 某节");
}
