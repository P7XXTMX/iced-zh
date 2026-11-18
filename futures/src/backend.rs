//! `iced_futures` 契约的底层实现！
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! The underlying implementations of the `iced_futures` contract!
//!
//! </details>
pub mod null;

#[cfg(not(target_arch = "wasm32"))]
pub mod native;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub mod default;
