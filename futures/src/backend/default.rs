//! 默认的跨平台后端。
//!
//! - 在原生平台上，它会使用：
//!   - 启用 `tokio` 特性时使用 `backend::native::tokio`。
//!   - 启用 `smol` 特性时使用 `backend::native::smol`。
//!   - 启用 `thread-pool` 特性时使用 `backend::native::thread_pool`。
//!   - 其他情况使用 `backend::null`。
//!
//! - 在 Wasm 上，它会使用 `backend::wasm::wasm_bindgen`。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! A default, cross-platform backend.
//!
//! - On native platforms, it will use:
//!   - `backend::native::tokio` when the `tokio` feature is enabled.
//!   - `backend::native::smol` when the `smol` feature is enabled.
//!   - `backend::native::thread_pool` when the `thread-pool` feature is enabled.
//!   - `backend::null` otherwise.
//!
//! - On Wasm, it will use `backend::wasm::wasm_bindgen`.
//!
//! </details>
#[cfg(not(target_arch = "wasm32"))]
mod platform {
    #[cfg(feature = "tokio")]
    pub use crate::backend::native::tokio::*;

    #[cfg(all(feature = "smol", not(feature = "tokio"),))]
    pub use crate::backend::native::smol::*;

    #[cfg(all(
        feature = "thread-pool",
        not(any(feature = "tokio", feature = "smol"))
    ))]
    pub use crate::backend::native::thread_pool::*;

    #[cfg(not(any(
        feature = "tokio",
        feature = "smol",
        feature = "thread-pool"
    )))]
    pub use crate::backend::null::*;
}

#[cfg(target_arch = "wasm32")]
mod platform {
    pub use crate::backend::wasm::wasm_bindgen::*;
}

pub use platform::*;
