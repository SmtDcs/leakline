# AI Usage

- Tool: Grok Build
- Models: Grok orchestrates. DeepSeek (`deepseek-flash`) reviews each gate and runs the tests. The product does not call either API.
- Scope: spec, Rust checker, tests, documentation, the local demo script
- Human direction: idea choice, architecture lock, accept or reject a gate
- Files: `src/`, `tests/cli.rs`, `examples/`, `README.md`, `SECURITY.md`, `hackathon/10-pair-system.md`, `.grok/workflows/leakline-gate.rhai`, `collab/LOG.md`
- Verification: `cargo test --offline` and `./examples/demo.sh`. No testnet deploy. This tool does not send a transaction.

All AI-generated code was reviewed against the decision table. AI was used as an assistant. The product direction and the accept or reject decision on each gate are human.
