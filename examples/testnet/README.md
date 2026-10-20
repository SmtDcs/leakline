# Testnet examples

Real bytes from public Zcash testnet, made on 20–21 Oct 2026 with throwaway wallets and
[zcash-devtool](https://github.com/zcash/zcash-devtool) against `testnet.zec.rocks:443`.
Funds came from the Valar testnet faucet. No mainnet keys or funds were involved.

All five are NU6.3 (branch id `0x37a5165b`) v6 transactions spending from or into the Ironwood pool.

| File | What it is | Leakline (`FullPrivacy`) |
|---|---|---|
| `leak-transparent-v6.hex` | Signed tx `809a1dc3e2d4475811fb28d82760f134517bb2a8dc7db01af70dcae32528a890`. Pays 0.01 TAZ to a transparent `tm…` address, change back to Ironwood | `LEAK`, `REVEALED_RECIPIENT` |
| `clean-shielded-v6.hex` | Signed tx `27c7fc1f569307c662076681d478504adff5683879f043a847e0742712b9f70a`. Ironwood → unified address, fully shielded | `CLEAN`, `FEE_CANDIDATE` (the 10000 zat fee) |
| `shield-transparent-input-v6.hex` | Signed tx `e42375480680cc0f6b909d39c20825ffae3d564a5cdd39ecfda557490d9aad39`. Shields the 0.01 TAZ that the leak tx above paid to a throwaway `tm…` address into Ironwood (`wallet shield`). Spends a transparent input | `LEAK`, `REVEALED_SENDER` |
| `leak-transparent-v6.pczt.hex` | Unproven, unsigned PCZT for the same kind of transparent payment (`pczt create`) | `LEAK`, `REVEALED_RECIPIENT`, `PCZT_METADATA` |
| `clean-shielded-v6.pczt.hex` | Unproven, unsigned PCZT for a shielded payment to a unified address. Never signed or broadcast | `CLEAN`, `FEE_CANDIDATE`, `PCZT_METADATA` |

The PCZTs are the interesting case: they are what a signer or hardware wallet receives
from a builder it does not control. Leakline gives the verdict before any proof or signature exists.

`PCZT_METADATA` fires because the creator fills in fields such as `user_address` and the
dummy-spend `dummy_sk`. These are throwaway testnet values. Running `zcash-devtool pczt redact`
before handing a PCZT to another party is a good habit.

## Reproduce

```bash
# build zcash-devtool (rustc >= 1.89), then:
zcash-devtool wallet -w ./w init --name sender -i ./id.txt -n test -s testnet.zec.rocks:443
# fund the printed unified address from a testnet faucet, wait ~10 blocks, then
zcash-devtool wallet -w ./w sync -s testnet.zec.rocks:443

# a PCZT that pays a transparent address
zcash-devtool pczt -w ./w create --address <tm-address> --value 1000000 --output t.pczt
od -An -v -tx1 t.pczt | tr -d ' \n' > t.pczt.hex
leakline t.pczt.hex            # exit 1

# a signed transaction: send, then read the raw bytes the wallet stored
zcash-devtool wallet -w ./w send -i ./id.txt -s testnet.zec.rocks:443 --address <ua> --value 1000000
sqlite3 ./w/data.sqlite "select hex(raw) from transactions order by id_tx desc limit 1" > tx.hex
leakline tx.hex                # exit 0 for a shielded-only payment
```

Command flags follow the zcash-devtool revision of 17 Oct 2026; check `--help` on yours.
