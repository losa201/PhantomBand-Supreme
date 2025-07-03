
use rand::Rng;
use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce // Or `XChaCha20Poly1305`, `XNonce`
};
use chacha20poly1305::aead::{Aead, NewAead};
use log::{info, error};

pub fn generate_keypair() -> [u8; 32] {
    let mut rng = rand::thread_rng();
    let mut key = [0u8; 32];
    rng.fill(&mut key);
    info!("Generated new keypair.");
    key
}

pub fn encrypt(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, String> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let mut rng = rand::thread_rng();
    let mut nonce_bytes = [0u8; 12]; // 96-bit nonce for ChaCha20Poly1305
    rng.fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    cipher.encrypt(nonce, data)
        .map(|mut ciphertext| {
            let mut result = nonce_bytes.to_vec();
            result.append(&mut ciphertext);
            info!("Data encrypted successfully.");
            result
        })
        .map_err(|e| {
            error!("Encryption error: {:?}", e);
            format!("Encryption error: {:?}", e)
        })
}

pub fn decrypt(encrypted_data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, String> {
    if encrypted_data.len() < 12 {
        error!("Ciphertext too short to contain nonce.");
        return Err("Ciphertext too short to contain nonce".to_string());
    }

    let nonce_bytes = &encrypted_data[..12];
    let ciphertext = &encrypted_data[12..];

    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher.decrypt(nonce, ciphertext)
        .map_err(|e| {
            error!("Decryption error: {:?}", e);
            format!("Decryption error: {:?}", e)
        })
}
