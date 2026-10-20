# Leakline çift model sistemi

Tarih: 2026-10-20. Ürün Leakline. Bu dosya, Grok ile DeepSeek'in birlikte çalıştığı işletim sistemidir. Ürünün içine model çağrısı girmez.

## Kim ne yapar

| Rol | Model | İş |
|---|---|---|
| Orkestratör | Grok | Plan, spec kilidi, kodun yazılması, kabul ya da ret |
| Denetçi | DeepSeek (`deepseek-flash`, `https://api.deepseek.com/v1`) | Kapıyı spec'e göre okur, kendi başına `cargo test` çalıştırır |
| Karar | Grok | İki sonuç çatışırsa gerekçe `collab/LOG.md` içine yazılır ve Grok seçer |

DeepSeek spec'i genişletmez, dosyayı kendisi birleştirmez, anahtar istemez, sunucu eklemez. Sidefee, Quiet Quote, Sealed Lot ve Till kapalıdır.

Anahtar `~/.grok/config.toml` içindeki `deepseek-flash` kaydındadır. Repoya, loga ve sohbete yazılmaz.

## Döngü

Her kapı aynı sırayı izler.

1. Grok kapıyı kodlar ve kendi `cargo test` çıktısını alır.
2. Workflow `leakline-gate` çalışır. Argüman `gate`. DeepSeek önce dosyaları okur, sonra test komutunu kendisi çalıştırır.
3. DeepSeek bulgusu spec ile çelişmiyorsa Grok düzeltir ve testi yeniden alır.
4. İki model de aynı kapıda yeşil olmadan sonraki kapı açılmaz. Workflow çalışmadıysa kapı açık sayılmaz.

Workflow: `.grok/workflows/leakline-gate.rhai`. Çalıştırma: `gate` değeri `spec`, `parse`, `policy`, `pczt` veya `readme`.

## Kapılar

| Kapı | Bitti sayılması |
|---|---|
| spec | Bu dosyadaki karar tablosu duruyor. DeepSeek tabloyu okudu. |
| parse | Kilitli crate ile v5 hex açılıyor. Tanınmayan sürüm `LEAK` / `UNRECOGNIZED`. |
| policy | Üç sınıf: transparent çıkış, Sapling↔Orchard geçişi, tek havuz. Aynı bayt iki politikada iki çıkış kodu. |
| pczt | PCZT okunuyor. Ham işlemde para üstü tahmini yok. Metadata uyarı, yükleme yok. |
| readme | `cargo test` yeşil. README iddiayı ve iddia dışı kalanı yazıyor. `examples/demo.sh` üç sınıfı yerelde basar. |

Demo ve video kapı 5, form kapı 6. İkisi de bu döngünün dışında, ürün testleri yeşil olduktan sonra.

## Kilitli crate

- `zcash_primitives` `0.31.0-pre.2` (yayın 2026-10-14). `default-features = false`. `std` açık. `circuits` kapalı.
- `pczt` `0.10.0-pre.2`. `default-features = false`. `std`, `orchard`, `sapling`, `transparent` açık. `prover` ve `tx-extractor` kapalı.
- Ayrıştırma `librustzcash` ile. El yazması ayrıştırıcı yok.
- Kütüphanenin reddettiği sürüm veya demet `CLEAN` olmaz.
- `0.31.0-pre.2` Ironwood demetini ayrı alan olarak okur. Sprout joinsplit bakiyesi de bir havuzdur. İki havuzun bakiyesi birden sıfır değilse kod `POOL_CROSSING` olur. El yazması ayrıştırıcı yine yok.

## Karar tablosu

Girdi bir PCZT ya da işlem serileştirmesidir. İmza ve kanıt doğrulanmaz. Anahtar alanı yok. Ağ çağrısı yok.

Okunan alanlar: sürüm, transparent girdi sayısı, transparent çıktı tutarları, Sapling `valueBalance`, Orchard `valueBalance`. Ham v5 transparent girdi tutarını taşımaz. O tutar yalnız PCZT açıkça veriyorsa yazılır.

| Kod | Ne zaman | Tutar |
|---|---|---|
| `REVEALED_SENDER` | En az bir transparent girdi | Ham işlemde yazılmaz |
| `REVEALED_RECIPIENT` | En az bir transparent çıktı | Çıktının kendi tutarı |
| `POOL_CROSSING` | Sapling ve Orchard bakiyeleri ikisi de sıfır değil | İki bakiye de yazılır |
| `FEE_CANDIDATE` | Tek havuzun bakiyesi sıfır değil ve transparent parça yok | Bilgi. Tek başına `LEAK` yapmaz |
| `PCZT_METADATA` | PCZT içinde açık cüzdan yolu veya benzeri metadata | Uyarı. Tek başına `LEAK` yapmaz |
| `UNRECOGNIZED` | Sürüm v4, v5 veya v6 değil, ya da kütüphane demeti reddetti | Karar `LEAK` |

Para üstü kodu yalnız PCZT o çıktıyı change diye işaretlediyse konur. Bu, FullPrivacy altında transparent çıktıyı gizlemez: adres yine açıktır, kod `REVEALED_RECIPIENT` kalır. Ham işlemde change tahmini yapılmaz.

Tek havuzdan negatif bakiye ve transparent girdi, ayrı bir "havuza giriş her zaman sızıntı" kodu değildir. Görünen bulgu `REVEALED_SENDER` olur. Tutar, PCZT vermedikçe yazılmaz.

İki havuzun bakiyesi birden sıfır değilse, biri ücret kadar küçük olsa da kod `POOL_CROSSING` olur.

## Politika

Adlar `zcashd` adlarıdır. Varsayılan `FullPrivacy`. Daha gevşek politika, kendisinden sıkı olanın izin verdiğini de kabul eder.

| Politika | `LEAK` sayılmayan kodlar |
|---|---|
| `FullPrivacy` | `FEE_CANDIDATE`, `PCZT_METADATA` |
| `AllowRevealedAmounts` | öncekiler + `POOL_CROSSING` |
| `AllowRevealedRecipients` | öncekiler + `REVEALED_RECIPIENT` |
| `AllowRevealedSenders` | `AllowRevealedAmounts` + `REVEALED_SENDER` |
| `AllowFullyTransparent` | alıcı, gönderen ve havuz geçişi |
| `AllowLinkingAccountAddresses` | `AllowRevealedSenders` ile aynı: havuz geçişi ve gönderen. Alıcı yok |
| `NoPrivacy` | `UNRECOGNIZED` dışında hiçbir kod kararı `LEAK` yapmaz; kodlar listede durur |

`zcashd` politikaları tam bir sıradan çok kısmi bir sıra kurar. `AllowLinkingAccountAddresses` göndereni ve havuz geçişini kabul eder, transparent alıcıyı kabul etmez. Hesabın başka adreslerini bayttan göremez. README bunu iddia dışı bırakır.

`PCZT_METADATA` yalnız bu crate sürümünün açık bıraktığı alanlara bakar: `proprietary`, `user_address`, `dummy_sk`. Merkle witness uyarı değildir. ZIP-32 ve BIP-32 yollarının getter'ı yoktur; o yollar bu sürümde okunamaz. Public bir change bayrağı da yoktur, para üstü hiç işaretlenmez.

İzin verilmeyen bir kod varsa karar `LEAK`, çıkış `1`. Yoksa `CLEAN`, çıkış `0`. Bozuk hex, boş girdi, tek sayıda hex, erken biten okuma (`UnexpectedEof`) ve `pczt::parse` hatası çıkış `2`. Sürüm okunup v4, v5 veya v6 değilse çıkış `1`, kod `UNRECOGNIZED`. Sürüm okunup kütüphane işlemi EOF dışında reddederse de çıkış `1`'dir. Bilinmeyen consensus branch bu ikinci sınıftır: başlık okunmuştur. `UNRECOGNIZED`, `NoPrivacy` dahil her politikada `LEAK` kalır. DeepSeek spec kapısı 2026-10-20'de bu istisnayı işaret etti; tablo ona göre düzeltildi. Readme turu, bilinmeyen branch'i çıkış `2` sanmıştı. Kod policy kapısında kilitliydi; cümle koda çekildi.

## Zorunlu örnekler

1. Transparent çıktı. `FullPrivacy` → `LEAK` / `REVEALED_RECIPIENT`.
2. Transparent parça yok, Sapling ve Orchard bakiyeleri ters işaretli ve sıfır değil. `FullPrivacy` → `LEAK` / `POOL_CROSSING`. `AllowRevealedAmounts` → `CLEAN`.
3. Yalnız Orchard, bakiye sıfır ya da tek havuz ücret adayı. `FullPrivacy` → `CLEAN`.
4. Aynı bayt, `FullPrivacy` çıkış `1`, daha gevşek bir politikada çıkış `0`. Hangisi değiştiyse o kod listede görünür. Bu crate'in açık API'si boş olmayan bir PCZT kuramıyor; o yüzden PCZT bayt testi boş bir Nu6 PCZT'dir. İki politikadaki fark, karar tablosunda ve yapısal v5 serileştirmesinde durur. Bu v5 örnekleri transparent alıcıyı ve Sapling ile Orchard bakiyesini taşır. İmzalar doğrulanmaz. `REVEALED_SENDER` bir v5 dosyasında değil, karar tablosunda ve sentetik politika testindedir.

## İddia dışı

- Global anonimlik, çözücünün parayı donduracağı, memo içeriği, IP, lightwalletd.
- Kaynak kod taraması. O iş ShieldLint'tedir.
- Ayrı bir Ironwood ayrıştırıcısı. Tanınmayan demet `UNRECOGNIZED`.
- ZIP-317 sapması, birden fazla transparent girdinin bağlanması, ZIP-321 karşılaştırması. Çekirdek yeşil olduktan sonra, bu turda şart değil.
