//! 监听时间并做出响应。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Listen and react to time.
//!
//! </details>
pub use crate::core::time::*;

#[allow(unused_imports)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "tokio", feature = "smol", target_arch = "wasm32")))
)]
pub use iced_futures::backend::default::time::*;

use crate::Task;

/// 返回一个 [`Task`]，它通过调用 [`Instant::now`]
/// 产生当前的 [`Instant`]。
///
/// 虽然你可以在应用程序中直接调用 [`Instant::now`]，
/// 但这会让你的应用程序变得“不纯”（即失去引用透明性）。
///
/// 如果你想正确地使用 `time-travel` 特性，
/// 就可能需要关心纯度。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns a [`Task`] that produces the current [`Instant`]
/// by calling [`Instant::now`].
///
/// While you can call [`Instant::now`] directly in your application;
/// that renders your application "impure" (i.e. no referential transparency).
///
/// You may care about purity if you want to leverage the `time-travel`
/// feature properly.
///
/// </details>
pub fn now() -> Task<Instant> {
    Task::future(async { Instant::now() })
}
