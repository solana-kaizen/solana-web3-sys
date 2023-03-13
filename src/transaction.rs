//!
//! [`Transaction`](https://solana-labs.github.io/solana-web3.js/classes/Transaction.html) class bindings.
//!

use crate::imports::*;
use crate::instruction::TransactionInstruction;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=solanaWeb3, js_name = Transaction)]
    #[derive(Debug, Clone)]
    /// Transaction
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/Transaction.html)
    ///
    pub type Transaction;

    #[wasm_bindgen(constructor, js_namespace=["solanaWeb3"])]
    /// Construct an empty Transaction
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/Transaction.html)
    ///
    pub fn new() -> Transaction;

    #[wasm_bindgen(setter, method, js_namespace=["solanaWeb3"], js_name="feePayer")]
    /// Set the transaction fee payer
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/Transaction.html#feePayer)
    ///
    pub fn set_fee_payer(this: &Transaction, fee_payer_pubkey: JsValue);

    #[wasm_bindgen(setter, method, js_namespace=["solanaWeb3"], js_name="recentBlockhash")]
    /// A recent transaction id. Must be populated by the caller
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/Transaction.html#recentBlockhash)
    ///
    pub fn set_recent_block_hash(this: &Transaction, recent_blockhash: JsValue);

    #[wasm_bindgen(method, js_namespace=["solanaWeb3"], js_name="add")]
    /// Add one instruction to this Transaction
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/Transaction.html#add)
    ///
    pub fn add(this: &Transaction, instruction: TransactionInstruction);

    #[wasm_bindgen(method, js_namespace=["solanaWeb3"], js_name="serialize")]
    /// Serialize the Transaction in the wire format.
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/Transaction.html#serialize)
    ///
    pub fn serialize(this: &Transaction, config: SerializeConfig) -> JsValue;

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Configuration object for Transaction.serialize()
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/types/SerializeConfig.html)
    ///
    pub type SerializeConfig;

    #[wasm_bindgen(setter, method, js_name = "requireAllSignatures")]
    /// Setter: requireAllSignatures Require all transaction signatures be present (default: true)
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/types/SerializeConfig.html)
    ///
    pub fn set_require_all_signatures(this: &SerializeConfig, require_all_signatures: bool);

    #[wasm_bindgen(setter, method, js_name = "verifySignatures")]
    /// Setter: Verify provided signatures (default: true)
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/types/SerializeConfig.html)
    ///
    pub fn set_verify_signatures(this: &SerializeConfig, verify_signatures: bool);
}

impl Transaction {}

impl OptionsTrait for SerializeConfig {}
