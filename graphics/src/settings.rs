use crate::Antialiasing;
use crate::core::{self, Font, Pixels};

/// 渲染器的设置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The settings of a renderer.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    /// 要使用的默认 [`Font`]。
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
    /// 默认情况下，它会被设置为 `16.0`。
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

    /// 是否同步帧。
    ///
    /// 默认为 `true`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether or not to synchronize frames.
    ///
    /// By default, it is `true`.
    ///
    /// </details>
    pub vsync: bool,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            default_font: Font::default(),
            default_text_size: Pixels(16.0),
            antialiasing: None,
            vsync: true,
        }
    }
}

impl From<core::Settings> for Settings {
    fn from(settings: core::Settings) -> Self {
        Self {
            default_font: if cfg!(all(
                target_arch = "wasm32",
                feature = "fira-sans"
            )) && settings.default_font == Font::default()
            {
                Font::with_name("Fira Sans")
            } else {
                settings.default_font
            },
            default_text_size: settings.default_text_size,
            antialiasing: settings.antialiasing.then_some(Antialiasing::MSAAx4),
            vsync: settings.vsync,
        }
    }
}
