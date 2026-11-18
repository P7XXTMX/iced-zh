use crate::core::event;
use crate::core::time::Instant;
use crate::core::window;

/// 可由某些部件执行的运行时动作。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A runtime action that can be performed by some widgets.
///
/// </details>
#[derive(Debug, Clone)]
pub struct Action<Message> {
    message_to_publish: Option<Message>,
    redraw_request: window::RedrawRequest,
    event_status: event::Status,
}

impl<Message> Action<Message> {
    fn new() -> Self {
        Self {
            message_to_publish: None,
            redraw_request: window::RedrawRequest::Wait,
            event_status: event::Status::Ignored,
        }
    }

    /// 创建一个新的“捕获型” [`Action`]。捕获型 [`Action`]
    /// 会让其他部件把它视为最终结果，并阻止进一步的
    /// 处理。
    ///
    /// 可以阻止“事件冒泡”。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new "capturing" [`Action`]. A capturing [`Action`]
    /// will make other widgets consider it final and prevent further
    /// processing.
    ///
    /// Prevents "event bubbling".
    ///
    /// </details>
    pub fn capture() -> Self {
        Self {
            event_status: event::Status::Captured,
            ..Self::new()
        }
    }

    /// 创建一个新的 [`Action`]，它会发布给定的 `Message`，
    /// 交由应用程序处理。
    ///
    /// 发布 `Message` 总是会触发一次重绘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Action`] that publishes the given `Message` for
    /// the application to handle.
    ///
    /// Publishing a `Message` always produces a redraw.
    ///
    /// </details>
    pub fn publish(message: Message) -> Self {
        Self {
            message_to_publish: Some(message),
            ..Self::new()
        }
    }

    /// 创建一个新的 [`Action`]，它请求尽快进行一次重绘，
    /// 但不发布任何 `Message`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Action`] that requests a redraw to happen as
    /// soon as possible; without publishing any `Message`.
    ///
    /// </details>
    pub fn request_redraw() -> Self {
        Self {
            redraw_request: window::RedrawRequest::NextFrame,
            ..Self::new()
        }
    }

    /// 创建一个新的 [`Action`]，它请求在给定的 [`Instant`]
    /// 进行一次重绘，但不发布任何 `Message`。
    ///
    /// 这可以用来高效地为内容制作动画，例如
    /// 文本输入框里闪烁的插入符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Action`] that requests a redraw to happen at
    /// the given [`Instant`]; without publishing any `Message`.
    ///
    /// This can be useful to efficiently animate content, like a
    /// blinking caret on a text input.
    ///
    /// </details>
    pub fn request_redraw_at(at: Instant) -> Self {
        Self {
            redraw_request: window::RedrawRequest::At(at),
            ..Self::new()
        }
    }

    /// 把 [`Action`] 标记为“捕获型”。参见 [`Self::capture`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Marks the [`Action`] as "capturing". See [`Self::capture`].
    ///
    /// </details>
    pub fn and_capture(mut self) -> Self {
        self.event_status = event::Status::Captured;
        self
    }

    /// 把 [`Action`] 转换为它的内部组成部分。
    ///
    /// 此方法供运行时、库或内部的部件实现
    /// 使用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Converts the [`Action`] into its internal parts.
    ///
    /// This method is meant to be used by runtimes, libraries, or internal
    /// widget implementations.
    ///
    /// </details>
    pub fn into_inner(
        self,
    ) -> (Option<Message>, window::RedrawRequest, event::Status) {
        (
            self.message_to_publish,
            self.redraw_request,
            self.event_status,
        )
    }
}
