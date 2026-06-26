# Application chrome, startup page, forms, buttons, and center-panel labels.

app-title = iztro-rs

# Language switcher
ui-language = Language
ui-english = English
ui-simplified-chinese = Simplified Chinese

# Startup page
startup-title = Zi Wei Dou Shu · Static Chart
startup-subtitle = Enter birth details to generate a chart, or open a saved one.
chart-saved-charts = Saved Charts
saved-empty = No saved charts yet. Generated charts are saved locally automatically.

# Birth-input form
field-name = Name
chart-name-placeholder = Chart name
field-year = Year
field-month = Month
field-day = Day
field-time = Time
field-gender = Gender

# Buttons
button-generate = Generate Chart
button-update = Update Chart
button-cancel = Cancel
button-edit = Edit
button-delete = Delete
button-save = Save
button-load = Load
button-confirm = Confirm
button-next = Next
button-previous = Previous
button-back = ← Back

# Gender values
gender-male = Male
gender-female = Female

# Error / empty-state messages
input-error = Input error: { $message }
name-required = Please enter a name for the chart
error-year = Year must be a whole number
error-month = Month must be a whole number
error-day = Day must be a whole number
error-invalid-calendar-date = That date does not exist on the calendar
error-invalid-birth-time = The selected birth time is not valid
error-invalid-temporal-selection = The selected period is out of range
error-chart-generation-failed = Could not generate the chart from this input
persistence-unavailable = Persistent storage unavailable; generated charts won't be saved this session.

# Center panel — section headers and row labels
center-basic-info = Basic Info
center-temporal-info = Period Info
center-five-element-bureau = Bureau
center-four-pillars = Four Pillars
center-lunar = Lunar
center-zodiac = Zodiac
center-soul-master = Life Master
center-life-palace = Life Palace
center-nominal-age = Age (nominal)
center-solar = Solar
center-birth-time = Birth Time
center-constellation = Sign
center-body-master = Body Master
center-body-palace = Body Palace

# Composite labels
age-label = Age { $n }

# Test-only: intentionally present in en-US but absent from other locales so the
# English-fallback behavior can be verified deterministically.
test-fallback-only = English fallback

# Right inspector panel (QuanShu Rules / Patterns / Settings)
right-panel-toggle = Sidebar
right-panel-tab-quan-shu-rules = QuanShu Rules
right-panel-tab-patterns = Patterns
right-panel-tab-settings = Settings

rules-panel-empty = No supported QuanShu rules matched the current view.
rules-panel-unknown-rule = Unknown QuanShu rule
rules-panel-no-claim = No interpreted claim text is available yet.

patterns-panel-empty = No supported patterns matched the current view.
patterns-status-fulfilled = Fulfilled
patterns-status-partial = Partial
patterns-status-weakened = Weakened
patterns-status-broken = Broken

settings-panel-title = Settings
settings-language = Language
settings-sidebar-mode = Sidebar mode
settings-sidebar-hidden = Hidden
settings-sidebar-compact = Compact
settings-sidebar-expanded = Expanded

rules-scope-natal = Natal
rules-scope-decadal = Decadal
rules-scope-age = Minor limit
rules-scope-yearly = Yearly
rules-scope-monthly = Monthly
rules-scope-daily = Daily
rules-scope-hourly = Hourly

# Pattern expansion detail row labels
patterns-detail-strength = Strength
patterns-detail-stars = Stars
patterns-detail-palaces = Palaces
patterns-detail-mutagens = Mutagens
pattern-strength-weak = Weak
pattern-strength-medium = Medium
pattern-strength-strong = Strong
