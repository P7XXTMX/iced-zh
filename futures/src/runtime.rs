//! 运行命令并跟踪订阅。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Run commands and keep track of subscriptions.
//!
//! </details>
use crate::subscription;
use crate::{BoxStream, Executor, MaybeSend};

use futures::{Sink, SinkExt, channel::mpsc};
use std::marker::PhantomData;

/// 一个开箱即用的命令与订阅运行时。
///
/// 如果你有一个 [`Executor`]，就可以利用 [`Runtime`] 运行任何
/// `Command` 或 [`Subscription`]，并在有结果时得到通知！
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A batteries-included runtime of commands and subscriptions.
///
/// If you have an [`Executor`], a [`Runtime`] can be leveraged to run any
/// `Command` or [`Subscription`] and get notified of the results!
///
/// </details>
///
/// [`Subscription`]: crate::Subscription
#[derive(Debug)]
pub struct Runtime<Executor, Sender, Message> {
    executor: Executor,
    sender: Sender,
    subscriptions: subscription::Tracker,
    _message: PhantomData<Message>,
}

impl<Executor, Sender, Message> Runtime<Executor, Sender, Message>
where
    Executor: self::Executor,
    Sender: Sink<Message, Error = mpsc::SendError> + Unpin + MaybeSend + Clone + 'static,
    Message: MaybeSend + 'static,
{
    /// 创建一个新的空 [`Runtime`]。
    ///
    /// 你需要提供：
    /// - 一个用于派生 future 的 [`Executor`]
    /// - 一个实现了 `Sink` 的 `Sender`，用于接收结果
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new empty [`Runtime`].
    ///
    /// You need to provide:
    /// - an [`Executor`] to spawn futures
    /// - a `Sender` implementing `Sink` to receive the results
    ///
    /// </details>
    pub fn new(executor: Executor, sender: Sender) -> Self {
        Self {
            executor,
            sender,
            subscriptions: subscription::Tracker::new(),
            _message: PhantomData,
        }
    }

    /// 在 [`Runtime`] 的 [`Executor`] 内运行给定的闭包。
    ///
    /// 详见 [`Executor::enter`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Runs the given closure inside the [`Executor`] of the [`Runtime`].
    ///
    /// See [`Executor::enter`] to learn more.
    ///
    /// </details>
    pub fn enter<R>(&self, f: impl FnOnce() -> R) -> R {
        self.executor.enter(f)
    }

    /// 在 [`Runtime`] 内，于当前线程上运行一个 future 直至完成。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Runs a future to completion in the current thread within the [`Runtime`].
    ///
    /// </details>
    #[cfg(not(target_arch = "wasm32"))]
    pub fn block_on<T>(&mut self, future: impl Future<Output = T>) -> T {
        self.executor.block_on(future)
    }

    /// 在 [`Runtime`] 中运行一个 [`Stream`] 直至完成。
    ///
    /// 产生的 `Message` 会被转发给 [`Runtime`] 的
    /// `Sender`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Runs a [`Stream`] in the [`Runtime`] until completion.
    ///
    /// The resulting `Message`s will be forwarded to the `Sender` of the
    /// [`Runtime`].
    ///
    /// </details>
    ///
    /// [`Stream`]: BoxStream
    pub fn run(&mut self, stream: BoxStream<Message>) {
        use futures::{FutureExt, StreamExt};

        let sender = self.sender.clone();
        let future = stream.map(Ok).forward(sender).map(|result| match result {
            Ok(()) => (),
            Err(error) => {
                log::warn!("Stream could not run until completion: {error}");
            }
        });

        self.executor.spawn(future);
    }

    /// 通过 [`Runtime`] 并发地发送一条消息。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sends a message concurrently through the [`Runtime`].
    ///
    /// </details>
    pub fn send(&mut self, message: Message) {
        let mut sender = self.sender.clone();

        self.executor.spawn(async move {
            let _ = sender.send(message).await;
        });
    }

    /// 在 [`Runtime`] 中跟踪一个 [`Subscription`]。
    ///
    /// 它会根据需要派生新的流或关闭旧的流！
    /// 详见 [`Tracker::update`]！
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Tracks a [`Subscription`] in the [`Runtime`].
    ///
    /// It will spawn new streams or close old ones as necessary! See
    /// [`Tracker::update`] to learn more about this!
    ///
    /// </details>
    ///
    /// [`Tracker::update`]: subscription::Tracker::update
    /// [`Subscription`]: crate::Subscription
    pub fn track(
        &mut self,
        recipes: impl IntoIterator<Item = Box<dyn subscription::Recipe<Output = Message>>>,
    ) {
        let Runtime {
            executor,
            subscriptions,
            sender,
            ..
        } = self;

        let futures = executor.enter(|| subscriptions.update(recipes.into_iter(), sender.clone()));

        for future in futures {
            executor.spawn(future);
        }
    }

    /// 向 [`Runtime`] 中当前存活的所有订阅广播一个
    /// 事件。
    ///
    /// 详见 [`Tracker::broadcast`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Broadcasts an event to all the subscriptions currently alive in the
    /// [`Runtime`].
    ///
    /// See [`Tracker::broadcast`] to learn more.
    ///
    /// </details>
    ///
    /// [`Tracker::broadcast`]: subscription::Tracker::broadcast
    pub fn broadcast(&mut self, event: subscription::Event) {
        self.subscriptions.broadcast(event);
    }
}
