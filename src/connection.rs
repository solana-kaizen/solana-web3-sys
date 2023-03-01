//!
//! `Connection` class bindings.
//!
use crate::imports::*;
use js_sys::Reflect;
use workflow_log::log_trace;
//use solana_program::account::AccountInfo;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=solanaWeb3, js_name = Connection)]
    #[derive(Debug, Clone)]
    pub type Connection;

    #[wasm_bindgen(constructor, js_namespace=["solanaWeb3"])]
    /// Create Connection
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/Connection.html)
    ///
    pub fn new(endpoint: String) -> Connection;

    #[wasm_bindgen(constructor, js_namespace=["solanaWeb3"])]
    /// Create Connection
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/Connection.html)
    ///
    pub fn new_with_commitment(endpoint: String, commitment: String) -> Connection;

    #[wasm_bindgen(method, catch, js_name = "getLatestBlockhash")]
    /// Fetch the latest blockhash from the cluster
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/Connection.html#getLatestBlockhash)
    ///
    pub async fn get_latest_block_hash_impl(this: &Connection) -> Result<JsValue>;

    #[wasm_bindgen(method, catch, js_name = "getLatestBlockhash")]
    /// Fetch the latest blockhash from the cluster
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/Connection.html#getLatestBlockhash)
    ///
    pub async fn get_latest_block_hash_with_commitment(
        this: &Connection,
        commitment: String,
    ) -> Result<JsValue>;

    #[wasm_bindgen(method, catch, js_name = "sendRawTransaction")]
    /// Send a transaction that has already been signed and serialized into the wire format
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/Connection.html#sendRawTransaction)
    ///
    pub async fn send_raw_transaction(this: &Connection, tx: JsValue) -> Result<JsValue>;

    #[wasm_bindgen(method, catch, js_name = "sendRawTransaction")]
    /// Send a transaction that has already been signed and serialized into the wire format
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/Connection.html#sendRawTransaction)
    ///
    pub async fn send_raw_transaction_with_options_impl(
        this: &Connection,
        tx: JsValue,
        options: JsValue,
    ) -> Result<JsValue>;

    #[wasm_bindgen(method, catch, js_name = "getAccountInfo")]
    /// Fetch all the account info for the specified public key
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/Connection.html#getAccountInfo)
    ///
    pub async fn get_account_info_impl(this: &Connection, public_key: JsValue) -> Result<JsValue>;

    #[wasm_bindgen(method, catch, js_name = "getAccountInfo")]
    /// Fetch all the account info for the specified public key
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/Connection.html#getAccountInfo)
    ///
    pub async fn get_account_info_with_options_impl(
        this: &Connection,
        public_key: JsValue,
        options: JsValue,
    ) -> Result<JsValue>;

    #[wasm_bindgen(method, catch, js_name = "getProgramAccounts")]
    /// Fetch all the account info for the specified public key
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/Connection.html#getProgramAccounts)
    ///
    pub async fn get_program_accounts_with_config_impl(
        this: &Connection,
        public_key: JsValue,
        config: GetProgramAccountsConfig,
    ) -> Result<JsValue>;

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type LatestBlockhashInfo;

    #[wasm_bindgen(getter, method, js_name = "blockhash")]
    /// get blockhash
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/Connection.html#getLatestBlockhash)
    ///
    pub fn block_hash(this: &LatestBlockhashInfo) -> JsValue;

    #[wasm_bindgen(getter, method, js_name = "lastValidBlockHeight")]
    /// get lastValidBlockHeight
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/Connection.html#getLatestBlockhash)
    ///
    pub fn last_valid_block_height(this: &LatestBlockhashInfo) -> JsValue;

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type SendRawTxOptions;

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

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug)]
    pub type ProgramAccount;

    #[wasm_bindgen(getter, method)]
    pub fn data(this: &ProgramAccount) -> Vec<u8>;

    #[wasm_bindgen(js_namespace=solanaWeb3, js_name = PublicKey)]
    #[derive(Debug)]
    pub type PublicKey;

    #[wasm_bindgen(method, js_name = "toBytes")]
    pub fn to_bytes(this: &PublicKey) -> Vec<u8>;

    #[wasm_bindgen(method, js_name = "toString")]
    pub fn to_string(this: &PublicKey) -> String;

}

impl Connection {
    pub async fn get_latest_block_hash(&self) -> Result<LatestBlockhashInfo> {
        Ok(self.get_latest_block_hash_impl().await?.into())
    }

    pub async fn get_account_info(&self, pubkey: &Pubkey) -> Result<JsValue> {
        self.get_account_info_impl(pubkey_to_jsvalue(pubkey)?).await
    }

    pub async fn get_account_info_with_options(
        &self,
        pubkey: &Pubkey,
        options: JsValue,
    ) -> Result<JsValue> {
        self.get_account_info_with_options_impl(pubkey_to_jsvalue(pubkey)?, options)
            .await
    }

    pub async fn send_raw_transaction_with_options(
        &self,
        tx: JsValue,
        options: SendRawTxOptions,
    ) -> Result<JsValue> {
        self.send_raw_transaction_with_options_impl(tx, options.into())
            .await
    }

    pub async fn get_program_accounts_with_config(
        &self,
        pubkey: &Pubkey,
        config: GetProgramAccountsConfig,
    ) -> Result<Vec<(Pubkey, ProgramAccount)>> {
        /*
        let commitment = config
            .account_config
            .commitment
            .unwrap_or_else(|| self.commitment());
        let commitment = self.maybe_map_ommitment(commitment).await?;
        config.account_config.commitment = Some(commitment);
        if let Some(filters) = config.filters {
            config.filters = Some(self.maybe_map_filters(filters).await?);
        }

        let accounts = self
            .send::<OptionalContext<Vec<RpcKeyedAccount>>>(
                RpcRequest::GetProgramAccounts,
                json!([pubkey.to_string(), config]),
            )
            .await?
            .parse_value();
        parse_keyed_accounts(accounts, RpcRequest::GetProgramAccounts)
        */
        //let config = config.into();

        log_trace!("config: {config:?}");

        let res = self
            .get_program_accounts_with_config_impl(pubkey_to_jsvalue(pubkey)?, config)
            .await?;

        log_trace!("array: {res:#?}, is_array:{}", res.is_array());
        let mut result = vec![];
        if res.is_array() {
            let array = Array::from(&res);
            let size = array.length();
            for index in 0..size {
                let item = array.get(index);
                if item.is_object() {
                    if let Ok(item) = TryInto::<ProgramAccountsResultItem>::try_into(item) {
                        let key = item.pubkey()?;
                        let account = item.account();
                        log_trace!("item.pubkey: {:?}", key);
                        log_trace!("item.account: {:?}", item.account());
                        log_trace!("item.account.data: {:?}", item.account().data());
                        result.push((key, account))
                    }
                }
            }
        }

        Ok(result)
    }
}

impl OptionsTrait for SendRawTxOptions {}

impl SendRawTxOptions {
    /// set skipPreflight
    pub fn skip_preflight(self, skip_preflight: bool) -> Self {
        self.set("skipPreflight", JsValue::from(skip_preflight))
    }
}

impl OptionsTrait for GetProgramAccountsConfig {}

pub enum GetProgramAccountsFilter {
    /// Memory comparison filter
    Memcmp(u64, String),
    /// Data size comparison filter
    DataSize(u64),
}

impl TryFrom<GetProgramAccountsFilter> for JsValue {
    type Error = crate::error::Error;
    fn try_from(value: GetProgramAccountsFilter) -> Result<Self> {
        let obj = Object::new();
        match value {
            GetProgramAccountsFilter::Memcmp(offset, data) => {
                let memcmp = Object::new();
                Reflect::set(&memcmp, &JsValue::from("offset"), &JsValue::from(offset))?;
                Reflect::set(&memcmp, &JsValue::from("bytes"), &JsValue::from(data))?;
                Reflect::set(&obj, &JsValue::from("memcmp"), &memcmp.into())?;
            }
            GetProgramAccountsFilter::DataSize(data_size) => {
                Reflect::set(&obj, &JsValue::from("dataSize"), &JsValue::from(data_size))?;
            }
        }

        Ok(obj.into())
    }
}

impl GetProgramAccountsConfig {
    pub fn add_filters(self, filters: Vec<GetProgramAccountsFilter>) -> Result<Self> {
        let list = Array::new();
        for filter in filters {
            list.push(&filter.try_into()?);
        }
        Ok(self.set("filters", list.into()))
    }
}

impl ProgramAccountsResultItem {
    pub fn pubkey(&self) -> Result<Pubkey> {
        match self.pubkey_impl().try_into() {
            Ok(value) => Ok(value),
            Err(_) => Err(JsValue::from("Invalid pubkey").into()),
        }
    }
}

impl TryFrom<PublicKey> for Pubkey {
    type Error = Vec<u8>;
    fn try_from(key: PublicKey) -> std::result::Result<Self, Vec<u8>> {
        Pubkey::try_from(key.to_bytes())
    }
}
