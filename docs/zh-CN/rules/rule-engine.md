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
地空既属空劫（`KongJie`）又属空曜（`VoidSymbol`）。基于该层现已新增两条可执行全书规则：

- **贪居亥子，名为犯水桃花**（`relationship.tan_ju_hai_zi.water_romance`）：保守取贪狼
  居亥或子二支。
- **刑遇贪狼，号曰风流彩杖**（`relationship.xing_yu_tan_lang.romance_with_penalty`）：
  保守取贪狼与刑曜（`StarTag::Punishment` = 擎羊、天刑）同宫。

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
