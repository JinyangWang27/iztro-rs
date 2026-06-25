# 经典规则引擎

本文档介绍 `crates/iztro/src/rules/classical/` 下引入的**经典规则引擎**。它将
《紫微斗数全书》等典籍中的规则编码为数据，并把图盘事实转化为结构化、带证据的
**判断（Claim）**。

它与上层的 [`rule-engine.md`](../rule-engine.md)（描述更长期的“特征 → 判断”引擎）
互补。经典规则引擎是该愿景的第一个具体的、数据驱动的切片。

> **过渡状态。** `crates/iztro/src/rules/` 中面向特征的占位脚手架
> （`Claim` / `RuleEngine` / `Evidence` 桩）**不是**最终形态。它将在后续 PR 中
> 迁移并入、或被经典引擎取代。二者目前并存，仅为保证既有脚手架测试继续通过。

## 流水线

```
图盘事实
  -> 特征/查询谓词        （复用 core/pattern 查询助手）
  -> 经典规则评估          （语料元数据 + 手写谓词）
  -> 结构化 Claim[]        （类型化枚举，serde）
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
| 规则编写 | **语料 TOML** | `crates/iztro/rule-corpus/quan-shu/rules.toml`。 |
| 导出 | **JSON** | 判断的序列化结果；绝非编写来源。 |

`iztro` 永不依赖 `iztro-i18n`：核心 crate 只产出稳定键与结构化事实；本地化文案
只存在于 Fluent 资源中。

## 混合设计（元数据 + 谓词）

目前刻意**不**做通用规则 DSL：

1. **规则元数据数据驱动**，来自语料 TOML（id、出处、状态、领域、主题、吉凶、
   基础强度、claim 键）。
2. **规则谓词手写**于 `predicates.rs`，复用 `core/pattern/` 中只读的图盘查询
   助手（夹宫匹配、亮度判定、星曜查找），不重复该逻辑。
3. `quan_shu.rs` 的评估器将每条规则的元数据与谓词配对，构建出 `Claim`。

## 保守触发与类型化诊断

只有当条件在**已建模的图盘事实**上匹配时，才会产出判断。每个评估器返回类型化的
`RuleOutcome`：

- `Emitted(Claim)`——事实已建模且条件匹配；
- `NotApplicable`——事实已建模但条件未匹配（无判断）；
- `Unsupported(UnsupportedReason)`——规则已编码，但其条件尚未由已建模的事实/
  既定策略支撑。

引擎提供两个入口：

- `evaluate_classical_claims(chart, &request) -> Vec<Claim>`——仅返回判断；
- `evaluate_classical(chart, &request) -> ClaimEvaluation { claims, diagnostics }`
  ——同时返回类型化的 `RuleDiagnostic`，使“不支持”的条件**可见**，而非被静默丢弃。

## 请求过滤与排序

`ClaimEvaluationRequest` 按 `domains`、`themes`、`polarities`、`works`、
`rule_ids`、`scopes` 过滤判断。每个字段都是允许列表；空向量表示该维度不施加约束。

不支持诊断默认使用 `DiagnosticMode::AllUnsupported`：即使判断过滤器已生效，也返回
所有不支持的语料规则诊断，使诊断通道保持完整。若调用方需要更窄的 UI/导出表面，
可选 `DiagnosticMode::MatchingRequest`（尽可能按规则元数据套用请求过滤器），或
`DiagnosticMode::None`（抑制诊断）。

返回的判断按 `(scope, domain, rule_id, claim_key)` 确定性排序。

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
   source_text_zh_hans = "马遇空亡，终身奔走"
   status = "executable"
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

4. **判断。** 命中时，评估器产出携带
   `EvidenceKind::StarAffectedByVoid { star: TianMa, void_kind, branch }` 的判断，
   连同语料的领域/主题/吉凶/强度、`SourceRef`（中文原文）与 `claim_key`。

5. **渲染（可选）。** `iztro-i18n` 的 `claim_text(&claim)` 将 `claim_key`（点替换
   为连字符）解析为本地化文案：*“天马受空亡影响，主奔波迁动之象。”*

6. **导出。** `serde_json::to_string(&claim)` 产出确定性 JSON，包含规则 id、claim
   键、中文原文、领域、主题、吉凶、强度、证据与反证。

语料编写格式详见 [`quan-shu-corpus.md`](./quan-shu-corpus.md)。
