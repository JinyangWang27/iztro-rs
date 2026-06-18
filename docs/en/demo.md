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
charts**. Generated charts are persisted to a small JSON file under the
per-user data directory (`<data_dir>/iztro-gui/charts.json`); only the
normalized birth input is stored, and each saved chart is rebuilt
deterministically through `by_solar` when reopened.

A generated chart renders from `StaticChartViewSnapshot` using the bundled
Source Han Serif SC font for Chinese text. The center panel shows factual
four-pillar labels (年柱/月柱/日柱/时柱) when the chart snapshot provides them.
Palace cells display their own stars with category-colored labels, with a
compact legend explaining those display categories.

Selecting a palace subtly highlights its **三方四正** (opposite / wealth / career)
related palaces; a toolbar toggle controls this highlight mode. The relationship
comes entirely from a prepared, renderer-neutral `surround` field on each palace
view — the GUI performs no branch arithmetic. Natal stars carrying a mutagen
show compact, category-colored **科 / 权 / 禄 / 忌** badges read from the prepared
`mutagen` fields; the GUI computes no mutagens itself.

A bottom temporal panel shows factual decadal cells where available plus month,
day, and hour navigation labels. Its enabled cells are now **clickable** and
track a selected temporal cell in GUI state only; disabled cells stay inert. The
panel still does not switch temporal scopes or provide target-date controls, and
there is no permanent selected-palace detail panel.

This GUI remains a prototype chart-fact viewer; it does not provide readings,
rules, 成格 detection, BaZi interpretation, or narrative output.
