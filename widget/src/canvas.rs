//! 画布可以用来绘制可交互的 2D 图形。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Canvases can be leveraged to draw interactive 2D graphics.
//!
//! </details>
//!
//! # 示例：绘制一个简单的圆（Example: Drawing a Simple Circle）
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
//! # pub type State = ();
//! # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
//! #
//! use iced::mouse;
//! use iced::widget::canvas;
//! use iced::{Color, Rectangle, Renderer, Theme};
//!
//! // First, we define the data we need for drawing
//! // 首先，定义绘制所需的数据
//! #[derive(Debug)]
//! struct Circle {
//!     radius: f32,
//! }
//!
//! // Then, we implement the `Program` trait
//! // 然后，实现 `Program`特质
//! impl<Message> canvas::Program<Message> for Circle {
//!     // No internal state
//!     // 没有内部状态
//!     type State = ();
//!
//!     fn draw(
//!         &self,
//!         _state: &(),
//!         renderer: &Renderer,
//!         _theme: &Theme,
//!         bounds: Rectangle,
//!         _cursor: mouse::Cursor
//!     ) -> Vec<canvas::Geometry> {
//!         // We prepare a new `Frame`
//!         // 准备一个新的 `Frame`
//!         let mut frame = canvas::Frame::new(renderer, bounds.size());
//!
//!         // We create a `Path` representing a simple circle
//!         // 创建一个表示简单圆形的 `Path`
//!         let circle = canvas::Path::circle(frame.center(), self.radius);
//!
//!         // And fill it with some color
//!         // 并用某种颜色填充它
//!         frame.fill(&circle, Color::BLACK);
//!
//!         // Then, we produce the geometry
//!         // 然后，生成几何图形
//!         vec![frame.into_geometry()]
//!     }
//! }
//!
//! // Finally, we simply use our `Circle` to create the `Canvas`!
//! // 最后，只需用我们的 `Circle` 创建 `Canvas`！
//! fn view<'a, Message: 'a>(_state: &'a State) -> Element<'a, Message> {
//!     canvas(Circle { radius: 50.0 }).into()
//! }
//! ```
mod program;

pub use program::Program;

pub use crate::Action;
pub use crate::core::event::Event;
pub use crate::graphics::cache::Group;
pub use crate::graphics::geometry::{
    Fill, Gradient, Image, LineCap, LineDash, LineJoin, Path, Stroke, Style,
    Text, fill, gradient, path, stroke,
};

use crate::core::event;
use crate::core::layout::{self, Layout};
use crate::core::mouse;
use crate::core::renderer;
use crate::core::widget::tree::{self, Tree};
use crate::core::window;
use crate::core::{
    Clipboard, Element, Length, Rectangle, Shell, Size, Vector, Widget,
};
use crate::graphics::geometry;

use std::marker::PhantomData;

/// 简单的缓存，存储已生成的 [`Geometry`] 以避免重复计算。
///
/// 除非所在图层的尺寸发生变化或被显式清除，
/// 否则 [`Cache`] 不会重绘它的几何图形。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A simple cache that stores generated [`Geometry`] to avoid recomputation.
///
/// A [`Cache`] will not redraw its geometry unless the dimensions of its layer
/// change or it is explicitly cleared.
///
/// </details>
pub type Cache<Renderer = crate::Renderer> = geometry::Cache<Renderer>;

/// 渲染器所支持的几何图形。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The geometry supported by a renderer.
///
/// </details>
pub type Geometry<Renderer = crate::Renderer> =
    <Renderer as geometry::Renderer>::Geometry;

/// 渲染器所支持的帧。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The frame supported by a renderer.
///
/// </details>
pub type Frame<Renderer = crate::Renderer> = geometry::Frame<Renderer>;

/// 能够绘制 2D 图形的部件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A widget capable of drawing 2D graphics.
///
/// </details>
///
/// # 示例：绘制一个简单的圆（Example: Drawing a Simple Circle）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// #
/// use iced::mouse;
/// use iced::widget::canvas;
/// use iced::{Color, Rectangle, Renderer, Theme};
///
/// // First, we define the data we need for drawing
/// // 首先，定义绘制所需的数据
/// #[derive(Debug)]
/// struct Circle {
///     radius: f32,
/// }
///
/// // Then, we implement the `Program` trait
/// // 然后，实现 `Program`特质
/// impl<Message> canvas::Program<Message> for Circle {
///     // No internal state
///     // 没有内部状态
///     type State = ();
///
///     fn draw(
///         &self,
///         _state: &(),
///         renderer: &Renderer,
///         _theme: &Theme,
///         bounds: Rectangle,
///         _cursor: mouse::Cursor
///     ) -> Vec<canvas::Geometry> {
///         // We prepare a new `Frame`
///         // 准备一个新的 `Frame`
///         let mut frame = canvas::Frame::new(renderer, bounds.size());
///
///         // We create a `Path` representing a simple circle
///         // 创建一个表示简单圆形的 `Path`
///         let circle = canvas::Path::circle(frame.center(), self.radius);
///
///         // And fill it with some color
///         // 并用某种颜色填充它
///         frame.fill(&circle, Color::BLACK);
///
///         // Then, we produce the geometry
///         // 然后，生成几何图形
///         vec![frame.into_geometry()]
///     }
/// }
///
/// // Finally, we simply use our `Circle` to create the `Canvas`!
/// // 最后，只需用我们的 `Circle` 创建 `Canvas`！
/// fn view<'a, Message: 'a>(_state: &'a State) -> Element<'a, Message> {
///     canvas(Circle { radius: 50.0 }).into()
/// }
/// ```
#[derive(Debug)]
pub struct Canvas<P, Message, Theme = crate::Theme, Renderer = crate::Renderer>
where
    Renderer: geometry::Renderer,
    P: Program<Message, Theme, Renderer>,
{
    width: Length,
    height: Length,
    program: P,
    message_: PhantomData<Message>,
    theme_: PhantomData<Theme>,
    renderer_: PhantomData<Renderer>,
    last_mouse_interaction: Option<mouse::Interaction>,
}

impl<P, Message, Theme, Renderer> Canvas<P, Message, Theme, Renderer>
where
    P: Program<Message, Theme, Renderer>,
    Renderer: geometry::Renderer,
{
    const DEFAULT_SIZE: f32 = 100.0;

    /// 创建一个新的 [`Canvas`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Canvas`].
    ///
    /// </details>
    pub fn new(program: P) -> Self {
        Canvas {
            width: Length::Fixed(Self::DEFAULT_SIZE),
            height: Length::Fixed(Self::DEFAULT_SIZE),
            program,
            message_: PhantomData,
            theme_: PhantomData,
            renderer_: PhantomData,
            last_mouse_interaction: None,
        }
    }

    /// 设置 [`Canvas`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Canvas`].
    ///
    /// </details>
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// 设置 [`Canvas`] 的高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the height of the [`Canvas`].
    ///
    /// </details>
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

impl<P, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Canvas<P, Message, Theme, Renderer>
where
    Renderer: geometry::Renderer,
    P: Program<Message, Theme, Renderer>,
{
    fn tag(&self) -> tree::Tag {
        struct Tag<T>(T);
        tree::Tag::of::<Tag<P::State>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(P::State::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.width, self.height)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        let state = tree.state.downcast_mut::<P::State>();
        let is_redraw_request = matches!(
            event,
            Event::Window(window::Event::RedrawRequested(_now)),
        );

        if let Some(action) = self.program.update(state, event, bounds, cursor)
        {
            let (message, redraw_request, event_status) = action.into_inner();

            shell.request_redraw_at(redraw_request);

            if let Some(message) = message {
                shell.publish(message);
            }

            if event_status == event::Status::Captured {
                shell.capture_event();
            }
        }

        if shell.redraw_request() != window::RedrawRequest::NextFrame {
            let mouse_interaction = self
                .mouse_interaction(tree, layout, cursor, viewport, renderer);

            if is_redraw_request {
                self.last_mouse_interaction = Some(mouse_interaction);
            } else if self.last_mouse_interaction.is_some_and(
                |last_mouse_interaction| {
                    last_mouse_interaction != mouse_interaction
                },
            ) {
                shell.request_redraw();
            }
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let bounds = layout.bounds();
        let state = tree.state.downcast_ref::<P::State>();

        self.program.mouse_interaction(state, bounds, cursor)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        if bounds.width < 1.0 || bounds.height < 1.0 {
            return;
        }

        let state = tree.state.downcast_ref::<P::State>();

        renderer.with_translation(
            Vector::new(bounds.x, bounds.y),
            |renderer| {
                let layers =
                    self.program.draw(state, renderer, theme, bounds, cursor);

                for layer in layers {
                    renderer.draw_geometry(layer);
                }
            },
        );
    }
}

impl<'a, P, Message, Theme, Renderer> From<Canvas<P, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: 'a + geometry::Renderer,
    P: 'a + Program<Message, Theme, Renderer>,
{
    fn from(
        canvas: Canvas<P, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(canvas)
    }
}
