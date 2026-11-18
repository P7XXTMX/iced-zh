//! 正确地定位你的部件。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Position your widgets properly.
//!
//! </details>
mod limits;
mod node;

pub mod flex;

pub use limits::Limits;
pub use node::Node;

use crate::{Length, Padding, Point, Rectangle, Size, Vector};

/// [`Node`] 及其子节点的边界，使用绝对坐标。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The bounds of a [`Node`] and its children, using absolute coordinates.
///
/// </details>
#[derive(Debug, Clone, Copy)]
pub struct Layout<'a> {
    position: Point,
    node: &'a Node,
}

impl<'a> Layout<'a> {
    /// 为给定的 [`Node`] 在原点处创建一个新的 [`Layout`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Layout`] for the given [`Node`] at the origin.
    ///
    /// </details>
    pub fn new(node: &'a Node) -> Self {
        Self::with_offset(Vector::new(0.0, 0.0), node)
    }

    /// 为给定的 [`Node`] 创建一个新的 [`Layout`]，
    /// 并带有给定的相对于原点的偏移量。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Layout`] for the given [`Node`] with the provided offset
    /// from the origin.
    ///
    /// </details>
    pub fn with_offset(offset: Vector, node: &'a Node) -> Self {
        let bounds = node.bounds();

        Self {
            position: Point::new(bounds.x, bounds.y) + offset,
            node,
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

    /// 返回 [`Layout`] 的边界。
    ///
    /// 返回的 [`Rectangle`] 描述了某个 [`Node`] 的
    /// 位置和尺寸。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the bounds of the [`Layout`].
    ///
    /// The returned [`Rectangle`] describes the position and size of a
    /// [`Node`].
    ///
    /// </details>
    pub fn bounds(&self) -> Rectangle {
        let bounds = self.node.bounds();

        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: bounds.width,
            height: bounds.height,
        }
    }

    /// 返回一个遍历此 [`Layout`] 的子级的迭代器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns an iterator over the children of this [`Layout`].
    ///
    /// </details>
    pub fn children(
        self,
    ) -> impl DoubleEndedIterator<Item = Layout<'a>> + ExactSizeIterator {
        self.node.children().iter().map(move |node| {
            Layout::with_offset(
                Vector::new(self.position.x, self.position.y),
                node,
            )
        })
    }

    /// 返回给定索引处子级的 [`Layout`]。
    ///
    /// 如果你为了分层而需要不按顺序访问子级，
    /// 这会很有用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Layout`] of the child at the given index.
    ///
    /// This can be useful if you ever need to access children out of order
    /// for layering purposes.
    ///
    /// </details>
    ///
    /// # Panics
    /// 如果索引越界，会发生 panic。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Panics if index is out of bounds.
    ///
    /// </details>
    pub fn child(self, index: usize) -> Layout<'a> {
        let node = &self.node.children()[index];

        Layout::with_offset(Vector::new(self.position.x, self.position.y), node)
    }
}

/// 生成一个 [`Node`]，它有两个紧挨着排列的子节点。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Produces a [`Node`] with two children nodes one right next to each other.
///
/// </details>
pub fn next_to_each_other(
    limits: &Limits,
    spacing: f32,
    left: impl FnOnce(&Limits) -> Node,
    right: impl FnOnce(&Limits) -> Node,
) -> Node {
    let left_node = left(limits);
    let left_size = left_node.size();

    let right_limits = limits.shrink(Size::new(left_size.width + spacing, 0.0));

    let right_node = right(&right_limits);
    let right_size = right_node.size();

    let (left_y, right_y) = if left_size.height > right_size.height {
        (0.0, (left_size.height - right_size.height) / 2.0)
    } else {
        ((right_size.height - left_size.height) / 2.0, 0.0)
    };

    Node::with_children(
        Size::new(
            left_size.width + spacing + right_size.width,
            left_size.height.max(right_size.height),
        ),
        vec![
            left_node.move_to(Point::new(0.0, left_y)),
            right_node.move_to(Point::new(left_size.width + spacing, right_y)),
        ],
    )
}

/// 给定宽度和高度要求，且没有固有尺寸时，
/// 计算出符合 [`Limits`] 的 [`Node`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Computes the resulting [`Node`] that fits the [`Limits`] given
/// some width and height requirements and no intrinsic size.
///
/// </details>
pub fn atomic(
    limits: &Limits,
    width: impl Into<Length>,
    height: impl Into<Length>,
) -> Node {
    let width = width.into();
    let height = height.into();

    Node::new(limits.resolve(width, height, Size::ZERO))
}

/// 给定宽度和高度要求，以及一个在给定 [`Limits`] 内
/// 产生固有 [`Size`] 的闭包，
/// 计算出符合 [`Limits`] 的 [`Node`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Computes the resulting [`Node`] that fits the [`Limits`] given
/// some width and height requirements and a closure that produces
/// the intrinsic [`Size`] inside the given [`Limits`].
///
/// </details>
pub fn sized(
    limits: &Limits,
    width: impl Into<Length>,
    height: impl Into<Length>,
    f: impl FnOnce(&Limits) -> Size,
) -> Node {
    let width = width.into();
    let height = height.into();

    let limits = limits.width(width).height(height);
    let intrinsic_size = f(&limits);

    Node::new(limits.resolve(width, height, intrinsic_size))
}

/// 给定宽度和高度要求，以及一个在给定 [`Limits`] 内
/// 产生内容 [`Node`] 的闭包，
/// 计算出符合 [`Limits`] 的 [`Node`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Computes the resulting [`Node`] that fits the [`Limits`] given
/// some width and height requirements and a closure that produces
/// the content [`Node`] inside the given [`Limits`].
///
/// </details>
pub fn contained(
    limits: &Limits,
    width: impl Into<Length>,
    height: impl Into<Length>,
    f: impl FnOnce(&Limits) -> Node,
) -> Node {
    let width = width.into();
    let height = height.into();

    let limits = limits.width(width).height(height);
    let content = f(&limits);

    Node::with_children(
        limits.resolve(width, height, content.size()),
        vec![content],
    )
}

/// 给定宽度、高度和 [`Padding`] 要求，以及一个在给定 [`Limits`] 内
/// 产生内容 [`Node`] 的闭包，
/// 计算出符合 [`Limits`] 的 [`Node`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Computes the [`Node`] that fits the [`Limits`] given some width, height, and
/// [`Padding`] requirements and a closure that produces the content [`Node`]
/// inside the given [`Limits`].
///
/// </details>
pub fn padded(
    limits: &Limits,
    width: impl Into<Length>,
    height: impl Into<Length>,
    padding: impl Into<Padding>,
    layout: impl FnOnce(&Limits) -> Node,
) -> Node {
    positioned(limits, width, height, padding, layout, |content, _| content)
}

/// 计算一个 [`padded`] 的 [`Node`]，并附带一个定位步骤。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Computes a [`padded`] [`Node`] with a positioning step.
///
/// </details>
pub fn positioned(
    limits: &Limits,
    width: impl Into<Length>,
    height: impl Into<Length>,
    padding: impl Into<Padding>,
    layout: impl FnOnce(&Limits) -> Node,
    position: impl FnOnce(Node, Size) -> Node,
) -> Node {
    let width = width.into();
    let height = height.into();
    let padding = padding.into();

    let limits = limits.width(width).height(height);
    let content = layout(&limits.shrink(padding));
    let padding = padding.fit(content.size(), limits.max());

    let size = limits
        .shrink(padding)
        .resolve(width, height, content.size());

    Node::with_children(
        size.expand(padding),
        vec![position(content.move_to((padding.left, padding.top)), size)],
    )
}
