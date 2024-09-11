fn main() {
    println!("wasm-zk-snarks projesi çalışıyor!");

    // Örnek olarak kriptografik bir anahtar oluştur ve zaman damgasını al
    let (key, timestamp) = wasm_zk_snarks::keygen::generate_key_with_timestamp();

    println!("Oluşturulan Anahtar: {:?}", key);
    println!("Zaman Damgası: {}", timestamp);

    // Anahtarı temizle
    let mut key_to_clear = key;
    wasm_zk_snarks::keygen::clear_key(&mut key_to_clear);
    println!("Temizlenen Anahtar: {:?}", key_to_clear);
}
