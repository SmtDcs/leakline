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

İkinci DeepSeek turu aynı 15 testi geçirdi ve karar hatası bulmadı. Üç delil notu vardı. Transparent çıktının gevşek politikada `CLEAN` olması hem karar tablosunda hem v5 baytında teste bağlandı. Kullanılmayan change alanı silindi. Boş olmayan PCZT, bu crate'in açık API'siyle kurulamıyor; o fark karar tablosunda ve yapısal v5 serileştirmesinde durur.

## readme — 2026-10-20

DeepSeek testi kendisi çalıştırdı: `cargo test --offline` çıkış 0, 22 test. Karar hatası yok. Üç yazım iddiası spec ile çelişiyordu. Grok kabul etti:

- Çıkış 2 JSON basmaz. Hata stderr'e gider.
- `examples/` içindeki her dosya ayrışmaz. `truncated.hex` çıkış 2, `unknown-v9.hex` `UNRECOGNIZED`.
- v5 örnekleri imzalı değildir. İmza doğrulanmaz. Bu dosyalar alıcıyı ve havuz bakiyesini taşır. `REVEALED_SENDER` v5 dosyasında değil, karar tablosunda ve politika testindedir.

İkinci turda karar hatası yine yok. `cargo test --offline` çıkış 0, 22 test. İki yazım farkı kaldı. Kod değişmedi; README koda çekildi:

- `pczt::parse` hatası çıkış 2'dir. `UNRECOGNIZED`, PCZT ayrıştıktan sonra destek kontrolü başarısızsa konur.
- v4, v5 ve v6 cümlesi ham işlem içindir. PCZT yalnız desteklenen v5 (boş Ironwood) veya v6'dır. v4 PCZT `UNRECOGNIZED` kalır.

Üçüncü turda karar hatası yok. `cargo test --offline` çıkış 0, 22 test. Üç yazım farkı kaldı. Kod yine değişmedi:

- İki havuz, `FullPrivacy` altında transparent parça olmasa da `CLEAN` olmaz. `AllowRevealedAmounts` o kodu kabul eder.
- Bilinmeyen consensus branch, sürüm okunduğu için çıkış `2` değil, `UNRECOGNIZED`. Policy kapısındaki eşleme duruyor.
- v5 PCZT kuralı "boş Ironwood demeti" değil. Koşul: Ironwood eylemi yok ve `value_sum` sıfır. Diğer Ironwood alanları bakılmaz.

Dördüncü turda `cargo test --offline` çıkış 0 idi. Beş yazım notundan üçü cümleydi, ikisi davranıştı. Grok şunları kabul etti:

- Erken biten okuma çıkış `2` kalır. `UNRECOGNIZED` satırı EOF'u dışarıda bırakır.
- Başarılı işlem okumasından sonra bayt kalırsa çıkış `2`. Bu, `CLEAN` bir işlemin ardına eklenen baytı temiz saymamak içindir.
- `examples/truncated.hex` artık dört baytlık kesik başlıktır. Tek nibble değil.
- `SECURITY.md` anahtar istemediğini söyler. PCZT `dummy_sk` taşıyabilir; JSON onu basmaz, yalnız `PCZT_METADATA` uyarısı verir.
- Crossing fixture'da Sapling kanıtı 192 sıfır bayttır. Orchard kanıtı boştur.

`cargo test --offline` bu turdan sonra 23 test, çıkış 0. `./examples/demo.sh` çıkış 0.

Beşinci turda karar hatası yok. Test yine 23, çıkış 0. İki cümle dardı. README'ye çekildi: v6 PCZT yalnız Orchard revision V3 (bu crate'te Nu6.3 ve Nu7). NU5 üzerindeki v6 PCZT `UNRECOGNIZED`. Hex dışı karakter de çıkış 2.

Altıncı turda karar hatası yok. Test 23, çıkış 0. İki madde kaldı. Grok ikisini de kabul etti:

- PCZT gövdesinden sonra kalan bayt çıkış `2`. `postcard` `1.1.3` yalnız `alloc` ile, `pczt` ile aynı özellikte. `cargo test --offline` 24 test, çıkış 0.
- v5 Ironwood karşılaştırması `(0, false)`. Negatif sıfır `UNRECOGNIZED`. README bunu yazıyor. Kod değişmedi.

Readme kapısı bu kayıtla kapanmaz. DeepSeek bu iki düzeltmeyi bir tur daha okur.
