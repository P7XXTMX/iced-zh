//! 用于 [`Program`](crate::Program) 消息类型的特质。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Traits for the message type of a [`Program`](crate::Program).
//!
//! </details>

/// [`Clone`] 的特质别名，但仅在启用 `time-travel`
/// 特性时生效。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A trait alias for [`Clone`], but only when the `time-travel`
/// feature is enabled.
///
/// </details>
#[cfg(feature = "time-travel")]
pub trait MaybeClone: Clone {}

#[cfg(feature = "time-travel")]
impl<T> MaybeClone for T where T: Clone {}

/// [`Clone`] 的特质别名，但仅在启用 `time-travel`
/// 特性时生效。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A trait alias for [`Clone`], but only when the `time-travel`
/// feature is enabled.
///
/// </details>
#[cfg(not(feature = "time-travel"))]
pub trait MaybeClone {}

#[cfg(not(feature = "time-travel"))]
impl<T> MaybeClone for T {}

/// [`Debug`](std::fmt::Debug) 的特质别名，但仅在启用
/// `debug` 特性时生效。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A trait alias for [`Debug`](std::fmt::Debug), but only when the
/// `debug` feature is enabled.
///
/// </details>
#[cfg(feature = "debug")]
pub trait MaybeDebug: std::fmt::Debug {}

#[cfg(feature = "debug")]
impl<T> MaybeDebug for T where T: std::fmt::Debug {}

/// [`Debug`](std::fmt::Debug) 的特质别名，但仅在启用
/// `debug` 特性时生效。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A trait alias for [`Debug`](std::fmt::Debug), but only when the
/// `debug` feature is enabled.
///
/// </details>
#[cfg(not(feature = "debug"))]
pub trait MaybeDebug {}

#[cfg(not(feature = "debug"))]
impl<T> MaybeDebug for T {}
