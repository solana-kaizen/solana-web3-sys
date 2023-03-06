//!
//! `AccountMeta` class bindings.
//!
use crate::imports::*;
use crate::publickey::PublicKey;
use js_sys::BigInt;
use solana_program::instruction::AccountMeta as SolanaAccountMeta;
use solana_sdk::account::Account;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type AccountMeta;

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug)]
    pub type ProgramAccount;

    #[wasm_bindgen(constructor, js_class=Object)]
    pub fn new() -> ProgramAccount;

    #[wasm_bindgen(getter, method)]
    pub fn data(this: &ProgramAccount) -> Vec<u8>;

    #[wasm_bindgen(setter, method, js_name = "data")]
    pub fn set_data(this: &ProgramAccount, data: Vec<u8>);

    #[wasm_bindgen(getter, method)]
    pub fn executable(this: &ProgramAccount) -> bool;

    #[wasm_bindgen(setter, method, js_name = "executable")]
    pub fn set_executable(this: &ProgramAccount, executable: bool);

    #[wasm_bindgen(getter, method)]
    pub fn lamports(this: &ProgramAccount) -> BigInt;

    #[wasm_bindgen(setter, method, js_name = "lamports")]
    pub fn set_lamports(this: &ProgramAccount, lamports: BigInt);

    #[wasm_bindgen(getter, method)]
    pub fn owner(this: &ProgramAccount) -> PublicKey;

    #[wasm_bindgen(setter, method, js_name = "owner")]
    pub fn set_owner(this: &ProgramAccount, owner: PublicKey);

    #[wasm_bindgen(getter, method, js_name = "rentEpoch")]
    pub fn rent_epoch(this: &ProgramAccount) -> BigInt;

    #[wasm_bindgen(setter, method, js_name = "rentEpoch")]
    pub fn set_rent_epoch(this: &ProgramAccount, rent_epoch: BigInt);

    #[wasm_bindgen(getter, method)]
    pub fn space(this: &ProgramAccount) -> BigInt;

    #[wasm_bindgen(setter, method, js_name = "space")]
    pub fn set_space(this: &ProgramAccount, space: BigInt);
}

impl OptionsTrait for AccountMeta {}

impl AccountMeta {
    /// Set writable
    pub fn is_writable(self, is_writable: bool) -> Self {
        self.set("isWritable", JsValue::from(is_writable))
    }

    /// Set signer
    pub fn is_signer(self, is_signer: bool) -> Self {
        self.set("isSigner", JsValue::from(is_signer))
    }

    /// Set pubkey
    pub fn pubkey(self, pubkey: &Pubkey) -> Result<Self> {
        Ok(self.set("pubkey", pubkey_to_jsvalue(pubkey)?))
    }
}
impl TryFrom<&SolanaAccountMeta> for AccountMeta {
    type Error = crate::error::Error;
    fn try_from(account: &SolanaAccountMeta) -> Result<Self> {
        AccountMeta::new()
            .is_signer(account.is_signer)
            .is_writable(account.is_writable)
            .pubkey(&account.pubkey)
    }
}

impl TryFrom<ProgramAccount> for Account {
    type Error = crate::error::Error;

    fn try_from(account: ProgramAccount) -> Result<Self> {
        if !account.is_object() {
            return Err(JsValue::from("Invalid ProgramAccount").into());
        }
        Ok(Self {
            lamports: account.lamports().as_f64().unwrap() as u64,
            data: account.data(),
            owner: account.owner().try_into()?,
            rent_epoch: account.rent_epoch().as_f64().unwrap() as u64,
            executable: account.executable(),
        })
    }
}

impl TryFrom<Account> for ProgramAccount {
    type Error = crate::error::Error;

    fn try_from(account: Account) -> Result<Self> {
        let acc = ProgramAccount::new();
        acc.set_lamports(BigInt::from(account.lamports));
        acc.set_data(account.data);
        acc.set_owner(account.owner.as_ref().try_into()?);
        acc.set_rent_epoch(BigInt::from(account.rent_epoch));
        acc.set_executable(account.executable);

        Ok(acc)
    }
}
