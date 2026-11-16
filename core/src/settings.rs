//! 配置你的应用程序。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Configure your application.
//!
//! </details>
use crate::backend;
use crate::renderer;
use crate::text;
use crate::{Backend, Font, Pixels};

use std::borrow::Cow;

/// iced 程序的设置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The settings of an iced program.
///
/// </details>
#[derive(Debug, Clone)]
pub struct Settings {
    /// 应用程序的标识符。
    ///
    /// 如果提供，这个标识符可以用来标识应用程序，或者
    /// 通过窗口系统与它通信。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The identifier of the application.
    ///
    /// If provided, this identifier may be used to identify the application or
    /// communicate with it through the windowing system.
    ///
    /// </details>
    pub id: Option<String>,

    /// 启动时要加载的字体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The fonts to load on boot.
    ///
    /// </details>
    pub fonts: Vec<Cow<'static, [u8]>>,

    /// 要使用的默认 [`Font`]。
    ///
    /// 默认情况下，它使用 [`Family::SansSerif`](crate::font::Family::SansSerif)。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default [`Font`] to be used.
    ///
    /// By default, it uses [`Family::SansSerif`](crate::font::Family::SansSerif).
    ///
    /// </details>
    pub font: Font,

    /// 默认使用的文本大小。
    ///
    /// 默认情况下，它是 `16.0`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text size that will be used by default.
    ///
    /// By default, it is `16.0`.
    ///
    /// </details>
    pub text_size: Pixels,

    /// 文本的默认行高。
    ///
    /// 默认情况下，它是 `LineHeight::Relative(1.375)`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default line height of text.
    ///
    /// By default, it is `LineHeight::Relative(1.375)`.
    ///
    /// </details>
    pub line_height: text::LineHeight,

    /// 某些部件是否应该使用度量微调（metrics hinting）来渲染。
    ///
    /// 度量微调可以提高较小文本在低 DPI 屏幕上的
    /// 可读性，也可以提高渲染细线的部件的清晰度。
    ///
    /// 默认情况下，它是启用的。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether certain widgets should be rendered using metrics hinting.
    ///
    /// Metrics hinting can improve the readability of smaller text in
    /// low-DPI screens, as well as the clarity of widgets that render thin lines.
    ///
    /// By default, it is enabled.
    ///
    /// </details>
    pub metrics_hinting: bool,

    /// 要使用的图形后端。
    ///
    /// 默认情况下，它是 [`Backend::Best`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The graphical backend to use.
    ///
    /// By default, it is [`Backend::Best`].
    ///
    /// </details>
    pub backend: Backend,

    /// 后端的 [`PowerPreference`](backend::PowerPreference)。
    ///
    /// 默认情况下，它是 [`backend::PowerPreference::None`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`PowerPreference`](backend::PowerPreference) of the backend.
    ///
    /// By default, it is [`backend::PowerPreference::None`].
    ///
    /// </details>
    pub power_preference: backend::PowerPreference,

    /// 如果设置为 true，渲染器会尝试对某些图元
    /// 执行抗锯齿。
    ///
    /// 启用它可以让某些部件（比如
    /// `canvas` 部件）的结果更平滑，但会有性能开销。
    ///
    /// 默认情况下，它是启用的。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// If set to true, the renderer will try to perform antialiasing for some
    /// primitives.
    ///
    /// Enabling it can produce a smoother result in some widgets, like the
    /// `canvas` widget, at a performance cost.
    ///
    /// By default, it is enabled.
    ///
    /// </details>
    pub antialiasing: bool,

    /// 是否在可能的情况下尝试同步渲染。
    ///
    /// 禁用它可以在某些平台上提升渲染性能。
    ///
    /// 默认情况下，它是启用的。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether or not to attempt to synchronize rendering when possible.
    ///
    /// Disabling it can improve rendering performance on some platforms.
    ///
    /// By default, it is enabled.
    ///
    /// </details>
    pub vsync: bool,
}

impl Default for Settings {
    fn default() -> Self {
        let renderer = renderer::Settings::default();

        Self {
            id: None,
            fonts: Vec::new(),
            font: renderer.font,
            text_size: renderer.text_size,
            line_height: renderer.line_height,
            metrics_hinting: true,
            backend: Backend::default(),
            power_preference: backend::PowerPreference::None,
            antialiasing: true,
            vsync: true,
        }
    }
}

impl From<&Settings> for renderer::Settings {
    fn from(settings: &Settings) -> Self {
        Self {
            font: settings.font,
            text_size: settings.text_size,
            line_height: settings.line_height,
            metrics_hinting: settings.metrics_hinting,
        }
    }
}
