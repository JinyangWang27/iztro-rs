use serde::{Deserialize, Serialize};

/// Stable identifiers for stars represented in chart facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarName {
    /// Zi Wei star (紫微).
    ZiWei,
    /// Tian Ji star (天机).
    TianJi,
    /// Tai Yang star (太阳).
    TaiYang,
    /// Wu Qu star (武曲).
    WuQu,
    /// Tian Tong star (天同).
    TianTong,
    /// Lian Zhen star (廉贞).
    LianZhen,
    /// Tian Fu star (天府).
    TianFu,
    /// Tai Yin star (太阴).
    TaiYin,
    /// Tan Lang star (贪狼).
    TanLang,
    /// Ju Men star (巨门).
    JuMen,
    /// Tian Xiang star (天相).
    TianXiang,
    /// Tian Liang star (天梁).
    TianLiang,
    /// Qi Sha star (七杀).
    QiSha,
    /// Po Jun star (破军).
    PoJun,
    /// Zuo Fu star (左辅).
    ZuoFu,
    /// You Bi star (右弼).
    YouBi,
    /// Wen Chang star (文昌).
    WenChang,
    /// Wen Qu star (文曲).
    WenQu,
    /// Tian Kui star (天魁).
    TianKui,
    /// Tian Yue star (天钺).
    TianYue,
    /// Lu Cun star (禄存).
    LuCun,
    /// Tian Ma star (天马).
    TianMa,
    /// Qing Yang star (擎羊).
    QingYang,
    /// Tuo Luo star (陀罗).
    TuoLuo,
    /// Huo Xing star (火星).
    HuoXing,
    /// Ling Xing star (铃星).
    LingXing,
    /// Di Kong star (地空).
    DiKong,
    /// Di Jie star (地劫).
    DiJie,
    /// Hong Luan star (红鸾).
    HongLuan,
    /// Tian Xi star (天喜).
    TianXi,
    /// Tian Yao star (天姚).
    TianYao,
    /// Tian Xing star (天刑).
    TianXing,
    /// Tai Fu star (台辅).
    TaiFu,
    /// Feng Gao star (封诰).
    FengGao,
    /// San Tai star (三台).
    SanTai,
    /// Ba Zuo star (八座).
    BaZuo,
    /// Long Chi star (龙池).
    LongChi,
    /// Feng Ge star (凤阁).
    FengGe,
    /// Tian Ku star (天哭).
    TianKu,
    /// Tian Xu star (天虚).
    TianXu,
    /// En Guang star (恩光).
    EnGuang,
    /// Tian Gui star (天贵).
    TianGui,
    /// Tian Wu star (天巫).
    TianWu,
    /// Tian Yue (天月) adjective star.
    ///
    /// Disambiguated from the minor star 天钺 ([`StarName::TianYue`]); both
    /// romanize to "Tian Yue", so this杂曜 uses the `tian_yue_adj` key.
    TianYueAdj,
    /// Yin Sha star (阴煞).
    YinSha,
    /// Jie Shen star (解神).
    JieShen,
    /// Hua Gai star (华盖).
    HuaGai,
    /// Gu Chen star (孤辰).
    GuChen,
    /// Gua Su star (寡宿).
    GuaSu,
    /// Fei Lian star (蜚廉).
    FeiLian,
    /// Po Sui star (破碎).
    PoSui,
    /// Tian De star (天德).
    TianDe,
    /// Yue De star (月德).
    YueDe,
    /// Nian Jie star (年解).
    NianJie,
    /// Xian Chi star (咸池), a peach-blossom 杂曜 from the birth year branch.
    XianChi,
    /// Tian Kong star (天空), one branch forward from the birth year branch.
    TianKong,
    /// Tian Guan star (天官), placed from the birth year stem.
    TianGuan,
    /// Tian Chu star (天厨), placed from the birth year stem.
    TianChu,
    /// Tian Fu (天福) adjective star, placed from the birth year stem.
    ///
    /// Disambiguated from the major star 天府 ([`StarName::TianFu`]); both
    /// romanize to "Tian Fu", so this 杂曜 uses the `tian_fu_adj` key.
    TianFuAdj,
    /// Tian Cai star (天才), anchored to the Life Palace.
    TianCai,
    /// Tian Shou star (天寿), anchored to the Body Palace.
    TianShou,
    /// Tian Shang star (天伤), anchored to the Life Palace (default algorithm).
    TianShang,
    /// Tian Shi star (天使), anchored to the Life Palace (default algorithm).
    TianShi,
    /// Jie Lu star (截路), placed from the birth year stem.
    JieLu,
    /// Kong Wang star (空亡), placed from the birth year stem.
    KongWang,
    /// Xun Kong star (旬空), the 旬中空亡 void branch matching the year polarity.
    XunKong,
}
