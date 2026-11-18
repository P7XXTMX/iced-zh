use crate::time::Instant;
use crate::{Point, Size};

use std::path::PathBuf;

/// 与窗口相关的事件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A window-related event.
///
/// </details>
#[derive(PartialEq, Clone, Debug)]
pub enum Event {
    /// 一个窗口被打开。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A window was opened.
    ///
    /// </details>
    Opened {
        /// 被打开窗口的位置。它相对于窗口所在桌面（包括虚拟桌面）
        /// 的左上角。指的是窗口的“外部”位置，
        /// 即窗口区域的位置，单位为逻辑像素。
        ///
        /// **注意**：在 Wayland 中不可用。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The position of the opened window. This is relative to the top-left corner of the desktop
        /// the window is on, including virtual desktops. Refers to window's "outer" position,
        /// or the window area, in logical pixels.
        ///
        /// **Note**: Not available in Wayland.
        ///
        /// </details>
        position: Option<Point>,
        /// 被创建窗口的大小。这是它的“内部”大小，即
        /// 客户区的大小，单位为逻辑像素。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The size of the created window. This is its "inner" size, or the size of the
        /// client area, in logical pixels.
        ///
        /// </details>
        size: Size,
    },

    /// 一个窗口被关闭。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A window was closed.
    ///
    /// </details>
    Closed,

    /// 一个窗口被移动。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A window was moved.
    ///
    /// </details>
    Moved(Point),

    /// 一个窗口的大小被调整。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A window was resized.
    ///
    /// </details>
    Resized(Size),

    /// 一个窗口的缩放因子发生了变化。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A window changed its scale factor.
    ///
    /// </details>
    Rescaled(f32),

    /// 请求重绘一个窗口。
    ///
    /// [`Instant`] 包含当前时间。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A window redraw was requested.
    ///
    /// The [`Instant`] contains the current time.
    ///
    /// </details>
    RedrawRequested(Instant),

    /// 用户请求关闭窗口。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The user has requested for the window to close.
    ///
    /// </details>
    CloseRequested,

    /// 一个窗口获得焦点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A window was focused.
    ///
    /// </details>
    Focused,

    /// 一个窗口失去焦点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A window was unfocused.
    ///
    /// </details>
    Unfocused,

    /// 一个文件正悬停在窗口上方。
    ///
    /// 当用户同时悬停多个文件时，会为每个文件
    /// 分别发出此事件。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A file is being hovered over the window.
    ///
    /// When the user hovers multiple files at once, this event will be emitted
    /// for each file separately.
    ///
    /// </details>
    ///
    /// ## 平台差异（Platform-specific）
    ///
    /// - **Wayland：** 未实现。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// - **Wayland:** Not implemented.
    ///
    /// </details>
    FileHovered(PathBuf),

    /// 一个文件被拖放到窗口中。
    ///
    /// 当用户同时拖放多个文件时，会为每个文件
    /// 分别发出此事件。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A file has been dropped into the window.
    ///
    /// When the user drops multiple files at once, this event will be emitted
    /// for each file separately.
    ///
    /// </details>
    ///
    /// ## 平台差异（Platform-specific）
    ///
    /// - **Wayland：** 未实现。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// - **Wayland:** Not implemented.
    ///
    /// </details>
    FileDropped(PathBuf),

    /// 一个文件曾悬停在窗口上方，但已经离开窗口。
    ///
    /// 即使悬停了多个文件，也只会触发一次
    /// `FilesHoveredLeft` 事件。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A file was hovered, but has exited the window.
    ///
    /// There will be a single `FilesHoveredLeft` event triggered even if
    /// multiple files were hovered.
    ///
    /// </details>
    ///
    /// ## 平台差异（Platform-specific）
    ///
    /// - **Wayland：** 未实现。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// - **Wayland:** Not implemented.
    ///
    /// </details>
    FilesHoveredLeft,
}
