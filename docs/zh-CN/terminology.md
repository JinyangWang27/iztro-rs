# 术语表

本术语表用于保持工程术语和紫微斗数术语在中英文文档中的一致性。

| Internal key | English | 中文 | 说明 |
|---|---|---|---|
| `heavenly_stem` | Heavenly Stem | 天干 | 十天干之一。 |
| `earthly_branch` | Earthly Branch | 地支 | 十二地支之一。 |
| `four_pillars` | Four Pillars | 四柱 | 年、月、日、时四组干支事实；本身不等于八字解读。 |
| `palace` | Palace | 宫位 | 星盘中的生活领域。 |
| `life_palace` | Life Palace | 命宫 | 自我、命格、核心身份。 |
| `body_palace` | Body Palace | 身宫 | 行动路径、后天体现、身体化方向。 |
| `career_palace` | Career Palace | 官禄宫 | 职业、职位、事业角色。 |
| `wealth_palace` | Wealth Palace | 财帛宫 | 钱财、资源流、收入方式。 |
| `spouse_palace` | Spouse Palace | 夫妻宫 | 婚姻、亲密关系、伴侣互动。 |
| `migration_palace` | Migration Palace | 迁移宫 | 外部环境、移动、平台、远方。 |
| `property_palace` | Property Palace | 田宅宫 | 房产、家庭基底、资产承载。 |
| `health_palace` | Health Palace | 疾厄宫 | 健康、身体、风险点。 |
| `spirit_palace` | Spirit Palace | 福德宫 | 内在状态、享受方式、精神底色。 |
| `star` | Star | 星曜 | 包括主星、辅星、煞星、杂曜等。 |
| `star_kind` | Star Kind | 星曜细分类 | 与 iztro 兼容的细分类，例如 `major`、`soft`、`tough` 或 `lucun`。 |
| `star_category` | Star Category | 星曜分组 | 从 `star_kind` 派生出的粗粒度宫位分组：major、minor 或 adjective。 |
| `major_star` | Major Star | 主星 | 十四主星。 |
| `minor_star` | Minor Star | 辅星 | 辅助性或次级星曜。 |
| `adjective_star` | Adjective Star | 杂曜 | 杂曜和其他象义标记。 |
| `star_family` | Star Family | 星曜族系 | 昌/曲/羊/陀/马族系，将本命基础星与其运/流/月/日/时飞星归为同一族系。族系是分类谱系，非等同关系：文曲与流曲同属 `Qu` 族但仍是不同的 `StarName`。 |
| `star_selector` | Star Selector | 星曜选择器 | 匹配意图：`Exact` 仅匹配单一星曜身份（文曲不匹配流曲）；`Family` 显式匹配整个族系。默认经典匹配为精确匹配。 |
| `brightness` | Brightness | 庙旺利陷 | 星曜在宫位中的状态和强弱。 |
| `mutagen` | Mutagen | 四化 | 禄、权、科、忌。 |
| `mutagen_lu` | Lu | 化禄 | 资源、缘分、顺流、收益。 |
| `mutagen_quan` | Quan | 化权 | 权责、压力、控制、推动。 |
| `mutagen_ke` | Ke | 化科 | 名声、秩序、修饰、缓冲。 |
| `mutagen_ji` | Ji | 化忌 | 执着、阻滞、债务、压力点。 |
| `palace_stem` | Palace Stem | 宫干 | 每个宫位所配的天干，由生年天干起五行寅例推定。 |
| `palace_stem_role` | Palace Stem Role | 宫干角色 | 因宫干形成的结构性角色，不含任何流派解读。 |
| `birth_year_stem_origin` | Birth-Year Stem Origin | 来因宫 | 宫干等于生年天干的宫位。以结构不变量命名，对应飞星派「来因宫」。辛、壬年会出现两个，一般命盘只有一个。 |
| `palace_stem_mutagen_flow` | Palace Stem Mutagen Flow | 宫干四化飞化 | 由某宫宫干经十干四化落到本命某星／宫的派生关系：源宫干 → 四化 → 目标星／宫。属派生事实，非虚拟星曜。 |
| `self_transform` | Self-transform | 自化 | 宫干四化落回本宫（源宫地支＝目标宫地支）的派生属性。保守首版不含向心／离心区分。 |
| `triad` | Triad | 三方 | 三合结构中的相关宫位。 |
| `opposite_palace` | Opposite Palace | 对宫 | 与目标宫相对的宫位。 |
| `surrounded_palaces` | Surrounding Palaces | 三方四正 | 本宫、对宫与三方相关宫位。 |
| `decadal` | Decadal Period | 大限 | 十年运限。 |
| `age` | Minor Limit | 小限 | 以虚岁为准的年度运限标记，区别于以太岁论的 `yearly`（流年）。 |
| `yearly` | Yearly Period | 流年 | 年度激活，以所选年份干支（太岁）为准。 |
| `monthly` | Monthly Period | 流月 | 月度激活。 |
| `daily` | Daily Period | 流日 | 日度激活。 |
| `feature` | Feature | 特征 | 从星盘中提取的结构化语义属性。 |
| `claim` | Claim | 判断 | 规则输出，包含领域、主题、强度和证据。 |
| `evidence` | Evidence | 证据 | 支撑判断的星盘事实。 |
| `counter_evidence` | Counter-evidence | 反证 | 削弱或限定判断的星盘事实。 |
| `method_profile` | Method Profile | 方法配置 | 策略和规则集的组合。 |

## 翻译政策

中文术语应保留标准紫微斗数术语。英文术语应优先保证软件用户可理解，不必机械直译。
