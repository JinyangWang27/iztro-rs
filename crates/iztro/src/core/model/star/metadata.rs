use crate::core::model::star::kind::{StarCategory, StarKind};
use crate::core::model::star::name::StarName;

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

/// Factual metadata for the supported natal adjective-star (杂曜) set.
///
/// This includes the complete set of 38 natal-origin 杂曜 emitted by iztro 2.5.8
/// `getAdjectiveStar` under the default (non-Zhongzhou) algorithm, plus the
/// four Zhongzhou-only natal adjective stars (龙德/截空/劫煞/大耗). 红鸾/天喜/
/// 天姚/咸池 are peach-blossom (`Flower`) stars; 解神/年解 are `Helper` stars;
/// the remaining represented natal 杂曜 are plain miscellaneous (`Adjective`)
/// stars. All derive [`StarCategory::Adjective`]. Adjective-star brightness and
/// 神煞 beyond this slice stay out of scope.
const ADJECTIVE_STAR_METADATA: [StarMetadata; 42] = [
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
    StarMetadata::new(
        "long_de_adj",
        "龙德",
        StarName::LongDeAdj,
        StarKind::Adjective,
    ),
    StarMetadata::new("jie_kong", "截空", StarName::JieKong, StarKind::Adjective),
    StarMetadata::new(
        "jie_sha_adj",
        "劫杀",
        StarName::JieShaAdj,
        StarKind::Adjective,
    ),
    StarMetadata::new(
        "da_hao_adj",
        "大耗",
        StarName::DaHaoAdj,
        StarKind::Adjective,
    ),
];

/// Factual metadata for all currently represented stars.
const REPRESENTED_STAR_METADATA: [StarMetadata; 70] = [
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
    StarMetadata::new(
        "long_de_adj",
        "龙德",
        StarName::LongDeAdj,
        StarKind::Adjective,
    ),
    StarMetadata::new("jie_kong", "截空", StarName::JieKong, StarKind::Adjective),
    StarMetadata::new(
        "jie_sha_adj",
        "劫杀",
        StarName::JieShaAdj,
        StarKind::Adjective,
    ),
    StarMetadata::new(
        "da_hao_adj",
        "大耗",
        StarName::DaHaoAdj,
        StarKind::Adjective,
    ),
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

/// Runtime inventory family for star names known from upstream iztro.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum KnownStarFamily {
    /// A star represented by current chart facts and placement metadata.
    Represented,
    /// Zhongzhou-only adjective star emitted by upstream iztro.
    ZhongzhouAdjective,
    /// Decorative 长生十二神 array entry.
    Changsheng12,
    /// Decorative 博士十二神 array entry.
    Boshi12,
    /// Decorative 岁前十二神 array entry.
    Suiqian12,
    /// Decorative 将前十二神 array entry.
    Jiangqian12,
    /// Decadal horoscope flow star.
    DecadalFlow,
    /// Yearly horoscope flow star.
    YearlyFlow,
    /// Monthly horoscope flow star.
    MonthlyFlow,
    /// Daily horoscope flow star.
    DailyFlow,
    /// Hourly horoscope flow star.
    HourlyFlow,
}

/// Metadata for an upstream iztro runtime star name known to this crate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct KnownStarMetadata {
    key: &'static str,
    upstream_key: &'static str,
    chinese_name: &'static str,
    name: StarName,
    family: KnownStarFamily,
    kind: Option<StarKind>,
}

impl KnownStarMetadata {
    /// Creates known upstream runtime star metadata.
    pub const fn new(
        key: &'static str,
        upstream_key: &'static str,
        chinese_name: &'static str,
        name: StarName,
        family: KnownStarFamily,
        kind: Option<StarKind>,
    ) -> Self {
        Self {
            key,
            upstream_key,
            chinese_name,
            name,
            family,
            kind,
        }
    }

    /// Returns the stable Rust inventory key.
    pub const fn key(&self) -> &'static str {
        self.key
    }

    /// Returns the upstream iztro locale/runtime key.
    pub const fn upstream_key(&self) -> &'static str {
        self.upstream_key
    }

    /// Returns the Chinese display name.
    pub const fn chinese_name(&self) -> &'static str {
        self.chinese_name
    }

    /// Returns the typed star identifier.
    pub const fn name(&self) -> StarName {
        self.name
    }

    /// Returns the upstream runtime inventory family.
    pub const fn family(&self) -> KnownStarFamily {
        self.family
    }

    /// Returns the iztro-compatible fine star type, if upstream assigns one.
    pub const fn kind(&self) -> Option<StarKind> {
        self.kind
    }

    /// Returns the coarse palace grouping, if this known star has a fine type.
    pub const fn category(&self) -> Option<StarCategory> {
        match self.kind {
            Some(kind) => Some(kind.category()),
            None => None,
        }
    }
}

/// Upstream iztro@2.5.8 runtime star-name inventory.
///
/// This table is intentionally broader than the represented metadata table. It
/// includes metadata-only runtime names from decorative arrays and horoscope
/// flow stars, but it does not imply placement support in `iztro-rs`.
const KNOWN_STAR_METADATA: [KnownStarMetadata; 170] = [
    KnownStarMetadata::new(
        "zi_wei",
        "ziweiMaj",
        "紫微",
        StarName::ZiWei,
        KnownStarFamily::Represented,
        Some(StarKind::Major),
    ),
    KnownStarMetadata::new(
        "tian_ji",
        "tianjiMaj",
        "天机",
        StarName::TianJi,
        KnownStarFamily::Represented,
        Some(StarKind::Major),
    ),
    KnownStarMetadata::new(
        "tai_yang",
        "taiyangMaj",
        "太阳",
        StarName::TaiYang,
        KnownStarFamily::Represented,
        Some(StarKind::Major),
    ),
    KnownStarMetadata::new(
        "wu_qu",
        "wuquMaj",
        "武曲",
        StarName::WuQu,
        KnownStarFamily::Represented,
        Some(StarKind::Major),
    ),
    KnownStarMetadata::new(
        "tian_tong",
        "tiantongMaj",
        "天同",
        StarName::TianTong,
        KnownStarFamily::Represented,
        Some(StarKind::Major),
    ),
    KnownStarMetadata::new(
        "lian_zhen",
        "lianzhenMaj",
        "廉贞",
        StarName::LianZhen,
        KnownStarFamily::Represented,
        Some(StarKind::Major),
    ),
    KnownStarMetadata::new(
        "tian_fu",
        "tianfuMaj",
        "天府",
        StarName::TianFu,
        KnownStarFamily::Represented,
        Some(StarKind::Major),
    ),
    KnownStarMetadata::new(
        "tai_yin",
        "taiyinMaj",
        "太阴",
        StarName::TaiYin,
        KnownStarFamily::Represented,
        Some(StarKind::Major),
    ),
    KnownStarMetadata::new(
        "tan_lang",
        "tanlangMaj",
        "贪狼",
        StarName::TanLang,
        KnownStarFamily::Represented,
        Some(StarKind::Major),
    ),
    KnownStarMetadata::new(
        "ju_men",
        "jumenMaj",
        "巨门",
        StarName::JuMen,
        KnownStarFamily::Represented,
        Some(StarKind::Major),
    ),
    KnownStarMetadata::new(
        "tian_xiang",
        "tianxiangMaj",
        "天相",
        StarName::TianXiang,
        KnownStarFamily::Represented,
        Some(StarKind::Major),
    ),
    KnownStarMetadata::new(
        "tian_liang",
        "tianliangMaj",
        "天梁",
        StarName::TianLiang,
        KnownStarFamily::Represented,
        Some(StarKind::Major),
    ),
    KnownStarMetadata::new(
        "qi_sha",
        "qishaMaj",
        "七杀",
        StarName::QiSha,
        KnownStarFamily::Represented,
        Some(StarKind::Major),
    ),
    KnownStarMetadata::new(
        "po_jun",
        "pojunMaj",
        "破军",
        StarName::PoJun,
        KnownStarFamily::Represented,
        Some(StarKind::Major),
    ),
    KnownStarMetadata::new(
        "zuo_fu",
        "zuofuMin",
        "左辅",
        StarName::ZuoFu,
        KnownStarFamily::Represented,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "you_bi",
        "youbiMin",
        "右弼",
        StarName::YouBi,
        KnownStarFamily::Represented,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "wen_chang",
        "wenchangMin",
        "文昌",
        StarName::WenChang,
        KnownStarFamily::Represented,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "wen_qu",
        "wenquMin",
        "文曲",
        StarName::WenQu,
        KnownStarFamily::Represented,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "tian_kui",
        "tiankuiMin",
        "天魁",
        StarName::TianKui,
        KnownStarFamily::Represented,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "tian_yue",
        "tianyueMin",
        "天钺",
        StarName::TianYue,
        KnownStarFamily::Represented,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "lu_cun",
        "lucunMin",
        "禄存",
        StarName::LuCun,
        KnownStarFamily::Represented,
        Some(StarKind::LuCun),
    ),
    KnownStarMetadata::new(
        "tian_ma",
        "tianmaMin",
        "天马",
        StarName::TianMa,
        KnownStarFamily::Represented,
        Some(StarKind::TianMa),
    ),
    KnownStarMetadata::new(
        "qing_yang",
        "qingyangMin",
        "擎羊",
        StarName::QingYang,
        KnownStarFamily::Represented,
        Some(StarKind::Tough),
    ),
    KnownStarMetadata::new(
        "tuo_luo",
        "tuoluoMin",
        "陀罗",
        StarName::TuoLuo,
        KnownStarFamily::Represented,
        Some(StarKind::Tough),
    ),
    KnownStarMetadata::new(
        "huo_xing",
        "huoxingMin",
        "火星",
        StarName::HuoXing,
        KnownStarFamily::Represented,
        Some(StarKind::Tough),
    ),
    KnownStarMetadata::new(
        "ling_xing",
        "lingxingMin",
        "铃星",
        StarName::LingXing,
        KnownStarFamily::Represented,
        Some(StarKind::Tough),
    ),
    KnownStarMetadata::new(
        "di_kong",
        "dikongMin",
        "地空",
        StarName::DiKong,
        KnownStarFamily::Represented,
        Some(StarKind::Tough),
    ),
    KnownStarMetadata::new(
        "di_jie",
        "dijieMin",
        "地劫",
        StarName::DiJie,
        KnownStarFamily::Represented,
        Some(StarKind::Tough),
    ),
    KnownStarMetadata::new(
        "hong_luan",
        "hongluan",
        "红鸾",
        StarName::HongLuan,
        KnownStarFamily::Represented,
        Some(StarKind::Flower),
    ),
    KnownStarMetadata::new(
        "tian_xi",
        "tianxi",
        "天喜",
        StarName::TianXi,
        KnownStarFamily::Represented,
        Some(StarKind::Flower),
    ),
    KnownStarMetadata::new(
        "tian_yao",
        "tianyao",
        "天姚",
        StarName::TianYao,
        KnownStarFamily::Represented,
        Some(StarKind::Flower),
    ),
    KnownStarMetadata::new(
        "tian_xing",
        "tianxing",
        "天刑",
        StarName::TianXing,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "tai_fu",
        "taifu",
        "台辅",
        StarName::TaiFu,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "feng_gao",
        "fenggao",
        "封诰",
        StarName::FengGao,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "san_tai",
        "santai",
        "三台",
        StarName::SanTai,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "ba_zuo",
        "bazuo",
        "八座",
        StarName::BaZuo,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "long_chi",
        "longchi",
        "龙池",
        StarName::LongChi,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "feng_ge",
        "fengge",
        "凤阁",
        StarName::FengGe,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "tian_ku",
        "tianku",
        "天哭",
        StarName::TianKu,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "tian_xu",
        "tianxu",
        "天虚",
        StarName::TianXu,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "en_guang",
        "engguang",
        "恩光",
        StarName::EnGuang,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "tian_gui",
        "tiangui",
        "天贵",
        StarName::TianGui,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "tian_wu",
        "tianwu",
        "天巫",
        StarName::TianWu,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "tian_yue_adj",
        "tianyue",
        "天月",
        StarName::TianYueAdj,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "yin_sha",
        "yinsha",
        "阴煞",
        StarName::YinSha,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "jie_shen",
        "jieshen",
        "解神",
        StarName::JieShen,
        KnownStarFamily::Represented,
        Some(StarKind::Helper),
    ),
    KnownStarMetadata::new(
        "hua_gai",
        "huagai",
        "华盖",
        StarName::HuaGai,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "gu_chen",
        "guchen",
        "孤辰",
        StarName::GuChen,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "gua_su",
        "guasu",
        "寡宿",
        StarName::GuaSu,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "fei_lian",
        "feilian",
        "蜚廉",
        StarName::FeiLian,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "po_sui",
        "posui",
        "破碎",
        StarName::PoSui,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "tian_de",
        "tiande",
        "天德",
        StarName::TianDe,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "yue_de",
        "yuede",
        "月德",
        StarName::YueDe,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "nian_jie",
        "nianjie",
        "年解",
        StarName::NianJie,
        KnownStarFamily::Represented,
        Some(StarKind::Helper),
    ),
    KnownStarMetadata::new(
        "xian_chi",
        "xianchi",
        "咸池",
        StarName::XianChi,
        KnownStarFamily::Represented,
        Some(StarKind::Flower),
    ),
    KnownStarMetadata::new(
        "tian_kong",
        "tiankong",
        "天空",
        StarName::TianKong,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "tian_guan",
        "tianguan",
        "天官",
        StarName::TianGuan,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "tian_chu",
        "tianchu",
        "天厨",
        StarName::TianChu,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "tian_fu_adj",
        "tianfu",
        "天福",
        StarName::TianFuAdj,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "tian_cai",
        "tiancai",
        "天才",
        StarName::TianCai,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "tian_shou",
        "tianshou",
        "天寿",
        StarName::TianShou,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "tian_shang",
        "tianshang",
        "天伤",
        StarName::TianShang,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "tian_shi",
        "tianshi",
        "天使",
        StarName::TianShi,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "jie_lu",
        "jielu",
        "截路",
        StarName::JieLu,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "kong_wang",
        "kongwang",
        "空亡",
        StarName::KongWang,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "xun_kong",
        "xunkong",
        "旬空",
        StarName::XunKong,
        KnownStarFamily::Represented,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "long_de_adj",
        "longde",
        "龙德",
        StarName::LongDeAdj,
        KnownStarFamily::ZhongzhouAdjective,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "jie_kong",
        "jiekong",
        "截空",
        StarName::JieKong,
        KnownStarFamily::ZhongzhouAdjective,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "jie_sha_adj",
        "jieshaAdj",
        "劫杀",
        StarName::JieShaAdj,
        KnownStarFamily::ZhongzhouAdjective,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "da_hao_adj",
        "dahao",
        "大耗",
        StarName::DaHaoAdj,
        KnownStarFamily::ZhongzhouAdjective,
        Some(StarKind::Adjective),
    ),
    KnownStarMetadata::new(
        "chang_sheng",
        "changsheng",
        "长生",
        StarName::ChangSheng,
        KnownStarFamily::Changsheng12,
        None,
    ),
    KnownStarMetadata::new(
        "mu_yu",
        "muyu",
        "沐浴",
        StarName::MuYu,
        KnownStarFamily::Changsheng12,
        None,
    ),
    KnownStarMetadata::new(
        "guan_dai",
        "guandai",
        "冠带",
        StarName::GuanDai,
        KnownStarFamily::Changsheng12,
        None,
    ),
    KnownStarMetadata::new(
        "lin_guan",
        "linguan",
        "临官",
        StarName::LinGuan,
        KnownStarFamily::Changsheng12,
        None,
    ),
    KnownStarMetadata::new(
        "di_wang",
        "diwang",
        "帝旺",
        StarName::DiWang,
        KnownStarFamily::Changsheng12,
        None,
    ),
    KnownStarMetadata::new(
        "shuai",
        "shuai",
        "衰",
        StarName::Shuai,
        KnownStarFamily::Changsheng12,
        None,
    ),
    KnownStarMetadata::new(
        "bing",
        "bing",
        "病",
        StarName::Bing,
        KnownStarFamily::Changsheng12,
        None,
    ),
    KnownStarMetadata::new(
        "si",
        "si",
        "死",
        StarName::Si,
        KnownStarFamily::Changsheng12,
        None,
    ),
    KnownStarMetadata::new(
        "mu",
        "mu",
        "墓",
        StarName::Mu,
        KnownStarFamily::Changsheng12,
        None,
    ),
    KnownStarMetadata::new(
        "jue",
        "jue",
        "绝",
        StarName::Jue,
        KnownStarFamily::Changsheng12,
        None,
    ),
    KnownStarMetadata::new(
        "tai",
        "tai",
        "胎",
        StarName::Tai,
        KnownStarFamily::Changsheng12,
        None,
    ),
    KnownStarMetadata::new(
        "yang",
        "yang",
        "养",
        StarName::Yang,
        KnownStarFamily::Changsheng12,
        None,
    ),
    KnownStarMetadata::new(
        "bo_shi",
        "boshi",
        "博士",
        StarName::BoShi,
        KnownStarFamily::Boshi12,
        None,
    ),
    KnownStarMetadata::new(
        "li_shi",
        "lishi",
        "力士",
        StarName::LiShi,
        KnownStarFamily::Boshi12,
        None,
    ),
    KnownStarMetadata::new(
        "qing_long",
        "qinglong",
        "青龙",
        StarName::QingLong,
        KnownStarFamily::Boshi12,
        None,
    ),
    KnownStarMetadata::new(
        "xiao_hao_boshi",
        "xiaohao",
        "小耗",
        StarName::XiaoHaoBoshi,
        KnownStarFamily::Boshi12,
        None,
    ),
    KnownStarMetadata::new(
        "jiang_jun",
        "jiangjun",
        "将军",
        StarName::JiangJun,
        KnownStarFamily::Boshi12,
        None,
    ),
    KnownStarMetadata::new(
        "zhou_shu",
        "zhoushu",
        "奏书",
        StarName::ZhouShu,
        KnownStarFamily::Boshi12,
        None,
    ),
    KnownStarMetadata::new(
        "fay_lian_boshi",
        "faylian",
        "飞廉",
        StarName::FayLianBoshi,
        KnownStarFamily::Boshi12,
        None,
    ),
    KnownStarMetadata::new(
        "xi_shen_boshi",
        "xishen",
        "喜神",
        StarName::XiShenBoshi,
        KnownStarFamily::Boshi12,
        None,
    ),
    KnownStarMetadata::new(
        "bing_fu_boshi",
        "bingfu",
        "病符",
        StarName::BingFuBoshi,
        KnownStarFamily::Boshi12,
        None,
    ),
    KnownStarMetadata::new(
        "da_hao_boshi",
        "dahao",
        "大耗",
        StarName::DaHaoBoshi,
        KnownStarFamily::Boshi12,
        None,
    ),
    KnownStarMetadata::new(
        "fu_bing",
        "fubing",
        "伏兵",
        StarName::FuBing,
        KnownStarFamily::Boshi12,
        None,
    ),
    KnownStarMetadata::new(
        "guan_fu_boshi",
        "guanfu",
        "官府",
        StarName::GuanFuBoshi,
        KnownStarFamily::Boshi12,
        None,
    ),
    KnownStarMetadata::new(
        "sui_jian",
        "suijian",
        "岁建",
        StarName::SuiJian,
        KnownStarFamily::Suiqian12,
        None,
    ),
    KnownStarMetadata::new(
        "hui_qi",
        "huiqi",
        "晦气",
        StarName::HuiQi,
        KnownStarFamily::Suiqian12,
        None,
    ),
    KnownStarMetadata::new(
        "sang_men",
        "sangmen",
        "丧门",
        StarName::SangMen,
        KnownStarFamily::Suiqian12,
        None,
    ),
    KnownStarMetadata::new(
        "guan_suo",
        "guansuo",
        "贯索",
        StarName::GuanSuo,
        KnownStarFamily::Suiqian12,
        None,
    ),
    KnownStarMetadata::new(
        "guan_fu_suiqian",
        "gwanfu",
        "官符",
        StarName::GuanFuSuiqian,
        KnownStarFamily::Suiqian12,
        None,
    ),
    KnownStarMetadata::new(
        "xiao_hao_suiqian",
        "xiaohao",
        "小耗",
        StarName::XiaoHaoSuiqian,
        KnownStarFamily::Suiqian12,
        None,
    ),
    KnownStarMetadata::new(
        "da_hao_suiqian",
        "dahao",
        "大耗",
        StarName::DaHaoSuiqian,
        KnownStarFamily::Suiqian12,
        None,
    ),
    KnownStarMetadata::new(
        "sui_po",
        "suipo",
        "岁破",
        StarName::SuiPo,
        KnownStarFamily::Suiqian12,
        None,
    ),
    KnownStarMetadata::new(
        "long_de_suiqian",
        "longde",
        "龙德",
        StarName::LongDeSuiqian,
        KnownStarFamily::Suiqian12,
        None,
    ),
    KnownStarMetadata::new(
        "bai_hu",
        "baihu",
        "白虎",
        StarName::BaiHu,
        KnownStarFamily::Suiqian12,
        None,
    ),
    KnownStarMetadata::new(
        "tian_de_suiqian",
        "tiande",
        "天德",
        StarName::TianDeSuiqian,
        KnownStarFamily::Suiqian12,
        None,
    ),
    KnownStarMetadata::new(
        "diao_ke",
        "diaoke",
        "吊客",
        StarName::DiaoKe,
        KnownStarFamily::Suiqian12,
        None,
    ),
    KnownStarMetadata::new(
        "bing_fu_suiqian",
        "bingfu",
        "病符",
        StarName::BingFuSuiqian,
        KnownStarFamily::Suiqian12,
        None,
    ),
    KnownStarMetadata::new(
        "jiang_xing",
        "jiangxing",
        "将星",
        StarName::JiangXing,
        KnownStarFamily::Jiangqian12,
        None,
    ),
    KnownStarMetadata::new(
        "pan_an",
        "panan",
        "攀鞍",
        StarName::PanAn,
        KnownStarFamily::Jiangqian12,
        None,
    ),
    KnownStarMetadata::new(
        "sui_yi",
        "suiyi",
        "岁驿",
        StarName::SuiYi,
        KnownStarFamily::Jiangqian12,
        None,
    ),
    KnownStarMetadata::new(
        "xi_shen_jiangqian",
        "xiishen",
        "息神",
        StarName::XiShenJiangqian,
        KnownStarFamily::Jiangqian12,
        None,
    ),
    KnownStarMetadata::new(
        "hua_gai_jiangqian",
        "huagai",
        "华盖",
        StarName::HuaGaiJiangqian,
        KnownStarFamily::Jiangqian12,
        None,
    ),
    KnownStarMetadata::new(
        "jie_sha",
        "jiesha",
        "劫煞",
        StarName::JieSha,
        KnownStarFamily::Jiangqian12,
        None,
    ),
    KnownStarMetadata::new(
        "zai_sha",
        "zhaisha",
        "灾煞",
        StarName::ZaiSha,
        KnownStarFamily::Jiangqian12,
        None,
    ),
    KnownStarMetadata::new(
        "tian_sha",
        "tiansha",
        "天煞",
        StarName::TianSha,
        KnownStarFamily::Jiangqian12,
        None,
    ),
    KnownStarMetadata::new(
        "zhi_bei",
        "zhibei",
        "指背",
        StarName::ZhiBei,
        KnownStarFamily::Jiangqian12,
        None,
    ),
    KnownStarMetadata::new(
        "xian_chi_jiangqian",
        "xianchi",
        "咸池",
        StarName::XianChiJiangqian,
        KnownStarFamily::Jiangqian12,
        None,
    ),
    KnownStarMetadata::new(
        "yue_sha",
        "yuesha",
        "月煞",
        StarName::YueSha,
        KnownStarFamily::Jiangqian12,
        None,
    ),
    KnownStarMetadata::new(
        "wang_shen",
        "wangshen",
        "亡神",
        StarName::WangShen,
        KnownStarFamily::Jiangqian12,
        None,
    ),
    KnownStarMetadata::new(
        "yun_kui",
        "yunkui",
        "运魁",
        StarName::YunKui,
        KnownStarFamily::DecadalFlow,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "yun_yue",
        "yunyue",
        "运钺",
        StarName::YunYue,
        KnownStarFamily::DecadalFlow,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "yun_chang",
        "yunchang",
        "运昌",
        StarName::YunChang,
        KnownStarFamily::DecadalFlow,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "yun_qu",
        "yunqu",
        "运曲",
        StarName::YunQu,
        KnownStarFamily::DecadalFlow,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "yun_lu",
        "yunlu",
        "运禄",
        StarName::YunLu,
        KnownStarFamily::DecadalFlow,
        Some(StarKind::LuCun),
    ),
    KnownStarMetadata::new(
        "yun_yang",
        "yunyang",
        "运羊",
        StarName::YunYang,
        KnownStarFamily::DecadalFlow,
        Some(StarKind::Tough),
    ),
    KnownStarMetadata::new(
        "yun_tuo",
        "yuntuo",
        "运陀",
        StarName::YunTuo,
        KnownStarFamily::DecadalFlow,
        Some(StarKind::Tough),
    ),
    KnownStarMetadata::new(
        "yun_ma",
        "yunma",
        "运马",
        StarName::YunMa,
        KnownStarFamily::DecadalFlow,
        Some(StarKind::TianMa),
    ),
    KnownStarMetadata::new(
        "yun_luan",
        "yunluan",
        "运鸾",
        StarName::YunLuan,
        KnownStarFamily::DecadalFlow,
        Some(StarKind::Flower),
    ),
    KnownStarMetadata::new(
        "yun_xi",
        "yunxi",
        "运喜",
        StarName::YunXi,
        KnownStarFamily::DecadalFlow,
        Some(StarKind::Flower),
    ),
    KnownStarMetadata::new(
        "liu_kui",
        "liukui",
        "流魁",
        StarName::LiuKui,
        KnownStarFamily::YearlyFlow,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "liu_yue",
        "liuyue",
        "流钺",
        StarName::LiuYue,
        KnownStarFamily::YearlyFlow,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "liu_chang",
        "liuchang",
        "流昌",
        StarName::LiuChang,
        KnownStarFamily::YearlyFlow,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "liu_qu",
        "liuqu",
        "流曲",
        StarName::LiuQu,
        KnownStarFamily::YearlyFlow,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "liu_lu",
        "liulu",
        "流禄",
        StarName::LiuLu,
        KnownStarFamily::YearlyFlow,
        Some(StarKind::LuCun),
    ),
    KnownStarMetadata::new(
        "liu_yang",
        "liuyang",
        "流羊",
        StarName::LiuYang,
        KnownStarFamily::YearlyFlow,
        Some(StarKind::Tough),
    ),
    KnownStarMetadata::new(
        "liu_tuo",
        "liutuo",
        "流陀",
        StarName::LiuTuo,
        KnownStarFamily::YearlyFlow,
        Some(StarKind::Tough),
    ),
    KnownStarMetadata::new(
        "liu_ma",
        "liuma",
        "流马",
        StarName::LiuMa,
        KnownStarFamily::YearlyFlow,
        Some(StarKind::TianMa),
    ),
    KnownStarMetadata::new(
        "liu_luan",
        "liuluan",
        "流鸾",
        StarName::LiuLuan,
        KnownStarFamily::YearlyFlow,
        Some(StarKind::Flower),
    ),
    KnownStarMetadata::new(
        "liu_xi",
        "liuxi",
        "流喜",
        StarName::LiuXi,
        KnownStarFamily::YearlyFlow,
        Some(StarKind::Flower),
    ),
    KnownStarMetadata::new(
        "nian_jie_yearly",
        "nianjie",
        "年解",
        StarName::NianJieYearly,
        KnownStarFamily::YearlyFlow,
        Some(StarKind::Helper),
    ),
    KnownStarMetadata::new(
        "yue_kui",
        "yuekui",
        "月魁",
        StarName::YueKui,
        KnownStarFamily::MonthlyFlow,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "yue_yue",
        "yueyue",
        "月钺",
        StarName::YueYue,
        KnownStarFamily::MonthlyFlow,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "yue_chang",
        "yuechang",
        "月昌",
        StarName::YueChang,
        KnownStarFamily::MonthlyFlow,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "yue_qu",
        "yuequ",
        "月曲",
        StarName::YueQu,
        KnownStarFamily::MonthlyFlow,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "yue_lu",
        "yuelu",
        "月禄",
        StarName::YueLu,
        KnownStarFamily::MonthlyFlow,
        Some(StarKind::LuCun),
    ),
    KnownStarMetadata::new(
        "yue_yang",
        "yueyang",
        "月羊",
        StarName::YueYang,
        KnownStarFamily::MonthlyFlow,
        Some(StarKind::Tough),
    ),
    KnownStarMetadata::new(
        "yue_tuo",
        "yuetuo",
        "月陀",
        StarName::YueTuo,
        KnownStarFamily::MonthlyFlow,
        Some(StarKind::Tough),
    ),
    KnownStarMetadata::new(
        "yue_ma",
        "yuema",
        "月马",
        StarName::YueMa,
        KnownStarFamily::MonthlyFlow,
        Some(StarKind::TianMa),
    ),
    KnownStarMetadata::new(
        "yue_luan",
        "yueluan",
        "月鸾",
        StarName::YueLuan,
        KnownStarFamily::MonthlyFlow,
        Some(StarKind::Flower),
    ),
    KnownStarMetadata::new(
        "yue_xi",
        "yuexi",
        "月喜",
        StarName::YueXi,
        KnownStarFamily::MonthlyFlow,
        Some(StarKind::Flower),
    ),
    KnownStarMetadata::new(
        "ri_kui",
        "rikui",
        "日魁",
        StarName::RiKui,
        KnownStarFamily::DailyFlow,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "ri_yue",
        "riyue",
        "日钺",
        StarName::RiYue,
        KnownStarFamily::DailyFlow,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "ri_chang",
        "richang",
        "日昌",
        StarName::RiChang,
        KnownStarFamily::DailyFlow,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "ri_qu",
        "riqu",
        "日曲",
        StarName::RiQu,
        KnownStarFamily::DailyFlow,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "ri_lu",
        "rilu",
        "日禄",
        StarName::RiLu,
        KnownStarFamily::DailyFlow,
        Some(StarKind::LuCun),
    ),
    KnownStarMetadata::new(
        "ri_yang",
        "riyang",
        "日羊",
        StarName::RiYang,
        KnownStarFamily::DailyFlow,
        Some(StarKind::Tough),
    ),
    KnownStarMetadata::new(
        "ri_tuo",
        "rituo",
        "日陀",
        StarName::RiTuo,
        KnownStarFamily::DailyFlow,
        Some(StarKind::Tough),
    ),
    KnownStarMetadata::new(
        "ri_ma",
        "rima",
        "日马",
        StarName::RiMa,
        KnownStarFamily::DailyFlow,
        Some(StarKind::TianMa),
    ),
    KnownStarMetadata::new(
        "ri_luan",
        "riluan",
        "日鸾",
        StarName::RiLuan,
        KnownStarFamily::DailyFlow,
        Some(StarKind::Flower),
    ),
    KnownStarMetadata::new(
        "ri_xi",
        "rixi",
        "日喜",
        StarName::RiXi,
        KnownStarFamily::DailyFlow,
        Some(StarKind::Flower),
    ),
    KnownStarMetadata::new(
        "shi_kui",
        "shikui",
        "时魁",
        StarName::ShiKui,
        KnownStarFamily::HourlyFlow,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "shi_yue",
        "shiyue",
        "时钺",
        StarName::ShiYue,
        KnownStarFamily::HourlyFlow,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "shi_chang",
        "shichang",
        "时昌",
        StarName::ShiChang,
        KnownStarFamily::HourlyFlow,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "shi_qu",
        "shiqu",
        "时曲",
        StarName::ShiQu,
        KnownStarFamily::HourlyFlow,
        Some(StarKind::Soft),
    ),
    KnownStarMetadata::new(
        "shi_lu",
        "shilu",
        "时禄",
        StarName::ShiLu,
        KnownStarFamily::HourlyFlow,
        Some(StarKind::LuCun),
    ),
    KnownStarMetadata::new(
        "shi_yang",
        "shiyang",
        "时羊",
        StarName::ShiYang,
        KnownStarFamily::HourlyFlow,
        Some(StarKind::Tough),
    ),
    KnownStarMetadata::new(
        "shi_tuo",
        "shituo",
        "时陀",
        StarName::ShiTuo,
        KnownStarFamily::HourlyFlow,
        Some(StarKind::Tough),
    ),
    KnownStarMetadata::new(
        "shi_ma",
        "shima",
        "时马",
        StarName::ShiMa,
        KnownStarFamily::HourlyFlow,
        Some(StarKind::TianMa),
    ),
    KnownStarMetadata::new(
        "shi_luan",
        "shiluan",
        "时鸾",
        StarName::ShiLuan,
        KnownStarFamily::HourlyFlow,
        Some(StarKind::Flower),
    ),
    KnownStarMetadata::new(
        "shi_xi",
        "shixi",
        "时喜",
        StarName::ShiXi,
        KnownStarFamily::HourlyFlow,
        Some(StarKind::Flower),
    ),
];

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

/// Returns metadata for all known upstream iztro runtime star names.
pub const fn known_star_metadata_table() -> &'static [KnownStarMetadata; 170] {
    &KNOWN_STAR_METADATA
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
        StarName::LongDeAdj => 38,
        StarName::JieKong => 39,
        StarName::JieShaAdj => 40,
        StarName::DaHaoAdj => 41,
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

/// Returns metadata for one known upstream runtime star, if inventoried.
pub fn try_known_star_metadata(star: StarName) -> Option<&'static KnownStarMetadata> {
    KNOWN_STAR_METADATA
        .iter()
        .find(|metadata| metadata.name == star)
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

/// Returns metadata for one known upstream runtime star.
pub fn known_star_metadata(star: StarName) -> &'static KnownStarMetadata {
    try_known_star_metadata(star).expect("star is not known in the upstream runtime inventory")
}
