//! 编辑文本。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Edit text.
//!
//! </details>
use crate::text::highlighter::{self, Highlighter};
use crate::text::{LineHeight, Wrapping};
use crate::{Pixels, Point, Rectangle, Size};

use std::borrow::Cow;
use std::sync::Arc;

/// 可供部件用来编辑多行文本的组件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A component that can be used by widgets to edit multi-line text.
///
/// </details>
pub trait Editor: Sized + Default {
    /// [`Editor`] 的字体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The font of the [`Editor`].
    ///
    /// </details>
    type Font: Copy + PartialEq + Default;

    /// 使用给定的文本创建一个已完成布局的新 [`Editor`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Editor`] laid out with the given text.
    ///
    /// </details>
    fn with_text(text: &str) -> Self;

    /// 如果 [`Editor`] 没有内容，返回 true。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns true if the [`Editor`] has no contents.
    ///
    /// </details>
    fn is_empty(&self) -> bool;

    /// 返回 [`Editor`] 当前的 [`Cursor`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current [`Cursor`] of the [`Editor`].
    ///
    /// </details>
    fn cursor(&self) -> Cursor;

    /// 返回 [`Editor`] 当前的 [`Selection`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current [`Selection`] of the [`Editor`].
    ///
    /// </details>
    fn selection(&self) -> Selection;

    /// 返回 [`Editor`] 当前选中的文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current selected text of the [`Editor`].
    ///
    /// </details>
    fn copy(&self) -> Option<String>;

    /// 返回 [`Editor`] 中给定行的文本（如果该行存在）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the text of the given line in the [`Editor`], if it exists.
    ///
    /// </details>
    fn line(&self, index: usize) -> Option<Line<'_>>;

    /// 返回 [`Editor`] 中的行数。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the amount of lines in the [`Editor`].
    ///
    /// </details>
    fn line_count(&self) -> usize;

    /// 在 [`Editor`] 上执行一个 [`Action`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Performs an [`Action`] on the [`Editor`].
    ///
    /// </details>
    fn perform(&mut self, action: Action);

    /// 把光标移动到给定的位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Moves the cursor to the given position.
    ///
    /// </details>
    fn move_to(&mut self, cursor: Cursor);

    /// 返回 [`Editor`] 当前的边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current boundaries of the [`Editor`].
    ///
    /// </details>
    fn bounds(&self) -> Size;

    /// 返回能容纳 [`Editor`] 当前内容的
    /// 最小边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the minimum boundaries to fit the current contents of
    /// the [`Editor`].
    ///
    /// </details>
    fn min_bounds(&self) -> Size;

    /// 使用一些新的属性更新 [`Editor`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Updates the [`Editor`] with some new attributes.
    ///
    /// </details>
    fn update(
        &mut self,
        new_bounds: Size,
        new_font: Self::Font,
        new_size: Pixels,
        new_line_height: LineHeight,
        new_wrapping: Wrapping,
        new_highlighter: &mut impl Highlighter,
    );

    /// 在 [`Editor`] 中运行一个文本 [`Highlighter`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Runs a text [`Highlighter`] in the [`Editor`].
    ///
    /// </details>
    fn highlight<H: Highlighter>(
        &mut self,
        font: Self::Font,
        highlighter: &mut H,
        format_highlight: impl Fn(&H::Highlight) -> highlighter::Format<Self::Font>,
    );
}

/// 与 [`Editor`] 的一次交互。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An interaction with an [`Editor`].
///
/// </details>
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// 应用一个 [`Motion`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Apply a [`Motion`].
    ///
    /// </details>
    Move(Motion),
    /// 使用给定的 [`Motion`] 选择文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Select text with a given [`Motion`].
    ///
    /// </details>
    Select(Motion),
    /// 选择当前光标处的单词。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Select the word at the current cursor.
    ///
    /// </details>
    SelectWord,
    /// 选择当前光标处的行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Select the line at the current cursor.
    ///
    /// </details>
    SelectLine,
    /// 选择整个缓冲区。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Select the entire buffer.
    ///
    /// </details>
    SelectAll,
    /// 执行一个 [`Edit`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Perform an [`Edit`].
    ///
    /// </details>
    Edit(Edit),
    /// 在给定的 [`Point`] 处点击 [`Editor`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Click the [`Editor`] at the given [`Point`].
    ///
    /// </details>
    Click(Point),
    /// 在 [`Editor`] 上把鼠标拖动到给定的 [`Point`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Drag the mouse on the [`Editor`] to the given [`Point`].
    ///
    /// </details>
    Drag(Point),
    /// 把 [`Editor`] 滚动一定的行数。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Scroll the [`Editor`] a certain amount of lines.
    ///
    /// </details>
    Scroll {
        /// 要滚动的行数。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The amount of lines to scroll.
        ///
        /// </details>
        lines: i32,
    },
}

impl Action {
    /// 返回该 [`Action`] 是否为编辑动作。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns whether the [`Action`] is an editing action.
    ///
    /// </details>
    pub fn is_edit(&self) -> bool {
        matches!(self, Self::Edit(_))
    }
}

/// 编辑文本的动作。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An action that edits text.
///
/// </details>
#[derive(Debug, Clone, PartialEq)]
pub enum Edit {
    /// 插入给定的字符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Insert the given character.
    ///
    /// </details>
    Insert(char),
    /// 粘贴给定的文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Paste the given text.
    ///
    /// </details>
    Paste(Arc<String>),
    /// 在当前行换行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Break the current line.
    ///
    /// </details>
    Enter,
    /// 缩进当前行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Indent the current line.
    ///
    /// </details>
    Indent,
    /// 取消当前行的缩进。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Unindent the current line.
    ///
    /// </details>
    Unindent,
    /// 删除前一个字符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Delete the previous character.
    ///
    /// </details>
    Backspace,
    /// 删除后一个字符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Delete the next character.
    ///
    /// </details>
    Delete,
}

/// 光标的移动。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A cursor movement.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Motion {
    /// 向左移动。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Move left.
    ///
    /// </details>
    Left,
    /// 向右移动。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Move right.
    ///
    /// </details>
    Right,
    /// 向上移动。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Move up.
    ///
    /// </details>
    Up,
    /// 向下移动。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Move down.
    ///
    /// </details>
    Down,
    /// 移动到单词的左边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Move to the left boundary of a word.
    ///
    /// </details>
    WordLeft,
    /// 移动到单词的右边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Move to the right boundary of a word.
    ///
    /// </details>
    WordRight,
    /// 移动到行首。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Move to the start of the line.
    ///
    /// </details>
    Home,
    /// 移动到行尾。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Move to the end of the line.
    ///
    /// </details>
    End,
    /// 移动到上一屏的开头。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Move to the start of the previous window.
    ///
    /// </details>
    PageUp,
    /// 移动到下一屏的开头。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Move to the start of the next window.
    ///
    /// </details>
    PageDown,
    /// 移动到文本的开头。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Move to the start of the text.
    ///
    /// </details>
    DocumentStart,
    /// 移动到文本的末尾。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Move to the end of the text.
    ///
    /// </details>
    DocumentEnd,
}

impl Motion {
    /// 如果可能，扩大该 [`Motion`] 的范围。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Widens the [`Motion`], if possible.
    ///
    /// </details>
    pub fn widen(self) -> Self {
        match self {
            Self::Left => Self::WordLeft,
            Self::Right => Self::WordRight,
            Self::Home => Self::DocumentStart,
            Self::End => Self::DocumentEnd,
            _ => self,
        }
    }

    /// 返回 [`Motion`] 的 [`Direction`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Direction`] of the [`Motion`].
    ///
    /// </details>
    pub fn direction(&self) -> Direction {
        match self {
            Self::Left
            | Self::Up
            | Self::WordLeft
            | Self::Home
            | Self::PageUp
            | Self::DocumentStart => Direction::Left,
            Self::Right
            | Self::Down
            | Self::WordRight
            | Self::End
            | Self::PageDown
            | Self::DocumentEnd => Direction::Right,
        }
    }
}

/// 文本中的方向。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A direction in some text.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// <-
    Left,
    /// ->
    Right,
}

/// [`Editor`] 的光标。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The cursor of an [`Editor`].
///
/// </details>
#[derive(Debug, Clone)]
pub enum Selection {
    /// 没有选区的光标
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Cursor without a selection
    ///
    /// </details>
    Caret(Point),

    /// 选中一段文本范围的光标
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Cursor selecting a range of text
    ///
    /// </details>
    Range(Vec<Rectangle>),
}

/// [`Editor`] 的范围。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The range of an [`Editor`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cursor {
    /// 光标位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The cursor position.
    ///
    /// </details>
    pub position: Position,

    /// 选区位置（如果有）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The selection position, if any.
    ///
    /// </details>
    pub selection: Option<Position>,
}

/// [`Editor`] 中的一个光标位置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A cursor position in an [`Editor`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    /// 文本所在的行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The line of text.
    ///
    /// </details>
    pub line: usize,
    /// 行内的列。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The column in the line.
    ///
    /// </details>
    pub column: usize,
}

/// [`Editor`] 的一行。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A line of an [`Editor`].
///
/// </details>
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Line<'a> {
    /// [`Line`] 的原始文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The raw text of the [`Line`].
    ///
    /// </details>
    pub text: Cow<'a, str>,
    /// [`Line`] 的行尾符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The line ending of the [`Line`].
    ///
    /// </details>
    pub ending: LineEnding,
}

/// [`Line`] 的行尾符。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The line ending of a [`Line`].
///
/// </details>
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LineEnding {
    /// 使用 `\n` 作为行尾符（POSIX 风格）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Use `\n` for line ending (POSIX-style)
    ///
    /// </details>
    #[default]
    Lf,
    /// 使用 `\r\n` 作为行尾符（Windows 风格）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Use `\r\n` for line ending (Windows-style)
    ///
    /// </details>
    CrLf,
    /// 使用 `\r` 作为行尾符（许多旧式系统）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Use `\r` for line ending (many legacy systems)
    ///
    /// </details>
    Cr,
    /// 使用 `\n\r` 作为行尾符（部分旧式系统）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Use `\n\r` for line ending (some legacy systems)
    ///
    /// </details>
    LfCr,
    /// 没有行尾符
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// No line ending
    ///
    /// </details>
    None,
}

impl LineEnding {
    /// 获取 [`LineEnding`] 的字符串表示。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Gets the string representation of the [`LineEnding`].
    ///
    /// </details>
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lf => "\n",
            Self::CrLf => "\r\n",
            Self::Cr => "\r",
            Self::LfCr => "\n\r",
            Self::None => "",
        }
    }
}
