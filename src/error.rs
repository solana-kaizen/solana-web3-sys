use thiserror::Error;
use wasm_bindgen::prelude::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0:?}")]
    JsValue(JsValue),
}

unsafe impl Send for Error {}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Self::JsValue(value)
    }
}
