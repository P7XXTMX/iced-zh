//! 对具有文本输入的部件进行操作。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Operate on widgets that have text input.
//!
//! </details>
use crate::Rectangle;
use crate::text;
use crate::widget::Id;
use crate::widget::operation::Operation;

/// 具有文本输入的部件的内部状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The internal state of a widget that has text input.
///
/// </details>
pub trait TextInput {
    /// 返回文本输入当前*可见*的文本。
    ///
    /// 通常，它要么是输入的值，要么是占位文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current _visible_ text of the text input.
    ///
    /// Normally, this is either its value or its placeholder.
    ///
    /// </details>
    fn text(&self) -> text::Fragment<'_>;

    /// 把文本输入的光标移动到输入文本的开头。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Moves the cursor of the text input to the front of the input text.
    ///
    /// </details>
    fn move_cursor_to_front(&mut self);

    /// 把文本输入的光标移动到输入文本的末尾。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Moves the cursor of the text input to the end of the input text.
    ///
    /// </details>
    fn move_cursor_to_end(&mut self);

    /// 把文本输入的光标移动到任意位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Moves the cursor of the text input to an arbitrary location.
    ///
    /// </details>
    fn move_cursor_to(&mut self, location: text::Position);

    /// 选择文本输入的全部内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Selects all the content of the text input.
    ///
    /// </details>
    fn select_all(&mut self);

    /// 选择文本输入中给定范围的内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Selects the given content range of the text input.
    ///
    /// </details>
    fn select_range(&mut self, start: text::Position, end: text::Position);
}

/// 产生一个 [`Operation`]，把具有给定 [`Id`] 的部件的光标移动到
/// 开头。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Produces an [`Operation`] that moves the cursor of the widget with the given [`Id`] to the
/// front.
///
/// </details>
pub fn move_cursor_to_front<T>(target: Id) -> impl Operation<T> {
    struct MoveCursor {
        target: Id,
    }

    impl<T> Operation<T> for MoveCursor {
        fn text_input(&mut self, id: Option<&Id>, _bounds: Rectangle, state: &mut dyn TextInput) {
            match id {
                Some(id) if id == &self.target => {
                    state.move_cursor_to_front();
                }
                _ => {}
            }
        }

        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<T>)) {
            operate(self);
        }
    }

    MoveCursor { target }
}

/// 产生一个 [`Operation`]，把具有给定 [`Id`] 的部件的光标移动到
/// 末尾。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Produces an [`Operation`] that moves the cursor of the widget with the given [`Id`] to the
/// end.
///
/// </details>
pub fn move_cursor_to_end<T>(target: Id) -> impl Operation<T> {
    struct MoveCursor {
        target: Id,
    }

    impl<T> Operation<T> for MoveCursor {
        fn text_input(&mut self, id: Option<&Id>, _bounds: Rectangle, state: &mut dyn TextInput) {
            match id {
                Some(id) if id == &self.target => {
                    state.move_cursor_to_end();
                }
                _ => {}
            }
        }

        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<T>)) {
            operate(self);
        }
    }

    MoveCursor { target }
}

/// 产生一个 [`Operation`]，把具有给定 [`Id`] 的部件的光标移动到
/// 给定的位置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Produces an [`Operation`] that moves the cursor of the widget with the given [`Id`] to the
/// provided position.
///
/// </details>
pub fn move_cursor_to<T>(target: Id, position: text::Position) -> impl Operation<T> {
    struct MoveCursor {
        target: Id,
        position: text::Position,
    }

    impl<T> Operation<T> for MoveCursor {
        fn text_input(&mut self, id: Option<&Id>, _bounds: Rectangle, state: &mut dyn TextInput) {
            match id {
                Some(id) if id == &self.target => {
                    state.move_cursor_to(self.position);
                }
                _ => {}
            }
        }

        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<T>)) {
            operate(self);
        }
    }

    MoveCursor { target, position }
}

/// 产生一个 [`Operation`]，选择具有给定 [`Id`] 的部件的全部内容。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Produces an [`Operation`] that selects all the content of the widget with the given [`Id`].
///
/// </details>
pub fn select_all<T>(target: Id) -> impl Operation<T> {
    struct MoveCursor {
        target: Id,
    }

    impl<T> Operation<T> for MoveCursor {
        fn text_input(&mut self, id: Option<&Id>, _bounds: Rectangle, state: &mut dyn TextInput) {
            match id {
                Some(id) if id == &self.target => {
                    state.select_all();
                }
                _ => {}
            }
        }

        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<T>)) {
            operate(self);
        }
    }

    MoveCursor { target }
}

/// 产生一个 [`Operation`]，选择具有给定 [`Id`] 的部件中给定范围的内容。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Produces an [`Operation`] that selects the given content range of the widget with the given [`Id`].
///
/// </details>
pub fn select_range<T>(
    target: Id,
    start: text::Position,
    end: text::Position,
) -> impl Operation<T> {
    struct SelectRange {
        target: Id,
        start: text::Position,
        end: text::Position,
    }

    impl<T> Operation<T> for SelectRange {
        fn text_input(&mut self, id: Option<&Id>, _bounds: Rectangle, state: &mut dyn TextInput) {
            match id {
                Some(id) if id == &self.target => {
                    state.select_range(self.start, self.end);
                }
                _ => {}
            }
        }

        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<T>)) {
            operate(self);
        }
    }

    SelectRange { target, start, end }
}
