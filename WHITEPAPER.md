# VERUM (VRM) — Whitepaper v1.0

> *"Gerçeği gizlemeden ispat et."*

**Sürüm:** 1.0  
**Tarih:** Haziran 2026  
**Ticker:** VRM  
**Toplam Arz:** 21.000.000 VRM  

---

## İçindekiler

1. [Özet](#1-özet)
2. [Problem](#2-problem)
3. [Çözüm — VERUM Protokolü](#3-çözüm--verum-protokolü)
4. [Teknik Mimari](#4-teknik-mimari)
5. [Konsensüs: PoAI](#5-konsensüs-poai)
6. [Gizlilik Katmanı: ZK-Shield](#6-gizlilik-katmanı-zk-shield)
7. [Gerçek Dünya Köprüsü: RealAnchor](#7-gerçek-dünya-köprüsü-realanchor)
8. [Yapay Zeka Katmanı: NeuralCore](#8-yapay-zeka-katmanı-neuralcore)
9. [Tokenomics](#9-tokenomics)
10. [Yol Haritası](#10-yol-haritası)
11. [Güvenlik](#11-güvenlik)
12. [Takım Vizyonu](#12-takım-vizyonu)

---

## 1. Özet

**VERUM**, mevcut kripto para sistemlerinin dört temel sorununu aynı anda çözmek için tasarlanmış, dünyada bir benzeri olmayan bir blockchain protokolüdür.

VERUM'un farkı şudur: Gizliliği ihlal etmeden doğruluğu kanıtlar. Yapay zeka ile kendi kendini yönetir. Gerçek dünya varlıklarını yerel olarak zincire bağlar. Ve hiçbir insan müdahalesine ihtiyaç duymadan evrilir.

**Tek cümlede:** VERUM, matematiğin gücüyle gerçeği kanıtlayan, yapay zekanın zekasıyla kendini yöneten, dünyanın ilk özerk ve gizli blockchain ekosistemidir.

---

## 2. Problem

### 2.1 Mevcut Sistemlerin Kırılma Noktaları

Bugünkü kripto para ekosistemi dört temel çelişkiyle boğuşmaktadır:

| Problem | Bitcoin/Ethereum | VERUM |
|---------|-----------------|-------|
| Gizlilik vs. Şeffaflık | Birini seç | İkisi birden |
| Enerji tüketimi | Çok yüksek (PoW) | Minimal (PoAI) |
| Gerçek dünya bağlantısı | Köprü gerekir | Protokol katmanı |
| Yönetim | İnsan oyu | AI + ZK oyu |

### 2.2 Neden Şimdiye Kadar Çözülmedi?

Çünkü bu dört sorunu **aynı anda** çözmek için üç teknolojinin eş zamanlı olgunlaşması gerekiyordu:

- Sıfır Bilgi İspatları (ZK-Proofs) — 2022+ itibariyle pratik
- Büyük Dil Modelleri (LLM) — 2023+ itibariyle dağıtık çalışabilir
- Post-Quantum Kriptografi — 2024+ NIST standardı

VERUM bu üçünü ilk kez tek bir protokolde birleştirir.

---

## 3. Çözüm — VERUM Protokolü

VERUM dört katmanlı bir mimari üzerine inşa edilmiştir:

```
┌─────────────────────────────────────────────┐
│              UYGULAMA KATMANI               │
│     (Cüzdan, DApp'ler, RealAnchor API)      │
├─────────────────────────────────────────────┤
│              ZK-SHIELD KATMANI              │
│   (Sıfır Bilgi İspatı — Gizlilik Zırh)     │
├─────────────────────────────────────────────┤
│              NEURALCORE KATMANI             │
│     (AI Konsensüs — PoAI Mekanizması)       │
├─────────────────────────────────────────────┤
│           BLOCKCHAIN ÇEKİRDEĞİ              │
│    (Rust tabanlı — Post-Quantum güvenli)    │
└─────────────────────────────────────────────┘
```

---

## 4. Teknik Mimari

### 4.1 Temel Teknoloji Yığını

- **Dil:** Rust (bellek güvenliği, hız)
- **Kriptografi:** Dilithium-3 (post-quantum imza), Kyber-1024 (anahtar değişimi)
- **ZK sistemi:** Plonky2 (hızlı özyinelemeli ZK-proof)
- **AI Framework:** Dağıtık inference — llama.cpp uyumlu
- **Konsensüs:** PoAI (Proof of AI — VERUM özgün)
- **VM:** WebAssembly tabanlı akıllı sözleşme ortamı

### 4.2 Blok Yapısı

```
VRM Bloğu {
  header: {
    önceki_hash:     [u8; 32],
    zaman_damgası:   u64,
    ai_imzaları:     Vec<AINodeSig>,
    zk_kök:          ZKRoot,
    quantum_proof:   DilithiumSig,
  },
  işlemler:          Vec<Transaction>,
  realanchor_kayıtları: Vec<RealAnchorEntry>,
  ai_karar_logu:     AIDecisionLog,
}
```

### 4.3 İşlem Gizliliği

Her işlem üç aşamadan geçer:

1. **Gizle:** Miktar ve alıcı ZK-proof ile şifrelenir
2. **Kanıtla:** "Bu işlem geçerlidir" matematiksel olarak ispatlanır
3. **Yayınla:** Sadece ispat zincire yazılır, gerçek veri değil

---

## 5. Konsensüs: PoAI

### 5.1 Proof of AI Nedir?

PoAI, dünyada ilk kez bir blockchain'in bloklarını insan madenciler veya stake sahipleri yerine **dağıtık yapay zeka nodeları** tarafından onaylandığı bir konsensüs mekanizmasıdır.

### 5.2 Nasıl Çalışır?

```
Adım 1: İşlem havuzuna işlem gelir
         ↓
Adım 2: 21 AI node işlemi bağımsız değerlendirir
         ↓
Adım 3: Her node kendi ZK-imzasıyla oy verir
         ↓
Adım 4: 14/21 çoğunluk sağlanırsa blok onaylanır
         ↓
Adım 5: Onay süreci zincire yazılır
```

### 5.3 AI Node'ları Kim Çalıştırır?

- İlk 21 node: VERUM Vakfı
- Yıl 2 sonrası: Topluluk node'ları (stake + AI model gereksinimi)
- Node olmak için: Minimum 10.000 VRM stake + geçerli AI model çalıştırma

### 5.4 Neden PoW/PoS'tan Üstün?

| Özellik | PoW | PoS | PoAI |
|---------|-----|-----|------|
| Enerji | ❌ Çok yüksek | ✅ Düşük | ✅ Minimal |
| Merkezileşme riski | ❌ Yüksek | ❌ Orta | ✅ Düşük |
| 51% saldırı | ❌ Mümkün | ❌ Mümkün | ✅ Matematiksel engel |
| Özerk yönetim | ❌ | ❌ | ✅ |

---

## 6. Gizlilik Katmanı: ZK-Shield

### 6.1 Temel Prensip

ZK-Shield, **sıfır bilgi ispatı** (Zero-Knowledge Proof) teknolojisini kullanarak bir işlemin doğruluğunu, işlemin kendisini açıklamadan kanıtlar.

Örnek:
> "Bu cüzdanda en az 100 VRM var" → KANITLANIR  
> "Bu cüzdanda kaç VRM var?" → BİLİNMEZ

### 6.2 Kimlik Doğrulama (ZK-Identity)

Kişi kimliğini kanıtlamak için:
- Gerçek kimlik bilgilerini açıklamak ZORUNDA DEĞİL
- "Ben gerçek bir insanım ve 18 yaşın üzerindeyim" → ZK-proof ile kanıtlanır
- Hangi ülkeden olduğu → İSTEĞE BAĞLI açıklanır

### 6.3 Gizli Akıllı Sözleşmeler

Sözleşme koşulları gizlenebilir:
- Hangi koşul sağlandığında ne olacağı → Sadece taraflar bilir
- Koşul gerçekleşti mi? → ZK-proof ile herkes doğrulayabilir

---

## 7. Gerçek Dünya Köprüsü: RealAnchor

### 7.1 Problem

Mevcut blockchain sistemleri fiziksel dünyayla bağlantı kurmak için **oracle** ya da **bridge** kullanır. Bu merkezi noktalar güvenlik açığı oluşturur.

### 7.2 VERUM'un Çözümü

RealAnchor, gerçek dünya varlıklarını zincire **native protokol** olarak bağlar:

```
Desteklenen Varlık Türleri:
├── Gayrimenkul (tapu kaydı hash'i)
├── Şirket hisseleri (ticaret sicili hash'i)
├── Entelektüel mülk (patent/telif hash'i)
├── Alacak senetleri
└── Emtia (altın, gümüş sertifikaları)
```

### 7.3 Doğrulama Süreci

1. Varlık sahibi belgeleri ZK-proof ile zincire yükler
2. AI nodeları belge geçerliliğini değerlendirir
3. 3 bağımsız doğrulayıcı onaylar
4. Varlık token haline gelir (RealAnchor Token)

---

## 8. Yapay Zeka Katmanı: NeuralCore

### 8.1 Özerk Protokol Yönetimi

NeuralCore, VERUM protokolünü insan müdahalesi olmadan yönetir:

- **Güvenlik:** Anormal işlem örüntüsü tespit → Otomatik dondurma
- **Ölçekleme:** Ağ yoğunluğuna göre blok boyutu otomatik ayarı
- **Güncellemeler:** Kod açığı tespit → Otomatik yama önerisi + topluluk oyu

### 8.2 Self-Healing Smart Contracts

```
Akıllı Sözleşme Yaşam Döngüsü:
Deploy → Çalış → Anomali Tespit → Dondur → Analiz → Güncelle → Devam
```

Eğer sözleşmede bir açık tespit edilirse:
- Otomatik dondurulur
- NeuralCore analiz eder
- Düzeltme önerir
- Kullanıcı onayıyla güncellenir

---

## 9. Tokenomics

### 9.1 Toplam Arz

**21.000.000 VRM** (sabit — Bitcoin gibi deflasyoner)

### 9.2 Dağılım

```
┌─────────────────────────────────────────┐
│  Topluluk & Ekosistem    %40 — 8.4M VRM │
│  AI Node Ödülleri        %20 — 4.2M VRM │
│  Vakıf Rezervi           %15 — 3.15M VRM│
│  Erken Yatırımcılar      %10 — 2.1M VRM │
│  Takım (4 yıl kilit)     %10 — 2.1M VRM │
│  Likidite Havuzu         %05 — 1.05M VRM│
└─────────────────────────────────────────┘
```

### 9.3 VRM Kullanım Alanları

| Kullanım | Detay |
|----------|-------|
| İşlem ücreti | Her işlemde küçük VRM yakar (deflasyon) |
| AI Node stake | Node çalıştırmak için 10.000 VRM stake |
| Yönetim oyu | Protokol kararlarında oy hakkı |
| ZK-Identity | Kimlik doğrulama ücreti |
| RealAnchor | Varlık tokenlaştırma ücreti |

### 9.4 Deflasyon Mekanizması

Her işlemde ücretin **%50'si yakılır**. Zaman geçtikçe:
- Toplam arz azalır
- Talep arttıkça değer artar
- Maksimum 21M, fiilen daha az olacak

---

## 10. Yol Haritası

### Faz 1 — Kuruluş (2026 Q3)
- [ ] Blockchain çekirdeği (Rust)
- [ ] Temel konsensüs (PoAI v1)
- [ ] Cüzdan uygulaması (beta)
- [ ] Whitepaper yayını

### Faz 2 — Testnet (2026 Q4)
- [ ] Public testnet lansmanı
- [ ] ZK-Shield entegrasyonu
- [ ] İlk 21 AI node aktivasyonu
- [ ] Hata ödül programı (bug bounty)

### Faz 3 — Mainnet (2027 Q1)
- [ ] Mainnet lansmanı
- [ ] Exchange listelemeleri
- [ ] RealAnchor v1 (gayrimenkul)
- [ ] Mobil cüzdan

### Faz 4 — Ekosistem (2027 Q2-Q4)
- [ ] NeuralCore tam aktivasyon
- [ ] Topluluk AI nodeları
- [ ] DApp geliştirici SDK
- [ ] Kurumsal ortaklıklar

### Faz 5 — Olgunluk (2028+)
- [ ] Post-quantum tam geçiş
- [ ] Self-healing sözleşmeler
- [ ] Küresel RealAnchor ağı
- [ ] 1M+ aktif cüzdan hedefi

---

## 11. Güvenlik

### 11.1 Tehdit Modeli

| Tehdit | Koruma |
|--------|--------|
| 51% saldırısı | 21 AI node + ZK çoğunluk |
| Quantum bilgisayar | Dilithium-3 + Kyber-1024 |
| Akıllı sözleşme açığı | Self-healing + NeuralCore |
| Kimlik sahteciliği | ZK-Identity |
| Gizli veri sızıntısı | ZK-Shield |

### 11.2 Bağımsız Denetim

Mainnet öncesi 3 bağımsız güvenlik denetimi yapılacaktır.

---

## 12. Takım Vizyonu

VERUM, merkezi bir şirket değil, **bir protokoldür**.

Hedef: 2028 yılına kadar tamamen topluluğa devredilmiş, NeuralCore tarafından yönetilen, hiçbir insanın tek başına kontrol edemediği bir sistem kurmak.

> *"VERUM, gerçeğin sahibi değildir. Gerçeğin kendisidir."*

---

## Dipnotlar ve Referanslar

- Groth16 ZK-Proof sistemi — Groth (2016)
- Plonky2 — Polygon Labs (2022)
- CRYSTALS-Dilithium — NIST PQC Standardı (2024)
- CRYSTALS-Kyber — NIST PQC Standardı (2024)
- Proof of AI konsensüs — VERUM Protokol Araştırması (2026, özgün)

---

*Bu belge VERUM Protokolü v1.0 için hazırlanmıştır.*  
*Teknik detaylar geliştirme sürecinde güncellenebilir.*
