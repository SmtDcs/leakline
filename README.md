# Leakline

Leakline reads a Zcash transaction before it is broadcast and says whether the amount would be public.

A signer, a CI job, or a hardware companion that did not build the transaction gives it the bytes. A verdict prints one JSON line and exits `0` for `CLEAN` or `1` for `LEAK`. Some inputs are not a verdict: they print an error on stderr and exit `2`. Those cases are listed below. Nothing is uploaded. No key is requested.

Track: ZECATHON Core & Tooling. Deadline: 2026-10-28 23:59 UTC.

## What it claims

The input is hex of a PCZT (the bytes start with the ASCII magic `PCZT`) or of a serialized v4, v5, or v6 transaction. Signatures and proofs are not checked. The parser is `zcash_primitives` `0.31.0-pre.2` and `pczt` `0.10.0-pre.2`. Circuit, prover, and tx-extractor features are off. There is no hand-written bundle parser.

A serialized transaction is recognized when its version is v4, v5, or v6 and the parser consumes every byte. Any other version is `LEAK` with code `UNRECOGNIZED`. The same code is used when the version is readable but `Transaction::read` rejects the bytes for a reason other than running out of input. An unknown consensus branch id is that case. `UNRECOGNIZED` stays `LEAK` under every policy, including `NoPrivacy`. Bytes left over after a transaction or after a PCZT body are not a verdict: exit `2`.

A PCZT is recognized in two cases. A v5 PCZT needs a consensus branch of NU5 or newer, no Ironwood actions, and an Ironwood value sum encoded as magnitude 0 with the sign bit clear. A negative zero, magnitude 0 with the sign set, is `UNRECOGNIZED`. A v6 PCZT needs Orchard protocol revision V3 or later. In this crate that revision is Nu6.3 and Nu7, so a v6 PCZT on NU5, Nu6, Nu6.1, or Nu6.2 is `UNRECOGNIZED`. Other Ironwood fields, such as an anchor or a proof, are not part of the v5 check. A parsed v4 PCZT, and any other parsed PCZT outside those two cases, is `LEAK` / `UNRECOGNIZED`. If `pczt::parse` itself fails, including a missing field inside a bundle, the container was not decoded: exit `2`, not a verdict.

Empty input, an odd number of hex digits, a character other than a hex digit, whitespace, or `x`, a transaction read that stops early (`UnexpectedEof`, including a truncated header), bytes left after a transaction, and a PCZT that does not parse are exit `2`.

| Code | When it is reported | Amount in the JSON |
|---|---|---|
| `REVEALED_SENDER` | At least one transparent input | Printed only when a PCZT gives every input value. A raw transaction leaves this `null` |
| `REVEALED_RECIPIENT` | At least one transparent output | That output's value |
| `POOL_CROSSING` | Two or more of Sprout, Sapling, Orchard, and Ironwood have a non-zero value balance | Those balances |
| `FEE_CANDIDATE` | Exactly one pool balance is non-zero and there is no transparent input or output | Info only. Does not make the verdict `LEAK` |
| `PCZT_METADATA` | A public PCZT field carries `proprietary`, `user_address`, or `dummy_sk` | Warning only. Does not make the verdict `LEAK` |
| `UNRECOGNIZED` | A parsed transaction is not v4, v5, or v6, or the library rejects it after the version bytes for a reason other than running out of input (including an unknown consensus branch). A parsed PCZT is not the supported v5 or v6 described above | No invented amounts |

A fee-sized second pool is still `POOL_CROSSING`. Under `FullPrivacy`, no transparent parts does not make a two-pool transaction `CLEAN`. `AllowRevealedAmounts` allows that code, and the code stays in the list. Sprout joinsplit value balance and the v6 Ironwood bundle are pools. Ironwood is read by the pinned crate. Leakline does not ship a second Ironwood parser.

Default policy is `FullPrivacy`. Names are the `zcashd` names. The lattice is partial:

| Policy | Codes that do not flip the verdict to `LEAK` |
|---|---|
| `FullPrivacy` | `FEE_CANDIDATE`, `PCZT_METADATA` |
| `AllowRevealedAmounts` | those, plus `POOL_CROSSING` |
| `AllowRevealedRecipients` | those, plus `POOL_CROSSING` and `REVEALED_RECIPIENT` |
| `AllowRevealedSenders` | `AllowRevealedAmounts`, plus `REVEALED_SENDER` |
| `AllowLinkingAccountAddresses` | same as `AllowRevealedSenders`. Recipients still leak |
| `AllowFullyTransparent` | crossing, recipient, and sender |
| `NoPrivacy` | every code except `UNRECOGNIZED`. The other codes stay in the list |

`AllowLinkingAccountAddresses` cannot see a wallet's other addresses from these bytes. The name is the `zcashd` name. The check is only the sender and the crossing.

Same bytes, two policies, two exit codes. `examples/transparent-v5.hex` is `LEAK` under `FullPrivacy` and `CLEAN` under `AllowRevealedRecipients`. `examples/crossing-v5.hex` is `LEAK` under `FullPrivacy` and `CLEAN` under `AllowRevealedAmounts`. The code stays in the JSON either way.

## What it does not claim

- Anonymity of a person, a note, or a set of transactions.
- That a solver or a bridge can freeze funds. Bytes do not prove that.
- Memo contents, an IP address, or a lightwalletd leak.
- A source-code scan. That is [ShieldLint](https://github.com/ace-coderr/ShieldLint). Leakline reads the bytes that would be broadcast.
- A label of "this transparent output is change." This crate has no public change flag. A transparent output stays `REVEALED_RECIPIENT`. A raw transaction is never guessed.
- ZIP-32 or BIP-32 derivation paths. Those getters are not public in `pczt` `0.10.0-pre.2`, so the paths are not read.
- A Merkle witness as metadata. A witness is not `PCZT_METADATA`.
- ZIP-317 fee anomalies, linked transparent inputs, or a ZIP-321 payment-request match.
- A constructed or broadcast transaction, or a check that a signature or a proof is valid. A `CLEAN` or `LEAK` byte string can still be rejected by a node.
- That every file in `examples/` is a parsed, signed transaction. The v5 files and the empty PCZT are structural fixtures. The crossing file uses zero spend-auth and binding signatures. Its Sapling proof is 192 zero bytes, and its Orchard proof is empty. Do not broadcast them. `examples/truncated.hex` is a four-byte header and exits `2`. `examples/unknown-v9.hex` is `UNRECOGNIZED`.
- A non-empty PCZT built through this crate's public API. The public roles can build an empty Nu6 PCZT. They cannot add inputs or a value sum. The v5 fixtures cover a transparent recipient and the Sapling and Orchard value balances. `REVEALED_SENDER` is covered by the decision table and the policy test, not by a v5 file. See `hackathon/10-pair-system.md`.

## How this differs from nearby tools

[ZecBlock](https://zecblock.com/) scores a transaction after it has been seen on the network. [ZEC Invoice Preflight](https://github.com/panagot/ZCASH-INVOICE-preflight) checks ZIP-321 request text. [shieldedvault-safety](https://lib.rs/crates/shieldedvault-safety) looks for secret strings in logs. `zcashd` applies a privacy policy while it is building a transaction. Leakline is a local exit code for bytes someone else already built, including a Sapling and Orchard value balance with no transparent parts.

## Run it

No network and no wallet are required. The pinned crates are in `Cargo.lock`.

```bash
cargo test --offline
./examples/demo.sh
./target/debug/leakline --policy FullPrivacy examples/transparent-v5.hex
echo $?
```

`./examples/demo.sh` builds the binary and prints the three classes: a transparent output, a Sapling/Orchard crossing, and an Orchard-only fee candidate. It also shows an unknown version staying `LEAK` under `NoPrivacy`, an empty v5, an empty Nu6 PCZT, and a truncated header exiting `2`.

Pass `--policy` with one of the names above. Omit the file to read stdin. `--help` exits `0`.

The JSON line has `status`, `policy`, `exit`, `codes`, `transparent_input_zatoshis`, `transparent_output_zatoshis`, and the four pool balances in zatoshis.

## Development

Grok writes the spec and the code. DeepSeek reviews each gate and runs `cargo test` itself. The checker does not call either API. See `AI_USAGE.md` and `hackathon/10-pair-system.md`.

License: MIT OR Apache-2.0. See `LICENSE-MIT` and `LICENSE-APACHE`.
