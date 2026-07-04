# Terminology

This glossary keeps engineering terms and Zi Wei Dou Shu terms aligned across languages.

| Internal key | English | 中文 | Notes |
|---|---|---|---|
| `heavenly_stem` | Heavenly Stem | 天干 | One of the ten stems. |
| `earthly_branch` | Earthly Branch | 地支 | One of the twelve branches. |
| `four_pillars` | Four Pillars | 四柱 | Factual year/month/day/hour stem-branch pillars; not BaZi interpretation by itself. |
| `palace` | Palace | 宫位 | A semantic area of the chart. |
| `life_palace` | Life Palace | 命宫 | Core self and natal identity. |
| `body_palace` | Body Palace | 身宫 | Manifested life path and embodied direction. |
| `career_palace` | Career Palace | 官禄宫 | Career and professional role. |
| `wealth_palace` | Wealth Palace | 财帛宫 | Money, resource flow, and income. |
| `spouse_palace` | Spouse Palace | 夫妻宫 | Marriage and intimate partnership. |
| `migration_palace` | Migration Palace | 迁移宫 | External environment, movement, platforms. |
| `property_palace` | Property Palace | 田宅宫 | Property, home, assets, living base. |
| `health_palace` | Health Palace | 疾厄宫 | Health, body, vulnerabilities. |
| `spirit_palace` | Spirit Palace | 福德宫 | Inner state, enjoyment, mental baseline. |
| `star` | Star | 星曜 | Includes major, minor, malefic, auxiliary, and other stars. |
| `star_kind` | Star Kind | 星曜细分类 | iztro-compatible fine type such as `major`, `soft`, `tough`, or `lucun`. |
| `star_category` | Star Category | 星曜分组 | Coarse palace grouping derived from `star_kind`: major, minor, or adjective. |
| `major_star` | Major Star | 主星 | The fourteen major stars. |
| `minor_star` | Minor Star | 辅星 | Supportive or secondary stars. |
| `adjective_star` | Adjective Star | 杂曜 | Miscellaneous stars and symbolic markers. |
| `star_family` | Star Family | 星曜族系 | The 昌/曲/羊/陀/马 lineage grouping a natal base star with its 运/流/月/日/时 flow variants. Taxonomy, not equality: 文曲 (`WenQu`) and 流曲 (`LiuQu`) share the `Qu` family but stay distinct `StarName`s. |
| `star_selector` | Star Selector | 星曜选择器 | Matching intent: `Exact` matches one star identity (文曲 does not match 流曲); `Family` matches a whole family explicitly. Default classical matching is exact. |
| `brightness` | Brightness | 庙旺利陷 | Strength/state of a star in a palace. |
| `mutagen` | Mutagen | 四化 | Lu, Quan, Ke, Ji transformation. |
| `mutagen_lu` | Lu | 化禄 | Resource, affinity, flow, gain. |
| `mutagen_quan` | Quan | 化权 | Authority, pressure, control, activation. |
| `mutagen_ke` | Ke | 化科 | Reputation, order, refinement, mitigation. |
| `mutagen_ji` | Ji | 化忌 | Attachment, obstruction, debt, pressure point. |
| `palace_stem` | Palace Stem | 宫干 | The Heavenly Stem assigned to each palace, derived from the birth-year stem via 起五行寅例. |
| `palace_stem_role` | Palace Stem Role | 宫干角色 | A structural role a palace plays because of its stem, with no school-specific interpretation. |
| `birth_year_stem_origin` | Birth-Year Stem Origin | 来因宫 | A palace whose stem equals the birth-year stem. Named for the structural invariant; corresponds to 飞星 practice's 来因宫. 辛/壬 years yield two; a normal chart yields one. |
| `palace_stem_mutagen_flow` | Palace Stem Mutagen Flow | 宫干四化飞化 | A derived relation from a palace stem through the 十干四化 table to a natal star/palace: source palace stem -> mutagen -> target star/palace. A derived fact, not a placed star. |
| `self_transform` | Self-transform | 自化 | A derived property of a palace-stem mutagen flow that lands back in its own source palace branch. Conservative first version; no 向心/离心 distinction. |
| `triad` | Triad | 三方 | Related palaces in the triadic structure. |
| `opposite_palace` | Opposite Palace | 对宫 | The palace opposite the target palace. |
| `surrounded_palaces` | Surrounding Palaces | 三方四正 | Target palace, opposite palace, and triadic support. |
| `decadal` | Decadal Period | 大限 | Ten-year period. |
| `age` | Minor Limit | 小限 | Annual age marker keyed by nominal age (虚岁), distinct from `yearly` (太岁-based). |
| `yearly` | Yearly Period | 流年 | Annual activation keyed by the selected year's stem-branch (太岁). |
| `monthly` | Monthly Period | 流月 | Monthly activation. |
| `daily` | Daily Period | 流日 | Daily activation. |
| `feature` | Feature | 特征 | Structured semantic property derived from a chart. |
| `claim` | Claim | 判断 | Rule output with domain, theme, strength, and evidence. |
| `evidence` | Evidence | 证据 | Chart fact supporting a claim. |
| `counter_evidence` | Counter-evidence | 反证 | Chart fact weakening or qualifying a claim. |
| `method_profile` | Method Profile | 方法配置 | A composition of strategies and rule sets. |

## Translation policy

Chinese terms should preserve standard Zi Wei Dou Shu terminology. English terms should be clear for software users even if they are not literal translations.
