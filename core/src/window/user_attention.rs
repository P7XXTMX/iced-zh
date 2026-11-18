/// 要请求的用户注意力类型。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The type of user attention to request.
///
/// </details>
///
/// ## 平台差异（Platform-specific）
///
/// - **X11：** 设置窗口管理器的 `XUrgencyHint`。不区分 [`Critical`] 和 [`Informational`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// - **X11:** Sets the WM's `XUrgencyHint`. No distinction between [`Critical`] and [`Informational`].
///
/// </details>
///
/// [`Critical`]: Self::Critical
/// [`Informational`]: Self::Informational
#[derive(Debug, Clone, Copy)]
pub enum UserAttention {
    /// ## 平台差异（Platform-specific）
    ///
    /// - **macOS：** 让程序坞图标持续弹跳，直到应用程序获得焦点。
    /// - **Windows：** 让窗口和任务栏按钮同时闪烁，直到应用程序获得焦点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// - **macOS:** Bounces the dock icon until the application is in focus.
    /// - **Windows:** Flashes both the window and the taskbar button until the application is in focus.
    ///
    /// </details>
    Critical,
    /// ## 平台差异（Platform-specific）
    ///
    /// - **macOS：** 让程序坞图标弹跳一次。
    /// - **Windows：** 让任务栏按钮闪烁，直到应用程序获得焦点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// - **macOS:** Bounces the dock icon once.
    /// - **Windows:** Flashes the taskbar button until the application is in focus.
    ///
    /// </details>
    Informational,
}
