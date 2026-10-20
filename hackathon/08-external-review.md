# ZECATHON 2026 — project options for an external model review

Status: research only. No repository of product code exists. No idea has been chosen. Do not pick for the author, and do not write an implementation. The author will bring your critique back to the original assistant.

Written: 2026-10-20. Audience: a model that has not seen the prior conversation. Everything you need is in this file. If a fact is marked UNVERIFIED, treat it as unknown.

## What to review

For each project shape below, answer:

1. Is the differentiator real against the linked prior art, or is this a rename of an existing project?
2. Does the design leak something the judges would treat as disqualifying? Name the field, the party who learns it, and whether that party is the chain, a server, a counterparty, or a lightwalletd operator.
3. Can one person who knows Rust and web frontends, and who has not shipped a Zcash wallet, finish the core loop before 2026-10-28 23:59 UTC?
4. What is the smallest demo that still proves the claim, and what claim should be deleted so the demo stays honest?
5. Which shape should be built, which should be cut, and which pair must not be combined because the event allows one track only?

Disagree with the scores. They are the author's judgment, on a 0–5 weighted scale, not the judges' marks.

## Event

- Name: ZECATHON. Organizer: zkSNARKs (@zksnarks_, marketplace https://zilkroad.com).
- Site: https://thezecathon.com/
- The public site is a terminal-style page. Rules, tracks, dates, and the submission form live in `https://thezecathon.com/darkpool/data.js` and `https://thezecathon.com/darkpool/forms.js`, fetched 2026-10-20. The organizer's posts on X repeat the same prize split.
- Format: online, asynchronous. Registration and submissions opened 2026-09-28 00:00 UTC.
- Deadline: 2026-10-28 23:59 UTC. In Europe/Istanbul (UTC+3) that is 2026-10-29 02:59. About eight days remain as of this writing.
- Judging: 2026-10-29 through 2026-11-22. Judge names are six empty seats until 2026-10-29. Winners announced 2026-11-23. UNVERIFIED who the judges are.
- Team: 1–5 registered people. One account submits. The prize is paid to that account. Assumed team for these shapes: one person. UNVERIFIED.
- Support contact published on the site: bladet928@gmail.com.

### Prizes

Paid in ZEC to a shielded address. The address is collected after judging, over Telegram. Total pool $100,000.

| Prize | Amount | Official description, paraphrased from `data.js` |
|---|---|---|
| Shielded Payments | $15,000 | Point of sale, payroll, remittance, subscriptions. Money moves without publishing who paid whom. |
| Private Markets | $15,000 | The dark-pool thesis. Sealed-bid auctions, order flow, settlement. The price may be public. The participants should not. |
| Cross-Chain | $15,000 | Swaps, bridges, and intents that reach other chains without unshielding on the way through. |
| Core & Tooling | $15,000 | Zebra, librustzcash, indexers, wallet SDKs, provers. Work the next Zcash developer will thank you for. |
| Wildcard | $15,000 | Art, games, social, hardware. Anything that makes a shielded transaction worth making. |
| Grand prize | $20,000 | Best overall submission, awarded on top of its track prize. |
| Community bounties | $5,000 | Smaller bounties posted during the build window. |

One submission per account, entered in exactly one track. The grand prize stacks on that track. It is not a second entry.

`data.js` still contains a developer comment calling the split a placeholder. The organizer's own X posts from 2026-09-28 state the same five tracks at $15,000. The amounts are treated as official. The comment is evidence the file was once a draft.

The $5,000 community line was used by the "Zero to ZEC" onboarding challenge. It closed 2026-10-04 23:59 America/New_York and was paid 2026-10-06: $2,500, $1,500, $1,000, plus a $500 bonus. The Wildcard track stayed open. No further open bounty is listed. UNVERIFIED whether another bounty appears before the deadline.

### Rules that change the build

Quoted from the site:

- "One person registers and submits. The prize goes to that account; build with anyone."
- "All code written during the build window. Libraries are fine; existing products are not."
- "Open source at submission. A private repo made public on Oct 28 counts."
- "One submission per account, entered in exactly one track."
- "Submissions stay sealed until judging opens. You can edit yours until the deadline."
- "Organisers and judges cannot enter."
- "Does it actually preserve privacy, or does it only look like it does? Leaks are disqualifying, not deductions."
- "Would someone use this on Monday."
- "Does it run. Working beats ambitious."
- "Built for this hackathon, not repackaged."

Judging criteria have no published weights. Privacy is an elimination rule. The other three are Usefulness, Execution, Originality.

Submission form fields: project name, exactly one track, description, repository URL, working demo URL, two-minute demo video URL. A deck is not requested. The video is. AI-tool disclosure was asked on X on 2026-10-18 and no answer was visible. UNVERIFIED. The team will keep an `AI_USAGE.md` anyway.

### Chain facts a reviewer must not "fix"

Zcash is not an EVM chain. There is no general smart-contract platform to deploy a Solidity escrow, AMM, or auction into. A design that needs a contract to hold the funds and enforce the outcome is the wrong shape unless it names a real Zcash mechanism that replaces the contract.

Value lives in notes inside pools:

- Transparent: amounts, sender inputs, and recipient outputs are public, Bitcoin-style.
- Sapling and Orchard: amounts and recipients are encrypted. A shielded memo of up to 512 bytes rides inside the note and is readable by the recipient (and by a holder of the right viewing key), not by the public chain.
- Ironwood is the NU6.3 value pool, still in the Orchard protocol family. NU7 is a later upgrade. As of 2026-10-20 the NU7 mainnet activation decision was being made the same day, with a discussed target of 2026-11-05. Do not depend on NU7 being live on mainnet during this hackathon.

Mechanisms that do exist, and that these shapes are allowed to use as libraries:

- Unified addresses and diversified receivers. One viewing key can recognize notes sent to many diversified addresses derived from it.
- ZIP-321 payment request URIs (`zcash:...`) carrying address, amount, and a base64url memo.
- Incoming and full viewing keys (IVK, UFVK). They reveal notes to the holder. They do not spend. Putting a UFVK on a server reveals every note that key can see, to whoever operates that server.
- Outgoing viewing keys, and payer-produced delivery proofs (`zcash-delivery-proof`) that show one note's value and memo without revealing the wallet.
- PCZT (partially constructed Zcash transactions, ZIP 374): create, prove, sign, and extract can be split across parties. `zcash-devtool` already implements inspect, redact, prove, and send.
- FROST threshold signatures on RedPallas can spend a shielded note without reconstituting the key. This was already demonstrated on mainnet by other teams. These shapes do not use FROST.
- Light wallets talk to a lightwalletd (or Zaino) over gRPC. The server sees the wallet's IP and which ranges it scans. It does not see shielded memo plaintext. That metadata channel is a separate leak from the transaction bytes.

Testnet coin is TAZ. It has no market value. Faucets have been unreliable across upgrade boundaries: drips have landed on a pre-NU7 chain that current wallets never scan. A demo that needs a fresh shielded send must budget a day for faucet and sync, or be designed so the core claim is visible on already-mined public transactions.

Official starting libraries, from the event's own resource list: https://zcash.readthedocs.io/en/latest/ , https://zips.z.cash/ , https://github.com/zcash/librustzcash , https://github.com/ZcashFoundation/zebra , https://github.com/zodl-inc (Zodl, formerly Zashi), https://docs.near-intents.org/ .

## How the shapes were scored

Each criterion is 0 to 5. The headline number is a weighted average, still out of 5. Below 3.5 is cut. Above 4.0 is strong.

| Criterion | Weight | Question |
|---|---|---|
| Originality | 25% | Is the gap real against the linked prior art? |
| Hackathon fit | 20% | Does it match the theme and the elimination rule on leaks? |
| Feasibility | 20% | Can one person finish the core loop in about eight days? |
| Technical depth | 15% | Is there a real engineering decision, or only glue? |
| Demoability | 10% | Is the result visible in under 90 seconds? |
| Track fit | 10% | Does the chosen track pay for this exact integration? |

When two shapes are within 0.2, the one that answers a quoted user complaint ranks higher.

| Shape | Track | Score / 5 | On the shortlist? |
|---|---|---|---|
| Leakline | Core & Tooling | 4.43 | Yes. Recommended. |
| Sidefee | Shielded Payments | 4.20 | Yes. |
| Quiet Quote | Private Markets | 4.04 | Yes, with no matching complaint. |
| Sealed Lot | Private Markets | 3.89 | No. Kept as a variant of Quiet Quote. |
| Till | Cross-Chain | 3.78 | No. Cut after the problem search. |

Per-criterion marks:

| Shape | Originality | Fit | Feasibility | Depth | Demo | Track |
|---|---|---|---|---|---|---|
| Leakline | 4.2 | 4.8 | 4.4 | 4.0 | 4.6 | 4.8 |
| Sidefee | 4.3 | 4.7 | 3.2 | 4.2 | 4.5 | 4.6 |
| Quiet Quote | 4.0 | 4.6 | 3.3 | 3.8 | 4.3 | 4.6 |
| Sealed Lot | 3.6 | 4.7 | 3.0 | 3.9 | 4.1 | 4.5 |
| Till | 3.8 | 4.5 | 2.9 | 3.4 | 4.0 | 4.4 |

## Complaints these shapes are answering

Searched 2026-10-20 on https://forum.zcashcommunity.com , GitHub, and X. Price and delisting posts were ignored.

| Complaint | Source | Status | Which shape |
|---|---|---|---|
| A user sent 1,120 shielded ZEC from Zodl through NEAR Intents on 2026-07-20. About $589k USDT was still held on 2026-09-08, fifty days later, after a written compliance clearance. The in-wallet swap did not make the trust change obvious. | https://forum.zcashcommunity.com/t/my-experience-exiting-shielded-zec-from-zodl-to-near-intents-589k-usdt-still-held-50-days-despite-a-written-compliance-clearance/57497 | Out of scope to recover those funds or to build another exit rail. In scope: say, before broadcast, which fields become public and that a solver can hold the other side. | Leakline |
| Wallets must show a boundary before a swap leaves the shielded pool. Calling a solver network "trustless" is inaccurate if solvers can pause or freeze. The same post asks for merchant invoicing that never leaves the pool, and says the fix is not a bigger bridge. A shielded stablecoin (ZSA) is proposed and is protocol work. | https://forum.zcashcommunity.com/t/the-bridge-is-a-trap-why-timtech-s-50-day-freeze-proves-we-must-close-the-shielded-loop/57616 | Open for a warning tool and for an in-pool payment. Out of scope for a new asset. | Leakline, Sidefee |
| "If I shield ZEC, what can I do with it? To do any DeFi I have to unshield? Very few accept shielded ZEC." | https://x.com/DeFi_Dad/status/2105719600436445489 (2026-10-01) | Open. A new DEX is the wrong answer. An in-pool payment, or an honest exit warning, is the answer. | Sidefee, Leakline |
| Testnet faucets paid the wrong chain. Fauzec drips landed on the pre-NU7 chain, so wallets never saw them. Jinolabs paused for NU7. A Zcash Turkey mini-hackathon could not get TAZ. Later builders report Valar at 0.125 TAZ/day and the same split-brain. | https://forum.zcashcommunity.com/t/we-need-support-with-a-testnet-zec-faucet/57087 (2026-08-16) and follow-ups in that thread | Open, too small to be the whole submission. Optional second check inside Leakline: "this tx is not on the branch your wallet scans." | Optional Leakline check |
| Unmodified WebZjs cannot sync the NU7 testnet. A builder named Paul already described this as his ZECATHON Core & Tooling entry and has a workaround that syncs to tip. Send and receive were not tested yet. | https://forum.zcashcommunity.com/t/nu7-timeline/57655 | Taken. Do not build. | None |
| ZEC Order Pay: fresh shielded address per order, UFVK detection, refunds, webhooks, offline-signed PCZT payouts, for a Python marketplace. Author says no Zcash code exists yet. The grant was filtered. | https://forum.zcashcommunity.com/t/grant-application-zec-order-pay-shielded-zec-checkout-refunds-and-creator-payouts-for-service-marketplaces/58168 (2026-10-13) | The loop already exists in Zink. Do not rebuild that checkout. | None |
| `zcash-devtool inspect` already decodes transactions, addresses, and PCZTs, and `pczt redact` strips sensitive PCZT metadata. | https://github.com/zcash/zcash-devtool | Exists. Leakline is not allowed to be a prettier inspect dump. | Constrains Leakline |

Quiet Quote and Sealed Lot matched no complaint in this pass. They stay because the Private Markets track text asks for sealed bids and hidden participants.

## Shared non-goals

None of the shapes will:

- Move, trace, or recover the frozen USDT, or contact NEAR Intents about that case.
- Ship a bridge, a solver, a shielded stablecoin, or a FROST ceremony.
- Put a spending key, mnemonic, or UFVK in the repository or in a hosted log.
- Use mainnet ZEC with market value. Testnet only, if a send is required at all.
- Enter more than one track.
- Pretend a viewing key on a server is "non-custodial privacy." The server operator sees every note that key can decrypt.

---

## Shape A — Leakline

Recommended. Track: Core & Tooling ($15,000), eligible for the $20,000 grand prize on top.

### Claim

A developer pastes a signed Zcash transaction, or a PCZT, before broadcast. Leakline prints `CLEAN` or `LEAK`, a stable reason code, and exits 0 or 1. A `LEAK` fails CI. The tool never sees a spending key.

One sentence a judge can repeat: ZecBlock scores a transaction after it is mined. Leakline refuses the broadcast when a transparent input, a transparent output, or transparent change would publish a sender, a recipient, or an amount.

### User and moment

A wallet or checkout engineer, one second before `SendTransaction`. The failure mode they are preventing is the one in the September forum posts: the product looked shielded, and the user had already left the pool.

### Core loop

1. Input is raw transaction hex, or a PCZT. No key is accepted. A key-shaped string is rejected.
2. The parser splits the transaction into transparent inputs, transparent outputs, Sapling, Orchard, and Ironwood bundles.
3. It classifies change when it can do so from the transaction alone. If it cannot know which output is change, it says so, and still flags every transparent output.
4. It prints one verdict plus reason codes. `LEAK` exits 1.
5. The same hex is opened in a public explorer. Fields the tool called public are visible there. A `CLEAN` orchard transaction shows no amount.

The core loop does not require a faucet, a lightwalletd sync, or Halo2 proving. Fixtures are already-mined transactions. One self-built testnet transaction is a bonus, not the proof.

### Reason codes, v1

| Code | Meaning | Verdict |
|---|---|---|
| `TRANSPARENT_INPUT` | A transparent prevout is spent. The amount and the source address are public. | `LEAK` |
| `TRANSPARENT_OUTPUT` | A transparent recipient is paid. The amount and the destination are public. | `LEAK` |
| `TRANSPARENT_CHANGE` | Change returns to a transparent address. | `LEAK` |
| `POOL_EXIT` | Shielded value is consumed and transparent value is created in the same transaction. This is the unshield. | `LEAK` |
| `POOL_ENTER` | Transparent value is consumed and only shielded outputs are created. The amount entering the pool is public. The destination note is not. | `LEAK` unless the caller passed `--allow-shield` |
| `SHIELDED_ONLY` | No transparent inputs or outputs. Amounts and recipients are not on the public transaction. | `CLEAN` |
| `UNKNOWN_CHANGE` | The tool will not guess which output is change. | Informational, attached to whatever verdict the transparent checks produced |

`--allow-shield` exists because a user who is deliberately shielding must not be told the tool is broken. The default is still to shout, because a checkout that accidentally shields from a transparent source has published the amount.

### What Leakline refuses to claim

- It does not detect round-trip timing, common-amount fingerprinting, or "this note was later unshielded for a similar amount." ZecBlock already does post-hoc heuristics of that kind, and they false-positive.
- It does not see the sender's IP, the lightwalletd query, or the wallet birthday. Those leaks are not in the transaction bytes.
- It does not decrypt memos. It does not ask for a viewing key.
- It does not say a `CLEAN` transaction is anonymous against a global adversary. It says the transaction does not publish sender, recipient, or amount in consensus fields.
- It does not replace `zcash-devtool inspect`. Inspect prints a debug dump. Leakline prints a pass/fail a CI script can branch on, with a sentence a wallet can show: "This transaction leaves the shielded pool. The amount and the transparent address will be public. A third party who receives those funds can freeze them."

### Architecture

- Language: Rust library `leakline-core` plus a `leakline` CLI. Optional tiny web page that calls the same library through WASM or a local binary. The library is the product. The page is the demo.
- Parse with the current `librustzcash` / `zcash_primitives` transaction types. Do not hand-roll a consensus parser.
- Output: human text on stdout, and `--json` for CI. Schema is versioned (`leakline.v1`).
- No network in the default path. `--explorer-url` only prints a link. It does not fetch.
- Tests: fixture hex files checked into the repo, with expected verdicts. At least one all-transparent, one shield, one unshield, one Orchard-only.
- Optional stretch, only if the core is green: given a txid, warn when the transaction's branch or network does not match the testnet a current wallet would scan. This answers the faucet split-brain. It is not required for the claim.

### On-chain vs off-chain

Nothing in Leakline is a new consensus rule. The chain is evidence. The product is a local gate in front of broadcast.

### Prior art

| Project | URL | What it does | Gap |
|---|---|---|---|
| ZecBlock, formerly CipherScan | https://zecblock.com/ and https://forum.zcashcommunity.com/t/cipherscan-2026-privacy-intelligence-for-zcash/54151 | Explorer heuristics, round-trip flags, a raw-tx decoder, a privacy grade after the fact. A pre-transaction advisory API was proposed for 2026. UNVERIFIED whether that API shipped. | It speaks after mining. It is not a local exit code. Even if the advisory API exists, Leakline stays offline, keyless, and CI-shaped. |
| ZEC Invoice Preflight | https://github.com/panagot/ZCASH-INVOICE-preflight | Validates a ZIP-321 URI before a merchant shares a QR. Settlement in the demo is mocked. | It inspects the request string, not the signed transaction. |
| shieldedvault-safety | https://lib.rs/crates/shieldedvault-safety | Scans logs and HTTP for addresses, viewing keys, and memo blobs. | It looks for secrets in text. It does not say which consensus field publishes an amount. |
| zcash-devtool | https://github.com/zcash/zcash-devtool | `inspect` decodes transactions and PCZTs. `pczt redact` removes optional metadata before sharing a PCZT. | A debug printer and a metadata stripper. Neither fails a build when the transaction unshields. |

### Demo, 60 seconds

Two fixtures side by side. Left: an unshield, verdict `LEAK` / `POOL_EXIT`, explorer shows the transparent amount. Right: an Orchard-only payment, verdict `CLEAN`, explorer shows no amount. Then one command in a shell: `leakline tx.hex ; echo $?` prints `1`.

### Eight-day cut

Days 1–2: parser and the four leak codes on fixtures. Days 3–4: JSON, CLI, tests. Day 5: demo page. Day 6: only if green, one testnet transaction the team builds, plus the optional branch check. Day 7: two-minute video and README. Day 8: submit. If day 2 does not parse an Orchard transaction, drop the web page and ship the CLI.

### Ways this fails

- The parser silently ignores a bundle it does not know, and calls a leaking transaction `CLEAN`. Unknown bundles must be `LEAK` with `UNRECOGNIZED_BUNDLE`.
- The demo is a screenshot of an explorer with no local tool.
- The copy says "anonymous" instead of "these fields are not public."
- v1 grows into a wallet, a broadcaster, or a viewing-key scanner.

### Score notes

Feasibility is 4.4 because the proof is public hex. Depth is 4.0 because the engineering decision is the code list and the fail-closed parser, not a new circuit. Originality is 4.2, not 5, because inspect and ZecBlock already look at transactions.

---

## Shape B — Sidefee

Shortlist. Track: Shielded Payments ($15,000), eligible for the grand prize on top.

### Claim

One shielded Orchard transaction pays two notes: a client amount and a platform fee. The platform can verify only its own note. The client's amount is never shown to the platform and never written to the public chain.

One sentence: split contracts publish every leg. Zink watches one recipient. Sidefee lets the platform prove its fee arrived inside a two-note shielded payment without holding a key that can see the client note.

### User and moment

A client pays a freelancer in ZEC, and a platform takes a fee. On an EVM chain a splitter contract publishes both amounts. Giving the platform the freelancer's viewing key opens the freelancer's whole wallet. The September forum post asked for this class of tool directly: invoice and settle in shielded ZEC so the payer never crosses a bridge to pay a person.

### Core loop

1. The freelancer's page builds a ZIP-321 request with two shielded receivers, two amounts, and two memos. The platform receiver is a diversified address that only the platform's incoming viewing key recognizes. The freelancer receiver is a different address, not covered by the platform key.
2. The payer's wallet builds one Orchard transaction with two outputs.
3. The platform syncs with its own IVK, or checks a payer-produced delivery proof for its note only (`zcash-delivery-proof`).
4. The platform UI shows "fee received" and the fee amount. It has no field for the client amount, because it cannot compute one.
5. A public explorer shows a shielded transaction and no amounts.

### Who learns what

| Party | Learns | Does not learn |
|---|---|---|
| Public chain | That a shielded transaction exists, its fee, its size, and the number of shielded actions. | Amounts, recipients, memos. |
| Payer | Both amounts and both addresses. The payer created them. | — |
| Platform | Its own fee, its own memo, and that a note to its address was mined. | The client amount, the client address, the client's other notes. |
| Freelancer | Their own amount and memo, if they scan their own key. | The platform's other customers. |
| lightwalletd used by the platform | The platform wallet's IP and scan ranges. | Memo plaintext. |

The action count is a real residual. A two-output payment is distinguishable from a one-output payment by someone who can see the transaction shape. The design must say that in the README. Hiding the action count is out of scope. It is a protocol property, not a UI property.

### Architecture

- A static merchant page: amount, fee, two receivers, ZIP-321 QR, copy-to-wallet.
- A verifier. Preferred path: the payer pastes a delivery proof and the platform checks it against the transaction bytes, so the platform server stores no viewing key. Fallback path: a local process, on the freelancer or platform operator's machine, scans with an IVK that never leaves that machine.
- The happy path does not broadcast from the web server and does not hold a spending key.
- The hard part is step 2, constructing the two-output Orchard transaction. v1 may let Zodl or Zallet construct it from the ZIP-321 URI, and keep our code on request-building plus verification. That cut must be written on the demo, or the judges will think we sent the transaction when a wallet did.

### Prior art

| Project | URL | What it does | Gap |
|---|---|---|---|
| 0xSplits | https://github.com/0xSplits/splits-contracts | Splits one payment across recipients on an EVM chain. | Every leg and amount is public. |
| Zink | https://github.com/KaranSinghBisht/zink | Fresh Orchard-only address per invoice, ZIP-321, encrypted memo, reconciliation through a UFVK. Built for ZecHub Hackathon 3.0. | One recipient. It does not separate a fee note from a client note under two different keys. |
| zcash-delivery-proof | https://github.com/saplingcash/zcash-delivery-proof | A receiver or sender proves one note's value and memo. The checker needs no key. | A library, not a two-note checkout. |
| ZEC Order Pay | forum link above | The marketplace checkout Zink already sketched, described again as a grant, with no Zcash code yet. | Rebuilding it is the repackaged project the rules reject. |

### Demo, 70 seconds

Platform panel: "fee 0.01 ZEC, paid." No client amount on that screen. Freelancer wallet: client amount increased. Explorer: shielded transaction, amounts absent. README sentence on action-count leakage, said out loud in the video so it is not a hidden caveat.

### Eight-day cut

Days 1–2: ZIP-321 builder for two receivers, plus a fixture transaction someone else already mined, verified by the delivery-proof path. If that works, the claim is real even before we can send. Days 3–5: send one two-output testnet transaction from an existing wallet, then detect only the fee note. Days 6–8: page, video, submit. If the send is blocked by the faucet, ship the verifier on a pre-mined fixture and label the broadcast as blocked. Do not fake a send.

### Ways this fails

- The platform key can also decrypt the client note. That is a viewing-key leak, and it is disqualifying relative to the claim.
- The "client amount" on the platform page is typed in by the server from the invoice database. The chain did not reveal it, but the platform still has it, because the platform wrote the invoice. Be honest: if the platform composes both amounts, it already knows both amounts. The privacy claim is then only against the public chain and against a later compromise of the platform key, not against the platform operator at invoice time.

This last point is the main attack on the shape. A reviewer should force a decision:

- Version 1, operator already knows the split, because they priced it. The product's claim shrinks to "the chain and any later viewer of the platform key cannot see the client leg." That is weaker, and closer to "Zink plus a second output."
- Version 2, the freelancer sets the client amount locally, and the platform only ever learns the fee. The invoice the payer sees is composed on the freelancer's machine. The platform provides a fee address and a fee amount. This is the version that matches the claim. It is also the version that takes longer.

The author currently means version 2. If version 2 slips, the shape should be withdrawn rather than demoed as version 1 under version 2's sentence.

### Score notes

Originality 4.3 depends on version 2. Feasibility 3.2 is the two-output send plus a correct key separation. If a reviewer thinks version 2 is not finishable, the right advice is to cut Sidefee, not to demo version 1.

---

## Shape C — Quiet Quote

Shortlist, third. Track: Private Markets ($15,000). No matching complaint in the 2026-10-20 search. It exists because the track text asks for hidden participants and a price that need not be public.

### Claim

A seller sends a price to one buyer inside a shielded memo. The price is never a commitment on the chain, so there is no later reveal transaction. The buyer pays that exact amount with a second shielded transaction. A wrong amount does not match and does not publish the price.

One sentence: Renegade and Nightpool clear hidden orders with an operator and a circuit. Quiet Quote leaves the price inside one memo and settles when the buyer pays from their own wallet.

### User and moment

Two people negotiating an OTC sale of an off-chain good or service for ZEC. Posting the price on an order book shows it to everyone. A commit-reveal auction opens every bid, including the losing ones, when the reveal phase arrives.

### Core loop

1. Off-chain, the two people exchange unified addresses. That exchange is not private against their chat channel. The chain is not involved yet.
2. The seller sends a small shielded transaction to the buyer's address. The memo contains a quote id, a price in zatoshis, an expiry height, and a hash of the terms. The public site stores the quote id and the state `open`. It does not store the price.
3. The buyer's wallet decrypts the memo.
4. The buyer sends one shielded payment to the seller for that exact amount, with the quote id in the memo.
5. The seller's local scanner, using the seller's own viewing key, marks the quote `filled` when the amount and the id match. Any other amount stays an unmatched note. The price is still absent from the site.

### Who learns what

| Party | Learns |
|---|---|
| Public chain | Two shielded transactions exist. Not the price, not the addresses, not that they form a pair. |
| Seller | The price, because they wrote it, and the buyer's address, because they were given it. |
| Buyer | The price, because the memo was encrypted to them. |
| Anyone the seller forwards the memo to | The price. The seller is trusted not to do this. |
| The website operator | The quote id and the state. Not the price, unless the seller pastes it. |
| A global observer | Cannot link the two transactions from chain data alone. The dust amount and the payment amount are hidden. |

There is no escrow. The buyer can pay and the seller can keep the ZEC and not deliver the off-chain good. The ZEC leg is final when mined. This is not a trustless market. The README has to say that in the first screen, or the shape is a privacy costume on an IOU.

### Architecture

- Memo codec: a versioned, length-checked payload inside 512 bytes. Reject unknown versions.
- Local matcher: a CLI next to a light wallet that scans the seller's incoming notes and writes quote state to a local file. The hosted page reads only that state file's public projection (id, open/filled/expired).
- No order-book history. Filled quotes disappear or remain as ids.
- v1 is one seller, one buyer, one quote. Not a book. Not a batch auction.

### Prior art

| Project | URL | What it does | Gap |
|---|---|---|---|
| Renegade | https://github.com/renegade-fi/renegade | On-chain dark pool on Arbitrum. Relayers see plaintext orders. Matching is an MPC plus a validity proof. Settlement is a contract. | Different chain, operator network, and circuit. The price exists as hidden protocol state, not as a Zcash memo. |
| Nightpool | https://github.com/KaranSinghBisht/nightpool | Midnight hackathon winner. Hidden RFQ and a sealed-bid batch. Losing bids are never opened on that chain. An operator clears the round. Settlement is a ZK proof. | Same product sentence on another chain, with trustless clearing. Quiet Quote has no operator and no proof. That is both the simplification and the weakness. |
| THORChain ZEC | https://crypto-economy.com/thorchain-completes-zec-integration/ | Native ZEC swaps. | Transparent ZEC only. Amounts and addresses are public. |

### Demo, 80 seconds

Two testnet wallets. The public page never shows a number. The buyer reads the price only in their wallet. The seller's local tool flips to `filled` after the second transaction. The explorer shows two shielded transactions.

### Eight-day cut

This shape needs two synced wallets and two real sends. Budget two days for faucet and sync before any UI. If those sends are not mined by day 5, stop and do not submit Quiet Quote. A mocked memo on a page is a privacy costume.

### Ways this fails

- The price is stored in the web app's database "just for the demo."
- The seller's viewing key is uploaded so the site can scan. That key sees every note the seller receives.
- The video claims the trade is trustless. It is private against the chain, and trusted against the seller.
- No complaint in the problem search asked for this. A reviewer who thinks track text is weaker than a quoted complaint should rank it under Sidefee, which is what the author did.

### Score notes

Originality 4.0 because Nightpool already hid the losing price. Feasibility 3.3 because of the two-wallet sync. Depth 3.8 is the memo codec and the local matcher, not a new market design.

---

## Shape D — Sealed Lot

Not on the shortlist. Documented so a reviewer can compare it with Quiet Quote. Same track: Private Markets. Same complaint status: none found.

### Claim

Several bidders pay nothing yet. Each sends a bid amount inside a shielded memo to the auctioneer's address. After the deadline the auctioneer opens the memos locally and the public page shows only the winning price. Losing amounts are never exported.

One sentence: a commit-reveal auction opens every bid on chain. Sealed Lot never has a reveal transaction, because the bid was a note, not a commitment that must be opened.

### How it differs from Quiet Quote

Quiet Quote is one price, offered to one buyer, settled by that buyer paying. Sealed Lot is many bids, one auctioneer who sees all of them, and a published winning price. The auctioneer is a stronger trusted party. Nightpool already clears a sealed batch without revealing losers, with a proof. ZecVault (https://github.com/Ridwannurudeen/zecvault) already does 2-of-3 shielded escrow on mainnet and does not do bids. Adding FROST so the auctioneer cannot run off with a deposit is a second project, and that project has been built.

Classic commit-reveal reference: https://programtheblockchain.com/posts/2018/03/20/writing-a-sealed-bid-auction-contract/

### Why it is not recommended

Originality 3.6 after Nightpool. Feasibility 3.0 because N bidders means N synced sends plus a trusted reveal UI that must be shown not to log the losing memos. Quiet Quote is the same privacy idea with one counterparty. Build that, or build neither.

### Score

3.89 out of 5. See the table above.

---

## Shape E — Till

Cut. Track it would have entered: Cross-Chain. Included so a reviewer can attack the cut.

### Claim it would have made

A merchant shares a link: "pay this Base USDC invoice." The payer spends shielded ZEC inside Zodl. NEAR Intents delivers USDC to the merchant's Base address. The page never displays the payer's ZEC address.

### Why it was cut

- The track text requires reaching the other chain without unshielding on the way through. The live NEAR Intents path from shielded ZEC moves value to a transparent address a solver can see, then to the other chain. That is an unshield. THORChain's ZEC integration is transparent-only: https://crypto-economy.com/thorchain-completes-zec-integration/
- CrossPay already does the payer-side flow inside Zodl: https://electriccoin.co/blog/private-cross-chain-payments-with-zashi-crosspay/
- Zaygent already wrapped that flow in a trading agent for ZecHub 2026: https://github.com/ZecHub/zechub/tree/main/Hackathon/2026/zaygent
- The 2026-09-08 forum report is this product's failure mode at full size. Funds left the pool, a solver held the other side, and the wallet had presented the step as one swap. Building a merchant deep link onto that rail repeats the failure. The forum post of 2026-09-15 says the response is not a bigger bridge.
- Eight days, a second chain, intent liquidity, and an UNVERIFIED deep link are enough to miss the deadline even if the privacy story were acceptable.
- Feasibility mark 2.9. Score 3.78, and the problem search then removed it from the shortlist regardless of the score.

Do not revive Till unless a reviewer can point to a specific, already-deployed interface that delivers to the other chain without a transparent ZEC output and without a solver who can freeze the destination. As of 2026-10-20 the author does not have that interface.

---

## Shapes that were considered and dropped before scoring

| Idea | Why it is not a candidate |
|---|---|
| FROST 2-of-3 escrow or payroll | ZecVault, Konclave, and CYZE already broadcast this on mainnet or testnet during ZecHub Hackathon 3.0. https://github.com/Ridwannurudeen/zecvault |
| Invoice desk: fresh shielded address, ZIP-321, UFVK reconcile | Zink. https://github.com/KaranSinghBisht/zink |
| Selective disclosure of one payout from a treasury | Glasspane Rooms. https://github.com/dolepee/glasspane |
| Trading bot funded by shielded ZEC via CrossPay | Zaygent. |
| ZIP-321 lint with mocked settlement | ZEC Invoice Preflight. |
| Explorer privacy grade | ZecBlock. |
| WebZjs on NU7 testnet | Another ZECATHON entrant is already porting it. |
| Shielded stablecoin / ZSA | Protocol change. Not a weekend build. |
| Recovering the frozen Intents USDT | Someone else's funds and a third party's compliance hold. |
| Onboarding video | The $5,000 bounty for that already paid out. |

ZecHub Hackathon 3.0 (submissions closed 2026-07-15, 36 projects) is the set of demos a Zcash judge is most likely to have already seen. Winners included ZEC-OS, zk.poker, CYZE, and Pendrake Watch. Overlap with that set is treated as fatal.

## Recommendation inside this document

Build Leakline.

It is the only shortlisted shape whose proof does not wait on a faucet. It answers the complaint the forum wrote down twice in September and that a well-known trader repeated on X on October 1: the product must say when the user is leaving the pool, and it must fail closed. The track text for Core & Tooling is this object. The grand-prize risk is that a judge wants a consumer product. Leakline's Monday user is a developer, which is the user that track names.

Build Sidefee only if the team accepts version 2 (the platform never learns the client amount, even at invoice time) and accepts a real chance that the two-output send slips. If that version slips, withdraw it.

Build Quiet Quote only if the team deliberately wants the Private Markets prize and can get two testnet sends mined early. It is not the answer to the complaints collected in this file.

Do not combine two shapes into one submission. The rule is one track.

## Questions the reviewing model should answer in writing

1. Is Leakline's gap against `zcash-devtool inspect` large enough for the Originality criterion, or is a CI wrapper around inspect a repackage?
2. For Sidefee version 2, is there a ZIP-321 or wallet bug that prevents two shielded outputs with two memos from a single URI in current Zodl? If you know that wallets drop the second memo, say so. The author has not verified current Zodl behavior.
3. Does an Orchard transaction with two actions tell a chain observer enough that the "client amount stays hidden" sentence is misleading even in version 2?
4. Is there a 2026 project, newer than Zink and Nightpool, that already does one of these three loops? The search date is 2026-10-20.
5. Given eight days and one Rust-capable person, which single shape has the highest chance of a working demo that survives the leak rule? If that answer is "none," say what to delete from Leakline until it fits.

## Source index

- Event data: https://thezecathon.com/darkpool/data.js and https://thezecathon.com/darkpool/forms.js
- Timtech freeze report: https://forum.zcashcommunity.com/t/my-experience-exiting-shielded-zec-from-zodl-to-near-intents-589k-usdt-still-held-50-days-despite-a-written-compliance-clearance/57497
- "The bridge is a trap": https://forum.zcashcommunity.com/t/the-bridge-is-a-trap-why-timtech-s-50-day-freeze-proves-we-must-close-the-shielded-loop/57616
- DeFi Dad, 2026-10-01: https://x.com/DeFi_Dad/status/2105719600436445489
- Testnet faucet split: https://forum.zcashcommunity.com/t/we-need-support-with-a-testnet-zec-faucet/57087
- NU7 / WebZjs note: https://forum.zcashcommunity.com/t/nu7-timeline/57655
- ZEC Order Pay grant: https://forum.zcashcommunity.com/t/grant-application-zec-order-pay-shielded-zec-checkout-refunds-and-creator-payouts-for-service-marketplaces/58168
- zcash-devtool: https://github.com/zcash/zcash-devtool
- Zink: https://github.com/KaranSinghBisht/zink
- ZecVault: https://github.com/Ridwannurudeen/zecvault
- Glasspane: https://github.com/dolepee/glasspane
- zcash-delivery-proof: https://github.com/saplingcash/zcash-delivery-proof
- Nightpool: https://github.com/KaranSinghBisht/nightpool
- Renegade: https://github.com/renegade-fi/renegade
- ZecBlock: https://zecblock.com/
- CrossPay: https://electriccoin.co/blog/private-cross-chain-payments-with-zashi-crosspay/
- Invoice preflight: https://github.com/panagot/ZCASH-INVOICE-preflight
