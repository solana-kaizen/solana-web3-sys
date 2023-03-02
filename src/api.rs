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
    pub type GetProgramAccountsConfig;

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug)]
    pub type ProgramAccountsResultItem;

    #[wasm_bindgen(getter, method)]
    pub fn account(this: &ProgramAccountsResultItem) -> ProgramAccount;

    #[wasm_bindgen(getter, method, js_name = "pubkey")]
    pub fn pubkey_impl(this: &ProgramAccountsResultItem) -> PublicKey;

}

impl OptionsTrait for GetProgramAccountsConfig {}

impl GetProgramAccountsConfig {
    pub fn add_filters(self, filters: Vec<GetProgramAccountsFilter>) -> Result<Self> {
        let list = Array::new();
        for filter in filters {
            list.push(&filter.try_into()?);
        }
        Ok(self.set("filters", list.into()))
    }

    pub fn encoding(self, encoding: UiAccountEncoding) -> Result<Self> {
        Ok(self.set("encoding", encoding.into()))
    }

    pub fn data_slice(self, data_slice: UiDataSliceConfig) -> Result<Self> {
        Ok(self.set("dataSlice", data_slice.try_into()?))
    }

    pub fn commitment(self, commitment: CommitmentConfig) -> Result<Self> {
        Ok(self.set("commitment", commitment.commitment.to_string().into()))
    }

    pub fn min_context_slot(self, min_context_slot: Slot) -> Result<Self> {
        Ok(self.set("minContextSlot", min_context_slot.into()))
    }

    pub fn with_context(self, _with_context: bool) -> Result<Self> {
        //self.with_context = Some(with_context);
        Ok(self)
    }

    pub fn build(self) -> Result<Self> {
        Ok(self)
    }
}

pub enum UiAccountEncoding {
    Base58,
    Base64,
}

impl From<UiAccountEncoding> for String {
    fn from(value: UiAccountEncoding) -> Self {
        match value {
            UiAccountEncoding::Base58 => "Base58".to_string(),
            UiAccountEncoding::Base64 => "Base64".to_string(),
        }
    }
}
impl From<UiAccountEncoding> for JsValue {
    fn from(value: UiAccountEncoding) -> Self {
        String::from(value).to_lowercase().into()
    }
}

pub struct UiDataSliceConfig {
    pub offset: usize,
    pub length: usize,
}

impl TryFrom<UiDataSliceConfig> for JsValue {
    type Error = crate::error::Error;

    fn try_from(value: UiDataSliceConfig) -> Result<Self> {
        let obj = Object::new();
        Reflect::set(&obj, &JsValue::from("offset"), &JsValue::from(value.offset))?;
        Reflect::set(&obj, &JsValue::from("length"), &JsValue::from(value.length))?;
        Ok(obj.into())
    }
}

#[derive(Debug, Clone)]
pub enum GetProgramAccountsFilter {
    /// Memory comparison filter using offset and base58 encoded string
    MemcmpEncodedBase58(usize, String),

    /// Memory comparison filter using offset and base64 encoded string
    MemcmpEncodedBase64(usize, String),

    /// Memory comparison filter using offset and bytes which will be encoded as base58
    MemcmpEncodeBase58(usize, Vec<u8>),

    /// Memory comparison filter using offset and bytes which will be encoded as base64
    MemcmpEncodeBase64(usize, Vec<u8>),

    /// Data size comparison filter
    DataSize(usize),
}

fn create_memcmp_filter(
    holder: &Object,
    offset: usize,
    data: String,
    encoding: &str,
) -> Result<()> {
    let memcmp = Object::new();
    Reflect::set(&memcmp, &JsValue::from("offset"), &JsValue::from(offset))?;
    Reflect::set(&memcmp, &JsValue::from("bytes"), &JsValue::from(data))?;
    Reflect::set(
        &memcmp,
        &JsValue::from("encoding"),
        &JsValue::from(encoding),
    )?;
    Reflect::set(holder, &JsValue::from("memcmp"), &memcmp.into())?;

    Ok(())
}

impl TryFrom<GetProgramAccountsFilter> for JsValue {
    type Error = crate::error::Error;
    fn try_from(value: GetProgramAccountsFilter) -> Result<Self> {
        let obj = Object::new();
        match value {
            GetProgramAccountsFilter::MemcmpEncodedBase58(offset, data) => {
                create_memcmp_filter(&obj, offset, data, "base58")?;
            }
            GetProgramAccountsFilter::MemcmpEncodedBase64(offset, data) => {
                create_memcmp_filter(&obj, offset, data, "base64")?;
            }
            GetProgramAccountsFilter::MemcmpEncodeBase58(offset, bytes) => {
                let data = bs58::encode(bytes).into_string();
                create_memcmp_filter(&obj, offset, data, "base58")?;
            }
            GetProgramAccountsFilter::MemcmpEncodeBase64(offset, bytes) => {
                let data = base64::encode(bytes);
                create_memcmp_filter(&obj, offset, data, "base64")?;
            }
            GetProgramAccountsFilter::DataSize(data_size) => {
                Reflect::set(&obj, &JsValue::from("dataSize"), &JsValue::from(data_size))?;
            }
        }

        Ok(obj.into())
    }
}

impl ProgramAccountsResultItem {
    pub fn pubkey(&self) -> Result<Pubkey> {
        self.pubkey_impl().try_into()
    }
}
