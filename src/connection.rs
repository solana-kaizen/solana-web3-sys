//!
//! [`Connection`](https://solana-labs.github.io/solana-web3.js/classes/Connection.html) class bindings.
//!
use crate::account::ProgramAccount;
use crate::api::*;
use crate::imports::*;
use crate::publickey::PublicKey;
use solana_sdk::account::Account;
//use workflow_log::log_trace;

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
        config: RpcProgramAccountsConfig,
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

}

impl Connection {
    pub async fn get_latest_block_hash(&self) -> Result<LatestBlockhashInfo> {
        Ok(self.get_latest_block_hash_impl().await?.into())
    }

    pub async fn get_account_info(&self, pubkey: &Pubkey) -> Result<Account> {
        let value = self
            .get_account_info_impl(PublicKey::try_from(pubkey)?.into())
            .await?;
        if !value.is_object() {
            return Err(JsValue::from(format!("Account not found: {pubkey:?}")).into());
        }
        let account: ProgramAccount = value.try_into().map_err(|err| {
            JsValue::from(format!(
                "Unable to convert account into to ProgramAccount: {err}"
            ))
        })?;
        account.try_into()
    }

    pub async fn get_account_info_with_options(
        &self,
        pubkey: &Pubkey,
        options: JsValue,
    ) -> Result<JsValue> {
        self.get_account_info_with_options_impl(PublicKey::try_from(pubkey)?.into(), options)
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
        config: RpcProgramAccountsConfig,
    ) -> Result<Vec<(Pubkey, Account)>> {
        let res = self
            .get_program_accounts_with_config_impl(pubkey_to_jsvalue(pubkey)?, config)
            .await?;

        //log_trace!("array: {res:#?}, is_array:{}", res.is_array());
        let mut result = vec![];
        if !res.is_array() {
            return Ok(result);
        }
        let array = Array::from(&res);
        let size = array.length();
        for index in 0..size {
            let item = array.get(index);
            if !item.is_object() {
                continue;
            }
            if let Ok(item) = ProgramAccountsResultItem::try_from(item) {
                result.push((item.pubkey()?, item.account().try_into()?))
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
