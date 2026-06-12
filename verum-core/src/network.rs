use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Deserialize, Serialize};
use crate::block::Block;
use crate::transaction::Transaction;
use crate::blockchain::Blockchain;

// P2P mesaj protokolü
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    // Yeni işlem yayını
    NewTransaction(Transaction),
    // Yeni blok yayını
    NewBlock(Block),
    // Zincir isteği (yeni node bağlandığında)
    RequestChain,
    // Zincir yanıtı
    ResponseChain(Vec<Block>),
    // Node tanıtımı
    Handshake { node_id: String, version: String },
    // Ping/Pong
    Ping,
    Pong,
}

impl Message {
    pub fn to_bytes(&self) -> Vec<u8> {
        let json = serde_json::to_string(self).unwrap_or_default();
        let len = json.len() as u32;
        let mut bytes = len.to_be_bytes().to_vec();
        bytes.extend(json.as_bytes());
        bytes
    }

    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        serde_json::from_slice(data).ok()
    }
}

#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub address: String,
    pub node_id: String,
}

pub struct P2PNode {
    pub node_id: String,
    pub port: u16,
    pub peers: Arc<Mutex<Vec<PeerInfo>>>,
    pub blockchain: Arc<Mutex<Blockchain>>,
}

impl P2PNode {
    pub fn new(node_id: &str, port: u16, blockchain: Arc<Mutex<Blockchain>>) -> Self {
        P2PNode {
            node_id: node_id.to_string(),
            port,
            peers: Arc::new(Mutex::new(Vec::new())),
            blockchain,
        }
    }

    // TCP sunucuyu başlat — gelen bağlantıları dinle
    pub async fn start(&self) {
        let addr = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&addr).await.expect("Port açılamadı");
        println!("🌐 Node {} dinliyor: {}", self.node_id, addr);

        let blockchain = Arc::clone(&self.blockchain);
        let peers = Arc::clone(&self.peers);
        let node_id = self.node_id.clone();

        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, peer_addr)) => {
                        println!("🔗 Yeni bağlantı: {}", peer_addr);
                        let bc = Arc::clone(&blockchain);
                        let ps = Arc::clone(&peers);
                        let nid = node_id.clone();
                        tokio::spawn(async move {
                            handle_connection(stream, bc, ps, nid).await;
                        });
                    }
                    Err(e) => eprintln!("Bağlantı hatası: {}", e),
                }
            }
        });
    }

    // Başka bir node'a bağlan
    pub async fn connect_to_peer(&self, peer_addr: &str) -> bool {
        match TcpStream::connect(peer_addr).await {
            Ok(mut stream) => {
                // Handshake gönder
                let msg = Message::Handshake {
                    node_id: self.node_id.clone(),
                    version: "0.1.0".to_string(),
                };
                if stream.write_all(&msg.to_bytes()).await.is_ok() {
                    println!("🤝 {} → {} bağlantısı kuruldu", self.node_id, peer_addr);
                    self.peers.lock().unwrap().push(PeerInfo {
                        address: peer_addr.to_string(),
                        node_id: "unknown".to_string(),
                    });
                    true
                } else {
                    false
                }
            }
            Err(_) => {
                println!("❌ {} adresine bağlanılamadı", peer_addr);
                false
            }
        }
    }

    // Tüm peerlara mesaj yayınla
    pub async fn broadcast(&self, message: &Message) {
        let peers = self.peers.lock().unwrap().clone();
        let msg_bytes = message.to_bytes();

        for peer in &peers {
            if let Ok(mut stream) = TcpStream::connect(&peer.address).await {
                let _ = stream.write_all(&msg_bytes).await;
            }
        }

        let peer_count = peers.len();
        if peer_count > 0 {
            println!("📡 Yayın yapıldı → {} peer", peer_count);
        }
    }

    // Yeni işlemi yayınla
    pub async fn broadcast_transaction(&self, tx: Transaction) {
        let msg = Message::NewTransaction(tx.clone());
        self.broadcast(&msg).await;
        println!("📤 İşlem yayınlandı: {}...  ", &tx.id[..8]);
    }

    // Yeni bloğu yayınla
    pub async fn broadcast_block(&self, block: &Block) {
        let msg = Message::NewBlock(block.clone());
        self.broadcast(&msg).await;
        println!("📦 Blok #{} yayınlandı", block.header.index);
    }

    pub fn peer_count(&self) -> usize {
        self.peers.lock().unwrap().len()
    }
}

// Gelen bağlantıyı işle
async fn handle_connection(
    mut stream: TcpStream,
    blockchain: Arc<Mutex<Blockchain>>,
    _peers: Arc<Mutex<Vec<PeerInfo>>>,
    _node_id: String,
) {
    let mut len_buf = [0u8; 4];

    loop {
        // Önce 4 byte uzunluk oku
        if stream.read_exact(&mut len_buf).await.is_err() {
            break;
        }

        let msg_len = u32::from_be_bytes(len_buf) as usize;
        if msg_len == 0 || msg_len > 10_000_000 {
            break;
        }

        let mut msg_buf = vec![0u8; msg_len];
        if stream.read_exact(&mut msg_buf).await.is_err() {
            break;
        }

        if let Some(message) = Message::from_bytes(&msg_buf) {
            handle_message(message, &blockchain, &mut stream).await;
        }
    }
}

async fn handle_message(
    message: Message,
    blockchain: &Arc<Mutex<Blockchain>>,
    stream: &mut TcpStream,
) {
    match message {
        Message::Handshake { node_id, version } => {
            println!("🤝 Handshake alındı — Node: {} (v{})", node_id, version);
            // Zinciri gönder
            let chain = blockchain.lock().unwrap().chain.clone();
            let response = Message::ResponseChain(chain);
            let _ = stream.write_all(&response.to_bytes()).await;
        }

        Message::NewTransaction(tx) => {
            println!("📨 Yeni işlem alındı: {}... ({:.2} VRM)", &tx.id[..8], tx.amount);
            let mut bc = blockchain.lock().unwrap();
            bc.add_transaction(tx);
        }

        Message::NewBlock(block) => {
            println!("📦 Yeni blok alındı: #{} ({}...)", block.header.index, &block.hash[..8]);
            let mut bc = blockchain.lock().unwrap();
            let last_hash = bc.chain.last().map(|b| b.hash.clone()).unwrap_or_default();
            if block.header.previous_hash == last_hash {
                bc.chain.push(block);
                println!("✅ Blok zincire eklendi");
            } else {
                println!("⚠️  Blok reddedildi — zincir uyumsuz");
            }
        }

        Message::RequestChain => {
            let chain = blockchain.lock().unwrap().chain.clone();
            let response = Message::ResponseChain(chain);
            let _ = stream.write_all(&response.to_bytes()).await;
        }

        Message::ResponseChain(chain) => {
            println!("⛓️  Zincir alındı — {} blok", chain.len());
            let mut bc = blockchain.lock().unwrap();
            if chain.len() > bc.chain.len() {
                bc.chain = chain;
                println!("✅ Zincir güncellendi (daha uzun zincir kabul edildi)");
            }
        }

        Message::Ping => {
            let _ = stream.write_all(&Message::Pong.to_bytes()).await;
        }

        Message::Pong => {
            println!("🏓 Pong alındı");
        }
    }
}
