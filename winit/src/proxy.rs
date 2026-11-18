use crate::futures::futures::{
    Future, Sink, StreamExt,
    channel::mpsc,
    select,
    task::{Context, Poll},
};
use crate::graphics::shell;
use crate::runtime::Action;
use crate::runtime::window;
use std::pin::Pin;

/// 带背压、实现了 `Sink` 的事件循环代理。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An event loop proxy with backpressure that implements `Sink`.
///
/// </details>
#[derive(Debug)]
pub struct Proxy<T: 'static> {
    raw: winit::event_loop::EventLoopProxy<Action<T>>,
    sender: mpsc::Sender<Action<T>>,
    notifier: mpsc::Sender<usize>,
}

impl<T: 'static> Clone for Proxy<T> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            sender: self.sender.clone(),
            notifier: self.notifier.clone(),
        }
    }
}

impl<T: 'static> Proxy<T> {
    const MAX_SIZE: usize = 100;

    /// 根据 `EventLoopProxy` 创建一个新的 [`Proxy`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Proxy`] from an `EventLoopProxy`.
    ///
    /// </details>
    pub fn new(
        raw: winit::event_loop::EventLoopProxy<Action<T>>,
    ) -> (Self, impl Future<Output = ()>) {
        let (notifier, mut processed) = mpsc::channel(Self::MAX_SIZE);
        let (sender, mut receiver) = mpsc::channel(Self::MAX_SIZE);
        let proxy = raw.clone();

        let worker = async move {
            let mut count = 0;

            loop {
                if count < Self::MAX_SIZE {
                    select! {
                        message = receiver.select_next_some() => {
                            let _ = proxy.send_event(message);
                            count += 1;

                        }
                        amount = processed.select_next_some() => {
                            count = count.saturating_sub(amount);
                        }
                        complete => break,
                    }
                } else {
                    select! {
                        amount = processed.select_next_some() => {
                            count = count.saturating_sub(amount);
                        }
                        complete => break,
                    }
                }
            }
        };

        (
            Self {
                raw,
                sender,
                notifier,
            },
            worker,
        )
    }

    /// 向事件循环发送一个值。
    ///
    /// 注意：它使用无界通道，绕过了背压机制。
    /// 请谨慎使用！
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sends a value to the event loop.
    ///
    /// Note: This skips the backpressure mechanism with an unbounded
    /// channel. Use sparingly!
    ///
    /// </details>
    pub fn send(&self, value: T) {
        self.send_action(Action::Output(value));
    }

    /// 向事件循环发送一个动作。
    ///
    /// 注意：它使用无界通道，绕过了背压机制。
    /// 请谨慎使用！
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sends an action to the event loop.
    ///
    /// Note: This skips the backpressure mechanism with an unbounded
    /// channel. Use sparingly!
    ///
    /// </details>
    pub fn send_action(&self, action: Action<T>) {
        let _ = self.raw.send_event(action);
    }

    /// 释放一定数量的槽位，让更多消息可以在
    /// 这个 [`Proxy`] 中排队。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Frees an amount of slots for additional messages to be queued in
    /// this [`Proxy`].
    ///
    /// </details>
    pub fn free_slots(&mut self, amount: usize) {
        let _ = self.notifier.start_send(amount);
    }
}

impl<T: 'static> Sink<Action<T>> for Proxy<T> {
    type Error = mpsc::SendError;

    fn poll_ready(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.sender.poll_ready(cx)
    }

    fn start_send(
        mut self: Pin<&mut Self>,
        action: Action<T>,
    ) -> Result<(), Self::Error> {
        self.sender.start_send(action)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        match self.sender.poll_ready(cx) {
            Poll::Ready(Err(ref e)) if e.is_disconnected() => {
                // If the receiver disconnected, we consider the sink to be flushed.
                // 如果接收端已断开连接，我们就认为这个 sink 已经刷新完毕。
                Poll::Ready(Ok(()))
            }
            x => x,
        }
    }

    fn poll_close(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.sender.disconnect();
        Poll::Ready(Ok(()))
    }
}

impl<T> shell::Notifier for Proxy<T>
where
    T: Send,
{
    fn request_redraw(&self) {
        self.send_action(Action::Window(window::Action::RedrawAll));
    }

    fn invalidate_layout(&self) {
        self.send_action(Action::Window(window::Action::RelayoutAll));
    }
}
