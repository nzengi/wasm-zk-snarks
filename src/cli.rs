use structopt::StructOpt;
use crate::keygen::{generate_key_with_timestamp, KeyWithExpiration};

/// CLI için komut satırı seçenekleri
#[derive(StructOpt)]
pub struct Cli {
    /// Anahtarın süresinin ne kadar olacağını belirtir
    #[structopt(short, long, default_value = "60")]
    pub expires_in: u64,
}

pub fn run_cli() {
    let args = Cli::from_args();
    let (key, created_at) = generate_key_with_timestamp();
    let key_with_expiration = KeyWithExpiration {
        key,
        created_at,
        expires_in: args.expires_in,
    };

    println!("Anahtar: {:?}", key_with_expiration.key);
    println!("Oluşturma zamanı: {}", key_with_expiration.created_at);
    println!("Anahtar geçerlilik süresi: {} saniye", key_with_expiration.expires_in);

    if key_with_expiration.is_expired() {
        println!("Anahtarın süresi dolmuş.");
    } else {
        println!("Anahtar geçerli.");
    }
}
