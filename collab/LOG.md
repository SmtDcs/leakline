# Kapı kaydı

## spec — 2026-10-20

DeepSeek tabloyu yedi senaryoya karşı okudu. Altısı tabloyla uyumluydu. Biri defektti: `NoPrivacy` cümlesi tanınmayan sürümü `CLEAN` yapabiliyordu. `UNRECOGNIZED` her politikada `LEAK` olacak şekilde tablo ve `src/policy.rs` güncellendi. Grok kabul etti.

Ayrıştırıcı `zcash_primitives 0.31.0-pre.2` ve `pczt 0.10.0-pre.2` ile bağlı. `cargo test` 11 geçti. Aynı v5 transparent çıktı, `FullPrivacy` çıkış 1, `AllowRevealedRecipients` çıkış 0.

## policy — 2026-10-20

DeepSeek `cargo test --offline` çalıştırdı: o andaki 11 test geçti. Kaynak incelemesinde beş madde vardı. Grok şunları kabul etti:

- v3 artık `UNRECOGNIZED`. v4, v5 ve v6 açıkça kabul. Boş v6 `CLEAN`.
- PCZT, ayrıştırıcının reddettiği sürüm çiftini ve NU5 öncesi dalı `UNRECOGNIZED` sayar.
- `AllowLinkingAccountAddresses` transparent alıcıya izin vermez. Gönderen ve havuz geçişi yeter.
- `PCZT_METADATA` Orchard ve Ironwood alanlarına da bakar. Merkle witness uyarı değildir.
- Sapling ve Orchard bakiyesi olan bir v5 bayt dizisi iki politikada iki çıkış verir. `cargo test` 15 geçti.
