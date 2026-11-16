//! `wasm-bindgen-futures` 后端。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! A `wasm-bindgen-futures` backend.
//!
//! </details>

/// `wasm-bindgen-futures` 执行器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A `wasm-bindgen-futures` executor.
///
/// </details>
#[derive(Debug)]
pub struct Executor;

impl crate::Executor for Executor {
    fn new() -> Result<Self, futures::io::Error> {
        Ok(Self)
    }

    fn spawn(&self, future: impl futures::Future<Output = ()> + 'static) {
        wasm_bindgen_futures::spawn_local(future);
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
    use crate::subscription::Subscription;

    use wasmtimer::std::Instant;

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
    pub fn every(duration: std::time::Duration) -> Subscription<Instant> {
        Subscription::run_with(duration, |duration| {
            use futures::stream::StreamExt;

            let mut interval = wasmtimer::tokio::interval(*duration);
            interval.set_missed_tick_behavior(wasmtimer::tokio::MissedTickBehavior::Skip);

            let stream = {
                futures::stream::unfold(interval, |mut interval| async move {
                    Some((interval.tick().await, interval))
                })
            };

            stream.boxed()
        })
    }
}
