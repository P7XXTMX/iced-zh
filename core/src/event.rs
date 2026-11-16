//! 处理用户界面的事件。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Handle events of a user interface.
//!
//! </details>
use crate::clipboard;
use crate::input_method;
use crate::keyboard;
use crate::mouse;
use crate::touch;
use crate::window;

/// 用户界面事件。
///
/// _**注意：** 这个类型还很不完整！如果你需要跟踪
/// 其他事件，欢迎[提交 issue][open an issue] 并分享你的使用场景！_
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A user interface event.
///
/// _**Note:** This type is largely incomplete! If you need to track
/// additional events, feel free to [open an issue] and share your use case!_
///
/// </details>
///
/// [open an issue]: https://github.com/iced-rs/iced/issues
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// 键盘事件
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A keyboard event
    ///
    /// </details>
    Keyboard(keyboard::Event),

    /// 鼠标事件
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A mouse event
    ///
    /// </details>
    Mouse(mouse::Event),

    /// 窗口事件
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A window event
    ///
    /// </details>
    Window(window::Event),

    /// 触摸事件
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A touch event
    ///
    /// </details>
    Touch(touch::Event),

    /// 输入法事件
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An input method event
    ///
    /// </details>
    InputMethod(input_method::Event),

    /// 剪贴板事件
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A clipboard event
    ///
    /// </details>
    Clipboard(clipboard::Event),

    /// 唤醒事件，由 [`Waker`](crate::shell::Waker) 产生。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A wake event, produced by a [`Waker`](crate::shell::Waker).
    ///
    /// </details>
    Waken,
}

/// 一个 [`Event`] 被处理之后的状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The status of an [`Event`] after being processed.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// 这个 [`Event`] **没有**被任何部件处理。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Event`] was **NOT** handled by any widget.
    ///
    /// </details>
    Ignored,

    /// 这个 [`Event`] 已被某个部件处理。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Event`] was handled and processed by a widget.
    ///
    /// </details>
    Captured,
}

impl Status {
    /// 把两个 [`Status`] 合并为一个。
    ///
    /// `Captured` 优先于 `Ignored`：
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Merges two [`Status`] into one.
    ///
    /// `Captured` takes precedence over `Ignored`:
    ///
    /// </details>
    ///
    /// ```
    /// use iced_core::event::Status;
    ///
    /// assert_eq!(Status::Ignored.merge(Status::Ignored), Status::Ignored);
    /// assert_eq!(Status::Ignored.merge(Status::Captured), Status::Captured);
    /// assert_eq!(Status::Captured.merge(Status::Ignored), Status::Captured);
    /// assert_eq!(Status::Captured.merge(Status::Captured), Status::Captured);
    /// ```
    pub fn merge(self, b: Self) -> Self {
        match self {
            Status::Ignored => b,
            Status::Captured => Status::Captured,
        }
    }
}
