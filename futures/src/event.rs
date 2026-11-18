//! 监听运行时事件。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Listen to runtime events.
//!
//! </details>
use crate::MaybeSend;
use crate::core::event::{self, Event};
use crate::core::window;
use crate::subscription::{self, Subscription};

/// 返回一个 [`Subscription`]，监听所有被忽略的运行时事件。
///
/// 这个订阅会把任何没有被部件捕获的 [`Event`]
/// 通知给你的应用程序。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns a [`Subscription`] to all the ignored runtime events.
///
/// This subscription will notify your application of any [`Event`] that was
/// not captured by any widget.
///
/// </details>
pub fn listen() -> Subscription<Event> {
    listen_with(|event, status, _window| match status {
        event::Status::Ignored => Some(event),
        event::Status::Captured => None,
    })
}

/// 创建一个 [`Subscription`]，它监听所有运行时事件并用给定的函数
/// 进行过滤，据此产生消息。
///
/// 这个订阅会对运行时处理的每一个 [`Event`] 调用给定的函数。
/// 如果该函数：
///
/// - 返回 `None`，这个 [`Event`] 会被丢弃。
/// - 返回 `Some` 消息，则产生这条 `Message`。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a [`Subscription`] that listens and filters all the runtime events
/// with the provided function, producing messages accordingly.
///
/// This subscription will call the provided function for every [`Event`]
/// handled by the runtime. If the function:
///
/// - Returns `None`, the [`Event`] will be discarded.
/// - Returns `Some` message, the `Message` will be produced.
///
/// </details>
pub fn listen_with<Message>(
    f: fn(Event, event::Status, window::Id) -> Option<Message>,
) -> Subscription<Message>
where
    Message: 'static + MaybeSend,
{
    #[derive(Hash)]
    struct EventsWith;

    subscription::filter_map((EventsWith, f), move |event| match event {
        subscription::Event::Interaction {
            event: Event::Window(window::Event::RedrawRequested(_)),
            ..
        }
        | subscription::Event::SystemThemeChanged(_) => None,
        subscription::Event::Interaction {
            window,
            event,
            status,
        } => f(event, status, window),
    })
}

/// 创建一个 [`Subscription`]，它为每一个运行时事件产生一条消息，
/// 包括重绘请求事件。
///
/// **警告：** 如果不加过滤，这个 [`Subscription`] 可能会
/// 无限循环地产生消息。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a [`Subscription`] that produces a message for every runtime event,
/// including the redraw request events.
///
/// **Warning:** This [`Subscription`], if unfiltered, may produce messages in
/// an infinite loop.
///
/// </details>
pub fn listen_raw<Message>(
    f: fn(Event, event::Status, window::Id) -> Option<Message>,
) -> Subscription<Message>
where
    Message: 'static + MaybeSend,
{
    #[derive(Hash)]
    struct RawEvents;

    subscription::filter_map((RawEvents, f), move |event| match event {
        subscription::Event::Interaction {
            window,
            event,
            status,
        } => f(event, status, window),
        subscription::Event::SystemThemeChanged(_) => None,
    })
}
