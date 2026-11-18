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
    use crate::subscription::{self, Hasher, Subscription};

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
    pub fn every(
        duration: std::time::Duration,
    ) -> Subscription<std::time::Instant> {
        subscription::from_recipe(Every(duration))
    }

    #[derive(Debug)]
    struct Every(std::time::Duration);

    impl subscription::Recipe for Every {
        type Output = std::time::Instant;

        fn hash(&self, state: &mut Hasher) {
            use std::hash::Hash;

            std::any::TypeId::of::<Self>().hash(state);
            self.0.hash(state);
        }

        fn stream(
            self: Box<Self>,
            _input: subscription::EventStream,
        ) -> futures::stream::BoxStream<'static, Self::Output> {
            use futures::stream::StreamExt;

            smol::Timer::interval(self.0).boxed()
        }
    }
}
