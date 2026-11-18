use crate::core::{Font, Pixels};
use crate::graphics;

/// [`Compositor`] 的设置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The settings of a [`Compositor`].
///
/// </details>
///
/// [`Compositor`]: crate::window::Compositor
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
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
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            default_font: Font::default(),
            default_text_size: Pixels(16.0),
        }
    }
}

impl From<graphics::Settings> for Settings {
    fn from(settings: graphics::Settings) -> Self {
        Self {
            default_font: settings.default_font,
            default_text_size: settings.default_text_size,
        }
    }
}
