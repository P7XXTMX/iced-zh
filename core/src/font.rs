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

    /// 使用给定的 [`Family::Name`] 和默认属性创建一个 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Font`] with the given [`Family::Name`] and default attributes.
    ///
    /// </details>
    pub const fn new(name: &'static str) -> Self {
        Self {
            family: Family::Name(name),
            ..Self::DEFAULT
        }
    }

    /// 使用给定的 [`Family`] 和默认属性创建一个 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Font`] with the given [`Family`] and default attributes.
    ///
    /// </details>
    pub fn with_family(family: impl Into<Family>) -> Self {
        Font {
            family: family.into(),
            ..Self::DEFAULT
        }
    }

    /// 设置 [`Font`] 的 [`Weight`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Weight`] of the [`Font`].
    ///
    /// </details>
    pub const fn weight(self, weight: Weight) -> Self {
        Self { weight, ..self }
    }

    /// 设置 [`Font`] 的 [`Stretch`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Stretch`] of the [`Font`].
    ///
    /// </details>
    pub const fn stretch(self, stretch: Stretch) -> Self {
        Self { stretch, ..self }
    }

    /// 设置 [`Font`] 的 [`Style`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Style`] of the [`Font`].
    ///
    /// </details>
    pub const fn style(self, style: Style) -> Self {
        Self { style, ..self }
    }
}

impl From<&'static str> for Font {
    fn from(name: &'static str) -> Self {
        Font::new(name)
    }
}

impl From<Family> for Font {
    fn from(family: Family) -> Self {
        Font::with_family(family)
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

impl Family {
    /// 所有独立的字体族变体的列表。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A list of all the different standalone family variants.
    ///
    /// </details>
    pub const VARIANTS: &[Self] = &[
        Self::Serif,
        Self::SansSerif,
        Self::Cursive,
        Self::Fantasy,
        Self::Monospace,
    ];

    /// 根据给定的字符串创建一个 [`Family::Name`]。
    ///
    /// 这个名称会被驻留在一个全局缓存中，并且永远不会释放。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Family::Name`] from the given string.
    ///
    /// The name is interned in a global cache and never freed.
    ///
    /// </details>
    pub fn name(name: &str) -> Self {
        use rustc_hash::FxHashSet;
        use std::sync::{LazyLock, Mutex};

        static NAMES: LazyLock<Mutex<FxHashSet<&'static str>>> = LazyLock::new(Mutex::default);

        let mut names = NAMES.lock().expect("lock font name cache");

        let Some(name) = names.get(name) else {
            let name: &'static str = name.to_owned().leak();
            let _ = names.insert(name);

            return Self::Name(name);
        };

        Self::Name(name)
    }
}

impl From<&str> for Family {
    fn from(name: &str) -> Self {
        Family::name(name)
    }
}

impl std::fmt::Display for Family {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Family::Name(name) => name,
            Family::Serif => "Serif",
            Family::SansSerif => "Sans-serif",
            Family::Cursive => "Cursive",
            Family::Fantasy => "Fantasy",
            Family::Monospace => "Monospace",
        })
    }
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

/// 字体错误。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A font error.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {}
