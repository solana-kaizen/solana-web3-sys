//!
//! `WalletAdapter` class bindings.
//!

use crate::imports::*;
use crate::prelude::Connection;
use crate::transaction::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=solanaWeb3, js_name = PhantomWalletAdapter)]
    #[derive(Debug, Clone)]
    /// WalletAdapter
    ///
    pub type WalletAdapter;

    #[wasm_bindgen(getter, method, js_namespace=solanaWeb3, js_name = "publicKey")]
    /// get pubKey
    ///
    pub fn pubkey(this: &WalletAdapter) -> JsValue;

    #[wasm_bindgen(method, catch, js_namespace=solanaWeb3, js_name = "signTransaction")]
    /// sign transaction
    ///
    pub async fn sign_transaction_impl(this: &WalletAdapter, tx: Transaction) -> Result<JsValue>;

    #[wasm_bindgen(method, catch, js_namespace=solanaWeb3, js_name = "signAndSendTransaction")]
    /// sign and send transaction
    ///
    pub async fn sign_and_send_transaction(
        this: &WalletAdapter,
        tx: Transaction,
    ) -> Result<JsValue>;

    #[wasm_bindgen(method, catch, js_namespace=solanaWeb3, js_name = "sendTransaction")]
    /// send transaction
    ///
    pub async fn send_transaction(
        this: &WalletAdapter,
        tx: Transaction,
        con: Connection,
    ) -> Result<JsValue>;
}

impl WalletAdapter {
    pub async fn sign_transaction(&self, tx: &Transaction) -> Result<JsValue> {
        self.sign_transaction_impl(tx.clone()).await
    }
}
