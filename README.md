# VERUM (VRM)

> *"Gerçeği gizlemeden ispat et."*

**VERUM**, AI konsensüsü (PoAI), sıfır-bilgi gizliliği (ZK-Shield) ve gerçek
dünya varlık bağlama (RealAnchor) konseptlerini birleştiren deneysel bir
blockchain protokolü ve ERC-20 tokenıdır.

## 🟢 Canlı — Sepolia Testnet

| | |
|---|---|
| **Token** | VERUM (VRM) — ERC-20 |
| **Sözleşme** | [`0x4b1b5B57504C7269847f2593D77d354Ad72e84C9`](https://eth-sepolia.blockscout.com/token/0x4b1b5B57504C7269847f2593D77d354Ad72e84C9) |
| **Toplam arz** | 21.000.000 VRM (sabit — mint fonksiyonu yok) |
| **Deflasyon** | Transfer ücretinin %50'si yakılır — [zincirde kanıtlandı](https://eth-sepolia.blockscout.com/tx/0x5448e4747700dad0c2b70aa11287ec2967b8565482ed49d28b9d6489c7497f8c) |
| **Kaynak kod** | Gezginde doğrulanmış ✅ |

## Depo Yapısı

```
verum/
├── WHITEPAPER.md      # Protokol whitepaper'ı (v1.0)
├── verum-token/       # ERC-20 sözleşmesi (Solidity + OpenZeppelin v5)
│   ├── contracts/     #   VerumToken.sol — 21M sabit arz, yakma mekanizması
│   └── test/          #   14 birim test (Hardhat)
└── verum-core/        # Kendi L1 zincir prototipi (Rust)
    └── src/           #   PoAI konsensüs, P2P ağ, ZK-Shield, cüzdan, web paneli
```

## verum-token — ERC-20 Sözleşmesi

Whitepaper §9 tokenomics'inin birebir uygulaması:

- **21.000.000 VRM sabit arz** — sonradan basım imkânsız
- **%0,1 transfer ücreti** → yarısı yakılır (arz kalıcı azalır), yarısı hazineye
- **%1 ücret tavanı koda gömülü** — sözleşme sahibi bile aşamaz
- `Ownable2Step` + `ERC20Permit`, OpenZeppelin v5 tabanı

```bash
cd verum-token
npm install
npm test        # 14 birim test
```

## verum-core — Rust L1 Prototipi

Gelecekteki bağımsız VERUM ağının çalışan prototipi:

- **PoAI konsensüs** — 7 AI node simülasyonu, %67 çoğunluk
- **P2P ağ** — TCP, deterministik genesis, blok yayını ve senkronizasyon
- **ZK-Shield** — commitment/nullifier tabanlı gizli işlem simülasyonu
- **Cüzdan** — secp256k1 + ECDSA, Base58Check `VRM` adresleri
- **Web paneli** — blok gezgini + cüzdan arayüzü

```bash
cd verum-core
cargo test                 # birim testler
cargo run                  # 3-node P2P + ZK-Shield demosu
cargo run --bin verum-web  # http://127.0.0.1:3000 — web paneli
```

## Yol Haritası Durumu

- [x] Whitepaper v1.0
- [x] Rust çekirdek prototipi (konsensüs, P2P, ZK, cüzdan, web)
- [x] ERC-20 sözleşmesi + birim testler
- [x] Sepolia testnet dağıtımı + kaynak doğrulama
- [x] Yakma mekanizmasının zincir üstünde doğrulanması
- [ ] Fuzz testleri (Foundry)
- [ ] Bağımsız güvenlik denetimi
- [ ] Mainnet + likidite

## Sorumluluk Reddi

Bu proje **deneysel ve eğitim amaçlıdır**. Token şu an yalnızca Sepolia test
ağında yaşamaktadır ve hiçbir parasal değeri yoktur. PoAI ve ZK-Shield
bileşenleri kavram kanıtı (proof-of-concept) simülasyonlarıdır. Hiçbir içerik
yatırım tavsiyesi değildir.

## Lisans

MIT
