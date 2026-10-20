# Hackathon Plan

## Demo sentence

Bir geliştirici, henüz yayınlanmamış bir Zcash işlemini Leakline'a verir. Araç "sızıyor" ya da "temiz" der ve buna göre programdan çıkar.

## Seçim

Leakline. Track: Core & Tooling. Kullanıcı 2026-10-20'de seçti.

Etkinlik 2026-10-28 23:59 UTC'de kapanır. Kullanılabilir emek dar kabul edilmiyor. Bu, diğer fikirleri geri getirmez. Sidefee Zodl'da kırılıyor ve tutarı ele veriyor. Till havuzdan çıkıyor. Quiet Quote ve Sealed Lot emanet olmadan pazar iddiası kuruyor. Fazla zaman, Leakline'ın ince halini değil tam kontrolünü kurmak için.

## Scope

### In scope

1. Rust kütüphanesi ve komut satırı. Girdi bir PCZT ya da imzalı işlem. Anahtar kabul edilmez.
2. Karar: `CLEAN` ya da `LEAK`, sabit gerekçe kodu, çıkış kodu 0 ya da 1.
3. Havuzlar arası açık bakiye. Sapling, Orchard veya transparent arasında tutar düz metinse `LEAK`. Transparent parça yok diye `CLEAN` denmez.
4. `zcashd` politika adları: `FullPrivacy` ve `AllowRevealed*` ailesi. Aynı işlem, seçilen politikaya göre geçebilir ya da kalabilir.
5. Ham işlemde para üstü tahmini yok. Para üstü ayrımı yalnız PCZT bunu söylüyorsa yapılır.
6. Bilinmeyen işlem sürümü `LEAK`.
7. Sabit örnek işlemler ve bunları beklenen kararla karşılaştıran testler.
8. Demo, örneklerin tarayıcıda ya da komut satırında gösterilmesi. İşlem bir sunucuya gitmez.

### Out of scope

- Cüzdan, ödeme sayfası, ihale, köprü, faucet.
- İşlemi ağa göndermek.
- Viewing key, memo çözmek, IP veya lightwalletd sızıntısı aramak.
- "Bu adres bir çözücü, parayı dondurur" cümlesi.
- ShieldLint'in yaptığı iş: kaynak kodundaki adres metnini taramak.
- Ayrı bir Ironwood ayrıştırıcısı. Kütüphanenin tanımadığı sürüm kapalı başarısız olur.

Sığarsa, şart değil: ZIP-317 ücret sapması, birden fazla transparent adresin bağlanması, "bu işlem şu ZIP-321 isteğini ödüyor mu" karşılaştırması.

## Roles

| Role | Owner | Responsibility |
|---|---|---|
| Parser and policy | build | İşlem baytları ve karar kodları |
| Fixtures and tests | build | Bilinen sızan ve temiz işlemler |
| Demo | build | Sunucusuz sayfa ya da komut kaydı |
| Submit | user | thezecathon.com hesabı, repo, iki dakikalık video |

## Stack

- Chain: Zcash. Sözleşme yok.
- Language: Rust, `librustzcash` / `zcash_primitives` ve `pczt`.
- Frontend: statik sayfa, işlem sunucuya gitmez. WASM derlenemezse demo komut satırıdır.
- Wallet: yok.
- Indexing: yok.

## Timeboxes

Teslim 2026-10-28 23:59 UTC. Sıra önem sırasıdır, saat kotası değil.

| Sıra | Work | Gate |
|---|---|---|
| 1 | Kilitli bir crate sürümüyle v5 işlem okunur | Örnek hex açılır |
| 2 | Havuzlar arası bakiye ve transparent çıkış `LEAK` | Üç örnek: çıkış, havuz geçişi, yalnız Orchard |
| 3 | Politika bayrağı ve PCZT | Aynı bayt, iki politikada iki sonuç |
| 4 | Testler ve README | `cargo test` yeşil |
| 5 | Demo ve iki dakikalık video | Jüri işlemi sunucuya vermeden görür |
| 6 | Teslim | Repo public, form mühürlü |

## Risks

| Risk | Likelihood | Mitigation |
|---|---|---|
| Crate sürümleri Ironwood döneminde kayar | yüksek | Bilinen bir revizyona kilitle. Tanınmayan sürüme `LEAK` |
| WASM derlemesi şişer | orta | Yarım günden sonra bırak. Demo komut satırı kaydı olur |
| Ham işlemde girdi tutarı yazmaz | kesin | Tutar yalnız PCZT'de ya da bakiye işaretinden yazılır |
| Jüri ShieldLint ile karıştırır | orta | README ilk ekranda: o kaynak metnine bakar, bu yayınlanacak bayta bakar |

## Çift model

İşletim sistemi `hackathon/10-pair-system.md`. Karar tablosunun tek kopyası odur. Grok kodlar. DeepSeek her kapıda spec'i okur ve testi kendisi çalıştırır. Workflow: `.grok/workflows/leakline-gate.rhai`.

## Definition of done

- [ ] `cargo test` üç sınıf örneği doğru etiketler
- [ ] `LEAK` çıkış kodu 1
- [ ] Anahtar istenmez, ağ çağrısı yapılmaz
- [ ] README neyin iddia edildiğini ve edilmediğini söyler
- [ ] İki dakikalık video
- [ ] Teslim formu
- [ ] Deck yok
