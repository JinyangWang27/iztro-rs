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

It renders a static natal chart from `StaticChartViewSnapshot` using the bundled
Source Han Serif SC font for Chinese text. The center panel shows factual
four-pillar labels (年柱/月柱/日柱/时柱) when the chart snapshot provides them.
This GUI remains a prototype chart-fact viewer; it does not provide readings,
rules, 成格 detection, BaZi interpretation, or narrative output.
