use serde::{Deserialize, Serialize};

/// Stable identifiers for represented stars and inventoried upstream runtime names.
///
/// The derived [`Ord`]/[`PartialOrd`] follow the variant declaration order and
/// exist only to give facade/export snapshots a stable, deterministic star
/// ordering key (see [`crate::core::model::chart::facade_snapshot`]). They are a
/// canonical sort key, not an astrological ranking, and do not affect placement.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
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
    /// Zhongzhou-only Long De adjective star (龙德).
    LongDeAdj,
    /// Zhongzhou-only Jie Kong adjective star (截空).
    JieKong,
    /// Zhongzhou-only Jie Sha adjective star (劫杀).
    JieShaAdj,
    /// Zhongzhou-only Da Hao adjective star (大耗).
    DaHaoAdj,
    /// 12 Changsheng phase (长生12神：长生).
    ChangSheng,
    /// 12 Changsheng phase (长生12神：沐浴).
    MuYu,
    /// 12 Changsheng phase (长生12神：冠带).
    GuanDai,
    /// 12 Changsheng phase (长生12神：临官).
    LinGuan,
    /// 12 Changsheng phase (长生12神：帝旺).
    DiWang,
    /// 12 Changsheng phase (长生12神：衰).
    Shuai,
    /// 12 Changsheng phase (长生12神：病).
    Bing,
    /// 12 Changsheng phase (长生12神：死).
    Si,
    /// 12 Changsheng phase (长生12神：墓).
    Mu,
    /// 12 Changsheng phase (长生12神：绝).
    Jue,
    /// 12 Changsheng phase (长生12神：胎).
    Tai,
    /// 12 Changsheng phase (长生12神：养).
    Yang,
    /// Boshi 12 god entry (博士12神：博士).
    BoShi,
    /// Boshi 12 god entry (博士12神：力士).
    LiShi,
    /// Boshi 12 god entry (博士12神：青龙).
    QingLong,
    /// Boshi 12 god entry (博士12神：小耗).
    XiaoHaoBoshi,
    /// Boshi 12 god entry (博士12神：将军).
    JiangJun,
    /// Boshi 12 god entry (博士12神：奏书).
    ZhouShu,
    /// Boshi 12 god entry (博士12神：飞廉), disambiguated from natal 蜚廉.
    FayLianBoshi,
    /// Boshi 12 god entry (博士12神：喜神).
    XiShenBoshi,
    /// Boshi 12 god entry (博士12神：病符).
    BingFuBoshi,
    /// Boshi 12 god entry (博士12神：大耗).
    DaHaoBoshi,
    /// Boshi 12 god entry (博士12神：伏兵).
    FuBing,
    /// Boshi 12 god entry (博士12神：官府).
    GuanFuBoshi,
    /// Suiqian 12 god (岁前12神：岁建).
    SuiJian,
    /// Suiqian 12 god (岁前12神：晦气).
    HuiQi,
    /// Suiqian 12 god (岁前12神：丧门).
    SangMen,
    /// Suiqian 12 god (岁前12神：贯索).
    GuanSuo,
    /// Suiqian 12 god (岁前12神：官符).
    GuanFuSuiqian,
    /// Suiqian 12 god (岁前12神：小耗).
    XiaoHaoSuiqian,
    /// Suiqian 12 god (岁前12神：大耗).
    DaHaoSuiqian,
    /// Zhongzhou Suiqian god (岁破).
    SuiPo,
    /// Suiqian 12 god (岁前12神：龙德).
    LongDeSuiqian,
    /// Suiqian 12 god (岁前12神：白虎).
    BaiHu,
    /// Suiqian 12 god (岁前12神：天德).
    TianDeSuiqian,
    /// Suiqian 12 god (岁前12神：吊客).
    DiaoKe,
    /// Suiqian 12 god (岁前12神：病符).
    BingFuSuiqian,
    /// Jiangqian 12 god (将前12神：将星).
    JiangXing,
    /// Jiangqian 12 god (将前12神：攀鞍).
    PanAn,
    /// Jiangqian 12 god (将前12神：岁驿).
    SuiYi,
    /// Jiangqian 12 god (将前12神：息神).
    XiShenJiangqian,
    /// Jiangqian 12 god (将前12神：华盖).
    HuaGaiJiangqian,
    /// Jiangqian 12 god (将前12神：劫煞).
    JieSha,
    /// Jiangqian 12 god (将前12神：灾煞).
    ZaiSha,
    /// Jiangqian 12 god (将前12神：天煞).
    TianSha,
    /// Jiangqian 12 god (将前12神：指背).
    ZhiBei,
    /// Jiangqian 12 god (将前12神：咸池).
    XianChiJiangqian,
    /// Jiangqian 12 god (将前12神：月煞).
    YueSha,
    /// Jiangqian 12 god (将前12神：亡神).
    WangShen,
    /// Decadal flow star (运魁).
    YunKui,
    /// Decadal flow star (运钺).
    YunYue,
    /// Decadal flow star (运昌).
    YunChang,
    /// Decadal flow star (运曲).
    YunQu,
    /// Decadal flow star (运禄).
    YunLu,
    /// Decadal flow star (运羊).
    YunYang,
    /// Decadal flow star (运陀).
    YunTuo,
    /// Decadal flow star (运马).
    YunMa,
    /// Decadal flow star (运鸾).
    YunLuan,
    /// Decadal flow star (运喜).
    YunXi,
    /// Yearly flow star (流魁).
    LiuKui,
    /// Yearly flow star (流钺).
    LiuYue,
    /// Yearly flow star (流昌).
    LiuChang,
    /// Yearly flow star (流曲).
    LiuQu,
    /// Yearly flow star (流禄).
    LiuLu,
    /// Yearly flow star (流羊).
    LiuYang,
    /// Yearly flow star (流陀).
    LiuTuo,
    /// Yearly flow star (流马).
    LiuMa,
    /// Yearly flow star (流鸾).
    LiuLuan,
    /// Yearly flow star (流喜).
    LiuXi,
    /// Yearly flow 年解 helper, disambiguated from natal 年解.
    NianJieYearly,
    /// Monthly flow star (月魁).
    YueKui,
    /// Monthly flow star (月钺).
    YueYue,
    /// Monthly flow star (月昌).
    YueChang,
    /// Monthly flow star (月曲).
    YueQu,
    /// Monthly flow star (月禄).
    YueLu,
    /// Monthly flow star (月羊).
    YueYang,
    /// Monthly flow star (月陀).
    YueTuo,
    /// Monthly flow star (月马).
    YueMa,
    /// Monthly flow star (月鸾).
    YueLuan,
    /// Monthly flow star (月喜).
    YueXi,
    /// Daily flow star (日魁).
    RiKui,
    /// Daily flow star (日钺).
    RiYue,
    /// Daily flow star (日昌).
    RiChang,
    /// Daily flow star (日曲).
    RiQu,
    /// Daily flow star (日禄).
    RiLu,
    /// Daily flow star (日羊).
    RiYang,
    /// Daily flow star (日陀).
    RiTuo,
    /// Daily flow star (日马).
    RiMa,
    /// Daily flow star (日鸾).
    RiLuan,
    /// Daily flow star (日喜).
    RiXi,
    /// Hourly flow star (时魁).
    ShiKui,
    /// Hourly flow star (时钺).
    ShiYue,
    /// Hourly flow star (时昌).
    ShiChang,
    /// Hourly flow star (时曲).
    ShiQu,
    /// Hourly flow star (时禄).
    ShiLu,
    /// Hourly flow star (时羊).
    ShiYang,
    /// Hourly flow star (时陀).
    ShiTuo,
    /// Hourly flow star (时马).
    ShiMa,
    /// Hourly flow star (时鸾).
    ShiLuan,
    /// Hourly flow star (时喜).
    ShiXi,
}
