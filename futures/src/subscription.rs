//! 在你的应用程序中监听外部事件。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Listen to external events in your application.
//!
//! </details>
mod tracker;

pub use tracker::Tracker;

use crate::core::event;
use crate::core::theme;
use crate::core::window;
use crate::futures::Stream;
use crate::{BoxStream, MaybeSend};

use std::any::TypeId;
use std::hash::Hash;

/// 订阅事件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A subscription event.
///
/// </details>
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// 用户与某个窗口中的用户界面进行了交互。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A user interacted with a user interface in a window.
    ///
    /// </details>
    Interaction {
        /// 这次交互的界面所在的窗口。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The window holding the interface of the interaction.
        ///
        /// </details>
        window: window::Id,
        /// 描述这次交互的 [`Event`]。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The [`Event`] describing the interaction.
        ///
        /// </details>
        ///
        /// [`Event`]: event::Event
        event: event::Event,

        /// 这次交互的 [`event::Status`]。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The [`event::Status`] of the interaction.
        ///
        /// </details>
        status: event::Status,
    },

    /// 系统主题发生了变化。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The system theme has changed.
    ///
    /// </details>
    SystemThemeChanged(theme::Mode),

    /// 平台特定的事件。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A platform specific event.
    ///
    /// </details>
    PlatformSpecific(PlatformSpecific),
}

/// 平台特定的事件
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A platform specific event
///
/// </details>
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformSpecific {
    /// MacOS 特定的事件
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A MacOS specific event
    ///
    /// </details>
    MacOS(MacOS),
}

/// 描述一个 MacOS 特有的事件
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Describes an event specific to MacOS
///
/// </details>
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MacOS {
    /// 当应用收到来自系统的 URL 时触发
    ///
    /// _**注意：** 要触发这个事件，可执行文件需要正确[打包][bundled]！_
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Triggered when the app receives an URL from the system
    ///
    /// _**Note:** For this event to be triggered, the executable needs to be properly [bundled]!_
    ///
    /// </details>
    ///
    /// [bundled]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFBundles/BundleTypes/BundleTypes.html#//apple_ref/doc/uid/10000123i-CH101-SW19
    ReceivedUrl(String),
}

/// 运行时事件的流。
///
/// 它是 [`Subscription`] 的输入。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A stream of runtime events.
///
/// It is the input of a [`Subscription`].
///
/// </details>
pub type EventStream = BoxStream<Event>;

/// 用于标识订阅的哈希器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The hasher used for identifying subscriptions.
///
/// </details>
pub type Hasher = rustc_hash::FxHasher;

/// 监听外部事件的请求。
///
/// 除了用 `Task` 按需执行异步操作之外，大多数应用程序
/// 还需要被动地监听外部事件。
///
/// [`Subscription`] 通常像 `Task` 一样交给某个运行时，
/// 只要用户持续请求它，它就会不断产生事件。
///
/// 例如，你可以用 [`Subscription`] 监听 `WebSocket`
/// 连接、按键、鼠标事件、时钟滴答等。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A request to listen to external events.
///
/// Besides performing async actions on demand with `Task`, most
/// applications also need to listen to external events passively.
///
/// A [`Subscription`] is normally provided to some runtime, like a `Task`,
/// and it will generate events as long as the user keeps requesting it.
///
/// For instance, you can use a [`Subscription`] to listen to a `WebSocket`
/// connection, keyboard presses, mouse events, time ticks, etc.
///
/// </details>
///
/// <a id="the-lifetime-of-a-subscription"></a>
///
/// # [`Subscription`] 的生命周期（The Lifetime of a [`Subscription`]）
/// 与 [`Future`] 或 [`Stream`] 很相似，[`Subscription`] 本身不会产生任何效果。
/// 要让 [`Subscription`] 运行，必须把它返回给 iced 运行时——通常是在
/// `application` 或 `daemon` 的 `subscription` 函数中。
///
/// 当一个 [`Subscription`] 第一次被交给运行时，运行时会开始异步地运行它。
/// 运行一个 [`Subscription`] 就是构建它底层的 [`Stream`]，
/// 并在异步运行时中执行它。
///
/// 因此，你可以把 [`Subscription`] 看作一个“流构建器”。它只是表示了
/// 构建某个 [`Stream`] 的方式，以及用来_标识_它的方式。
///
/// 标识之所以重要，是因为当某个 [`Subscription`] 不再被返回给 iced 运行时，
/// 运行时就会终止与它关联的 [`Stream`]。运行时使用 [`Subscription`] 的标识
/// 来跟踪它。
///
/// 通过这种方式，iced 让你可以在需要的时候，以声明式的方式临时__订阅__
/// 特定的数据流。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Much like a [`Future`] or a [`Stream`], a [`Subscription`] does not produce any effects
/// on its own. For a [`Subscription`] to run, it must be returned to the iced runtime—normally
/// in the `subscription` function of an `application` or a `daemon`.
///
/// When a [`Subscription`] is provided to the runtime for the first time, the runtime will
/// start running it asynchronously. Running a [`Subscription`] consists in building its underlying
/// [`Stream`] and executing it in an async runtime.
///
/// Therefore, you can think of a [`Subscription`] as a "stream builder". It simply represents a way
/// to build a certain [`Stream`] together with some way to _identify_ it.
///
/// Identification is important because when a specific [`Subscription`] stops being returned to the
/// iced runtime, the runtime will kill its associated [`Stream`]. The runtime uses the identity of a
/// [`Subscription`] to keep track of it.
///
/// This way, iced allows you to declaratively __subscribe__ to particular streams of data temporarily
/// and whenever necessary.
///
/// </details>
///
/// ```
/// # mod iced {
/// #     pub mod time {
/// #         pub use iced_futures::backend::default::time::every;
/// #         pub use std::time::{Duration, Instant};
/// #     }
/// #
/// #     pub use iced_futures::Subscription;
/// # }
/// use iced::time::{self, Duration, Instant};
/// use iced::Subscription;
///
/// struct State {
///     timer_enabled: bool,
/// }
///
/// fn subscription(state: &State) -> Subscription<Instant> {
///     if state.timer_enabled {
///         time::every(Duration::from_secs(1))
///     } else {
///         Subscription::none()
///     }
/// }
/// ```
///
/// [`Future`]: std::future::Future
#[must_use = "`Subscription` must be returned to the runtime to take effect; normally in your `subscription` function."]
pub struct Subscription<T> {
    recipes: Vec<Box<dyn Recipe<Output = T>>>,
}

impl<T> Subscription<T> {
    /// 返回一个不会产生任何输出的空 [`Subscription`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns an empty [`Subscription`] that will not produce any output.
    ///
    /// </details>
    pub fn none() -> Self {
        Self {
            recipes: Vec::new(),
        }
    }

    /// 返回一个 [`Subscription`]，它会调用给定的函数来创建
    /// 给定的 [`Stream`] 并异步地运行它。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a [`Subscription`] that will call the given function to create and
    /// asynchronously run the given [`Stream`].
    ///
    /// </details>
    ///
    /// # 创建可双向通信的异步工作者（Creating an asynchronous worker with bidirectional communication）
    /// 你可以利用这个辅助函数创建一个 [`Subscription`]，在后台派生
    /// 一个异步工作者，并与 `iced` 应用程序建立
    /// 通信通道。
    ///
    /// 做法是在闭包内创建一个 `mpsc` 通道，并把 `Sender`
    /// 作为 `Message` 返回给 `Application`：
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// You can leverage this helper to create a [`Subscription`] that spawns
    /// an asynchronous worker in the background and establish a channel of
    /// communication with an `iced` application.
    ///
    /// You can achieve this by creating an `mpsc` channel inside the closure
    /// and returning the `Sender` as a `Message` for the `Application`:
    ///
    /// </details>
    ///
    /// ```
    /// # mod iced {
    /// #     pub use iced_futures::Subscription;   
    /// #     pub use iced_futures::futures;
    /// #     pub use iced_futures::stream;
    /// # }
    /// use iced::futures::channel::mpsc;
    /// use iced::futures::sink::SinkExt;
    /// use iced::futures::Stream;
    /// use iced::stream;
    /// use iced::Subscription;
    ///
    /// pub enum Event {
    ///     Ready(mpsc::Sender<Input>),
    ///     WorkFinished,
    ///     // ...
    /// }
    ///
    /// enum Input {
    ///     DoSomeWork,
    ///     // ...
    /// }
    ///
    /// fn some_worker() -> impl Stream<Item = Event> {
    ///     stream::channel(100, async |mut output| {
    ///         // Create channel
    ///         // 创建通道
    ///         let (sender, mut receiver) = mpsc::channel(100);
    ///
    ///         // Send the sender back to the application
    ///         // 把发送端传回应用程序
    ///         output.send(Event::Ready(sender)).await;
    ///
    ///         loop {
    ///             use iced_futures::futures::StreamExt;
    ///
    ///             // Read next input sent from `Application`
    ///             // 读取 `Application` 发来的下一个输入
    ///             let input = receiver.select_next_some().await;
    ///
    ///             match input {
    ///                 Input::DoSomeWork => {
    ///                     // Do some async work...
    ///                     // 执行一些异步工作……
    ///
    ///                     // Finally, we can optionally produce a message to tell the
    ///                     // 最后，我们可以选择产生一条消息，告诉
    ///                     // `Application` the work is done
    ///                     // `Application` 工作已完成
    ///                     output.send(Event::WorkFinished).await;
    ///                 }
    ///             }
    ///         }
    ///     })
    /// }
    ///
    /// fn subscription() -> Subscription<Event> {
    ///     Subscription::run(some_worker)
    /// }
    /// ```
    ///
    /// 可以参考 [`websocket`] 示例，它演示了如何用这种模式保持
    /// `WebSocket` 连接。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Check out the [`websocket`] example, which showcases this pattern to maintain a `WebSocket`
    /// connection open.
    ///
    /// </details>
    ///
    /// [`websocket`]: https://github.com/iced-rs/iced/tree/master/examples/websocket
    pub fn run<S>(builder: fn() -> S) -> Self
    where
        S: Stream<Item = T> + MaybeSend + 'static,
        T: 'static,
    {
        from_recipe(Runner {
            data: builder,
            spawn: |builder, _| builder(),
        })
    }

    /// 返回一个 [`Subscription`]，它会创建给定的 [`Stream`] 并
    /// 异步地运行它。
    ///
    /// `data` 和函数指针都会被用来唯一标识
    /// 这个 [`Subscription`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a [`Subscription`] that will create and asynchronously run the
    /// given [`Stream`].
    ///
    /// Both the `data` and the function pointer will be used to uniquely identify
    /// the [`Subscription`].
    ///
    /// </details>
    pub fn run_with<D, S>(data: D, builder: fn(&D) -> S) -> Self
    where
        D: Hash + 'static,
        S: Stream<Item = T> + MaybeSend + 'static,
        T: 'static,
    {
        from_recipe(Runner {
            data: (data, builder),
            spawn: |(data, builder), _| builder(data),
        })
    }

    /// 把给定的所有订阅合并成一批，并返回得到的
    /// [`Subscription`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Batches all the provided subscriptions and returns the resulting
    /// [`Subscription`].
    ///
    /// </details>
    pub fn batch(subscriptions: impl IntoIterator<Item = Subscription<T>>) -> Self {
        Self {
            recipes: subscriptions
                .into_iter()
                .flat_map(|subscription| subscription.recipes)
                .collect(),
        }
    }

    /// 向 [`Subscription`] 的上下文中添加一个值。
    ///
    /// 这个值会成为 [`Subscription`] 标识的一部分。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adds a value to the [`Subscription`] context.
    ///
    /// The value will be part of the identity of a [`Subscription`].
    ///
    /// </details>
    pub fn with<A>(self, value: A) -> Subscription<(A, T)>
    where
        T: 'static,
        A: std::hash::Hash + Clone + Send + Sync + 'static,
    {
        struct With<A, B> {
            recipe: Box<dyn Recipe<Output = A>>,
            value: B,
        }

        impl<A, B> Recipe for With<A, B>
        where
            A: 'static,
            B: 'static + std::hash::Hash + Clone + Send + Sync,
        {
            type Output = (B, A);

            fn hash(&self, state: &mut Hasher) {
                std::any::TypeId::of::<B>().hash(state);
                self.value.hash(state);
                self.recipe.hash(state);
            }

            fn stream(self: Box<Self>, input: EventStream) -> BoxStream<Self::Output> {
                use futures::StreamExt;

                let value = self.value;

                Box::pin(
                    self.recipe
                        .stream(input)
                        .map(move |element| (value.clone(), element)),
                )
            }
        }

        Subscription {
            recipes: self
                .recipes
                .into_iter()
                .map(|recipe| {
                    Box::new(With {
                        recipe,
                        value: value.clone(),
                    }) as Box<dyn Recipe<Output = (A, T)>>
                })
                .collect(),
        }
    }

    /// 用给定的函数转换 [`Subscription`] 的输出。
    ///
    /// 提供的闭包必须是不捕获环境的闭包。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Transforms the [`Subscription`] output with the given function.
    ///
    /// The closure provided must be a non-capturing closure.
    ///
    /// </details>
    pub fn map<F, A>(self, f: F) -> Subscription<A>
    where
        T: 'static,
        F: Fn(T) -> A + MaybeSend + Clone + 'static,
        A: 'static,
    {
        const {
            check_zero_sized::<F>();
        }

        struct Map<A, B, F>
        where
            F: Fn(A) -> B + 'static,
        {
            recipe: Box<dyn Recipe<Output = A>>,
            mapper: F,
        }

        impl<A, B, F> Recipe for Map<A, B, F>
        where
            A: 'static,
            B: 'static,
            F: Fn(A) -> B + 'static + MaybeSend,
        {
            type Output = B;

            fn hash(&self, state: &mut Hasher) {
                TypeId::of::<F>().hash(state);
                self.recipe.hash(state);
            }

            fn stream(self: Box<Self>, input: EventStream) -> BoxStream<Self::Output> {
                use futures::StreamExt;

                Box::pin(self.recipe.stream(input).map(self.mapper))
            }
        }

        Subscription {
            recipes: self
                .recipes
                .into_iter()
                .map(|recipe| {
                    Box::new(Map {
                        recipe,
                        mapper: f.clone(),
                    }) as Box<dyn Recipe<Output = A>>
                })
                .collect(),
        }
    }

    /// 用给定的函数转换 [`Subscription`] 的输出，只有当函数返回
    /// `Some(A)` 时才产生值。
    ///
    /// 提供的闭包必须是不捕获环境的闭包。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Transforms the [`Subscription`] output with the given function, yielding only
    /// values only when the function returns `Some(A)`.
    ///
    /// The closure provided must be a non-capturing closure.
    ///
    /// </details>
    pub fn filter_map<F, A>(mut self, f: F) -> Subscription<A>
    where
        T: MaybeSend + 'static,
        F: Fn(T) -> Option<A> + MaybeSend + Clone + 'static,
        A: MaybeSend + 'static,
    {
        const {
            check_zero_sized::<F>();
        }

        struct FilterMap<A, B, F>
        where
            F: Fn(A) -> Option<B> + 'static,
        {
            recipe: Box<dyn Recipe<Output = A>>,
            mapper: F,
        }

        impl<A, B, F> Recipe for FilterMap<A, B, F>
        where
            A: 'static,
            B: 'static + MaybeSend,
            F: Fn(A) -> Option<B> + MaybeSend,
        {
            type Output = B;

            fn hash(&self, state: &mut Hasher) {
                TypeId::of::<F>().hash(state);
                self.recipe.hash(state);
            }

            fn stream(self: Box<Self>, input: EventStream) -> BoxStream<Self::Output> {
                use futures::StreamExt;
                use futures::future;

                let mapper = self.mapper;

                Box::pin(
                    self.recipe
                        .stream(input)
                        .filter_map(move |a| future::ready(mapper(a))),
                )
            }
        }

        Subscription {
            recipes: self
                .recipes
                .drain(..)
                .map(|recipe| {
                    Box::new(FilterMap {
                        recipe,
                        mapper: f.clone(),
                    }) as Box<dyn Recipe<Output = A>>
                })
                .collect(),
        }
    }

    /// 返回这个 [`Subscription`] 中配方单元的数量。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the amount of recipe units in this [`Subscription`].
    ///
    /// </details>
    pub fn units(&self) -> usize {
        self.recipes.len()
    }
}

impl<T> Default for Subscription<T> {
    fn default() -> Self {
        Self::none()
    }
}

/// 根据描述它的 [`Recipe`] 创建一个 [`Subscription`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a [`Subscription`] from a [`Recipe`] describing it.
///
/// </details>
pub fn from_recipe<T>(recipe: impl Recipe<Output = T> + 'static) -> Subscription<T> {
    Subscription {
        recipes: vec![Box::new(recipe)],
    }
}

/// 返回 [`Subscription`] 中的各个配方。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns the different recipes of the [`Subscription`].
///
/// </details>
pub fn into_recipes<T>(subscription: Subscription<T>) -> Vec<Box<dyn Recipe<Output = T>>> {
    subscription.recipes
}

impl<T> std::fmt::Debug for Subscription<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Subscription").finish()
    }
}

/// 对一个 [`Subscription`] 的描述。
///
/// [`Recipe`] 是 [`Subscription`] 的内部定义。运行时用它来
/// 运行和标识订阅。你也可以用它创建
/// 自己的订阅！
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The description of a [`Subscription`].
///
/// A [`Recipe`] is the internal definition of a [`Subscription`]. It is used
/// by runtimes to run and identify subscriptions. You can use it to create your
/// own!
///
/// </details>
pub trait Recipe {
    /// 使用这个 [`Recipe`] 的 [`Subscription`] 将会产生的
    /// 事件。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The events that will be produced by a [`Subscription`] with this
    /// [`Recipe`].
    ///
    /// </details>
    type Output;

    /// 计算 [`Recipe`] 的哈希。
    ///
    /// 运行时用它来唯一标识一个 [`Subscription`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Hashes the [`Recipe`].
    ///
    /// This is used by runtimes to uniquely identify a [`Subscription`].
    ///
    /// </details>
    fn hash(&self, state: &mut Hasher);

    /// 执行 [`Recipe`]，并产生其 [`Subscription`] 的
    /// 事件流。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Executes the [`Recipe`] and produces the stream of events of its
    /// [`Subscription`].
    ///
    /// </details>
    fn stream(self: Box<Self>, input: EventStream) -> BoxStream<Self::Output>;
}

/// 根据一个可哈希的 id 和一个过滤函数创建 [`Subscription`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a [`Subscription`] from a hashable id and a filter function.
///
/// </details>
pub fn filter_map<I, F, T>(id: I, f: F) -> Subscription<T>
where
    I: Hash + 'static,
    F: Fn(Event) -> Option<T> + MaybeSend + 'static,
    T: 'static + MaybeSend,
{
    from_recipe(Runner {
        data: id,
        spawn: |_, events| {
            use futures::future;
            use futures::stream::StreamExt;

            events.filter_map(move |event| future::ready(f(event)))
        },
    })
}

struct Runner<I, F, S, T>
where
    F: FnOnce(&I, EventStream) -> S,
    S: Stream<Item = T>,
{
    data: I,
    spawn: F,
}

impl<I, F, S, T> Recipe for Runner<I, F, S, T>
where
    I: Hash + 'static,
    F: FnOnce(&I, EventStream) -> S,
    S: Stream<Item = T> + MaybeSend + 'static,
{
    type Output = T;

    fn hash(&self, state: &mut Hasher) {
        std::any::TypeId::of::<I>().hash(state);
        self.data.hash(state);
    }

    fn stream(self: Box<Self>, input: EventStream) -> BoxStream<Self::Output> {
        crate::boxed_stream((self.spawn)(&self.data, input))
    }
}

const fn check_zero_sized<T>() {
    if std::mem::size_of::<T>() != 0 {
        panic!(
            "The Subscription closure provided is not non-capturing. \
            Closures given to Subscription::map or filter_map cannot \
            capture external variables. If you need to capture state, \
            consider using Subscription::with."
        );
    }
}
