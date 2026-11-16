//! 弹出层是出现在某个元素之上的一块浮动内容。
//!
//! 与 [`tooltip`] 不同，弹出层不会在悬停时显示，也不
//! 控制自身的可见性。相反，`popover` 参数是一个
//! `Option`：`Some` 显示覆盖层（打开），`None` 隐藏它（关闭）。
//! 基底元素始终存在。
//!
//! 当用户在弹出层的边界之外点击时，弹出层会通过
//! 它的 `on_close` 处理函数通知应用程序，应用程序通常
//! 就是据此决定把 `popover` 参数设为 `None`（也就是
//! 关闭它）。
//!
//! [`popover()`] 辅助函数会用 [`opaque()`] 包装弹出框，因此
//! 弹出框边界内的鼠标按键按下事件会被捕获，不会穿透到
//! 下方的元素。使用 [`Popover::passthrough`] 可以让它们穿透
//! 到基底层，使用 [`Popover::capture_scroll`] 可以同时捕获
//! 弹出框上的滚轮事件。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! A popover is a floating piece of content that appears over some element.
//!
//! Unlike a [`tooltip`], a popover is not shown on hover, and it does not
//! control its own visibility. Instead, the `popover` argument is an
//! `Option`: `Some` displays the overlay (open), and `None` hides it (closed).
//! The base is always present.
//!
//! When the user clicks outside of the popover's bounds, the popover notifies
//! the application through its `on_close` handler, which is typically how the
//! application decides to set the `popover` argument to `None` (i.e. to close
//! it).
//!
//! The [`popover()`] helper wraps the popup in [`opaque()`], so mouse button
//! presses inside the popup's bounds are captured and do not pass through to
//! the elements below. Use [`Popover::passthrough`] to let them fall through
//! to the base layer, and [`Popover::capture_scroll`] to also capture scroll
//! wheel events over the popup.
//!
//! </details>
//!
//! [`tooltip`]: crate::tooltip::Tooltip
//! [`popover()`]: crate::popover()
//! [`opaque()`]: crate::opaque()
//!
//! # 示例（Example）
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
//! # use iced::widget::Widget;
//! # pub struct State {
//! #     is_open: bool,
//! # }
//! use iced::widget::{button, container, popover, text};
//!
//! #[derive(Clone)]
//! enum Message {
//!     Close,
//! }
//!
//! fn view(state: &State) -> impl Widget<Message> {
//!     // The base is always present. The `popover` argument is `Some` when the
//!     // 基底元素始终存在。弹出层打开时，`popover` 参数为 `Some`；
//!     // popover is open and `None` when it is closed.
//!     // 关闭时为 `None`。
//!     popover(
//!         button(text("Click me!")).on_press(Message::Close),
//!         state.is_open.then(|| {
//!             container(text("This is the popover contents!")).padding(10)
//!         }),
//!     )
//!     .position(popover::Position::Bottom)
//!     .on_close(Message::Close)
//! }
//! ```
use crate::Opaque;
use crate::core::layout::{self, Layout};
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::touch;
use crate::core::widget::{self, Widget};
use crate::core::{Event, Length, Pixels, Point, Rectangle, Shell, Size, Vector};

/// 出现在另一个元素之上的一块浮动内容。
///
/// 弹出层不控制自身的可见性。相反，`popover`
/// 参数是一个 `Option`：`Some` 显示覆盖层（打开），`None`
/// 隐藏它（关闭）。基底元素始终存在。
///
/// 当用户在弹出层的边界之外点击时，弹出层会通过
/// 它的 `on_close` 处理函数通知应用程序。
///
/// [`popover()`] 辅助函数会用 [`opaque()`] 包装弹出框，因此
/// 弹出框边界内的鼠标按键按下事件会被捕获，不会穿透到
/// 下方的元素。使用 [`Self::passthrough`] 可以让它们穿透到
/// 基底层，使用 [`Self::capture_scroll`] 可以同时捕获
/// 弹出框上的滚轮事件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A floating piece of content that appears over another element.
///
/// The popover does not control its own visibility. Instead, the `popover`
/// argument is an `Option`: `Some` displays the overlay (open), and `None`
/// hides it (closed). The base is always present.
///
/// When the user clicks outside of the popover's bounds, the popover notifies
/// the application through its `on_close` handler.
///
/// The [`popover()`] helper wraps the popup in [`opaque()`], so mouse button
/// presses inside the popup's bounds are captured and do not pass through to
/// the elements below. Use [`Self::passthrough`] to let them fall through to
/// the base layer, and [`Self::capture_scroll`] to also capture scroll wheel
/// events over the popup.
///
/// </details>
///
/// [`popover()`]: crate::popover()
/// [`opaque()`]: crate::opaque()
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// # pub struct State {
/// #     is_open: bool,
/// # }
/// use iced::widget::{button, container, popover, text};
///
/// #[derive(Clone)]
/// enum Message {
///     Close,
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     // The base is always present. The `popover` argument is `Some` when the
///     // 基底元素始终存在。弹出层打开时，`popover` 参数为 `Some`；
///     // popover is open and `None` when it is closed.
///     // 关闭时为 `None`。
///     popover(
///         button(text("Click me!")).on_press(Message::Close),
///         state.is_open.then(|| {
///             container(text("This is the popover contents!")).padding(10)
///         }),
///     )
///     .position(popover::Position::Bottom)
///     .on_close(Message::Close)
/// }
/// ```
pub struct Popover<W, V, Message> {
    content: W,
    popup: Option<V>,
    position: Position,
    gap: f32,
    snap_within_viewport: bool,
    on_close: Option<Message>,
}

impl<W, V, Message> Popover<W, V, Message> {
    /// 创建一个新的 [`Popover`]。
    ///
    /// 它需要：
    ///   * 弹出层所锚定的 `content` 部件（基底元素），以及
    ///   * 可选的、要显示的 `popup` 部件：弹出层打开时为 `Some`，
    ///     关闭时为 `None`。
    ///
    /// 基底元素始终存在；是否显示覆盖层由 `popup` 参数
    /// 控制。默认情况下，[`Popover`] 的位置是
    /// [`Position::Auto`]；使用 [`Self::position`] 方法可以设置
    /// 特定的位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Popover`].
    ///
    /// It expects:
    ///   * the `content` widget that the popover is anchored to (the base), and
    ///   * the optional `popup` widget to display: `Some` when the popover is
    ///     open and `None` when it is closed.
    ///
    /// The base is always present; it is the `popup` argument that controls
    /// whether the overlay is displayed. By default, the [`Popover`] is
    /// positioned [`Position::Auto`]; use the [`Self::position`] method to set
    /// a specific position.
    ///
    /// </details>
    pub fn new(content: W, popup: Option<V>) -> Self {
        Popover {
            content,
            popup,
            position: Position::default(),
            gap: 0.0,
            snap_within_viewport: true,
            on_close: None,
        }
    }

    /// 设置用户在 [`Popover`] 的边界之外点击时
    /// 产生的消息。
    ///
    /// 应用程序通常就是据此决定关闭弹出层的，
    /// 做法是把 `popover` 参数设为 `None`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the message that will be produced when the user clicks outside of
    /// the [`Popover`]'s bounds.
    ///
    /// This is typically how the application decides to close the popover, by
    /// setting the `popover` argument to `None`.
    ///
    /// </details>
    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
        self
    }

    /// 设置 [`Popover`] 的 [`Position`]。
    ///
    /// 默认情况下，[`Popover`] 的位置是 [`Position::Auto`]，
    /// 它会把弹出层放在基底元素可用空间最大的一侧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Position`] of the [`Popover`].
    ///
    /// By default, the [`Popover`] is positioned [`Position::Auto`], which
    /// places it on the side of the base with the most available space.
    ///
    /// </details>
    pub fn position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    /// 设置内容与其 [`Popover`] 之间的间隙。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the gap between the content and its [`Popover`].
    ///
    /// </details>
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into().0;
        self
    }

    /// 设置 [`Popover`] 是否吸附在视口之内。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets whether the [`Popover`] is snapped within the viewport.
    ///
    /// </details>
    pub fn snap_within_viewport(mut self, snap: bool) -> Self {
        self.snap_within_viewport = snap;
        self
    }
}

impl<W, V, Message> Popover<W, Opaque<V>, Message> {
    /// 让 [`Popover`] 弹出框上的交互穿透
    /// 到基底层。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Makes interactions on the popup of the [`Popover`] fall through
    /// to the base layer.
    ///
    /// </details>
    pub fn passthrough(self) -> Popover<W, V, Message> {
        Popover {
            content: self.content,
            popup: self.popup.map(Opaque::into_inner),
            position: self.position,
            gap: self.gap,
            snap_within_viewport: self.snap_within_viewport,
            on_close: self.on_close,
        }
    }

    /// 设置当光标位于弹出框的边界之上时，
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
    /// over the bounds of the popup.
    ///
    /// By default, only mouse button presses are captured. Enabling this flag
    /// also captures scroll wheel events, preventing them from passing through
    /// to the elements below.
    ///
    /// </details>
    pub fn capture_scroll(mut self, capture: bool) -> Self {
        self.popup = self
            .popup
            .take()
            .map(|popup| Opaque::new(popup.into_inner()).capture_scroll(capture));
        self
    }
}

impl<W, V, Message> widget::Meta for Popover<W, V, Message> {}

impl<W, V, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Popover<W, V, Message>
where
    W: Widget<Message, Theme, Renderer>,
    V: Widget<Message, Theme, Renderer>,
    Message: Clone,
    Renderer: crate::core::Renderer,
{
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(State {
            needs_relayout: true,
        })
    }

    fn diff(&mut self, tree: &mut widget::Tree) {
        // The popup's contents may have changed, so the cached layout
        // (if any) is no longer valid
        // 弹出框的内容可能已经改变，因此缓存的布局
        // （如果有）不再有效
        tree.state.downcast_mut::<State>().needs_relayout = true;

        if tree.children.len() != 2 {
            tree.children = vec![
                widget::Tree::new(&self.content),
                widget::Tree::new(&self.popup),
            ];
        }

        tree.children[0].diff(&mut self.content);
        tree.children[1].diff(&mut self.popup);
    }

    fn size(&self) -> Size<Length> {
        self.content.size()
    }

    fn layout(&mut self, tree: &mut widget::Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.content.layout(&mut tree.children[0], renderer, limits);

        tree.size = tree.children[0].size;
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

    fn mouse_interaction(
        &self,
        tree: &widget::Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content
            .mouse_interaction(&tree.children[0], layout, cursor, viewport, renderer)
    }

    fn operate(
        &mut self,
        tree: &mut widget::Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        operation.container(None, layout.bounds(), viewport);
        operation.traverse(&mut |operation| {
            self.content
                .operate(&mut tree.children[0], layout, viewport, renderer, operation);
        });
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
        let mut children = tree.children.iter_mut();

        let base = children.next().unwrap();
        let popup_tree = children.next();

        let mut overlays =
            self.content
                .overlay(base, layout, renderer, viewport, translation, window);

        let viewport = *viewport + translation;

        if let (Some(popup), Some(popup_tree)) = (self.popup.as_mut(), popup_tree) {
            let state = tree.state.downcast_mut::<State>();

            // (Re)compute the popup's layout if it was invalidated by
            // `Widget::diff`
            // 如果弹出框的布局已被 `Widget::diff` 作废，
            // 就（重新）计算它
            if state.needs_relayout {
                popup.layout(
                    popup_tree,
                    renderer,
                    &layout::Limits::new(
                        Size::ZERO,
                        if self.snap_within_viewport {
                            viewport.size()
                        } else {
                            Size::INFINITE
                        },
                    ),
                );

                state.needs_relayout = false;
            }

            let popup_size = popup_tree.size;

            let content_bounds = layout.bounds() + translation;

            let popup_bounds = crate::overlay::Position::from(self.position).resolve(
                content_bounds,
                popup_size,
                self.gap,
                Point::ORIGIN,
                viewport,
                self.snap_within_viewport,
            );

            overlays.push(overlay::Element::new(Box::new(Overlay {
                content: popup,
                tree: popup_tree,
                layout: Layout::new(popup_size).move_to(popup_bounds.position()),
                content_bounds,
                on_close: self.on_close.clone(),
                viewport,
                window,
            })));
        }

        overlays
    }
}

/// 弹出层的位置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The position of the popover.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Position {
    /// 弹出层出现在部件可用空间最大的
    /// 一侧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The popover will appear on the side of the widget with the most
    /// available space.
    ///
    /// </details>
    #[default]
    Auto,
    /// 弹出层出现在部件的上方。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The popover will appear on the top of the widget.
    ///
    /// </details>
    Top,
    /// 弹出层出现在部件的下方。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The popover will appear on the bottom of the widget.
    ///
    /// </details>
    Bottom,
    /// 弹出层出现在部件的左侧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The popover will appear on the left of the widget.
    ///
    /// </details>
    Left,
    /// 弹出层出现在部件的右侧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The popover will appear on the right of the widget.
    ///
    /// </details>
    Right,
}

impl From<Position> for crate::overlay::Position {
    fn from(position: Position) -> Self {
        match position {
            Position::Auto => Self::Auto {
                preference: crate::overlay::Side::default(),
            },
            Position::Top => Self::Top,
            Position::Bottom => Self::Bottom,
            Position::Left => Self::Left,
            Position::Right => Self::Right,
        }
    }
}

struct State {
    needs_relayout: bool,
}

struct Overlay<'b, V, Message> {
    content: &'b mut V,
    tree: &'b mut widget::Tree,
    layout: Layout,
    content_bounds: Rectangle,
    on_close: Option<Message>,
    viewport: Rectangle,
    window: Size,
}

impl<V, Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for Overlay<'_, V, Message>
where
    V: Widget<Message, Theme, Renderer>,
    Renderer: crate::core::Renderer,
{
    fn update(
        &mut self,
        event: &Event,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
    ) {
        let cursor_position = cursor.position();

        let is_inside =
            cursor_position.is_some_and(|position| self.layout.bounds().contains(position));

        let is_over_base =
            cursor_position.is_some_and(|position| self.content_bounds.contains(position));

        if matches!(
            event,
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. })
        ) && !is_inside
        {
            if is_over_base {
                // The base is responsible for its own behavior, so let the
                // event reach it.
                // 基底元素负责它自己的行为，所以让
                // 事件到达它。
                return;
            }

            // The user clicked outside of the popover: notify the application
            // of a close request.
            // 用户在弹出层之外点击：向应用程序通知
            // 关闭请求。
            if let Some(on_close) = self.on_close.take() {
                shell.publish(on_close);
            }

            return;
        }

        self.content.update(
            self.tree,
            event,
            self.layout,
            cursor,
            renderer,
            shell,
            &self.viewport,
        );
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        cursor_position: mouse::Cursor,
    ) {
        renderer.with_layer(self.viewport, |renderer| {
            self.content.draw(
                self.tree,
                renderer,
                theme,
                style,
                self.layout,
                cursor_position,
                &self.viewport,
            );
        });
    }

    fn mouse_interaction(&self, cursor: mouse::Cursor, renderer: &Renderer) -> mouse::Interaction {
        if !cursor.is_over(self.layout.bounds()) {
            return mouse::Interaction::None;
        }

        self.content
            .mouse_interaction(self.tree, self.layout, cursor, &self.viewport, renderer)
    }

    fn operate(&mut self, renderer: &Renderer, operation: &mut dyn widget::Operation) {
        self.content
            .operate(self.tree, self.layout, &self.viewport, renderer, operation);
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

    /// 在其他覆盖层之上绘制弹出层。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the popover on top of other overlays.
    ///
    /// </details>
    fn index(&self) -> f32 {
        2.0
    }
}
