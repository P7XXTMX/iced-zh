//! 包装给定的部件，并捕获该部件边界内的所有鼠标按键按下事件，
//! 相当于让它变得“不透明”（_opaque_）。
//!
//! 这个部件用于标记 [`Stack`] 中的元素，或者覆盖层内部的元素
//! （例如 [`Popover`] 的弹出框），以免鼠标事件
//! 穿透到下方的元素。
//!
//! 默认只捕获鼠标按键的按下事件。使用
//! [`Opaque::capture_scroll`] 可以同时捕获滚轮事件，
//! 这样滚动就不会穿透到下方的元素。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Wraps the given widget and captures any mouse button presses inside the bounds of
//! the widget—effectively making it _opaque_.
//!
//! This widget is meant to be used to mark elements in a [`Stack`], or inside an
//! overlay—like the popup of a [`Popover`]—to avoid mouse events from passing
//! through to the elements below.
//!
//! By default, only mouse button presses are captured. Use
//! [`Opaque::capture_scroll`] to also capture scroll wheel events, so that
//! scrolling does not pass through to the elements below.
//!
//! </details>
//!
//! [`Stack`]: crate::Stack
//! [`Popover`]: crate::popover::Popover
//!
//! # 示例（Example）
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
//! # use iced::widget::Widget;
//! # pub type State = ();
//! use iced::widget::{button, opaque, stack, text};
//!
//! #[derive(Clone)]
//! enum Message {
//!     ButtonPressed,
//! }
//!
//! fn view(state: &State) -> impl Widget<Message> {
//!     stack![
//!         // This button lies below the opaque top layer: mouse events over
//!         // 这个按钮位于不透明的顶层之下：顶层上方的鼠标事件
//!         // the top layer do not pass through to it.
//!         // 不会穿透到它。
//!         button("Click me!").on_press(Message::ButtonPressed),
//!         opaque(text("I am opaque")),
//!     ]
//! }
//! ```
use crate::core::layout;
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::widget::tree::{self, Tree};
use crate::core::widget::{self, Operation, Widget};
use crate::core::{Element, Event, Layout, Length, Rectangle, Shell, Size, Vector};

/// 包装给定的部件，并捕获该部件边界内的所有鼠标按键按下事件，
/// 相当于让它变得“不透明”（_opaque_）。
///
/// 这个部件用于标记 [`Stack`] 中的元素，或者覆盖层内部的元素
/// （例如 [`Popover`] 的弹出框），以免鼠标事件
/// 穿透到下方的元素。
///
/// 默认只捕获鼠标按键的按下事件。使用
/// [`Self::capture_scroll`] 可以同时捕获滚轮事件，
/// 这样滚动就不会穿透到下方的元素。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Wraps the given widget and captures any mouse button presses inside the bounds of
/// the widget—effectively making it _opaque_.
///
/// This widget is meant to be used to mark elements in a [`Stack`], or inside an
/// overlay—like the popup of a [`Popover`]—to avoid mouse events from passing
/// through to the elements below.
///
/// By default, only mouse button presses are captured. Use
/// [`Self::capture_scroll`] to also capture scroll wheel events, so that
/// scrolling does not pass through to the elements below.
///
/// </details>
///
/// [`Stack`]: crate::Stack
/// [`Popover`]: crate::popover::Popover
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// use iced::widget::{button, opaque, stack, text};
///
/// #[derive(Clone)]
/// enum Message {
///     ButtonPressed,
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     stack![
///         // This button lies below the opaque top layer: mouse events over
///         // 这个按钮位于不透明的顶层之下：顶层上方的鼠标事件
///         // the top layer do not pass through to it.
///         // 不会穿透到它。
///         button("Click me!").on_press(Message::ButtonPressed),
///         opaque(text("I am opaque")),
///     ]
/// }
/// ```
pub struct Opaque<W> {
    content: W,
    capture_scroll: bool,
}

impl<W> Opaque<W> {
    /// 创建一个新的 [`Opaque`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Opaque`].
    ///
    /// </details>
    pub fn new(content: W) -> Self {
        Opaque {
            content,
            capture_scroll: false,
        }
    }

    /// 设置当光标位于 [`Opaque`] 的边界之上时，
    /// 是否捕获滚轮事件。
    ///
    /// 默认只捕获鼠标按键的按下事件。启用这个标志后
    /// 还会捕获滚轮事件，阻止它们穿透到
    /// 下方的元素。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets whether the scroll wheel should be captured when the cursor is
    /// over the bounds of the [`Opaque`].
    ///
    /// By default, only mouse button presses are captured. Enabling this flag
    /// also captures scroll wheel events, preventing them from passing through
    /// to the elements below.
    ///
    /// </details>
    pub fn capture_scroll(mut self, capture: bool) -> Self {
        self.capture_scroll = capture;
        self
    }

    /// 返回被包装的内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the wrapped content.
    ///
    /// </details>
    pub fn into_inner(self) -> W {
        self.content
    }
}

impl<W> widget::Meta for Opaque<W> {}

impl<W, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Opaque<W>
where
    W: Widget<Message, Theme, Renderer>,
{
    fn tag(&self) -> tree::Tag {
        self.content.tag()
    }

    fn state(&self) -> tree::State {
        self.content.state()
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.content));
    }

    fn size(&self) -> Size<Length> {
        self.content.size()
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.content.layout(&mut tree.children[0], renderer, limits);

        tree.size = tree.children[0].size;
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let (layout, tree) = layout.iter(&tree.children).next().unwrap();

        self.content
            .draw(tree, renderer, theme, style, layout, cursor, viewport);
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let (layout, tree) = layout.iter_mut(&mut tree.children).next().unwrap();

        self.content
            .operate(tree, layout, viewport, renderer, operation);
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let (layout, tree) = layout.iter_mut(&mut tree.children).next().unwrap();

        let is_mouse_press = matches!(event, Event::Mouse(mouse::Event::ButtonPressed(_)));
        let is_wheel_scroll = matches!(event, Event::Mouse(mouse::Event::WheelScrolled { .. }));

        self.content
            .update(tree, event, layout, cursor, renderer, shell, viewport);

        if (is_mouse_press || (self.capture_scroll && is_wheel_scroll))
            && cursor.is_over(layout.bounds())
        {
            shell.capture_event();
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let (layout, tree) = layout.iter(&tree.children).next().unwrap();

        let interaction = self
            .content
            .mouse_interaction(tree, layout, cursor, viewport, renderer);

        if interaction == mouse::Interaction::None && cursor.is_over(layout.bounds()) {
            mouse::Interaction::Idle
        } else {
            interaction
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Message, Theme, Renderer>> {
        let (layout, tree) = layout.iter_mut(&mut tree.children).next().unwrap();

        self.content
            .overlay(tree, layout, renderer, viewport, translation, window)
    }
}

impl<'a, W, Message, Theme, Renderer> From<Opaque<W>> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: 'a,
    W: Widget<Message, Theme, Renderer> + 'a,
{
    fn from(opaque: Opaque<W>) -> Element<'a, Message, Theme, Renderer> {
        opaque._boxed()
    }
}
