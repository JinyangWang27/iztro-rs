# 应用框架、起始页、表单、按钮与中宫面板标签。

app-title = iztro-rs

# 语言切换
ui-language = 语言
ui-english = 英文
ui-simplified-chinese = 简体中文

# 起始页
startup-title = 紫微斗数 · 静态命盘
startup-subtitle = 输入出生信息生成命盘，或打开已保存的命盘。
chart-saved-charts = 已保存命盘
saved-empty = 暂无保存的命盘。生成命盘后会自动保存到本地。

# 出生信息表单
field-name = 名称
chart-name-placeholder = 命盘名称
field-year = 年
field-month = 月
field-day = 日
field-time = 时
field-gender = 性别

# 出生时间输入方式
field-input-mode = 输入方式
input-mode-clock = 出生钟表时间
input-mode-known-time-branch = 已知时辰
field-clock-hour = 时
field-clock-minute = 分
field-utc-offset = 出生地 UTC 偏移
field-apparent-solar-time = 视太阳时（常称真太阳时）校正
field-longitude = 出生地经度（东经为正，西经为负）
field-longitude-hint = 东经为正，西经为负

# 按钮
button-generate = 生成命盘
button-update = 更新命盘
button-cancel = 取消
button-edit = 修改
button-delete = 删除
button-save = 保存
button-load = 加载
button-confirm = 确认
button-next = 下一个
button-previous = 上一个
button-back = ← 返回

# 性别
gender-male = 男
gender-female = 女

# 错误与空状态提示
input-error = 输入错误：{ $message }
name-required = 请为命盘输入名称
error-year = 年份必须是整数
error-month = 月份必须是整数
error-day = 日期必须是整数
error-invalid-calendar-date = 该日期在日历中不存在
error-invalid-clock-time = 请输入 0–23 的小时与 0–59 的分钟
error-invalid-utc-offset = 出生地 UTC 偏移超出范围
error-invalid-longitude = 经度须为 -180 到 180 之间的数字
error-longitude-required = 启用校正需填写出生地经度
error-unsupported-equation-of-time = 暂不支持该真太阳时校正方式
error-invalid-birth-time = 所选出生时辰无效
error-invalid-temporal-selection = 所选运限超出范围
error-chart-generation-failed = 无法根据该输入生成命盘
persistence-unavailable = 无法访问本地存储，本次生成的命盘将不会被保存。

# 中宫面板——分区标题与行标签
center-basic-info = 基本信息
center-temporal-info = 运限信息
center-five-element-bureau = 五行局
center-four-pillars = 四柱
center-lunar = 农历
center-zodiac = 生肖
center-soul-master = 命主
center-life-palace = 命宫
center-nominal-age = 年龄(虚岁)
center-solar = 阳历
center-birth-time = 时辰
center-constellation = 星座
center-body-master = 身主
center-body-palace = 身宫

# 复合标签
age-label = { $n }岁

# 右侧检视面板（全书规则 / 格局 / 设置）
right-panel-toggle = 侧栏
right-panel-tab-quan-shu-rules = 全书规则
right-panel-tab-patterns = 格局
right-panel-tab-settings = 设置

rules-panel-empty = 当前视图未命中已支持的全书规则。
rules-panel-unknown-rule = 未知全书规则
rules-panel-no-claim = 暂未生成解释文本

patterns-panel-empty = 当前视图未命中已支持的格局。
patterns-status-fulfilled = 成格
patterns-status-weakened = 减力
patterns-status-broken = 破格

settings-panel-title = 设置
settings-language = 语言
settings-sidebar-mode = 侧栏模式
settings-sidebar-hidden = 隐藏
settings-sidebar-compact = 紧凑
settings-sidebar-expanded = 展开
settings-theme = 主题
theme-ink-paper = 水墨纸笺
theme-jade-light = 青玉明笺
theme-deep-ink = 深墨夜笺

rules-scope-natal = 本命
rules-scope-decadal = 大限
rules-scope-age = 小限
rules-scope-yearly = 流年
rules-scope-monthly = 流月
rules-scope-daily = 流日
rules-scope-hourly = 流时

# 格局展开详情行标签
patterns-detail-strength = 强度
patterns-detail-stars = 星曜
patterns-detail-palaces = 宫位
patterns-detail-mutagens = 四化
pattern-strength-weak = 弱
pattern-strength-medium = 中
pattern-strength-strong = 强

# Pattern polarity labels
pattern-polarity-auspicious = 吉
pattern-polarity-inauspicious = 凶
pattern-polarity-neutral = 平
patterns-detail-polarity = 吉凶
