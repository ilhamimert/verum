# VERUM Token (VRM) — ERC-20 Sözleşmesi

Whitepaper v1.0 tokenomics'inin Ethereum üzerinde gerçek uygulaması.
Bu, VERUM'un **listelenebilirlik yolundaki ilk gerçek adımıdır** — Rust
çekirdeği (verum-core) gelecekteki kendi ana ağımızın prototipi olarak
paralel geliştirilmeye devam eder.

## Sözleşme Özellikleri

| Özellik | Değer | Whitepaper Karşılığı |
|---------|-------|----------------------|
| İsim / Sembol | VERUM / VRM | §1 |
| Toplam arz | 21.000.000 VRM (sabit, mint fonksiyonu yok) | §9.1 |
| Deflasyon | Transfer ücretinin %50'si yakılır | §9.4 |
| Başlangıç ücreti | %0.1 (10 baz puan) | §9.3 |
| Ücret üst sınırı | %1 — sahip bile aşamaz (kod sabiti) | güven önlemi |
| Standartlar | ERC-20 + ERC-20 Permit (imzayla onay) | — |
| Altyapı | OpenZeppelin v5 (denetimden geçmiş kütüphane) | §11.2 |

## Adım Adım Testnet Dağıtımı (Remix — kurulum gerektirmez)

1. **MetaMask kur** (metamask.io) ve bir cüzdan oluştur.
2. MetaMask'te ağı **Sepolia testnet**'e çevir (Ayarlar → Ağlar → Sepolia'yı göster).
3. Ücretsiz test ETH al: `https://sepoliafaucet.com` veya `https://faucets.chain.link`
4. Tarayıcıda **remix.ethereum.org** aç.
5. `contracts/VerumToken.sol` dosyasını Remix'e kopyala
   (OpenZeppelin importlarını Remix otomatik indirir).
6. **Solidity Compiler** sekmesi → sürüm `0.8.24+` seç → **Compile**.
7. **Deploy & Run** sekmesi → Environment: `Injected Provider — MetaMask`.
8. Constructor parametreleri:
   - `initialHolder`: kendi MetaMask adresin (tüm arz buraya basılır)
   - `treasury_`: vakıf adresi (test için yine kendi adresin olabilir)
9. **Deploy** → MetaMask'te onayla.
10. Sözleşme adresini kaydet, `https://sepolia.etherscan.io` üzerinde görüntüle.
11. Etherscan'de **Verify & Publish** ile kaynak kodu doğrula
    (şeffaflık — listing için zorunlu sayılır).

## Tokenomics Dağıtımı (deploy sonrası yapılacak transferler)

| Havuz | Pay | Miktar | Nasıl |
|-------|-----|--------|-------|
| Topluluk & Ekosistem | %40 | 8.400.000 VRM | Çoklu imza cüzdanına (Gnosis Safe) |
| AI Node Ödülleri | %20 | 4.200.000 VRM | Ödül sözleşmesine (ileride) |
| Vakıf Rezervi | %15 | 3.150.000 VRM | Treasury adresine |
| Erken Yatırımcılar | %10 | 2.100.000 VRM | Yatırımcı geldiğinde |
| Takım (4 yıl kilit) | %10 | 2.100.000 VRM | OpenZeppelin **VestingWallet** ile |
| Likidite Havuzu | %5 | 1.050.000 VRM | DEX havuzuna (Uniswap) |

**Takım kilidi:** Özel kod yazma — OpenZeppelin'in hazır `VestingWallet`
sözleşmesini deploy et (4 yıl = `duration: 126144000` saniye), takım VRM'sini
oraya gönder. Denetimden geçmiş hazır kod > el yazması kilit.

**Önemli:** VestingWallet ve DEX havuzu adreslerini `setFeeExempt(adres, true)`
ile ücretten muaf tut — fee-on-transfer tokenlar bazı DEX işlemlerinde sorun
çıkarabilir.

## Listelenmeye Giden Gerçekçi Yol

1. ✅ Token sözleşmesi (bu repo)
2. ⏳ Sepolia testnet'te deploy + Etherscan doğrulaması
3. ⏳ Birim testler (Foundry/Hardhat) + test coverage
4. ⏳ Bağımsız güvenlik denetimi (CertiK, Hacken vb. — 10-50k$)
5. ⏳ Hukuki yapı: şirket/vakıf kuruluşu, Türkiye SPK kripto mevzuatına uyum
6. ⏳ Mainnet deploy + Uniswap'ta likidite havuzu (LP token kilidi ile)
7. ⏳ CoinGecko / CoinMarketCap kayıtları (ücretsiz başvuru)
8. ⏳ Merkezi borsa başvuruları (önce küçük borsalar, sonra BTCTurk/Midas gibi)

> Not: Midas dahil hiçbir borsa "yükleme" kabul etmez — coin zaten yaşıyor
> ve likit olmalı, borsa kendi ticari/hukuki süreciyle listeler.

## Güvenlik Notları

- Mint fonksiyonu YOK — arz hiçbir koşulda 21M'yi aşamaz, sadece azalır.
- Ücret tavanı %1 koda gömülü — sahip cüzdanı ele geçirilse bile
  kullanıcılar %1'den fazla ücrete maruz kalamaz.
- `Ownable2Step`: sahiplik devri iki aşamalı — yanlış adrese devir riski yok.
- Mainnet öncesi sahipliği çoklu imza cüzdanına (Gnosis Safe) devret.
