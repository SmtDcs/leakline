# Kapı kaydı

## spec — 2026-10-20

DeepSeek tabloyu yedi senaryoya karşı okudu. Altısı tabloyla uyumluydu. Biri defektti: `NoPrivacy` cümlesi tanınmayan sürümü `CLEAN` yapabiliyordu. `UNRECOGNIZED` her politikada `LEAK` olacak şekilde tablo ve `src/policy.rs` güncellendi. Grok kabul etti.

Ayrıştırıcı `zcash_primitives 0.31.0-pre.2` ve `pczt 0.10.0-pre.2` ile bağlı. `cargo test` 11 geçti. Aynı v5 transparent çıktı, `FullPrivacy` çıkış 1, `AllowRevealedRecipients` çıkış 0.

Sapling↔Orchard geçişi karar tablosunda testli. Gerçek bayt örneği henüz yok: sıfırla doldurulmuş kanıt noktası kütüphane tarafından reddediliyor.
