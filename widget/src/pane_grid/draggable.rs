use crate::core::{Layout, Point};

/// 可以被拖动的窗格。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A pane that can be dragged.
///
/// </details>
pub trait Draggable {
    /// 返回具有给定 [`Layout`] 的 [`Draggable`] 能否在
    /// 提供的光标位置被拾取。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns whether the [`Draggable`] with the given [`Layout`] can be picked
    /// at the provided cursor position.
    ///
    /// </details>
    fn can_be_dragged_at(&self, layout: Layout<'_>, cursor: Point) -> bool;
}
