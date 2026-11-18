//! 对可以滚动的部件进行操作。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Operate on widgets that can be scrolled.
//!
//! </details>
use crate::widget::{Id, Operation};
use crate::{Rectangle, Vector};

/// 可以滚动的部件的内部状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The internal state of a widget that can be scrolled.
///
/// </details>
pub trait Scrollable {
    /// 把部件的滚动位置在水平和垂直轴上对齐到给定的 `percentage`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Snaps the scroll of the widget to the given `percentage` along the horizontal & vertical axis.
    ///
    /// </details>
    fn snap_to(&mut self, offset: RelativeOffset<Option<f32>>);

    /// 把部件在水平和垂直轴上滚动到给定的 [`AbsoluteOffset`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Scroll the widget to the given [`AbsoluteOffset`] along the horizontal & vertical axis.
    ///
    /// </details>
    fn scroll_to(&mut self, offset: AbsoluteOffset<Option<f32>>);

    /// 把部件在水平和垂直轴上滚动给定的 [`AbsoluteOffset`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Scroll the widget by the given [`AbsoluteOffset`] along the horizontal & vertical axis.
    ///
    /// </details>
    fn scroll_by(
        &mut self,
        offset: AbsoluteOffset,
        bounds: Rectangle,
        content_bounds: Rectangle,
    );
}

/// 产生一个 [`Operation`]，把具有给定 [`Id`] 的部件对齐到
/// 给定的 `percentage`。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Produces an [`Operation`] that snaps the widget with the given [`Id`] to
/// the provided `percentage`.
///
/// </details>
pub fn snap_to<T>(
    target: Id,
    offset: RelativeOffset<Option<f32>>,
) -> impl Operation<T> {
    struct SnapTo {
        target: Id,
        offset: RelativeOffset<Option<f32>>,
    }

    impl<T> Operation<T> for SnapTo {
        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<T>)) {
            operate(self);
        }

        fn scrollable(
            &mut self,
            id: Option<&Id>,
            _bounds: Rectangle,
            _content_bounds: Rectangle,
            _translation: Vector,
            state: &mut dyn Scrollable,
        ) {
            if Some(&self.target) == id {
                state.snap_to(self.offset);
            }
        }
    }

    SnapTo { target, offset }
}

/// 产生一个 [`Operation`]，把具有给定 [`Id`] 的部件滚动到
/// 给定的 [`AbsoluteOffset`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Produces an [`Operation`] that scrolls the widget with the given [`Id`] to
/// the provided [`AbsoluteOffset`].
///
/// </details>
pub fn scroll_to<T>(
    target: Id,
    offset: AbsoluteOffset<Option<f32>>,
) -> impl Operation<T> {
    struct ScrollTo {
        target: Id,
        offset: AbsoluteOffset<Option<f32>>,
    }

    impl<T> Operation<T> for ScrollTo {
        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<T>)) {
            operate(self);
        }

        fn scrollable(
            &mut self,
            id: Option<&Id>,
            _bounds: Rectangle,
            _content_bounds: Rectangle,
            _translation: Vector,
            state: &mut dyn Scrollable,
        ) {
            if Some(&self.target) == id {
                state.scroll_to(self.offset);
            }
        }
    }

    ScrollTo { target, offset }
}

/// 产生一个 [`Operation`]，把具有给定 [`Id`] 的部件滚动
/// 给定的 [`AbsoluteOffset`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Produces an [`Operation`] that scrolls the widget with the given [`Id`] by
/// the provided [`AbsoluteOffset`].
///
/// </details>
pub fn scroll_by<T>(target: Id, offset: AbsoluteOffset) -> impl Operation<T> {
    struct ScrollBy {
        target: Id,
        offset: AbsoluteOffset,
    }

    impl<T> Operation<T> for ScrollBy {
        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<T>)) {
            operate(self);
        }

        fn scrollable(
            &mut self,
            id: Option<&Id>,
            bounds: Rectangle,
            content_bounds: Rectangle,
            _translation: Vector,
            state: &mut dyn Scrollable,
        ) {
            if Some(&self.target) == id {
                state.scroll_by(self.offset, bounds, content_bounds);
            }
        }
    }

    ScrollBy { target, offset }
}

/// [`Scrollable`] 在各个方向上的绝对偏移量。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The amount of absolute offset in each direction of a [`Scrollable`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AbsoluteOffset<T = f32> {
    /// 水平偏移量
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The amount of horizontal offset
    ///
    /// </details>
    pub x: T,
    /// 垂直偏移量
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The amount of vertical offset
    ///
    /// </details>
    pub y: T,
}

impl From<AbsoluteOffset> for AbsoluteOffset<Option<f32>> {
    fn from(offset: AbsoluteOffset) -> Self {
        Self {
            x: Some(offset.x),
            y: Some(offset.y),
        }
    }
}

/// [`Scrollable`] 在各个方向上的相对偏移量。
///
/// `0.0` 表示起点，`1.0` 表示终点。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The amount of relative offset in each direction of a [`Scrollable`].
///
/// A value of `0.0` means start, while `1.0` means end.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RelativeOffset<T = f32> {
    /// 水平偏移量
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The amount of horizontal offset
    ///
    /// </details>
    pub x: T,
    /// 垂直偏移量
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The amount of vertical offset
    ///
    /// </details>
    pub y: T,
}

impl RelativeOffset {
    /// 指向 [`Scrollable`] 左上角的相对偏移。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A relative offset that points to the top-left of a [`Scrollable`].
    ///
    /// </details>
    pub const START: Self = Self { x: 0.0, y: 0.0 };

    /// 指向 [`Scrollable`] 右下角的相对偏移。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A relative offset that points to the bottom-right of a [`Scrollable`].
    ///
    /// </details>
    pub const END: Self = Self { x: 1.0, y: 1.0 };
}

impl From<RelativeOffset> for RelativeOffset<Option<f32>> {
    fn from(offset: RelativeOffset) -> Self {
        Self {
            x: Some(offset.x),
            y: Some(offset.y),
        }
    }
}
