//! 正确地定位你的部件。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Position your widgets properly.
//!
//! </details>
mod limits;

pub mod flex;

pub use limits::Limits;

use crate::widget;
use crate::{Length, Padding, Point, Rectangle, Size, Vector};

/// 部件及其子部件的边界，使用绝对坐标。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The bounds of a widget and its children, using absolute coordinates.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Layout {
    position: Point,
    size: Size,
    parent: Option<Rectangle>,
}

impl Layout {
    /// 为给定的 [`Size`] 在原点处创建一个新的 [`Layout`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Layout`] for the given [`Size`] at the origin.
    ///
    /// </details>
    pub fn new(size: Size) -> Self {
        Self {
            position: Point::ORIGIN,
            size,
            parent: None,
        }
    }

    /// 返回 [`Layout`] 的位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the position of the [`Layout`].
    ///
    /// </details>
    pub fn position(&self) -> Point {
        self.position
    }

    /// 把 [`Layout`] 移动到给定的位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Moves the [`Layout`] to the given position.
    ///
    /// </details>
    pub fn move_to(self, position: impl Into<Point>) -> Self {
        Self {
            position: position.into(),
            size: self.size,
            parent: self.parent,
        }
    }

    /// 返回 [`Layout`] 的边界。
    ///
    /// 返回的 [`Rectangle`] 描述了已布局部件的
    /// 位置和尺寸。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the bounds of the [`Layout`].
    ///
    /// The returned [`Rectangle`] describes the position and size of the
    /// laid out widget.
    ///
    /// </details>
    pub fn bounds(&self) -> Rectangle {
        Rectangle::new(self.position, self.size)
    }

    /// 返回 [`Layout`] 的尺寸。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the size of the [`Layout`].
    ///
    /// </details>
    pub fn size(&self) -> Size {
        self.size
    }

    /// 返回此 [`Layout`] 的父级的边界（如果有）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the bounds of the parent of this [`Layout`], if any.
    ///
    /// </details>
    pub fn parent(&self) -> Option<Rectangle> {
        self.parent
    }

    /// 返回一个遍历此 [`Layout`] 的子级的迭代器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns an iterator over the children of this [`Layout`].
    ///
    /// </details>
    pub fn iter(
        self,
        children: &[widget::Tree],
    ) -> impl DoubleEndedIterator<Item = (Self, &widget::Tree)> + ExactSizeIterator {
        children.iter().map(move |child| (self.child(child), child))
    }

    /// 返回一个遍历此 [`Layout`] 的子级的可变迭代器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a mutable iterator over the children of this [`Layout`].
    ///
    /// </details>
    pub fn iter_mut(
        self,
        children: &mut [widget::Tree],
    ) -> impl DoubleEndedIterator<Item = (Self, &mut widget::Tree)> + ExactSizeIterator {
        children
            .iter_mut()
            .map(move |child| (self.child(child), child))
    }

    fn child(&self, child: &widget::Tree) -> Self {
        Self {
            position: self.position + child.translation,
            size: child.size,
            parent: Some(self.bounds()),
        }
    }
}

/// 在给定的 [`widget::Tree`] 中把两个子级并排布局。
///
/// 该树必须恰好有两个子级。第一个放在左边缘，
/// 第二个放在它右侧并间隔 `spacing`，两者
/// 在垂直方向上相互居中对齐。
///
/// 得到的尺寸会存储在树中，同时存储的还有各子级的
/// 尺寸和平移量。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Lays out two children side by side in the provided [`widget::Tree`].
///
/// The tree must have exactly two children. The first is placed at the left
/// edge, and the second to its right after `spacing`, both vertically
/// centered with respect to each other.
///
/// The resulting size is stored in the tree, along with the children's
/// sizes and translations.
///
/// </details>
pub fn next_to_each_other(tree: &mut widget::Tree, left: Size, right: Size, spacing: f32) {
    let (left_y, right_y) = if left.height > right.height {
        (0.0, (left.height - right.height) / 2.0)
    } else {
        ((right.height - left.height) / 2.0, 0.0)
    };

    tree.size = Size::new(
        left.width + spacing + right.width,
        left.height.max(right.height),
    );

    tree.children[0].size = left;
    tree.children[0].translation = Vector::new(0.0, left_y);

    tree.children[1].size = right;
    tree.children[1].translation = Vector::new(left.width + spacing, right_y);
}

/// 给定宽度和高度要求，且没有固有尺寸时，
/// 计算出符合 [`Limits`] 的 [`Size`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Computes the resulting [`Size`] that fits the [`Limits`] given
/// some width and height requirements and no intrinsic size.
///
/// </details>
pub fn atomic(limits: &Limits, width: impl Into<Length>, height: impl Into<Length>) -> Size {
    let width = width.into();
    let height = height.into();

    limits
        .width(width)
        .height(height)
        .resolve(width, height, Size::ZERO)
}

/// 给定宽度和高度要求，以及一个在给定 [`Limits`] 内
/// 产生固有 [`Size`] 的闭包，
/// 计算出符合 [`Limits`] 的 [`Size`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Computes the resulting [`Size`] that fits the [`Limits`] given
/// some width and height requirements and a closure that produces
/// the intrinsic [`Size`] inside the given [`Limits`].
///
/// </details>
pub fn sized(
    limits: &Limits,
    width: impl Into<Length>,
    height: impl Into<Length>,
    f: impl FnOnce(&Limits) -> Size,
) -> Size {
    let width = width.into();
    let height = height.into();

    let limits = limits.width(width).height(height);
    let intrinsic_size = f(&limits);

    limits.resolve(width, height, intrinsic_size)
}

/// 给定宽度和高度要求，以及一个在给定 [`Limits`] 内
/// 产生内容的闭包，
/// 计算出符合 [`Limits`] 的 [`Size`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Computes the resulting [`Size`] that fits the [`Limits`] given
/// some width and height requirements and a closure that produces
/// the content inside the given [`Limits`].
///
/// </details>
pub fn contained(
    tree: &mut widget::Tree,
    limits: &Limits,
    width: impl Into<Length>,
    height: impl Into<Length>,
    f: impl FnOnce(&mut widget::Tree, &Limits) -> Size,
) {
    let width = width.into();
    let height = height.into();
    let limits = limits.width(width).height(height);

    let child = &mut tree.children[0];
    let content = f(child, &limits);

    tree.size = limits.resolve(width, height, content);
    child.translation = Vector::ZERO;
}

/// 给定宽度、高度和 [`Padding`] 要求，以及一个在给定 [`Limits`] 内
/// 产生内容的闭包，
/// 计算出符合 [`Limits`] 的 [`Size`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Computes the [`Size`] that fits the [`Limits`] given some width, height, and
/// [`Padding`] requirements and a closure that produces the content
/// inside the given [`Limits`].
///
/// </details>
pub fn padded(
    tree: &mut widget::Tree,
    limits: &Limits,
    width: impl Into<Length>,
    height: impl Into<Length>,
    padding: impl Into<Padding>,
    layout: impl FnOnce(&mut widget::Tree, &Limits) -> Size,
) {
    positioned(tree, limits, width, height, padding, layout, |_, _| {
        Vector::ZERO
    });
}

/// 计算一个 [`padded`] 的 [`Size`]，并附带一个定位步骤。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Computes a [`padded`] [`Size`] with a positioning step.
///
/// </details>
pub fn positioned(
    tree: &mut widget::Tree,
    limits: &Limits,
    width: impl Into<Length>,
    height: impl Into<Length>,
    padding: impl Into<Padding>,
    layout: impl FnOnce(&mut widget::Tree, &Limits) -> Size,
    translate: impl FnOnce(Size, Size) -> Vector,
) {
    let width = width.into();
    let height = height.into();
    let padding = padding.into();
    let limits = limits.width(width).height(height);

    let child = &mut tree.children[0];
    let content = layout(child, &limits.shrink(padding));
    let padding = padding.fit(content, limits.bounds());

    let container = limits.shrink(padding).resolve(width, height, content);

    tree.size = container.expand(padding);
    child.translation = Vector::new(padding.left, padding.top) + translate(content, container);
}
