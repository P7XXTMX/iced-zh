//! 配置你的窗口。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Configure your windows.
//!
//! </details>
#[cfg(target_os = "windows")]
#[path = "settings/windows.rs"]
pub mod platform;

#[cfg(target_os = "macos")]
#[path = "settings/macos.rs"]
mod platform;

#[cfg(target_os = "linux")]
#[path = "settings/linux.rs"]
mod platform;

#[cfg(target_arch = "wasm32")]
#[path = "settings/wasm.rs"]
mod platform;

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "linux",
    target_arch = "wasm32"
)))]
#[path = "settings/other.rs"]
mod platform;

use crate::Size;
use crate::window::{Icon, Level, Position};

pub use platform::PlatformSpecific;

/// 应用程序的窗口设置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The window settings of an application.
///
/// </details>
#[derive(Debug, Clone)]
pub struct Settings {
    /// 窗口的初始逻辑尺寸。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The initial logical dimensions of the window.
    ///
    /// </details>
    pub size: Size,

    /// 窗口启动时是否最大化。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the window should start maximized.
    ///
    /// </details>
    pub maximized: bool,

    /// 窗口启动时是否全屏。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the window should start fullscreen.
    ///
    /// </details>
    pub fullscreen: bool,

    /// 窗口的初始位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The initial position of the window.
    ///
    /// </details>
    pub position: Position,

    /// 窗口的最小尺寸。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The minimum size of the window.
    ///
    /// </details>
    pub min_size: Option<Size>,

    /// 窗口的最大尺寸。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The maximum size of the window.
    ///
    /// </details>
    pub max_size: Option<Size>,

    /// 窗口是否可见。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the window should be visible or not.
    ///
    /// </details>
    pub visible: bool,

    /// 窗口是否可以调整大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the window should be resizable or not.
    ///
    /// </details>
    pub resizable: bool,

    /// 标题栏是否带有关闭按钮
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the title bar has Close button or not
    ///
    /// </details>
    pub closeable: bool,

    /// 标题栏是否带有最小化按钮
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the title bar has Minimize button or not
    ///
    /// </details>
    pub minimizable: bool,

    /// 窗口是否带有边框、标题栏等装饰。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the window should have a border, a title bar, etc. or not.
    ///
    /// </details>
    pub decorations: bool,

    /// 窗口是否透明。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the window should be transparent.
    ///
    /// </details>
    pub transparent: bool,

    /// 窗口是否带有模糊背景。
    ///
    /// 注意：模糊效果是应用在透明窗口上的。你需要启用
    /// [`Settings::transparent`]，并通过 `Application::style`
    /// 为背景色设置合适的不透明度。
    ///
    /// 此选项仅在 macOS 和 Linux 上受支持。更多细节请阅读
    /// [winit 文档][winit]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the window should have blurry background.
    ///
    /// Note that the blurry effect is applied to the transparent window. You need to enable
    /// [`Settings::transparent`] and set a proper opacity value to the background color with
    /// `Application::style`.
    ///
    /// This option is only supported on macOS and Linux. Please read the [winit document][winit]
    /// for more details.
    ///
    /// </details>
    ///
    /// [winit]: https://docs.rs/winit/0.30/winit/window/struct.Window.html#method.set_blur
    pub blur: bool,

    /// 窗口的 [`Level`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The window [`Level`].
    ///
    /// </details>
    pub level: Level,

    /// 窗口的图标。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The icon of the window.
    ///
    /// </details>
    pub icon: Option<Icon>,

    /// 平台特定的设置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Platform specific settings.
    ///
    /// </details>
    pub platform_specific: PlatformSpecific,

    /// 当用户请求关闭窗口时（例如用户按下关闭按钮时），
    /// 窗口是否关闭。
    ///
    /// 如果你希望在窗口真正被销毁之前执行某些行为，这会很有用。
    /// 如果禁用此项，你必须使用 `window::close` 命令
    /// 手动关闭窗口。
    ///
    /// 默认启用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the window will close when the user requests it, e.g. when a user presses the
    /// close button.
    ///
    /// This can be useful if you want to have some behavior that executes before the window is
    /// actually destroyed. If you disable this, you must manually close the window with the
    /// `window::close` command.
    ///
    /// By default this is enabled.
    ///
    /// </details>
    pub exit_on_close_request: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            size: Size::new(1024.0, 768.0),
            maximized: false,
            fullscreen: false,
            position: Position::default(),
            min_size: None,
            max_size: None,
            visible: true,
            resizable: true,
            minimizable: true,
            closeable: true,
            decorations: true,
            transparent: false,
            blur: false,
            level: Level::default(),
            icon: None,
            exit_on_close_request: true,
            platform_specific: PlatformSpecific::default(),
        }
    }
}
