//! Central registry for canonical pattern identity, display metadata, and source provenance.
//!
//! Detector logic still owns evaluation semantics. This registry is the single
//! metadata table for stable ids, display names, aliases, family/polarity, and
//! optional verified source provenance.

use crate::rules::pattern::display_metadata::PatternDisplayMetadata;
use crate::rules::pattern::metadata::{PatternSourceGroup, PatternSourceMetadata};
use crate::rules::pattern::model::{PatternFamily, PatternId, PatternPolarity};
use crate::rules::source::ClassicalWork;

/// Canonical metadata for one pattern id.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PatternSpec {
    /// Stable pattern identifier.
    pub id: PatternId,
    /// Runtime display name.
    pub name_zh: &'static str,
    /// Runtime display aliases.
    pub aliases_zh: &'static [&'static str],
    /// Coarse family used for filtering and display grouping.
    pub family: PatternFamily,
    /// Pattern valence used for filtering and display grouping.
    pub polarity: PatternPolarity,
    /// Display/runtime metadata.
    pub display: PatternDisplayMetadata,
    /// Verified source provenance, when available.
    pub source: Option<PatternSourceMetadata>,
}

const EMPTY_ALIASES: &[&str] = &[];
const RI_CHU_FU_SANG_ALIASES: &[&str] = &["日出扶桑格"];
const LIAN_ZHEN_QI_SHA_ALIASES: &[&str] = &["廉贞七杀同宫"];
const TIAN_YI_GONG_MING_ALIASES: &[&str] = &["坐贵向贵"];
const QING_YANG_RU_MIAO_ALIASES: &[&str] = &["羊刃入庙"];

const fn source(
    pattern_id: PatternId,
    name_zh: &'static str,
    source_id: &'static str,
    source_text_zh_hans: &'static str,
    section: &'static str,
    group: PatternSourceGroup,
) -> PatternSourceMetadata {
    PatternSourceMetadata {
        pattern_id,
        name_zh,
        work: ClassicalWork::ZiWeiDouShuQuanShu,
        source_id,
        source_text_zh_hans,
        section,
        group,
    }
}

macro_rules! spec {
    (
        $id:expr,
        $name_zh:expr,
        $aliases_zh:expr,
        $family:expr,
        $polarity:expr,
        $condition_note_zh_hans:expr,
        $source_note_zh_hans:expr,
        $interpretation_note_zh_hans:expr,
        $source:expr $(,)?
    ) => {
        PatternSpec {
            id: $id,
            name_zh: $name_zh,
            aliases_zh: $aliases_zh,
            family: $family,
            polarity: $polarity,
            display: PatternDisplayMetadata {
                pattern_id: $id,
                name_zh: $name_zh,
                aliases_zh: $aliases_zh,
                condition_note_zh_hans: $condition_note_zh_hans,
                source_note_zh_hans: $source_note_zh_hans,
                interpretation_note_zh_hans: $interpretation_note_zh_hans,
            },
            source: $source,
        }
    };
}

/// Canonical pattern metadata for every [`PatternId`].
static PATTERN_SPECS_INNER: [PatternSpec; 31] = [
    spec!(
        PatternId::ZiFuChaoYuan,
        "紫府朝垣",
        EMPTY_ALIASES,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        "紫微与天府同在命宫三方四正。",
        None,
        None,
        None,
    ),
    spec!(
        PatternId::JiYueTongLiang,
        "机月同梁",
        EMPTY_ALIASES,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        "天机、太阴、天同、天梁齐会命宫三方四正。",
        None,
        None,
        None,
    ),
    spec!(
        PatternId::YangTuoJiaJi,
        "羊陀夹忌",
        EMPTY_ALIASES,
        PatternFamily::ShaJi,
        PatternPolarity::Inauspicious,
        "擎羊与陀罗夹住承载化忌的宫位。",
        None,
        None,
        None,
    ),
    spec!(
        PatternId::LingChangTuoWu,
        "铃昌陀武",
        EMPTY_ALIASES,
        PatternFamily::ShaJi,
        PatternPolarity::Inauspicious,
        "铃星、文昌、陀罗、武曲相关结构；当前保留 id，未注册检测器。",
        None,
        None,
        None,
    ),
    spec!(
        PatternId::ZuoYouJiaMing,
        "左右夹命",
        EMPTY_ALIASES,
        PatternFamily::AuxiliaryStarCombination,
        PatternPolarity::Auspicious,
        "左辅与右弼夹住命宫。",
        None,
        None,
        None,
    ),
    spec!(
        PatternId::ChangQuJiaMing,
        "昌曲夹命",
        EMPTY_ALIASES,
        PatternFamily::AuxiliaryStarCombination,
        PatternPolarity::Auspicious,
        "文昌与文曲夹住命宫。",
        None,
        None,
        None,
    ),
    spec!(
        PatternId::RiYueBingMing,
        "日月并明",
        EMPTY_ALIASES,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        "太阳与太阴皆在盘，且二者均为明亮状态。",
        None,
        None,
        None,
    ),
    spec!(
        PatternId::RiYueFanBei,
        "日月反背",
        EMPTY_ALIASES,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Inauspicious,
        "太阳与太阴皆在盘，且二者均为失辉落陷状态。",
        None,
        None,
        None,
    ),
    spec!(
        PatternId::JinCanGuangHui,
        "金灿光辉",
        EMPTY_ALIASES,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        "命宫在午，太阳在命宫，且太阳是该宫唯一主星。",
        None,
        None,
        Some(source(
            PatternId::JinCanGuangHui,
            "金灿光辉",
            "quan_shu.v01.ding_fu_ju.jin_can_guang_hui",
            "金灿光辉 太阳单守，命在午宫是也",
            "定富局",
            PatternSourceGroup::Wealth,
        )),
    ),
    spec!(
        PatternId::RiChuFuSang,
        "日照雷门",
        RI_CHU_FU_SANG_ALIASES,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        "出生时辰为卯至未，太阳天梁在卯宫坐命，与禄存、科权禄、左右、曲昌加会。",
        Some("日出扶桑 日在卯守命是也，守官禄宫亦然（紫微斗数全书）"),
        Some("公开 id 保留 RiChuFuSang；运行时显示采用日照雷门，又名日出扶桑格。"),
        Some(source(
            PatternId::RiChuFuSang,
            "日出扶桑",
            "quan_shu.v01.ding_gui_ju.ri_chu_fu_sang",
            "日出扶桑 日在卯守命是也，守官禄宫亦然",
            "定贵局",
            PatternSourceGroup::Noble,
        )),
    ),
    spec!(
        PatternId::YueLuoHaiGong,
        "月落亥宫",
        EMPTY_ALIASES,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        "太阴在亥，且亥宫为命宫。",
        None,
        None,
        Some(source(
            PatternId::YueLuoHaiGong,
            "月落亥宫",
            "quan_shu.v01.ding_gui_ju.yue_luo_hai_gong",
            "月落亥宫 月在亥守命是也，又名月朗天门",
            "定贵局",
            PatternSourceGroup::Noble,
        )),
    ),
    spec!(
        PatternId::YueShengCangHai,
        "月生沧海",
        EMPTY_ALIASES,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        "太阴在子，且子宫为田宅宫。",
        None,
        None,
        Some(source(
            PatternId::YueShengCangHai,
            "月生沧海",
            "quan_shu.v01.ding_gui_ju.yue_sheng_cang_hai",
            "月生沧海 月在子宫守田宅是也",
            "定贵局",
            PatternSourceGroup::Noble,
        )),
    ),
    spec!(
        PatternId::MaTouDaiJian,
        "马头带剑",
        EMPTY_ALIASES,
        PatternFamily::ShaJi,
        PatternPolarity::Neutral,
        "天马与擎羊同宫。",
        None,
        None,
        Some(source(
            PatternId::MaTouDaiJian,
            "马头带剑",
            "quan_shu.v01.ding_gui_ju.ma_tou_dai_jian",
            "马头带剑 谓马有刃是也不是居午格",
            "定贵局",
            PatternSourceGroup::Noble,
        )),
    ),
    spec!(
        PatternId::TanHuoXiangFeng,
        "贪火相逢",
        EMPTY_ALIASES,
        PatternFamily::ShaJi,
        PatternPolarity::Auspicious,
        "贪狼与火星同守命宫，且二者皆为明亮状态。",
        None,
        None,
        Some(source(
            PatternId::TanHuoXiangFeng,
            "贪火相逢",
            "quan_shu.v01.ding_gui_ju.tan_huo_xiang_feng",
            "贪火相逢 谓二星守命同居庙旺是也",
            "定贵局",
            PatternSourceGroup::Noble,
        )),
    ),
    spec!(
        PatternId::WuQuShouYuan,
        "武曲守垣",
        EMPTY_ALIASES,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        "武曲在命宫，且命宫地支为卯。",
        None,
        None,
        Some(source(
            PatternId::WuQuShouYuan,
            "武曲守垣",
            "quan_shu.v01.ding_gui_ju.wu_qu_shou_yuan",
            "武曲守垣 武守命卯宫是也，余不是",
            "定贵局",
            PatternSourceGroup::Noble,
        )),
    ),
    spec!(
        PatternId::CaiYuQiuChou,
        "财与囚仇",
        EMPTY_ALIASES,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Inauspicious,
        "武曲与廉贞同在命宫或身宫。",
        None,
        None,
        Some(source(
            PatternId::CaiYuQiuChou,
            "财与囚仇",
            "quan_shu.v01.ding_pin_jian_ju.cai_yu_qiu_chou",
            "财与囚仇 武贞同守身命是也",
            "定贫贱局",
            PatternSourceGroup::PovertyLowStatus,
        )),
    ),
    spec!(
        PatternId::MaLuoKongWang,
        "马落空亡",
        EMPTY_ALIASES,
        PatternFamily::ShaJi,
        PatternPolarity::Inauspicious,
        "天马与已建模空亡族星同宫。",
        None,
        None,
        Some(source(
            PatternId::MaLuoKongWang,
            "马落空亡",
            "quan_shu.v01.ding_pin_jian_ju.ma_luo_kong_wang",
            "马落空亡 马既落亡虽禄冲会无用主奔波",
            "定贫贱局",
            PatternSourceGroup::PovertyLowStatus,
        )),
    ),
    spec!(
        PatternId::MingLiFengKong,
        "命里逢空",
        EMPTY_ALIASES,
        PatternFamily::ShaJi,
        PatternPolarity::Inauspicious,
        "地劫、地空二星或其中一星守命。",
        Some("命里逢空不飘流即主疾苦（斗数骨髓赋）"),
        Some("有精神上孤独，钱不易留住之迹象。"),
        None,
    ),
    spec!(
        PatternId::LuFengChongPo,
        "禄逢冲破",
        EMPTY_ALIASES,
        PatternFamily::ShaJi,
        PatternPolarity::Inauspicious,
        "禄存或化禄坐命，在三方四正中，有地劫、地空冲破。",
        Some("禄逢冲破，吉处藏凶（太微赋）"),
        Some("吉处藏凶之象、应居安思危。"),
        None,
    ),
    spec!(
        PatternId::WenXingGongMing,
        "文星拱命",
        EMPTY_ALIASES,
        PatternFamily::AuxiliaryStarCombination,
        PatternPolarity::Auspicious,
        "文昌、文曲在命宫三方四正。",
        Some("阴阳会曲昌，出世荣华（斗数骨髓赋）"),
        Some("经国济世之天才，文学、医学、经济学亦显荣耀。"),
        None,
    ),
    spec!(
        PatternId::TianJiSiHai,
        "天机巳亥",
        EMPTY_ALIASES,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Inauspicious,
        "天机在巳或亥坐守命宫。",
        Some("出处待考。参考断语：天机巳亥宫，为人性似弓；商贾多精诈，计谋必离宗。"),
        Some("机智机诈难以分辨。"),
        None,
    ),
    spec!(
        PatternId::ZuoYouTongGong,
        "左右同宫",
        EMPTY_ALIASES,
        PatternFamily::AuxiliaryStarCombination,
        PatternPolarity::Auspicious,
        "命身宫入丑未，左辅右弼同宫，更于吉星同宫或加会者，为本格。",
        Some("出处待考。"),
        Some("端庄高士，性喜助人。"),
        None,
    ),
    spec!(
        PatternId::MingZhuChuHai,
        "明珠出海",
        EMPTY_ALIASES,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        "安命在未无正曜，卯宫太阳天梁、亥宫太阴入庙旺合照命宫，三方四正见禄存、科权禄、左右、曲昌、魁钺加会。",
        Some("三合明珠生旺地稳步蟾宫（斗数骨髓赋）"),
        None,
        None,
    ),
    spec!(
        PatternId::MingWuZhengYao,
        "命无正曜",
        EMPTY_ALIASES,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Neutral,
        "命宫里无主星坐命。",
        None,
        Some("个人特质不明显，发展不具特定方向。"),
        None,
    ),
    spec!(
        PatternId::JiXiangLiMing,
        "极向离明",
        EMPTY_ALIASES,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        "紫微在午宫坐命，三方四正无凑煞。",
        None,
        None,
        None,
    ),
    spec!(
        PatternId::FuXiangChaoYuan,
        "府相朝垣",
        EMPTY_ALIASES,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        "天府、天相二星一居财帛宫，一居官禄宫，来合照命宫，或者天府坐命，加会天相。命宫三方四正有禄存、科权禄、左右、曲昌、魁钺加会。",
        Some("府相朝垣 见前批注（紫微斗数全书）"),
        None,
        None,
    ),
    spec!(
        PatternId::ShiZhongYinYu,
        "石中隐玉",
        EMPTY_ALIASES,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        "命宫在子或午，巨门坐命，命宫三方四正有禄存／左右／曲昌／魁钺或禄／权／科加会。",
        None,
        None,
        Some(source(
            PatternId::ShiZhongYinYu,
            "石中隐玉",
            "quan_shu.v01.dou_shu_gu_sui_fu.shi_zhong_yin_yu",
            "子午巨门石中隐玉，明禄暗禄锦上添花",
            "斗数骨髓赋",
            PatternSourceGroup::DouShuGuSuiFu,
        )),
    ),
    spec!(
        PatternId::ZiFuJiaMing,
        "紫府夹命",
        EMPTY_ALIASES,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        "紫微与天府分居命宫两侧夹宫，任一方向皆可。",
        None,
        None,
        Some(source(
            PatternId::ZiFuJiaMing,
            "紫府夹命",
            "quan_shu.v03.zhu_xing_tong_yuan.zi_fu_jia_ming",
            "紫府夹命为贵格",
            "论诸星同垣各司所宜分别富贵贫贱夭寿",
            PatternSourceGroup::ZhuXingTongYuan,
        )),
    ),
    spec!(
        PatternId::LianZhenQiShaTongGong,
        "贞杀同宫",
        LIAN_ZHEN_QI_SHA_ALIASES,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Neutral,
        "命宫在丑或未，廉贞与七杀同守命宫。",
        None,
        Some(
            "本检测器仅识别廉贞七杀同守丑未命宫的结构；出处同时讨论庙旺与陷地化忌之别，此处只识别基础结构，不推衍如法律纠纷等现代断语。",
        ),
        Some(source(
            PatternId::LianZhenQiShaTongGong,
            "贞杀同宫",
            "quan_shu.v03.zhu_xing_tong_yuan.lian_zhen_qi_sha_miao_wang",
            "廉贞七杀居庙旺反为积富之人 杀居午奇格，若陷地化忌，贫贱残疾",
            "论诸星同垣各司所宜分别富贵贫贱夭寿",
            PatternSourceGroup::ZhuXingTongYuan,
        )),
    ),
    spec!(
        PatternId::TianYiGongMing,
        "天乙拱命",
        TIAN_YI_GONG_MING_ALIASES,
        PatternFamily::AuxiliaryStarCombination,
        PatternPolarity::Auspicious,
        "天魁、天钺一在命宫、一在迁移宫相对拱照。",
        None,
        Some("公开运行时显示名为天乙拱命，出处名为坐贵向贵。"),
        Some(source(
            PatternId::TianYiGongMing,
            "坐贵向贵",
            "quan_shu.v01.ding_gui_ju.zuo_gui_xiang_gui",
            "坐贵向贵 谓魁钺在命迭相坐拱是也",
            "定贵局",
            PatternSourceGroup::Noble,
        )),
    ),
    spec!(
        PatternId::QingYangRuMiao,
        "擎羊入庙",
        QING_YANG_RU_MIAO_ALIASES,
        PatternFamily::ShaJi,
        PatternPolarity::Auspicious,
        "命宫在辰戌丑未，擎羊守命，且命宫三方四正有禄存／左右／曲昌／魁钺或禄／权／科加会。无吉加会不产出。",
        Some("擎羊入庙富贵声扬 加吉万论（紫微斗数全书·卷三）"),
        Some("公开运行时显示名为擎羊入庙，出处名为羊刃入庙。"),
        Some(source(
            PatternId::QingYangRuMiao,
            "羊刃入庙",
            "quan_shu.v01.ding_gui_ju.yang_ren_ru_miao",
            "羊刃入庙 辰戍丑未守命遇吉是也",
            "定贵局",
            PatternSourceGroup::Noble,
        )),
    ),
];

/// Returns the canonical pattern spec for a known id.
///
/// This panics only if the internal registry fails to cover a public
/// [`PatternId`] — a registry-exhaustiveness invariant, not a user-input path.
/// External, binding, or MCP surfaces and any user-shaped lookup should call
/// [`try_pattern_spec`], which returns `None` instead of panicking.
pub fn pattern_spec(id: PatternId) -> &'static PatternSpec {
    try_pattern_spec(id).expect("pattern registry must cover every PatternId")
}

/// Returns the canonical pattern registry as a stable slice.
pub fn pattern_specs() -> &'static [PatternSpec] {
    &PATTERN_SPECS_INNER
}

/// Returns the canonical pattern spec if the id is present in the registry.
pub fn try_pattern_spec(id: PatternId) -> Option<&'static PatternSpec> {
    pattern_specs().iter().find(|spec| spec.id == id)
}
