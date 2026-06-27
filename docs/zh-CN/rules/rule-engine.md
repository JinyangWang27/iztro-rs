# 经典规则引擎

本文档介绍 `crates/iztro/src/rules/classical/` 下的**经典规则引擎**。它是当前
启用的规则引擎：将《紫微斗数全书》等典籍中的规则编码为数据，并把图盘事实转化为
结构化、带证据的**判断（Claim）**。

`rules::classical` 是规则引擎的权威命名空间。`crates/iztro/src/rules/` 模块直接
暴露它（`pub mod classical`）并再导出经典类型/函数，因此 `rules::Claim` 等指向
经典判断模型。

## 流水线

```
图盘事实
  -> 特征/查询谓词        （复用 core/pattern 查询助手）
  -> 经典规则评估          （语料元数据 + 手写谓词）
  -> ClassicalSourceHit[]  （命中的出处/来源记录）
  -> Claim[]               （仅当 rule.claim 存在时产出）
  -> RuleDiagnostic[]      （类型化、可见的不支持条件）
  -> [可选] 本地化渲染      （iztro-i18n，经由 claim_key）
  -> JSON 导出             （serde）
```

## 各自的权威来源

引擎刻意将四种表示分离：

| 关注点 | 权威来源 | 说明 |
| --- | --- | --- |
| 经典术语 | **中文原文** | `SourceRef::source_text_zh_hans`，在语料 TOML 中编写。 |
| 机器逻辑 | **Rust 枚举 / 稳定键** | `ClassicalRuleId`、`ClaimDomain`、`ClaimTheme`……中文字符串绝不作逻辑键。 |
| 输出渲染 | **Fluent `.ftl` 资源** | `iztro-i18n` 由稳定键渲染标签与判断短文。 |
| 规则编写 | **语料 TOML** | `crates/iztro/rule-corpus/quan-shu/rules.toml` 与 `crates/iztro/rule-corpus/patterns/rules.toml`。 |
| 导出 | **JSON** | 判断的序列化结果；绝非编写来源。 |

`iztro` 永不依赖 `iztro-i18n`：核心 crate 只产出稳定键与结构化事实；本地化文案
只存在于 Fluent 资源中。

## 混合设计（元数据 + 谓词）

目前刻意**不**做通用规则 DSL：

1. **规则出处/谓词元数据数据驱动**，来自语料 TOML（id、出处、状态、典籍、流派）。
   可选 `[rule.claim]` 保存解释性判断字段（领域、主题、吉凶、基础强度、claim 键）。
2. **规则谓词手写**于 `predicates.rs`，复用 `core/pattern/` 中只读的图盘查询
   助手（夹宫匹配、亮度判定、星曜查找），不重复该逻辑。
3. 评估器将每条规则的元数据与谓词配对，先构建 `ClassicalSourceHit`；只有
   `rule.claim` 存在时才进一步构建 `Claim`。

## 保守触发与类型化诊断

只有当可执行规则的条件在**已建模的图盘事实**上匹配时，才会产出出处命中记录。
判断只在同一次命中且规则带有 `ClaimSpec` 时产出。每个评估器返回类型化的
`RuleOutcome`：

- `Matched { source_hit, claim }`——事实已建模且条件匹配；
- `NotApplicable`——事实已建模但条件未匹配（无判断）；
- `Unsupported(UnsupportedReason)`——规则已编码，但其条件尚未由已建模的事实/
  既定策略支撑。

引擎提供两个入口：

- `evaluate_classical_claims(chart, &request) -> Vec<Claim>`——仅返回判断；
- `evaluate_classical(chart, &request) -> ClaimEvaluation { claims, source_hits, diagnostics }`
  ——同时返回命中的 `ClassicalSourceHit` 与类型化的 `RuleDiagnostic`，使“不支持”的条件
  **可见**，而非被静默丢弃。

`SourceRef` 仍是 `Claim` 内面向判断的引用类型。`ClassicalSourceHit` 是评估结果中
面向出处/来源命中的记录。

## 请求过滤与排序

`ClaimEvaluationRequest` 按 `domains`、`themes`、`polarities`、`works`、
`rule_ids`、`scopes` 过滤判断。每个字段都是允许列表；空向量表示该维度不施加约束。

出处命中只按来源维度过滤：`works`、`rule_ids`、`scopes`。`domains`、`themes`、
`polarities` 仍只过滤判断，不会压掉已经命中的来源记录。

不支持诊断默认使用 `DiagnosticMode::AllUnsupported`：即使判断过滤器已生效，也返回
所有不支持的语料规则诊断，使诊断通道保持完整。若调用方需要更窄的 UI/导出表面，
可选 `DiagnosticMode::MatchingRequest`（尽可能按规则元数据套用请求过滤器）。其中
领域/主题/吉凶诊断过滤只匹配带有 `rule.claim` 的规则；纯出处规则不会伪造解释性
元数据。`DiagnosticMode::None` 会抑制诊断。

返回的判断按 `(scope, domain, rule_id, claim_key)` 确定性排序。
返回的出处命中按 `(scope, work, source_id, source_clause_id, rule_id)` 确定性排序。

## 面向渲染层的规则面板

`evaluate_classical` 是底层评估 API。面向 GUI/渲染层的封装是
`classical_rule_panel_view(chart, &ClassicalRulePanelRequest)`，它运行一次
`evaluate_classical`，并把结果与可选的语料规则元数据组装成单个
`ClassicalRulePanelView` 读模型。

该面板**仍然保持**既有的拆分，而不是把它们合并：`claims`、`source_hits`、
`diagnostics`、`corpus_rules` 各为独立向量。解释性判断与命中出处不会被合并成
单一卡片模型，因此命中但无 claim 元数据的规则仍通过 `source_hits` 出现；
`corpus_rules` 只是用于展示/筛选的元数据，并非评估输出。

`ClassicalRulePanelRequest::user_facing()` 隐藏未支持诊断
（`DiagnosticMode::None`）；`developer()` 则展示（`DiagnosticMode::AllUnsupported`）。
语料规则可按状态筛选（`with_corpus_statuses`）或整体省略（`without_corpus`），
并按 `(work, source_id, source_clause_id, rule_id)` 确定性排序。

与别处一致，`iztro` 在此不输出本地化长文本：面板只携带 `claim_key`、类型化字段
和中文原文出处；本地化渲染仍由 `iztro-i18n` 负责。

## 面向上下文的评估

除了仅本命的 `evaluate_classical(chart, &request)`，引擎还提供面向上下文的入口：

```rust
evaluate_classical_in_context(&ClassicalRuleContext, &request) -> ClaimEvaluation
```

`ClassicalRuleContext` 对应 `core::pattern::PatternContext`：携带本命 `chart`、
可选的 `&HoroscopeChart` 以及规则可检视的 `active_scopes`；构造函数为
`ClassicalRuleContext::natal(chart)` 与
`ClassicalRuleContext::horoscope(chart, active_scopes)`。
`evaluate_classical(chart, &request)` 只是对上下文 API 的仅本命薄封装，
`classical_rule_panel_view` 亦封装 `classical_rule_panel_view_in_context`，
因此既有调用点保持不变。

当前可执行规则仍只匹配本命事实，所以横盘上下文目前与本命上下文结果相同。
该上下文的存在是为了让未来的运限规则可在不改变 API 的前提下检视上层叠加。

## 分层分析（`analysis`）

`analysis` 模块是一个轻量协调层，组合格局与全书规则两个引擎，提供**可缓存的
逐层**检测。它位于 `core` 之外（`core` 不得依赖 `rules`），用于支撑未来 GUI 的
两个侧栏标签——全书规则与格局——而无需急切计算所有叠加层，也不产出庞大的分组
文本负载。

关键类型：

- `AnalysisLayerKey`——标识一个可缓存层（`Natal`、`Decadal`、`Age`、`Yearly`、
  `Monthly`、`Daily`、`Hourly`），携带定位该层所需的时间索引。`scope()`、
  `claim_scope()`、`pattern_scope()` 将其映射到既有的 `Scope` / `ClaimScope` /
  `PatternScope`。
- `analysis_layers_for_selection(selection)`——把
  `StaticTemporalNavigationSelection` 展开为它所呈现的祖先层链。选中某一年时会
  **同时**包含 `Age`（小限）与 `Yearly`（流年），二者是不同的作用范围。
- `detect_analysis_layer(&ctx, key, &request) -> AnalysisLayerResult`——在
  `TemporalAnalysisContext { natal, horoscope }` 上分析恰好一层。它只把底层
  全书/格局请求的**作用范围**改写为 `key`（其余过滤条件——尤其是 `works`——
  均沿用调用方的请求），返回紧凑的 `rule_hits: Vec<ClassicalRuleHitRef>`
  与 `pattern_hits: Vec<PatternDetection>`。`TemporalAnalysisContext` 必须与
  `key` 对应：`key` 用于缓存标识与作用范围归属，当前**不会**针对横盘已选叠加做
  校验，因此保持上下文与 `key` 一致是调用方的责任。
- `AnalysisLayerRequest::user_facing()` 把全书规则流限制为
  `ClassicalWork::ZiWeiDouShuQuanShu`。由于 GUI 将全书规则与格局放在**分开**的
  标签页，分析的规则命中流不得包含项目格局目录规则
  （`ClassicalWork::IztroPatternCatalog`）——它们应通过格局流呈现。未来的全书规则
  标签页应消费这些经全书过滤的规则命中；`classical_rule_metadata` 保持与 work
  无关，可解析任意规则 id（含格局目录条目）的元数据。
- `ClassicalRuleHitRef`——紧凑命中（`rule_id`、`scope`、`claim_key`、`evidence`），
  刻意**不含** `source_text_zh_hans`；渲染层通过
  `classical_rule_metadata(rule_id) -> Option<&'static ClassicalRuleMetadata>`
  按规则一次性解析原文。`ClassicalRuleMetadata::source_text_zh_hans` 是逐字原文，
  绝不放入解读或判断文本。当前可执行规则的 `applicable_scopes = &[ClaimScope::Natal]`；
  全书 / 太微赋规则不会被自动推广到所有运限范围。

**层归属与缓存。** 某层的检测可以**检视**上层叠加，但返回的命中始终归属被请求的
层。`detect_analysis_layer` 不计算祖先层；调用方分别请求缺失的祖先层，并按
`AnalysisLayerKey` 缓存各层结果。未来的跨层规则（如 流年化忌冲照本命命宫，本次
未实现）须将命中归到**最深**的触发层：

| 交互 | 归属层 |
| --- | --- |
| 本命 + 流年 | 流年（Yearly） |
| 大限 + 流年 | 流年（Yearly） |
| 流年 + 流月 | 流月（Monthly） |
| 流月 + 流日 | 流日（Daily） |

这让缓存天然有效：同一年内切换月/日/时不会使已缓存的流年结果失效，同一月内
切换日/时也不会使已缓存的流月结果失效。GUI 按 `AnalysisLayerKey::scope()` 对
缓存结果分组并隐藏空分组；`iztro` 中不含任何渲染逻辑。

## 规则状态

`RuleStatus` 记录规则的编码成熟度：

| 状态 | 含义 |
| --- | --- |
| `Raw` | 未断句的原文。 |
| `Segmented` | 已拆分为独立陈述。 |
| `Normalized` | 已规范为结构化意图。 |
| `Executable` | 已有可运行的谓词，基于已建模事实。 |
| `Tested` | 可执行，且已有真实生成盘或经审核的出处依据 fixture 的正反例覆盖，适合稳定公开使用。仅有合成试点测试不代表可标为此状态。 |
| `Ambiguous` | 含义或条件存在歧义。 |
| `Rejected` | 不予采用。 |

并非每句全书原文都能立即可执行；状态使之明确。

卷一「太微赋」的出处单元现已**完整链接**到运行时规则元数据：每条原子 rule-candidate
断语（一个 `source_item`）都链接到一条或多条规则，段落收束语则链接到一条 `Rejected`
规则以记录其被排除（见 `docs/zh-CN/rules/quan-shu-coverage.md`）。其中多数规则为
`Normalized` 或 `Ambiguous` 而非 `Executable`，且在实现谓词前不带 `[rule.claim]`——
**可执行覆盖刻意保守**。非可执行规则在运行期既不产出判断也不产出出处命中（评估器返回
`NotApplicable`），其价值在于为每一条受引出处单元保留可审计、带状态标注的记录。每条
非可执行规则都必须填写 `normalized_note_zh_hans`，由
`crates/iztro/tests/classical_source_inventory.rs` 强制校验。

## PatternDetection 与 Claim 的区别

`iztro` 已有 `core::pattern` 的**格局识别**。二者不同：

- **`PatternDetection`** 是“某已知格局形态存在”的结构化陈述（状态、家族、涉及的
  星曜/宫位、证据），是关于**排布的图盘事实**。
- **`Claim`** 是规则给出的结构化**判断**（领域、主题、吉凶、强度、证据、反证、
  出处），供下游解读、过滤与本地化渲染。

经典规则可以匹配与某已知格局相同的结构形态（昌曲夹命的判断会记录
`EvidenceKind::PatternShapeMatched { pattern: ChangQuJiaMing }`），但这不表示已经运行
`core::pattern::detect_patterns`。判断仍携带格局识别所没有的领域/主题/吉凶/出处语义。

## 示例：马遇空亡，终身奔走

1. **出处。** 语料条目：

   ```toml
   id = "migration.tian_ma_void.restless_movement"
   source_id = "quan_shu.v01.tai_wei_fu.ma_yu_kong_wang"
   source_text_zh_hans = "马遇空亡，终身奔走"
   status = "executable"

   [rule.claim]
   domain = "migration"
   themes = ["restless_movement", "instability"]
   polarity = "mixed_negative"
   claim_key = "claim.migration.tian-ma-void.restless-movement"
   ```

2. **何为空亡？** 并非名字含“空”的都算。`VoidKind` 只列举已建模的空亡族
   （旬空 `XunKong`、空亡 `KongWang`、截路 `JieLu`、截空 `JieKong`），并**排除**
   天空/地空/地劫。`VoidPolicy` 明确规则所采用的集合；`VoidPolicy::DEFAULT` 包含所有
   已建模种类，`VoidPolicy::XUN_KONG_ONLY` 与 `VoidPolicy::new(...)` 可供未来规则或
   流派采用更窄策略。

3. **谓词。** `tian_ma_affected_by_void` 找到天马所在宫，检查是否有该策略所计的
   空亡星与之同宫。

4. **出处命中与判断。** 命中时，评估器先产出 `ClassicalSourceHit`，记录典籍、原子出处
   id、中文原文、规则状态、作用范围与证据。由于该规则带有 `[rule.claim]`，
   还会产出携带
   `EvidenceKind::StarAffectedByVoid { star: TianMa, void_kind, branch }` 的判断，
   连同语料的领域/主题/吉凶/强度、`SourceRef`（中文原文）与 `claim_key`。

5. **渲染（可选）。** `iztro-i18n` 的 `claim_text(&claim)` 将 `claim_key`（点替换
   为连字符）解析为本地化文案：*“天马受空亡影响，主奔波迁动之象。”*

6. **导出。** `serde_json::to_string(&claim)` 产出确定性 JSON，包含规则 id、claim
   键、中文原文、领域、主题、吉凶、强度、证据与反证。

语料编写格式详见 [`quan-shu-corpus.md`](./quan-shu-corpus.md)。

## 星曜标签（重叠的解读型分类）

`StarTag` 是核心星曜模型中挂在 `StarName` 上的**可重叠**分类，叠加在互斥的粗分类
`StarCategory`（`Major` / `Minor` / `Adjective`）之上。一颗星可同时携带多个标签，如
地空既属空劫（`KongJie`）又属空曜（`VoidSymbol`）。当前基于标签接线的可执行全书规则包括：

- **贪居亥子，名为犯水桃花**（`relationship.tan_ju_hai_zi.water_romance`）：保守取贪狼
  居亥或子二支。
- **刑遇贪狼，号曰风流彩杖**（`relationship.xing_yu_tan_lang.romance_with_penalty`）：
  保守取贪狼与刑曜（`StarTag::Punishment` = 擎羊、天刑）同宫。
- **福德遇空劫，奔走无力**（`fortune.fu_de_yu_kong_jie.restless_spirit`）：保守取
  福德宫见地空或地劫（`StarTag::KongJie`）；该规则仅产出出处命中与证据，不新增
  claim 元数据。

`StarTag::VoidSymbol`（空曜）为**广义解读型分类**，与 `VoidKind` 刻意区分：后者仍是
马遇空亡所用的**狭义、未改动**的空亡星族（旬空 / 空亡 / 截路 / 截空）。
二者概念不同，但并非完全无交集：旬空、截空可同时出现在两个分类中。
`VoidKind` 回答马遇空亡等规则所需的狭义空亡星族问题；
`StarTag::VoidSymbol` 回答更广义的空曜解读分类问题。
天空、地空、地劫属空曜，但绝不属 `VoidKind`。

## Source inventory（原子 source item）

规则的 `source_id` 指向 QuanShu source inventory
（`crates/iztro/rule-corpus/quan-shu/source/`）中的一条**原子受引出处单元**（一句
rule-candidate 断语）。一条 Markdown 物理行可含多个出处单元——边界是语义而非排版——
因此每个 `source_item` 即一句断语，而非含嵌套 clause 的物理行/段落。`source_id` 为稳定
助记符（如 `…tai_wei_fu.ma_yu_kong_wang`），`source_order` 单独保存出处顺序。source
item 通过 `linked_rule_ids` 链接零个或多个规则。全书规则不再使用 `source_clause_id`
（该字段仍保留在 `ClassicalRule` 上，供 pattern 目录与向后兼容使用）。

inventory TOML 以紧凑的**分组**形式存储：`source_group` 携带同段共享默认值，每个
`source_group.item` 为一条原子单元，`source_id = source_id_prefix + item.key`。该分组
TOML 是唯一规范来源；测试会将其展开为扁平的逐条视图。

对于全书规则，`ClassicalSourceHit` 引用典籍出处单元。对于 pattern 目录规则，
它引用项目自有的 `pattern.*` 元数据条目；这些 pattern 条目不进入 QuanShu source
inventory。

source inventory 是**语料治理数据，而非运行时数据**：`src/` 不解析它，
`evaluate_classical` 不依赖它，三卷 Markdown 也不在运行时解析。其一致性
（稳定 id 唯一、`source_order` 连续、source item↔规则链接、逐字出处文本）仅由
`crates/iztro/tests/classical_source_inventory.rs` 校验。未定位的单元可暂用
`section = "待校"` 与 `anchor = "TODO"`。
