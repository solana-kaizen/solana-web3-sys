//!
//! Misc. bindings for utility classes (`RpcProgramAccountsConfig` etc.)
//!

use crate::imports::*;
use js_sys::Reflect;
//use workflow_log::log_trace;
use crate::account::ProgramAccount;
use crate::publickey::PublicKey;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::slot_history::Slot;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug)]
    pub type RpcProgramAccountsConfig;

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug)]
    pub type ProgramAccountsResultItem;

    #[wasm_bindgen(getter, method)]
    pub fn account(this: &ProgramAccountsResultItem) -> ProgramAccount;

    #[wasm_bindgen(getter, method, js_name = "pubkey")]
    pub fn pubkey_impl(this: &ProgramAccountsResultItem) -> PublicKey;

}

impl OptionsTrait for RpcProgramAccountsConfig {}

impl RpcProgramAccountsConfig {
    pub fn add_filters(self, filters: Array) -> Result<Self> {
        Ok(self.set("filters", filters.into()))
    }

    pub fn encoding(self, encoding: RpcAccountEncoding) -> Result<Self> {
        Ok(self.set("encoding", encoding.into()))
    }

    pub fn data_slice(self, data_slice: RpcDataSliceConfig) -> Result<Self> {
        Ok(self.set("dataSlice", data_slice.try_into()?))
    }

    pub fn commitment(self, commitment: CommitmentConfig) -> Result<Self> {
        Ok(self.set("commitment", commitment.commitment.to_string().into()))
    }

    pub fn min_context_slot(self, min_context_slot: Slot) -> Result<Self> {
        Ok(self.set("minContextSlot", min_context_slot.into()))
    }
}

pub enum RpcAccountEncoding {
    Base58,
    Base64,
}

impl From<RpcAccountEncoding> for String {
    fn from(value: RpcAccountEncoding) -> Self {
        match value {
            RpcAccountEncoding::Base58 => "Base58".to_string(),
            RpcAccountEncoding::Base64 => "Base64".to_string(),
        }
    }
}
impl From<RpcAccountEncoding> for JsValue {
    fn from(value: RpcAccountEncoding) -> Self {
        String::from(value).to_lowercase().into()
    }
}

pub struct RpcDataSliceConfig {
    pub offset: usize,
    pub length: usize,
}

impl TryFrom<RpcDataSliceConfig> for JsValue {
    type Error = crate::error::Error;

    fn try_from(value: RpcDataSliceConfig) -> Result<Self> {
        let obj = Object::new();
        Reflect::set(&obj, &JsValue::from("offset"), &JsValue::from(value.offset))?;
        Reflect::set(&obj, &JsValue::from("length"), &JsValue::from(value.length))?;
        Ok(obj.into())
    }
}

impl ProgramAccountsResultItem {
    pub fn pubkey(&self) -> Result<Pubkey> {
        self.pubkey_impl().try_into()
    }
}
