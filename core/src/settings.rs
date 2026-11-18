//! 配置你的应用程序。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Configure your application.
//!
//! </details>
use crate::{Font, Pixels};

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
    pub default_font: Font,

    /// 默认使用的文本大小。
    ///
    /// 默认值是 `16.0`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text size that will be used by default.
    ///
    /// The default value is `16.0`.
    ///
    /// </details>
    pub default_text_size: Pixels,

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
        Self {
            id: None,
            fonts: Vec::new(),
            default_font: Font::default(),
            default_text_size: Pixels(16.0),
            antialiasing: true,
            vsync: true,
        }
    }
}
