# AI Usage

- Tool: Grok Build
- Models: Grok orchestrates. DeepSeek (`deepseek-flash`) reviews each gate and runs the tests. The product does not call either API.
- Scope: spec, Rust checker, tests, documentation, the local demo script
- Human direction: idea choice, architecture lock, accept or reject a gate
- Files: `src/`, `tests/cli.rs`, `examples/`, `README.md`, `SECURITY.md`, `docs/DESIGN.md`, `.grok/workflows/leakline-gate.rhai`, `.github/workflows/ci.yml`
- Verification: `cargo test --offline` and `./examples/demo.sh`, plus CI. The testnet examples were made with zcash-devtool on public testnet; Leakline itself never sends a transaction.
- Later work (testnet fixtures, CI, README, cleanup) was done with Grok Bot under my direction.

All AI-generated code was reviewed against the decision table. AI was used as an assistant. The product direction and the accept or reject decision on each gate are human.
