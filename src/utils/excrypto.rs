use std::io::{self, Error, ErrorKind};
use cmac::{Cmac, Mac};
use aes::Aes128;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{KeyIvInit, StreamCipher};

type Aes128Ctr64LE = ctr::Ctr64LE<aes::Aes128>;

pub fn encrypt_aes128_ctr(data_to_encrypt: &[u8], iv_bytes: &[u8], key: &str) -> Result<Vec<u8>, io::Error> {
    // Parse the key and iv string into a byte array
    let mut encrypted_data = data_to_encrypt.to_vec();
    let key_bytes: Vec<u8> = (0..key.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&key[i..i + 2], 16)
                .expect("Failed to parse hex string")
        })
        .collect();

    if key_bytes.len() != 16 || iv_bytes.len() != 16 {
        return Err(Error::new(ErrorKind::InvalidInput, "wrong key length"));
    }

    let mut cipher = Aes128Ctr64LE::new(GenericArray::from_slice(&key_bytes), GenericArray::from_slice(&iv_bytes));
    cipher.apply_keystream(&mut encrypted_data);

    Ok(encrypted_data)
}

pub fn decrypt_aes128_ctr(encrypted_data: &[u8], iv_bytes: &[u8], key: &str) -> Result<Vec<u8>, io::Error> {
    // Parse the key and iv string into a byte array
    let mut decrypted_data = encrypted_data.to_vec();
    let key_bytes: Vec<u8> = (0..key.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&key[i..i + 2], 16)
                .expect("Failed to parse hex string")
        })
        .collect();

    if key_bytes.len() != 16 || iv_bytes.len() != 16 {
        return Err(Error::new(ErrorKind::InvalidInput, "wrong key length"));
    }

    let mut cipher = Aes128Ctr64LE::new(GenericArray::from_slice(&key_bytes), GenericArray::from_slice(&iv_bytes));
    cipher.apply_keystream(&mut decrypted_data);

    Ok(decrypted_data)
}

pub fn encrypt_aes128_cmac(data_to_encrypt: &[u8], key: &str) -> Result<Vec<u8>, io::Error> {
    // Parse the key string into a byte array
    let key_bytes: Vec<u8> = (0..key.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&key[i..i + 2], 16)
                .expect("Failed to parse hex string")
        })
        .collect();

    if key_bytes.len() != 16 {
        return Err(Error::new(ErrorKind::InvalidInput, "wrong key length"));
    }

    // Create an AES-128-CMAC instance with the provided key
    let mut cmac = Cmac::<Aes128>::new(GenericArray::from_slice(&key_bytes));

    // Generate the CMAC tag by updating with the IV and data to encrypt
    cmac.update(data_to_encrypt);
    let result = cmac.finalize().into_bytes();

    // Return the encrypted data (CMAC tag appended to the data)
    let mut encrypted_data = Vec::new();
    encrypted_data.extend_from_slice(&result);
    Ok(encrypted_data)
}

pub fn verify_aes128_cmac(data_to_encrypt: &[u8], encrypted_data: Vec<u8>, key: &str) -> bool {
    match encrypt_aes128_cmac(data_to_encrypt, key) {
        Ok(authentication_data) => {
            if encrypted_data == authentication_data {
                return true;
            }
            eprintln!("decrypt_aes128_cmac ok but verify error with {:?}", authentication_data);
            return false;
        }
        Err(err) => {
            eprintln!("Encryption error: {}", err);
            return false;
        }
    }
}