//! 在其他部件之上显示可交互的元素。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Display interactive elements on top of other widgets.
//!
//! </details>
mod element;
mod group;

pub use element::Element;
pub use group::Group;

use crate::mouse;
use crate::renderer;
use crate::widget;
use crate::widget::Tree;
use crate::{Event, Layout, Rectangle, Shell, Size, Vector, Widget};

/// 可以显示在其他部件之上的可交互组件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An interactive component that can be displayed on top of other widgets.
///
/// </details>
pub trait Overlay<Message, Theme, Renderer> {
    /// 使用关联的 `Renderer` 绘制 [`Overlay`]。
    ///
    /// 实现者应该自行调用
    /// [`Renderer::with_layer`](renderer::Renderer::with_layer)，
    /// 以便其内容被正确裁剪。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the [`Overlay`] using the associated `Renderer`.
    ///
    /// Implementors are expected to call
    /// [`Renderer::with_layer`](renderer::Renderer::with_layer) themselves,
    /// so that their contents are properly clipped.
    ///
    /// </details>
    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        cursor: mouse::Cursor,
    );

    /// 对 [`Overlay`] 应用一个 [`widget::Operation`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies a [`widget::Operation`] to the [`Overlay`].
    ///
    /// </details>
    fn operate(&mut self, _renderer: &Renderer, _operation: &mut dyn widget::Operation) {}

    /// 处理运行时的 [`Event`]。
    ///
    /// 默认情况下，它什么都不做。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Processes a runtime [`Event`].
    ///
    /// By default, it does nothing.
    ///
    /// </details>
    fn update(
        &mut self,
        _event: &Event,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
        _shell: &mut Shell<'_, Message>,
    ) {
    }

    /// 返回 [`Overlay`] 当前的 [`mouse::Interaction`]。
    ///
    /// 默认情况下，它返回 [`mouse::Interaction::None`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current [`mouse::Interaction`] of the [`Overlay`].
    ///
    /// By default, it returns [`mouse::Interaction::None`].
    ///
    /// </details>
    fn mouse_interaction(
        &self,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        mouse::Interaction::None
    }

    /// 返回 [`Overlay`] 的嵌套覆盖层。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the nested overlays of the [`Overlay`].
    ///
    /// </details>
    fn overlay<'a>(
        &'a mut self,
        _renderer: &Renderer,
    ) -> Vec<Element<'a, Message, Theme, Renderer>> {
        Vec::new()
    }

    /// 覆盖层的索引。
    ///
    /// 索引较高的覆盖层会渲染在
    /// 索引较低的覆盖层之上。
    ///
    /// 默认情况下，它返回 `1.0`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The index of the overlay.
    ///
    /// Overlays with a higher index will be rendered on top of overlays with
    /// a lower index.
    ///
    /// By default, it returns `1.0`.
    ///
    /// </details>
    fn index(&self) -> f32 {
        1.0
    }
}

/// 返回给定 [`Widget`] 子级的覆盖层。
///
/// 此方法通常只会被正在实现 [`Widget`] 特质的
/// 高级用户使用。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns the overlays of the given [`Widget`] children.
///
/// This method will generally only be used by advanced users that are
/// implementing the [`Widget`] trait.
///
/// </details>
pub fn from_children<'a, Message, Theme, Renderer>(
    children: &'a mut [impl Widget<Message, Theme, Renderer>],
    tree: &'a mut Tree,
    layout: Layout,
    renderer: &Renderer,
    viewport: &Rectangle,
    translation: Vector,
    window: Size,
) -> Vec<Element<'a, Message, Theme, Renderer>> {
    children
        .iter_mut()
        .zip(layout.iter_mut(&mut tree.children))
        .flat_map(|(child, (layout, state))| {
            child.overlay(state, layout, renderer, viewport, translation, window)
        })
        .collect()
}
