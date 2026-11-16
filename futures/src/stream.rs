//! 创建异步数据流。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Create asynchronous streams of data.
//!
//! </details>
use futures::channel::mpsc;
use futures::stream::{self, Stream, StreamExt};

/// 创建一个新的 [`Stream`]，它产生由某个 [`Future`] 发送到
/// 闭包所得到的 [`mpsc::Sender`] 中的各个项。
///
/// 这是更符合人体工学的 [`stream::unfold`]：只需在 [`Future`]
/// 内部循环并向一个异步通道发布数据，就可以从“future 的世界”
/// 进入“流的世界”。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Stream`] that produces the items sent from a [`Future`]
/// to the [`mpsc::Sender`] provided to the closure.
///
/// This is a more ergonomic [`stream::unfold`], which allows you to go
/// from the "world of futures" to the "world of streams" by simply looping
/// and publishing to an async channel from inside a [`Future`].
///
/// </details>
pub fn channel<T>(size: usize, f: impl AsyncFnOnce(mpsc::Sender<T>)) -> impl Stream<Item = T> {
    let (sender, receiver) = mpsc::channel(size);

    let runner = stream::once(f(sender)).filter_map(|_| async { None });

    stream::select(receiver, runner)
}

/// 创建一个新的 [`Stream`]，它产生由某个可能失败的 [`Future`]
/// 发送到闭包所得到的 [`mpsc::Sender`] 中的各个项。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Stream`] that produces the items sent from a [`Future`]
/// that can fail to the [`mpsc::Sender`] provided to the closure.
///
/// </details>
pub fn try_channel<T, E>(
    size: usize,
    f: impl AsyncFnOnce(mpsc::Sender<T>) -> Result<(), E>,
) -> impl Stream<Item = Result<T, E>> {
    let (sender, receiver) = mpsc::channel(size);

    let runner = stream::once(f(sender)).filter_map(|result| async {
        match result {
            Ok(()) => None,
            Err(error) => Some(Err(error)),
        }
    });

    stream::select(receiver.map(Ok), runner)
}
