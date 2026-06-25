# 经典规则引擎的判断标签：领域、主题、吉凶、判断短文与证据模板。
# 键由 `iztro::rules::classical` 中稳定的枚举标识（kebab-case）派生，
# 不以中文显示文本作为逻辑键。
#
# 判断短文的键为规则 `claim_key` 中的点替换为连字符（Fluent 标识符不允许点）。

# -- 判断领域 --------------------------------------------------------------
claim-domain-life = 命身
claim-domain-body = 身
claim-domain-temperament = 性情
claim-domain-career = 事业
claim-domain-wealth = 财帛
claim-domain-migration = 迁移
claim-domain-relationship = 人际
claim-domain-marriage = 婚姻
claim-domain-children = 子女
claim-domain-parents = 父母
claim-domain-siblings = 兄弟
claim-domain-friends = 交友
claim-domain-property = 田宅
claim-domain-health = 疾厄
claim-domain-fortune = 福德
claim-domain-timing = 运限

# -- 判断主题 --------------------------------------------------------------
claim-theme-restless-movement = 奔波迁动
claim-theme-remote-development = 远方发展
claim-theme-instability = 不稳定
claim-theme-obstruction = 阻碍
claim-theme-nobleman-support = 贵人扶持
claim-theme-lack-of-support = 缺乏助力
claim-theme-authority = 权威
claim-theme-responsibility = 责任
claim-theme-work-pressure = 劳碌压力
claim-theme-career-achievement = 事业成就
claim-theme-wealth-accumulation = 财富积累
claim-theme-financial-volatility = 财务波动
claim-theme-financial-loss = 破财
claim-theme-asset-building = 置产
claim-theme-reputation = 声名
claim-theme-literary-talent = 文才
claim-theme-communication = 沟通
claim-theme-harmony = 和谐
claim-theme-conflict = 冲突
claim-theme-separation = 分离
claim-theme-loneliness = 孤独
claim-theme-stability = 稳定
claim-theme-ambition = 抱负
claim-theme-impulsiveness = 冲动
claim-theme-anxiety = 焦虑
claim-theme-vitality = 活力
claim-theme-illness-risk = 疾病风险
claim-theme-injury-risk = 受伤风险
claim-theme-blessing = 福泽
claim-theme-constraint = 牵制
claim-theme-damage = 损害
claim-theme-hidden-risk = 隐患

# -- 判断吉凶 --------------------------------------------------------------
claim-polarity-positive = 吉
claim-polarity-negative = 凶
claim-polarity-mixed = 吉凶参半
claim-polarity-mixed-positive = 偏吉
claim-polarity-mixed-negative = 偏凶

# -- 判断短文 --------------------------------------------------------------
claim-migration-tian-ma-void-restless-movement = 天马受空亡影响，主奔波迁动之象。
claim-life-yang-tuo-clamp-life-constraint-damage = 羊陀夹命，主牵制、压力与损伤之象。
claim-life-chang-qu-clamp-life-literary-reputation = 昌曲夹命，主文才、声名与贵显之象。
claim-wealth-lu-ma-remote-wealth = 禄马交驰，主远方求财、动中得财之象。
claim-life-ri-yue-fan-bei-hardship-pressure = 日月反背，主劳碌辛苦、光明不足之象。

# -- 证据模板（示意，供后续叙事层渲染） -----------------------------------
claim-evidence-star-clamps-palace = { $star } 自 { $clamp } 夹 { $target } 宫
claim-evidence-affected-by-void = { $star } 于 { $branch } 受空亡（{ $void }）影响
claim-evidence-brightness = { $star } 于 { $branch } 为 { $brightness }
