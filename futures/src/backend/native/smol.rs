//! `smol` 后端。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! A `smol` backend.
//!
//! </details>

/// `smol` 执行器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A `smol` executor.
///
/// </details>
#[derive(Debug)]
pub struct Executor;

impl crate::Executor for Executor {
    fn new() -> Result<Self, futures::io::Error> {
        Ok(Self)
    }

    fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        smol::spawn(future).detach();
    }

    fn block_on<T>(&self, future: impl Future<Output = T>) -> T {
        smol::block_on(future)
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
    use crate::MaybeSend;
    use crate::core::time::{Duration, Instant};
    use crate::subscription::Subscription;

    use futures::stream;

    /// 返回一个按固定间隔产生消息的 [`Subscription`]。
    ///
    /// 第一条消息在经过 `duration` 之后产生，此后每隔
    /// `duration` 继续产生一条消息。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a [`Subscription`] that produces messages at a set interval.
    ///
    /// The first message is produced after a `duration`, and then continues to
    /// produce more messages every `duration` after that.
    ///
    /// </details>
    pub fn every(duration: Duration) -> Subscription<Instant> {
        Subscription::run_with(duration, |duration| {
            use futures::stream::StreamExt;

            let start = Instant::now() + *duration;

            smol::Timer::interval_at(start, *duration).boxed()
        })
    }

    /// 返回一个 [`Subscription`]，它按固定间隔运行给定的异步函数，
    /// 并把函数的结果作为输出。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a [`Subscription`] that runs the given async function at a
    /// set interval; producing the result of the function as output.
    ///
    /// </details>
    pub fn repeat<F, T>(f: fn() -> F, interval: Duration) -> Subscription<T>
    where
        F: Future<Output = T> + MaybeSend + 'static,
        T: MaybeSend + 'static,
    {
        Subscription::run_with((f, interval), |(f, interval)| {
            let f = *f;
            let interval = *interval;

            stream::unfold(0, move |i| async move {
                if i > 0 {
                    _ = smol::Timer::after(interval).await;
                }

                Some((f().await, i + 1))
            })
        })
    }
}
