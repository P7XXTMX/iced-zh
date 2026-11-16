//! 创建运行时任务。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Create runtime tasks.
//!
//! </details>
use crate::Action;
use crate::core::widget;
use crate::futures::futures::channel::mpsc;
use crate::futures::futures::channel::oneshot;
use crate::futures::futures::future::{self, FutureExt};
use crate::futures::futures::stream::{self, Stream, StreamExt};
use crate::futures::{BoxStream, MaybeSend, boxed_stream};

use std::convert::Infallible;
use std::pin::Pin;
use std::sync::Arc;
use std::task;
use std::thread;

#[cfg(feature = "sipper")]
#[doc(no_inline)]
pub use sipper::{Never, Sender, Sipper, Straw, sipper, stream};

/// 由 iced 运行时执行的一组并发动作。
///
/// 一个 [`Task`] _可能_会产生若干个 `T` 类型的值。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A set of concurrent actions to be performed by the iced runtime.
///
/// A [`Task`] _may_ produce a bunch of values of type `T`.
///
/// </details>
#[must_use = "`Task` must be returned to the runtime to take effect; normally in your `update` or `new` functions."]
pub struct Task<T> {
    stream: Option<BoxStream<Action<T>>>,
    units: usize,
}

impl<T> Task<T> {
    /// 创建一个什么都不做的 [`Task`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Task`] that does nothing.
    ///
    /// </details>
    pub fn none() -> Self {
        Self {
            stream: None,
            units: 0,
        }
    }

    /// 创建一个立即产生给定值的新 [`Task`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Task`] that instantly produces the given value.
    ///
    /// </details>
    pub fn done(value: T) -> Self
    where
        T: MaybeSend + 'static,
    {
        Self {
            stream: Some(boxed_stream(stream::once(future::ready(Action::Output(
                value,
            ))))),
            units: 0,
        }
    }

    /// 创建一个 [`Task`]，它运行给定的 [`Future`] 直至完成，并用
    /// 给定的闭包映射其输出。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Task`] that runs the given [`Future`] to completion and maps its
    /// output with the given closure.
    ///
    /// </details>
    pub fn perform<A>(
        future: impl Future<Output = A> + MaybeSend + 'static,
        f: impl FnOnce(A) -> T + MaybeSend + 'static,
    ) -> Self
    where
        T: MaybeSend + 'static,
        A: MaybeSend + 'static,
    {
        Self::future(future.map(f))
    }

    /// 创建一个 [`Task`]，它运行给定的 [`Stream`] 直至完成，并用
    /// 给定的闭包映射每一项。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Task`] that runs the given [`Stream`] to completion and maps each
    /// item with the given closure.
    ///
    /// </details>
    pub fn run<A>(
        stream: impl Stream<Item = A> + MaybeSend + 'static,
        f: impl Fn(A) -> T + MaybeSend + 'static,
    ) -> Self
    where
        T: 'static,
    {
        Self::stream(stream.map(f))
    }

    /// 创建一个 [`Task`]，它运行给定的 [`Sipper`] 直至完成，用第一个闭包
    /// 映射进度，用第二个闭包映射输出。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Task`] that runs the given [`Sipper`] to completion, mapping
    /// progress with the first closure and the output with the second one.
    ///
    /// </details>
    #[cfg(feature = "sipper")]
    pub fn sip<S>(
        sipper: S,
        on_progress: impl FnMut(S::Item) -> T + MaybeSend + 'static,
        on_output: impl FnOnce(<S as Future>::Output) -> T + MaybeSend + 'static,
    ) -> Self
    where
        S: sipper::Core + MaybeSend + 'static,
        T: MaybeSend + 'static,
    {
        Self::stream(stream(sipper::sipper(move |sender| async move {
            on_output(sipper.with(on_progress).run(sender).await)
        })))
    }

    /// 合并给定的任务，产生一个会并行运行所有这些任务的
    /// [`Task`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Combines the given tasks and produces a single [`Task`] that will run all of them
    /// in parallel.
    ///
    /// </details>
    pub fn batch(tasks: impl IntoIterator<Item = Self>) -> Self
    where
        T: 'static,
    {
        let mut select_all = stream::SelectAll::new();
        let mut units = 0;

        for task in tasks.into_iter() {
            if let Some(stream) = task.stream {
                select_all.push(stream);
            }

            units += task.units;
        }

        Self {
            stream: Some(boxed_stream(select_all)),
            units,
        }
    }

    /// 用给定的闭包映射 [`Task`] 的输出。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Maps the output of a [`Task`] with the given closure.
    ///
    /// </details>
    pub fn map<O>(self, mut f: impl FnMut(T) -> O + MaybeSend + 'static) -> Task<O>
    where
        T: MaybeSend + 'static,
        O: MaybeSend + 'static,
    {
        self.then(move |output| Task::done(f(output)))
    }

    /// 用给定的闭包，为当前 [`Task`] 的每一个输出执行一个新的
    /// [`Task`]。
    ///
    /// 这是 [`Task`] 的单子式接口——类似于 [`Future`] 和
    /// [`Stream`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Performs a new [`Task`] for every output of the current [`Task`] using the
    /// given closure.
    ///
    /// This is the monadic interface of [`Task`]—analogous to [`Future`] and
    /// [`Stream`].
    ///
    /// </details>
    pub fn then<O>(self, mut f: impl FnMut(T) -> Task<O> + MaybeSend + 'static) -> Task<O>
    where
        T: MaybeSend + 'static,
        O: MaybeSend + 'static,
    {
        Task {
            stream: match self.stream {
                None => None,
                Some(stream) => Some(boxed_stream(stream.flat_map(move |action| {
                    match action.output() {
                        Ok(output) => f(output)
                            .stream
                            .unwrap_or_else(|| boxed_stream(stream::empty())),
                        Err(action) => boxed_stream(stream::once(async move { action })),
                    }
                }))),
            },
            units: self.units,
        }
    }

    /// 串联一个新的 [`Task`]，在当前任务完全结束后执行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Chains a new [`Task`] to be performed once the current one finishes completely.
    ///
    /// </details>
    pub fn chain(self, task: Self) -> Self
    where
        T: 'static,
    {
        match self.stream {
            None => task,
            Some(first) => match task.stream {
                None => Self {
                    stream: Some(first),
                    units: self.units,
                },
                Some(second) => Self {
                    stream: Some(boxed_stream(first.chain(second))),
                    units: self.units + task.units,
                },
            },
        }
    }

    /// 创建一个新的 [`Task`]，把当前任务的所有输出收集到一个 [`Vec`] 中。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Task`] that collects all the output of the current one into a [`Vec`].
    ///
    /// </details>
    pub fn collect(self) -> Task<Vec<T>>
    where
        T: MaybeSend + 'static,
    {
        match self.stream {
            None => Task::done(Vec::new()),
            Some(stream) => Task {
                stream: Some(boxed_stream(
                    stream::unfold(
                        (stream, Some(Vec::new())),
                        move |(mut stream, outputs)| async move {
                            let mut outputs = outputs?;

                            let Some(action) = stream.next().await else {
                                return Some((Some(Action::Output(outputs)), (stream, None)));
                            };

                            match action.output() {
                                Ok(output) => {
                                    outputs.push(output);

                                    Some((None, (stream, Some(outputs))))
                                }
                                Err(action) => Some((Some(action), (stream, Some(outputs)))),
                            }
                        },
                    )
                    .filter_map(future::ready),
                )),
                units: self.units,
            },
        }
    }

    /// 创建一个新的 [`Task`]，丢弃当前任务的结果。
    ///
    /// 当你只关心 [`Task`] 的副作用时很有用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Task`] that discards the result of the current one.
    ///
    /// Useful if you only care about the side effects of a [`Task`].
    ///
    /// </details>
    pub fn discard<O>(self) -> Task<O>
    where
        T: MaybeSend + 'static,
        O: MaybeSend + 'static,
    {
        self.then(|_| Task::none())
    }

    /// 创建一个新的 [`Task`]，可以用返回的 [`Handle`] 中止它。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Task`] that can be aborted with the returned [`Handle`].
    ///
    /// </details>
    pub fn abortable(self) -> (Self, Handle)
    where
        T: 'static,
    {
        let (stream, handle) = match self.stream {
            Some(stream) => {
                let (stream, handle) = stream::abortable(stream);

                (Some(boxed_stream(stream)), InternalHandle::Manual(handle))
            }
            None => (
                None,
                InternalHandle::Manual(stream::AbortHandle::new_pair().0),
            ),
        };

        (
            Self {
                stream,
                units: self.units,
            },
            Handle { internal: handle },
        )
    }

    /// 创建一个新的 [`Task`]，它运行给定的 [`Future`] 并产生
    /// 其输出。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Task`] that runs the given [`Future`] and produces
    /// its output.
    ///
    /// </details>
    pub fn future(future: impl Future<Output = T> + MaybeSend + 'static) -> Self
    where
        T: 'static,
    {
        Self::stream(stream::once(future))
    }

    /// 创建一个新的 [`Task`]，它运行给定的 [`Stream`] 并产生
    /// 其中的每一项。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Task`] that runs the given [`Stream`] and produces
    /// each of its items.
    ///
    /// </details>
    pub fn stream(stream: impl Stream<Item = T> + MaybeSend + 'static) -> Self
    where
        T: 'static,
    {
        Self {
            stream: Some(boxed_stream(
                stream::once(yield_now())
                    .filter_map(|_| async { None })
                    .chain(stream.map(Action::Output)),
            )),
            units: 1,
        }
    }

    /// 返回 [`Task`] 的工作“单元”数量。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the amount of work "units" of the [`Task`].
    ///
    /// </details>
    pub fn units(&self) -> usize {
        self.units
    }
}

impl<T> std::fmt::Debug for Task<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Task<{}>", std::any::type_name::<T>()))
            .field("units", &self.units)
            .finish()
    }
}

/// [`Task`] 的句柄，可以用来中止它。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A handle to a [`Task`] that can be used for aborting it.
///
/// </details>
#[derive(Debug, Clone)]
pub struct Handle {
    internal: InternalHandle,
}

#[derive(Debug, Clone)]
enum InternalHandle {
    Manual(stream::AbortHandle),
    AbortOnDrop(Arc<stream::AbortHandle>),
}

impl InternalHandle {
    pub fn as_ref(&self) -> &stream::AbortHandle {
        match self {
            InternalHandle::Manual(handle) => handle,
            InternalHandle::AbortOnDrop(handle) => handle.as_ref(),
        }
    }
}

impl Handle {
    /// 中止这个 [`Handle`] 对应的 [`Task`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Aborts the [`Task`] of this [`Handle`].
    ///
    /// </details>
    pub fn abort(&self) {
        self.internal.as_ref().abort();
    }

    /// 返回一个新的 [`Handle`]，当它的所有实例都被丢弃时，会调用
    /// [`Handle::abort`]。
    ///
    /// 如果 [`Handle`] 被克隆，只有在所有克隆都被丢弃之后
    /// 才会调用 [`Handle::abort`]。
    ///
    /// 如果你不想操心自己调用 [`Handle::abort`]，
    /// 这会非常有用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a new [`Handle`] that will call [`Handle::abort`] whenever
    /// all of its instances are dropped.
    ///
    /// If a [`Handle`] is cloned, [`Handle::abort`] will only be called
    /// once all of the clones are dropped.
    ///
    /// This can be really useful if you do not want to worry about calling
    /// [`Handle::abort`] yourself.
    ///
    /// </details>
    pub fn abort_on_drop(self) -> Self {
        match &self.internal {
            InternalHandle::Manual(handle) => Self {
                internal: InternalHandle::AbortOnDrop(Arc::new(handle.clone())),
            },
            InternalHandle::AbortOnDrop(_) => self,
        }
    }

    /// 如果这个 [`Handle`] 对应的 [`Task`] 已被中止，返回 `true`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns `true` if the [`Task`] of this [`Handle`] has been aborted.
    ///
    /// </details>
    pub fn is_aborted(&self) -> bool {
        self.internal.as_ref().is_aborted()
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        if let InternalHandle::AbortOnDrop(handle) = &mut self.internal {
            let handle = std::mem::replace(handle, Arc::new(stream::AbortHandle::new_pair().0));

            if let Some(handle) = Arc::into_inner(handle) {
                handle.abort();
            }
        }
    }
}

impl<T> Task<Option<T>> {
    /// 在这个任务之后执行一个新的 [`Task`]，仅当它产生 `Some` 值时才执行。
    ///
    /// 该值会传给闭包，用来创建后续的 [`Task`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Executes a new [`Task`] after this one, only when it produces `Some` value.
    ///
    /// The value is provided to the closure to create the subsequent [`Task`].
    ///
    /// </details>
    pub fn and_then<A>(self, f: impl Fn(T) -> Task<A> + MaybeSend + 'static) -> Task<A>
    where
        T: MaybeSend + 'static,
        A: MaybeSend + 'static,
    {
        self.then(move |option| option.map_or_else(Task::none, &f))
    }
}

impl<T, E> Task<Result<T, E>> {
    /// 在这个任务之后执行一个新的 [`Task`]，仅当它以 `Ok` 值成功时才执行。
    ///
    /// 成功的值会传给闭包，用来创建后续的 [`Task`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Executes a new [`Task`] after this one, only when it succeeds with an `Ok` value.
    ///
    /// The success value is provided to the closure to create the subsequent [`Task`].
    ///
    /// </details>
    pub fn and_then<A>(
        self,
        f: impl Fn(T) -> Task<Result<A, E>> + MaybeSend + 'static,
    ) -> Task<Result<A, E>>
    where
        T: MaybeSend + 'static,
        E: MaybeSend + 'static,
        A: MaybeSend + 'static,
    {
        self.then(move |result| result.map_or_else(|error| Task::done(Err(error)), &f))
    }

    /// 用给定的函数把这个 [`Task`] 的错误类型映射为另一种
    /// 类型。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Maps the error type of this [`Task`] to a different one using the given
    /// function.
    ///
    /// </details>
    pub fn map_err<E2>(self, f: impl Fn(E) -> E2 + MaybeSend + 'static) -> Task<Result<T, E2>>
    where
        T: MaybeSend + 'static,
        E: MaybeSend + 'static,
        E2: MaybeSend + 'static,
    {
        self.map(move |result| result.map_err(&f))
    }
}

impl<T> Default for Task<T> {
    fn default() -> Self {
        Self::none()
    }
}

impl<T> From<()> for Task<T> {
    fn from(_value: ()) -> Self {
        Self::none()
    }
}

/// 创建一个新的 [`Task`]，它运行给定的 [`widget::Operation`] 并产生
/// 其输出。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Task`] that runs the given [`widget::Operation`] and produces
/// its output.
///
/// </details>
pub fn widget<T>(operation: impl widget::Operation<T> + 'static) -> Task<T>
where
    T: Send + 'static,
{
    channel(move |sender| {
        let operation = widget::operation::map(Box::new(operation), move |value| {
            let _ = sender.clone().try_send(value);
        });

        Action::Widget(Box::new(operation))
    })
}

/// 创建一个新的 [`Task`]，它执行闭包返回的 [`Action`]，并产生
/// 送入 [`oneshot::Sender`] 的值。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Task`] that executes the [`Action`] returned by the closure and
/// produces the value fed to the [`oneshot::Sender`].
///
/// </details>
pub fn oneshot<T>(f: impl FnOnce(oneshot::Sender<T>) -> Action<T>) -> Task<T>
where
    T: MaybeSend + 'static,
{
    let (sender, receiver) = oneshot::channel();

    let action = f(sender);

    Task {
        stream: Some(boxed_stream(
            stream::once(async move { action }).chain(
                receiver
                    .into_stream()
                    .filter_map(|result| async move { Some(Action::Output(result.ok()?)) }),
            ),
        )),
        units: 1,
    }
}

/// 创建一个新的 [`Task`]，它执行闭包返回的 [`Action`]，并产生
/// 送入 [`mpsc::Sender`] 的各个值。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Task`] that executes the [`Action`] returned by the closure and
/// produces the values fed to the [`mpsc::Sender`].
///
/// </details>
pub fn channel<T>(f: impl FnOnce(mpsc::Sender<T>) -> Action<T>) -> Task<T>
where
    T: MaybeSend + 'static,
{
    let (sender, receiver) = mpsc::channel(1);

    let action = f(sender);

    Task {
        stream: Some(boxed_stream(
            stream::once(async move { action })
                .chain(receiver.map(|result| Action::Output(result))),
        )),
        units: 1,
    }
}

/// 创建一个新的 [`Task`]，它执行给定的 [`Action`]，不产生任何输出。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Task`] that executes the given [`Action`] and produces no output.
///
/// </details>
pub fn effect<T>(action: impl Into<Action<Infallible>>) -> Task<T> {
    let action = action.into();

    Task {
        stream: Some(boxed_stream(stream::once(async move {
            action.output().expect_err("no output")
        }))),
        units: 1,
    }
}

/// 返回 [`Task`] 底层的 [`Stream`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns the underlying [`Stream`] of the [`Task`].
///
/// </details>
pub fn into_stream<T>(task: Task<T>) -> Option<BoxStream<Action<T>>> {
    task.stream
}

/// 创建一个新的 [`Task`]，它会在一个新线程中运行给定的闭包。
///
/// 闭包通过 [`mpsc::Sender`] 发送的任何数据都会由
/// 这个 [`Task`] 产生。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Task`] that will run the given closure in a new thread.
///
/// Any data sent by the closure through the [`mpsc::Sender`] will be produced
/// by the [`Task`].
///
/// </details>
pub fn blocking<T>(f: impl FnOnce(mpsc::Sender<T>) + Send + 'static) -> Task<T>
where
    T: Send + 'static,
{
    let (sender, receiver) = mpsc::channel(1);

    let _ = thread::spawn(move || {
        f(sender);
    });

    Task::stream(receiver)
}

/// 创建一个新的 [`Task`]，它会在一个新线程中运行给定的
/// 可能失败的闭包。
///
/// 闭包通过 [`mpsc::Sender`] 发送的任何数据都会由
/// 这个 [`Task`] 产生。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Task`] that will run the given closure that can fail in a new
/// thread.
///
/// Any data sent by the closure through the [`mpsc::Sender`] will be produced
/// by the [`Task`].
///
/// </details>
pub fn try_blocking<T, E>(
    f: impl FnOnce(mpsc::Sender<T>) -> Result<(), E> + Send + 'static,
) -> Task<Result<T, E>>
where
    T: Send + 'static,
    E: Send + 'static,
{
    let (sender, receiver) = mpsc::channel(1);
    let (error_sender, error_receiver) = oneshot::channel();

    let _ = thread::spawn(move || {
        if let Err(error) = f(sender) {
            let _ = error_sender.send(Err(error));
        }
    });

    Task::stream(stream::select(
        receiver.map(Ok),
        stream::once(error_receiver).filter_map(async |result| result.ok()),
    ))
}

async fn yield_now() {
    struct YieldNow {
        yielded: bool,
    }

    impl Future for YieldNow {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<()> {
            if self.yielded {
                return task::Poll::Ready(());
            }

            self.yielded = true;

            cx.waker().wake_by_ref();

            task::Poll::Pending
        }
    }

    YieldNow { yielded: false }.await;
}
