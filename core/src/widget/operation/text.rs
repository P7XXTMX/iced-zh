//! 对含有文本的部件进行操作。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Operate on widgets that have text.
//!
//! </details>
use crate::Point;
use crate::Vector;
use crate::text::{Fragment, Target};
use crate::widget::operation::{Operation, Outcome};

/// 含有文本的部件的内部状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The internal state of a widget that has text.
///
/// </details>
pub trait Text {
    /// 返回部件的文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the text of the widget.
    ///
    /// </details>
    fn text(&self) -> Fragment<'_>;

    /// 选择给定位置之间的文本，位置使用布局坐标。
    ///
    /// `start` 是用户开始选择的位置，
    /// 而 `end` 是选区的最终位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Selects text between the given positions in layout coordinates.
    ///
    /// `start` is the position where the user started selecting from,
    /// while `end` is the final position of the selection.
    ///
    /// </details>
    fn select(&mut self, start: Point, end: Point, target: Target);

    /// 选择部件的全部文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Selects all the text of the widget.
    ///
    /// </details>
    fn select_all(&mut self);

    /// 取消选择所有已选中的文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Deselects any selected text;
    ///
    /// </details>
    fn deselect(&mut self);

    /// 返回选中的文本（如果有）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the selected text, if any.
    ///
    /// </details>
    ///
    // TODO: Make immutable
    // 待办：改为不可变
    fn copy(&mut self) -> Option<String>;
}

impl Text for String {
    fn text(&self) -> Fragment<'_> {
        self.as_str().into()
    }

    fn select(&mut self, _start: Point, _end: Point, _target: Target) {}
    fn select_all(&mut self) {}
    fn deselect(&mut self) {}

    fn copy(&mut self) -> Option<String> {
        None
    }
}

/// 选择给定位置之间的文本，位置使用布局坐标。
///
/// `start` 是用户开始选择的位置，
/// 而 `end` 是选区的最终位置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Selects text between the given positions in layout coordinates.
///
/// `start` is the position where the user started selecting from,
/// while `end` is the final position of the selection.
///
/// </details>
pub fn select(start: Point, end: Point, target: Target) -> impl Operation {
    struct Select {
        start: Point,
        end: Point,
        target: Target,
        translation: Vector,
    }

    impl Operation for Select {
        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
            let translation = self.translation;
            operate(self);
            self.translation = translation;
        }

        fn scrollable(
            &mut self,
            _id: Option<&crate::widget::Id>,
            _bounds: crate::Rectangle,
            _content: crate::Size,
            translation: crate::Vector,
            _state: &mut dyn super::Scrollable,
        ) {
            self.translation += translation;
        }

        fn text(
            &mut self,
            _id: Option<&crate::widget::Id>,
            _bounds: crate::Rectangle,
            state: &mut dyn Text,
        ) {
            state.select(
                self.start + self.translation,
                self.end + self.translation,
                self.target,
            );
        }
    }

    Select {
        start,
        end,
        target,
        translation: Vector::ZERO,
    }
}

/// 选择所有可选择的文本。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Selects all selectable text.
///
/// </details>
pub fn select_all() -> impl Operation {
    struct SelectAll;

    impl Operation for SelectAll {
        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
            operate(self);
        }

        fn text(
            &mut self,
            _id: Option<&crate::widget::Id>,
            _bounds: crate::Rectangle,
            state: &mut dyn Text,
        ) {
            state.select_all();
        }
    }

    SelectAll
}

/// 取消选择所有已选中的文本。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Deselects any selected text.
///
/// </details>
pub fn deselect() -> impl Operation {
    struct Deselect;

    impl Operation for Deselect {
        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
            operate(self);
        }

        fn text(
            &mut self,
            _id: Option<&crate::widget::Id>,
            _bounds: crate::Rectangle,
            state: &mut dyn Text,
        ) {
            state.deselect();
        }
    }

    Deselect
}

/// 收集所有选中的文本。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Collects all the selected text.
///
/// </details>
pub fn copy() -> impl Operation<String> {
    struct Copy {
        text: String,
    }

    impl Operation<String> for Copy {
        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<String>)) {
            operate(self);
        }

        fn text(
            &mut self,
            _id: Option<&crate::widget::Id>,
            _bounds: crate::Rectangle,
            state: &mut dyn Text,
        ) {
            let Some(text) = state.copy() else {
                return;
            };

            if text.is_empty() {
                return;
            }

            if !self.text.is_empty() {
                self.text.push('\n');
            }

            self.text.push_str(&text);
        }

        fn finish(&self) -> Outcome<String> {
            Outcome::Some(self.text.clone())
        }
    }

    Copy {
        text: String::new(),
    }
}
