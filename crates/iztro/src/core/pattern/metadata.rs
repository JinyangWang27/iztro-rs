//! Static source metadata for executable source-backed patterns.
//!
//! The QuanShu source inventory TOML remains governance/test data. Runtime code
//! only carries metadata for patterns that have executable detections.

use serde::{Deserialize, Serialize};

use crate::core::pattern::model::PatternId;

/// Source catalogue group for a source-backed pattern.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternSourceGroup {
    /// 定富局.
    Wealth,
    /// 定贵局.
    Noble,
    /// 定贫贱局.
    PovertyLowStatus,
    /// 定杂局.
    Miscellaneous,
}

/// Static source metadata for one executable pattern detection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PatternSourceMetadata {
    /// Executable pattern id.
    pub pattern_id: PatternId,
    /// Canonical Chinese pattern name.
    pub name_zh: &'static str,
    /// Classical work identifier, matching source-inventory TOML.
    pub work: &'static str,
    /// Full source inventory id.
    pub source_id: &'static str,
    /// Verbatim Simplified Chinese source text, without final `。`.
    pub source_text_zh_hans: &'static str,
    /// Source section heading.
    pub section: &'static str,
    /// Source catalogue group.
    pub group: PatternSourceGroup,
}

const QUAN_SHU_WORK: &str = "zi_wei_dou_shu_quan_shu";

/// Returns static source metadata for executable source-backed patterns.
pub fn pattern_source_metadata(pattern_id: PatternId) -> Option<&'static PatternSourceMetadata> {
    SOURCE_BACKED_PATTERN_METADATA
        .iter()
        .find(|metadata| metadata.pattern_id == pattern_id)
}

static SOURCE_BACKED_PATTERN_METADATA: [PatternSourceMetadata; 9] = [
    PatternSourceMetadata {
        pattern_id: PatternId::JinCanGuangHui,
        name_zh: "金灿光辉",
        work: QUAN_SHU_WORK,
        source_id: "quan_shu.v01.ding_fu_ju.jin_can_guang_hui",
        source_text_zh_hans: "金灿光辉 太阳单守，命在午宫是也",
        section: "定富局",
        group: PatternSourceGroup::Wealth,
    },
    PatternSourceMetadata {
        pattern_id: PatternId::RiChuFuSang,
        name_zh: "日出扶桑",
        work: QUAN_SHU_WORK,
        source_id: "quan_shu.v01.ding_gui_ju.ri_chu_fu_sang",
        source_text_zh_hans: "日出扶桑 日在卯守命是也，守官禄宫亦然",
        section: "定贵局",
        group: PatternSourceGroup::Noble,
    },
    PatternSourceMetadata {
        pattern_id: PatternId::YueLuoHaiGong,
        name_zh: "月落亥宫",
        work: QUAN_SHU_WORK,
        source_id: "quan_shu.v01.ding_gui_ju.yue_luo_hai_gong",
        source_text_zh_hans: "月落亥宫 月在亥守命是也，又名月朗天门",
        section: "定贵局",
        group: PatternSourceGroup::Noble,
    },
    PatternSourceMetadata {
        pattern_id: PatternId::YueShengCangHai,
        name_zh: "月生沧海",
        work: QUAN_SHU_WORK,
        source_id: "quan_shu.v01.ding_gui_ju.yue_sheng_cang_hai",
        source_text_zh_hans: "月生沧海 月在子宫守田宅是也",
        section: "定贵局",
        group: PatternSourceGroup::Noble,
    },
    PatternSourceMetadata {
        pattern_id: PatternId::MaTouDaiJian,
        name_zh: "马头带剑",
        work: QUAN_SHU_WORK,
        source_id: "quan_shu.v01.ding_gui_ju.ma_tou_dai_jian",
        source_text_zh_hans: "马头带剑 谓马有刃是也不是居午格",
        section: "定贵局",
        group: PatternSourceGroup::Noble,
    },
    PatternSourceMetadata {
        pattern_id: PatternId::TanHuoXiangFeng,
        name_zh: "贪火相逢",
        work: QUAN_SHU_WORK,
        source_id: "quan_shu.v01.ding_gui_ju.tan_huo_xiang_feng",
        source_text_zh_hans: "贪火相逢 谓二星守命同居庙旺是也",
        section: "定贵局",
        group: PatternSourceGroup::Noble,
    },
    PatternSourceMetadata {
        pattern_id: PatternId::WuQuShouYuan,
        name_zh: "武曲守垣",
        work: QUAN_SHU_WORK,
        source_id: "quan_shu.v01.ding_gui_ju.wu_qu_shou_yuan",
        source_text_zh_hans: "武曲守垣 武守命卯宫是也，余不是",
        section: "定贵局",
        group: PatternSourceGroup::Noble,
    },
    PatternSourceMetadata {
        pattern_id: PatternId::CaiYuQiuChou,
        name_zh: "财与囚仇",
        work: QUAN_SHU_WORK,
        source_id: "quan_shu.v01.ding_pin_jian_ju.cai_yu_qiu_chou",
        source_text_zh_hans: "财与囚仇 武贞同守身命是也",
        section: "定贫贱局",
        group: PatternSourceGroup::PovertyLowStatus,
    },
    PatternSourceMetadata {
        pattern_id: PatternId::MaLuoKongWang,
        name_zh: "马落空亡",
        work: QUAN_SHU_WORK,
        source_id: "quan_shu.v01.ding_pin_jian_ju.ma_luo_kong_wang",
        source_text_zh_hans: "马落空亡 马既落亡虽禄冲会无用主奔波",
        section: "定贫贱局",
        group: PatternSourceGroup::PovertyLowStatus,
    },
];
