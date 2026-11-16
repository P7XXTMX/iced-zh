//! 编辑文本。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Edit text.
//!
//! </details>
use crate::clipboard;
use crate::input_method;
use crate::keyboard;
use crate::keyboard::key;
use crate::mouse;
use crate::renderer;
use crate::text::highlighter;
use crate::text::{self, Alignment, LineHeight, Position, Wrapping};
use crate::time::{Duration, Instant};
use crate::touch;
use crate::widget::operation::{Focusable, TextInput};
use crate::window;
use crate::{
    Color, Event, Font, InputMethod, Padding, Pixels, Point, Rectangle, Size, SmolStr, Vector,
};

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

    /// 返回 [`Editor`] 当前的 [`Selection`]（如果可见）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current [`Selection`] of the [`Editor`], if visible.
    ///
    /// </details>
    fn selection(&self) -> Option<Selection>;

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

    /// 返回 [`Editor`] 的微调（hint）因子。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the hint factor of the [`Editor`].
    ///
    /// </details>
    fn hint_factor(&self) -> Option<f32>;

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
        new_font: Font,
        new_size: Pixels,
        new_line_height: LineHeight,
        new_wrapping: Wrapping,
        new_alignment: Alignment,
        new_hint_factor: Option<f32>,
        new_parser: &mut impl text::Parser,
    );

    /// 覆盖 [`Editor`] 当前的内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Overwrites the current contents of the [`Editor`].
    ///
    /// </details>
    fn overwrite(&mut self, new_text: &str);

    /// 在 [`Editor`] 中运行一个 [`text::Highlighter`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Runs a [`text::Highlighter`] in the [`Editor`].
    ///
    /// </details>
    fn highlight<P: text::Parser>(
        &mut self,
        font: Font,
        parser: &mut P,
        highlight: impl Fn(P::Output) -> highlighter::Style,
    );

    /// 返回一个迭代器，遍历 [`Editor`] 中各行的文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns an iterator of the text of the lines in the [`Editor`].
    ///
    /// </details>
    fn lines(&self) -> impl Iterator<Item = Line<'_>> {
        (0..)
            .map(|i| self.line(i))
            .take_while(Option::is_some)
            .flatten()
    }

    /// 返回 [`Editor`] 的文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the text of the [`Editor`].
    ///
    /// </details>
    fn text(&self) -> String {
        let mut contents = String::new();
        let mut lines = self.lines().peekable();

        while let Some(line) = lines.next() {
            contents.push_str(&line.text);

            if lines.peek().is_some() {
                contents.push_str(if line.ending == LineEnding::None {
                    LineEnding::default().as_str()
                } else {
                    line.ending.as_str()
                });
            }
        }

        contents
    }

    /// 返回 [`Editor`] 当前的 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current [`Font`] of the [`Editor`].
    ///
    /// </details>
    fn font(&self) -> Font;

    /// 返回 [`Editor`] 当前的文本大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current text size of the [`Editor`].
    ///
    /// </details>
    fn text_size(&self) -> Pixels;

    /// 返回 [`Editor`] 当前的 [`LineHeight`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current [`LineHeight`] of the [`Editor`].
    ///
    /// </details>
    fn line_height(&self) -> LineHeight;
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
    Click(Point, mouse::click::Kind),
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
    /// 删除光标之前的单词。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Delete the word before the cursor.
    ///
    /// </details>
    BackspaceWord,
    /// 删除光标之前的行内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Delete the line before the cursor.
    ///
    /// </details>
    BackspaceLine,
    /// 删除后一个字符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Delete the next character.
    ///
    /// </details>
    Delete,
    /// 删除光标之后的单词。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Delete the word after the cursor.
    ///
    /// </details>
    DeleteWord,
    /// 删除光标之后的行内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Delete the line after the cursor.
    ///
    /// </details>
    DeleteLine,
    /// 撤销在 [`Editor`] 上执行的上一次更改。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Undo the last change performed on the [`Editor`].
    ///
    /// </details>
    Undo,
    /// 重做 [`Editor`] 上最近一次被撤销的更改。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Redo the last undone change on the [`Editor`].
    ///
    /// </details>
    Redo,
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
    Range(Arc<[Rectangle]>),
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

/// [`Editor`] 的内部状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The internal state of an [`Editor`].
///
/// </details>
#[derive(Debug, Clone, Default)]
pub struct State {
    focus: Option<Focus>,
    preedit: Option<input_method::Preedit>,
    last_click: Option<mouse::Click>,
    is_dragging: bool,
    partial_scroll: f32,
}

impl State {
    /// 创建一个新的 [`State`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`State`].
    ///
    /// </details>
    pub fn new() -> Self {
        Self::default()
    }

    /// 针对给定的 [`Editor`] 更新 [`State`]，并返回相关的 [`Update`]（如果有）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Updates the [`State`] for the given [`Editor`] and returns any relevant [`Update`].
    ///
    /// </details>
    pub fn update<Message>(
        &mut self,
        editor: &impl Editor,
        event: &Event,
        bounds: Rectangle,
        padding: Padding,
        cursor: mouse::Cursor,
        key_binding: impl Fn(KeyPress) -> Option<Binding<Message>>,
    ) -> Option<Update<Message>> {
        match event {
            Event::Window(window::Event::Unfocused) => {
                if let Some(focus) = &mut self.focus {
                    focus.is_window_focused = false;
                }

                None
            }
            Event::Window(window::Event::Focused) => {
                if let Some(focus) = &mut self.focus {
                    focus.is_window_focused = true;
                    focus.updated_at = Instant::now();
                }

                Some(Update::Focus)
            }
            Event::Window(window::Event::RedrawRequested(now)) => {
                let focus = self.focus.as_mut()?;

                if !focus.is_window_focused {
                    return None;
                }

                focus.now = *now;

                let millis_until_redraw = Focus::CURSOR_BLINK_INTERVAL_MILLIS
                    - (focus.now - focus.updated_at).as_millis()
                        % Focus::CURSOR_BLINK_INTERVAL_MILLIS;

                Some(Update::RedrawAt(
                    focus.now + Duration::from_millis(millis_until_redraw as u64),
                ))
            }
            Event::Clipboard(clipboard::Event::Read(Ok(content))) => {
                let focus = self.focus.as_ref()?;

                if !focus.is_window_focused {
                    return None;
                }

                let clipboard::Content::Text(text) = content.as_ref() else {
                    return None;
                };

                Some(Update::Action(Action::Edit(Edit::Paste(Arc::new(
                    text.clone(),
                )))))
            }
            Event::Mouse(event) => match event {
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    if let Some(cursor_position) = cursor.position_in(bounds) {
                        let cursor_position =
                            cursor_position - Vector::new(padding.left, padding.top);

                        let click = mouse::Click::new(
                            cursor_position,
                            mouse::Button::Left,
                            self.last_click,
                        );

                        self.focus = Some(Focus::now());
                        self.last_click = Some(click);
                        self.is_dragging = true;

                        Some(Update::Action(Action::Click(
                            click.position(),
                            click.kind(),
                        )))
                    } else if self.focus.is_some() {
                        self.focus = None;

                        Some(Update::Unfocus)
                    } else {
                        None
                    }
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    self.is_dragging = false;

                    Some(Update::Release)
                }
                mouse::Event::CursorMoved { .. } if self.is_dragging => {
                    let position = cursor.position_from(bounds.position())?
                        - Vector::new(padding.left, padding.top);

                    Some(Update::Action(Action::Drag(position)))
                }
                mouse::Event::WheelScrolled { delta } if cursor.is_over(bounds) => {
                    let bounds = editor.bounds();

                    if bounds.height >= i32::MAX as f32 {
                        return None;
                    }

                    let lines = match delta {
                        mouse::ScrollDelta::Lines { y, .. } => {
                            if y.abs() > 0.0 {
                                y.signum() * -(y.abs() * 4.0).max(1.0)
                            } else {
                                0.0
                            }
                        }
                        mouse::ScrollDelta::Pixels { y, .. } => -y / 4.0,
                    };

                    let lines = lines + self.partial_scroll;
                    self.partial_scroll = lines.fract();

                    Some(Update::Action(Action::Scroll {
                        lines: lines as i32,
                    }))
                }
                _ => None,
            },
            Event::Touch(event) => match event {
                touch::Event::FingerPressed { .. } => {
                    if let Some(cursor_position) = cursor.position_in(bounds) {
                        let cursor_position =
                            cursor_position - Vector::new(padding.left, padding.top);

                        let click = mouse::Click::new(
                            cursor_position,
                            mouse::Button::Left,
                            self.last_click,
                        );

                        self.focus = Some(Focus::now());
                        self.last_click = Some(click);
                        self.is_dragging = true;

                        Some(Update::Action(Action::Click(
                            click.position(),
                            click.kind(),
                        )))
                    } else if self.focus.is_some() {
                        self.focus = None;

                        Some(Update::Unfocus)
                    } else {
                        None
                    }
                }
                touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. } => {
                    self.is_dragging = false;

                    Some(Update::Release)
                }
                touch::Event::FingerMoved { .. } if self.is_dragging => {
                    let position =
                        cursor.position_in(bounds)? - Vector::new(padding.left, padding.top);

                    Some(Update::Action(Action::Drag(position)))
                }
                touch::Event::FingerMoved { .. } => None,
            },
            Event::InputMethod(event) => match event {
                input_method::Event::Opened | input_method::Event::Closed => {
                    let is_open = matches!(event, input_method::Event::Opened);
                    self.preedit = is_open.then(input_method::Preedit::new);

                    Some(Update::InputMethod)
                }
                input_method::Event::Preedit(content, selection) if self.focus.is_some() => {
                    self.preedit = Some(input_method::Preedit {
                        content: content.clone(),
                        selection: selection.clone(),
                        text_size: Some(editor.text_size()),
                    });

                    Some(Update::InputMethod)
                }
                input_method::Event::Commit(content) if self.focus.is_some() => Some(
                    Update::Action(Action::Edit(Edit::Paste(Arc::new(content.clone())))),
                ),
                _ => None,
            },
            Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                modified_key,
                physical_key,
                modifiers,
                text,
                ..
            }) => {
                let key_press = KeyPress {
                    key: key.clone(),
                    modified_key: modified_key.clone(),
                    physical_key: *physical_key,
                    modifiers: *modifiers,
                    text: text.clone(),
                    is_focused: self.is_focused(),
                };

                fn apply_binding<Message>(
                    binding: Binding<Message>,
                    editor: &impl Editor,
                    state: &mut State,
                ) -> Option<Update<Message>> {
                    let action = |action| Update::Action(action);
                    let edit = |edit| action(Action::Edit(edit));

                    match binding {
                        Binding::Unfocus => {
                            state.focus = None;
                            state.is_dragging = false;

                            None
                        }
                        Binding::Copy => {
                            let selection = editor.copy()?;

                            Some(Update::Copy(selection))
                        }
                        Binding::Cut => {
                            let selection = editor.copy()?;

                            Some(Update::Sequence(vec![
                                Update::Copy(selection),
                                edit(Edit::Backspace),
                            ]))
                        }
                        Binding::Paste => Some(Update::Paste),
                        Binding::Undo => Some(edit(Edit::Undo)),
                        Binding::Redo => Some(edit(Edit::Redo)),
                        Binding::Move(motion) => Some(action(Action::Move(motion))),
                        Binding::Select(motion) => Some(action(Action::Select(motion))),
                        Binding::SelectWord => Some(action(Action::SelectWord)),
                        Binding::SelectLine => Some(action(Action::SelectLine)),
                        Binding::SelectAll => Some(action(Action::SelectAll)),
                        Binding::Insert(c) => Some(edit(Edit::Insert(c))),
                        Binding::Enter => Some(edit(Edit::Enter)),
                        Binding::Backspace => Some(edit(Edit::Backspace)),
                        Binding::BackspaceWord => Some(edit(Edit::BackspaceWord)),
                        Binding::BackspaceLine => Some(edit(Edit::BackspaceLine)),
                        Binding::Delete => Some(action(Action::Edit(Edit::Delete))),
                        Binding::DeleteWord => Some(edit(Edit::DeleteWord)),
                        Binding::DeleteLine => Some(edit(Edit::DeleteLine)),
                        Binding::Sequence(sequence) => {
                            let updates: Vec<_> = sequence
                                .into_iter()
                                .flat_map(|binding| apply_binding(binding, editor, state))
                                .collect();

                            if updates.is_empty() {
                                return None;
                            }

                            Some(Update::Sequence(updates))
                        }
                        Binding::Custom(message) => Some(Update::Custom(message)),
                    }
                }

                let update = apply_binding(key_binding(key_press)?, editor, self);

                if let Some(focus) = &mut self.focus {
                    focus.updated_at = Instant::now();
                }

                update
            }
            _ => None,
        }
    }

    /// 返回 [`State`] 针对给定 [`Editor`] 的当前 [`InputMethod`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current [`InputMethod`] of the [`State`] for the given [`Editor`].
    ///
    /// </details>
    pub fn input_method<'a>(
        &'a self,
        editor: &impl Editor,
        position: Point,
    ) -> InputMethod<&'a str> {
        let Some(Focus {
            is_window_focused: true,
            ..
        }) = &self.focus
        else {
            return InputMethod::Disabled;
        };

        let translation = position - Point::ORIGIN;

        let Some(selection) = editor.selection() else {
            return InputMethod::Disabled;
        };

        let cursor = match selection {
            Selection::Caret(position) => position,
            Selection::Range(ranges) => ranges.first().cloned().unwrap_or_default().position(),
        };

        let line_height = editor.line_height().to_absolute(editor.text_size());

        let position = cursor + translation;

        InputMethod::Enabled {
            cursor: Rectangle::new(position, Size::new(1.0, f32::from(line_height))),
            purpose: input_method::Purpose::Normal,
            preedit: self.preedit.as_ref().map(input_method::Preedit::as_ref),
        }
    }

    /// 使用当前的 [`State`] 绘制给定的 [`Editor`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the given [`Editor`] with the current [`State`].
    ///
    /// </details>
    pub fn draw<Renderer: text::Renderer>(
        &self,
        editor: &Renderer::Editor,
        renderer: &mut Renderer,
        position: Point,
        clip_bounds: Rectangle,
        style: Style,
    ) {
        let bounds = Rectangle::new(position, editor.bounds());

        let Some(clip_bounds) = clip_bounds.intersection(&bounds) else {
            return;
        };

        if !editor.is_empty() {
            renderer.fill_editor(editor, position, style.value, clip_bounds);
        }

        if !self.is_focused() {
            return;
        }

        let translation = position - Point::ORIGIN;
        let text_size = editor.text_size();
        let line_height = editor.line_height();

        let Some(selection) = editor.selection() else {
            return;
        };

        match selection {
            Selection::Caret(position) if self.is_cursor_visible() => {
                let cursor = Rectangle::new(
                    position + translation,
                    Size::new(
                        if renderer::CRISP {
                            (1.0 / renderer.hint_factor().unwrap_or(1.0)).max(1.0)
                        } else {
                            1.0
                        },
                        line_height.to_absolute(text_size).into(),
                    ),
                );

                if let Some(clipped_cursor) = clip_bounds.intersection(&cursor) {
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: clipped_cursor,
                            ..renderer::Quad::default()
                        },
                        style.value,
                    );
                }
            }
            Selection::Range(ranges) => {
                for range in ranges
                    .iter()
                    .filter_map(|range| clip_bounds.intersection(&(*range + translation)))
                {
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: range.round(),
                            ..renderer::Quad::default()
                        },
                        style.selection,
                    );
                }
            }
            Selection::Caret(_) => {
                // Drawing an empty quad helps some renderers to track the damage of the blinking cursor
                // 绘制一个空的四边形，有助于某些渲染器跟踪闪烁光标的损坏区域
                renderer.fill_quad(renderer::Quad::default(), Color::TRANSPARENT);
            }
        }
    }

    /// 返回 [`Editor`] 的光标是否可见。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns whether the cursor of the [`Editor`] is visible.
    ///
    /// </details>
    pub fn is_cursor_visible(&self) -> bool {
        self.focus.as_ref().is_some_and(Focus::is_cursor_visible)
    }
}

/// [`Editor`] 的视觉样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The visual style of an [`Editor`].
///
/// </details>
pub struct Style {
    /// 内容的 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Color`] of the contents.
    ///
    /// </details>
    pub value: Color,

    /// 选区的背景 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The background [`Color`] of any selection.
    ///
    /// </details>
    pub selection: Color,
}

#[derive(Debug, Clone)]
struct Focus {
    updated_at: Instant,
    now: Instant,
    is_window_focused: bool,
}

impl Focus {
    const CURSOR_BLINK_INTERVAL_MILLIS: u128 = 500;

    fn now() -> Self {
        let now = Instant::now();

        Self {
            updated_at: now,
            now,
            is_window_focused: true,
        }
    }

    fn is_cursor_visible(&self) -> bool {
        self.is_window_focused
            && ((self.now - self.updated_at).as_millis() / Self::CURSOR_BLINK_INTERVAL_MILLIS)
                .is_multiple_of(2)
    }
}

impl State {
    /// 返回 [`Editor`] 当前是否获得焦点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns whether the [`Editor`] is currently focused or not.
    ///
    /// </details>
    pub fn is_focused(&self) -> bool {
        self.focus.is_some()
    }
}

impl Focusable for State {
    fn is_focused(&self) -> bool {
        self.focus.is_some()
    }

    fn focus(&mut self) {
        self.focus = Some(Focus::now());
    }

    fn unfocus(&mut self) {
        self.focus = None;
    }
}

/// 到 [`Editor`] 中某个动作的绑定。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A binding to an action in the [`Editor`].
///
/// </details>
#[derive(Debug, Clone, PartialEq)]
pub enum Binding<Message> {
    /// 让 [`Editor`] 失去焦点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Unfocus the [`Editor`].
    ///
    /// </details>
    Unfocus,
    /// 复制 [`Editor`] 的选区。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Copy the selection of the [`Editor`].
    ///
    /// </details>
    Copy,
    /// 剪切 [`Editor`] 的选区。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Cut the selection of the [`Editor`].
    ///
    /// </details>
    Cut,
    /// 把剪贴板的内容粘贴到 [`Editor`] 中。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Paste the clipboard contents in the [`Editor`].
    ///
    /// </details>
    Paste,
    /// 撤销在 [`Editor`] 中执行的上一次更改。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Undo the last change peformed in the [`Editor`].
    ///
    /// </details>
    Undo,
    /// 重做 [`Editor`] 中最近一次被撤销的更改。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Redo the last change undone in the [`Editor`].
    ///
    /// </details>
    Redo,
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
    /// 插入给定的字符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Insert the given character.
    ///
    /// </details>
    Insert(char),
    /// 在当前行换行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Break the current line.
    ///
    /// </details>
    Enter,
    /// 删除前一个字符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Delete the previous character.
    ///
    /// </details>
    Backspace,
    /// 删除光标之前的单词。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Delete the word before the cursor.
    ///
    /// </details>
    BackspaceWord,
    /// 删除光标之前的行内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Delete the line before the cursor.
    ///
    /// </details>
    BackspaceLine,
    /// 删除后一个字符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Delete the next character.
    ///
    /// </details>
    Delete,
    /// 删除光标之后的单词。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Delete the word after the cursor.
    ///
    /// </details>
    DeleteWord,
    /// 删除光标之后的行内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Delete the line after the cursor.
    ///
    /// </details>
    DeleteLine,
    /// 要依次执行的一系列绑定。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A sequence of bindings to execute.
    ///
    /// </details>
    Sequence(Vec<Self>),
    /// 产生给定的消息。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Produce the given message.
    ///
    /// </details>
    Custom(Message),
}

/// 一次按键。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A key press.
///
/// </details>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyPress {
    /// 按下的原始按键，未应用修饰键。
    ///
    /// 组合键（例如 Ctrl+C）应当使用这个按键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The original key pressed without modifiers applied to it.
    ///
    /// You should use this key for combinations (e.g. Ctrl+C).
    ///
    /// </details>
    pub key: keyboard::Key,
    /// 按下的按键，已应用修饰键。
    ///
    /// 任何单键绑定（例如光标移动）都应当使用这个按键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The key pressed with modifiers applied to it.
    ///
    /// You should use this key for any single key bindings (e.g. motions).
    ///
    /// </details>
    pub modified_key: keyboard::Key,
    /// 按下的物理按键。
    ///
    /// 与键盘布局无关的绑定应当使用这个按键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The physical key pressed.
    ///
    /// You should use this key for layout-independent bindings.
    ///
    /// </details>
    pub physical_key: keyboard::key::Physical,
    /// 键盘修饰键的状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The state of the keyboard modifiers.
    ///
    /// </details>
    pub modifiers: keyboard::Modifiers,
    /// 这次按键产生的文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text produced by the key press.
    ///
    /// </details>
    pub text: Option<SmolStr>,
    /// [`Editor`] 是否获得焦点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the [`Editor`] is focused.
    ///
    /// </details>
    pub is_focused: bool,
}

impl<Message> Binding<Message> {
    /// 返回给定按键对应的默认 [`Binding`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the default [`Binding`] for the given key press.
    ///
    /// </details>
    pub fn from_key_press(event: KeyPress) -> Option<Self> {
        let KeyPress {
            key,
            modified_key,
            physical_key,
            modifiers,
            text,
            is_focused,
        } = event;

        if !is_focused {
            return None;
        }

        let combination = match key.to_latin(physical_key) {
            Some('c') if modifiers.command() => Some(Self::Copy),
            Some('x') if modifiers.command() => Some(Self::Cut),
            Some('v') if modifiers.command() && !modifiers.alt() => Some(Self::Paste),
            Some('a') if modifiers.command() => Some(Self::SelectAll),
            Some('z') if modifiers.command() => Some(Self::Undo),
            Some('y') if modifiers.command() => Some(Self::Redo),
            _ => None,
        };

        if let Some(binding) = combination {
            return Some(binding);
        }

        #[cfg(target_os = "macos")]
        let modified_key = convert_macos_shortcut(&key, modifiers).unwrap_or(modified_key);

        match modified_key.as_ref() {
            keyboard::Key::Named(key::Named::Enter) => Some(Self::Enter),
            keyboard::Key::Named(key::Named::Backspace) => Some(
                if modifiers.macos_command() || (modifiers.command() && modifiers.shift()) {
                    Self::BackspaceLine
                } else if modifiers.jump() {
                    Self::BackspaceWord
                } else {
                    Self::Backspace
                },
            ),
            keyboard::Key::Named(key::Named::Delete)
                if text.is_none() || text.as_deref() == Some("\u{7f}") =>
            {
                Some(
                    if modifiers.macos_command() || (modifiers.command() && modifiers.shift()) {
                        Self::DeleteLine
                    } else if modifiers.jump() {
                        Self::DeleteWord
                    } else {
                        Self::Delete
                    },
                )
            }
            keyboard::Key::Named(key::Named::Escape) => Some(Self::Unfocus),
            _ => {
                if let Some(text) = text {
                    let c = text.chars().find(|c| !c.is_control())?;

                    Some(Self::Insert(c))
                } else if let keyboard::Key::Named(named_key) = key.as_ref() {
                    let motion = motion(named_key)?;

                    let motion = if modifiers.macos_command() {
                        match motion {
                            Motion::Left => Motion::Home,
                            Motion::Right => Motion::End,
                            _ => motion,
                        }
                    } else {
                        motion
                    };

                    let motion = if modifiers.jump() {
                        motion.widen()
                    } else {
                        motion
                    };

                    Some(if modifiers.shift() {
                        Self::Select(motion)
                    } else {
                        Self::Move(motion)
                    })
                } else {
                    None
                }
            }
        }
    }
}

/// [`Editor`] 的更新，由 [`State::update`] 返回。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The update of an [`Editor`], returned by [`State::update`].
///
/// </details>
pub enum Update<Message> {
    /// 必须在 [`Editor`] 中执行一个 [`Action`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An [`Action`] must be performed in the [`Editor`].
    ///
    /// </details>
    Action(Action),
    /// [`Editor`] 刚刚获得焦点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Editor`] just gained focus.
    ///
    /// </details>
    Focus,
    /// [`Editor`] 刚刚失去焦点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Editor`] just lost focus.
    ///
    /// </details>
    Unfocus,
    /// [`Editor`] 改变了它的 [`InputMethod`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Editor`] changed its [`InputMethod`].
    ///
    /// </details>
    InputMethod,
    /// 在 [`Editor`] 中松开了一次鼠标按压。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A mouse press was released in the [`Editor`].
    ///
    /// </details>
    Release,
    /// [`Editor`] 必须把一些文本复制到剪贴板。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Editor`] must copy some text to the clipboard.
    ///
    /// </details>
    Copy(String),
    /// [`Editor`] 必须粘贴剪贴板的内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Editor`] must paste the clipboard contents.
    ///
    /// </details>
    Paste,
    /// [`Editor`] 必须在给定的 [`Instant`] 重绘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Editor`] must be redrawn at the given [`Instant`].
    ///
    /// </details>
    RedrawAt(Instant),
    /// [`Editor`] 产生了一条必须发布的自定义消息。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Editor`] produced a custom message that must be published.
    ///
    /// </details>
    Custom(Message),
    /// [`Editor`] 产生了一系列更新。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Editor`] produced a sequence of updates.
    ///
    /// </details>
    Sequence(Vec<Update<Message>>),
}

fn motion(key: key::Named) -> Option<Motion> {
    match key {
        key::Named::ArrowLeft => Some(Motion::Left),
        key::Named::ArrowRight => Some(Motion::Right),
        key::Named::ArrowUp => Some(Motion::Up),
        key::Named::ArrowDown => Some(Motion::Down),
        key::Named::Home => Some(Motion::Home),
        key::Named::End => Some(Motion::End),
        key::Named::PageUp => Some(Motion::PageUp),
        key::Named::PageDown => Some(Motion::PageDown),
        _ => None,
    }
}

#[cfg(target_os = "macos")]
fn convert_macos_shortcut(
    key: &keyboard::Key,
    modifiers: keyboard::Modifiers,
) -> Option<keyboard::Key> {
    if modifiers != keyboard::Modifiers::CTRL {
        return None;
    }

    let key = match key.as_ref() {
        keyboard::Key::Character("b") => key::Named::ArrowLeft,
        keyboard::Key::Character("f") => key::Named::ArrowRight,
        keyboard::Key::Character("a") => key::Named::Home,
        keyboard::Key::Character("e") => key::Named::End,
        keyboard::Key::Character("h") => key::Named::Backspace,
        keyboard::Key::Character("d") => key::Named::Delete,
        _ => return None,
    };

    Some(keyboard::Key::Named(key))
}

impl<T: Editor> TextInput for T {
    fn text(&self) -> text::Fragment<'_> {
        text::Fragment::Owned(Editor::text(self))
    }

    fn move_cursor_to_front(&mut self) {
        self.perform(Action::Move(Motion::DocumentStart));
    }

    fn move_cursor_to_end(&mut self) {
        self.perform(Action::Move(Motion::DocumentEnd));
    }

    fn move_cursor_to(&mut self, position: text::Position) {
        self.move_to(Cursor {
            position,
            selection: None,
        });
    }

    fn select_all(&mut self) {
        self.perform(Action::SelectAll);
    }

    fn select_range(&mut self, start: text::Position, end: text::Position) {
        self.move_to(Cursor {
            position: start,
            selection: Some(end),
        });
    }
}
