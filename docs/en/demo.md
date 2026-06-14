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
