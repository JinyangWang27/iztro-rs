@AGENTS.md

## Tooling: rtk (token-optimized command output)

In Claude Code on the web, a SessionStart hook (`.claude/hooks/session-start.sh`)
installs a **pinned, checksum-verified** [`rtk`](https://github.com/rtk-ai/rtk)
release (the downloaded archive is checked against a per-arch SHA256 before
extraction; a mismatch aborts the install). `rtk` is a CLI proxy that condenses
command output before it reaches the model. When `rtk` is on `PATH`, prefer it
for high-volume output:

- diffs: `rtk git diff` / `rtk git diff --cached`
- status/log: `rtk git status`, `rtk log`
- tests: `rtk test <cmd>` (shows only failures)
- errors only: `rtk err <cmd>`
- reading/searching: `rtk read <file>`, `rtk find`, `rtk tree`, `rtk ls`

Use plain commands when exact, unsummarized output matters (e.g. crafting a
precise `Edit`). `rtk` is an output filter only — it never changes what runs.