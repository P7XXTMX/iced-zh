//! 跟踪文本输入框的光标。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Track the cursor of a text input.
//!
//! </details>
use crate::text_input::Value;

/// 文本输入框的光标。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The cursor of a text input.
///
/// </details>
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Cursor {
    state: State,
}

/// [`Cursor`] 的状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The state of a [`Cursor`].
///
/// </details>
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    /// 没有选区的光标
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Cursor without a selection
    ///
    /// </details>
    Index(usize),

    /// 正在选中一段文本的光标
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Cursor selecting a range of text
    ///
    /// </details>
    Selection {
        /// 选区的起点
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The start of the selection
        ///
        /// </details>
        start: usize,
        /// 选区的终点
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The end of the selection
        ///
        /// </details>
        end: usize,
    },
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            state: State::Index(0),
        }
    }
}

impl Cursor {
    /// 返回 [`Cursor`] 的 [`State`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`State`] of the [`Cursor`].
    ///
    /// </details>
    pub fn state(&self, value: &Value) -> State {
        match self.state {
            State::Index(index) => State::Index(index.min(value.len())),
            State::Selection { start, end } => {
                let start = start.min(value.len());
                let end = end.min(value.len());

                if start == end {
                    State::Index(start)
                } else {
                    State::Selection { start, end }
                }
            }
        }
    }

    /// 返回 [`Cursor`] 在给定 [`Value`] 上的当前选区。
    ///
    /// `start` 保证小于等于 `end`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current selection of the [`Cursor`] for the given [`Value`].
    ///
    /// `start` is guaranteed to be <= than `end`.
    ///
    /// </details>
    pub fn selection(&self, value: &Value) -> Option<(usize, usize)> {
        match self.state(value) {
            State::Selection { start, end } => {
                Some((start.min(end), start.max(end)))
            }
            State::Index(_) => None,
        }
    }

    pub(crate) fn move_to(&mut self, position: usize) {
        self.state = State::Index(position);
    }

    pub(crate) fn move_right(&mut self, value: &Value) {
        self.move_right_by_amount(value, 1);
    }

    pub(crate) fn move_right_by_words(&mut self, value: &Value) {
        self.move_to(value.next_end_of_word(self.right(value)));
    }

    pub(crate) fn move_right_by_amount(
        &mut self,
        value: &Value,
        amount: usize,
    ) {
        match self.state(value) {
            State::Index(index) => {
                self.move_to(index.saturating_add(amount).min(value.len()));
            }
            State::Selection { start, end } => self.move_to(end.max(start)),
        }
    }

    pub(crate) fn move_left(&mut self, value: &Value) {
        match self.state(value) {
            State::Index(index) if index > 0 => self.move_to(index - 1),
            State::Selection { start, end } => self.move_to(start.min(end)),
            State::Index(_) => self.move_to(0),
        }
    }

    pub(crate) fn move_left_by_words(&mut self, value: &Value) {
        self.move_to(value.previous_start_of_word(self.left(value)));
    }

    pub(crate) fn select_range(&mut self, start: usize, end: usize) {
        if start == end {
            self.state = State::Index(start);
        } else {
            self.state = State::Selection { start, end };
        }
    }

    pub(crate) fn select_left(&mut self, value: &Value) {
        match self.state(value) {
            State::Index(index) if index > 0 => {
                self.select_range(index, index - 1);
            }
            State::Selection { start, end } if end > 0 => {
                self.select_range(start, end - 1);
            }
            _ => {}
        }
    }

    pub(crate) fn select_right(&mut self, value: &Value) {
        match self.state(value) {
            State::Index(index) if index < value.len() => {
                self.select_range(index, index + 1);
            }
            State::Selection { start, end } if end < value.len() => {
                self.select_range(start, end + 1);
            }
            _ => {}
        }
    }

    pub(crate) fn select_left_by_words(&mut self, value: &Value) {
        match self.state(value) {
            State::Index(index) => {
                self.select_range(index, value.previous_start_of_word(index));
            }
            State::Selection { start, end } => {
                self.select_range(start, value.previous_start_of_word(end));
            }
        }
    }

    pub(crate) fn select_right_by_words(&mut self, value: &Value) {
        match self.state(value) {
            State::Index(index) => {
                self.select_range(index, value.next_end_of_word(index));
            }
            State::Selection { start, end } => {
                self.select_range(start, value.next_end_of_word(end));
            }
        }
    }

    pub(crate) fn select_all(&mut self, value: &Value) {
        self.select_range(0, value.len());
    }

    pub(crate) fn start(&self, value: &Value) -> usize {
        let start = match self.state {
            State::Index(index) => index,
            State::Selection { start, .. } => start,
        };

        start.min(value.len())
    }

    pub(crate) fn end(&self, value: &Value) -> usize {
        let end = match self.state {
            State::Index(index) => index,
            State::Selection { end, .. } => end,
        };

        end.min(value.len())
    }

    fn left(&self, value: &Value) -> usize {
        match self.state(value) {
            State::Index(index) => index,
            State::Selection { start, end } => start.min(end),
        }
    }

    fn right(&self, value: &Value) -> usize {
        match self.state(value) {
            State::Index(index) => index,
            State::Selection { start, end } => start.max(end),
        }
    }
}
