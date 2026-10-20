# Demo Script

- Event: ZECATHON
- Limit: 2 minutes
- Target length: 2 minutes
- Resolution: 1080p
- Recording tool: OBS or ffmpeg screen capture
- Backup recording: record a second take
- Audio: the submitter's own voice. No AI voiceover, no music under the speech, no speed-up.

The screen is a terminal in this repo. Do not open a block explorer. Do not paste the hex into a website.

## Narration

| Time | Narration | Screen action |
|---|---|---|
| 0:00 | A signer, or a CI job, is about to accept a Zcash transaction someone else built. The bytes can still publish a recipient, a sender, or move value between Sapling and Orchard in the clear. | Title card: Leakline. Then the repo root. |
| 0:18 | Leakline reads those bytes on this machine. It prints CLEAN or LEAK and exits 0 or 1. It does not broadcast, and it does not ask for a key. | `./examples/demo.sh` starts. Leave the command visible. |
| 0:35 | This file pays a transparent output of 100000 zatoshis. Under FullPrivacy the exit code is 1 and the code is REVEALED_RECIPIENT. The input amount is null, because a raw v5 transaction does not carry it. | Pause on the first JSON block. Point at `status`, `exit`, and `transparent_input_zatoshis`. |
| 0:55 | The same file under AllowRevealedRecipients exits 0. The code stays in the list. The policy changed the decision, not the facts. | Pause on the second block. Show the shell `exit` line. |
| 1:10 | These bytes have no transparent parts. Sapling and Orchard balances are both non-zero, so this is a pool crossing. FullPrivacy exits 1. AllowRevealedAmounts exits 0. | Pause on `POOL_CROSSING` and the two balances. |
| 1:28 | Only Orchard is non-zero, by 10000 zatoshis. That is a fee candidate. It is information. FullPrivacy still exits 0. | Pause on `FEE_CANDIDATE` and exit 0. |
| 1:40 | An unknown version stays LEAK under NoPrivacy. A truncated header is not a verdict. It exits 2. No request left this machine. | Scroll to the unknown-version block, then the final `demo ok` line. |
| 1:50 | This is not a source linter, and it is not an anonymity proof. It is the exit code for the bytes that would have been broadcast. | `cargo test --offline` summary, or the README "does not claim" list. Stop at two minutes. |

## Pre-record checklist

- [ ] `./examples/demo.sh` already passed once in this checkout
- [ ] Notifications off
- [ ] Terminal font large enough to read the JSON keys
- [ ] Microphone tested
- [ ] No wallet, no explorer tab, no secret in the scrollback
- [ ] Backup take recorded

## Post-record verification

```bash
ffprobe -v error -show_entries format=duration -show_entries stream=width,height -of default=noprint_wrappers=1 demo.mp4
```

- [ ] Length at most 2 minutes
- [ ] At least 720p
- [ ] Audio is a human voice and stays in sync
- [ ] No speed-up
- [ ] No AI voiceover
- [ ] The public link opens without login
