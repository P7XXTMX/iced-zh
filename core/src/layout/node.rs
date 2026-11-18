use crate::{Alignment, Padding, Point, Rectangle, Size, Vector};

/// 元素及其子元素的边界。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The bounds of an element and its children.
///
/// </details>
#[derive(Debug, Clone, Default)]
pub struct Node {
    bounds: Rectangle,
    children: Vec<Node>,
}

impl Node {
    /// 使用给定的 [`Size`] 创建一个新的 [`Node`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Node`] with the given [`Size`].
    ///
    /// </details>
    pub const fn new(size: Size) -> Self {
        Self::with_children(size, Vec::new())
    }

    /// 使用给定的 [`Size`] 和子节点创建一个新的 [`Node`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Node`] with the given [`Size`] and children.
    ///
    /// </details>
    pub const fn with_children(size: Size, children: Vec<Node>) -> Self {
        Node {
            bounds: Rectangle {
                x: 0.0,
                y: 0.0,
                width: size.width,
                height: size.height,
            },
            children,
        }
    }

    /// 创建一个新的 [`Node`]，用一定的 [`Padding`] 包裹单个子节点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Node`] that wraps a single child with some [`Padding`].
    ///
    /// </details>
    pub fn container(child: Self, padding: Padding) -> Self {
        Self::with_children(
            child.bounds.size().expand(padding),
            vec![child.move_to(Point::new(padding.left, padding.top))],
        )
    }

    /// 返回 [`Node`] 的 [`Size`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Size`] of the [`Node`].
    ///
    /// </details>
    pub fn size(&self) -> Size {
        Size::new(self.bounds.width, self.bounds.height)
    }

    /// 返回 [`Node`] 的边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the bounds of the [`Node`].
    ///
    /// </details>
    pub fn bounds(&self) -> Rectangle {
        self.bounds
    }

    /// 返回 [`Node`] 的子节点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the children of the [`Node`].
    ///
    /// </details>
    pub fn children(&self) -> &[Node] {
        &self.children
    }

    /// 在给定的空间内对齐 [`Node`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Aligns the [`Node`] in the given space.
    ///
    /// </details>
    pub fn align(
        mut self,
        align_x: Alignment,
        align_y: Alignment,
        space: Size,
    ) -> Self {
        self.align_mut(align_x, align_y, space);
        self
    }

    /// [`Self::align`] 的可变引用版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Mutable reference version of [`Self::align`].
    ///
    /// </details>
    pub fn align_mut(
        &mut self,
        align_x: Alignment,
        align_y: Alignment,
        space: Size,
    ) {
        match align_x {
            Alignment::Start => {}
            Alignment::Center => {
                self.bounds.x += (space.width - self.bounds.width) / 2.0;
            }
            Alignment::End => {
                self.bounds.x += space.width - self.bounds.width;
            }
        }

        match align_y {
            Alignment::Start => {}
            Alignment::Center => {
                self.bounds.y += (space.height - self.bounds.height) / 2.0;
            }
            Alignment::End => {
                self.bounds.y += space.height - self.bounds.height;
            }
        }
    }

    /// 把 [`Node`] 移动到给定的位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Moves the [`Node`] to the given position.
    ///
    /// </details>
    pub fn move_to(mut self, position: impl Into<Point>) -> Self {
        self.move_to_mut(position);
        self
    }

    /// [`Self::move_to`] 的可变引用版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Mutable reference version of [`Self::move_to`].
    ///
    /// </details>
    pub fn move_to_mut(&mut self, position: impl Into<Point>) {
        let position = position.into();

        self.bounds.x = position.x;
        self.bounds.y = position.y;
    }

    /// 按给定的平移量平移 [`Node`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Translates the [`Node`] by the given translation.
    ///
    /// </details>
    pub fn translate(mut self, translation: impl Into<Vector>) -> Self {
        self.translate_mut(translation);
        self
    }

    /// 按给定的平移量平移 [`Node`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Translates the [`Node`] by the given translation.
    ///
    /// </details>
    pub fn translate_mut(&mut self, translation: impl Into<Vector>) {
        self.bounds = self.bounds + translation.into();
    }
}
