# Event Brief

- Event: ZECATHON, zkSNARKs tarafından düzenlenen Zcash gizlilik hackathonu
- Site: https://thezecathon.com/
- Organizatör: zkSNARKs (@zksnarks_, https://zilkroad.com). Destek: bladet928@gmail.com
- Format: çevrimiçi, asenkron. "NODE 07 — ONLINE"
- Kayıt ve teslim açılışı: 28 Eylül 2026, 00:00 UTC
- Teslim: 28 Ekim 2026, 23:59 UTC
- Kullanıcı saati varsayımı: Europe/Istanbul (UTC+3). Bu saatle son an 29 Ekim 2026, 02:59
- Bugün: 20 Ekim 2026. Kalan süre yaklaşık 8 gün
- Jüri açıklanması: 29 Ekim 2026
- Jürilik: 29 Ekim – 22 Kasım 2026
- Kazananlar: 23 Kasım 2026
- Takım: 1–5 kayıtlı kişi. Tek hesap teslim eder. Ödül o hesaba gider
- Parça: klasik. Mevcut ürün teslim edilemez
- Zincir: Zcash. EVM sözleşmesi yok. Havuzlar: transparent, Sapling, Orchard, Ironwood (NU6.3)
- Ağ: demo için Zcash testnet. `chains.yaml` içinde Zcash kaydı yok; chain id uydurulmayacak

Kaynak: sitedeki `darkpool/data.js` (20 Ekim 2026'da çekildi) ve @zksnarks_ duyuruları. Site bir terminal arayüzü; kurallar, track'ler ve takvim bu dosyada duruyor. Jüriler dosyada altı boş koltuk. Sponsor logoları bilerek boş: "bir sponsor adı, parayı kimin koyduğuna dair bir iddiadır."

## Ödüller

Toplam $100.000, ZEC olarak, kazananın shielded adresine. Adres yalnızca kazanandan, jürilikten sonra Telegram ile istenir.

| Ödül | Tutar | Ne |
|---|---|---|
| Shielded Payments | $15.000 | POS, bordro, havale, abonelik. Kim kime ödedi yayınlanmaz |
| Private Markets | $15.000 | Kapalı zarf ihale, emir akışı, uzlaşma. Fiyat kamuya, taraflar gizli |
| Cross-Chain | $15.000 | Başka zincire gidiş-geliş, yolda unshield etmeden |
| Core & Tooling | $15.000 | Zebra, librustzcash, indexer, cüzdan SDK, prover |
| Wildcard | $15.000 | Sanat, oyun, sosyal, donanım. Shielded bir işlemi yapmaya değer kılan şey |
| Grand prize | $20.000 | En iyi proje, track ödülünün üstüne |
| Community bounties | $5.000 | Build penceresinde açılan küçük hedefler |

Track metinleri sitede bu. Organizatörün 28 Eylül 2026 thread'i aynı beş başlığı ve $15K'yı ayrıca yazdı. Grand prize sitede "track ödülünün üstüne" diye duruyor.

`data.js` içinde "PLACEHOLDER SPLIT" yorumu var. Bölünme, organizatörün kendi duyurularıyla aynı olduğu için resmi kabul edildi. Yorum, dosyanın eskiden bir taslak olduğunu gösteriyor.

### $5.000 bounty durumu

Zero to ZEC yarışması 4 Ekim 2026 gecesi kapandı (11:59 PM ET). 6 Ekim'de ödendi:

- 1. @theonlywill — $2.500
- 2. @ArtofOrb — $1.500
- 3. @adamzkdev — $1.000
- Bonus @m44444nu — $500

Bu, ilan edilen $5.000 community bounty hattını dolduruyor; bonusla birlikte $5.500. Organizatör, bu tarihin yalnızca o yarışma için olduğunu ve Wildcard track'in açık kaldığını yazdı. Başka bir açık bounty sayfada yok. UNVERIFIED: pencere kapanmadan yeni bir bounty açılır mı.

## Jüri ölçütleri

Ağırlık yayınlanmadı. Dördü eşit önemde yazılmış, biri eleme.

| Ölçüt | Not |
|---|---|
| Privacy | Gerçekten gizli mi, yoksa öyle mi görünüyor. Sızıntı puan kırmaz, eler |
| Usefulness | Pazartesi biri bunu kullanır mı |
| Execution | Çalışıyor mu. Çalışan, iddialı olandan üstün |
| Originality | Bu hackathon için yazılmış mı, yoksa yeniden paketlenmiş mi |

Jüri adları 29 Ekim'e kadar boş. UNVERIFIED: kim bakacak.

## Zorunlu teslimler

Form alanları (`darkpool/forms.js`): proje adı, track, açıklama, repo URL, çalışan demo URL, iki dakikalık demo videosu URL'si.

- [ ] Build penceresinde yazılmış kod (28 Eylül 2026 00:00 UTC – 28 Ekim 2026 23:59 UTC)
- [ ] Teslimde açık kaynak. 28 Ekim'de public yapılan private repo kabul
- [ ] Tam bir track. Hesap başına bir teslim
- [ ] Çalışan demo linki
- [ ] İki dakikalık video (YouTube, Loom veya link veren herhangi bir yer)
- [ ] Mühürlü form. Teslimler jürilik açılana kadar diğer takımlara ve organizatöre kapalı. Deadline'a kadar düzenlenebilir; değişiklik için yeniden submit

Deck istenmiyor. Video isteniyor: form checklist'inde ayrı satır.

AI araçları: 18 Ekim'de @kelly_onchain sordu, görülen yanıt yok. UNVERIFIED. `AI_USAGE.md` yine de tutulacak. Ölçüt "repackaged değil" diyor; AI yasağı yazmıyor.

## Kurallar, siteden

- "One person registers and submits. The prize goes to that account; build with anyone."
- "All code written during the build window. Libraries are fine; existing products are not."
- "Open source at submission. A private repo made public on Oct 28 counts."
- "One submission per account, entered in exactly one track."
- "Submissions stay sealed until judging opens. You can edit yours until the deadline."
- "Organisers and judges cannot enter."
- "Does it actually preserve privacy, or does it only look like it does? Leaks are disqualifying, not deductions."
- "Prizes are paid in ZEC to a shielded address."

## Bu pencere için anlamı

Zcash'te genel amaçlı sözleşme yok. Ürün; not, memo, unified address, viewing key, ZIP-321 ödeme isteği ve PCZT üzerinde durmalı. FROST emanet, viewing-key ile fatura eşleme, CrossPay üstüne trading bot ve seçili ödeme ifşası ZecHub Hackathon 3.0'da (36 proje, Ağustos 2026) zaten gösterildi. Jürinin "repackaged" ölçütü bunları eleyebilir.

Kalan süre tek kişi için bir çekirdek döngü. İkinci entegrasyon yok. Faucet, lightwalletd senkronu ve Halo2 proof, demo'nun tek yolu olmamalı.

## Resmi kaynaklar

| Kaynak | Link | Not |
|---|---|---|
| Etkinlik | https://thezecathon.com/ | İçerik `darkpool/data.js` |
| Zcash docs | https://zcash.readthedocs.io/en/latest/ | Sitenin listesi. "start here" |
| ZIP'ler | https://zips.z.cash/ | Protokol |
| librustzcash | https://github.com/zcash/librustzcash | Rust çekirdek |
| Zebra | https://github.com/ZcashFoundation/zebra | Node |
| ZODL | https://github.com/zodl-inc | Referans cüzdan, eski adı Zashi |
| NEAR Intents | https://docs.near-intents.org/ | Cross-chain uzlaşma |
| Geliştirici haritası | https://raw.githubusercontent.com/ZecHub/zechub/main/site/Start_Here/Developer_Resources.md | zcashd artık başlangıç değil |

Sitedeki kaynak listesinin üstünde "STARTER LIST — confirm every one before launch" yorumu duruyor. Linkler 20 Ekim 2026'da bu araştırmada açıldı.

## Bilinçli olarak yapılmayacaklar

- Fikir seçmek veya iskelet kurmak. Karar kullanıcıda.
- Deck. İstenmiyor.
- Video. Form istiyor; fikir seçilmeden çekilmez.
- Mainnet'te değerli ZEC. Testnet.
