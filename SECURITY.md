# Security

Leakline is a local checker. It is not a wallet, a node, or a deployed contract. It has not had a third-party audit.

## What the program is allowed to do

- Read hex from a file or from stdin.
- Parse it with the pinned `librustzcash` crates.
- Print one JSON verdict and an exit code.

## What it refuses

- Spending keys, viewing keys, and seeds. There is no field for them.
- Broadcasting a transaction, or opening a network connection. The binary does not fetch an explorer.
- Inventing a transparent input amount the bytes do not carry. A raw transaction reports that amount as `null`.
- Treating an unrecognized version as `CLEAN`.

## Known limits

A `CLEAN` result means the selected policy allows every code this build can see. It does not mean the sender is anonymous, the memo is hidden, or a later wallet behavior is safe.

`FEE_CANDIDATE` is a conventional reading of a single non-zero pool balance. It is not a ZIP-317 proof. Two non-zero pools are `POOL_CROSSING` even when one balance is fee-sized.

`PCZT_METADATA` sees only `proprietary`, `user_address`, and `dummy_sk`. ZIP-32 and BIP-32 paths are not readable in this crate version. A transparent output is never labeled as change.

The files in `examples/` parse. They are not valid zero-knowledge proofs. Do not broadcast them.

`AllowLinkingAccountAddresses` does not discover other addresses of an account. The bytes do not contain that information.
