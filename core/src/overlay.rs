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
mod nested;

pub use element::Element;
pub use group::Group;
pub use nested::Nested;

use crate::layout;
use crate::mouse;
use crate::renderer;
use crate::widget;
use crate::widget::Tree;
use crate::{Clipboard, Event, Layout, Rectangle, Shell, Size, Vector};

/// 可以显示在其他部件之上的可交互组件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An interactive component that can be displayed on top of other widgets.
///
/// </details>
pub trait Overlay<Message, Theme, Renderer>
where
    Renderer: crate::Renderer,
{
    /// 返回 [`Overlay`] 的布局 [`Node`]。
    ///
    /// 运行时用这个 [`Node`] 来计算用户界面的
    /// [`Layout`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the layout [`Node`] of the [`Overlay`].
    ///
    /// This [`Node`] is used by the runtime to compute the [`Layout`] of the
    /// user interface.
    ///
    /// </details>
    ///
    /// [`Node`]: layout::Node
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node;

    /// 使用关联的 `Renderer` 绘制 [`Overlay`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the [`Overlay`] using the associated `Renderer`.
    ///
    /// </details>
    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
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
    fn operate(
        &mut self,
        _layout: Layout<'_>,
        _renderer: &Renderer,
        _operation: &mut dyn widget::Operation,
    ) {
    }

    /// 处理运行时的 [`Event`]。
    ///
    /// 它接收：
    ///   * 一个描述用户交互的 [`Event`]
    ///   * 计算得到的 [`Overlay`] 的 [`Layout`]
    ///   * 当前的光标位置
    ///   * 一个可变的 `Message` 列表，让 [`Overlay`] 可以
    ///     根据用户交互产生新的消息。
    ///   * `Renderer`
    ///   * 一个 [`Clipboard`]（如果可用）
    ///
    /// 默认情况下，它什么都不做。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Processes a runtime [`Event`].
    ///
    /// It receives:
    ///   * an [`Event`] describing user interaction
    ///   * the computed [`Layout`] of the [`Overlay`]
    ///   * the current cursor position
    ///   * a mutable `Message` list, allowing the [`Overlay`] to produce
    ///     new messages based on user interaction.
    ///   * the `Renderer`
    ///   * a [`Clipboard`], if available
    ///
    /// By default, it does nothing.
    ///
    /// </details>
    fn update(
        &mut self,
        _event: &Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
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
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        mouse::Interaction::None
    }

    /// 返回 [`Overlay`] 的嵌套覆盖层（如果有）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the nested overlay of the [`Overlay`], if there is any.
    ///
    /// </details>
    fn overlay<'a>(
        &'a mut self,
        _layout: Layout<'a>,
        _renderer: &Renderer,
    ) -> Option<Element<'a, Message, Theme, Renderer>> {
        None
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

/// 返回由覆盖层 [`Element`] 子级组成的 [`Group`]。
///
/// 此方法通常只会被正在实现 [`Widget`](crate::Widget) 特质的
/// 高级用户使用。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns a [`Group`] of overlay [`Element`] children.
///
/// This method will generally only be used by advanced users that are
/// implementing the [`Widget`](crate::Widget) trait.
///
/// </details>
pub fn from_children<'a, Message, Theme, Renderer>(
    children: &'a mut [crate::Element<'_, Message, Theme, Renderer>],
    tree: &'a mut Tree,
    layout: Layout<'a>,
    renderer: &Renderer,
    viewport: &Rectangle,
    translation: Vector,
) -> Option<Element<'a, Message, Theme, Renderer>>
where
    Renderer: crate::Renderer,
{
    let children = children
        .iter_mut()
        .zip(&mut tree.children)
        .zip(layout.children())
        .filter_map(|((child, state), layout)| {
            child.as_widget_mut().overlay(
                state,
                layout,
                renderer,
                viewport,
                translation,
            )
        })
        .collect::<Vec<_>>();

    (!children.is_empty()).then(|| Group::with_children(children).overlay())
}
