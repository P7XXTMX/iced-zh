//! 高亮文本。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Highlight text.
//!
//! </details>
use crate::Color;

use std::ops::Range;

/// 一种能够高亮文本的类型。
///
/// [`Highlighter`] 按顺序高亮各行。某一行发生变化时，
/// 必须通知它，并且必须把变化行之后的各行
/// 重新交给 [`Highlighter`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A type capable of highlighting text.
///
/// A [`Highlighter`] highlights lines in sequence. When a line changes,
/// it must be notified and the lines after the changed one must be fed
/// again to the [`Highlighter`].
///
/// </details>
pub trait Highlighter: 'static {
    /// 用来配置 [`Highlighter`] 的设置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The settings to configure the [`Highlighter`].
    ///
    /// </details>
    type Settings: PartialEq + Clone;

    /// [`Highlighter`] 的输出。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The output of the [`Highlighter`].
    ///
    /// </details>
    type Highlight;

    /// 高亮迭代器的类型。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The highlight iterator type.
    ///
    /// </details>
    type Iterator<'a>: Iterator<Item = (Range<usize>, Self::Highlight)>
    where
        Self: 'a;

    /// 根据其 [`Self::Settings`] 创建一个新的 [`Highlighter`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Highlighter`] from its [`Self::Settings`].
    ///
    /// </details>
    fn new(settings: &Self::Settings) -> Self;

    /// 用新的 [`Self::Settings`] 更新 [`Highlighter`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Updates the [`Highlighter`] with some new [`Self::Settings`].
    ///
    /// </details>
    fn update(&mut self, new_settings: &Self::Settings);

    /// 通知 [`Highlighter`]：给定索引处的行已经发生变化。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Notifies the [`Highlighter`] that the line at the given index has changed.
    ///
    /// </details>
    fn change_line(&mut self, line: usize);

    /// 高亮给定的行。
    ///
    /// 如果在此之前有某一行发生了变化，那么这里提供的第一行
    /// 就是发生变化的那一行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Highlights the given line.
    ///
    /// If a line changed prior to this, the first line provided here will be the
    /// line that changed.
    ///
    /// </details>
    fn highlight_line(&mut self, line: &str) -> Self::Iterator<'_>;

    /// 返回 [`Highlighter`] 的当前行。
    ///
    /// 如果调用过 `change_line`，它通常是发生变化的
    /// 最小索引。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current line of the [`Highlighter`].
    ///
    /// If `change_line` has been called, this will normally be the least index
    /// that changed.
    ///
    /// </details>
    fn current_line(&self) -> usize;
}

/// 一个什么都不高亮的高亮器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A highlighter that highlights nothing.
///
/// </details>
#[derive(Debug, Clone, Copy)]
pub struct PlainText;

impl Highlighter for PlainText {
    type Settings = ();
    type Highlight = ();

    type Iterator<'a> = std::iter::Empty<(Range<usize>, Self::Highlight)>;

    fn new(_settings: &Self::Settings) -> Self {
        Self
    }

    fn update(&mut self, _new_settings: &Self::Settings) {}

    fn change_line(&mut self, _line: usize) {}

    fn highlight_line(&mut self, _line: &str) -> Self::Iterator<'_> {
        std::iter::empty()
    }

    fn current_line(&self) -> usize {
        usize::MAX
    }
}

/// 某段文本的格式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The format of some text.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Format<Font> {
    /// 文本的 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Color`] of the text.
    ///
    /// </details>
    pub color: Option<Color>,
    /// 文本的 `Font`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The `Font` of the text.
    ///
    /// </details>
    pub font: Option<Font>,
}

impl<Font> Default for Format<Font> {
    fn default() -> Self {
        Self {
            color: None,
            font: None,
        }
    }
}
