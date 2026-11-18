//! 加载并使用字体。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Load and use fonts.
//!
//! </details>
use std::hash::Hash;

/// 字体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A font.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Font {
    /// [`Font`] 的 [`Family`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Family`] of the [`Font`].
    ///
    /// </details>
    pub family: Family,
    /// [`Font`] 的 [`Weight`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Weight`] of the [`Font`].
    ///
    /// </details>
    pub weight: Weight,
    /// [`Font`] 的 [`Stretch`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Stretch`] of the [`Font`].
    ///
    /// </details>
    pub stretch: Stretch,
    /// [`Font`] 的 [`Style`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Style`] of the [`Font`].
    ///
    /// </details>
    pub style: Style,
}

impl Font {
    /// 具有常规 [`Weight`] 的非等宽无衬线字体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A non-monospaced sans-serif font with normal [`Weight`].
    ///
    /// </details>
    pub const DEFAULT: Font = Font {
        family: Family::SansSerif,
        weight: Weight::Normal,
        stretch: Stretch::Normal,
        style: Style::Normal,
    };

    /// 具有常规 [`Weight`] 的等宽字体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A monospaced font with normal [`Weight`].
    ///
    /// </details>
    pub const MONOSPACE: Font = Font {
        family: Family::Monospace,
        ..Self::DEFAULT
    };

    /// 使用给定的 [`Family::Name`] 和常规 [`Weight`]
    /// 创建一个非等宽的 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a non-monospaced [`Font`] with the given [`Family::Name`] and
    /// normal [`Weight`].
    ///
    /// </details>
    pub const fn with_name(name: &'static str) -> Self {
        Font {
            family: Family::Name(name),
            ..Self::DEFAULT
        }
    }
}

/// 字体族。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A font family.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Family {
    /// 所选字体族的名称。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The name of a font family of choice.
    ///
    /// </details>
    Name(&'static str),

    /// 衬线字体代表一种文字的正式文本风格。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Serif fonts represent the formal text style for a script.
    ///
    /// </details>
    Serif,

    /// 按照 CSS 中对这个术语的用法，无衬线字体的字形通常对比度较低，
    /// 笔画末端朴素——没有任何外扩、
    /// 横向笔画或其他装饰。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Glyphs in sans-serif fonts, as the term is used in CSS, are generally low
    /// contrast and have stroke endings that are plain — without any flaring,
    /// cross stroke, or other ornamentation.
    ///
    /// </details>
    #[default]
    SansSerif,

    /// 草书字体的字形通常采用更随意的书写风格，
    /// 效果看起来更像是用钢笔或毛笔手写的，而不是印刷的
    /// 字体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Glyphs in cursive fonts generally use a more informal script style, and
    /// the result looks more like handwritten pen or brush writing than printed
    /// letterwork.
    ///
    /// </details>
    Cursive,

    /// 奇幻字体主要是装饰性或表现性的字体，包含
    /// 对字符的装饰性或表现性呈现。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Fantasy fonts are primarily decorative or expressive fonts that contain
    /// decorative or expressive representations of characters.
    ///
    /// </details>
    Fantasy,

    /// 等宽字体的唯一标准是所有字形具有相同的
    /// 固定宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The sole criterion of a monospace font is that all glyphs have the same
    /// fixed width.
    ///
    /// </details>
    Monospace,
}

/// 文本的字重。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The weight of some text.
///
/// </details>
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Weight {
    Thin,
    ExtraLight,
    Light,
    #[default]
    Normal,
    Medium,
    Semibold,
    Bold,
    ExtraBold,
    Black,
}

/// 文本的字宽。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The width of some text.
///
/// </details>
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Stretch {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    #[default]
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

/// 文本的字形样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The style of some text.
///
/// </details>
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Style {
    #[default]
    Normal,
    Italic,
    Oblique,
}
