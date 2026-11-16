//! 对可以获得焦点的部件进行操作。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Operate on widgets that can be focused.
//!
//! </details>
use crate::Rectangle;
use crate::widget::Id;
use crate::widget::operation::{self, Operation, Outcome};

/// 可以获得焦点的部件的内部状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The internal state of a widget that can be focused.
///
/// </details>
pub trait Focusable {
    /// 返回部件是否获得焦点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns whether the widget is focused or not.
    ///
    /// </details>
    fn is_focused(&self) -> bool;

    /// 让部件获得焦点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Focuses the widget.
    ///
    /// </details>
    fn focus(&mut self);

    /// 让部件失去焦点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Unfocuses the widget.
    ///
    /// </details>
    fn unfocus(&mut self);
}

/// 部件树中可获得焦点的部件的汇总信息。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A summary of the focusable widgets present on a widget tree.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Count {
    /// 当前获得焦点的部件的索引（如果有）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The index of the current focused widget, if any.
    ///
    /// </details>
    pub focused: Option<usize>,

    /// 可获得焦点的部件的总数。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The total amount of focusable widgets.
    ///
    /// </details>
    pub total: usize,
}

/// 产生一个 [`Operation`]，让具有给定 [`Id`] 的部件获得焦点。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Produces an [`Operation`] that focuses the widget with the given [`Id`].
///
/// </details>
pub fn focus<T>(target: Id) -> impl Operation<T> {
    struct Focus {
        target: Id,
    }

    impl<T> Operation<T> for Focus {
        fn focusable(&mut self, id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
            match id {
                Some(id) if id == &self.target => {
                    state.focus();
                }
                _ => {
                    state.unfocus();
                }
            }
        }

        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<T>)) {
            operate(self);
        }
    }

    Focus { target }
}

/// 产生一个 [`Operation`]，让当前获得焦点的部件失去焦点。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Produces an [`Operation`] that unfocuses the focused widget.
///
/// </details>
pub fn unfocus<T>() -> impl Operation<T> {
    struct Unfocus;

    impl<T> Operation<T> for Unfocus {
        fn focusable(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
            state.unfocus();
        }

        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<T>)) {
            operate(self);
        }
    }

    Unfocus
}

/// 产生一个 [`Operation`]，它生成一个 [`Count`]，并将其与
/// 给定的函数串联起来，构建一个新的 [`Operation`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Produces an [`Operation`] that generates a [`Count`] and chains it with the
/// provided function to build a new [`Operation`].
///
/// </details>
pub fn count() -> impl Operation<Count> {
    struct CountFocusable {
        count: Count,
    }

    impl Operation<Count> for CountFocusable {
        fn focusable(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
            if state.is_focused() {
                self.count.focused = Some(self.count.total);
            }

            self.count.total += 1;
        }

        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<Count>)) {
            operate(self);
        }

        fn finish(&self) -> Outcome<Count> {
            Outcome::Some(self.count)
        }
    }

    CountFocusable {
        count: Count::default(),
    }
}

/// 产生一个 [`Operation`]，它查找当前获得焦点的部件，并且
/// - 如果找到，让上一个可获得焦点的部件获得焦点。
/// - 如果没找到，让最后一个可获得焦点的部件获得焦点。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Produces an [`Operation`] that searches for the current focused widget, and
/// - if found, focuses the previous focusable widget.
/// - if not found, focuses the last focusable widget.
///
/// </details>
pub fn focus_previous<T>() -> impl Operation<T>
where
    T: Send + 'static,
{
    struct FocusPrevious {
        count: Count,
        current: usize,
    }

    impl<T> Operation<T> for FocusPrevious {
        fn focusable(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
            if self.count.total == 0 {
                return;
            }

            match self.count.focused {
                None if self.current == self.count.total - 1 => state.focus(),
                Some(0) if self.current == 0 => state.unfocus(),
                Some(0) => {}
                Some(focused) if focused == self.current => state.unfocus(),
                Some(focused) if focused - 1 == self.current => state.focus(),
                _ => {}
            }

            self.current += 1;
        }

        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<T>)) {
            operate(self);
        }
    }

    operation::then(count(), |count| FocusPrevious { count, current: 0 })
}

/// 产生一个 [`Operation`]，它查找当前获得焦点的部件，并且
/// - 如果找到，让下一个可获得焦点的部件获得焦点。
/// - 如果没找到，让第一个可获得焦点的部件获得焦点。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Produces an [`Operation`] that searches for the current focused widget, and
/// - if found, focuses the next focusable widget.
/// - if not found, focuses the first focusable widget.
///
/// </details>
pub fn focus_next<T>() -> impl Operation<T>
where
    T: Send + 'static,
{
    struct FocusNext {
        count: Count,
        current: usize,
    }

    impl<T> Operation<T> for FocusNext {
        fn focusable(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
            match self.count.focused {
                None if self.current == 0 => state.focus(),
                Some(focused) if focused == self.current => state.unfocus(),
                Some(focused) if focused + 1 == self.current => state.focus(),
                _ => {}
            }

            self.current += 1;
        }

        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<T>)) {
            operate(self);
        }
    }

    operation::then(count(), |count| FocusNext { count, current: 0 })
}

/// 产生一个 [`Operation`]，它查找当前获得焦点的部件
/// 并保存其 ID。没有 ID 的部件会被忽略。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Produces an [`Operation`] that searches for the current focused widget
/// and stores its ID. This ignores widgets that do not have an ID.
///
/// </details>
pub fn find_focused() -> impl Operation<Id> {
    struct FindFocused {
        focused: Option<Id>,
    }

    impl Operation<Id> for FindFocused {
        fn focusable(&mut self, id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
            if state.is_focused() && id.is_some() {
                self.focused = id.cloned();
            }
        }

        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<Id>)) {
            operate(self);
        }

        fn finish(&self) -> Outcome<Id> {
            if let Some(id) = &self.focused {
                Outcome::Some(id.clone())
            } else {
                Outcome::None
            }
        }
    }

    FindFocused { focused: None }
}

/// 产生一个 [`Operation`]，它查找指定的可获得焦点的部件，
/// 并保存它是否获得焦点。没有 ID 的部件
/// 会被忽略。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Produces an [`Operation`] that searches for the focusable widget
/// and stores whether it is focused or not. This ignores widgets that
/// do not have an ID.
///
/// </details>
pub fn is_focused(target: Id) -> impl Operation<bool> {
    struct IsFocused {
        target: Id,
        is_focused: Option<bool>,
    }

    impl Operation<bool> for IsFocused {
        fn focusable(&mut self, id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
            if id.is_some_and(|id| *id == self.target) {
                self.is_focused = Some(state.is_focused());
            }
        }

        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<bool>)) {
            if self.is_focused.is_some() {
                return;
            }

            operate(self);
        }

        fn finish(&self) -> Outcome<bool> {
            self.is_focused.map_or(Outcome::None, Outcome::Some)
        }
    }

    IsFocused {
        target,
        is_focused: None,
    }
}
