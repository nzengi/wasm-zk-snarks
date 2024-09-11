use rand::rngs::OsRng;
use rand::RngCore;
use zeroize::Zeroize;
use crate::utils::current_timestamp;

/// Kriptografik olarak güvenli bir anahtar oluşturur ve zaman damgasını döndürür.
pub fn generate_key_with_timestamp() -> ([u8; 32], u64) {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);  // Güvenli anahtar üretimi
    let timestamp = current_timestamp();
    (key, timestamp)
}

/// Anahtarın bellekten güvenli bir şekilde temizlenmesi.
pub fn clear_key(key: &mut [u8; 32]) {
    key.zeroize();
}

/// Süreli anahtar yapısı
pub struct KeyWithExpiration {
    pub key: [u8; 32],
    pub created_at: u64,
    pub expires_in: u64, // Saniye cinsinden geçerlilik süresi
}

impl KeyWithExpiration {
    pub fn is_expired(&self) -> bool {
        current_timestamp() > (self.created_at + self.expires_in)
    }
}
