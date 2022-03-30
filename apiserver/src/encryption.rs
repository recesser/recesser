use std::io::{Read, Write};
use std::path::Path;

use anyhow::Result;
use ring::aead::{self, Aad, LessSafeKey, Nonce, UnboundKey, CHACHA20_POLY1305};
use ring::rand::{self, SecureRandom};

pub const KEY_LEN: usize = 32;
pub const NONCE_LEN: usize = aead::NONCE_LEN;

struct SecretBox {
    nonce: [u8; NONCE_LEN],
    content: Vec<u8>,
}

impl SecretBox {
    fn new(rng: &dyn SecureRandom, content: Vec<u8>) -> Result<Self> {
        let nonce = generate_random_nonce(rng)?;
        Ok(Self { nonce, content })
    }

    fn encrypt(&mut self, key_bytes: &[u8; KEY_LEN]) -> Result<()> {
        let key = construct_key(key_bytes)?;
        let nonce = Nonce::assume_unique_for_key(self.nonce);
        key.seal_in_place_append_tag(nonce, Aad::empty(), &mut self.content)?;
        Ok(())
    }

    fn decrypt(&mut self, key_bytes: &[u8; KEY_LEN]) -> Result<&[u8]> {
        let key = construct_key(key_bytes)?;
        let nonce = Nonce::assume_unique_for_key(self.nonce);
        let plaintext = key.open_in_place(nonce, Aad::empty(), &mut self.content)?;
        Ok(plaintext)
    }

    fn from_slice(input: &[u8]) -> Result<Self> {
        Ok(Self {
            nonce: input[..NONCE_LEN].try_into()?,
            content: input[NONCE_LEN..].try_into()?,
        })
    }

    fn into_token(self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.nonce);
        let mut content = self.content;
        buf.append(&mut content);
        buf
    }
}

fn construct_key(key_bytes: &[u8]) -> Result<LessSafeKey> {
    let unbound_key = UnboundKey::new(&CHACHA20_POLY1305, key_bytes)?;
    Ok(LessSafeKey::new(unbound_key))
}

fn generate_random_nonce(rng: &dyn SecureRandom) -> Result<[u8; NONCE_LEN]> {
    let mut nonce_bytes = [0; NONCE_LEN];
    rng.fill(&mut nonce_bytes)?;
    Ok(nonce_bytes)
}

pub fn generate_random_key(rng: &dyn SecureRandom) -> Result<[u8; KEY_LEN]> {
    Ok(rand::generate(rng)?.expose())
}

pub fn encrypt_file(
    rng: &dyn SecureRandom,
    file_path: &Path,
    key_bytes: &[u8; KEY_LEN],
) -> Result<()> {
    let mut file_content = Vec::new();
    let mut file = std::fs::File::open(&file_path)?;
    file.read_to_end(&mut file_content)?;

    let mut secret_box = SecretBox::new(rng, file_content)?;
    secret_box.encrypt(key_bytes)?;

    let mut file = std::fs::File::create(&file_path)?;
    file.write_all(&secret_box.into_token())?;

    Ok(())
}

pub fn decrypt_file(file_path: &Path, key_bytes: &[u8; KEY_LEN]) -> Result<()> {
    let mut file_content = Vec::new();
    let mut file = std::fs::File::open(&file_path)?;
    file.read_to_end(&mut file_content)?;

    let mut secret_box = SecretBox::from_slice(&file_content)?;
    let plaintext = secret_box.decrypt(key_bytes)?;

    let mut file = std::fs::File::create(&file_path)?;
    file.write_all(plaintext)?;

    Ok(())
}
