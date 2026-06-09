use crate::model::star::kind::{StarCategory, StarKind};
use crate::model::star::name::StarName;

/// Factual metadata for the fourteen major stars.
const MAJOR_STAR_METADATA: [StarMetadata; 14] = [
    StarMetadata::new("zi_wei", "紫微", StarName::ZiWei, StarKind::Major),
    StarMetadata::new("tian_ji", "天机", StarName::TianJi, StarKind::Major),
    StarMetadata::new("tai_yang", "太阳", StarName::TaiYang, StarKind::Major),
    StarMetadata::new("wu_qu", "武曲", StarName::WuQu, StarKind::Major),
    StarMetadata::new("tian_tong", "天同", StarName::TianTong, StarKind::Major),
    StarMetadata::new("lian_zhen", "廉贞", StarName::LianZhen, StarKind::Major),
    StarMetadata::new("tian_fu", "天府", StarName::TianFu, StarKind::Major),
    StarMetadata::new("tai_yin", "太阴", StarName::TaiYin, StarKind::Major),
    StarMetadata::new("tan_lang", "贪狼", StarName::TanLang, StarKind::Major),
    StarMetadata::new("ju_men", "巨门", StarName::JuMen, StarKind::Major),
    StarMetadata::new("tian_xiang", "天相", StarName::TianXiang, StarKind::Major),
    StarMetadata::new("tian_liang", "天梁", StarName::TianLiang, StarKind::Major),
    StarMetadata::new("qi_sha", "七杀", StarName::QiSha, StarKind::Major),
    StarMetadata::new("po_jun", "破军", StarName::PoJun, StarKind::Major),
];

/// Factual metadata for supported fourteen minor stars.
const MINOR_STAR_METADATA: [StarMetadata; 14] = [
    StarMetadata::new("zuo_fu", "左辅", StarName::ZuoFu, StarKind::Soft),
    StarMetadata::new("you_bi", "右弼", StarName::YouBi, StarKind::Soft),
    StarMetadata::new("wen_chang", "文昌", StarName::WenChang, StarKind::Soft),
    StarMetadata::new("wen_qu", "文曲", StarName::WenQu, StarKind::Soft),
    StarMetadata::new("tian_kui", "天魁", StarName::TianKui, StarKind::Soft),
    StarMetadata::new("tian_yue", "天钺", StarName::TianYue, StarKind::Soft),
    StarMetadata::new("lu_cun", "禄存", StarName::LuCun, StarKind::LuCun),
    StarMetadata::new("tian_ma", "天马", StarName::TianMa, StarKind::TianMa),
    StarMetadata::new("qing_yang", "擎羊", StarName::QingYang, StarKind::Tough),
    StarMetadata::new("tuo_luo", "陀罗", StarName::TuoLuo, StarKind::Tough),
    StarMetadata::new("huo_xing", "火星", StarName::HuoXing, StarKind::Tough),
    StarMetadata::new("ling_xing", "铃星", StarName::LingXing, StarKind::Tough),
    StarMetadata::new("di_kong", "地空", StarName::DiKong, StarKind::Tough),
    StarMetadata::new("di_jie", "地劫", StarName::DiJie, StarKind::Tough),
];

/// Factual metadata for the full default-algorithm adjective-star (杂曜) set.
///
/// This is the complete set of 38 natal-origin 杂曜 emitted by iztro 2.5.8
/// `getAdjectiveStar` under the default (non-Zhongzhou) algorithm. 红鸾/天喜/
/// 天姚/咸池 are peach-blossom (`Flower`) stars; 解神/年解 are `Helper` stars;
/// the remaining 32 are plain miscellaneous (`Adjective`) stars. All derive
/// [`StarCategory::Adjective`]. The Zhongzhou-only 杂曜 (龙德/截空/劫煞/大耗),
/// adjective-star brightness, and 神煞 beyond this slice stay out of scope.
const ADJECTIVE_STAR_METADATA: [StarMetadata; 38] = [
    StarMetadata::new("hong_luan", "红鸾", StarName::HongLuan, StarKind::Flower),
    StarMetadata::new("tian_xi", "天喜", StarName::TianXi, StarKind::Flower),
    StarMetadata::new("tian_yao", "天姚", StarName::TianYao, StarKind::Flower),
    StarMetadata::new("tian_xing", "天刑", StarName::TianXing, StarKind::Adjective),
    StarMetadata::new("tai_fu", "台辅", StarName::TaiFu, StarKind::Adjective),
    StarMetadata::new("feng_gao", "封诰", StarName::FengGao, StarKind::Adjective),
    StarMetadata::new("san_tai", "三台", StarName::SanTai, StarKind::Adjective),
    StarMetadata::new("ba_zuo", "八座", StarName::BaZuo, StarKind::Adjective),
    StarMetadata::new("long_chi", "龙池", StarName::LongChi, StarKind::Adjective),
    StarMetadata::new("feng_ge", "凤阁", StarName::FengGe, StarKind::Adjective),
    StarMetadata::new("tian_ku", "天哭", StarName::TianKu, StarKind::Adjective),
    StarMetadata::new("tian_xu", "天虚", StarName::TianXu, StarKind::Adjective),
    StarMetadata::new("en_guang", "恩光", StarName::EnGuang, StarKind::Adjective),
    StarMetadata::new("tian_gui", "天贵", StarName::TianGui, StarKind::Adjective),
    StarMetadata::new("tian_wu", "天巫", StarName::TianWu, StarKind::Adjective),
    StarMetadata::new(
        "tian_yue_adj",
        "天月",
        StarName::TianYueAdj,
        StarKind::Adjective,
    ),
    StarMetadata::new("yin_sha", "阴煞", StarName::YinSha, StarKind::Adjective),
    StarMetadata::new("jie_shen", "解神", StarName::JieShen, StarKind::Helper),
    StarMetadata::new("hua_gai", "华盖", StarName::HuaGai, StarKind::Adjective),
    StarMetadata::new("gu_chen", "孤辰", StarName::GuChen, StarKind::Adjective),
    StarMetadata::new("gua_su", "寡宿", StarName::GuaSu, StarKind::Adjective),
    StarMetadata::new("fei_lian", "蜚廉", StarName::FeiLian, StarKind::Adjective),
    StarMetadata::new("po_sui", "破碎", StarName::PoSui, StarKind::Adjective),
    StarMetadata::new("tian_de", "天德", StarName::TianDe, StarKind::Adjective),
    StarMetadata::new("yue_de", "月德", StarName::YueDe, StarKind::Adjective),
    StarMetadata::new("nian_jie", "年解", StarName::NianJie, StarKind::Helper),
    StarMetadata::new("xian_chi", "咸池", StarName::XianChi, StarKind::Flower),
    StarMetadata::new("tian_kong", "天空", StarName::TianKong, StarKind::Adjective),
    StarMetadata::new("tian_guan", "天官", StarName::TianGuan, StarKind::Adjective),
    StarMetadata::new("tian_chu", "天厨", StarName::TianChu, StarKind::Adjective),
    StarMetadata::new(
        "tian_fu_adj",
        "天福",
        StarName::TianFuAdj,
        StarKind::Adjective,
    ),
    StarMetadata::new("tian_cai", "天才", StarName::TianCai, StarKind::Adjective),
    StarMetadata::new("tian_shou", "天寿", StarName::TianShou, StarKind::Adjective),
    StarMetadata::new(
        "tian_shang",
        "天伤",
        StarName::TianShang,
        StarKind::Adjective,
    ),
    StarMetadata::new("tian_shi", "天使", StarName::TianShi, StarKind::Adjective),
    StarMetadata::new("jie_lu", "截路", StarName::JieLu, StarKind::Adjective),
    StarMetadata::new("kong_wang", "空亡", StarName::KongWang, StarKind::Adjective),
    StarMetadata::new("xun_kong", "旬空", StarName::XunKong, StarKind::Adjective),
];

/// Factual metadata for all currently represented stars.
const REPRESENTED_STAR_METADATA: [StarMetadata; 66] = [
    StarMetadata::new("zi_wei", "紫微", StarName::ZiWei, StarKind::Major),
    StarMetadata::new("tian_ji", "天机", StarName::TianJi, StarKind::Major),
    StarMetadata::new("tai_yang", "太阳", StarName::TaiYang, StarKind::Major),
    StarMetadata::new("wu_qu", "武曲", StarName::WuQu, StarKind::Major),
    StarMetadata::new("tian_tong", "天同", StarName::TianTong, StarKind::Major),
    StarMetadata::new("lian_zhen", "廉贞", StarName::LianZhen, StarKind::Major),
    StarMetadata::new("tian_fu", "天府", StarName::TianFu, StarKind::Major),
    StarMetadata::new("tai_yin", "太阴", StarName::TaiYin, StarKind::Major),
    StarMetadata::new("tan_lang", "贪狼", StarName::TanLang, StarKind::Major),
    StarMetadata::new("ju_men", "巨门", StarName::JuMen, StarKind::Major),
    StarMetadata::new("tian_xiang", "天相", StarName::TianXiang, StarKind::Major),
    StarMetadata::new("tian_liang", "天梁", StarName::TianLiang, StarKind::Major),
    StarMetadata::new("qi_sha", "七杀", StarName::QiSha, StarKind::Major),
    StarMetadata::new("po_jun", "破军", StarName::PoJun, StarKind::Major),
    StarMetadata::new("zuo_fu", "左辅", StarName::ZuoFu, StarKind::Soft),
    StarMetadata::new("you_bi", "右弼", StarName::YouBi, StarKind::Soft),
    StarMetadata::new("wen_chang", "文昌", StarName::WenChang, StarKind::Soft),
    StarMetadata::new("wen_qu", "文曲", StarName::WenQu, StarKind::Soft),
    StarMetadata::new("tian_kui", "天魁", StarName::TianKui, StarKind::Soft),
    StarMetadata::new("tian_yue", "天钺", StarName::TianYue, StarKind::Soft),
    StarMetadata::new("lu_cun", "禄存", StarName::LuCun, StarKind::LuCun),
    StarMetadata::new("tian_ma", "天马", StarName::TianMa, StarKind::TianMa),
    StarMetadata::new("qing_yang", "擎羊", StarName::QingYang, StarKind::Tough),
    StarMetadata::new("tuo_luo", "陀罗", StarName::TuoLuo, StarKind::Tough),
    StarMetadata::new("huo_xing", "火星", StarName::HuoXing, StarKind::Tough),
    StarMetadata::new("ling_xing", "铃星", StarName::LingXing, StarKind::Tough),
    StarMetadata::new("di_kong", "地空", StarName::DiKong, StarKind::Tough),
    StarMetadata::new("di_jie", "地劫", StarName::DiJie, StarKind::Tough),
    StarMetadata::new("hong_luan", "红鸾", StarName::HongLuan, StarKind::Flower),
    StarMetadata::new("tian_xi", "天喜", StarName::TianXi, StarKind::Flower),
    StarMetadata::new("tian_yao", "天姚", StarName::TianYao, StarKind::Flower),
    StarMetadata::new("tian_xing", "天刑", StarName::TianXing, StarKind::Adjective),
    StarMetadata::new("tai_fu", "台辅", StarName::TaiFu, StarKind::Adjective),
    StarMetadata::new("feng_gao", "封诰", StarName::FengGao, StarKind::Adjective),
    StarMetadata::new("san_tai", "三台", StarName::SanTai, StarKind::Adjective),
    StarMetadata::new("ba_zuo", "八座", StarName::BaZuo, StarKind::Adjective),
    StarMetadata::new("long_chi", "龙池", StarName::LongChi, StarKind::Adjective),
    StarMetadata::new("feng_ge", "凤阁", StarName::FengGe, StarKind::Adjective),
    StarMetadata::new("tian_ku", "天哭", StarName::TianKu, StarKind::Adjective),
    StarMetadata::new("tian_xu", "天虚", StarName::TianXu, StarKind::Adjective),
    StarMetadata::new("en_guang", "恩光", StarName::EnGuang, StarKind::Adjective),
    StarMetadata::new("tian_gui", "天贵", StarName::TianGui, StarKind::Adjective),
    StarMetadata::new("tian_wu", "天巫", StarName::TianWu, StarKind::Adjective),
    StarMetadata::new(
        "tian_yue_adj",
        "天月",
        StarName::TianYueAdj,
        StarKind::Adjective,
    ),
    StarMetadata::new("yin_sha", "阴煞", StarName::YinSha, StarKind::Adjective),
    StarMetadata::new("jie_shen", "解神", StarName::JieShen, StarKind::Helper),
    StarMetadata::new("hua_gai", "华盖", StarName::HuaGai, StarKind::Adjective),
    StarMetadata::new("gu_chen", "孤辰", StarName::GuChen, StarKind::Adjective),
    StarMetadata::new("gua_su", "寡宿", StarName::GuaSu, StarKind::Adjective),
    StarMetadata::new("fei_lian", "蜚廉", StarName::FeiLian, StarKind::Adjective),
    StarMetadata::new("po_sui", "破碎", StarName::PoSui, StarKind::Adjective),
    StarMetadata::new("tian_de", "天德", StarName::TianDe, StarKind::Adjective),
    StarMetadata::new("yue_de", "月德", StarName::YueDe, StarKind::Adjective),
    StarMetadata::new("nian_jie", "年解", StarName::NianJie, StarKind::Helper),
    StarMetadata::new("xian_chi", "咸池", StarName::XianChi, StarKind::Flower),
    StarMetadata::new("tian_kong", "天空", StarName::TianKong, StarKind::Adjective),
    StarMetadata::new("tian_guan", "天官", StarName::TianGuan, StarKind::Adjective),
    StarMetadata::new("tian_chu", "天厨", StarName::TianChu, StarKind::Adjective),
    StarMetadata::new(
        "tian_fu_adj",
        "天福",
        StarName::TianFuAdj,
        StarKind::Adjective,
    ),
    StarMetadata::new("tian_cai", "天才", StarName::TianCai, StarKind::Adjective),
    StarMetadata::new("tian_shou", "天寿", StarName::TianShou, StarKind::Adjective),
    StarMetadata::new(
        "tian_shang",
        "天伤",
        StarName::TianShang,
        StarKind::Adjective,
    ),
    StarMetadata::new("tian_shi", "天使", StarName::TianShi, StarKind::Adjective),
    StarMetadata::new("jie_lu", "截路", StarName::JieLu, StarKind::Adjective),
    StarMetadata::new("kong_wang", "空亡", StarName::KongWang, StarKind::Adjective),
    StarMetadata::new("xun_kong", "旬空", StarName::XunKong, StarKind::Adjective),
];

/// Factual metadata for a represented star.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct StarMetadata {
    key: &'static str,
    chinese_name: &'static str,
    name: StarName,
    kind: StarKind,
}

impl StarMetadata {
    /// Creates factual star metadata.
    pub const fn new(
        key: &'static str,
        chinese_name: &'static str,
        name: StarName,
        kind: StarKind,
    ) -> Self {
        Self {
            key,
            chinese_name,
            name,
            kind,
        }
    }

    /// Returns the stable internal key.
    pub const fn key(&self) -> &'static str {
        self.key
    }

    /// Returns the Chinese display name.
    pub const fn chinese_name(&self) -> &'static str {
        self.chinese_name
    }

    /// Returns the typed star identifier.
    pub const fn name(&self) -> StarName {
        self.name
    }

    /// Returns the iztro-compatible fine star type.
    pub const fn kind(&self) -> StarKind {
        self.kind
    }

    /// Returns the coarse palace grouping.
    pub const fn category(&self) -> StarCategory {
        self.kind.category()
    }
}

/// Returns factual metadata for the fourteen major stars.
pub const fn major_star_metadata_table() -> &'static [StarMetadata; 14] {
    &MAJOR_STAR_METADATA
}

/// Returns factual metadata for the supported fourteen minor stars.
pub const fn minor_star_metadata_table() -> &'static [StarMetadata; 14] {
    &MINOR_STAR_METADATA
}

/// Returns factual metadata for the supported adjective-star subset.
pub const fn adjective_star_metadata_table() -> &'static [StarMetadata] {
    &ADJECTIVE_STAR_METADATA
}

/// Returns factual metadata for all currently represented stars.
pub const fn represented_star_metadata_table() -> &'static [StarMetadata] {
    &REPRESENTED_STAR_METADATA
}

/// Returns factual metadata for one represented major star, if represented.
pub fn try_major_star_metadata(star: StarName) -> Option<&'static StarMetadata> {
    let index = match star {
        StarName::ZiWei => 0,
        StarName::TianJi => 1,
        StarName::TaiYang => 2,
        StarName::WuQu => 3,
        StarName::TianTong => 4,
        StarName::LianZhen => 5,
        StarName::TianFu => 6,
        StarName::TaiYin => 7,
        StarName::TanLang => 8,
        StarName::JuMen => 9,
        StarName::TianXiang => 10,
        StarName::TianLiang => 11,
        StarName::QiSha => 12,
        StarName::PoJun => 13,
        _ => return None,
    };

    Some(&MAJOR_STAR_METADATA[index])
}

/// Returns factual metadata for one represented minor star, if represented.
pub fn try_minor_star_metadata(star: StarName) -> Option<&'static StarMetadata> {
    let index = match star {
        StarName::ZuoFu => 0,
        StarName::YouBi => 1,
        StarName::WenChang => 2,
        StarName::WenQu => 3,
        StarName::TianKui => 4,
        StarName::TianYue => 5,
        StarName::LuCun => 6,
        StarName::TianMa => 7,
        StarName::QingYang => 8,
        StarName::TuoLuo => 9,
        StarName::HuoXing => 10,
        StarName::LingXing => 11,
        StarName::DiKong => 12,
        StarName::DiJie => 13,
        _ => return None,
    };

    Some(&MINOR_STAR_METADATA[index])
}

/// Returns factual metadata for one supported adjective star, if represented.
pub fn try_adjective_star_metadata(star: StarName) -> Option<&'static StarMetadata> {
    let index = match star {
        StarName::HongLuan => 0,
        StarName::TianXi => 1,
        StarName::TianYao => 2,
        StarName::TianXing => 3,
        StarName::TaiFu => 4,
        StarName::FengGao => 5,
        StarName::SanTai => 6,
        StarName::BaZuo => 7,
        StarName::LongChi => 8,
        StarName::FengGe => 9,
        StarName::TianKu => 10,
        StarName::TianXu => 11,
        StarName::EnGuang => 12,
        StarName::TianGui => 13,
        StarName::TianWu => 14,
        StarName::TianYueAdj => 15,
        StarName::YinSha => 16,
        StarName::JieShen => 17,
        StarName::HuaGai => 18,
        StarName::GuChen => 19,
        StarName::GuaSu => 20,
        StarName::FeiLian => 21,
        StarName::PoSui => 22,
        StarName::TianDe => 23,
        StarName::YueDe => 24,
        StarName::NianJie => 25,
        StarName::XianChi => 26,
        StarName::TianKong => 27,
        StarName::TianGuan => 28,
        StarName::TianChu => 29,
        StarName::TianFuAdj => 30,
        StarName::TianCai => 31,
        StarName::TianShou => 32,
        StarName::TianShang => 33,
        StarName::TianShi => 34,
        StarName::JieLu => 35,
        StarName::KongWang => 36,
        StarName::XunKong => 37,
        _ => return None,
    };

    Some(&ADJECTIVE_STAR_METADATA[index])
}

/// Returns factual metadata for one represented star, if represented.
pub fn try_star_metadata(star: StarName) -> Option<&'static StarMetadata> {
    try_major_star_metadata(star)
        .or_else(|| try_minor_star_metadata(star))
        .or_else(|| try_adjective_star_metadata(star))
}

/// Returns factual metadata for one represented major star.
pub fn major_star_metadata(star: StarName) -> &'static StarMetadata {
    try_major_star_metadata(star).expect("star is not a represented major star")
}

/// Returns factual metadata for one represented minor star.
pub fn minor_star_metadata(star: StarName) -> &'static StarMetadata {
    try_minor_star_metadata(star).expect("star is not a represented minor star")
}

/// Returns factual metadata for one supported adjective star.
pub fn adjective_star_metadata(star: StarName) -> &'static StarMetadata {
    try_adjective_star_metadata(star).expect("star is not a represented adjective star")
}

/// Returns factual metadata for one represented star.
pub fn star_metadata(star: StarName) -> &'static StarMetadata {
    try_star_metadata(star).expect("star is not represented")
}
