use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, generic_array::GenericArray},
    Aes256Gcm, // Можно взять 128/256
    Key, Nonce // 96-битный уникальный nonce
};
use rand::RngCore;
use std::error::Error;

/// Шифрование секрета
///
/// - `plain_secret` — исходный секрет пользователя (в открытом виде)
/// - `master_key` — ваш SALT_KEY из `.env`
///
/// Возвращаем `ciphertext` в hex-формате, внутри которого
/// будет nonce (12 байт) + зашифрованные данные.
pub fn encrypt_secret(plain_secret: &str, master_key: &str) -> Result<String, Box<dyn Error>> {
    // 1. Из master_key (string) получаем 32 байта (Aes256 требует 32 byte key)
    // Упрощённый подход: берём SHA-256 от master_key
    let key_hash = blake3::hash(master_key.as_bytes());
    let key_bytes = key_hash.as_bytes(); // 32 bytes
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);

    let cipher = Aes256Gcm::new(key);

    // 2. Генерируем случайный nonce (12 байт)
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 3. Шифруем
    let ciphertext = cipher.encrypt(nonce, plain_secret.as_bytes()).unwrap(); //TODO PROPER ERRORS

    // 4. Формируем результирующую строку:
    // Первые 12 байт -> nonce
    // Далее зашифрованные данные
    // Всё это закодируем в hex
    let mut result = Vec::new();
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);

    Ok(hex::encode(result))
}

/// Расшифровка секрета
///
/// - `encrypted_hex` — результат `encrypt_secret`
/// - `master_key` — ваш SALT_KEY из `.env`
pub fn decrypt_secret(encrypted_hex: &str, master_key: &str) -> Result<String, Box<dyn Error>> {
    let data = hex::decode(encrypted_hex).unwrap(); //TODO PROPER ERRORS

    // 1. Выделяем первые 12 байт под nonce
    if data.len() < 12 {
        return Err("Ciphertext too short".into());
    }
    let nonce_bytes = &data[..12];
    let ciphertext = &data[12..];

    // 2. Генерируем ключ
    let key_hash = blake3::hash(master_key.as_bytes());
    let key_bytes = key_hash.as_bytes(); // 32 bytes
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);

    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    // 3. Дешифруем
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap(); //TODO PROPER ERRORS
    let secret_str = String::from_utf8(plaintext); //TODO PROPER ERRORS

    Ok(secret_str.unwrap())
}
