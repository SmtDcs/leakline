# Track matrisi

Ayrı sponsor yok. `data.js` altı sponsor yuvasını boş bırakmış. Ödülü koyan taraf zkSNARKs. Aşağıdaki satırlar beş track. Hesap başına tam bir track. Grand prize seçilen track'in üstüne biner, ikinci bir entegrasyon istemez.

`sponsors.yaml` içinde Zcash, NEAR Intents, ZODL veya Zebra kaydı yok. Hesap, testnet ve kanıt bu yüzden etkinlik sayfası ile resmi repolardan.

| Track | Ödül | İstenen iş | Hesap / anahtar | Testnet | Kanıt | Efor (8 gün, tek kişi) |
|---|---|---|---|---|---|---|
| Shielded Payments | $15.000 | Ödeme kimliği zincire çıkmadan tamamlanır | Yok. Cüzdan testnet shielded adres. Harcama anahtarı repoda durmaz | Zcash testnet, faucet | Explorer'da tutar yok; uygulamada fatura "ödendi" | Yüksek. Senkron ve gerçek gönderim şart |
| Private Markets | $15.000 | Fiyat kamuya, teklif sahibi ve kaybeden teklif gizli | İki testnet cüzdan | Zcash testnet | İki shielded tx; sayfada yalnızca kazanan fiyat | Yüksek. İki taraf senkronu |
| Cross-Chain | $15.000 | Başka zincire geçerken unshield etmemek | NEAR Intents hesabı UNVERIFIED. Base veya Solana cüzdanı | Zcash testnet + hedef zincirin testneti | Hedef zincirde USDC/SOL, ZEC adresi sayfada yok | 8 güne sığmaz. İkinci ağ ve likidite |
| Core & Tooling | $15.000 | Sonraki Zcash geliştiricisinin kullanacağı araç | Yok. İşlem hex'i public veri | İşlenmiş mainnet tx ile demo; bir testnet tx ile teyit | LEAK / CLEAN çıkışı, explorer'daki public alanla aynı | Sığar. Faucet demo'nun önkoşulu değil |
| Wildcard | $15.000 | Sanat, oyun, sosyal, donanım | — | — | Shielded bir işlem olmalı, yoksa eler | Açık bounty kapandı. Yeni ürün yine de girebilir |

## Bu pencere için gerçekçi track'ler

En fazla üç, ve yalnızca süre yettiği için:

1. Core & Tooling. Çekirdek döngü bir işlem çözümleyici ve bir CI çıkış kodu. Lightwalletd şart değil.
2. Shielded Payments. Yalnızca tek not, tek alıcı, tek memo. FROST ve çoklu bordro yok.
3. Private Markets. Yalnızca iki kişilik teklif. Emir defteri ve ihale motoru yok.

Cross-Chain bu sürede seçilmez. Track metni "yolda unshield etmeden" diyor. THORChain'in ZEC entegrasyonu yalnızca transparent UTXO. Zashi CrossPay (şimdi ZODL) shielded ZEC'ten NEAR destekli bir varlığa ödüyor; bu cüzdanın içinde. 8 Eylül 2026'da bir kullanıcı, Zodl içinden NEAR Intents'e çıkan 1.120 shielded ZEC karşılığı USDT'nin 50 gündür tutulduğunu yazdı. 15 Eylül'de forum, cevabın daha büyük bir köprü olmadığını yazdı. Bu track'e girmek o arızanın üstüne ikinci bir çıkış kapısı eklemek olur.

Wildcard seçilmez. $5.000 Zero to ZEC bounty ödendi. Kalan $15.000'lık track açık, fakat "beklemediğimiz bir şey" ölçütü çalışan bir ödeme veya araçdan daha zayıf duruyor.

## Jürinin göreceği kanıt

Fikir seçilmeden hash yok.

- Core: ham tx hex girilir, araç `LEAK` veya `CLEAN` der, explorer aynı tx'te tutarın görünüp görünmediğini gösterir.
- Payments: ZIP-321 istek, testnet shielded ödeme, uygulamada durum değişir, explorer tutarı göstermez.
- Markets: teklif memosu yalnızca alıcıda açılır, dolum tek shielded tx'tir, kaybeden fiyat hiçbir sayfada yoktur.

## Elenen yönler

| Yön | Neden |
|---|---|
| FROST 2-of-3 emanet | ZecVault bunu mainnet'te yayınladı. CYZE ve Konclave aynı seremoniyi yaptı |
| Viewing key ile fatura masası | Zink bunu yaptı: fatura başına taze Orchard adres, ZIP-321, memo, UFVK ile eşleme |
| CrossPay trading bot | Zaygent. Cüzdan özelliğinin üstüne ajan |
| Seçili ödeme ifşası | Glasspane Rooms ve `zcash-delivery-proof` |
| ZIP-321 ön kontrol, zincir yok | ZEC Invoice Preflight. Uzlaşma mock |
| Explorer üstünde gizlilik puanı | ZecBlock (eski CipherScan) bunu zincir sonrası yapıyor |
| Mainnet bakiyesiyle demo | Hackathon OS kuralı: testnet, düşük bakiyeli cüzdan |
| İkinci track'e de girmek | Kural: hesap başına tam bir track |
