use crate::Point;

use super::Button;

/// 鼠标事件。
///
/// _**注意：** 这个类型还很不完整！如果你需要跟踪
/// 其他事件，欢迎[提交 issue][open an issue] 并分享你的使用场景！_
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A mouse event.
///
/// _**Note:** This type is largely incomplete! If you need to track
/// additional events, feel free to [open an issue] and share your use case!_
///
/// </details>
///
/// [open an issue]: https://github.com/iced-rs/iced/issues
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// 鼠标光标进入了窗口。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The mouse cursor entered the window.
    ///
    /// </details>
    CursorEntered,

    /// 鼠标光标离开了窗口。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The mouse cursor left the window.
    ///
    /// </details>
    CursorLeft,

    /// 鼠标光标被移动了
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The mouse cursor was moved
    ///
    /// </details>
    CursorMoved {
        /// 鼠标光标的新位置
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The new position of the mouse cursor
        ///
        /// </details>
        position: Point,
    },

    /// 某个鼠标按键被按下。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A mouse button was pressed.
    ///
    /// </details>
    ButtonPressed(Button),

    /// 某个鼠标按键被释放。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A mouse button was released.
    ///
    /// </details>
    ButtonReleased(Button),

    /// 鼠标滚轮被滚动。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The mouse wheel was scrolled.
    ///
    /// </details>
    WheelScrolled {
        /// 滚动位移。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The scroll movement.
        ///
        /// </details>
        delta: ScrollDelta,
    },
}

/// 滚动位移。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A scroll movement.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollDelta {
    /// 基于行的滚动位移
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A line-based scroll movement
    ///
    /// </details>
    Lines {
        /// 水平方向滚动的行数
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The number of horizontal lines scrolled
        ///
        /// </details>
        x: f32,

        /// 垂直方向滚动的行数
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The number of vertical lines scrolled
        ///
        /// </details>
        y: f32,
    },
    /// 基于像素的滚动位移
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A pixel-based scroll movement
    ///
    /// </details>
    Pixels {
        /// 水平方向滚动的像素数
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The number of horizontal pixels scrolled
        ///
        /// </details>
        x: f32,
        /// 垂直方向滚动的像素数
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The number of vertical pixels scrolled
        ///
        /// </details>
        y: f32,
    },
}
