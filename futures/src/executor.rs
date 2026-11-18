//! 选择你偏好的执行器来驱动运行时。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Choose your preferred executor to power a runtime.
//!
//! </details>
use crate::MaybeSend;

/// 一种可以运行 future 的类型。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A type that can run futures.
///
/// </details>
pub trait Executor: Sized {
    /// 创建一个新的 [`Executor`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Executor`].
    ///
    /// </details>
    fn new() -> Result<Self, futures::io::Error>
    where
        Self: Sized;

    /// 在 [`Executor`] 中派生一个 future。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Spawns a future in the [`Executor`].
    ///
    /// </details>
    fn spawn(&self, future: impl Future<Output = ()> + MaybeSend + 'static);

    /// 在 [`Executor`] 内，于当前线程上运行一个 future 直至完成。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Runs a future to completion in the current thread within the [`Executor`].
    ///
    /// </details>
    #[cfg(not(target_arch = "wasm32"))]
    fn block_on<T>(&self, future: impl Future<Output = T>) -> T;

    /// 在 [`Executor`] 内运行给定的闭包。
    ///
    /// 有些执行器（例如 `tokio`）要求在创建 future 之前先准备好
    /// 某些全局状态。可以利用这个方法来设置这些全局状态、
    /// 调用函数、恢复状态，并取得调用的
    /// 结果。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Runs the given closure inside the [`Executor`].
    ///
    /// Some executors, like `tokio`, require some global state to be in place
    /// before creating futures. This method can be leveraged to set up this
    /// global state, call a function, restore the state, and obtain the result
    /// of the call.
    ///
    /// </details>
    fn enter<R>(&self, f: impl FnOnce() -> R) -> R {
        f()
    }
}
