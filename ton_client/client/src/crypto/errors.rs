use crate::error::ApiError;
use std::fmt::Display;
const CRYPTO: isize = ApiError::CRYPTO; // 100

pub enum Code {
    InvalidPublicKey = CRYPTO + 0,
    InvalidSecretKey = CRYPTO + 1,
    InvalidKey = CRYPTO + 2,
    InvalidFactorizeChallenge = CRYPTO + 6,
    InvalidBigInt = CRYPTO + 7,
    ScryptFailed = CRYPTO + 8,
    InvalidKeySize = CRYPTO + 9,
    NaclSecretBoxFailed = CRYPTO + 10,
    NaclBoxFailed = CRYPTO + 11,
    NaclSignFailed = CRYPTO + 12,
    Bip39InvalidEntropy = CRYPTO + 13,
    Bip39InvalidPhrase = CRYPTO + 14,
    Bip32InvalidKey = CRYPTO + 15,
    Bip32InvalidDerivePath = CRYPTO + 16,
    Bip39InvalidDictionary = CRYPTO + 17,
    Bip39InvalidWordCount = CRYPTO + 18,
    MnemonicGenerationFailed = CRYPTO + 19,
    MnemonicFromEntropyFailed = CRYPTO + 20,
}
pub struct Error;

fn error(code: Code, message: String) -> ApiError {
    ApiError::with_code_message(code as isize, message)
}

impl Error {
    pub fn invalid_factorize_challenge<E: Display>(hex: &String, err: E) -> ApiError {
        error(
            Code::InvalidFactorizeChallenge,
            format!(
                "Invalid factorize challenge: {}\r\nchallenge: [{}]",
                err, hex
            ),
        )
    }

    pub fn invalid_big_int(hex: &String) -> ApiError {
        error(Code::InvalidBigInt, format!("Invalid big int [{}]", hex))
    }

    pub fn scrypt_failed<E: Display>(err: E) -> ApiError {
        error(Code::ScryptFailed, format!(r#"Scrypt failed: {}"#, err))
    }

    pub fn invalid_key_size(actual: usize, expected: usize) -> ApiError {
        error(
            Code::InvalidKeySize,
            format!("Invalid key size {}. Expected {}.", actual, expected),
        )
    }

    pub fn nacl_secret_box_failed<E: Display>(err: E) -> ApiError {
        error(
            Code::NaclSecretBoxFailed,
            format!("Nacl Secret Box failed: {}", err),
        )
    }

    pub fn nacl_box_failed<E: Display>(err: E) -> ApiError {
        error(Code::NaclBoxFailed, format!("Box failed: {}", err))
    }

    pub fn nacl_sign_failed<E: Display>(err: E) -> ApiError {
        error(Code::NaclSignFailed, format!("Sign failed: {}", err))
    }

    pub fn bip39_invalid_entropy<E: Display>(err: E) -> ApiError {
        error(
            Code::Bip39InvalidEntropy,
            format!("Invalid bip39 entropy: {}", err),
        )
    }

    pub fn bip39_invalid_phrase<E: Display>(err: E) -> ApiError {
        error(
            Code::Bip39InvalidPhrase,
            format!("Invalid bip39 phrase: {}", err),
        )
    }

    pub fn bip32_invalid_key<E: Display>(key: E) -> ApiError {
        error(Code::Bip32InvalidKey, format!("Invalid bip32 key: {}", key))
    }

    pub fn bip32_invalid_derive_path<E: Display>(path: E) -> ApiError {
        error(
            Code::Bip32InvalidDerivePath,
            format!("Invalid bip32 derive path: {}", path),
        )
    }

    pub fn bip39_invalid_dictionary(dictionary: u8) -> ApiError {
        error(
            Code::Bip39InvalidDictionary,
            format!("Invalid mnemonic dictionary: {}", dictionary),
        )
    }

    pub fn bip39_invalid_word_count(word_count: u8) -> ApiError {
        error(
            Code::Bip39InvalidWordCount,
            format!("Invalid mnemonic word count: {}", word_count),
        )
    }

    pub fn invalid_secret_key<E: Display>(err: E, key: &String) -> ApiError {
        error(
            Code::InvalidSecretKey,
            format!("Invalid secret key [{}]: {}", err, key),
        )
    }

    pub fn invalid_public_key<E: Display>(err: E, key: &String) -> ApiError {
        error(
            Code::InvalidPublicKey,
            format!("Invalid public key [{}]: {}", err, key),
        )
    }

    pub fn invalid_key<E: Display>(err: E, key: &String) -> ApiError {
        error(Code::InvalidKey, format!("Invalid key [{}]: {}", err, key))
    }

    pub fn mnemonic_generation_failed() -> ApiError {
        error(
            Code::MnemonicGenerationFailed,
            "Mnemonic generation failed".into(),
        )
    }

    pub fn mnemonic_from_entropy_failed(reason: &str) -> ApiError {
        error(
            Code::MnemonicFromEntropyFailed,
            format!("Mnemonic from entropy failed: {}", reason),
        )
    }
}
