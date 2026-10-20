# AI Usage

- Tool: Grok Build
- Models: Grok orchestrates. DeepSeek (`deepseek-flash`) reviews each gate and runs the tests. The product does not call either API.
- Scope: spec, Rust checker, tests, documentation
- Human direction: idea choice, architecture lock, accept or reject a gate
- Files or commits tagged `ai-assisted`:
- Verification: tests run, manual review, testnet deploy

All AI-generated code was reviewed and tested by the team. AI was used as an assistant; the product direction and final decisions are human.
