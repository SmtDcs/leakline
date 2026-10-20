# Fikir laboratuvarı

Araştırma tarihi: 20 Ekim 2026.

## Kısıtlar

- Etkinlik: ZECATHON. Teslim 28 Ekim 2026, 23:59 UTC.
- Varsayılan ekip: tek kişi. Saat: yaklaşık 8 gün. İkisi de kullanıcıdan teyit bekliyor.
- Kod bu pencerede yazılacak. Kütüphane serbest, mevcut ürün değil.
- Tek track. Sızıntı eler. Çalışan, iddialı olandan üstün.
- Zcash'te sözleşme yok. Demo bir shielded not, bir memo veya bir işlemin public alanları üzerinde durur.
- Elenen hazır kalıplar: Zink fatura masası, ZecVault / Konclave / CYZE emanet, Glasspane ifşası, Zaygent trading botu, ZIP-321 ön kontrol, ZecBlock explorer puanı.

## Aday 1 — Leakline

Core & Tooling.

Bir cümle: İmzalı bir Zcash işlemini yayınlamadan okur; gönderen, alıcı veya tutar public alana çıkıyorsa süreci kırar.

Kullanıcı ve an: Cüzdan veya ödeme entegrasyonu yazan geliştirici, işlemi ağa vermeden bir saniye önce. Hata, müşteri mainnet'te unshield ettikten sonra ortaya çıkıyor.

Çekirdek döngü:

1. Geliştirici ham tx hex veya PCZT yapıştırır.
2. Araç havuzları ayırır: transparent girdi, transparent çıktı, Sapling, Orchard, Ironwood.
3. Para üstünün transparent adrese dönüp dönmediğine bakar.
4. `CLEAN` veya `LEAK` ve kırılma kodu basar. `LEAK` çıkış kodu 1'dir.
5. Aynı hex bir explorer'da açılır. Aracın "public" dediği alan orada görünür, `CLEAN` işlemde tutar görünmez.

Zincirde yeni işlem şart değil. Doğrulama, kazınmış bir mainnet tx ve bizim ürettiğimiz bir testnet tx ile yapılır. Harcama anahtarı araca girmez.

Track: Core & Tooling cümlesi birebir bu. "SDKs, explorers, testing tools."

Demo: İki hex, iki etiket, bir explorer sekmesi. 60 saniye.

Risk: `librustzcash` işlem çözümleyicisi Ironwood paketinde API kayması yaşatır. Küçültme: önce transparent ve Orchard ayrımını çözen dar bir çözümleyici, PCZT ikinci gün.

### Önceki işler

| Proje | Link | Ne yapıyor | Fark |
|---|---|---|---|
| ZecBlock, eski CipherScan | https://zecblock.com/ ve https://forum.zcashcommunity.com/t/cipherscan-2026-privacy-intelligence-for-zcash/54151 | Kazınmış işlemlerde round-trip ve gizlilik puanı. Ham tx çözücü ve yayın aracı da var | İşlem ağda olduktan sonra konuşur. CI kapısı değil |
| ZEC Invoice Preflight | https://github.com/panagot/ZCASH-INVOICE-preflight | ZIP-321 isteğini paylaşmadan önce doğrular. Uzlaşma mock | İstek metnine bakar, imzalı işleme bakmaz |
| shieldedvault-safety | https://lib.rs/crates/shieldedvault-safety | Log ve istekte adres, viewing key, memo sızıntısı arar | Anahtar metni arar, işlemin hangi alanının public olduğunu söylemez |

Ayırıcı cümle: ZecBlock işlemi kazındıktan sonra puanlar; Leakline imzalı işlemi yayınlanmadan okur ve transparent girdi, transparent çıktı veya açık para üstü varsa süreci kırar.

UNVERIFIED: ZecBlock'un 2026 teklifindeki "pre-transaction advisory" API'si yayında mı. Yayında olsa bile fark yerelde kalır: hesap yok, anahtar yok, çıkış kodu var.

### Puan

| Ölçüt | Puan | Ağırlık | Ağırlıklı |
|---|---|---|---|
| Özgünlük | 4.2 | 25% | 1.05 |
| Hackathon uyumu | 4.8 | 20% | 0.96 |
| Yapılabilirlik | 4.4 | 20% | 0.88 |
| Teknik derinlik | 4.0 | 15% | 0.60 |
| Demo | 4.6 | 10% | 0.46 |
| Track uyumu | 4.8 | 10% | 0.48 |
| **Toplam** | | | **4.43** |

## Aday 2 — Sidefee

Shielded Payments.

Bir cümle: Tek shielded işlem müşteriye ve platforma ayrı not gönderir; platform yalnızca kendi notunun geldiğini doğrular, müşteri tutarını görmez.

Kullanıcı ve an: Freelance işi ZEC ile ödeyen biri. Platform komisyonu istiyor. EVM'de split sözleşmesi her iki tutarı da yayınlıyor. Platforma viewing key vermek de tüm kasayı açıyor.

Çekirdek döngü:

1. Müşteri bir ZIP-321 istek görür: iki shielded alıcı, iki tutar, iki memo.
2. Cüzdan tek Orchard işlemi kurar.
3. Platform sayfası yalnızca kendi notasını, kendi incoming viewing key veya bir teslim kanıtıyla doğrular.
4. Müşteri tutarı platformun ekranında yoktur.
5. Explorer işlemi shielded gösterir. Tutarlar yazmaz.

Zincir: Zcash testnet. Sözleşme yok. Kanıt, iki nottan yalnız birinin açılmasıdır.

Track: Shielded Payments. Checkout ve fatura örneklerinin içindedir.

Demo: Platform paneli "komisyon geldi, 0.01". Müşteri bakiyesi arttı. Explorer boş. 70 saniye.

Risk: Çok çıktılı Orchard işlemini 8 günde kurmak. Küçültme: gönderimi ZODL veya Zallet yapar, bizim kod platform notunu doğrular. Bu, derinliği düşürür; yine de döngü gerçektir.

### Önceki işler

| Proje | Link | Ne yapıyor | Fark |
|---|---|---|---|
| 0xSplits | https://github.com/0xSplits/splits-contracts | Tek ödemeyi birden çok alıcıya böler | Her bacak ve tutar zincirde public |
| Zink | https://github.com/KaranSinghBisht/zink | Fatura başına taze Orchard adres, ZIP-321, UFVK ile eşleme | Tek alıcı. Platform ile müşteriyi ayırmaz |
| zcash-delivery-proof | https://github.com/saplingcash/zcash-delivery-proof | Bir notanın değerini ve memosunu, cüzdanı açmadan kanıtlar | Kütüphane. İki notalı ödeme akışı yok |

Ayırıcı cümle: Bölücü sözleşmeler her tutarı yayınlar; Zink tek alıcıyı izler. Sidefee tek shielded işlemde platformun yalnız kendi notasını görmesini sağlar.

### Puan

| Ölçüt | Puan | Ağırlık | Ağırlıklı |
|---|---|---|---|
| Özgünlük | 4.3 | 25% | 1.08 |
| Hackathon uyumu | 4.7 | 20% | 0.94 |
| Yapılabilirlik | 3.2 | 20% | 0.64 |
| Teknik derinlik | 4.2 | 15% | 0.63 |
| Demo | 4.5 | 10% | 0.45 |
| Track uyumu | 4.6 | 10% | 0.46 |
| **Toplam** | | | **4.20** |

## Aday 3 — Quiet Quote

Private Markets.

Bir cümle: Satıcı fiyatı tek bir alıcının shielded adresine memo olarak yazar; fiyat hiçbir taahhüt olarak zincire çıkmaz, dolum alıcının kendi ödemesidir.

Kullanıcı ve an: İki kişi ZEC karşılığı bir şey için OTC konuşuyor. Emir defterine yazmak fiyatı herkese gösterir. Commit-reveal ise süre bitince her teklifi açar.

Çekirdek döngü:

1. Satıcı, alıcının unified adresine küçük bir shielded tx atar. Memo fiyat ve vade taşır.
2. Public sayfa yalnız "teklif var" der. Fiyat yok, boyut yok.
3. Alıcı memo'yu kendi cüzdanında açar.
4. Alıcı tam o tutarda ikinci bir shielded ödeme yapar. Memo teklif kimliğini taşır.
5. Satıcı ekranı "doldu" der. Yanlış tutar eşleşmez ve fiyat açıklanmaz.

Track metni: "fiyat kamuya, taraflar değil." Burada fiyat, işlem bitince bile kamu değildir. Satıcı isterse fiyatı sonradan söyler. Zincir söyleyemez.

Demo: İki cüzdan, bir sayfa, explorer'da iki shielded tx. 80 saniye.

Risk: İki cüzdanın testnet senkronu. Küçültme: tek makinede iki light client. Eşleşme sunucusu yok.

Dürüst sınır: Satıcı memo'yu başkasına okutabilir. Gizlilik, zincire ve üçüncü bir operatöre karşıdır. Satıcıya karşı değildir. Jüriye böyle söylenir.

### Önceki işler

| Proje | Link | Ne yapıyor | Fark |
|---|---|---|---|
| Renegade | https://github.com/renegade-fi/renegade | Arbitrum'da MPC ile dark pool. Relayer düz metin emri görür, eşleşme devre ile kanıtlanır | Başka zincir, operatör ağı, sözleşme. Tek memo değil |
| Nightpool | https://github.com/KaranSinghBisht/nightpool | Midnight'ta kapalı zarf ve RFQ. Kaybeden teklif zincire açılmaz, uzlaşma ZK | Operatör turu temizler. Zcash notası kullanmaz |
| THORChain ZEC | https://crypto-economy.com/thorchain-completes-zec-integration/ | Native ZEC takası | Yalnız transparent. Tutar ve adres public |

Ayırıcı cümle: Renegade ve Nightpool eşleşmeyi operatör ve devre ile yapar. Quiet Quote'ta fiyat tek bir shielded memo'da kalır ve dolum, alıcının cüzdanından giden normal bir Zcash ödemesidir.

### Puan

| Ölçüt | Puan | Ağırlık | Ağırlıklı |
|---|---|---|---|
| Özgünlük | 4.0 | 25% | 1.00 |
| Hackathon uyumu | 4.6 | 20% | 0.92 |
| Yapılabilirlik | 3.3 | 20% | 0.66 |
| Teknik derinlik | 3.8 | 15% | 0.57 |
| Demo | 4.3 | 10% | 0.43 |
| Track uyumu | 4.6 | 10% | 0.46 |
| **Toplam** | | | **4.04** |

## Aday 4 — Sealed Lot

Private Markets.

Bir cümle: Bir açık artırmada kaybeden tekliflerin tutarı shielded notanın içinde kalır ve tur bitince de açılmaz.

Kullanıcı ve an: Bir bağış veya lot için teklif veren kişi. Commit-reveal turunda kaybedenler de fiyatını yayınlamak zorunda kalır.

Çekirdek döngü: satış ilanı public açıklama taşır, teklifler ihale adresine encrypted memo olarak gider, ihale sahibi yerelde açar, sayfa yalnız kazanan fiyatı yazar, kaybeden tutar hiçbir exportta yoktur. Uzlaşma, kazananın ödediği shielded tx'tir. Emanet sözleşmesi yok.

Dürüst sınır: İhale sahibi tüm memoları görür. ZecVault'taki gibi FROST ile "ihale sahibi tek başına çekemez" demek bu süreye sığmaz ve o iş yapıldı.

### Önceki işler

| Proje | Link | Ne yapıyor | Fark |
|---|---|---|---|
| ZecVault | https://github.com/Ridwannurudeen/zecvault | 2-of-3 FROST shielded emanet. Mainnet'te release ve refund | Teklif yok. Emanet var |
| Nightpool | https://github.com/KaranSinghBisht/nightpool | Kaybeden teklif hiç açılmayan kapalı zarf turu | Midnight, operatör, ZK üyelik kanıtı |
| Program the Blockchain, sealed-bid | https://programtheblockchain.com/posts/2018/03/20/writing-a-sealed-bid-auction-contract/ | Zincir üstü kapalı zarf, reveal ile her teklif açılır | Reveal turu kaybeden fiyatı da yayınlar |

Ayırıcı cümle: Klasik kapalı zarf her teklifi reveal'da açar. Sealed Lot'ta kaybeden tutar bir Zcash notasının dışında hiç var olmaz.

Nightpool bu cümleyi başka zincirde kurduğu için puan düşürüldü. Quiet Quote daha dar ve daha kurulabilir olduğu için kısa listedeki markets adayı odur.

### Puan

| Ölçüt | Puan | Ağırlık | Ağırlıklı |
|---|---|---|---|
| Özgünlük | 3.6 | 25% | 0.90 |
| Hackathon uyumu | 4.7 | 20% | 0.94 |
| Yapılabilirlik | 3.0 | 20% | 0.60 |
| Teknik derinlik | 3.9 | 15% | 0.59 |
| Demo | 4.1 | 10% | 0.41 |
| Track uyumu | 4.5 | 10% | 0.45 |
| **Toplam** | | | **3.89** |

## Aday 5 — Till

Cross-Chain.

Bir cümle: Müşteri shielded ZEC ile bir Base faturasını öder; sayfa müşterinin ZEC adresini hiç tutmaz, satıcı USDC görür.

Kullanıcı ve an: ZEC tutan biri, USDC isteyen bir satıcıya ödeyecek. Bugün ya CEX'e çıkar ya da ZODL CrossPay'i satıcının linki olmadan kendi başına kullanır.

Çekirdek döngü: satıcı Base adresi ve USDC tutarı olan bir link verir, müşteri ZODL içinde shielded ZEC'ten o adrese intent açar, Base'de USDC gelir, ZEC adresi linkte yoktur.

### Önceki işler

| Proje | Link | Ne yapıyor | Fark |
|---|---|---|---|
| Zashi CrossPay | https://electriccoin.co/blog/private-cross-chain-payments-with-zashi-crosspay/ | Shielded ZEC'ten NEAR destekli bir varlığa, cüzdanın içinden | Satıcının gömeceği bir link değil |
| Zaygent | https://github.com/ZecHub/zechub/tree/main/Hackathon/2026/zaygent | CrossPay ile trading agent | Merchant faturası değil |
| THORChain ZEC | https://crypto-economy.com/thorchain-completes-zec-integration/ | Native ZEC takası | Transparent. Track'in "unshield etmeden" cümlesini karşılamaz |

Ayırıcı cümle: CrossPay cüzdanın içinde biter. Till, satıcının paylaştığı faturayı o akışa bağlar ve ZEC adresini satıcıya göstermez.

Bu, 8 günde ikinci ağ, intent API ve likidite istediği için kısa listede değil. Deep link'in public olup olmadığı UNVERIFIED. Yarı shielded bir köprü, privacy ölçütünde elenebilir.

### Puan

| Ölçüt | Puan | Ağırlık | Ağırlıklı |
|---|---|---|---|
| Özgünlük | 3.8 | 25% | 0.95 |
| Hackathon uyumu | 4.5 | 20% | 0.90 |
| Yapılabilirlik | 2.9 | 20% | 0.58 |
| Teknik derinlik | 3.4 | 15% | 0.51 |
| Demo | 4.0 | 10% | 0.40 |
| Track uyumu | 4.4 | 10% | 0.44 |
| **Toplam** | | | **3.78** |

## Puan tablosu

| Aday | Track | Toplam |
|---|---|---|
| Leakline | Core & Tooling | 4.43 |
| Sidefee | Shielded Payments | 4.20 |
| Quiet Quote | Private Markets | 4.04 |
| Sealed Lot | Private Markets | 3.89 |
| Till | Cross-Chain | 3.78 |

3.5 altı yok. Hepsi duruyor. Kısa liste ilk üç.

## Kısa liste

1. Leakline. Araç. Yayın öncesi gizlilik kapısı.
2. Sidefee. Ürün. Platform komisyonu, müşteri tutarı gizli.
3. Quiet Quote. Ürün. Fiyat memo'da kalır, dolum normal bir shielded ödemedir.

## Öneri

Leakline.

Sekiz gün ve tek kişi varsayımıyla çekirdek döngü faucet, lightwalletd ve Halo2 proof beklemiyor. Yine de gerçek işlemlere bakıyor. Jürinin ilk ölçütü "sızıntı eler". Track'in kendi cümlesi "sonraki geliştiricinin teşekkür edeceği araç." Grand prize için zayıf kalan yer kullanım hikâyesi: bu bir cüzdan değil, bir kapı. Pazartesi kullanan kişi Zcash entegrasyonu yazan geliştirici.

Sidefee daha iyi bir ürün hikâyesi. Yapılabilirlik 3.2, çünkü çok çıktılı shielded işlem bu ekip için yeni. Quiet Quote track metnine en yakın pazar işi. İki cüzdan senkronu aynı riski taşıyor.

Üçü de birbirinin parçası değil. Biri seçilecek. İkisi birden teslim edilemez.

## Karar

Kullanıcı Leakline, Sidefee veya Quiet Quote seçer. Dördüncü ve beşinci aday da açık. Seçimden önce iskelet, sözleşme ve deploy yok.

```text
Decision: pending user choice
```
