use crate::imports::*;

pub fn pubkey_to_jsvalue(pubkey: &Pubkey) -> Result<JsValue> {
    let pubkey_bytes = pubkey.to_bytes();
    let u8arr = Uint8Array::from(&pubkey_bytes[..]);
    let pkargs = Array::new_with_length(1);
    pkargs.set(0u32, u8arr.into());
    let ctor = js_sys::Reflect::get(&solana()?, &JsValue::from("PublicKey"))?;
    let pk_jsv = js_sys::Reflect::construct(&ctor.into(), &pkargs)?;
    Ok(pk_jsv)
}
