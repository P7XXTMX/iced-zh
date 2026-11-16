//! 在元素之间添加显式的间距。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Add some explicit spacing between elements.
//!
//! </details>
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::widget::{Meta, Tree};
use crate::core::{Layout, Length, Rectangle, Size, Widget};

/// 创建一个新的 [`Space`] 部件，填满可用的
/// 水平空间。
///
/// 可以用来分隔 [`Row`](crate::Row) 中的部件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Space`] widget that fills the available
/// horizontal space.
///
/// This can be useful to separate widgets in a [`Row`](crate::Row).
///
/// </details>
pub fn horizontal() -> Space {
    Space::new().width(Length::Fill)
}

/// 创建一个新的 [`Space`] 部件，填满可用的
/// 垂直空间。
///
/// 可以用来分隔 [`Column`](crate::Column) 中的部件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Space`] widget that fills the available
/// vertical space.
///
/// This can be useful to separate widgets in a [`Column`](crate::Column).
///
/// </details>
pub fn vertical() -> Space {
    Space::new().height(Length::Fill)
}

/// 一定大小的空白空间。
///
/// 如果你想用空白填充一些空间，它会很有用。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An amount of empty space.
///
/// It can be useful if you want to fill some space with nothing.
///
/// </details>
#[derive(Debug)]
pub struct Space {
    width: Length,
    height: Length,
}

impl Space {
    /// 创建一个没有大小的空白 [`Space`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates some empty [`Space`] with no size.
    ///
    /// </details>
    pub fn new() -> Self {
        Space {
            width: Length::Fit,
            height: Length::Fit,
        }
    }

    /// 设置 [`Space`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Space`].
    ///
    /// </details>
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// 设置 [`Space`] 的高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the height of the [`Space`].
    ///
    /// </details>
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

impl Default for Space {
    fn default() -> Self {
        Space::new()
    }
}

impl Meta for Space {}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Space {
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&mut self, tree: &mut Tree, _renderer: &Renderer, limits: &layout::Limits) {
        tree.size = layout::atomic(limits, self.width, self.height);
    }

    fn draw(
        &self,
        _tree: &Tree,
        _renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        _layout: Layout,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
    }
}
