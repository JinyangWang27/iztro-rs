# Plain Text Chart Demo

This demo shows the current supported natal chart fact surface flowing from a typed solar input into `by_solar`, then into a renderer-neutral stack snapshot, then into the `render` plain text demo.

```text
solar input -> by_solar -> ChartStackSnapshot -> render module plain text output
```

Run it with:

```bash
cargo run -p iztro --example plain_text
```

The example uses fixture-backed supported fields for a 1990-05-17 Chen-hour female natal chart. It renders chart facts only; interpretive claims and narrative reports remain separate from chart generation.

The captured output is stored at [`docs/examples/plain_text_1990_05_17_chen_female.txt`](../examples/plain_text_1990_05_17_chen_female.txt).

For the broader implemented/deferred surface around this demo, see [`current-status.md`](current-status.md).

## Local Iced GUI Prototype

The workspace also includes a local desktop prototype:

```bash
cargo run -p iztro-gui
```

It opens on a **startup page** rather than generating a default chart: enter
birth details and generate a chart, or reopen one of the locally **saved
charts**. Generated charts are persisted to a small JSON file under the per-user
local data directory (`<data_local_dir>/iztro-gui/charts.json`); only the
normalized birth input is stored, and each saved chart is rebuilt
deterministically through the core facade when reopened. There is no
current-directory fallback: if no local data directory is available, the GUI
starts without persistence and surfaces a non-fatal notice rather than scattering
saved charts.

A generated chart renders from `StaticChartViewSnapshot` using the bundled
Source Han Serif SC font for Chinese text. The center panel shows factual
four-pillar labels (年柱/月柱/日柱/时柱) when the chart snapshot provides them.
Palace cells use an iztro-like static layout: stars are zoned within each cell
rather than shown as labeled category badges. Major stars (主星) appear in the
upper-left in bold, larger, purple type; auxiliary/minor stars (辅星) appear in
the upper-middle; adjective/miscellaneous stars (杂曜) appear in the upper-right.
Brightness labels (庙旺得利平陷不) and **科 / 权 / 禄 / 忌** mutagen markers render
inline immediately after each star name. Color and position carry the category,
so the cells no longer show 主星/辅星/杂曜/神煞 group labels. The "twelve gods"
decorative stars (神煞) sit along the bottom: 长生/博士 to the lower-left, 将前/岁前
to the lower-right. A compact legend explains the tone colors. This is purely a
renderer-side display of prepared core view models — the tone of each star is
classified from its prepared `kind`/`family`; the GUI computes no stars, four
pillars, decadal periods, temporal overlays, mutagens, 三方四正, readings, rules,
成格, BaZi interpretation, or narrative.

**Hovering** a palace highlights it together with its **三方四正** (opposite /
wealth / career) related palaces: the hovered palace gets a stronger emphasis and
the related palaces a subtle filled background. Hover takes priority over the
sticky click selection while the pointer is over a palace; a toolbar toggle
controls the highlight mode, and with it off only the active palace itself is
emphasized. The relationship comes entirely from a prepared, renderer-neutral
`surround` field on each palace view — the GUI performs no branch arithmetic.
Natal stars carrying a mutagen show compact, category-colored **科 / 权 / 禄 / 忌**
badges read from the prepared `mutagen` fields; the GUI computes no mutagens
itself.

The bottom temporal panel is **effective**, not merely a selection indicator.
Its first row carries the **本命** (natal) and **限前** (pre-decadal) cells before
the normal **大限** decadal row, followed by month, day, and hour navigation
labels. Clicking an enabled cell asks core for a freshly prepared
`StaticChartViewSnapshot`: selecting a 大限 cell attaches that decadal period's
overlay to the palaces, while 本命 / 限前 (and the flowing scopes, which still need
a target date core cannot yet infer from a cell index) resolve to the natal base
slice. All temporal-overlay derivation stays in the core
`static_temporal_chart_view` facade — the GUI never builds a horoscope, temporal
layer, decadal frame, or palace names itself. Natal facts are immutable across
selections; only overlays change. Disabled cells stay inert, and there is still
no target-date control or permanent selected-palace detail panel.

This GUI remains a prototype chart-fact viewer; it does not provide readings,
rules, 成格 detection, BaZi interpretation, or narrative output.
