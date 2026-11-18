//! WebAssembly 的平台特定设置。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Platform specific settings for WebAssembly.
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformSpecific {
    /// 将被应用程序替换的 DOM 元素的
    /// 标识符。
    ///
    /// 如果设置为 `None`，应用程序会被追加到 HTML body 中。
    ///
    /// 默认设置为 `"iced"`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The identifier of a DOM element that will be replaced with the
    /// application.
    ///
    /// If set to `None`, the application will be appended to the HTML body.
    ///
    /// By default, it is set to `"iced"`.
    ///
    /// </details>
    pub target: Option<String>,
}

impl Default for PlatformSpecific {
    fn default() -> Self {
        Self {
            target: Some(String::from("iced")),
        }
    }
}
