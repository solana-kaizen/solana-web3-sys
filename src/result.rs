//! [`Result`] wrapping the [`Error`](crate::error::Error) enum variants produced by this crate.

pub type Result<T> = std::result::Result<T, crate::error::Error>;
pub type JsResult<T> = std::result::Result<T, wasm_bindgen::JsValue>;
