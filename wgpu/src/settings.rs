//! 配置渲染器。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Configure a renderer.
//!
//! </details>
use crate::core::{Font, Pixels};
use crate::graphics::{self, Antialiasing};

/// [`Renderer`] 的设置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The settings of a [`Renderer`].
///
/// </details>
///
/// [`Renderer`]: crate::Renderer
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    /// [`Renderer`] 的呈现模式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The present mode of the [`Renderer`].
    ///
    /// </details>
    ///
    /// [`Renderer`]: crate::Renderer
    pub present_mode: wgpu::PresentMode,

    /// 要使用的图形后端。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The graphics backends to use.
    ///
    /// </details>
    pub backends: wgpu::Backends,

    /// 默认使用的 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default [`Font`] to use.
    ///
    /// </details>
    pub default_font: Font,

    /// 文本的默认大小。
    ///
    /// 默认情况下，它会被设为 `16.0`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default size of text.
    ///
    /// By default, it will be set to `16.0`.
    ///
    /// </details>
    pub default_text_size: Pixels,

    /// 用于三角形图元的抗锯齿策略。
    ///
    /// 默认情况下，它是 `None`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The antialiasing strategy that will be used for triangle primitives.
    ///
    /// By default, it is `None`.
    ///
    /// </details>
    pub antialiasing: Option<Antialiasing>,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            present_mode: wgpu::PresentMode::AutoVsync,
            backends: wgpu::Backends::all(),
            default_font: Font::default(),
            default_text_size: Pixels(16.0),
            antialiasing: None,
        }
    }
}

impl From<graphics::Settings> for Settings {
    fn from(settings: graphics::Settings) -> Self {
        Self {
            present_mode: if settings.vsync {
                wgpu::PresentMode::AutoVsync
            } else {
                wgpu::PresentMode::AutoNoVsync
            },
            default_font: settings.default_font,
            default_text_size: settings.default_text_size,
            antialiasing: settings.antialiasing,
            ..Settings::default()
        }
    }
}

/// 从当前的环境配置中获取 [`wgpu::PresentMode`]
/// （如果已设置）。
///
/// 可以通过设置 `ICED_PRESENT_MODE` 环境变量来改变
/// 这个函数的返回值。可用的值有：
///
/// - `vsync` → [`wgpu::PresentMode::AutoVsync`]
/// - `no_vsync` → [`wgpu::PresentMode::AutoNoVsync`]
/// - `immediate` → [`wgpu::PresentMode::Immediate`]
/// - `fifo` → [`wgpu::PresentMode::Fifo`]
/// - `fifo_relaxed` → [`wgpu::PresentMode::FifoRelaxed`]
/// - `mailbox` → [`wgpu::PresentMode::Mailbox`]
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Obtains a [`wgpu::PresentMode`] from the current environment
/// configuration, if set.
///
/// The value returned by this function can be changed by setting
/// the `ICED_PRESENT_MODE` env variable. The possible values are:
///
/// - `vsync` → [`wgpu::PresentMode::AutoVsync`]
/// - `no_vsync` → [`wgpu::PresentMode::AutoNoVsync`]
/// - `immediate` → [`wgpu::PresentMode::Immediate`]
/// - `fifo` → [`wgpu::PresentMode::Fifo`]
/// - `fifo_relaxed` → [`wgpu::PresentMode::FifoRelaxed`]
/// - `mailbox` → [`wgpu::PresentMode::Mailbox`]
///
/// </details>
pub fn present_mode_from_env() -> Option<wgpu::PresentMode> {
    let present_mode = std::env::var("ICED_PRESENT_MODE").ok()?;

    match present_mode.to_lowercase().as_str() {
        "vsync" => Some(wgpu::PresentMode::AutoVsync),
        "no_vsync" => Some(wgpu::PresentMode::AutoNoVsync),
        "immediate" => Some(wgpu::PresentMode::Immediate),
        "fifo" => Some(wgpu::PresentMode::Fifo),
        "fifo_relaxed" => Some(wgpu::PresentMode::FifoRelaxed),
        "mailbox" => Some(wgpu::PresentMode::Mailbox),
        _ => None,
    }
}
