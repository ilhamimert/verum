use std::sync::{Arc, Mutex};
use verum_core::blockchain::Blockchain;
use verum_core::wallet::Wallet;
use verum_core::network::P2PNode;
use verum_core::zk_shield::{generate_proof, ShieldedPool};

#[tokio::main]
async fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║     VERUM (VRM) Blockchain v0.1.0        ║");
    println!("║     Proof of AI Consensus — Testnet      ║");
    println!("║     P2P Network Demo                     ║");
    println!("╚══════════════════════════════════════════╝\n");

    // --- CÜZDANLAR ---
    println!("🔑 Cüzdanlar olusturuluyor...\n");
    let alice = Wallet::new();
    let bob   = Wallet::new();

    println!("👤 Alice : {}", alice.info.address);
    println!("👤 Bob   : {}\n", bob.info.address);

    // --- 3 BAGIMSIZ NODE OLUSTUR ---
    println!("🌐 P2P Nodelar baslatiliyor...\n");

    let bc1 = Arc::new(Mutex::new(Blockchain::new()));
    let bc2 = Arc::new(Mutex::new(Blockchain::new()));
    let bc3 = Arc::new(Mutex::new(Blockchain::new()));

    let node1 = Arc::new(P2PNode::new("VRM-NODE-1", 8001, Arc::clone(&bc1)));
    let node2 = Arc::new(P2PNode::new("VRM-NODE-2", 8002, Arc::clone(&bc2)));
    let node3 = Arc::new(P2PNode::new("VRM-NODE-3", 8003, Arc::clone(&bc3)));

    // Nodeları başlat (arka planda dinle)
    node1.start().await;
    node2.start().await;
    node3.start().await;

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // --- PEER BAĞLANTILARI ---
    println!("\n🔗 Peer bağlantıları kuruluyor...\n");
    node1.connect_to_peer("127.0.0.1:8002").await;
    node1.connect_to_peer("127.0.0.1:8003").await;
    node2.connect_to_peer("127.0.0.1:8003").await;

    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    println!("\n📊 Peer Durumu:");
    println!("   Node-1 peers: {}", node1.peer_count());
    println!("   Node-2 peers: {}", node2.peer_count());
    println!("   Node-3 peers: {}", node3.peer_count());

    // --- ISLEM OLUSTUR VE YAYINLA ---
    println!("\n📤 Node-1 üzerinden işlem yayınlanıyor...\n");

    let tx1 = alice.create_transaction(&bob.info.address, 500.0);
    println!("✍️  Alice → Bob: 500 VRM");
    println!("   İmza: {}...", &tx1.signature[..16]);

    // Node-1 işlemi blockchain'e ekle
    bc1.lock().unwrap().add_transaction(tx1.clone());

    // Tüm peerlara yayınla
    node1.broadcast_transaction(tx1).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    // --- BLOK MADENCILIGI ---
    println!("\n⛏️  Node-1 blok olusturuyor (PoAI)...");
    let mined = bc1.lock().unwrap().mine_pending_transactions().is_some();

    if mined {
        let block = bc1.lock().unwrap().chain.last().cloned();
        if let Some(blk) = block {
            println!("📡 Yeni blok tüm ağa yayınlanıyor...");
            node1.broadcast_block(&blk).await;
        }
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    // --- SONUC ---
    println!("\n╔══════════════════════════════════════════╗");
    println!("║              AĞ DURUMU                   ║");
    println!("╚══════════════════════════════════════════╝");

    let len1 = bc1.lock().unwrap().block_count();
    let len2 = bc2.lock().unwrap().block_count();
    let len3 = bc3.lock().unwrap().block_count();

    println!("   Node-1 zincir uzunlugu: {} blok", len1);
    println!("   Node-2 zincir uzunlugu: {} blok", len2);
    println!("   Node-3 zincir uzunlugu: {} blok", len3);

    let valid1 = bc1.lock().unwrap().is_valid();
    println!("\n   Node-1 zincir gecerli: {}", if valid1 { "EVET ✅" } else { "HAYIR ❌" });

    println!("\n✅ VERUM P2P Agi calisiyor!");
    println!("   3 node birbirine bagli ve iletisim kuruyor\n");

    // --- ZK-SHIELD: GIZLI ISLEMLER ---
    println!("╔══════════════════════════════════════════╗");
    println!("║         ZK-SHIELD — Gizli İşlemler       ║");
    println!("╚══════════════════════════════════════════╝\n");

    let mut pool = ShieldedPool::new();

    // 1. Alice 1000 VRM'yi gizli havuza kilitler
    println!("🛡️  Alice 1000 VRM'yi shield ediyor (gizli havuza kilitliyor)...");
    let (note, alice_secret) = pool
        .shield(1000.0, &alice.info.address)
        .expect("shield başarısız");
    println!("   Zincirde görünen commitment: {}...", &note.commitment[..24]);
    println!("   Havuzda kilitli: {} VRM\n", pool.total_locked);

    // 2. Alice, Bob'a gizli transfer yapar — miktar zincire hiç yazılmaz
    println!("🕶️  Alice → Bob gizli transfer (miktar görünmez)...");
    let proof = generate_proof(&alice_secret);
    let (bob_note, bob_secret) = pool
        .transfer(&alice_secret, &proof, &bob.info.address)
        .expect("gizli transfer başarısız");
    println!("   Yeni commitment: {}...", &bob_note.commitment[..24]);
    println!("   Gözlemcinin gördüğü: sadece 2 commitment, 1 nullifier — miktar YOK\n");

    // 3. Alice aynı notu tekrar harcamaya çalışır → çift harcama engellenir
    println!("🚫 Alice aynı notu tekrar harcamayı deniyor...");
    match pool.transfer(&alice_secret, &proof, &alice.info.address) {
        Err(e) => println!("   Reddedildi: {} ✅\n", e),
        Ok(_) => println!("   HATA: çift harcama engellenemedi! ❌\n"),
    }

    // 4. Bob notunu unshield eder — açık VRM'ye geri döner
    println!("🔓 Bob notunu unshield ediyor...");
    let bob_proof = generate_proof(&bob_secret);
    match pool.unshield(&bob_secret, &bob_proof) {
        Ok(amount) => println!("   Bob {} VRM'yi açık bakiyesine aldı ✅", amount),
        Err(e) => println!("   Unshield başarısız: {} ❌", e),
    }
    println!("   Havuzda kalan kilitli: {} VRM", pool.total_locked);

    println!("\n✅ ZK-Shield calisiyor: shield → gizli transfer → unshield\n");
}
