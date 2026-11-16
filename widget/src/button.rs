//! 按钮让用户可以通过按下它们来执行动作。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Buttons allow your users to perform actions by pressing them.
//!
//! </details>
//!
//! # 示例（Example）
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
//! # use iced::widget::Widget;
//! # pub type State = ();
//! use iced::widget::button;
//!
//! #[derive(Clone)]
//! enum Message {
//!     ButtonPressed,
//! }
//!
//! fn view(state: &State) -> impl Widget<Message> {
//!     button("Press me!").on_press(Message::ButtonPressed)
//! }
//! ```
use crate::core::border::{self, Border};
use crate::core::layout;
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::theme::palette;
use crate::core::touch;
use crate::core::widget::tree::{self, Tree};
use crate::core::widget::{Meta, Operation};
use crate::core::window;
use crate::core::{
    Background, Color, Event, Layout, Length, Padding, Rectangle, Shadow, Shell, Size, Theme,
    Vector, Widget,
};

/// 被按下时产生一条消息的通用部件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A generic widget that produces a message when pressed.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// use iced::widget::button;
///
/// #[derive(Clone)]
/// enum Message {
///     ButtonPressed,
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     button("Press me!").on_press(Message::ButtonPressed)
/// }
/// ```
///
/// 如果没有设置 [`Button::on_press`] 处理函数，得到的 [`Button`] 将
/// 处于禁用状态：
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// If a [`Button::on_press`] handler is not set, the resulting [`Button`] will
/// be disabled:
///
/// </details>
///
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// use iced::widget::button;
///
/// #[derive(Clone)]
/// enum Message {
///     ButtonPressed,
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     button("I am disabled!")
/// }
/// ```
pub struct Button<'a, Message, W = crate::Element<'a, Message>, Theme = crate::Theme>
where
    Theme: Catalog,
{
    content: W,
    on_press: Option<OnPress<'a, Message>>,
    width: Length,
    height: Length,
    padding: Padding,
    clip: bool,
    class: Theme::Class<'a>,
    status: Option<Status>,
}

enum OnPress<'a, Message> {
    Direct(Message),
    Closure(Box<dyn Fn() -> Message + 'a>),
}

impl<Message: Clone> OnPress<'_, Message> {
    fn get(&self) -> Message {
        match self {
            OnPress::Direct(message) => message.clone(),
            OnPress::Closure(f) => f(),
        }
    }
}

impl<'a, Message, W, Theme> Button<'a, Message, W, Theme>
where
    Theme: Catalog,
{
    /// 使用给定的内容创建一个新的 [`Button`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Button`] with the given content.
    ///
    /// </details>
    pub fn new(content: W) -> Self {
        Button {
            content,
            on_press: None,
            width: Length::Fit,
            height: Length::Fit,
            padding: DEFAULT_PADDING,
            clip: false,
            class: Theme::default(),
            status: None,
        }
    }

    /// 设置 [`Button`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Button`].
    ///
    /// </details>
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// 设置 [`Button`] 的高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the height of the [`Button`].
    ///
    /// </details>
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// 设置 [`Button`] 的 [`Padding`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Padding`] of the [`Button`].
    ///
    /// </details>
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// 设置 [`Button`] 被按下时产生的消息。
    ///
    /// 除非调用了 `on_press`，否则 [`Button`] 将处于禁用状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the message that will be produced when the [`Button`] is pressed.
    ///
    /// Unless `on_press` is called, the [`Button`] will be disabled.
    ///
    /// </details>
    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_press = Some(OnPress::Direct(on_press));
        self
    }

    /// 设置 [`Button`] 被按下时产生的消息。
    ///
    /// 它与 [`Button::on_press`] 类似，但使用闭包来产生
    /// 消息。
    ///
    /// 该闭包只有在 [`Button`] 真正被按下时才会被调用，
    /// 因此，如果创建消息的开销较大，此方法有助于
    /// 减少开销。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the message that will be produced when the [`Button`] is pressed.
    ///
    /// This is analogous to [`Button::on_press`], but using a closure to produce
    /// the message.
    ///
    /// This closure will only be called when the [`Button`] is actually pressed and,
    /// therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow.
    ///
    /// </details>
    pub fn on_press_with(mut self, on_press: impl Fn() -> Message + 'a) -> Self {
        self.on_press = Some(OnPress::Closure(Box::new(on_press)));
        self
    }

    /// 设置 [`Button`] 被按下时产生的消息，
    /// 如果为 `Some` 的话。
    ///
    /// 如果为 `None`，[`Button`] 将处于禁用状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the message that will be produced when the [`Button`] is pressed,
    /// if `Some`.
    ///
    /// If `None`, the [`Button`] will be disabled.
    ///
    /// </details>
    pub fn on_press_maybe(mut self, on_press: Option<Message>) -> Self {
        self.on_press = on_press.map(OnPress::Direct);
        self
    }

    /// 设置 [`Button`] 被按下时产生的消息，
    /// 如果为 `Some` 的话。
    ///
    /// 它与 [`Button::on_press`] 类似，但使用闭包来产生
    /// 消息。
    ///
    /// 该闭包只有在 [`Button`] 真正被按下时才会被调用，
    /// 因此，如果创建消息的开销较大，此方法有助于
    /// 减少开销。
    ///
    /// 如果为 `None`，[`Button`] 将处于禁用状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the message that will be produced when the [`Button`] is pressed,
    /// if `Some`.
    ///
    /// This is analogous to [`Button::on_press`], but using a closure to produce
    /// the message.
    ///
    /// This closure will only be called when the [`Button`] is actually pressed and,
    /// therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow.
    ///
    /// If `None`, the [`Button`] will be disabled.
    ///
    /// </details>
    pub fn on_press_maybe_with(mut self, on_press: Option<impl Fn() -> Message + 'a>) -> Self {
        self.on_press = on_press.map(|on_press| OnPress::Closure(Box::new(on_press)));
        self
    }

    /// 设置 [`Button`] 的内容在溢出时是否应该
    /// 被裁剪。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets whether the contents of the [`Button`] should be clipped on
    /// overflow.
    ///
    /// </details>
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// 设置 [`Button`] 的样式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style of the [`Button`].
    ///
    /// </details>
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// 设置 [`Button`] 的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style class of the [`Button`].
    ///
    /// </details>
    #[cfg(feature = "advanced")]
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct State {
    is_pressed: bool,
}

impl<'a, Message, W, Theme> Meta for Button<'a, Message, W, Theme> where Theme: Catalog {}

impl<'a, Message, W, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Button<'a, Message, W, Theme>
where
    Message: 'a + Clone,
    W: Widget<Message, Theme, Renderer>,
    Renderer: 'a + crate::core::Renderer,
    Theme: Catalog,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.content));

        let size = self.content.size();
        self.width = self.width.stack(size.width);
        self.height = self.height.stack(size.height);
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        layout::padded(
            tree,
            limits,
            self.width,
            self.height,
            self.padding,
            |tree, limits| {
                self.content.layout(tree, renderer, limits);

                tree.size
            },
        );
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds(), viewport);
        operation.traverse(&mut |operation| {
            let (layout, tree) = layout.iter_mut(&mut tree.children).next().unwrap();

            self.content
                .operate(tree, layout, viewport, renderer, operation);
        });
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
        let (content_layout, content_tree) = layout.iter_mut(&mut tree.children).next().unwrap();

        self.content.update(
            content_tree,
            event,
            content_layout,
            cursor,
            renderer,
            shell,
            viewport,
        );

        if shell.is_event_captured() {
            return;
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. })
                if self.on_press.is_some() =>
            {
                let bounds = layout.bounds();

                if cursor.is_over(bounds) {
                    let state = tree.state.downcast_mut::<State>();

                    state.is_pressed = true;

                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if let Some(on_press) = &self.on_press {
                    let state = tree.state.downcast_mut::<State>();

                    if state.is_pressed {
                        state.is_pressed = false;

                        let bounds = layout.bounds();

                        if cursor.is_over(bounds) {
                            shell.publish(on_press.get());
                        }
                    }
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                let state = tree.state.downcast_mut::<State>();

                state.is_pressed = false;
            }
            _ => {}
        }

        let current_status = if self.on_press.is_none() {
            Status::Disabled
        } else if cursor.is_over(layout.bounds()) {
            let state = tree.state.downcast_ref::<State>();

            if state.is_pressed {
                Status::Pressed
            } else {
                Status::Hovered
            }
        } else {
            Status::Active
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.status = Some(current_status);
        } else if self.status.is_some_and(|status| status != current_status) {
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let (layout, tree) = layout.iter(&tree.children).next().unwrap();
        let style = theme.style(&self.class, self.status.unwrap_or(Status::Disabled));

        if style.background.is_some() || style.border.width > 0.0 || style.shadow.color.a > 0.0 {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: style.border,
                    shadow: style.shadow,
                    snap: style.snap,
                },
                style
                    .background
                    .unwrap_or(Background::Color(Color::TRANSPARENT)),
            );
        }

        let viewport = if self.clip {
            bounds.intersection(viewport).unwrap_or(*viewport)
        } else {
            *viewport
        };

        self.content.draw(
            tree,
            renderer,
            theme,
            &renderer::Style {
                text_color: style.text_color,
            },
            layout,
            cursor,
            &viewport,
        );
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let is_mouse_over = cursor.is_over(layout.bounds());

        if is_mouse_over && self.on_press.is_some() {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
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

/// [`Button`] 的默认 [`Padding`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The default [`Padding`] of a [`Button`].
///
/// </details>
pub const DEFAULT_PADDING: Padding = Padding {
    top: 5.0,
    bottom: 5.0,
    right: 10.0,
    left: 10.0,
};

/// [`Button`] 可能的交互状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The possible status of a [`Button`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// [`Button`] 可以被按下。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Button`] can be pressed.
    ///
    /// </details>
    Active,
    /// [`Button`] 可以被按下，并且正处于悬停状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Button`] can be pressed and it is being hovered.
    ///
    /// </details>
    Hovered,
    /// [`Button`] 正在被按下。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Button`] is being pressed.
    ///
    /// </details>
    Pressed,
    /// [`Button`] 不能被按下。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Button`] cannot be pressed.
    ///
    /// </details>
    Disabled,
}

/// 按钮的样式。
///
/// 如果没有通过 [`Button::style`] 指定，
/// 将由主题提供样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The style of a button.
///
/// If not specified with [`Button::style`]
/// the theme will provide the style.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// 按钮的 [`Background`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Background`] of the button.
    ///
    /// </details>
    pub background: Option<Background>,
    /// 按钮的文本 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text [`Color`] of the button.
    ///
    /// </details>
    pub text_color: Color,
    /// 按钮的 [`Border`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Border`] of the button.
    ///
    /// </details>
    pub border: Border,
    /// 按钮的 [`Shadow`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Shadow`] of the button.
    ///
    /// </details>
    pub shadow: Shadow,
    /// 按钮是否应该对齐到像素网格。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the button should be snapped to the pixel grid.
    ///
    /// </details>
    pub snap: bool,
}

impl Style {
    /// 使用给定的 [`Background`] 更新 [`Style`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Updates the [`Style`] with the given [`Background`].
    ///
    /// </details>
    pub fn with_background(self, background: impl Into<Background>) -> Self {
        Self {
            background: Some(background.into()),
            ..self
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: None,
            text_color: Color::BLACK,
            border: Border::default(),
            shadow: Shadow::default(),
            snap: renderer::CRISP,
        }
    }
}

/// [`Button`] 的主题样式目录。
///
/// 所有可以与 [`Button`] 一起使用的主题
/// 都必须实现此特质。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The theme catalog of a [`Button`].
///
/// All themes that can be used with [`Button`]
/// must implement this trait.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # use iced_widget::core::{Color, Background};
/// # use iced_widget::button::{Catalog, Status, Style};
/// # struct MyTheme;
/// #[derive(Debug, Default)]
/// pub enum ButtonClass {
///     #[default]
///     Primary,
///     Secondary,
///     Danger
/// }
///
/// impl Catalog for MyTheme {
///     type Class<'a> = ButtonClass;
///     fn default<'a>() -> Self::Class<'a> {
///         ButtonClass::default()
///     }
///
///     fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
///         let mut style = Style::default();
///
///         match class {
///             ButtonClass::Primary => {
///                 style.background = Some(Background::Color(Color::from_rgb(0.529, 0.808, 0.921)));
///             },
///             ButtonClass::Secondary => {
///                 style.background = Some(Background::Color(Color::WHITE));
///             },
///             ButtonClass::Danger => {
///                 style.background = Some(Background::Color(Color::from_rgb(0.941, 0.502, 0.502)));
///             },
///         }
///
///         style
///     }
/// }
/// ```
///
/// 不过，要想对 `MyTheme` 使用 [`Button::style`]，
/// [`Catalog::Class`] 必须实现
/// `From<StyleFn<'_, MyTheme>>`。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Although, in order to use [`Button::style`]
/// with `MyTheme`, [`Catalog::Class`] must implement
/// `From<StyleFn<'_, MyTheme>>`.
///
/// </details>
pub trait Catalog {
    /// [`Catalog`] 的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The item class of the [`Catalog`].
    ///
    /// </details>
    type Class<'a>;

    /// [`Catalog`] 产生的默认样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default class produced by the [`Catalog`].
    ///
    /// </details>
    fn default<'a>() -> Self::Class<'a>;

    /// 某个样式类在给定交互状态下的 [`Style`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Style`] of a class with the given status.
    ///
    /// </details>
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// [`Button`] 的样式函数。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A styling function for a [`Button`].
///
/// </details>
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// 主要按钮；表示主要动作。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A primary button; denoting a main action.
///
/// </details>
pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let base = styled(palette.primary.base);

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            background: Some(Background::Color(palette.primary.strong.color)),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// 次要按钮；表示补充性的动作。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A secondary button; denoting a complementary action.
///
/// </details>
pub fn secondary(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let base = styled(palette.secondary.base);

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            background: Some(Background::Color(palette.secondary.strong.color)),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// 成功按钮；表示良好的结果。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A success button; denoting a good outcome.
///
/// </details>
pub fn success(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let base = styled(palette.success.base);

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            background: Some(Background::Color(palette.success.strong.color)),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// 警告按钮；表示有风险的动作。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A warning button; denoting a risky action.
///
/// </details>
pub fn warning(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let base = styled(palette.warning.base);

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            background: Some(Background::Color(palette.warning.strong.color)),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// 危险按钮；表示破坏性的动作。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A danger button; denoting a destructive action.
///
/// </details>
pub fn danger(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let base = styled(palette.danger.base);

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            background: Some(Background::Color(palette.danger.strong.color)),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// 文本按钮；适合用作链接。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A text button; useful for links.
///
/// </details>
pub fn text(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();

    let base = Style {
        text_color: palette.background.base.text,
        ..Style::default()
    };

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            text_color: palette.background.base.text.scale_alpha(0.8),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// 使用背景色阶的按钮。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A button using background shades.
///
/// </details>
pub fn background(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let base = styled(palette.background.base);

    match status {
        Status::Active => base,
        Status::Pressed => Style {
            background: Some(Background::Color(palette.background.strong.color)),
            ..base
        },
        Status::Hovered => Style {
            background: Some(Background::Color(palette.background.weak.color)),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// 使用较弱背景色阶的低调按钮。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A subtle button using weak background shades.
///
/// </details>
pub fn subtle(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let base = styled(palette.background.weakest);

    match status {
        Status::Active => base,
        Status::Pressed => Style {
            background: Some(Background::Color(palette.background.strong.color)),
            ..base
        },
        Status::Hovered => Style {
            background: Some(Background::Color(palette.background.weaker.color)),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

fn styled(pair: palette::Pair) -> Style {
    Style {
        background: Some(Background::Color(pair.color)),
        text_color: pair.text,
        border: border::rounded(2),
        ..Style::default()
    }
}

fn disabled(style: Style) -> Style {
    Style {
        background: style
            .background
            .map(|background| background.scale_alpha(0.5)),
        text_color: style.text_color.scale_alpha(0.5),
        ..style
    }
}
