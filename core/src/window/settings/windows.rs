//! Windows 的平台特定设置。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Platform specific settings for Windows.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformSpecific {
    /// 拖放支持
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Drag and drop support
    ///
    /// </details>
    pub drag_and_drop: bool,

    /// 是否在任务栏中显示或隐藏窗口图标。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether show or hide the window icon in the taskbar.
    ///
    /// </details>
    pub skip_taskbar: bool,

    /// 显示或隐藏无装饰窗口的背景投影。
    ///
    /// 阴影默认是隐藏的。
    /// 启用阴影会导致窗口顶部出现一条 1px 的细线。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Shows or hides the background drop shadow for undecorated windows.
    ///
    /// The shadow is hidden by default.
    /// Enabling the shadow causes a thin 1px line to appear on the top of the window.
    ///
    /// </details>
    pub undecorated_shadow: bool,

    /// 设置窗口圆角的首选样式。
    ///
    /// 从 Windows 11 Build 22000 开始支持。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the preferred style of the window corners.
    ///
    /// Supported starting with Windows 11 Build 22000.
    ///
    /// </details>
    pub corner_preference: CornerPreference,
}

impl Default for PlatformSpecific {
    fn default() -> Self {
        Self {
            drag_and_drop: true,
            skip_taskbar: false,
            undecorated_shadow: false,
            corner_preference: Default::default(),
        }
    }
}

/// 描述窗口的角应该是什么样子。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Describes how the corners of a window should look like.
///
/// </details>
#[repr(i32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CornerPreference {
    /// 对应 `DWMWCP_DEFAULT`。
    ///
    /// 由系统决定何时把窗口的角变为圆角。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Corresponds to `DWMWCP_DEFAULT`.
    ///
    /// Let the system decide when to round window corners.
    ///
    /// </details>
    #[default]
    Default = 0,

    /// 对应 `DWMWCP_DONOTROUND`。
    ///
    /// 从不把窗口的角变为圆角。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Corresponds to `DWMWCP_DONOTROUND`.
    ///
    /// Never round window corners.
    ///
    /// </details>
    DoNotRound = 1,

    /// 对应 `DWMWCP_ROUND`。
    ///
    /// 在合适的情况下使用圆角。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Corresponds to `DWMWCP_ROUND`.
    ///
    /// Round the corners, if appropriate.
    ///
    /// </details>
    Round = 2,

    /// 对应 `DWMWCP_ROUNDSMALL`。
    ///
    /// 在合适的情况下使用小半径的圆角。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Corresponds to `DWMWCP_ROUNDSMALL`.
    ///
    /// Round the corners if appropriate, with a small radius.
    ///
    /// </details>
    RoundSmall = 3,
}
