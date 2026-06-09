# 兼容性政策

`iztro-rs` 受 `iztro` 启发，早期应在适用范围内以 `iztro` 校验排盘行为。

## 兼容性的含义

兼容性意味着：

- 选定的排盘输出应与 `iztro` golden fixtures 保持一致；
- 差异应明确记录；
- Rust 公开模型应尽量保留相同的星盘事实；
- 测试应显式说明兼容目标，而不是隐含假设。

## 兼容性不意味着

兼容性不要求：

- 内部架构完全一致；
- 公开 API 名称完全一致；
- 数据表示仍然是字符串优先；
- 解盘叙事输出完全一致；
- 第一版支持 `iztro` 的全部功能。

## 兼容目标

当前兼容目标为：

- `iztro` npm package version `2.5.8`。

后续兼容 fixture 如需更新目标版本，必须同时记录版本变化和预期输出差异。

本地检查上游行为时，可使用 `tools/iztro-reference` 下固定版本的 npm reference
workspace：`npm ci --prefix tools/iztro-reference`。已提交的 fixture JSON 仍是兼容性
source of truth。

## 星曜名称清单

`iztro-core` 现在保留两套相互独立的星曜 metadata surface：

- `represented_star_metadata_table()` 仍保持严格边界：只覆盖当前由星盘事实表示、由
  Rust 代码安放、并由 fixtures 校验的 **66** 颗星。
- `known_star_metadata_table()` 清点更广的上游 `iztro@2.5.8` runtime 星曜名称宇宙：
  **170** 个已知条目，包含已表示星曜、中州派特有杂曜、装饰性 runtime 数组
  （`changsheng12`、`boshi12`、`suiqian12`、`jiangqian12`），以及大限、流年、流月、
  流日、流时的 horoscope 流曜名称。

Known metadata 仅为清单，不表示 `iztro-rs` 已安放这些星曜、以 fixtures 校验它们、
分配亮度、推导时间宫位，或实现 horoscope 安星。装饰性 runtime 条目在上游没有
`FunctionalStar` type，因此没有 `StarKind`；horoscope 流曜条目则保留 iztro 已分配
的细分类。

上游 locale key `xunzhong` / `旬中` 被有意排除，因为在 `iztro@2.5.8` 中没有找到
内置的 `FunctionalStar` 构造或 `StarType` 分配。四化仍作为 `Mutagen` /
`MutagenActivation` 事实存在，而不是 `StarName` variants。

## 公开 facade 兼容性

`by_lunar` 是 `iztro-rs` 的第一个 iztro-compatible facade 入口。它在概念上对应
iztro 的 `astro.byLunar(...)`，但使用强类型的 `LunarChartRequest` 请求对象，而
不是 JavaScript 风格的位置参数。

该 facade 只把传入的农历日期记录为星盘输入事实，并委托给已支持星曜的本命盘
builder；它不执行阳历转农历转换。出生年干和年支仍需显式提供，因为公历/农历年份
到干支年的推导仍未实现。

`by_solar`、闰月处理、早晚子时变体，以及完整历法行为仍延期实现。

## 运限层模型

`iztro-core` 定义了仅模型的运限叠加层：`HoroscopeChart` 包裹不可变的本命 `Chart`，
并持有零个或多个 `TemporalLayer`，每个层带有非本命的 `Scope`、强类型的
`TemporalContext`、按范围划分的 `StarPlacement` 和 `MutagenActivation`。这些模型
只承载调用方显式提供的时间事实，且层不会复制本命落点。

流年四化叠加层 builder（`build_yearly_mutagen_layer`）现已作为第一个模型级时间
激活 builder 提供。给定本命 `Chart` 和显式的流年干支 / 农历年
（`YearlyMutagenLayerInput`），它生成一个 `Scope::Yearly` 的 `TemporalLayer`，
其中的 `MutagenActivation` 将流年天干套用到本命盘中实际存在的已表示星曜上，复用
共享的天干四化表。它不推导任何历法事实、不安放流曜、不修改本命落点，也不做完整
运限解读。缺失或不支持的目标星会被跳过而非凭空生成。四化仍作为
`MutagenActivation` 事实，而非独立星曜。

大限四化叠加层 builder（`build_decadal_mutagen_layer`）现已作为模型级时间激活
builder 与流年版一并提供。给定本命 `Chart` 和显式的大限干支及起运年龄
（`DecadalMutagenLayerInput`），它生成一个 `Scope::Decadal` 的 `TemporalLayer`，
其中的 `MutagenActivation` 将大限天干套用到本命盘中实际存在的已表示星曜上，复用
同一张共享的天干四化表。它只接受显式的大限干支/上下文事实：不推导年龄区间、不推导
大限命宫、不推导大限宫位布局、不推导任何历法事实，不安放流曜，不修改本命落点，也
不做完整运限解读。缺失或不支持的目标星会被跳过而非凭空生成。对于相同的天干和本命
盘，大限与流年 builder 产出相同的目标星 / 目标地支 / 四化三元组，只在 `source_scope`
和 `TemporalContext` 上不同。四化仍作为 `MutagenActivation` 事实，而非独立星曜。

时间范围安星、大限/流年推导、年份到干支转换，以及历法推导仍不在范围内。这些模型
目前尚未以 `iztro` 运限 fixtures 校验。

## 当前 fixtures

fixtures 为：

- `fixtures/iztro/minimal_natal_1990_05_17_chen_female.json`
- `fixtures/iztro/major_stars_1990_05_17_chen_female.json`
- `fixtures/iztro/minor_stars_1990_05_17_chen_female.json`
- `fixtures/iztro/minor_stars_1988_03_14_zi_male.json`
- `fixtures/iztro/minor_stars_1991_08_09_hai_female.json`
- `fixtures/iztro/adjective_stars_full_default_1990_05_17_chen_female.json`
- `fixtures/iztro/adjective_stars_full_default_1988_03_14_zi_male.json`
- `fixtures/iztro/adjective_stars_full_default_1991_08_09_hai_female.json`

仅保留当前完整默认算法杂曜 fixtures（每个 38 颗星）在源码树中；更早、更小的杂曜
子集可通过 git history 获取。

minimal-natal fixture 只比较 `iztro-rs` 当前已实现的字段：

- 出生时辰；
- 性别；
- 命宫地支；
- 身宫地支；
- 十二宫地支；
- 十二宫名称；
- 十二宫天干；
- 五行局。

十二宫天干由出生年干按起五行寅例生成，并与 iztro 每宫的 `heavenlyStem`
对照。五行局与 iztro 的 `fiveElementsClass` 对照（`火六局` 映射为 `fire6`）。

出生年干目前通过 fixture input 中的 `birth_year_stem` 显式提供，因为公历到干支年
推导尚未实现。

它有意不比较星曜、亮度、四化、大限、流年或解读文本。

### 十四主星

`major_stars_1990_05_17_chen_female.json` 比较十四主星已表示的事实，与 iztro
每宫的 `majorStars` 对照：

- 每宫的主星名称；
- 每颗主星所在的宫位地支；
- 每颗主星的亮度；
- 已表示主星的生年四化。

安星复现 iztro 2.5.8（`getStartIndex` 与 `getMajorStar`）：紫微由五行局与农历
日推出，天府为紫微关于寅–申轴的镜像，紫微系与天府系按固定偏移排布。每颗星的
`StarKind` 为 `Major`，派生出的 `StarCategory` 为 `Major`，scope 为 `natal`
（iztro 的 `origin`）。
亮度复现 iztro 2.5.8 `STARS_INFO` 中十四主星的表，保留 `de`（`得`）为
`advantage`，`li`（`利`）为 `favourable`。生年四化复现 iztro 2.5.8 的天干四化
表，但仅在目标星属于当前已表示的十四主星时记录。

星曜分类采用两层模型。`StarKind` 存储与 iztro 兼容的细分类（`major`、`soft`、
`tough`、`lucun`、`tianma`、`adjective`、`flower` 或 `helper`）。
`StarCategory` 是派生出的粗粒度宫位分组：`major`、`minor` 或 `adjective`。
四化作为 `mutagen: Option<Mutagen>` 保留在 placement 的独立事实状态中，不编码为
star kind 或 category。

农历日通过 `input.lunar_day` 显式提供，因为完整历法转换尚未实现。兼容性测试走公开
的 `build_natal_chart_with_major_stars` builder path：先生成 minimal natal chart，
再使用其派生出的五行局、显式农历日和显式出生年干安十四主星，并附加已支持的事实
状态。该 fixture 仍**不**比较特征提取、规则引擎输出、叙事输出、历法转换、辅星、
杂曜、非主星、非主星四化、大限、流年或其他时间范围。

### 十四已支持辅星

三个 `minor_stars_*` fixtures 比较十四颗已支持本命辅星的事实，与 iztro 每宫的
`minorStars` 对照：

- 每宫的辅星名称；
- 每颗辅星所在的宫位地支；
- 与 iztro 兼容的星曜细分类（`soft`、`tough`、`lucun` 或 `tianma`）；
- iztro 2.5.8 有亮度表时的亮度；
- 已表示辅星目标的生年四化。

安星复现 iztro 2.5.8 的寅宫索引公式：

- 左辅、右弼由显式农历月份决定；
- 文昌、文曲和地空、地劫由出生时辰地支决定；
- 天魁、天钺和禄存、擎羊、陀罗由出生年干决定；
- 天马和火星、铃星由出生年支决定，其中火星、铃星还使用出生时辰地支。

每颗已支持辅星的派生 `StarCategory` 为 `Minor`，scope 为本命。`StarKind` 保留
与 iztro 兼容的细分类：`soft`、`tough`、`lucun` 或 `tianma`。iztro 仅为文昌、
文曲、火星、铃星、擎羊、陀罗提供亮度表；其他已支持辅星使用
`Brightness::Unknown`。生年四化改为通用的已表示星曜表，并包含 iztro 中落到辅星的
目标：丙文昌科、戊右弼科、己文曲忌、辛文曲科/文昌忌、壬左辅科。原有仅主星 API
保留为 wrapper，并且只对已表示主星返回四化。

公开的 `build_natal_chart_with_supported_stars` builder path 会先生成 minimal
natal chart，再安十四主星，最后安十四颗已支持辅星。`by_lunar` facade 委托给这个
supported-star builder，并要求显式提供 `birth_year_stem` 和 `birth_year_branch`。

这些 fixtures 仍**不**比较杂曜、flower/helper/adjective 子集、特征提取、规则引擎
输出、解读或叙事输出、阳历转农历、闰月行为、早晚子时变体、时间范围星曜、CLI
bindings、Python bindings 或 WebAssembly bindings。

### 默认算法本命杂曜全集

三个 `adjective_stars_full_default_*` fixtures 比较**完整**的默认算法本命
38 颗杂曜/辅助星，与 iztro 每宫的 `adjectiveStars` 对照：

- 每宫的杂曜名称；
- 每颗杂曜所在的宫位地支；
- iztro 上游星曜 `type`，原样保留（`flower`、`adjective` 或 `helper`），并映射到
  Rust 的 `StarKind`。

该全集含四颗 `flower` 星——红鸾（HongLuan）、天喜（TianXi）、天姚（TianYao）、
咸池（XianChi）；两颗 `helper` 星——解神（JieShen）与年解（NianJie）；以及
32 颗普通 `adjective` 星。安星复现 iztro 2.5.8（`getAdjectiveStar` 配合
`getLuanXiIndex`、`getMonthlyStarIndex`、`getTimelyStarIndex`、
`getDailyStarIndex`、`getHuagaiXianchiIndex`、`getYearlyStarIndex`），并从 iztro
的寅宫索引框架转换为地支偏移，按落点基准分组：

- **出生年支**：红鸾、天喜（天喜与红鸾相对）；龙池、凤阁与天哭、天虚；华盖、咸池
  （`getHuagaiXianchiIndex` 三合同族）、孤辰、寡宿、蜚廉、破碎、天德、月德、年解
  （`getYearlyStarIndex`）；以及天空（年支 + 1）。年解只表示 `getAdjectiveStar`
  输出的本命 `origin` `helper`，不实现流年/horoscope 流曜范围。
- **农历月份**：天姚、天刑；以及天巫、天月、阴煞、解神由固定月份查表决定
  （`getMonthlyStarIndex`）。
- **出生时辰地支**：台辅、封诰（`getTimelyStarIndex`）。
- **已安辅星锚定 + 农历日**（`getDailyStarIndex`）：三台由已安左辅顺数农历日偏移
  （初一 = 0）；八座由已安右弼逆数；恩光由已安文昌、天贵由已安文曲，各顺数农历日
  偏移减一。
- **出生年干**：天官、天厨、天福（`tian_fu_adj`）、截路、空亡由固定年干查表决定。
- **命/身宫锚定**：天才由命宫、天寿由身宫顺数出生年支序号；天伤居仆役宫（命宫 + 5）、
  天使居疾厄宫（命宫 + 7）。默认算法不做阴阳/性别互换（互换为中州派特有）。
- **旬空（旬中空亡）**：出生年所在甲旬的空亡地支中，阴阳属性与出生年支相同的那一颗。
  iztro 先算基础宫位索引，再在阴阳不一致时进一位；宫位索引与地支索引相差固定偶数
  （寅 = 2），故规则可直接转换到地支空间。一个聚焦表测试覆盖全部 60 甲子。

天福使用 `tian_fu_adj` 键 / `StarName::TianFuAdj`，以与主星天府（`tian_fu` /
`StarName::TianFu`）区分（两者拼音都是 “Tian Fu”）。天月同样使用 `tian_yue_adj` /
`StarName::TianYueAdj`，以与辅星天钺（`tian_yue` / `StarName::TianYue`）区分。
每颗已安杂曜派生 `StarCategory::Adjective`（`StarKind::Flower`、
`StarKind::Adjective` 与 `StarKind::Helper` 都映射到它），亮度为
`Brightness::Unknown` 且无四化，scope 为本命。
`build_natal_chart_with_supported_stars` builder 在主星与辅星之后安放该全集，
因此 `by_lunar` 现在产出 14 主星 + 14 辅星 + 38 杂曜/辅助星 = **66 颗本命星**。

该组 fixtures 仍**不**比较杂曜亮度、特征提取、规则引擎输出、叙事输出、阳历转农历、
闰月行为、早晚子时变体，或时间范围星曜（大限、流年或其他流曜范围）。

### 杂曜/辅助星覆盖

iztro 2.5.8 `getAdjectiveStar` 在默认（非中州派）算法下输出 **38** 颗本命
`origin` 杂曜。`iztro-rs` 现已安**全部 38 颗**，因此默认算法本命杂曜/辅助星覆盖
**已完成**。每颗都是本命 `origin`（`scope: origin`），且都可由 `by_lunar` 已传入的
输入推出——农历月、农历日、出生时辰、出生年干、出生年支，以及命宫/身宫地支。它们
都不需要时间范围层、阳历转农历、闰月或早晚子时变体。

其余四颗杂曜为**中州派特有**（中州派变体 `algorithm: 'zhongzhou'`），与中州派算法
选择本身一并延后：龙德 LongDe、截空 JieKong、劫煞 JieSha（杂曜）、大耗 DaHao
（杂曜）。在中州派下，这四颗取代默认的截路/空亡两颗。本默认 `getAdjectiveStar`
切片以外的神煞、流曜，以及所有时间/horoscope 安星同样延后。这四个中州派特有名称
已进入 metadata 清单，但尚未安放，也没有 fixture 覆盖。四化仍作为
`mutagen: Option<Mutagen>` 事实附于安星，而非独立星曜。

## Golden tests

Golden tests 应包括：

- 阳历排盘；
- 农历排盘；
- 闰月行为；
- 早子时与晚子时行为；
- 年分界行为；
- default algorithm 行为；
- 若支持中州派，则包含中州派行为。

## 致谢

`iztro` 使用 MIT License。若直接改写或移植部分逻辑，应在源码注释或文档中保留适当 attribution。
