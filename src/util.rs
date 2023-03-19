use std::path::Path;

use crate::Error;
use anyhow::Result;
use bech32::{self, convert_bits, u5, Variant};
use hex::ToHex;
use reqwest::Identity;
use tokio::fs::read;

#[cfg(feature = "disassemble")]
use pyo3::{prelude::*, types::IntoPyDict};

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
    let hex = decoded?.encode_hex::<String>();
    Ok(hex)
}

pub fn encode_puzzle_hash(puzzle_hash: &str, prefix: &str) -> Result<String, Error> {
    let mut bytes = [0u8; 32];
    hex::decode_to_slice(puzzle_hash.replace("0x", ""), &mut bytes as &mut [u8])?;
    let bits: Vec<u5> = convert_bits(&bytes, 8, 5, true)?
        .iter()
        .map(|b| u5::try_from_u8(*b).map_err(|_| Error::BitConversionError))
        .collect::<Result<Vec<u5>, Error>>()?;
    let encoded = bech32::encode(prefix, bits, Variant::Bech32m)?;
    Ok(encoded)
}

pub fn mojo_to_xch(amount: u64) -> f64 {
    amount as f64 / 1_000_000_000_000.0
}

#[cfg(feature = "disassemble")]
pub fn disassemble_program(program: &str) -> PyResult<String> {
    pyo3::prepare_freethreaded_python();
    let gil = Python::acquire_gil();
    let py = gil.python();

    let locals = [("program", program)].into_py_dict(py);
    py.run(
        "
from clvm_tools.binutils import disassemble
from cdv.cmds.util import parse_program

parsed_program = parse_program(program)
disassembled_program = disassemble(parsed_program)
",
        None,
        Some(locals),
    )?;
    let disassembled_program: String =
        locals.get_item("disassembled_program").unwrap().extract()?;
    Ok(disassembled_program)
}
