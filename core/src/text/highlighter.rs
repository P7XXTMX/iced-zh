//! 高亮文本。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Highlight text.
//!
//! </details>
use crate::Color;
use crate::font;

/// 描述如何用某种 [`Style`] 高亮一个 `Input`
/// 的类型。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A type that describes how to highlight an `Input`
/// with some [`Style`].
///
/// </details>
pub trait Highlighter<Input, Theme = crate::Theme> {
    /// 高亮器的唯一标识符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A unique identifier for the highlighter.
    ///
    /// </details>
    fn id(&self) -> &str;

    /// 返回给定 `Input` 的 [`Style`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Style`] of the given `Input`.
    ///
    /// </details>
    fn highlight(&self, input: Input, theme: &Theme) -> Style;
}

/// 被高亮文本的样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The style of some highlighted text.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Style {
    /// 文本的 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Color`] of the text.
    ///
    /// </details>
    pub color: Option<Color>,
    /// 文本的 [`font::Style`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`font::Style`] of the text.
    ///
    /// </details>
    pub style: Option<font::Style>,
}

impl<T, Input, Theme> Highlighter<Input, Theme> for T
where
    T: Fn(Input, &Theme) -> Style,
{
    fn id(&self) -> &str {
        std::any::type_name_of_val(self) // Hack: Best effort（权宜之计：尽力而为）
    }

    fn highlight(&self, input: Input, theme: &Theme) -> Style {
        (self)(input, theme)
    }
}
