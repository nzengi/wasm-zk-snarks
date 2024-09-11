use std::time::{SystemTime, UNIX_EPOCH};

/// UNIX zaman damgasını döndüren yardımcı fonksiyon
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Zaman geri gitmiş olamaz")
        .as_secs()
}
