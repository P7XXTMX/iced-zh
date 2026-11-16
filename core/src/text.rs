//! 绘制文本并与之交互。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Draw and interact with text.
//!
//! </details>

pub mod editor;
pub mod highlighter;
pub mod input;
pub mod paragraph;
pub mod parser;

pub use editor::Editor;
pub use highlighter::Highlighter;
pub use input::Input;
pub use paragraph::Paragraph;
pub use parser::Parser;

use crate::alignment;
use crate::{Background, Border, Color, Font, Padding, Pixels, Point, Rectangle, Size};

use std::borrow::Cow;
use std::hash::{Hash, Hasher};

/// 段落。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A paragraph.
///
/// </details>
#[derive(Debug, Clone, Copy)]
pub struct Text<Content = String> {
    /// 段落的内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The content of the paragraph.
    ///
    /// </details>
    pub content: Content,

    /// 段落的边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The bounds of the paragraph.
    ///
    /// </details>
    pub bounds: Size,

    /// [`Text`] 的大小，以逻辑像素为单位。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The size of the [`Text`] in logical pixels.
    ///
    /// </details>
    pub size: Pixels,

    /// [`Text`] 的行高。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The line height of the [`Text`].
    ///
    /// </details>
    pub line_height: LineHeight,

    /// [`Text`] 的 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Font`] of the [`Text`].
    ///
    /// </details>
    pub font: Font,

    /// [`Text`] 的水平对齐方式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The horizontal alignment of the [`Text`].
    ///
    /// </details>
    pub align_x: Alignment,

    /// [`Text`] 的垂直对齐方式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The vertical alignment of the [`Text`].
    ///
    /// </details>
    pub align_y: alignment::Vertical,

    /// [`Text`] 的 [`Shaping`] 策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Shaping`] strategy of the [`Text`].
    ///
    /// </details>
    pub shaping: Shaping,

    /// [`Text`] 的 [`Wrapping`] 策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Wrapping`] strategy of the [`Text`].
    ///
    /// </details>
    pub wrapping: Wrapping,

    /// [`Text`] 的 [`Ellipsis`] 策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Ellipsis`] strategy of the [`Text`].
    ///
    /// </details>
    pub ellipsis: Ellipsis,

    /// 缩放因子，可用于在内部缩放 [`Paragraph`] 的布局
    /// 计算，并利用度量微调（metrics hinting）。
    ///
    /// 实际上，它定义了用于线性缩放的
    /// “基准”布局。
    ///
    /// 如果为 `None`，微调会被禁用，并执行
    /// 亚像素定位。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The scale factor that may be used to internally scale the layout
    /// calculation of the [`Paragraph`] and leverage metrics hinting.
    ///
    /// Effectively, this defines the "base" layout that will be used for
    /// linear scaling.
    ///
    /// If `None`, hinting will be disabled and subpixel positioning will be
    /// performed.
    ///
    /// </details>
    pub hint_factor: Option<f32>,
}

impl<Content> Text<Content> {
    /// 返回一个新的 [`Text`]，只把内容替换为
    /// 给定的值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a new [`Text`] replacing only the content with the
    /// given value.
    ///
    /// </details>
    pub fn with_content<T>(&self, content: T) -> Text<T> {
        Text {
            content,
            bounds: self.bounds,
            size: self.size,
            line_height: self.line_height,
            font: self.font,
            align_x: self.align_x,
            align_y: self.align_y,
            shaping: self.shaping,
            wrapping: self.wrapping,
            ellipsis: self.ellipsis,
            hint_factor: self.hint_factor,
        }
    }
}

impl<Content: AsRef<str>> Text<Content> {
    /// 返回 [`Text`] 的借用版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a borrowed version of [`Text`].
    ///
    /// </details>
    pub fn as_ref(&self) -> Text<&str> {
        self.with_content(self.content.as_ref())
    }
}

/// 某些文本的对齐方式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The alignment of some text.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Alignment {
    /// 没有特定的对齐方式。
    ///
    /// 从左到右的文本会左对齐，而
    /// 从右到左的文本会右对齐。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// No specific alignment.
    ///
    /// Left-to-right text will be aligned to the left, while
    /// right-to-left text will be aligned to the right.
    ///
    /// </details>
    #[default]
    Default,
    /// 文本左对齐。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Align text to the left.
    ///
    /// </details>
    Left,
    /// 文本居中。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Center text.
    ///
    /// </details>
    Center,
    /// 文本右对齐。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Align text to the right.
    ///
    /// </details>
    Right,
    /// 文本两端对齐。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Justify text.
    ///
    /// </details>
    Justified,
}

impl From<alignment::Horizontal> for Alignment {
    fn from(alignment: alignment::Horizontal) -> Self {
        match alignment {
            alignment::Horizontal::Left => Self::Left,
            alignment::Horizontal::Center => Self::Center,
            alignment::Horizontal::Right => Self::Right,
        }
    }
}

impl From<crate::Alignment> for Alignment {
    fn from(alignment: crate::Alignment) -> Self {
        match alignment {
            crate::Alignment::Start => Self::Left,
            crate::Alignment::Center => Self::Center,
            crate::Alignment::End => Self::Right,
        }
    }
}

impl From<Alignment> for alignment::Horizontal {
    fn from(alignment: Alignment) -> Self {
        match alignment {
            Alignment::Default | Alignment::Left | Alignment::Justified => {
                alignment::Horizontal::Left
            }
            Alignment::Center => alignment::Horizontal::Center,
            Alignment::Right => alignment::Horizontal::Right,
        }
    }
}

/// 某些文本的整形策略。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The shaping strategy of some text.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Shaping {
    /// 根据文本自动检测最佳的整形策略。
    ///
    /// 如果文本只包含 ASCII 字符，此策略会使用
    /// [`Basic`](Self::Basic) 整形；否则，它会
    /// 使用 [`Advanced`](Self::Advanced) 整形。
    ///
    /// 如果 `basic-shaping` 和 `advanced-shaping` 特性都没有启用，
    /// 这就是默认值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Auto-detect the best shaping strategy from the text.
    ///
    /// This strategy will use [`Basic`](Self::Basic) shaping if the
    /// text consists of only ASCII characters; otherwise, it will
    /// use [`Advanced`](Self::Advanced) shaping.
    ///
    /// This is the default, if neither the `basic-shaping` nor `advanced-shaping`
    /// features are enabled.
    ///
    /// </details>
    Auto,
    /// 不进行整形，也不进行字体回退。
    ///
    /// 这种整形策略开销很低，但它无法正确显示复杂的
    /// 文字，也不会尝试在系统字体中查找缺失的字形。
    ///
    /// 当你完全掌控应用程序中显示的文本和字体时，
    /// 应该使用这种策略。
    ///
    /// 如果启用了 `basic-shaping` 特性并且
    /// 禁用了 `advanced-shaping` 特性，这就是默认值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// No shaping and no font fallback.
    ///
    /// This shaping strategy is very cheap, but it will not display complex
    /// scripts properly nor try to find missing glyphs in your system fonts.
    ///
    /// You should use this strategy when you have complete control of the text
    /// and the font you are displaying in your application.
    ///
    /// This will be the default if the `basic-shaping` feature is enabled and
    /// the `advanced-shaping` feature is disabled.
    ///
    /// </details>
    Basic,
    /// 高级文本整形和字体回退。
    ///
    /// 如果文本包含复杂的文字、所用的字体需要它，
    /// 和（或）可能需要系统中的多种字体才能显示所有字形，
    /// 你就需要启用此标志。
    ///
    /// 高级整形开销很大！你应该只在必要时启用它。
    ///
    /// 如果启用了 `advanced-shaping` 特性，这就是默认值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Advanced text shaping and font fallback.
    ///
    /// You will need to enable this flag if the text contains a complex
    /// script, the font used needs it, and/or multiple fonts in your system
    /// may be needed to display all of the glyphs.
    ///
    /// Advanced shaping is expensive! You should only enable it when necessary.
    ///
    /// This will be the default if the `advanced-shaping` feature is enabled.
    ///
    /// </details>
    Advanced,
}

impl Default for Shaping {
    fn default() -> Self {
        if cfg!(feature = "advanced-shaping") {
            Self::Advanced
        } else if cfg!(feature = "basic-shaping") {
            Self::Basic
        } else {
            Self::Auto
        }
    }
}

/// 某些文本的换行策略。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The wrapping strategy of some text.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Wrapping {
    /// 不换行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// No wrapping.
    ///
    /// </details>
    None,
    /// 按单词换行。
    ///
    /// 这是默认值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Wraps at the word level.
    ///
    /// This is the default.
    ///
    /// </details>
    #[default]
    Word,
    /// 按字形换行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Wraps at the glyph level.
    ///
    /// </details>
    Glyph,
    /// 按单词换行；如果某个单词无法单独放进一行，则回退到按字形换行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Wraps at the word level, or fallback to glyph level if a word can't fit on a line by itself.
    ///
    /// </details>
    WordOrGlyph,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// 某些文本的省略策略。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The ellipsis strategy of some text.
///
/// </details>
pub enum Ellipsis {
    /// 不省略。
    ///
    /// 这是默认值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// No ellipsis.
    ///
    /// This is the default.
    ///
    /// </details>
    #[default]
    None,
    /// 在文本最后一个视觉行的开头处省略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Ellipsize the start of the last visual line in the text.
    ///
    /// </details>
    Start,
    /// 在文本最后一个视觉行的中间处省略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Ellipsize the middle of the last visual line in the text.
    ///
    /// </details>
    Middle,
    /// 在文本最后一个视觉行的末尾处省略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Ellipsize the end of the last visual line in the text.
    ///
    /// </details>
    End,
}

/// 段落中一行文本的高度。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The height of a line of text in a paragraph.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineHeight {
    /// 文本大小的倍数。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A factor of the size of the text.
    ///
    /// </details>
    Relative(f32),

    /// 绝对高度，以逻辑像素为单位。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An absolute height in logical pixels.
    ///
    /// </details>
    Absolute(Pixels),
}

impl LineHeight {
    /// 返回以绝对逻辑像素表示的 [`LineHeight`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`LineHeight`] in absolute logical pixels.
    ///
    /// </details>
    pub fn to_absolute(self, text_size: impl Into<Pixels>) -> Pixels {
        match self {
            Self::Relative(factor) => Pixels(factor * text_size.into().0),
            Self::Absolute(pixels) => pixels,
        }
    }
}

impl Default for LineHeight {
    fn default() -> Self {
        Self::Relative(1.375)
    }
}

impl From<f32> for LineHeight {
    fn from(factor: f32) -> Self {
        Self::Relative(factor)
    }
}

impl From<Pixels> for LineHeight {
    fn from(pixels: Pixels) -> Self {
        Self::Absolute(pixels)
    }
}

impl Hash for LineHeight {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Relative(factor) => {
                state.write_u8(0);
                factor.to_bits().hash(state);
            }
            Self::Absolute(pixels) => {
                state.write_u8(1);
                f32::from(*pixels).to_bits().hash(state);
            }
        }
    }
}

/// 对文本进行命中测试的结果。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The result of hit testing on text.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hit {
    /// 命中的行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The line of the hit.
    ///
    /// </details>
    pub line: usize,
    /// 最接近的字形的首字节索引。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The first byte index of the closest glyph.
    ///
    /// </details>
    pub index: usize,
}

/// 在某些文本中检测到的差异。
///
/// 当你把一个 [`Paragraph`] 与某个 [`Text`] 进行 [`compare`] 时，
/// 会得到一个 [`Difference`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The difference detected in some text.
///
/// You will obtain a [`Difference`] when you [`compare`] a [`Paragraph`] with some
/// [`Text`].
///
/// </details>
///
/// [`compare`]: Paragraph::compare
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difference {
    /// 没有差异。
    ///
    /// 文本可以原样复用！
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// No difference.
    ///
    /// The text can be reused as it is!
    ///
    /// </details>
    None,

    /// 边界差异。
    ///
    /// 这通常意味着需要重新布局，但文本的形状可以
    /// 复用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A bounds difference.
    ///
    /// This normally means a relayout is necessary, but the shape of the text can
    /// be reused.
    ///
    /// </details>
    Bounds,

    /// 形状差异。
    ///
    /// 文本形状的内容、对齐、大小、字体或任何其他关键属性
    /// 发生了变化。需要对文本进行完整的重新整形和
    /// 重新布局。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A shape difference.
    ///
    /// The contents, alignment, sizes, fonts, or any other essential attributes
    /// of the shape of the text have changed. A complete reshape and relayout of
    /// the text is necessary.
    ///
    /// </details>
    Shape,
}

/// 能够测量和绘制 [`Text`] 的渲染器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A renderer capable of measuring and drawing [`Text`].
///
/// </details>
pub trait Renderer: crate::Renderer {
    /// 此 [`Renderer`] 的 [`Paragraph`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Paragraph`] of this [`Renderer`].
    ///
    /// </details>
    type Paragraph: Paragraph + 'static;

    /// 此 [`Renderer`] 的 [`Editor`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Editor`] of this [`Renderer`].
    ///
    /// </details>
    type Editor: Editor + 'static;

    /// 后端的图标字体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The icon font of the backend.
    ///
    /// </details>
    const ICON_FONT: Font;

    /// 在 [`ICON_FONT`] 中表示 ✔ 图标的 `char`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The `char` representing a ✔ icon in the [`ICON_FONT`].
    ///
    /// </details>
    ///
    /// [`ICON_FONT`]: Self::ICON_FONT
    const CHECKMARK_ICON: char;

    /// 在内置的 [`ICON_FONT`] 中表示 ▼ 图标的 `char`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The `char` representing a ▼ icon in the built-in [`ICON_FONT`].
    ///
    /// </details>
    ///
    /// [`ICON_FONT`]: Self::ICON_FONT
    const ARROW_DOWN_ICON: char;

    /// 在内置的 [`ICON_FONT`] 中表示 ^ 图标的 `char`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The `char` representing a ^ icon in the built-in [`ICON_FONT`].
    ///
    /// </details>
    ///
    /// [`ICON_FONT`]: Self::ICON_FONT
    const SCROLL_UP_ICON: char;

    /// 在内置的 [`ICON_FONT`] 中表示 v 图标的 `char`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The `char` representing a v icon in the built-in [`ICON_FONT`].
    ///
    /// </details>
    ///
    /// [`ICON_FONT`]: Self::ICON_FONT
    const SCROLL_DOWN_ICON: char;

    /// 在内置的 [`ICON_FONT`] 中表示 < 图标的 `char`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The `char` representing a < icon in the built-in [`ICON_FONT`].
    ///
    /// </details>
    ///
    /// [`ICON_FONT`]: Self::ICON_FONT
    const SCROLL_LEFT_ICON: char;

    /// 在内置的 [`ICON_FONT`] 中表示 > 图标的 `char`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The `char` representing a > icon in the built-in [`ICON_FONT`].
    ///
    /// </details>
    ///
    /// [`ICON_FONT`]: Self::ICON_FONT
    const SCROLL_RIGHT_ICON: char;

    /// 在内置的 ['ICON_FONT'] 中表示 iced 徽标的 'char'。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The 'char' representing the iced logo in the built-in ['ICON_FONT'].
    ///
    /// </details>
    ///
    /// ['ICON_FONT']: Self::ICON_FONT
    const ICED_LOGO: char;

    /// 返回默认的 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the default [`Font`].
    ///
    /// </details>
    fn font(&self) -> Font {
        self.settings().font
    }

    /// 返回 [`Text`] 的默认大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the default size of [`Text`].
    ///
    /// </details>
    fn text_size(&self) -> Pixels {
        self.settings().text_size
    }

    /// 返回 [`Text`] 的默认行高。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the default line height of [`Text`].
    ///
    /// </details>
    fn line_height(&self) -> LineHeight {
        self.settings().line_height
    }

    /// 在给定的位置、使用给定的 [`Color`]
    /// 绘制给定的 [`Paragraph`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the given [`Paragraph`] at the given position and with the given
    /// [`Color`].
    ///
    /// </details>
    fn fill_paragraph(
        &mut self,
        text: &Self::Paragraph,
        position: Point,
        color: Color,
        clip_bounds: Rectangle,
    );

    /// 在给定的位置、使用给定的 [`Color`]
    /// 绘制给定的 [`Editor`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the given [`Editor`] at the given position and with the given
    /// [`Color`].
    ///
    /// </details>
    fn fill_editor(
        &mut self,
        editor: &Self::Editor,
        position: Point,
        color: Color,
        clip_bounds: Rectangle,
    );

    /// 在给定的位置、使用给定的 [`Color`]
    /// 绘制给定的 [`Text`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the given [`Text`] at the given position and with the given
    /// [`Color`].
    ///
    /// </details>
    fn fill_text(
        &mut self,
        text: Text<String>,
        position: Point,
        color: Color,
        clip_bounds: Rectangle,
    );
}

/// 一段文本（span）。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A span of text.
///
/// </details>
#[derive(Debug, Clone)]
pub struct Span<'a, Link = ()> {
    /// 文本的 [`Fragment`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Fragment`] of text.
    ///
    /// </details>
    pub text: Fragment<'a>,
    /// [`Span`] 的大小，以 [`Pixels`] 表示。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The size of the [`Span`] in [`Pixels`].
    ///
    /// </details>
    pub size: Option<Pixels>,
    /// [`Span`] 的 [`LineHeight`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`LineHeight`] of the [`Span`].
    ///
    /// </details>
    pub line_height: Option<LineHeight>,
    /// [`Span`] 的 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Font`] of the [`Span`].
    ///
    /// </details>
    pub font: Option<Font>,
    /// [`Span`] 的 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Color`] of the [`Span`].
    ///
    /// </details>
    pub color: Option<Color>,
    /// [`Span`] 的链接。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The link of the [`Span`].
    ///
    /// </details>
    pub link: Option<Link>,
    /// [`Span`] 的 [`Highlight`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Highlight`] of the [`Span`].
    ///
    /// </details>
    pub highlight: Option<Highlight>,
    /// [`Span`] 的 [`Padding`]。
    ///
    /// 目前，它只影响 [`Highlight`] 的边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Padding`] of the [`Span`].
    ///
    /// Currently, it only affects the bounds of the [`Highlight`].
    ///
    /// </details>
    pub padding: Padding,
    /// [`Span`] 是否应该带下划线。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the [`Span`] should be underlined or not.
    ///
    /// </details>
    pub underline: bool,
    /// [`Span`] 是否应该带删除线。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the [`Span`] should be struck through or not.
    ///
    /// </details>
    pub strikethrough: bool,
}

/// 文本高亮。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A text highlight.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Highlight {
    /// 高亮的 [`Background`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Background`] of the highlight.
    ///
    /// </details>
    pub background: Background,
    /// 高亮的 [`Border`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Border`] of the highlight.
    ///
    /// </details>
    pub border: Border,
}

impl<'a, Link> Span<'a, Link> {
    /// 使用给定的文本片段创建一个新的文本 [`Span`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Span`] of text with the given text fragment.
    ///
    /// </details>
    pub fn new(fragment: impl IntoFragment<'a>) -> Self {
        Self {
            text: fragment.into_fragment(),
            ..Self::default()
        }
    }

    /// 设置 [`Span`] 的大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the size of the [`Span`].
    ///
    /// </details>
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// 设置 [`Span`] 的 [`LineHeight`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`LineHeight`] of the [`Span`].
    ///
    /// </details>
    pub fn line_height(mut self, line_height: impl Into<LineHeight>) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    /// 设置 [`Span`] 的字体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the font of the [`Span`].
    ///
    /// </details>
    pub fn font(mut self, font: impl Into<Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// 设置 [`Span`] 的字体（如果有）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the font of the [`Span`], if any.
    ///
    /// </details>
    pub fn font_maybe(mut self, font: Option<impl Into<Font>>) -> Self {
        self.font = font.map(Into::into);
        self
    }

    /// 设置 [`Span`] 的 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Color`] of the [`Span`].
    ///
    /// </details>
    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// 设置 [`Span`] 的 [`Color`]（如果有）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Color`] of the [`Span`], if any.
    ///
    /// </details>
    pub fn color_maybe(mut self, color: Option<impl Into<Color>>) -> Self {
        self.color = color.map(Into::into);
        self
    }

    /// 设置 [`Span`] 的链接。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the link of the [`Span`].
    ///
    /// </details>
    pub fn link(mut self, link: impl Into<Link>) -> Self {
        self.link = Some(link.into());
        self
    }

    /// 设置 [`Span`] 的链接（如果有）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the link of the [`Span`], if any.
    ///
    /// </details>
    pub fn link_maybe(mut self, link: Option<impl Into<Link>>) -> Self {
        self.link = link.map(Into::into);
        self
    }

    /// 设置 [`Span`] 的 [`Background`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Background`] of the [`Span`].
    ///
    /// </details>
    pub fn background(self, background: impl Into<Background>) -> Self {
        self.background_maybe(Some(background))
    }

    /// 设置 [`Span`] 的 [`Background`]（如果有）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Background`] of the [`Span`], if any.
    ///
    /// </details>
    pub fn background_maybe(mut self, background: Option<impl Into<Background>>) -> Self {
        let Some(background) = background else {
            return self;
        };

        match &mut self.highlight {
            Some(highlight) => {
                highlight.background = background.into();
            }
            None => {
                self.highlight = Some(Highlight {
                    background: background.into(),
                    border: Border::default(),
                });
            }
        }

        self
    }

    /// 设置 [`Span`] 的 [`Border`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Border`] of the [`Span`].
    ///
    /// </details>
    pub fn border(self, border: impl Into<Border>) -> Self {
        self.border_maybe(Some(border))
    }

    /// 设置 [`Span`] 的 [`Border`]（如果有）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Border`] of the [`Span`], if any.
    ///
    /// </details>
    pub fn border_maybe(mut self, border: Option<impl Into<Border>>) -> Self {
        let Some(border) = border else {
            return self;
        };

        match &mut self.highlight {
            Some(highlight) => {
                highlight.border = border.into();
            }
            None => {
                self.highlight = Some(Highlight {
                    border: border.into(),
                    background: Background::Color(Color::TRANSPARENT),
                });
            }
        }

        self
    }

    /// 设置 [`Span`] 的 [`Padding`]。
    ///
    /// 目前，它只影响 [`Span`] 的 [`background`] 和
    /// [`border`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Padding`] of the [`Span`].
    ///
    /// It only affects the [`background`] and [`border`] of the
    /// [`Span`], currently.
    ///
    /// </details>
    ///
    /// [`background`]: Self::background
    /// [`border`]: Self::border
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// 设置 [`Span`] 是否应该带下划线。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets whether the [`Span`] should be underlined or not.
    ///
    /// </details>
    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }

    /// 设置 [`Span`] 是否应该带删除线。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets whether the [`Span`] should be struck through or not.
    ///
    /// </details>
    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = strikethrough;
        self
    }

    /// 把 [`Span`] 转换为静态的版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Turns the [`Span`] into a static one.
    ///
    /// </details>
    pub fn to_static(self) -> Span<'static, Link> {
        Span {
            text: Cow::Owned(self.text.into_owned()),
            size: self.size,
            line_height: self.line_height,
            font: self.font,
            color: self.color,
            link: self.link,
            highlight: self.highlight,
            padding: self.padding,
            underline: self.underline,
            strikethrough: self.strikethrough,
        }
    }
}

impl<Link> Default for Span<'_, Link> {
    fn default() -> Self {
        Self {
            text: Cow::default(),
            size: None,
            line_height: None,
            font: None,
            color: None,
            link: None,
            highlight: None,
            padding: Padding::default(),
            underline: false,
            strikethrough: false,
        }
    }
}

impl<'a, Link> From<&'a str> for Span<'a, Link> {
    fn from(value: &'a str) -> Self {
        Span::new(value)
    }
}

impl<Link> PartialEq for Span<'_, Link> {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
            && self.size == other.size
            && self.line_height == other.line_height
            && self.font == other.font
            && self.color == other.color
    }
}

/// 某个 [`Text`] 中的特定位置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A specific position in some [`Text`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    /// [`Text`] 的行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The line of the [`Text`].
    ///
    /// </details>
    pub line: usize,

    /// 该行中某个字符边界的首字节索引。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The first byte index of a character boundary in the line.
    ///
    /// </details>
    pub index: usize,
}

/// 文本选区的粒度。
///
/// 目标决定了选区如何围绕它的
/// 起始位置扩展：
///
/// - [`Target::Character`] 选中给定位置之间的
///   各个字符；
/// - [`Target::Word`] 把选区扩展到最近的完整单词；
/// - [`Target::Line`] 把选区扩展到最近的完整行。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The granularity of a text selection.
///
/// The target determines how a selection is expanded around its
/// starting position:
///
/// - [`Target::Character`] selects the individual characters between
///   the given positions;
/// - [`Target::Word`] expands the selection to the nearest whole words;
/// - [`Target::Line`] expands the selection to the nearest whole lines.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// 选中单个字符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Selects individual characters.
    ///
    /// </details>
    Character,
    /// 选中整个单词。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Selects whole words.
    ///
    /// </details>
    Word,
    /// 选中整行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Selects whole lines.
    ///
    /// </details>
    Line,
}

/// [`Text`] 的片段。
///
/// 它只是字符串的别名，该字符串可以是
/// 借用的，也可以是拥有所有权的。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A fragment of [`Text`].
///
/// This is just an alias to a string that may be either
/// borrowed or owned.
///
/// </details>
pub type Fragment<'a> = Cow<'a, str>;

/// 用于把值转换为文本 [`Fragment`] 的特质。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A trait for converting a value to some text [`Fragment`].
///
/// </details>
pub trait IntoFragment<'a> {
    /// 把值转换为文本 [`Fragment`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Converts the value to some text [`Fragment`].
    ///
    /// </details>
    fn into_fragment(self) -> Fragment<'a>;
}

impl<'a> IntoFragment<'a> for Fragment<'a> {
    fn into_fragment(self) -> Fragment<'a> {
        self
    }
}

impl<'a> IntoFragment<'a> for &'a Fragment<'_> {
    fn into_fragment(self) -> Fragment<'a> {
        Fragment::Borrowed(self)
    }
}

impl<'a> IntoFragment<'a> for &'a str {
    fn into_fragment(self) -> Fragment<'a> {
        Fragment::Borrowed(self)
    }
}

impl<'a> IntoFragment<'a> for &'a String {
    fn into_fragment(self) -> Fragment<'a> {
        Fragment::Borrowed(self.as_str())
    }
}

impl<'a> IntoFragment<'a> for String {
    fn into_fragment(self) -> Fragment<'a> {
        Fragment::Owned(self)
    }
}

macro_rules! into_fragment {
    ($type:ty) => {
        impl<'a> IntoFragment<'a> for $type {
            fn into_fragment(self) -> Fragment<'a> {
                Fragment::Owned(self.to_string())
            }
        }

        impl<'a> IntoFragment<'a> for &$type {
            fn into_fragment(self) -> Fragment<'a> {
                Fragment::Owned(self.to_string())
            }
        }
    };
}

into_fragment!(char);
into_fragment!(bool);

into_fragment!(u8);
into_fragment!(u16);
into_fragment!(u32);
into_fragment!(u64);
into_fragment!(u128);
into_fragment!(usize);

into_fragment!(i8);
into_fragment!(i16);
into_fragment!(i32);
into_fragment!(i64);
into_fragment!(i128);
into_fragment!(isize);

into_fragment!(f32);
into_fragment!(f64);
