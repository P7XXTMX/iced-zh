//! macOS 的平台特定设置。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Platform specific settings for macOS.
//!
//! </details>

/// 应用程序的平台特定窗口设置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The platform specific window settings of an application.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PlatformSpecific {
    /// 隐藏窗口标题。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Hides the window title.
    ///
    /// </details>
    pub title_hidden: bool,
    /// 让标题栏透明，并允许内容显示在它的后面。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Makes the titlebar transparent and allows the content to appear behind it.
    ///
    /// </details>
    pub titlebar_transparent: bool,
    /// 让窗口内容显示在标题栏的后面。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Makes the window content appear behind the titlebar.
    ///
    /// </details>
    pub fullsize_content_view: bool,
}
