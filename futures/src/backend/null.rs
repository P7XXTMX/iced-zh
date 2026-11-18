//! 一个什么都不做的后端！
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! A backend that does nothing!
//!
//! </details>
use crate::MaybeSend;

/// 一个丢弃所有 future 而不是派生它们的执行器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An executor that drops all the futures, instead of spawning them.
///
/// </details>
#[derive(Debug)]
pub struct Executor;

impl crate::Executor for Executor {
    fn new() -> Result<Self, futures::io::Error> {
        Ok(Self)
    }

    fn spawn(&self, _future: impl Future<Output = ()> + MaybeSend + 'static) {}

    #[cfg(not(target_arch = "wasm32"))]
    fn block_on<T>(&self, _future: impl Future<Output = T>) -> T {
        unimplemented!()
    }
}

pub mod time {
    //! 监听时间并做出响应。
    //!
    //! <details>
    //! <summary>显示英文原文</summary>
    //!
    //! Listen and react to time.
    //!
    //! </details>
}
