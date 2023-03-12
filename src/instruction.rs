//!
//! `TransactionInstruction` class bindings.
//!
use crate::account::*;
use crate::imports::*;
use solana_program::instruction::Instruction;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    /// TransactionInstructionConfig (TransactionInstructionCtorFields)
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/types/TransactionInstructionCtorFields.html)
    ///
    pub type TransactionInstructionConfig;
}

impl OptionsTrait for TransactionInstructionConfig {}

impl TransactionInstructionConfig {
    /// Set keys
    pub fn keys(self, keys: Vec<AccountMeta>) -> Self {
        let list = Array::new();
        for key in keys {
            list.push(&key.into());
        }
        self.set("keys", JsValue::from(list))
    }

    /// Set programId
    pub fn program_id(self, program_id: &Pubkey) -> Result<Self> {
        Ok(self.set("programId", pubkey_to_jsvalue(program_id)?))
    }

    /// Set data
    pub fn data(self, data: &[u8]) -> Self {
        self.set("data", Uint8Array::from(data).into())
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=solanaWeb3, js_name = TransactionInstruction)]
    #[derive(Debug, Clone)]
    /// TransactionInstruction
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/TransactionInstruction.html)
    ///
    pub type TransactionInstruction;

    #[wasm_bindgen(constructor, js_namespace=["solanaWeb3"])]
    /// Create TransactionInstruction
    ///
    /// ⧉ [Solana Documentation](https://solana-labs.github.io/solana-web3.js/classes/TransactionInstruction.html)
    ///
    pub fn new(options: &TransactionInstructionConfig) -> TransactionInstruction;
}

impl TryFrom<&Instruction> for TransactionInstruction {
    type Error = crate::error::Error;
    fn try_from(instruction: &Instruction) -> Result<Self> {
        let mut accounts_list = vec![];

        for account in &instruction.accounts {
            accounts_list.push(account.try_into()?);
        }

        let cfg = TransactionInstructionConfig::new()
            .data(&instruction.data)
            .keys(accounts_list)
            .program_id(&instruction.program_id)?;

        Ok(TransactionInstruction::new(&cfg))
    }
}
