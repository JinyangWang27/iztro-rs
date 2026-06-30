//! Display/runtime metadata for canonical pattern detections.
//!
//! This table is separate from [`PatternSourceMetadata`](super::metadata::PatternSourceMetadata).
//! Source metadata is verified provenance; display metadata is presentation
//! context for names, aliases, condition notes, source notes, and interpretation
//! notes.

use crate::core::pattern::model::PatternId;

/// Runtime/display metadata for one canonical pattern id.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PatternDisplayMetadata {
    /// Canonical pattern id this display metadata describes.
    pub pattern_id: PatternId,
    /// Runtime display name.
    pub name_zh: &'static str,
    /// Runtime display aliases.
    pub aliases_zh: &'static [&'static str],
    /// Normalized condition note for display/help surfaces.
    pub condition_note_zh_hans: &'static str,
    /// Optional source note for display surfaces.
    pub source_note_zh_hans: Option<&'static str>,
    /// Optional interpretation note for display surfaces.
    pub interpretation_note_zh_hans: Option<&'static str>,
}

const EMPTY_ALIASES: &[&str] = &[];
const RI_CHU_FU_SANG_ALIASES: &[&str] = &["日出扶桑格"];

/// Returns static display metadata for every canonical pattern id.
pub fn pattern_display_metadata(pattern_id: PatternId) -> &'static PatternDisplayMetadata {
    DISPLAY_PATTERN_METADATA
        .iter()
        .find(|metadata| metadata.pattern_id == pattern_id)
        .expect("display metadata must cover every PatternId")
}

static DISPLAY_PATTERN_METADATA: [PatternDisplayMetadata; 26] = [
    PatternDisplayMetadata {
        pattern_id: PatternId::ZiFuChaoYuan,
        name_zh: "紫府朝垣",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "紫微与天府同在命宫三方四正。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::JiYueTongLiang,
        name_zh: "机月同梁",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "天机、太阴、天同、天梁齐会命宫三方四正。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::YangTuoJiaJi,
        name_zh: "羊陀夹忌",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "擎羊与陀罗夹住承载化忌的宫位。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::LingChangTuoWu,
        name_zh: "铃昌陀武",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "铃星、文昌、陀罗、武曲相关结构；当前保留 id，未注册检测器。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::ZuoYouJiaMing,
        name_zh: "左右夹命",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "左辅与右弼夹住命宫。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::ChangQuJiaMing,
        name_zh: "昌曲夹命",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "文昌与文曲夹住命宫。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::RiYueBingMing,
        name_zh: "日月并明",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "太阳与太阴皆在盘，且二者均为明亮状态。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::RiYueFanBei,
        name_zh: "日月反背",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "太阳与太阴皆在盘，且二者均为失辉落陷状态。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::JinCanGuangHui,
        name_zh: "金灿光辉",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "命宫在午，太阳在命宫，且太阳是该宫唯一主星。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::RiChuFuSang,
        name_zh: "日照雷门",
        aliases_zh: RI_CHU_FU_SANG_ALIASES,
        condition_note_zh_hans: "出生时辰为卯至未，命宫在卯，太阳与天梁同在卯宫命宫，且命宫三方四正有已建模支持。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: Some("公开 id 保留 RiChuFuSang；运行时显示采用日照雷门。"),
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::YueLuoHaiGong,
        name_zh: "月落亥宫",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "太阴在亥，且亥宫为命宫。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::YueShengCangHai,
        name_zh: "月生沧海",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "太阴在子，且子宫为田宅宫。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::MaTouDaiJian,
        name_zh: "马头带剑",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "天马与擎羊同宫。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::TanHuoXiangFeng,
        name_zh: "贪火相逢",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "贪狼与火星同守命宫，且二者皆为明亮状态。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::WuQuShouYuan,
        name_zh: "武曲守垣",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "武曲在命宫，且命宫地支为卯。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::CaiYuQiuChou,
        name_zh: "财与囚仇",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "武曲与廉贞同在命宫或身宫。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::MaLuoKongWang,
        name_zh: "马落空亡",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "天马与已建模空亡族星同宫。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::MingLiFengKong,
        name_zh: "命里逢空",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "命宫有已建模空亡族星。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::LuFengChongPo,
        name_zh: "禄逢冲破",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "命宫三方四正有禄存或化禄支持，且该支持被同宫或对宫的煞星或空亡族星冲破。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::WenXingGongMing,
        name_zh: "文星拱命",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "文昌与文曲皆在命宫三方四正。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::TianJiSiHai,
        name_zh: "天机巳亥",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "天机在巳或亥，且该宫为命宫或在命宫三方四正。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::ZuoYouTongGong,
        name_zh: "左右同宫",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "仅本命：左辅与右弼同在身宫。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::MingZhuChuHai,
        name_zh: "明珠出海",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "太阳与太阴皆在命宫三方四正，且二者均为明亮状态。",
        source_note_zh_hans: Some("三合明珠生旺地稳步蟾宫（斗数骨髓赋）"),
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::MingWuZhengYao,
        name_zh: "命无正曜",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "命宫无主星。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::JiXiangLiMing,
        name_zh: "极向离明",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "命宫在午且紫微在命宫；命宫三方四正有煞星时破格。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    },
    PatternDisplayMetadata {
        pattern_id: PatternId::FuXiangChaoYuan,
        name_zh: "府相朝垣",
        aliases_zh: EMPTY_ALIASES,
        condition_note_zh_hans: "天府与天相朝拱命宫；支持三方四正、天府在命、财帛官禄分居等结构。",
        source_note_zh_hans: Some("府相朝垣命必荣（女命骨髓赋）"),
        interpretation_note_zh_hans: None,
    },
];
