//!
//! `solana-web3-sys` initialization hooks.
//!

use crate::imports::*;

static mut SOLANA_WEB3JS_GLOBAL: Option<JsValue> = None;

cfg_if! {
    if #[cfg(feature = "init")] {
        #[wasm_bindgen]
        pub fn init_solana_web3_sys(solana_web3js_global: &JsValue) -> JsResult<()> {
            unsafe { SOLANA_WEB3JS_GLOBAL = Some(solana_web3js_global.clone()) };
            Ok(())
        }
    } else {
        pub fn init_solana_web3_sys(solana_web3js_global: &JsValue) -> JsResult<()> {
            unsafe { SOLANA_WEB3JS_GLOBAL = Some(solana_web3js_global.clone()) };
            Ok(())
        }
    }
}

pub fn solana() -> Result<JsValue> {
    let solana_web3js_global = unsafe { SOLANA_WEB3JS_GLOBAL.as_ref() }
        .expect("SOLANA_WEB3JS_GLOBAL is not initialized; please use `solana::init_solana_web3_sys()` to initialize.")
        .clone();
    Ok(solana_web3js_global)
}
