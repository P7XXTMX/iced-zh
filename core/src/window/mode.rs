/// 基于窗口的应用程序的模式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The mode of a window-based application.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// 应用程序显示在它自己的窗口中。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The application appears in its own window.
    ///
    /// </details>
    Windowed,

    /// 应用程序占满其当前显示器的整个屏幕。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The application takes the whole screen of its current monitor.
    ///
    /// </details>
    Fullscreen,

    /// 应用程序被隐藏
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The application is hidden
    ///
    /// </details>
    Hidden,
}
