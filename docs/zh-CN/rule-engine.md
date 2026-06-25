# 规则引擎

规则引擎把提取后的特征转换成结构化判断。它不应该直接生成最终文章。

> **当前启用的具体实现是经典规则引擎**（中文优先的《紫微斗数全书》）。它位于
> `rules::classical`，是唯一的规则引擎。其实现设计、语料格式与流水线参见
> [`rules/rule-engine.md`](./rules/rule-engine.md)、
> [`rules/quan-shu-corpus.md`](./rules/quan-shu-corpus.md) 与
> [ADR 0007](./adr/0007-classical-rule-engine.md)。
>
> 本文余下部分是“特征 → 判断”愿景的**概念性设计词汇**，并非第二套已实现的引擎；
> 下面的通用条件/效果草案仅为示意——经典引擎刻意采用数据驱动元数据加手写谓词，
> 而非通用条件 DSL。

## 规则形态（概念性）

概念上，一条规则包含三类内容：

1. 元数据。
2. 条件。
3. 效果。

示意 TOML 草案（并非已实现的语料格式——真实经典语料形态见
[`rules/rule-engine.md`](./rules/rule-engine.md)）：

```toml
id = "career.wuqu_huaquan.in_career"
domain = "career"
source = "seed"
school = "basic"
priority = 50

[condition]
palace = "career_palace"
has_star = "wuqu"
star_mutagen = "quan"

[effect]
themes = ["resource_control", "responsibility", "management_pressure"]
polarity = "mixed_positive"
strength = 0.75
evidence_template = "官禄宫见武曲化权。"
```

## 判断

规则命中后输出一个 claim：

- 领域；
- 主题；
- 极性；
- 强度；
- 证据；
- 反证；
- 来源元数据。

Claim 是中间产物，可以被测试、聚合、过滤、翻译或渲染。

## 聚合

同一领域的多个 claim 应在叙事渲染前先聚合。

例如：

- 事业执行力；
- 事业权责；
- 事业压力；
- 事业支持；
- 事业波动。

这些应该综合成领域级判断，例如“压力推动型事业发展”，而不是打印成互不相关的句子。

## 冲突处理

规则引擎必须支持限定性证据。一个盘可以同时包含支持性和阻滞性信号。

示例：

- 官禄宫强但见化忌；
- 吉曜多但煞曜也重；
- 本命格局强但当前运限未引动；
- 本宫有利但对宫受损。

输出应区分优势、风险、条件和时间。

## 规则来源

规则可来自：

- 古籍；
- 特定派别传统；
- 专家笔记；
- 用于权重校准的已有解盘；
- 真实案例反馈。

应保留来源元数据，以便审查规则。

## 权重

初始权重可以人工设定。后续版本可以用已有解盘或真实案例标签调整权重。

权重不是绝对真理，而是用于排序和综合判断的模型参数。

## 与叙事层分离

规则不应包含最终解盘长段落。规则可以包含证据模板和短标签。最终人类可读报告属于 Narrative Layer。
