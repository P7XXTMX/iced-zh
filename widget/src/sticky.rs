//! 把元素粘在视口上！
//!
//! [`Sticky`] 会在其内容本该离开视野时让它们保持可见，
//! 例如当它们被滚动出
//! [`Scrollable`](crate::Scrollable) 时。只要内容的任意一条边
//! 超出可见边界，它们就会被移到可见边界内的一个 [`overlay`] 中，
//! 从而停留在视口的边缘。
//!
//! 在被粘住期间，内容保持原有的布局：它们
//! 只是浮动到可见边界内的一个新位置。
//!
//! 当内容的边到达其父级的边时，它们会保持
//! 附着在父级的边上：保留原有的边界，并改为被裁剪到
//! 可见边界。
//!
//! 一旦内容的父级离开了可见边界，它们就会
//! 停止浮动并随父级一起滚动。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Stick elements to the viewport!
//!
//! [`Sticky`] keeps its contents visible when they would otherwise go out of
//! view, for example when they are scrolled out of a
//! [`Scrollable`](crate::Scrollable). As soon as any of the contents' edges
//! goes out of the visible bounds, they are moved into an [`overlay`] inside
//! the visible bounds, so that they stay at the edge of the viewport.
//!
//! While they are stuck, the contents keep their original layout: they
//! simply float in a new position, inside the visible bounds.
//!
//! When the edges of the contents reach the edges of their parent, they stay
//! attached to them: they keep their original bounds, and are clipped to the
//! visible bounds instead.
//!
//! Once the parent of the contents has gone out of the visible bounds, they
//! stop floating and scroll with it.
//!
//! </details>
//!
//! # 示例（Example）
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; pub use iced_widget::core::Length::Fill; }
//! # use iced::widget::Widget;
//! # pub type State = ();
//! use iced::widget::{column, container, scrollable, sticky, space};
//! use iced::Fill;
//!
//! enum Message {
//!     // ...
//! }
//!
//! fn view(state: &State) -> impl Widget<Message> {
//!     scrollable(column![
//!         sticky(container("I always stay in view!").width(Fill).padding(10)),
//!         space().height(3000),
//!     ])
//! }
//! ```
use crate::core;
use crate::core::layout;
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::widget;
use crate::core::window;
use crate::core::{Event, Layout, Length, Point, Rectangle, Shell, Size, Vector, Widget};

/// 让其内容保持在视野内的部件。
///
/// 当 [`Sticky`] 部件的内容本该离开视野时（例如
/// 当它们被滚动出 [`Scrollable`](crate::Scrollable) 时），
/// 它们会被移到一个 [`overlay`] 中，该覆盖层位于父级的
/// [`layout.bounds`](Layout::bounds) 与视口的交集之内，从而
/// 在视口的边缘保持可见。
///
/// 只要内容的任意一条边超出可见边界，它们就会开始浮动，
/// 并且始终被限制在可见边界内，
/// 从而停留在其中。
///
/// 在被粘住期间，内容保持原有的布局：它们
/// 只是浮动到一个新位置。
///
/// 当内容的边到达其父级的边时，它们会保持
/// 附着在父级的边上：保留原有的边界，并改为被裁剪到
/// 可见边界。
///
/// 一旦内容的父级离开了可见边界，它们就会
/// 停止浮动并随父级一起滚动。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A widget that keeps its contents in view.
///
/// When the contents of a [`Sticky`] widget would go out of view (for
/// example, when they are scrolled out of a [`Scrollable`](crate::Scrollable)),
/// they are moved into an [`overlay`] inside the intersection of the parent's
/// [`layout.bounds`](Layout::bounds) and the viewport, so that they stay
/// visible at the edge of the viewport.
///
/// The contents start floating as soon as any of their edges goes out of the
/// visible bounds, and they are always clamped to the visible bounds, so
/// that they stay inside them.
///
/// While they are stuck, the contents keep their original layout: they
/// simply float in a new position.
///
/// When the edges of the contents reach the edges of their parent, they stay
/// attached to them: they keep their original bounds, and are clipped to the
/// visible bounds instead.
///
/// Once the parent of the contents has gone out of the visible bounds, they
/// stop floating and scroll with it.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; pub use iced_widget::core::Length::Fill; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// use iced::widget::{column, container, scrollable, sticky, space};
/// use iced::Fill;
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     scrollable(column![
///         sticky(container("I always stay in view!").width(Fill).padding(10)),
///         space().height(3000),
///     ])
/// }
/// ```
pub struct Sticky<W> {
    content: W,
}

impl<W> Sticky<W> {
    /// 使用给定的内容创建一个新的 [`Sticky`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Sticky`] with the given content.
    ///
    /// </details>
    pub fn new(content: W) -> Self {
        Self { content }
    }
}

struct State {
    is_stuck: bool,
}

impl<W> widget::Meta for Sticky<W> {}

impl<W, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Sticky<W>
where
    Renderer: core::Renderer,
    W: Widget<Message, Theme, Renderer>,
{
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(State { is_stuck: false })
    }

    fn diff(&mut self, tree: &mut widget::Tree) {
        tree.diff_children(&mut [&mut self.content]);
    }

    fn size(&self) -> Size<Length> {
        self.content.size()
    }

    fn layout(&mut self, tree: &mut widget::Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.content.layout(&mut tree.children[0], renderer, limits);

        tree.size = tree.children[0].size;
    }

    fn operate(
        &mut self,
        tree: &mut widget::Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        let state = tree.state.downcast_ref::<State>();

        if state.is_stuck {
            return;
        }

        self.content
            .operate(&mut tree.children[0], layout, viewport, renderer, operation);
    }

    fn update(
        &mut self,
        tree: &mut widget::Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();

        if let Event::Window(window::Event::RedrawRequested(_)) = event {
            let is_stuck = if let Some(parent) = layout.parent()
                && parent.intersects(viewport)
                && !layout.bounds().is_within(viewport)
            {
                true
            } else {
                false
            };

            if is_stuck != state.is_stuck {
                state.is_stuck = is_stuck;
                shell.invalidate_overlay();
                return; // Avoid propagating a stale redraw（避免传播过期的重绘）
            }
        }

        if state.is_stuck {
            return;
        }

        self.content.update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &widget::Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();

        if state.is_stuck {
            return mouse::Interaction::None;
        }

        self.content
            .mouse_interaction(&tree.children[0], layout, cursor, viewport, renderer)
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();

        if state.is_stuck {
            return;
        }

        self.content.draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut widget::Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Message, Theme, Renderer>> {
        let bounds = layout.bounds();
        let parent = layout.parent();
        let state = tree.state.downcast_ref::<State>();

        if let Some(parent) = parent
            && state.is_stuck
        {
            let viewport = *viewport + translation;
            let bounds = bounds + translation;
            let parent = parent + translation;

            let position = Point::new(
                stuck_axis(
                    bounds.x,
                    bounds.width,
                    viewport.x,
                    viewport.width,
                    (parent.x, parent.x + parent.width),
                ),
                stuck_axis(
                    bounds.y,
                    bounds.height,
                    viewport.y,
                    viewport.height,
                    (parent.y, parent.y + parent.height),
                ),
            );

            vec![overlay::Element::new(Box::new(Overlay {
                content: &mut self.content,
                tree: &mut tree.children[0],
                layout: layout.move_to(position),
                viewport,
                window,
            }))]
        } else {
            self.content.overlay(
                &mut tree.children[0],
                layout,
                renderer,
                viewport,
                translation,
                window,
            )
        }
    }
}

/// 计算内容沿某个轴的位置，使其粘在
/// 可见边界和父级边界之内。
///
/// `position` 和 `size` 是内容沿该轴的自然位置和大小，
/// `viewport` 和 `viewport_size` 描述可见
/// 边界，`parent` 描述父级的边界。
///
/// 返回的位置让内容同时位于可见边界
/// 和父级边界之内。如果做不到这一点（因为
/// 内容比两者的交集更大），内容会改为
/// 附着在父级的边上，从而保留
/// 原有的边界。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Computes the position of the contents along one axis, stuck inside the
/// visible bounds and the bounds of their parent.
///
/// `position` and `size` are the natural position and size of the contents
/// along the axis, `viewport` and `viewport_size` describe the visible
/// bounds, and `parent` describes the bounds of the parent.
///
/// The returned position keeps the contents inside both the visible bounds
/// and the bounds of the parent. When that is not possible, because the
/// contents are larger than their intersection, the contents are kept
/// attached to the edge of the parent instead, so that they keep their
/// original bounds.
///
/// </details>
fn stuck_axis(
    position: f32,
    size: f32,
    viewport: f32,
    viewport_size: f32,
    parent: (f32, f32),
) -> f32 {
    let viewport_max = viewport + viewport_size - size;
    let (parent_min, parent_max) = parent;
    let parent_max = parent_max - size;

    let min = viewport.max(parent_min);
    let max = viewport_max.min(parent_max);

    if min <= max {
        position.clamp(min, max)
    } else if parent_max < viewport {
        // The contents are attached to the upper edge of the parent
        // 内容附着在父级的上边缘
        parent_max
    } else {
        // The contents are attached to the lower edge of the parent
        // 内容附着在父级的下边缘
        parent_min
    }
}

struct Overlay<'b, W> {
    content: &'b mut W,
    tree: &'b mut widget::Tree,
    layout: Layout,
    viewport: Rectangle,
    window: Size,
}

impl<W> Overlay<'_, W> {
    fn bounds(&self) -> Rectangle {
        self.viewport
            .intersection(&self.layout.bounds())
            .unwrap_or(self.layout.bounds())
    }
}

impl<W, Message, Theme, Renderer> core::Overlay<Message, Theme, Renderer> for Overlay<'_, W>
where
    Renderer: core::Renderer,
    W: Widget<Message, Theme, Renderer>,
{
    fn operate(&mut self, renderer: &Renderer, operation: &mut dyn widget::Operation) {
        let bounds = self.bounds();

        self.content
            .operate(self.tree, self.layout, &bounds, renderer, operation);
    }

    fn update(
        &mut self,
        event: &Event,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
    ) {
        let bounds = self.bounds();

        self.content.update(
            self.tree,
            event,
            self.layout,
            cursor,
            renderer,
            shell,
            &bounds,
        );
    }

    fn mouse_interaction(&self, cursor: mouse::Cursor, renderer: &Renderer) -> mouse::Interaction {
        let bounds = self.bounds();

        let interaction =
            self.content
                .mouse_interaction(self.tree, self.layout, cursor, &bounds, renderer);

        if interaction == mouse::Interaction::None && cursor.is_over(bounds) {
            mouse::Interaction::Idle
        } else {
            interaction
        }
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        cursor: mouse::Cursor,
    ) {
        let bounds = self.bounds();

        renderer.with_layer(bounds, |renderer| {
            self.content.draw(
                self.tree,
                renderer,
                theme,
                style,
                self.layout,
                cursor,
                &bounds,
            );
        });
    }

    fn overlay<'c>(
        &'c mut self,
        renderer: &Renderer,
    ) -> Vec<overlay::Element<'c, Message, Theme, Renderer>> {
        self.content.overlay(
            self.tree,
            self.layout,
            renderer,
            &self.viewport,
            Vector::ZERO,
            self.window,
        )
    }
}
