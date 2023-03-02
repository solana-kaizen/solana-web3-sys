/*!

[<img alt="github" src="https://img.shields.io/badge/github-solana--kaizen/solana--web3--sys-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/solana-kaizen/solana-web3-sys)
[<img alt="crates.io" src="https://img.shields.io/crates/v/solana-web3-sys.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/solana-web3-sys)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-solana--web3--sys-56c2a5?maxAge=2592000&style=for-the-badge&logo=rust" height="20">](https://docs.rs/solana-web3-sys)
<img alt="license" src="https://img.shields.io/crates/l/solana-web3-sys.svg?maxAge=2592000&color=6ac&style=for-the-badge&logoColor=fff" height="20">

Rust `wasm_bindgen` bindings for [Solana `web3.js` APIs](https://docs.solana.com/developing/clients/javascript-api)

*/

pub mod account;
pub mod api;
pub mod connection;
pub mod error;
pub mod instruction;
pub mod options;
pub mod publickey;
pub mod result;
pub mod solana;
pub mod transaction;
pub mod utils;
pub mod wallet;

pub mod prelude {
    //!
    //! Prelude containing imports exposed by this crate.
    //!
    use super::*;
    pub use account::*;
    pub use api::*;
    pub use connection::*;
    pub use instruction::*;
    pub use options::*;
    pub use publickey::*;
    pub use solana::*;
    pub use transaction::*;
    pub use utils::*;
    pub use wallet::*;
}

mod imports;
