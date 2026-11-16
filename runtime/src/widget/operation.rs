//! 改变部件的内部状态。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Change internal widget state.
//!
//! </details>
use crate::core::widget::Id;
use crate::core::widget::operation;
use crate::task;
use crate::{Action, Task};

pub use crate::core::widget::operation::Animation;

/// 让上一个可获得焦点的部件获得焦点。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Focuses the previous focusable widget.
///
/// </details>
pub fn focus_previous<T>() -> Task<T> {
    task::effect(Action::widget(operation::focusable::focus_previous()))
}

/// 让下一个可获得焦点的部件获得焦点。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Focuses the next focusable widget.
///
/// </details>
pub fn focus_next<T>() -> Task<T> {
    task::effect(Action::widget(operation::focusable::focus_next()))
}

/// 返回具有给定 [`Id`] 的部件是否获得了焦点。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns whether the widget with the given [`Id`] is focused or not.
///
/// </details>
pub fn is_focused(id: impl Into<Id>) -> Task<bool> {
    task::widget(operation::focusable::is_focused(id.into()))
}

/// 让具有给定 [`Id`] 的部件获得焦点。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Focuses the widget with the given [`Id`].
///
/// </details>
pub fn focus<T>(id: impl Into<Id>) -> Task<T> {
    task::effect(Action::widget(operation::focusable::focus(id.into())))
}

/// 针对可滚动部件的操作。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Operations for widgets that can scroll.
///
/// </details>
pub mod scrollable {
    use super::*;

    pub use crate::core::widget::operation::scrollable::{AbsoluteOffset, RelativeOffset};

    /// 把具有给定 [`Id`] 的可滚动区域直接定位到给定的 [`RelativeOffset`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Snaps the scrollable with the given [`Id`] to the provided [`RelativeOffset`].
    ///
    /// </details>
    pub fn snap_to<T>(
        id: impl Into<Id>,
        offset: impl Into<RelativeOffset<Option<f32>>>,
        animation: Animation,
    ) -> Task<T> {
        task::effect(Action::widget(operation::scrollable::snap_to(
            id.into(),
            offset.into(),
            animation,
        )))
    }

    /// 把具有给定 [`Id`] 的可滚动区域直接定位到 [`RelativeOffset::END`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Snaps the scrollable with the given [`Id`] to the [`RelativeOffset::END`].
    ///
    /// </details>
    pub fn snap_to_end<T>(id: impl Into<Id>, animation: Animation) -> Task<T> {
        task::effect(Action::widget(operation::scrollable::snap_to(
            id.into(),
            RelativeOffset::END.into(),
            animation,
        )))
    }

    /// 把具有给定 [`Id`] 的可滚动区域滚动到给定的 [`AbsoluteOffset`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Scrolls the scrollable with the given [`Id`] to the provided [`AbsoluteOffset`].
    ///
    /// </details>
    pub fn scroll_to<T>(
        id: impl Into<Id>,
        offset: impl Into<AbsoluteOffset<Option<f32>>>,
        animation: Animation,
    ) -> Task<T> {
        task::effect(Action::widget(operation::scrollable::scroll_to(
            id.into(),
            offset.into(),
            animation,
        )))
    }

    /// 把具有给定 [`Id`] 的可滚动区域滚动给定的 [`AbsoluteOffset`] 偏移量。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Scrolls the scrollable with the given [`Id`] by the provided [`AbsoluteOffset`].
    ///
    /// </details>
    pub fn scroll_by<T>(
        id: impl Into<Id>,
        offset: AbsoluteOffset,
        animation: Animation,
    ) -> Task<T> {
        task::effect(Action::widget(operation::scrollable::scroll_by(
            id.into(),
            offset,
            animation,
        )))
    }
}

/// 针对包含文本的部件的操作。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Operations for widgets that contain text.
///
/// </details>
pub mod text {
    use super::*;
    use crate::core::Point;
    use crate::core::text::Target;

    /// 选中给定的两个位置（布局坐标）之间的文本。
    ///
    /// `start` 是用户开始选择的位置，
    /// `end` 是选择的最终位置。
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
    pub fn select<T>(start: Point, end: Point, target: Target) -> Task<T> {
        task::effect(Action::widget(operation::text::select(start, end, target)))
    }

    /// 选中具有给定 [`Id`] 的部件中的全部文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Selects all text in the widget with the given [`Id`].
    ///
    /// </details>
    pub fn select_all<T>(id: impl Into<Id>) -> Task<T> {
        task::effect(Action::widget(operation::scope(
            id.into(),
            operation::text::select_all(),
        )))
    }

    /// 取消选中任何已选中的文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Deselects any selected text.
    ///
    /// </details>
    pub fn deselect<T>() -> Task<T> {
        task::effect(Action::widget(operation::text::deselect()))
    }
}

/// 针对可用于文本输入的部件的操作。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Operations for widgets that can be used for text input.
///
/// </details>
pub mod text_input {
    use super::*;
    use crate::core::text::Position;

    /// 选中具有给定 [`Id`] 的部件的全部内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Selects all the content of the widget with the given [`Id`].
    ///
    /// </details>
    pub fn select_all<T>(id: impl Into<Id>) -> Task<T> {
        task::effect(Action::widget(operation::text_input::select_all(id.into())))
    }

    /// 选中具有给定 [`Id`] 的部件中给定的内容范围。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Selects the given content range of the widget with the given [`Id`].
    ///
    /// </details>
    pub fn select_range<T>(id: impl Into<Id>, start: Position, end: Position) -> Task<T> {
        task::effect(Action::widget(operation::text_input::select_range(
            id.into(),
            start,
            end,
        )))
    }

    /// 把具有给定 [`Id`] 的部件的光标移到末尾。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Moves the cursor of the widget with the given [`Id`] to the end.
    ///
    /// </details>
    pub fn move_cursor_to_end<T>(id: impl Into<Id>) -> Task<T> {
        task::effect(Action::widget(operation::text_input::move_cursor_to_end(
            id.into(),
        )))
    }

    /// 把具有给定 [`Id`] 的部件的光标移到开头。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Moves the cursor of the widget with the given [`Id`] to the front.
    ///
    /// </details>
    pub fn move_cursor_to_front<T>(id: impl Into<Id>) -> Task<T> {
        task::effect(Action::widget(operation::text_input::move_cursor_to_front(
            id.into(),
        )))
    }

    /// 把具有给定 [`Id`] 的部件的光标移到给定的位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Moves the cursor of the widget with the given [`Id`] to the provided position.
    ///
    /// </details>
    pub fn move_cursor_to<T>(id: impl Into<Id>, position: Position) -> Task<T> {
        task::effect(Action::widget(operation::text_input::move_cursor_to(
            id.into(),
            position,
        )))
    }
}
