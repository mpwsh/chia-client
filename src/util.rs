use std::path::Path;

use crate::Error;
use anyhow::Result;
use reqwest::Identity;
use tokio::fs::read;
use bech32::{self, convert_bits, u5, Variant};
use hex::ToHex;

pub(crate) async fn load_pem_pair(
    key: impl AsRef<Path>,
    cert: impl AsRef<Path>,
) -> Result<Identity, Error> {
    let mut buf = read(key.as_ref()).await?;
    buf.append(&mut read(cert.as_ref()).await?);
    Ok(Identity::from_pem(&buf)?)
}

pub fn decode_puzzle_hash(puzzle_hash: &str) -> Result<String> {
    let (_hrp, data, _variant) = bech32::decode(puzzle_hash)?;
    let decoded = convert_bits(&data, 5, 8, false);
    let hex = format!("0x{}", decoded?.encode_hex::<String>());
    Ok(hex)
}

pub fn encode_puzzle_hash(puzzle_hash: &str, prefix: &str) -> Result<String> {
    let mut bytes = [0u8; 32];
    hex::decode_to_slice(puzzle_hash.replace("0x", ""), &mut bytes as &mut [u8])?;
    let bits: Vec<u5> = convert_bits(&bytes, 8, 5, true)?
        .iter()
        .map(|b| u5::try_from_u8(*b).expect("Unable to convert u8 to u5"))
        .collect();
    let encoded = bech32::encode(prefix, bits, Variant::Bech32m)?;
    Ok(encoded)
}
