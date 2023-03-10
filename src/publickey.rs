//!
//! [`PublicKey`](https://solana-labs.github.io/solana-web3.js/classes/PublicKey.html) class bindings.
//!

use crate::imports::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=solanaWeb3, js_name = PublicKey)]
    #[derive(Debug)]
    /// PublicKey
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/PublicKey.html)
    ///
    pub type PublicKey;

    #[wasm_bindgen(constructor, js_namespace=solanaWeb3)]
    /// Create [`PublicKey`] from sring
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/PublicKey.html#constructor)
    ///
    pub fn new(str: String) -> PublicKey;

    #[wasm_bindgen(constructor, js_namespace=solanaWeb3)]
    /// Create [`PublicKey`] from bytes array
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/PublicKey.html#constructor)
    ///
    pub fn new_from_array(bytes: Vec<u8>) -> PublicKey;

    #[wasm_bindgen(method, js_name = "toBytes")]
    /// Convert [`PublicKey`] to bytes array
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/PublicKey.html#toBytes)
    ///
    pub fn to_bytes(this: &PublicKey) -> Vec<u8>;

    #[wasm_bindgen(method, js_name = "toString")]
    /// Convert [`PublicKey`] to string
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/PublicKey.html#toString)
    ///
    pub fn to_string(this: &PublicKey) -> String;
}

impl TryFrom<PublicKey> for Pubkey {
    type Error = crate::error::Error;

    fn try_from(key: PublicKey) -> Result<Self> {
        match Pubkey::try_from(key.to_bytes()) {
            Ok(value) => Ok(value),
            Err(_) => Err(JsValue::from("Invalid pubkey").into()),
        }
    }
}

impl TryFrom<&Pubkey> for PublicKey {
    type Error = crate::error::Error;

    fn try_from(key: &Pubkey) -> Result<Self> {
        Ok(PublicKey::new_from_array(key.to_bytes().to_vec()))
    }
}

impl TryFrom<&[u8]> for PublicKey {
    type Error = crate::error::Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        Ok(PublicKey::new_from_array(bytes.to_vec()))
    }
}

impl TryFrom<&str> for PublicKey {
    type Error = crate::error::Error;

    fn try_from(str: &str) -> Result<Self> {
        Ok(PublicKey::new(str.to_string()))
    }
}
