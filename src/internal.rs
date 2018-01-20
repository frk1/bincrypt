extern crate base64;
extern crate rust_sodium;

use std::fs::File;
use std::io::prelude::*;

use base64::{decode, encode};
use failure::Error;
use rust_sodium::crypto::secretbox;
use rust_sodium::crypto::secretbox::{Key, Nonce, NONCEBYTES};

/// Custom Error Type
#[derive(Debug, Fail)]
enum BincryptError {
    /// Could not read nonce
    #[fail(display = "invalid nonce")]
    InvalidNonce,

    /// Could not decrypt
    #[fail(display = "decryption failed")]
    FailedDecryption,
}

/// Convert base64 encoded key
pub fn transform_key(key: &str) -> Option<Key> {
    let key = decode(key).ok()?;
    Key::from_slice(&key)
}

/// Generate new key and write the base64 encoded version to key.txt and stdout
pub fn generate_key() -> Key {
    let write_to_file = |k: &Key| -> Result<(), Error> {
        let mut buffer = File::create("key.txt")?;
        let kstr = encode(&k.0);
        write!(buffer, "{}", kstr)?;
        println!("Randomly generated key: {}", kstr);
        Ok(())
    };

    let key = secretbox::gen_key();
    write_to_file(&key).unwrap_or_else(|err| println!("{}", err));
    key
}

/// Encrypt file using `XSalsa20Poly1305` with a randomly generated `Nonce`.
/// The `Nonce` will be saved as the first `NONCEBYTES` of the output file.
pub fn encrypt_file(path_input: &str, path_output: &str, key: &Key) -> Result<(), Error> {
    let mut file_input = File::open(path_input)?;

    let mut buffer = Vec::new();
    file_input.read_to_end(&mut buffer)?;

    let nonce = secretbox::gen_nonce();
    let cipher = secretbox::seal(&buffer, &nonce, key);

    let mut file_output = File::create(path_output)?;
    file_output.write_all(&nonce.0)?;
    file_output.write_all(&cipher)?;
    Ok(())
}

/// Decrypt file encrypted with `XSalsa20Poly1305`.
/// If the file already exists it will be overwritten!
pub fn decrypt_file(path_input: &str, path_output: &str, key: &Key) -> Result<(), Error> {
    let mut file_input = File::open(path_input)?;

    let mut nonce_buffer = [0u8; NONCEBYTES];
    file_input.read_exact(&mut nonce_buffer)?;
    let nonce = Nonce::from_slice(&nonce_buffer).ok_or(BincryptError::InvalidNonce)?;

    let mut buffer = Vec::new();
    file_input.read_to_end(&mut buffer)?;

    let plain = secretbox::open(&buffer, &nonce, key).or(Err(BincryptError::FailedDecryption))?;

    let mut file_output = File::create(path_output)?;
    file_output.write_all(&plain)?;
    Ok(())
}
