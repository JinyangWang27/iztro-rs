# 全书语料（rule-corpus）编写说明

经典规则的**编写来源**是中文优先的语料 TOML：

```
crates/iztro/rule-corpus/quan-shu/rules.toml
```

该文件在编译期通过 `include_str!` 嵌入（与 `iztro-i18n` 嵌入 Fluent 资源的方式
一致），因此运行期加载是确定的、且不触碰文件系统。JSON 只是导出格式，绝不在此
编写。

## 字段

每条规则是一个 `[[rule]]` 表，字段如下：

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `id` | 字符串 | 稳定的规则标识，如 `migration.tian_ma_void.restless_movement`。 |
| `source_id` | 字符串 | 稳定的**出处段落**标识，如 `quan_shu.v01.tai_wei_fu.001`。指向 source inventory 中的一段原文，而非语义规则短语。 |
| `source_clause_id` | 字符串（可选） | 该段落内的 clause 标识，如 `ma_yu_kong_wang`。与 `source_id` 一起把规则链接到具体候选短语。属于规则元数据，不加载 source inventory，也不进入运行时评估。 |
| `work` | 枚举 | 典籍，目前为 `zi_wei_dou_shu_quan_shu`。 |
| `source_text_zh_hans` | 字符串 | **中文原文**（术语的权威来源）。 |
| `normalized_note_zh_hans` | 字符串（可选） | 规范化注记，说明该句如何被解读为规则。 |
| `status` | 枚举 | 编码成熟度，见下。 |
| `school` | 枚举（可选） | 流派，缺省 `general`。 |
| `domain` | 枚举 | 判断领域（`migration`、`life`、`wealth`……）。 |
| `themes` | 枚举数组 | 判断主题（`restless_movement`、`instability`……）。 |
| `polarity` | 枚举 | 吉凶（`positive`/`negative`/`mixed`/`mixed_positive`/`mixed_negative`）。 |
| `base_strength` | 浮点 | 基础强度，规范化到 `0.0..=1.0`。 |
| `claim_key` | 字符串 | 渲染本地化短文所用的 i18n 键（点分形式）。 |

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
`executable`，其评估器返回类型化的 `Unsupported` 诊断（例如禄马交驰）。

## 当前试点规则

| id | 原文 | 状态 | 说明 |
| --- | --- | --- | --- |
| `migration.tian_ma_void.restless_movement` | 马遇空亡，终身奔走 | `executable` | 天马与已建模空亡星同宫即触发。 |
| `life.yang_tuo_clamp_life.constraint_damage` | 羊陀夹命，为祸不轻 | `executable` | 擎羊、陀罗夹命宫。 |
| `life.chang_qu_clamp_life.literary_reputation` | 昌曲夹命，主贵显 | `executable` | 文昌、文曲夹命宫；记录对应的格局结构形态，不表示运行完整格局识别器。 |
| `wealth.lu_ma_remote_wealth` | 禄马交驰，发财远方 | `normalized` | “交驰”关系随流派而异、尚未建模，**不触发**，给出类型化诊断。 |
| `life.ri_yue_fan_bei.hardship_pressure` | 日月反背，劳碌辛苦 | `executable` | 太阳、太阴俱失辉（不/陷）。 |

## 出处与覆盖

每条规则通过 `source_id` + `source_clause_id` 链接到 source inventory
（`crates/iztro/rule-corpus/quan-shu/source/`）中的某条 clause。source inventory 是仅供
测试校验的语料管理数据，不进入运行时评估。其覆盖情况维护在仓库中的覆盖报告：

```
docs/zh-CN/rules/quan-shu-coverage.md
```

该报告由 `crates/iztro/tests/classical_source_coverage.rs` 生成并校验。分句（segmentation）
PR 可以只新增未链接 clause（`linked_rule_ids = []`，表示已分句但尚未规范化/实现为规则），
不必同时新增可执行规则；此类改动须同步重新生成覆盖报告。

## 新增规则的步骤

1. 在 `rules.toml` 中新增 `[[rule]]`，填好上述字段（中文原文必填）。
2. 若可执行：在 `predicates.rs` 写谓词（尽量复用 `core/pattern` 助手），在
   `quan_shu.rs` 接线产出 `Claim`。
3. 在 `iztro-i18n` 的 `claims.ftl`（en-US 与 zh-Hans）中补齐 `claim_key`（连字符
   形式）及任何新主题/领域键。
4. 加测试：正例、反例，以及（若条件尚未建模）类型化的 `Unsupported` 诊断用例。
5. 更新中英文档。
