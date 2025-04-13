use aes::Aes256;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use block_modes::{BlockMode, Cbc};
use block_padding::Pkcs7;
use hostname;
use std::error::Error;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

fn get_host_name() -> String {
    hostname::get()
        .map(|host_name| host_name.to_string_lossy().into_owned())
        .unwrap_or_else(|_| String::from("unknown_host"))
}

fn fill_to_byte_length(value: &str, length: usize) -> Vec<u8> {
    let mut result = value.as_bytes().to_vec();

    while result.len() < length {
        result.extend_from_slice(value.as_bytes());
    }

    result.truncate(length);

    result
}

fn get_key_and_iv() -> (Vec<u8>, Vec<u8>) {
    let host_name = get_host_name();

    let key = fill_to_byte_length(&host_name, 32);
    let iv = key[..16].to_vec();

    (key, iv)
}

pub fn aes_encrypt(value: &str) -> String {
    if value.is_empty() {
        return String::new();
    }

    let (key, iv) = get_key_and_iv();
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).expect("cipher の生成に失敗しました。");

    let encrypted = cipher.encrypt_vec(value.as_bytes());

    STANDARD.encode(encrypted)
}

pub fn aes_decrypt(value: &str) -> Result<String, Box<dyn Error>> {
    if value.is_empty() {
        return Ok(String::new());
    }

    let (key, iv) = get_key_and_iv();
    let decipher = Aes256Cbc::new_from_slices(&key, &iv).expect("decipher の生成に失敗しました。");

    let decoded = STANDARD.decode(value).map_err(|e| Box::new(e))?;
    let decrypted = decipher.decrypt_vec(&decoded)?;
    let result = String::from_utf8(decrypted).map_err(|e| Box::new(e))?;

    Ok(result)
}
