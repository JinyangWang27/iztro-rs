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

`core` 现在保留两套相互独立的星曜 metadata surface：

- `represented_star_metadata_table().len() == 70` 仍保持严格边界：只覆盖当前由星盘事实
  表示、由 Rust 代码安放、并由 fixtures 校验的星。其中四颗已表示杂曜受算法门控，
  只会在 `ChartAlgorithmKind::Zhongzhou` 下出现在星盘输出中。
- `known_star_metadata_table().len() == 170` 清点更广的上游 `iztro@2.5.8` runtime 星曜
  名称宇宙，包含已表示星曜、装饰性 runtime 数组
  （`changsheng12`、`boshi12`、`suiqian12`、`jiangqian12`），以及大限、流年、流月、
  流日、流时的 horoscope 流曜名称。

`represented_star_metadata_table()` 保持**仅本命（70）**。现在还有两类 runtime
surface 已安放，但不会改变这张表：

- 四组装饰性 runtime 家族（长生/博士/岁前/将前十二神）作为**无类型**
  `DecorativeStarPlacement` 安放。它们在上游没有 `FunctionalStar` type，因此没有
  `StarKind`，永远不是有类型的 `StarPlacement`，也永远不会出现在 `Chart::stars()`
  中。
- horoscope 流曜（`Yun*`/`Liu*`/`Yue*`/`Ri*`/`Shi*`）作为**有类型、带地支标签**的
  `ScopedStarPlacement` 安放在 `TemporalLayer` 内。它们带有 known inventory 中的
  具体 `StarKind`，但因为属于时间层，仍在仅本命的 `represented_star_metadata_table()`
  之外：它们是已知、有类型、但属时间范围的事实。

Known metadata 仍不表示已支持亮度表或完整 horoscope 宫名推导。见
[Runtime 星曜家族安放](#runtime-星曜家族安放)。

上游 locale key `xunzhong` / `旬中` 被有意排除，因为在 `iztro@2.5.8` 中没有找到
内置的 `FunctionalStar` 构造或 `StarType` 分配。四化仍作为 `Mutagen` /
`MutagenActivation` 事实存在，而不是 `StarName` variants。

## 天地人三盘（Chart planes）

`ChartPlane` 与 `ChartAlgorithmKind` 是相互独立的两个维度。默认值为 `Heaven`
（天盘），其输出与既有星盘生成逐值一致。

中州派（Zhongzhou）额外支持地盘（Earth）与人盘（Human）。二者通过**带锚点的最小星盘
重建**生成，而不是对已完成的 `Chart` 做原地修改：

- `Zhongzhou + Earth`：将命宫重新锚定到天盘的身宫地支；
- `Zhongzhou + Human`：将命宫重新锚定到天盘的福德宫（`PalaceName::Spirit`）地支。

重新锚定后，宫名、宫干与五行局都依据新的命宫重新推导，而身宫地支仍保留其原始计算
值。随后运行既有的确定性安星策略，因此各安星器从不针对 `ChartPlane` 分支。盘别分派
集中在 `by_lunar` facade 边界（`resolve_natal_chart_anchor`）。中州天盘**不等于**全书
（QuanShu）算法，它只是中州派的天盘。

对任何非中州派族（QuanShu / Placeholder）请求 `Earth` 或 `Human` 都会返回
`ChartError::UnsupportedChartPlane`。

## 公开 facade 兼容性

`by_lunar` 与 `by_solar` 是 `iztro-rs` 的 iztro-compatible facade 入口。它们在概念
上对应 iztro 的 `astro.byLunar(...)` 与 `astro.bySolar(...)`，但使用强类型的
`LunarChartRequest` 与 `SolarChartRequest` 请求对象，而不是 JavaScript 风格的位置
参数。

`by_lunar` 把传入的农历日期记录为星盘输入事实，并委托给已支持星曜的本命盘
builder。它现在通过 `is_leap_month` 与 `fix_leap` 携带显式的闰月语义（builder 默认
分别为 `false` 与 `true`，保持原有非闰月行为）。请求的 `is_leap_month` 会先通过内部 
lunar-lite 历法 normalizer按真实历法解析；公开 API 不暴露 calendar-adapter类型。
只有当请求的月份确实是该年的闰月时，闰月标志才被采纳，复现上游 `lunar2solar`。
无效的闰月请求——例如 `2020-3-20` 且 `is_leap_month=true`，而 2020 年的闰月是四月而非
三月——会按普通月份处理。解析之后，真实闰月的后半月（农历日 > 15）且 `fix_leap` 时，
用于月份相关安星的有效月份加一，复现上游 `iztro@2.5.8` `fixLunarMonthIndex`；但出生
时间为晚子时（`timeIndex = 12`）时，上游不会推进有效月份。闰十二月会以
`ChartError::UnsupportedLeapMonthCombination` 拒绝而非猜测。出生年干和年支仍需显式传给
`by_lunar`，因为 `by_lunar` 本身不做农历年到干支年的推导。

出生时间由 `BirthTime` 表示，对应上游 `iztro` 的 `timeIndex` `0..=12`。早子时
（`0`）与晚子时（`12`）都投影到 `EarthlyBranch::Zi`；原有按地支传入的 request API
仍将 `Zi` 映射为早子时，以保持向后兼容。晚子时已用 `iztro@2.5.8` fixture 校验：按时辰
的月份/时系公式会像子时一样 wrap，主星安放使用下一农历日，日系杂曜使用上游
`fixLunarDayIndex`，且闰月后半月的有效月份推进会因晚子时而关闭。

`by_solar` 是同一已支持切片之上的适配层：它校验公历日期，通过内部 `lunar-lite`
适配器将其转换为农历事实，并用 `lunar-lite` 1.0.0 的
`four_pillars_from_solar_date_with_options`（`YearDivide::Normal` /
`MonthDivide::Normal`）推导事实性的本命四柱。它依据转换结果设置 `is_leap_month`、
依据请求设置 `fix_leap`，委托 `by_lunar` 安星，并把 `lunar_lite::FourPillars`
保留在 `Chart::four_pillars()` 上；它自身不新增安星逻辑。显式不变量是：只要
`Chart::four_pillars()` 存在，其年柱必须等于 `Chart::birth_year()`。
`by_lunar` 保持保守：它只接收显式出生年干支，目前不从农历输入伪造月柱、日柱或时柱，
所以 `by_lunar` 星盘的 `Chart::four_pillars()` 为 `None`。未来 PR 可决定
`by_lunar` 是否接受显式 `FourPillars`，或是否通过规范化阳历日期推导。
`lunar-lite` 拥有底层天干/地支、六十甲子与四柱原语（`HeavenlyStem`、
`EarthlyBranch`、`StemBranch`、`FourPillars`），并由 `core` 模块直接 re-export；
仅日历后端日期类型（`SolarDate`、`LunarError`）不出现在公开 API。转换以正月初一为
年界，与 iztro 默认的 `yearDivide: 'normal'` 一致，因此换算出的年干支即便落在
立春/正月初一之间的窗口也与上游一致。

事实性 `by_solar` 本命四柱之外的完整八字解读/输出、完整 facade 序列化对齐、bindings、特征提取、规则与叙事仍延期实现。`build_full_horoscope_chart`
已将大限、小限、流年、流月、流日、流时层组装为一个 `HoroscopeChart`，并保留组装所用的
数字化目标阳历/农历/时辰 context，但仍仅是已支持事实面的模型级组装，并非上游
`FunctionalAstrolabe#horoscope` 载荷形状。流年层现已附带
`yearlyDecStar`（岁前/将前十二神），作为流年范围的时间性装饰事实。
`HoroscopeSupportedFieldsSnapshot` 现在可从 `HoroscopeChart` 导出规范化的 supported-fields
快照，用于和 `crates/iztro/fixtures/iztro/horoscope.json` 中已实现的大限、小限、流年、
流月、流日、流时事实面做确定性兼容校验。该 DTO 使用 snake_case 字段和寅宫起序的宫名数组，
不包含原始中文标签、runtime 宫位投影或 facade 级本命 astrolabe，也不表示完整上游 facade JSON 对齐。
`HoroscopeRuntime` 现在提供已类型化的上游 runtime helper 切片：`age_palace`、`palace`、
`surround_palaces`、`has_horoscope_stars`、`not_have_horoscope_stars`、
`has_one_of_horoscope_stars` 与 `has_horoscope_mutagen`，并以
`crates/iztro/fixtures/iztro/horoscope_runtime.json` 对齐 `iztro@2.5.8`。
这些 helper 只查询/投影已有模型事实，不修改本命盘、不复制本命星曜到时间层，也不改变安星语义。

`HoroscopeFacadeSnapshot` 是可序列化的 facade/导出层，而非新的引擎层。
`HoroscopeFacadeSnapshot::from_horoscope_chart` 把已建模的事实组合成一个确定性载荷，
更接近上游 `FunctionalAstrolabe#horoscope` 形状：它原样复用 `HoroscopeSupportedFieldsSnapshot`
的大限/小限/流年/流月/流日/流时分块（扁平化到顶层），从 `HoroscopeChart::natal()` 嵌入
最小 `NatalFacadeSnapshot` 作为 `astrolabe`，在 `build_full_horoscope_chart`
构建的星盘上加入保留的数字化目标 `context`（阳历日期、农历日期、闰月标志与目标
`timeIndex`），并复用 `HoroscopeRuntime` 生成 `age_palace`、`palace_projections` 与
`surround_palaces` 的命宫投影——每个投影都保持本命与时间性事实分离（本命宫名/宫干/星曜与
该期的时间性宫名、时间性星曜、时间性四化分开）。最小 `astrolabe` 只包含已由 Rust 建模的
本命事实：性别、出生年干支、五行局、命/身宫地支、十二宫、宫位地支/宫名/宫干/角色、本命
有类型星曜和本命装饰星曜。手工组装且没有 target context 的
`HoroscopeChart` 仍可使用旧有的农历 year/month/day fallback，并省略阳历日期与目标
`timeIndex`。它以 `horoscope_facade.json` 校验，不新增任何安星逻辑，并明确标注延期字段：
上游本地化 `lunarDate` 与 `solarDate` 字符串、完整上游 astrolabe helper/query 方法、
本命本地化标签、八字字符串、大限 ranges、ages 数组、runtime 查询助手以及完整上游 package
对齐仍然延期。它建立在 `HoroscopeChart`、`HoroscopeSupportedFieldsSnapshot`、
`NatalFacadeSnapshot` 与 `HoroscopeRuntime` 之上，更接近上游
`FunctionalAstrolabe#horoscope` 载荷形状，但仍**不是**完整 package 对齐。

## 本地化标签

Rust 内部领域模型保持语言中立：天干、地支、宫位、星曜、四化、亮度、星类与十二神
族仍是强类型枚举，并以稳定的机器可读 key 序列化。facade/导出层的本命 astrolabe
快照额外以**附加**的 `*_zh` 字段暴露常用中文标签（例如 `branch`/`branch_zh`、
`name`/`name_zh`、`stem`/`stem_zh`），因为紫微斗数主要以中文消费。

这些标签由确定性、表驱动的 `core::labels::zh_cn` 查表生成，绝不替换规范身份，因此
兼容性断言仍只校验机器可读字段。完整多语言/i18n 基础设施与完整上游本地化字符串
（含八字）对齐仍然延期。

## 运限层模型

`core` 定义了仅模型的运限叠加层：`HoroscopeChart` 包裹不可变的本命 `Chart`，
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

scoped flow-star builder（`build_flow_star_layer`）用于安放一个时间范围的 horoscope
流曜。给定 `TemporalContext`，它会返回该 scope 的 `TemporalLayer`，其中包含带地支
标签的 `ScopedStarPlacement`。上面的四化 builder 仍不安放流曜；流曜安放由这个独立
builder 负责。大限/流年推导、年份到干支转换和历法推导仍不在范围内。

`build_monthly_period` 现在从本命事实、目标阳历日期和目标 `BirthTime` 推导一个流月
period。它对齐上游 `FunctionalAstrolabe#horoscope`：流月 `StemBranch` 来自目标日期的
normal 分界月柱；流月命宫索引则另由目标流年地支、本命农历月、本命出生时支和目标农历月
推导。两者是独立事实，不能把流月干支的地支当作流月命宫。`build_monthly_horoscope_layer`
会把该 period 组装为 `Scope::Monthly` 的 `TemporalLayer`，包含流月流曜、流月四化激活和
流月 `TemporalPalaceLayout`。它不组装流日/流时层，也不附加时间性 decorative arrays。

`build_daily_period` 从本命事实、目标阳历日期和目标 `BirthTime` 推导一个流日 period。
它对齐上游 `FunctionalAstrolabe#horoscope`：流日 `StemBranch` 来自目标日期的 normal 分界
日柱；流日命宫索引则另由流月命宫索引顺数目标农历日推导。两者是独立事实，不能把流日
干支的地支当作流日命宫。`build_daily_horoscope_layer` 会把该 period 组装为 `Scope::Daily`
的 `TemporalLayer`，包含流日流曜、流日四化激活和流日 `TemporalPalaceLayout`。它不组装
完整 horoscope，也不附加时间性 decorative arrays。

`build_hourly_period` 从本命事实、目标阳历日期和目标 `BirthTime` 推导一个流时 period。
它对齐上游 `FunctionalAstrolabe#horoscope`：流时 `StemBranch` 来自目标日期/时辰的 normal
分界时柱；流时命宫索引则另由流日命宫索引顺数目标时辰推导。两者是独立事实，不能把流时
干支的地支当作流时命宫。`build_hourly_horoscope_layer` 会把该 period 组装为 `Scope::Hourly`
的 `TemporalLayer`，包含流时流曜、流时四化激活和流时 `TemporalPalaceLayout`。它不附加时间性
decorative arrays。

`build_full_horoscope_chart` 把大限、小限、流年、流月、流日、流时层按确定顺序（大限 → 小限 →
流年 → 流月 → 流日 → 流时）组装为一个 `HoroscopeChart`（输入 `HoroscopeStackInput`）。它推导
目标农历日期与虚岁（目标农历年 − 本命农历年 + 1），并按虚岁选取覆盖的大限 period（不写死索引）。
它还会在 `HoroscopeChart` 上保留组装所用的目标 context：数字化目标阳历日期、数字化目标农历日期、
`lunar-lite` 返回的目标闰月标志，以及上游目标 `timeIndex`。流年层还会附带 `yearlyDecStar`，
作为流年范围的时间性装饰事实。这是已支持字段的模型级组装：不复刻上游
`FunctionalAstrolabe#horoscope` 载荷形状。

`HoroscopeSupportedFieldsSnapshot` 是独立的兼容性导出 DTO，而不是 renderer model。它从
`HoroscopeChart` 和各 `TemporalLayer` 导出已实现的 supported fields：各 scope 的 index、
干支、寅宫起序宫名、四化目标、已实现的流曜、小限虚岁、流年年解地支，以及流年
`yearlyDecStar`。渲染仍应使用 `ChartStackSnapshot`；fixture 兼容校验可使用
`HoroscopeSupportedFieldsSnapshot`。

`HoroscopeRuntime` 是模型级 runtime helper facade。`age_palace`、`palace` 与
`surround_palaces` 以地支为空间 identity 做投影：本命宫名、宫干和本命星曜仍保留，
时间宫名只是额外标签，不会覆盖本命事实。查询 helper 与上游 `iztro@2.5.8` 行为对齐：
星曜查询检查大限与流年流曜矩阵的合并结果，`has_horoscope_mutagen` 检查所选 scope 的四化目标
是否落在投影后的本命宫位中。它们都不生成新落点、不更改本命盘，也不代表完整 facade payload。

### Facade/导出星曜排序

核心引擎的落点事实是**与顺序无关的**。一个宫位就是落入其中的星曜*集合*，
因此核心落点兼容性测试比较的是星曜集合，而非数组顺序——Rust 与上游 TS `iztro`
不一定以相同的 `Vec` 顺序输出某宫位的星曜，这种偶然顺序没有语义含义。

facade/导出层不应依赖该偶然顺序。因此 `NatalFacadePalaceSnapshot` 对每个导出宫位的
星曜数组施加一个稳定、确定性的 Rust 侧排序：

- 有类型本命星按 `(kind, name, brightness, mutagen)` 排序；
- 装饰性本命星按 `(family, name)` 排序。

排序键使用 `StarKind`、`StarName`、`Brightness`、`Mutagen` 与 `DecorativeStarFamily`
按声明顺序派生的 `Ord`；这些派生仅作排序键，不含任何星曜强弱排名含义。重复构造同一
facade 快照在字节层面一致，该策略由 `facade_star_ordering.rs` 固定。

这是 Rust 侧的规范顺序，**并非**对上游 TS `iztro` 宫位星曜数组顺序对齐的声明，
后者仍被推迟。

## Runtime 星曜家族安放

有类型星曜和装饰性 runtime 条目是**分离的事实 surface**，`Chart::stars()` 只返回
有类型的 `StarPlacement`。

**装饰性家族。** 四组「十二神」家族（长生/博士/岁前/将前十二神）在上游以无
`StarKind` 的裸名称输出，因此建模为无类型的 `DecorativeStarPlacement`
（`name` + `DecorativeStarFamily` + `Scope`），而不是伪装成有类型的
`StarPlacement`。`DecorativeStarPlacement::try_new` 会按 known inventory 校验每个
条目（家族匹配，且没有 `StarKind`）。这些事实保存在独立的
`Palace::decorative_stars()` collection 中（为空时 serde 跳过），并通过
`Chart::decorative_stars()` / `Chart::decorative_star()` 读取。supported-star 本命
builder——因此也包括 `by_lunar`——会安放全部四组家族：长生从五行局地支起，博士从
禄存起，二者均按阳男阴女顺行；岁前和将前分别从出生年支 / 三合锚点顺行。岁破
（`SuiPo`）是已知的岁前名称，但不会作为第十三个岁前条目额外安放，因为上游 runtime
placement 只输出 12 个岁前条目；在 `ChartAlgorithmKind::Zhongzhou` 下，岁破占用第
七个岁前位置并替代大耗（`SuiPo` 替代 `DaHaoSuiqian`），其他算法下岁破 known 但不
placed。由于装饰性条目是独立事实，默认/非中州派 `Chart::stars()` 仍为 **66** 个有
类型的本命 `StarPlacement`，中州派 `Chart::stars()` 仍为 **68** 个有类型的本命
`StarPlacement`。

**中州派本命杂曜**（`LongDeAdj`、`JieKong`、`JieShaAdj`、`DaHaoAdj`）仍是有类型的
`StarPlacement`，只在 `ChartAlgorithmKind::Zhongzhou` 下安放，因此仍属于 represented
（本命）表。

**流曜。** 流曜安放通过规范化的 `FlowStarScope` + `FlowStarBase` identity 实现：
`flow_star_name(scope, base)` 返回具体 `StarName`，`build_flow_star_layer` 使用同一套
scope-generic 算法为大限、流年、流月、流日、流时安放十颗 matrix 星
（魁钺昌曲禄羊陀马鸾喜），落点来自该 period 的干支。流文昌/流文曲使用基于天干的
规则（不同于本命的时辰规则）。流年 scope 还会安放年解（`NianJieYearly`），它有意
保持在 `FlowStarBase` 之外。大限、小限、流月、流日、流时 layer 现在都会附带 fixture-backed
的宫名布局。流曜安放本身仍是地支层面的。

**流年装饰性数组（`yearlyDecStar`）。** 流年的岁前/将前十二神建模为流年范围的时间性
装饰事实。`build_yearly_decorative_star_placements` 复用本命岁前/将前规则，但锚定在流年
地支上，输出 `Scope::Yearly`、按地支标记的 `ScopedDecorativeStarPlacement`。它们是无类型
装饰事实：**不是**有类型星曜，永不出现在 `Chart::stars()`，也与本命
`Palace::decorative_stars()` 分离。`build_yearly_horoscope_layer`（因此也包括
`build_full_horoscope_chart`）会把它们附加到流年 `TemporalLayer`，通过
`TemporalLayer::temporal_decorative_stars()` 读取；快照在流年层通过
`PalaceLayerCellSnapshot::temporal_decorative_stars()` 暴露，与本命装饰事实分离。

四化仍是 `Mutagen` / `MutagenActivation` 事实，永远不是 `StarName` variants。
最小 `by_solar`（`lunar-lite` 支持的阳历转农历）、已支持 `by_lunar`/`by_solar` 切片的
fixture 支持闰月行为，以及 `BirthTime`/`timeIndex` `0..=12` 早晚子时变体现已实现
（见[公开 facade 兼容性](#公开-facade-兼容性)）。完整八字输出、完整 facade 载荷对齐、bindings、特征提取、规则与叙事仍然延期。完整 horoscope stack 组装现已实现（`build_full_horoscope_chart`），会保留数字化目标阳历/农历/时辰 context，流年层并附带 `yearlyDecStar`，可通过 `HoroscopeSupportedFieldsSnapshot` 导出规范化 supported-fields 快照，通过 `HoroscopeRuntime` 使用已类型化的 runtime helper，并可通过 `HoroscopeFacadeSnapshot` 把它们和最小本命 `astrolabe` 组合为一个上游风格、可序列化的 horoscope 载荷（更接近 `FunctionalAstrolabe#horoscope` 形状，但仍非完整 package 对齐）；这些仍仅覆盖已支持事实面。

## 当前 fixtures

fixtures 为：

- `crates/iztro/fixtures/iztro/minimal_natal_1990_05_17_chen_female.json`
- `crates/iztro/fixtures/iztro/major_stars_1990_05_17_chen_female.json`
- `crates/iztro/fixtures/iztro/minor_stars_1990_05_17_chen_female.json`
- `crates/iztro/fixtures/iztro/minor_stars_1988_03_14_zi_male.json`
- `crates/iztro/fixtures/iztro/minor_stars_1991_08_09_hai_female.json`
- `crates/iztro/fixtures/iztro/adjective_stars_full_default_1990_05_17_chen_female.json`
- `crates/iztro/fixtures/iztro/adjective_stars_full_default_1988_03_14_zi_male.json`
- `crates/iztro/fixtures/iztro/adjective_stars_full_default_1991_08_09_hai_female.json`
- `crates/iztro/fixtures/iztro/zhongzhou_adjective_stars_1990_05_17_chen_female.json`
- `crates/iztro/fixtures/iztro/zhongzhou_adjective_stars_1988_03_14_zi_male.json`
- `crates/iztro/fixtures/iztro/zhongzhou_adjective_stars_1991_08_09_hai_female.json`
- `crates/iztro/fixtures/iztro/runtime_decorative_default_1990_05_17_chen_female.json`
- `crates/iztro/fixtures/iztro/runtime_decorative_default_1988_03_14_zi_male.json`
- `crates/iztro/fixtures/iztro/runtime_decorative_default_1991_08_09_hai_female.json`
- `crates/iztro/fixtures/iztro/runtime_decorative_zhongzhou_1990_05_17_chen_female.json`
- `crates/iztro/fixtures/iztro/runtime_decorative_zhongzhou_1988_03_14_zi_male.json`
- `crates/iztro/fixtures/iztro/runtime_decorative_zhongzhou_1991_08_09_hai_female.json`
- `crates/iztro/fixtures/iztro/flow_stars.json`
- `crates/iztro/fixtures/iztro/horoscope.json`
- `crates/iztro/fixtures/iztro/horoscope_runtime.json`
- `crates/iztro/fixtures/iztro/horoscope_facade.json`
- `crates/iztro/fixtures/iztro/e2e_supported_by_lunar.json`
- `crates/iztro/fixtures/iztro/e2e_supported_by_solar.json`
- `crates/iztro/fixtures/iztro/leap_month_by_lunar.json`
- `crates/iztro/fixtures/iztro/time_index_rat_hour.json`

`e2e_supported_by_solar.json` 覆盖已支持 `by_solar` 切片的七个阳历用例（两种算法，
共十四例），含农历新年分界、普通日期、转换为闰月的日期、闰月之后的日期，以及同一个
闰月后半日期在 `fix_leap=true` 与 `fix_leap=false` 下的两种结果（月份相关安星不同）；
每个用例附带 `converted_lunar`（农历年/月/日、闰月标志、出生年干支），便于诊断历法
偏差。`leap_month_by_lunar.json` 用 2020 闰四月的真实农历日期，覆盖 `is_leap_month`
与 `fix_leap` 的组合：闰月前、与闰月同号的常规月、闰月前后半月，以及闰月之后；其中
闰四月、农历日 > 15 的 `fix_leap` true/false 对是有效月份进位的判别用例。它还覆盖
**无效**闰月请求（`is_leap_month=true` 但该月并非当年闰月——2020 年三月与五月，以及
普通的 2021 年某月）：上游会忽略该标志，每个用例记录上游 `resolved_lunar`，因此
Rust 测试断言相同的解析结果，而不仅仅回显输入标志。`time_index_rat_hour.json` 覆盖
上游 `iztro` `timeIndex` `0..=12` 行为：早子时（`0`）、晚子时（`12`）、一个普通非子时
时辰，以及证明晚子时不会推进有效月份的真实闰月后半月用例。这些 facade fixtures 均为
supported-field-only，排除流曜（仅依赖年干支，已由 `e2e_supported_by_lunar.json`
覆盖）、完整 facade 序列化对齐、horoscope 宫名推导、temporal decorative arrays、特征、
规则与叙事。`by_solar` 的转换由内部 `lunar-lite` 支持，日历后端类型不出现在公开 API 中。
重新生成：

```bash
npm ci --prefix tools/iztro-reference
npm run dump:e2e-supported-by-solar --prefix tools/iztro-reference -- --write
npm run dump:leap-month --prefix tools/iztro-reference -- --write
npm run dump:time-index-rat-hour --prefix tools/iztro-reference -- --write
```

`runtime_decorative_*` fixtures 覆盖默认与中州派下每宫的四组装饰性家族；
`flow_stars.json` 覆盖所有 scope、十天干和十二地支组合下的 scoped flow stars。见
[Runtime 星曜家族安放](#runtime-星曜家族安放)。

仅保留当前完整默认算法杂曜 fixtures（每个 38 颗星）和中州派杂曜 fixtures（每个
40 颗星）在源码树中；更早、更小的杂曜子集可通过 git history 获取。

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

minimal-natal fixture 通过 fixture input 中的 `birth_year_stem` 显式提供出生年干，
因为该 fixture 走 `by_lunar` / builder 路径；阳历路径的出生年干支与完整本命四柱由
`by_solar` 通过 `lunar-lite` 推导。

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
  输出的本命 `origin` `helper`；流年年解是独立的 `NianJieYearly` 时间层落点。
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

### 中州派本命杂曜

三个 `zhongzhou_adjective_stars_*` fixtures 将 `ChartAlgorithmKind::Zhongzhou`
的本命杂曜输出与 iztro 2.5.8 中州派 `getAdjectiveStar` 行为对照。中州派输出保留
默认算法共通的本命杂曜/辅助星，不安放默认的截路（`JieLu`）与空亡（`KongWang`）
两颗，并新增四颗中州派特有本命杂曜：龙德（`LongDeAdj`）、截空（`JieKong`）、
劫杀（`JieShaAdj`）和大耗（`DaHaoAdj`）。同时复现 iztro
`getTianshiTianshangIndex`，包括适用时中州派特有的天伤/天使阴阳性别互换。

在中州派下，`by_lunar` / `build_natal_chart_with_supported_stars` 现在产出
14 主星 + 14 辅星 + 40 杂曜/辅助星 = **68 颗本命星**。默认与
placeholder/非中州派 profile 输出保持不变，仍为 **66 颗本命星**。已表示 metadata
table 同时包含默认专属与中州派专属的算法门控本命杂曜，因此总数为 **70**：
14 主星 + 14 辅星 + 42 颗本命杂曜/辅助星。

该组 fixtures 仍**不**比较装饰性 runtime 数组、杂曜亮度、特征提取、规则引擎输出、
叙事输出、阳历转农历、闰月行为、早晚子时变体、horoscope 安星或时间范围星曜。
四化仍作为 `Mutagen` / `MutagenActivation` 事实存在，而不是 `StarName` variants。

### 杂曜/辅助星覆盖

iztro 2.5.8 `getAdjectiveStar` 在默认（非中州派）算法下输出 **38** 颗本命
`origin` 杂曜。`iztro-rs` 现已安**全部 38 颗**，因此默认算法本命杂曜/辅助星覆盖
**已完成**。每颗都是本命 `origin`（`scope: origin`），且都可由 `by_lunar` 已传入的
输入推出——农历月、农历日、出生时辰、出生年干、出生年支，以及命宫/身宫地支。它们
都不需要时间范围层、阳历转农历、闰月或早晚子时变体。

中州派变体 `algorithm: 'zhongzhou'` 现在已支持四颗中州派特有本命杂曜。完整 horoscope
组装与流年 yearly-scope 装饰性数组（`yearlyDecStar`）现已实现；本已支持的
`getAdjectiveStar` 切片以外的其余神煞仍然延期；scoped 流曜安放已由独立 fixture 覆盖。
四化仍作为 `mutagen: Option<Mutagen>` 事实附于安星，而非独立星曜。

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
