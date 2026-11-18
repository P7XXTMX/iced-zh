//! `ThreadPool` 后端。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! A `ThreadPool` backend.
//!
//! </details>

/// 用于 future 的线程池执行器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A thread pool executor for futures.
///
/// </details>
pub type Executor = futures::executor::ThreadPool;

impl crate::Executor for Executor {
    fn new() -> Result<Self, futures::io::Error> {
        futures::executor::ThreadPool::new()
    }

    fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        self.spawn_ok(future);
    }

    fn block_on<T>(&self, future: impl Future<Output = T>) -> T {
        futures::executor::block_on(future)
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
