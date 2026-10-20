# ZECATHON submission: Leakline

- **Project:** Leakline
- **Track:** Core & Tooling
- **Repo:** https://github.com/SmtDcs/leakline (must be public before 28 Oct 23:59 UTC)
- **Submitter:** Samet Şeyhanlıoğlu (@sametether)
- **Video:** _link after upload_ (rules mention a video of up to 10 minutes; this one aims for 3–5)

## Short description

Leakline is an offline checker that reads a Zcash transaction or PCZT before it is signed and tells you whether it would reveal a sender, a recipient, or an amount. It uses the `zcashd` privacy policy names, prints one JSON line and exits 0 (clean), 1 (leak) or 2 (not a verdict), so a signer, hardware-wallet companion or CI job can refuse to sign with a single `&&`.

It fills a gap Zallet itself documents: in zcash/zallet#531, `z_finalizetransaction` can only check the privacy policy for a PCZT it proposed and still has cached. For a PCZT built somewhere else, the policy can't be re-derived. Leakline derives it from the bytes. It parses with librustzcash (no hand-written parser), understands NU6.3 v6 transactions and the Ironwood pool, and ships with real testnet transactions and PCZTs as test fixtures.

## What works today

- v4/v5/v6 transactions and v5/v6 PCZTs, Sprout/Sapling/Orchard/Ironwood balances
- Codes: `REVEALED_SENDER`, `REVEALED_RECIPIENT`, `POOL_CROSSING`, `FEE_CANDIDATE`, `PCZT_METADATA`, `UNRECOGNIZED`
- All seven `zcashd` policies
- Real testnet fixtures (see `examples/testnet/README.md`), 30 tests + a doctest
- Rust library API (`inspect_bytes`, `inspect_hex`) as well as the CLI

## What it does not claim

No anonymity proof, no signature/proof check, no memo or network-level analysis. See the README.

## Video script (about 4 minutes)

Screen: a terminal in the repo, large font. No block explorer, nothing secret in scrollback.

**0:00, intro.**
"Hi, I'm Samet. This is Leakline, my project for the Core & Tooling track.
Here's the problem. With PCZTs, the program that builds a Zcash transaction and the one that signs it don't have to be the same. Your hardware wallet or your co-signer just gets some bytes. And nothing in the signing step asks: will this transaction make something public?"

**0:30, the Zallet gap.** _(show the PR #531 description, the cache-miss paragraph)_
"Zallet is adding exactly this two-step flow. In PR 531, propose returns a PCZT plus the privacy policy it needs, and finalize checks your policy against that. But it only knows the policy if it proposed the PCZT itself and still has it cached. If the PCZT came from somewhere else, the PR says the policy can't be re-derived from the PCZT. So that's the hole I wanted to fill."

**1:00, the tool.**
"Leakline reads the bytes and tells you. Offline, no keys, no network. It's a Rust CLI and also a library. It parses with librustzcash, so there's no custom parser to trust, and it knows about NU6.3 and the Ironwood pool."

**1:20, real testnet PCZT.** _(run `leakline examples/testnet/leak-transparent-v6.pczt.hex; echo $?`)_
"This is a real PCZT I made on testnet with zcash-devtool. It isn't proven or signed yet. It pays 0.01 TAZ to a transparent address. Under FullPrivacy, Leakline says LEAK, exit 1, revealed recipient, and it shows the amount that would be public. It also warns that the PCZT has metadata in it, like a user address."

**1:55, same bytes, another policy.** _(run with `--policy AllowRevealedRecipients`)_
"If I say I'm OK with a revealed recipient, the same bytes exit 0. The code is still in the output. The policy changes the decision, the facts stay the same. These are the same policy names as zcashd."

**2:15, shielded PCZT.** _(run on `clean-shielded-v6.pczt.hex`)_
"Here's one that goes to a unified address. It's all Ironwood, nothing transparent. CLEAN, exit 0. The one non-zero balance is just the fee."

**2:35, signed transactions.** _(run on the two signed `.hex` files, show the tx ids from examples/testnet/README.md)_
"Same thing works on signed transactions. These two actually went out on testnet, one transparent and one shielded, and here are the tx ids."

**2:55, as a gate.** _(show the one-line `leakline ... && sign` example in the README, then the library snippet)_
"In practice you'd put it right before signing: if Leakline doesn't exit 0, you don't sign. And a wallet like Zallet could call the library function when the cache misses."

**3:20, tests.** _(run `./examples/demo.sh`, then `cargo test`)_
"The demo runs every case, including broken input that exits 2 instead of guessing. All the tests pass."

**3:40, limits and close.**
"What it doesn't do: it's not an anonymity score, it doesn't check signatures or proofs, and it doesn't look at memos or your network. It just answers one question about the bytes you're about to sign. Next I'd like to add redaction checks and get it into a real wallet's finalize path. Thanks for watching."

## Checklist before submitting

- [ ] Make the repo public (it is private now)
- [ ] Add .github/workflows/ci.yml (needs a token with `workflow` scope) and check it is green
- [ ] Record and upload the video (own voice), paste the link above
- [ ] Fill the form on thezecathon.com: track Core & Tooling, repo URL, video URL, description above
