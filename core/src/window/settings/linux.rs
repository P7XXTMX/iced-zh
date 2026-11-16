//! Linux 的平台特定设置。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Platform specific settings for Linux.
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
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PlatformSpecific {
    /// 设置窗口的应用程序 id。
    ///
    /// 作为最佳实践，建议选择与应用程序 .desktop 文件的
    /// 基本名（basename）一致的应用程序 id。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the application id of the window.
    ///
    /// As a best practice, it is suggested to select an application id that match
    /// the basename of the application’s .desktop file.
    ///
    /// </details>
    pub application_id: String,

    /// 是否让 x11 窗口绕过窗口管理器的映射
    ///
    /// 此标志特别适合用来创建需要精确定位、立即显示
    /// 且不受窗口管理器干预的 UI 元素。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether bypass the window manager mapping for x11 windows
    ///
    /// This flag is particularly useful for creating UI elements that need precise
    /// positioning and immediate display without window manager interference.
    ///
    /// </details>
    pub override_redirect: bool,
}
