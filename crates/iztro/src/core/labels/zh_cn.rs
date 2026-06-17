//! Chinese (zh-CN) display labels for language-neutral domain enums.
//!
//! Every function here is a pure, deterministic, table-driven lookup from a
//! canonical Rust identity to its conventional Zi Wei Dou Shu Chinese term. The
//! exhaustive `match` arms mean the compiler enforces completeness: adding a new
//! enum variant fails to build until a label is supplied.
//!
//! These labels are an additive presentation concern. They never replace the
//! canonical machine-readable enum/value identity used internally or in JSON.

use crate::core::model::{
    chart::{DecorativeStarFamily, PalaceName},
    star::{
        Brightness, StarCategory, StarKind, StarName,
        mutagen::{Mutagen, Scope},
    },
};
use lunar_lite::{EarthlyBranch, HeavenlyStem};

/// Returns the Chinese label for a Heavenly Stem (天干).
pub const fn heavenly_stem_zh(stem: HeavenlyStem) -> &'static str {
    match stem {
        HeavenlyStem::Jia => "甲",
        HeavenlyStem::Yi => "乙",
        HeavenlyStem::Bing => "丙",
        HeavenlyStem::Ding => "丁",
        HeavenlyStem::Wu => "戊",
        HeavenlyStem::Ji => "己",
        HeavenlyStem::Geng => "庚",
        HeavenlyStem::Xin => "辛",
        HeavenlyStem::Ren => "壬",
        HeavenlyStem::Gui => "癸",
    }
}

/// Returns the Chinese label for an Earthly Branch (地支).
pub const fn earthly_branch_zh(branch: EarthlyBranch) -> &'static str {
    match branch {
        EarthlyBranch::Zi => "子",
        EarthlyBranch::Chou => "丑",
        EarthlyBranch::Yin => "寅",
        EarthlyBranch::Mao => "卯",
        EarthlyBranch::Chen => "辰",
        EarthlyBranch::Si => "巳",
        EarthlyBranch::Wu => "午",
        EarthlyBranch::Wei => "未",
        EarthlyBranch::Shen => "申",
        EarthlyBranch::You => "酉",
        EarthlyBranch::Xu => "戌",
        EarthlyBranch::Hai => "亥",
    }
}

/// Returns the Chinese label for a natal palace name (宫位).
pub const fn palace_name_zh(name: PalaceName) -> &'static str {
    match name {
        PalaceName::Life => "命宫",
        PalaceName::Siblings => "兄弟",
        PalaceName::Spouse => "夫妻",
        PalaceName::Children => "子女",
        PalaceName::Wealth => "财帛",
        PalaceName::Health => "疾厄",
        PalaceName::Migration => "迁移",
        PalaceName::Friends => "仆役",
        PalaceName::Career => "官禄",
        PalaceName::Property => "田宅",
        PalaceName::Spirit => "福德",
        PalaceName::Parents => "父母",
    }
}

/// Returns the Chinese label for a mutagen / four-transformation (四化).
pub const fn mutagen_zh(mutagen: Mutagen) -> &'static str {
    match mutagen {
        Mutagen::Lu => "禄",
        Mutagen::Quan => "权",
        Mutagen::Ke => "科",
        Mutagen::Ji => "忌",
    }
}

/// Returns the Chinese label for a star brightness (亮度).
///
/// [`Brightness::Unknown`] maps to the empty string, mirroring the upstream
/// convention that an uncalculated brightness carries no label.
pub const fn brightness_zh(brightness: Brightness) -> &'static str {
    match brightness {
        Brightness::Temple => "庙",
        Brightness::Prosperous => "旺",
        Brightness::Advantage => "得",
        Brightness::Favourable => "利",
        Brightness::Flat => "平",
        Brightness::Weak => "不",
        Brightness::Trapped => "陷",
        Brightness::Unknown => "",
    }
}

/// Returns the Chinese label for a coarse star category (星耀大类).
///
/// This is the grouping used by GUI-facing static chart views: the fourteen
/// major stars (主星), the supportive/tough minor stars (辅星), and the
/// miscellaneous symbolic markers (杂曜). It is coarser than [`star_kind_zh`].
pub const fn star_category_zh(category: StarCategory) -> &'static str {
    match category {
        StarCategory::Major => "主星",
        StarCategory::Minor => "辅星",
        StarCategory::Adjective => "杂曜",
    }
}

/// Returns the Chinese label for a horoscope scope (运限), as used by chart selectors.
pub const fn scope_zh(scope: Scope) -> &'static str {
    match scope {
        Scope::Natal => "本命",
        Scope::Decadal => "大限",
        Scope::Age => "小限",
        Scope::Yearly => "流年",
        Scope::Monthly => "流月",
        Scope::Daily => "流日",
        Scope::Hourly => "流时",
    }
}

/// Returns the Chinese label for a fine star kind (星耀分类).
pub const fn star_kind_zh(kind: StarKind) -> &'static str {
    match kind {
        StarKind::Major => "主星",
        StarKind::Soft => "吉星",
        StarKind::Tough => "煞星",
        StarKind::LuCun => "禄存",
        StarKind::TianMa => "天马",
        StarKind::Adjective => "杂曜",
        StarKind::Flower => "桃花星",
        StarKind::Helper => "助星",
    }
}

/// Returns the Chinese label for a decorative "twelve gods" family (十二神).
pub const fn decorative_star_family_zh(family: DecorativeStarFamily) -> &'static str {
    match family {
        DecorativeStarFamily::Changsheng12 => "长生十二神",
        DecorativeStarFamily::Boshi12 => "博士十二神",
        DecorativeStarFamily::Suiqian12 => "岁前十二神",
        DecorativeStarFamily::Jiangqian12 => "将前十二神",
    }
}

/// Returns the Chinese label for a star name (星耀).
pub const fn star_name_zh(name: StarName) -> &'static str {
    match name {
        StarName::ZiWei => "紫微",
        StarName::TianJi => "天机",
        StarName::TaiYang => "太阳",
        StarName::WuQu => "武曲",
        StarName::TianTong => "天同",
        StarName::LianZhen => "廉贞",
        StarName::TianFu => "天府",
        StarName::TaiYin => "太阴",
        StarName::TanLang => "贪狼",
        StarName::JuMen => "巨门",
        StarName::TianXiang => "天相",
        StarName::TianLiang => "天梁",
        StarName::QiSha => "七杀",
        StarName::PoJun => "破军",
        StarName::ZuoFu => "左辅",
        StarName::YouBi => "右弼",
        StarName::WenChang => "文昌",
        StarName::WenQu => "文曲",
        StarName::TianKui => "天魁",
        StarName::TianYue => "天钺",
        StarName::LuCun => "禄存",
        StarName::TianMa => "天马",
        StarName::QingYang => "擎羊",
        StarName::TuoLuo => "陀罗",
        StarName::HuoXing => "火星",
        StarName::LingXing => "铃星",
        StarName::DiKong => "地空",
        StarName::DiJie => "地劫",
        StarName::HongLuan => "红鸾",
        StarName::TianXi => "天喜",
        StarName::TianYao => "天姚",
        StarName::TianXing => "天刑",
        StarName::TaiFu => "台辅",
        StarName::FengGao => "封诰",
        StarName::SanTai => "三台",
        StarName::BaZuo => "八座",
        StarName::LongChi => "龙池",
        StarName::FengGe => "凤阁",
        StarName::TianKu => "天哭",
        StarName::TianXu => "天虚",
        StarName::EnGuang => "恩光",
        StarName::TianGui => "天贵",
        StarName::TianWu => "天巫",
        StarName::TianYueAdj => "天月",
        StarName::YinSha => "阴煞",
        StarName::JieShen => "解神",
        StarName::HuaGai => "华盖",
        StarName::GuChen => "孤辰",
        StarName::GuaSu => "寡宿",
        StarName::FeiLian => "蜚廉",
        StarName::PoSui => "破碎",
        StarName::TianDe => "天德",
        StarName::YueDe => "月德",
        StarName::NianJie => "年解",
        StarName::XianChi => "咸池",
        StarName::TianKong => "天空",
        StarName::TianGuan => "天官",
        StarName::TianChu => "天厨",
        StarName::TianFuAdj => "天福",
        StarName::TianCai => "天才",
        StarName::TianShou => "天寿",
        StarName::TianShang => "天伤",
        StarName::TianShi => "天使",
        StarName::JieLu => "截路",
        StarName::KongWang => "空亡",
        StarName::XunKong => "旬空",
        StarName::LongDeAdj => "龙德",
        StarName::JieKong => "截空",
        StarName::JieShaAdj => "劫杀",
        StarName::DaHaoAdj => "大耗",
        StarName::ChangSheng => "长生",
        StarName::MuYu => "沐浴",
        StarName::GuanDai => "冠带",
        StarName::LinGuan => "临官",
        StarName::DiWang => "帝旺",
        StarName::Shuai => "衰",
        StarName::Bing => "病",
        StarName::Si => "死",
        StarName::Mu => "墓",
        StarName::Jue => "绝",
        StarName::Tai => "胎",
        StarName::Yang => "养",
        StarName::BoShi => "博士",
        StarName::LiShi => "力士",
        StarName::QingLong => "青龙",
        StarName::XiaoHaoBoshi => "小耗",
        StarName::JiangJun => "将军",
        StarName::ZhouShu => "奏书",
        StarName::FayLianBoshi => "飞廉",
        StarName::XiShenBoshi => "喜神",
        StarName::BingFuBoshi => "病符",
        StarName::DaHaoBoshi => "大耗",
        StarName::FuBing => "伏兵",
        StarName::GuanFuBoshi => "官府",
        StarName::SuiJian => "岁建",
        StarName::HuiQi => "晦气",
        StarName::SangMen => "丧门",
        StarName::GuanSuo => "贯索",
        StarName::GuanFuSuiqian => "官符",
        StarName::XiaoHaoSuiqian => "小耗",
        StarName::DaHaoSuiqian => "大耗",
        StarName::SuiPo => "岁破",
        StarName::LongDeSuiqian => "龙德",
        StarName::BaiHu => "白虎",
        StarName::TianDeSuiqian => "天德",
        StarName::DiaoKe => "吊客",
        StarName::BingFuSuiqian => "病符",
        StarName::JiangXing => "将星",
        StarName::PanAn => "攀鞍",
        StarName::SuiYi => "岁驿",
        StarName::XiShenJiangqian => "息神",
        StarName::HuaGaiJiangqian => "华盖",
        StarName::JieSha => "劫煞",
        StarName::ZaiSha => "灾煞",
        StarName::TianSha => "天煞",
        StarName::ZhiBei => "指背",
        StarName::XianChiJiangqian => "咸池",
        StarName::YueSha => "月煞",
        StarName::WangShen => "亡神",
        StarName::YunKui => "运魁",
        StarName::YunYue => "运钺",
        StarName::YunChang => "运昌",
        StarName::YunQu => "运曲",
        StarName::YunLu => "运禄",
        StarName::YunYang => "运羊",
        StarName::YunTuo => "运陀",
        StarName::YunMa => "运马",
        StarName::YunLuan => "运鸾",
        StarName::YunXi => "运喜",
        StarName::LiuKui => "流魁",
        StarName::LiuYue => "流钺",
        StarName::LiuChang => "流昌",
        StarName::LiuQu => "流曲",
        StarName::LiuLu => "流禄",
        StarName::LiuYang => "流羊",
        StarName::LiuTuo => "流陀",
        StarName::LiuMa => "流马",
        StarName::LiuLuan => "流鸾",
        StarName::LiuXi => "流喜",
        StarName::NianJieYearly => "年解",
        StarName::YueKui => "月魁",
        StarName::YueYue => "月钺",
        StarName::YueChang => "月昌",
        StarName::YueQu => "月曲",
        StarName::YueLu => "月禄",
        StarName::YueYang => "月羊",
        StarName::YueTuo => "月陀",
        StarName::YueMa => "月马",
        StarName::YueLuan => "月鸾",
        StarName::YueXi => "月喜",
        StarName::RiKui => "日魁",
        StarName::RiYue => "日钺",
        StarName::RiChang => "日昌",
        StarName::RiQu => "日曲",
        StarName::RiLu => "日禄",
        StarName::RiYang => "日羊",
        StarName::RiTuo => "日陀",
        StarName::RiMa => "日马",
        StarName::RiLuan => "日鸾",
        StarName::RiXi => "日喜",
        StarName::ShiKui => "时魁",
        StarName::ShiYue => "时钺",
        StarName::ShiChang => "时昌",
        StarName::ShiQu => "时曲",
        StarName::ShiLu => "时禄",
        StarName::ShiYang => "时羊",
        StarName::ShiTuo => "时陀",
        StarName::ShiMa => "时马",
        StarName::ShiLuan => "时鸾",
        StarName::ShiXi => "时喜",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heavenly_stem_labels_cover_all_ten_stems() {
        let pairs = [
            (HeavenlyStem::Jia, "甲"),
            (HeavenlyStem::Yi, "乙"),
            (HeavenlyStem::Bing, "丙"),
            (HeavenlyStem::Ding, "丁"),
            (HeavenlyStem::Wu, "戊"),
            (HeavenlyStem::Ji, "己"),
            (HeavenlyStem::Geng, "庚"),
            (HeavenlyStem::Xin, "辛"),
            (HeavenlyStem::Ren, "壬"),
            (HeavenlyStem::Gui, "癸"),
        ];
        for (stem, expected) in pairs {
            assert_eq!(heavenly_stem_zh(stem), expected);
        }
    }

    #[test]
    fn earthly_branch_labels_cover_all_twelve_branches() {
        let pairs = [
            (EarthlyBranch::Zi, "子"),
            (EarthlyBranch::Chou, "丑"),
            (EarthlyBranch::Yin, "寅"),
            (EarthlyBranch::Mao, "卯"),
            (EarthlyBranch::Chen, "辰"),
            (EarthlyBranch::Si, "巳"),
            (EarthlyBranch::Wu, "午"),
            (EarthlyBranch::Wei, "未"),
            (EarthlyBranch::Shen, "申"),
            (EarthlyBranch::You, "酉"),
            (EarthlyBranch::Xu, "戌"),
            (EarthlyBranch::Hai, "亥"),
        ];
        for (branch, expected) in pairs {
            assert_eq!(earthly_branch_zh(branch), expected);
        }
    }

    #[test]
    fn palace_name_labels_cover_all_twelve_palaces() {
        let pairs = [
            (PalaceName::Life, "命宫"),
            (PalaceName::Siblings, "兄弟"),
            (PalaceName::Spouse, "夫妻"),
            (PalaceName::Children, "子女"),
            (PalaceName::Wealth, "财帛"),
            (PalaceName::Health, "疾厄"),
            (PalaceName::Migration, "迁移"),
            (PalaceName::Friends, "仆役"),
            (PalaceName::Career, "官禄"),
            (PalaceName::Property, "田宅"),
            (PalaceName::Spirit, "福德"),
            (PalaceName::Parents, "父母"),
        ];
        for (name, expected) in pairs {
            assert_eq!(palace_name_zh(name), expected);
        }
    }

    #[test]
    fn mutagen_labels_cover_all_four_transformations() {
        assert_eq!(mutagen_zh(Mutagen::Lu), "禄");
        assert_eq!(mutagen_zh(Mutagen::Quan), "权");
        assert_eq!(mutagen_zh(Mutagen::Ke), "科");
        assert_eq!(mutagen_zh(Mutagen::Ji), "忌");
    }

    #[test]
    fn brightness_labels_cover_all_states() {
        assert_eq!(brightness_zh(Brightness::Temple), "庙");
        assert_eq!(brightness_zh(Brightness::Prosperous), "旺");
        assert_eq!(brightness_zh(Brightness::Advantage), "得");
        assert_eq!(brightness_zh(Brightness::Favourable), "利");
        assert_eq!(brightness_zh(Brightness::Flat), "平");
        assert_eq!(brightness_zh(Brightness::Weak), "不");
        assert_eq!(brightness_zh(Brightness::Trapped), "陷");
        assert_eq!(brightness_zh(Brightness::Unknown), "");
    }

    #[test]
    fn star_category_labels_cover_all_categories() {
        assert_eq!(star_category_zh(StarCategory::Major), "主星");
        assert_eq!(star_category_zh(StarCategory::Minor), "辅星");
        assert_eq!(star_category_zh(StarCategory::Adjective), "杂曜");
    }

    #[test]
    fn scope_labels_cover_all_scopes() {
        assert_eq!(scope_zh(Scope::Natal), "本命");
        assert_eq!(scope_zh(Scope::Decadal), "大限");
        assert_eq!(scope_zh(Scope::Age), "小限");
        assert_eq!(scope_zh(Scope::Yearly), "流年");
        assert_eq!(scope_zh(Scope::Monthly), "流月");
        assert_eq!(scope_zh(Scope::Daily), "流日");
        assert_eq!(scope_zh(Scope::Hourly), "流时");
    }

    #[test]
    fn star_kind_labels_cover_all_kinds() {
        assert_eq!(star_kind_zh(StarKind::Major), "主星");
        assert_eq!(star_kind_zh(StarKind::Soft), "吉星");
        assert_eq!(star_kind_zh(StarKind::Tough), "煞星");
        assert_eq!(star_kind_zh(StarKind::LuCun), "禄存");
        assert_eq!(star_kind_zh(StarKind::TianMa), "天马");
        assert_eq!(star_kind_zh(StarKind::Adjective), "杂曜");
        assert_eq!(star_kind_zh(StarKind::Flower), "桃花星");
        assert_eq!(star_kind_zh(StarKind::Helper), "助星");
    }

    #[test]
    fn decorative_star_family_labels_cover_all_families() {
        assert_eq!(
            decorative_star_family_zh(DecorativeStarFamily::Changsheng12),
            "长生十二神"
        );
        assert_eq!(
            decorative_star_family_zh(DecorativeStarFamily::Boshi12),
            "博士十二神"
        );
        assert_eq!(
            decorative_star_family_zh(DecorativeStarFamily::Suiqian12),
            "岁前十二神"
        );
        assert_eq!(
            decorative_star_family_zh(DecorativeStarFamily::Jiangqian12),
            "将前十二神"
        );
    }

    #[test]
    fn star_name_labels_cover_representative_stars() {
        // Major stars (主星).
        assert_eq!(star_name_zh(StarName::ZiWei), "紫微");
        assert_eq!(star_name_zh(StarName::PoJun), "破军");
        // Minor / soft / tough stars (辅星).
        assert_eq!(star_name_zh(StarName::ZuoFu), "左辅");
        assert_eq!(star_name_zh(StarName::QingYang), "擎羊");
        assert_eq!(star_name_zh(StarName::LuCun), "禄存");
        // Adjective stars (杂曜), including the disambiguated homophones.
        assert_eq!(star_name_zh(StarName::HuaGai), "华盖");
        assert_eq!(star_name_zh(StarName::TianYueAdj), "天月");
        assert_eq!(star_name_zh(StarName::TianFuAdj), "天福");
        // Decorative "twelve gods" entries.
        assert_eq!(star_name_zh(StarName::ChangSheng), "长生");
        assert_eq!(star_name_zh(StarName::BoShi), "博士");
        assert_eq!(star_name_zh(StarName::JiangXing), "将星");
        // Flow stars across scopes.
        assert_eq!(star_name_zh(StarName::YunKui), "运魁");
        assert_eq!(star_name_zh(StarName::ShiXi), "时喜");
    }

    #[test]
    fn homophone_stars_keep_distinct_labels_from_their_namesakes() {
        // Disambiguated romanizations still carry their conventional characters.
        assert_eq!(star_name_zh(StarName::TianYue), "天钺");
        assert_eq!(star_name_zh(StarName::TianYueAdj), "天月");
        assert_eq!(star_name_zh(StarName::TianFu), "天府");
        assert_eq!(star_name_zh(StarName::TianFuAdj), "天福");
    }
}
