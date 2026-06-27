# 全书语料（rule-corpus）编写说明

经典规则的**编写来源**是中文优先的语料 TOML：

```
crates/iztro/rule-corpus/quan-shu/rules.toml
```

该文件在编译期通过 `include_str!` 嵌入（与 `iztro-i18n` 嵌入 Fluent 资源的方式
一致），因此运行期加载是确定的、且不触碰文件系统。JSON 只是导出格式，绝不在此
编写。

`rule-corpus/quan-shu/` 只收录出自《紫微斗数全书》的规则（`work =
"zi_wei_dou_shu_quan_shu"`）。由项目直接建模的夹宫/格局结构推导而来、并无全书出处
的规则（如 `羊陀夹命`、`昌曲夹命`）收录于 `rule-corpus/patterns/rules.toml`
（`work = "iztro_pattern_catalog"`）。两个语料在编译期一并嵌入，运行期由
`classical_rules()` 合并供 `evaluate_classical` 使用；`quan_shu_rules()` /
`pattern_rules()` 分别只返回各自来源的规则。

## 字段

每条规则是一个 `[[rule]]` 表，字段如下：

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `id` | 字符串 | 稳定的规则标识，如 `migration.tian_ma_void.restless_movement`。 |
| `source_id` | 字符串 | 稳定的**原子出处单元**助记标识，如 `quan_shu.v01.tai_wei_fu.ma_yu_kong_wang`。指向 source inventory 中的一条受引出处单元（一句断语），而非物理行/段落。 |
| `source_clause_id` | 字符串（可选） | 历史字段：旧模型中段落内的 clause 标识。全书规则已不再使用（`source_id` 已直接指向原子出处单元）；保留为可选仅供 pattern 目录与向后兼容。 |
| `work` | 枚举 | 典籍，目前为 `zi_wei_dou_shu_quan_shu`。 |
| `source_text_zh_hans` | 字符串 | **中文原文**（术语的权威来源）。 |
| `normalized_note_zh_hans` | 字符串（可选） | 规范化注记，说明该句如何被解读为规则。 |
| `status` | 枚举 | 编码成熟度，见下。 |
| `school` | 枚举（可选） | 流派，缺省 `general`。 |
| `claim` | 表（可选） | 判断元数据。存在时，命中的可执行规则会在 `ClassicalSourceHit` 之外再产出 `Claim`；不存在时只记录出处命中。 |

`[rule.claim]` 字段如下：

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `domain` | 枚举 | 判断领域（`migration`、`life`、`wealth`……）。 |
| `themes` | 枚举数组 | 判断主题（`restless_movement`、`instability`……）。 |
| `polarity` | 枚举 | 吉凶（`positive`/`negative`/`mixed`/`mixed_positive`/`mixed_negative`）。 |
| `base_strength` | 浮点 | 基础强度，规范化到 `0.0..=1.0`。 |
| `claim_key` | 字符串 | 渲染本地化短文所用的 i18n 键（点分形式）。 |

命中的可执行规则一定会先产出 `ClassicalSourceHit`，记录 `work`、`source_id`、
中文原文、状态、scope 与证据（`source_clause_id` 对全书规则为空）。`Claim` 是解释性
语义输出，必须有 `[rule.claim]` 才会产出。全书规则的 source hit 引用典籍出处单元；
pattern 目录规则的 source hit 引用项目自有的 `pattern.*` 元数据条目。

## 关于枚举大小写

语料中的枚举值采用 crate 全局统一的 **snake_case** serde 约定（与 `iztro` 中其余
所有枚举一致），从而让编写的 TOML 与导出的 JSON 共享同一种大小写。这与任务描述里
示意用的 PascalCase 不同；此处遵循仓库既有约定以保持一致性（见
[`AGENTS.md`](../../../AGENTS.md) 的工程原则）。

## 关于 `claim_key` 与 Fluent

`claim_key` 形如 `claim.migration.tian-ma-void.restless-movement`（含点）。Fluent
标识符不允许含点，故 `.ftl` 文件中以**点替换为连字符**后的键作为消息 id
（`claim-migration-tian-ma-void-restless-movement`）。该映射由
`iztro-i18n` 的 `claim_text_key` 确定性完成。

## 规则状态

| 状态 | 含义 |
| --- | --- |
| `raw` | 未断句的原文。 |
| `segmented` | 已断句。 |
| `normalized` | 已规范为结构化意图。 |
| `executable` | 已有可运行谓词，基于已建模事实。 |
| `tested` | 可执行，且已有真实生成盘或经审核的出处依据 fixture 的正反例覆盖，适合稳定公开使用。仅有合成试点测试不代表可标为此状态。 |
| `ambiguous` | 含义/条件有歧义。 |
| `rejected` | 不予采用。 |

并非每句全书原文都能立即可执行。当某条规则的条件尚未建模时，应将其标为非
`executable`，其评估器返回类型化的 `Unsupported` 诊断（例如禄马交驰）。不支持规则
只产出 `RuleDiagnostic`，不产出 `ClassicalSourceHit`。

## 当前试点规则

全书（QuanShu）规则：

| id | 原文 | 状态 | 说明 |
| --- | --- | --- | --- |
| `migration.tian_ma_void.restless_movement` | 马遇空亡，终身奔走 | `executable` | 天马与已建模空亡星同宫即触发。 |
| `fortune.lu_ma_jiao_chi.favorable_convergence` | 禄马最喜交驰 | `normalized` | 出处忠实引《太微赋》原句；“交驰”关系随流派而异、尚未建模，**不触发**，给出类型化诊断，无 ClaimSpec。 |
| `life.ri_yue_fan_bei.hardship_pressure` | 日月最嫌反背 | `executable` | 太阳、太阴俱失辉（不/陷）；解读「劳碌辛苦」见 `normalized_note`/claim 文案。 |
| `fortune.shan_fu_ju_kong.monastic_life` | 善福居空位，天竺生涯 | `executable` | 保守取天机、天同各自受已建模空亡族星影响；source-hit-only，无 ClaimSpec。 |
| `relationship.tan_ju_hai_zi.water_romance` | 贪居亥子，名为犯水桃花 | `executable` | 保守取贪狼居亥或子二支即触发。 |
| `relationship.xing_yu_tan_lang.romance_with_penalty` | 刑遇贪狼，号曰风流彩杖 | `executable` | 保守取贪狼与刑曜（`StarTag::Punishment` = 擎羊、天刑）同宫即触发。 |

pattern 规则（`rule-corpus/patterns/rules.toml`，非全书出处）：

| id | 原文 | 状态 | 说明 |
| --- | --- | --- | --- |
| `life.yang_tuo_clamp_life.constraint_damage` | 羊陀夹命，为祸不轻 | `executable` | 擎羊、陀罗夹命宫；项目夹宫结构推导，非全书出处。 |
| `life.chang_qu_clamp_life.literary_reputation` | 昌曲夹命，主贵显 | `executable` | 文昌、文曲夹命宫；记录对应的格局结构形态，非全书出处。 |

## 出处与覆盖

每条规则通过 `source_id` 直接链接到 source inventory
（`crates/iztro/rule-corpus/quan-shu/source/`）中的一条原子出处单元。source inventory 是仅供
测试校验的语料管理数据，不进入运行时评估。其覆盖情况维护在仓库中的覆盖报告：

```
docs/zh-CN/rules/quan-shu-coverage.md
```

该报告由 `crates/iztro/tests/classical_source_coverage.rs` 生成并校验。分段（segmentation）
PR 可以只新增未链接的 `raw`/`segmented` source item（`linked_rule_ids = []`，表示已切分但尚未
规范化/实现为规则），不必同时新增可执行规则；此类改动须同步重新生成覆盖报告。

## 太微赋规范化补全

卷一「太微赋」的全部 rule-candidate 出处单元现已链接到运行时规则元数据，覆盖报告中
**unlinked source items 为 0**。链接后的规则按 `status` 分布如下（见覆盖报告）：

| status | 数量 | 说明 |
| --- | ---: | --- |
| `executable` | 5 | 马遇空亡、日月反背、贪居亥子、刑遇贪狼、善福居空位（已接线谓词）。 |
| `normalized` | 41 | 句意结构清晰，但所依赖事实（如十二长生、夹宫目标宫、星曜化气、流年杂曜等）尚未建模，暂不可执行。 |
| `ambiguous` | 17 | 术语或流派口径不明（如「冲破」「贵乡」「旺宫」、博士十二神、流年杂曜等）。 |
| `rejected` | 1 | 收束语「学至此诚玄微矣」，非断语、非规则候选。 |

规范化补全得到的规则**多数为 `normalized`/`ambiguous` 而非 `executable`**——可执行覆盖
刻意保守。未实现谓词的规则不携带 `[rule.claim]`，在运行期由评估器返回
`NotApplicable`，既不产出 `Claim` 也不产出 `ClassicalSourceHit`。少量已接线但不新增
ClaimSpec 的规则只产出 `ClassicalSourceHit` 与结构化证据，用来扩大可审计出处覆盖而不臆测
领域、主题或吉凶。每条非 `executable` 规则都必须填写 `normalized_note_zh_hans`，由
`crates/iztro/tests/classical_source_inventory.rs` 强制校验。

## 新增规则的步骤

1. 在 `rules.toml` 中新增 `[[rule]]`，填好上述字段（中文原文必填）。
2. 若该规则要产出解释性判断，新增 `[rule.claim]`；若只是先记录来源命中，可暂不加。
3. 若可执行：在 `predicates.rs` 写谓词（尽量复用 `core/pattern` 助手），在评估器中
   接线产出 `ClassicalSourceHit`，并在有 `[rule.claim]` 时产出 `Claim`。
4. 若新增了 `claim_key`，在 `iztro-i18n` 的 `claims.ftl`（en-US 与 zh-Hans）中补齐
   连字符形式的键及任何新主题/领域键。
5. 加测试：正例、反例、source hit 输出，以及（若条件尚未建模）类型化的
   `Unsupported` 诊断用例。
6. 更新中英文档。
