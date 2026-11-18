use crate::{Point, Size};

/// 窗口在给定屏幕中的位置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The position of a window in a given screen.
///
/// </details>
#[derive(Debug, Clone, Copy, Default)]
pub enum Position {
    /// 新窗口在各平台上的默认位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The platform-specific default position for a new window.
    ///
    /// </details>
    #[default]
    Default,
    /// 窗口在屏幕上完全居中。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The window is completely centered on the screen.
    ///
    /// </details>
    Centered,
    /// 窗口按指定的坐标 `(X, Y)` 定位。
    ///
    /// 当窗口启用了装饰时，Windows 10 会给窗口添加一些
    /// 不可见的内边距。这些内边距会被计入
    /// 位置。因此，如果你启用了装饰并希望窗口位于
    /// (0, 0)，就必须把位置设置为
    /// `(PADDING_X, PADDING_Y)`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The window is positioned with specific coordinates: `(X, Y)`.
    ///
    /// When the decorations of the window are enabled, Windows 10 will add some
    /// invisible padding to the window. This padding gets included in the
    /// position. So if you have decorations enabled and want the window to be
    /// at (0, 0) you would have to set the position to
    /// `(PADDING_X, PADDING_Y)`.
    ///
    /// </details>
    Specific(Point),
    /// 与 [`Specific`] 类似，但窗口按给定函数返回的坐标定位。
    ///
    /// 该函数接收窗口大小和显示器的分辨率作为输入。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Like [`Specific`], but the window is positioned with the specific coordinates returned by the function.
    ///
    /// The function receives the window size and the monitor's resolution as input.
    ///
    /// </details>
    ///
    /// [`Specific`]: Self::Specific
    SpecificWith(fn(Size, Size) -> Point),
}
