//! 解析文本。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Parse text.
//!
//! </details>
use std::ops::Range;

/// 能够解析文本并产生某种 `Output` 的类型。
///
/// [`Parser`] 按顺序逐行解析。当某一行发生变化时，
/// 必须通知它，并且发生变化的那一行之后的各行
/// 必须重新提供给 [`Parser`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A type capable of parsing text, producing some `Output`.
///
/// A [`Parser`] parses lines in sequence. When a line changes,
/// it must be notified and the lines after the changed one must be fed
/// again to the [`Parser`].
///
/// </details>
pub trait Parser: 'static {
    /// 用于配置 [`Parser`] 的设置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The settings to configure the [`Parser`].
    ///
    /// </details>
    type Settings: PartialEq + Clone;

    /// 此 [`Parser`] 的输出。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The output of this [`Parser`].
    ///
    /// </details>
    type Output;

    /// 解析迭代器的类型。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The parse iterator type.
    ///
    /// </details>
    type Iterator<'a>: Iterator<Item = (Range<usize>, Self::Output)>
    where
        Self: 'a;

    /// 根据 [`Self::Settings`] 创建一个新的 [`Parser`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Parser`] from its [`Self::Settings`].
    ///
    /// </details>
    fn new(settings: &Self::Settings) -> Self;

    /// 使用新的 [`Self::Settings`] 更新 [`Parser`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Updates the [`Parser`] with some new [`Self::Settings`].
    ///
    /// </details>
    fn update(&mut self, new_settings: &Self::Settings);

    /// 通知 [`Parser`] 给定索引处的行已发生变化。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Notifies the [`Parser`] that the line at the given index has changed.
    ///
    /// </details>
    fn change_line(&mut self, line: usize);

    /// 解析给定的行。
    ///
    /// 如果在此之前有某一行发生了变化，那么这里提供的第一行
    /// 就是发生变化的那一行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Parses the given line.
    ///
    /// If a line changed prior to this, the first line provided here will be the
    /// line that changed.
    ///
    /// </details>
    fn parse_line(&mut self, line: &str) -> Self::Iterator<'_>;

    /// 返回 [`Parser`] 的当前行。
    ///
    /// 如果调用过 `change_line`，它通常是发生变化的
    /// 最小索引。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current line of the [`Parser`].
    ///
    /// If `change_line` has been called, this will normally be the least index
    /// that changed.
    ///
    /// </details>
    fn current_line(&self) -> usize;
}

/// 不产生任何输出的解析器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A parser that produces no output.
///
/// </details>
#[derive(Debug, Clone, Copy)]
pub struct PlainText;

impl Parser for PlainText {
    type Settings = ();
    type Output = ();

    type Iterator<'a> = std::iter::Empty<(Range<usize>, ())>;

    fn new(_settings: &Self::Settings) -> Self {
        Self
    }

    fn update(&mut self, _new_settings: &Self::Settings) {}

    fn change_line(&mut self, _line: usize) {}

    fn parse_line(&mut self, _line: &str) -> Self::Iterator<'_> {
        std::iter::empty()
    }

    fn current_line(&self) -> usize {
        usize::MAX
    }
}
