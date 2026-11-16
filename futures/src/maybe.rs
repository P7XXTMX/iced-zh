#[cfg(not(target_arch = "wasm32"))]
mod platform {
    /// 一个扩展特质，仅在原生平台上强制要求 `Send`。
    ///
    /// 编写跨平台异步代码时很有用！
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An extension trait that enforces `Send` only on native platforms.
    ///
    /// Useful for writing cross-platform async code!
    ///
    /// </details>
    pub trait MaybeSend: Send {}

    impl<T> MaybeSend for T where T: Send {}

    /// 一个扩展特质，仅在原生平台上强制要求 `Sync`。
    ///
    /// 编写跨平台异步代码时很有用！
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An extension trait that enforces `Sync` only on native platforms.
    ///
    /// Useful for writing cross-platform async code!
    ///
    /// </details>
    pub trait MaybeSync: Sync {}

    impl<T> MaybeSync for T where T: Sync {}
}

#[cfg(target_arch = "wasm32")]
mod platform {
    /// 一个扩展特质，仅在原生平台上强制要求 `Send`。
    ///
    /// 编写跨平台异步代码时很有用！
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An extension trait that enforces `Send` only on native platforms.
    ///
    /// Useful for writing cross-platform async code!
    ///
    /// </details>
    pub trait MaybeSend {}

    impl<T> MaybeSend for T {}

    /// 一个扩展特质，仅在原生平台上强制要求 `Sync`。
    ///
    /// 编写跨平台异步代码时很有用！
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An extension trait that enforces `Sync` only on native platforms.
    ///
    /// Useful for writing cross-platform async code!
    ///
    /// </details>
    pub trait MaybeSync {}

    impl<T> MaybeSync for T {}
}

pub use platform::{MaybeSend, MaybeSync};
