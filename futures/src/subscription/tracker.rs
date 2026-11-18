use crate::subscription::{Event, Hasher, Recipe};
use crate::{BoxFuture, MaybeSend};

use futures::channel::mpsc;
use futures::sink::{Sink, SinkExt};
use rustc_hash::FxHashMap;

use std::hash::Hasher as _;

/// 订阅流的注册表。
///
/// 如果你的应用程序不断返回一个 [`Subscription`]，
/// 就可以用 [`Tracker`] 跟踪各个配方，并让它们的
/// 执行保持存活。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A registry of subscription streams.
///
/// If you have an application that continuously returns a [`Subscription`],
/// you can use a [`Tracker`] to keep track of the different recipes and keep
/// its executions alive.
///
/// </details>
///
/// [`Subscription`]: crate::Subscription
#[derive(Debug, Default)]
pub struct Tracker {
    subscriptions: FxHashMap<u64, Execution>,
}

#[derive(Debug)]
pub struct Execution {
    _cancel: futures::channel::oneshot::Sender<()>,
    listener: Option<futures::channel::mpsc::Sender<Event>>,
}

impl Tracker {
    /// 创建一个新的空 [`Tracker`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new empty [`Tracker`].
    ///
    /// </details>
    pub fn new() -> Self {
        Self {
            subscriptions: FxHashMap::default(),
        }
    }

    /// 用给定的 [`Subscription`] 更新 [`Tracker`]。
    ///
    /// 一个 [`Subscription`] 可能导致新的流被派生，或旧的流
    /// 被关闭。
    ///
    /// [`Tracker`] 会在对这个方法的多次调用之间跟踪
    /// 这些流：
    ///
    /// - 如果给定的 [`Subscription`] 包含一个当前没有在运行的新 [`Recipe`]，
    ///   它会派生一个新的流并让它保持存活。
    /// - 反之，如果某个 [`Recipe`] 正在执行，而给定的
    ///   [`Subscription`] 已不再包含它，那么
    ///   [`Tracker`] 会关闭并丢弃相应的流。
    ///
    /// 它返回一组需要被派生的 future，用来落实
    /// [`Tracker`] 的这些变更。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Updates the [`Tracker`] with the given [`Subscription`].
    ///
    /// A [`Subscription`] can cause new streams to be spawned or old streams
    /// to be closed.
    ///
    /// The [`Tracker`] keeps track of these streams between calls to this
    /// method:
    ///
    /// - If the provided [`Subscription`] contains a new [`Recipe`] that is
    ///   currently not being run, it will spawn a new stream and keep it alive.
    /// - On the other hand, if a [`Recipe`] is currently in execution and the
    ///   provided [`Subscription`] does not contain it anymore, then the
    ///   [`Tracker`] will close and drop the relevant stream.
    ///
    /// It returns a list of futures that need to be spawned to materialize
    /// the [`Tracker`] changes.
    ///
    /// </details>
    ///
    /// [`Recipe`]: crate::subscription::Recipe
    /// [`Subscription`]: crate::Subscription
    pub fn update<Message, Receiver>(
        &mut self,
        recipes: impl Iterator<Item = Box<dyn Recipe<Output = Message>>>,
        receiver: Receiver,
    ) -> Vec<BoxFuture<()>>
    where
        Message: 'static + MaybeSend,
        Receiver: 'static
            + Sink<Message, Error = mpsc::SendError>
            + Unpin
            + MaybeSend
            + Clone,
    {
        use futures::stream::StreamExt;

        let mut futures: Vec<BoxFuture<()>> = Vec::new();
        let mut alive = std::collections::HashSet::new();

        for recipe in recipes {
            let id = {
                let mut hasher = Hasher::default();
                recipe.hash(&mut hasher);

                hasher.finish()
            };

            let _ = alive.insert(id);

            if self.subscriptions.contains_key(&id) {
                continue;
            }

            let (cancel, mut canceled) = futures::channel::oneshot::channel();

            // TODO: Use bus if/when it supports async
            // 待办：如果（或等到）bus 支持异步，就改用它
            let (event_sender, event_receiver) =
                futures::channel::mpsc::channel(100);

            let mut receiver = receiver.clone();
            let mut stream = recipe.stream(event_receiver.boxed());

            let future = async move {
                loop {
                    let select =
                        futures::future::select(&mut canceled, stream.next());

                    match select.await {
                        futures::future::Either::Left(_)
                        | futures::future::Either::Right((None, _)) => break,
                        futures::future::Either::Right((Some(message), _)) => {
                            let _ = receiver.send(message).await;
                        }
                    }
                }
            };

            let _ = self.subscriptions.insert(
                id,
                Execution {
                    _cancel: cancel,
                    listener: if event_sender.is_closed() {
                        None
                    } else {
                        Some(event_sender)
                    },
                },
            );

            futures.push(Box::pin(future));
        }

        self.subscriptions.retain(|id, _| alive.contains(id));

        futures
    }

    /// 向当前存活的订阅广播一个事件。
    ///
    /// 订阅的 [`Recipe::stream`] 总是接收一个事件流
    /// 作为输入。某些订阅可以用这个流来监听
    /// 外壳事件。
    ///
    /// 这个方法会把给定的事件发布到当前打开的所有
    /// 订阅流。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Broadcasts an event to the subscriptions currently alive.
    ///
    /// A subscription's [`Recipe::stream`] always receives a stream of events
    /// as input. This stream can be used by some subscription to listen to
    /// shell events.
    ///
    /// This method publishes the given event to all the subscription streams
    /// currently open.
    ///
    /// </details>
    ///
    /// [`Recipe::stream`]: crate::subscription::Recipe::stream
    pub fn broadcast(&mut self, event: Event) {
        self.subscriptions
            .values_mut()
            .filter_map(|connection| connection.listener.as_mut())
            .for_each(|listener| {
                if let Err(error) = listener.try_send(event.clone()) {
                    log::warn!(
                        "Error sending event to subscription: {error:?}"
                    );
                }
            });
    }
}
