# Leakline design notes

This is the decision table the code follows. The README is the user-facing version; this file keeps the reasoning.

## Locked crates

- `zcash_primitives` `0.31.0-pre.2`, `default-features = false`, `std` on, circuits off.
- `pczt` `0.10.0-pre.2`, `default-features = false`, `std`, `orchard`, `sapling`, `transparent` on. `prover` and `tx-extractor` off.
- All parsing goes through librustzcash. There is no hand-written bundle parser, and no second Ironwood parser.
- A version or bundle the library rejects is never `CLEAN`.

## Inputs and fields read

Input is hex of a PCZT or of a serialized transaction. Signatures and proofs are not verified. No keys, no network.

Fields read: version, transparent input count, transparent output values, and the Sprout, Sapling, Orchard and Ironwood value balances. A raw v5/v6 transaction does not carry transparent input values; they are printed only when a PCZT gives them.

## Codes

| Code | When | Amount |
|---|---|---|
| `REVEALED_SENDER` | At least one transparent input | Not printed for raw transactions |
| `REVEALED_RECIPIENT` | At least one transparent output | The output value |
| `POOL_CROSSING` | Two or more pools have a non-zero balance | All balances |
| `FEE_CANDIDATE` | Exactly one pool is non-zero and nothing is transparent | Info only |
| `PCZT_METADATA` | `proprietary`, `user_address` or `dummy_sk` is set in a PCZT | Warning only |
| `UNRECOGNIZED` | Not v4/v5/v6, or rejected by the library after the version bytes | Always `LEAK` |

Change is never guessed. The crate has no public change flag, so a transparent change output stays `REVEALED_RECIPIENT`.

A fee-sized second pool still counts as `POOL_CROSSING`.

## Policies

Names are the `zcashd` names; the default is `FullPrivacy`. The lattice is partial:

| Policy | Codes that do not flip the verdict |
|---|---|
| `FullPrivacy` | `FEE_CANDIDATE`, `PCZT_METADATA` |
| `AllowRevealedAmounts` | + `POOL_CROSSING` |
| `AllowRevealedRecipients` | + `POOL_CROSSING`, `REVEALED_RECIPIENT` |
| `AllowRevealedSenders` | `AllowRevealedAmounts` + `REVEALED_SENDER` |
| `AllowLinkingAccountAddresses` | same as `AllowRevealedSenders` |
| `AllowFullyTransparent` | crossing, recipient, sender |
| `NoPrivacy` | everything except `UNRECOGNIZED` |

`AllowLinkingAccountAddresses` cannot see a wallet's other addresses from bytes alone.

## Exit codes

- `0` CLEAN, `1` LEAK.
- `2`: empty input, bad hex, odd length, early EOF (including a truncated header), trailing bytes after a transaction or PCZT, or a PCZT that `pczt::parse` rejects.
- An unknown consensus branch id is a readable header that the library rejects: `1` / `UNRECOGNIZED`.
- A v5 PCZT Ironwood value sum must be magnitude 0 with the sign clear; a negative zero is `UNRECOGNIZED`.

## Out of scope

Global anonymity, solver/bridge freezing, memo contents, IP or lightwalletd leaks, source scanning (ShieldLint does that), ZIP-317 fee anomalies, linking of multiple transparent inputs, ZIP-321 matching.
