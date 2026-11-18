//! 工具提示在某个元素被悬停时，在其上方显示一条提示信息。
//!
//! 默认情况下，工具提示会立即显示，不过可以通过
//! [`Tooltip::delay`] 调整。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Tooltips display a hint of information over some element when hovered.
//!
//! By default, the tooltip is displayed immediately, however, this can be adjusted
//! with [`Tooltip::delay`].
//!
//! </details>
//!
//! # 示例（Example）
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } }
//! # pub type State = ();
//! # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
//! use iced::widget::{container, tooltip};
//!
//! enum Message {
//!     // ...
//! }
//!
//! fn view(_state: &State) -> Element<'_, Message> {
//!     tooltip(
//!         "Hover me to display the tooltip!",
//!         container("This is the tooltip contents!")
//!             .padding(10)
//!             .style(container::rounded_box),
//!         tooltip::Position::Bottom,
//!     ).into()
//! }
//! ```
use crate::container;
use crate::core::layout::{self, Layout};
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::text;
use crate::core::time::{Duration, Instant};
use crate::core::widget::{self, Widget};
use crate::core::window;
use crate::core::{
    Clipboard, Element, Event, Length, Padding, Pixels, Point, Rectangle,
    Shell, Size, Vector,
};

/// 用于在一个部件上方显示另一个部件的元素。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An element to display a widget over another.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } }
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// use iced::widget::{container, tooltip};
///
/// enum Message {
///     // ...
/// }
///
/// fn view(_state: &State) -> Element<'_, Message> {
///     tooltip(
///         "Hover me to display the tooltip!",
///         container("This is the tooltip contents!")
///             .padding(10)
///             .style(container::rounded_box),
///         tooltip::Position::Bottom,
///     ).into()
/// }
/// ```
pub struct Tooltip<
    'a,
    Message,
    Theme = crate::Theme,
    Renderer = crate::Renderer,
> where
    Theme: container::Catalog,
    Renderer: text::Renderer,
{
    content: Element<'a, Message, Theme, Renderer>,
    tooltip: Element<'a, Message, Theme, Renderer>,
    position: Position,
    gap: f32,
    padding: f32,
    snap_within_viewport: bool,
    delay: Duration,
    class: Theme::Class<'a>,
}

impl<'a, Message, Theme, Renderer> Tooltip<'a, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
{
    /// 由此渲染器绘制的 [`Tooltip`] 的默认内边距。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default padding of a [`Tooltip`] drawn by this renderer.
    ///
    /// </details>
    const DEFAULT_PADDING: f32 = 5.0;

    /// 创建一个新的 [`Tooltip`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Tooltip`].
    ///
    /// </details>
    ///
    /// [`Tooltip`]: struct.Tooltip.html
    pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        tooltip: impl Into<Element<'a, Message, Theme, Renderer>>,
        position: Position,
    ) -> Self {
        Tooltip {
            content: content.into(),
            tooltip: tooltip.into(),
            position,
            gap: 0.0,
            padding: Self::DEFAULT_PADDING,
            snap_within_viewport: true,
            delay: Duration::ZERO,
            class: Theme::default(),
        }
    }

    /// 设置内容与其 [`Tooltip`] 之间的间隙。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the gap between the content and its [`Tooltip`].
    ///
    /// </details>
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into().0;
        self
    }

    /// 设置 [`Tooltip`] 的内边距。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the padding of the [`Tooltip`].
    ///
    /// </details>
    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = padding.into().0;
        self
    }

    /// 设置 [`Tooltip`] 显示之前的延迟。
    ///
    /// 设置为 [`Duration::ZERO`] 则立即显示。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the delay before the [`Tooltip`] is shown.
    ///
    /// Set to [`Duration::ZERO`] to be shown immediately.
    ///
    /// </details>
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// 设置 [`Tooltip`] 是否被限制在视口之内。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets whether the [`Tooltip`] is snapped within the viewport.
    ///
    /// </details>
    pub fn snap_within_viewport(mut self, snap: bool) -> Self {
        self.snap_within_viewport = snap;
        self
    }

    /// 设置 [`Tooltip`] 的样式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style of the [`Tooltip`].
    ///
    /// </details>
    #[must_use]
    pub fn style(
        mut self,
        style: impl Fn(&Theme) -> container::Style + 'a,
    ) -> Self
    where
        Theme::Class<'a>: From<container::StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as container::StyleFn<'a, Theme>).into();
        self
    }

    /// 设置 [`Tooltip`] 的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style class of the [`Tooltip`].
    ///
    /// </details>
    #[cfg(feature = "advanced")]
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Tooltip<'_, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
{
    fn children(&self) -> Vec<widget::Tree> {
        vec![
            widget::Tree::new(&self.content),
            widget::Tree::new(&self.tooltip),
        ]
    }

    fn diff(&self, tree: &mut widget::Tree) {
        tree.diff_children(&[
            self.content.as_widget(),
            self.tooltip.as_widget(),
        ]);
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(State::default())
    }

    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<State>()
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.content.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content.as_widget_mut().layout(
            &mut tree.children[0],
            renderer,
            limits,
        )
    }

    fn update(
        &mut self,
        tree: &mut widget::Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if let Event::Mouse(_)
        | Event::Window(window::Event::RedrawRequested(_)) = event
        {
            let state = tree.state.downcast_mut::<State>();
            let now = Instant::now();
            let cursor_position = cursor.position_over(layout.bounds());

            match (*state, cursor_position) {
                (State::Idle, Some(cursor_position)) => {
                    if self.delay == Duration::ZERO {
                        *state = State::Open { cursor_position };
                        shell.invalidate_layout();
                    } else {
                        *state = State::Hovered { at: now };
                    }

                    shell.request_redraw_at(now + self.delay);
                }
                (State::Hovered { .. }, None) => {
                    *state = State::Idle;
                }
                (State::Hovered { at, .. }, _) if at.elapsed() < self.delay => {
                    shell.request_redraw_at(now + self.delay - at.elapsed());
                }
                (State::Hovered { .. }, Some(cursor_position)) => {
                    *state = State::Open { cursor_position };
                    shell.invalidate_layout();
                }
                (
                    State::Open {
                        cursor_position: last_position,
                    },
                    Some(cursor_position),
                ) if self.position == Position::FollowCursor
                    && last_position != cursor_position =>
                {
                    *state = State::Open { cursor_position };
                    shell.request_redraw();
                }
                (State::Open { .. }, None) => {
                    *state = State::Idle;
                    shell.invalidate_layout();

                    if !matches!(
                        event,
                        Event::Window(window::Event::RedrawRequested(_)),
                    ) {
                        shell.request_redraw();
                    }
                }
                (State::Open { .. }, Some(_)) | (State::Idle, None) => (),
            }
        }

        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &widget::Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        inherited_style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            inherited_style,
            layout,
            cursor,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut widget::Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let state = tree.state.downcast_ref::<State>();

        let mut children = tree.children.iter_mut();

        let content = self.content.as_widget_mut().overlay(
            children.next().unwrap(),
            layout,
            renderer,
            viewport,
            translation,
        );

        let tooltip = if let State::Open { cursor_position } = *state {
            Some(overlay::Element::new(Box::new(Overlay {
                position: layout.position() + translation,
                tooltip: &mut self.tooltip,
                tree: children.next().unwrap(),
                cursor_position,
                content_bounds: layout.bounds(),
                snap_within_viewport: self.snap_within_viewport,
                positioning: self.position,
                gap: self.gap,
                padding: self.padding,
                class: &self.class,
            })))
        } else {
            None
        };

        if content.is_some() || tooltip.is_some() {
            Some(
                overlay::Group::with_children(
                    content.into_iter().chain(tooltip).collect(),
                )
                .overlay(),
            )
        } else {
            None
        }
    }

    fn operate(
        &mut self,
        tree: &mut widget::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                layout,
                renderer,
                operation,
            );
        });
    }
}

impl<'a, Message, Theme, Renderer> From<Tooltip<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: container::Catalog + 'a,
    Renderer: text::Renderer + 'a,
{
    fn from(
        tooltip: Tooltip<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(tooltip)
    }
}

/// 工具提示的位置。默认跟随光标。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The position of the tooltip. Defaults to following the cursor.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Position {
    /// 工具提示会出现在部件的上方。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The tooltip will appear on the top of the widget.
    ///
    /// </details>
    #[default]
    Top,
    /// 工具提示会出现在部件的下方。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The tooltip will appear on the bottom of the widget.
    ///
    /// </details>
    Bottom,
    /// 工具提示会出现在部件的左侧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The tooltip will appear on the left of the widget.
    ///
    /// </details>
    Left,
    /// 工具提示会出现在部件的右侧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The tooltip will appear on the right of the widget.
    ///
    /// </details>
    Right,
    /// 工具提示会跟随光标。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The tooltip will follow the cursor.
    ///
    /// </details>
    FollowCursor,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum State {
    #[default]
    Idle,
    Hovered {
        at: Instant,
    },
    Open {
        cursor_position: Point,
    },
}

struct Overlay<'a, 'b, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
{
    position: Point,
    tooltip: &'b mut Element<'a, Message, Theme, Renderer>,
    tree: &'b mut widget::Tree,
    cursor_position: Point,
    content_bounds: Rectangle,
    snap_within_viewport: bool,
    positioning: Position,
    gap: f32,
    padding: f32,
    class: &'b Theme::Class<'a>,
}

impl<Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for Overlay<'_, '_, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let viewport = Rectangle::with_size(bounds);

        let tooltip_layout = self.tooltip.as_widget_mut().layout(
            self.tree,
            renderer,
            &layout::Limits::new(
                Size::ZERO,
                if self.snap_within_viewport {
                    viewport.size()
                } else {
                    Size::INFINITE
                },
            )
            .shrink(Padding::new(self.padding)),
        );

        let text_bounds = tooltip_layout.bounds();
        let x_center = self.position.x
            + (self.content_bounds.width - text_bounds.width) / 2.0;
        let y_center = self.position.y
            + (self.content_bounds.height - text_bounds.height) / 2.0;

        let mut tooltip_bounds = {
            let offset = match self.positioning {
                Position::Top => Vector::new(
                    x_center,
                    self.position.y
                        - text_bounds.height
                        - self.gap
                        - self.padding,
                ),
                Position::Bottom => Vector::new(
                    x_center,
                    self.position.y
                        + self.content_bounds.height
                        + self.gap
                        + self.padding,
                ),
                Position::Left => Vector::new(
                    self.position.x
                        - text_bounds.width
                        - self.gap
                        - self.padding,
                    y_center,
                ),
                Position::Right => Vector::new(
                    self.position.x
                        + self.content_bounds.width
                        + self.gap
                        + self.padding,
                    y_center,
                ),
                Position::FollowCursor => {
                    let translation =
                        self.position - self.content_bounds.position();

                    Vector::new(
                        self.cursor_position.x,
                        self.cursor_position.y - text_bounds.height,
                    ) + translation
                }
            };

            Rectangle {
                x: offset.x - self.padding,
                y: offset.y - self.padding,
                width: text_bounds.width + self.padding * 2.0,
                height: text_bounds.height + self.padding * 2.0,
            }
        };

        if self.snap_within_viewport {
            if tooltip_bounds.x < viewport.x {
                tooltip_bounds.x = viewport.x;
            } else if viewport.x + viewport.width
                < tooltip_bounds.x + tooltip_bounds.width
            {
                tooltip_bounds.x =
                    viewport.x + viewport.width - tooltip_bounds.width;
            }

            if tooltip_bounds.y < viewport.y {
                tooltip_bounds.y = viewport.y;
            } else if viewport.y + viewport.height
                < tooltip_bounds.y + tooltip_bounds.height
            {
                tooltip_bounds.y =
                    viewport.y + viewport.height - tooltip_bounds.height;
            }
        }

        layout::Node::with_children(
            tooltip_bounds.size(),
            vec![
                tooltip_layout
                    .translate(Vector::new(self.padding, self.padding)),
            ],
        )
        .translate(Vector::new(tooltip_bounds.x, tooltip_bounds.y))
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        inherited_style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
    ) {
        let style = theme.style(self.class);

        container::draw_background(renderer, &style, layout.bounds());

        let defaults = renderer::Style {
            text_color: style.text_color.unwrap_or(inherited_style.text_color),
        };

        self.tooltip.as_widget().draw(
            self.tree,
            renderer,
            theme,
            &defaults,
            layout.children().next().unwrap(),
            cursor_position,
            &Rectangle::with_size(Size::INFINITE),
        );
    }
}
