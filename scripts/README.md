# scripts

Developer helper scripts for `iztro-rs`. These are not part of the build or the
runtime; they are governance/authoring tools run by hand.

## `gen_quan_shu_source_inventory.py`

Regenerates the Volume 1 太微赋 QuanShu source inventory
(`crates/iztro/rule-corpus/quan-shu/source/volume-01.toml`) from an in-script
table of atomic source items (one cited source unit per row, in source order).
Each `source_id` is a stable mnemonic; `source_order` is assigned sequentially.

```sh
python3 scripts/gen_quan_shu_source_inventory.py
cargo test -p iztro   # validates the inventory and the coverage report
```

After editing the inventory, regenerate `docs/zh-CN/rules/quan-shu-coverage.md`
to match (the `classical_source_coverage` test fails with the expected output if
it is stale).
