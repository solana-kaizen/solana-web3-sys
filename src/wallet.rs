use crate::imports::*;
use crate::transaction::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type WalletAdapter;

    #[wasm_bindgen(getter, method, js_name = "publicKey")]
    /// get pubKey
    ///
    pub fn pubkey(this: &WalletAdapter) -> JsValue;

    #[wasm_bindgen(method, js_name = "signTransaction")]
    /// sign transaction
    ///
    pub async fn sign_transaction_impl(this: &WalletAdapter, tx: Transaction) -> JsValue;
}

impl WalletAdapter {
    pub async fn sign_transaction(&self, tx: &Transaction) -> Result<JsValue> {
        Ok(self.sign_transaction_impl(tx.clone()).await)
    }
}
