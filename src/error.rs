//!
//! [`Error`] variants produced by this crate.
//!
use solana_program::pubkey::ParsePubkeyError;
use wasm_bindgen::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0:?}")]
    JsValue(JsValue),

    #[error("ParsePubkeyError: {0:?}")]
    ParsePubkeyError(#[from] ParsePubkeyError),
}

unsafe impl Send for Error {}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Self::JsValue(value)
    }
}
