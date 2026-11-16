//! 开关让用户通过拨动一个切换钮来做出二元选择。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Togglers let users make binary choices by toggling a switch.
//!
//! </details>
//!
//! # 示例（Example）
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
//! # use iced::widget::Widget;
//! #
//! use iced::widget::toggler;
//!
//! struct State {
//!    is_checked: bool,
//! }
//!
//! enum Message {
//!     TogglerToggled(bool),
//! }
//!
//! fn view(state: &State) -> impl Widget<Message> {
//!     toggler(state.is_checked)
//!         .label("Toggle me!")
//!         .on_toggle(Message::TogglerToggled)
//! }
//!
//! fn update(state: &mut State, message: Message) {
//!     match message {
//!         Message::TogglerToggled(is_checked) => {
//!             state.is_checked = is_checked;
//!         }
//!     }
//! }
//! ```
use crate::core::alignment;
use crate::core::border;
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::text;
use crate::core::touch;
use crate::core::widget;
use crate::core::widget::tree::{self, Tree};
use crate::core::window;
use crate::core::{
    Background, Border, Color, Event, Font, Layout, Length, Pixels, Rectangle, Shell, Size, Theme,
    Widget,
};

/// 开关部件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A toggler widget.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// #
/// use iced::widget::toggler;
///
/// struct State {
///    is_checked: bool,
/// }
///
/// enum Message {
///     TogglerToggled(bool),
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     toggler(state.is_checked)
///         .label("Toggle me!")
///         .on_toggle(Message::TogglerToggled)
/// }
///
/// fn update(state: &mut State, message: Message) {
///     match message {
///         Message::TogglerToggled(is_checked) => {
///             state.is_checked = is_checked;
///         }
///     }
/// }
/// ```
pub struct Toggler<'a, Message, Theme = crate::Theme>
where
    Theme: Catalog,
{
    is_toggled: bool,
    on_toggle: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    label: Option<text::Fragment<'a>>,
    width: Length,
    size: f32,
    text_size: Option<Pixels>,
    line_height: Option<text::LineHeight>,
    alignment: text::Alignment,
    text_shaping: text::Shaping,
    wrapping: text::Wrapping,
    spacing: f32,
    font: Option<Font>,
    class: Theme::Class<'a>,
    last_status: Option<Status>,
}

impl<'a, Message, Theme> Toggler<'a, Message, Theme>
where
    Theme: Catalog,
{
    /// [`Toggler`] 的默认大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default size of a [`Toggler`].
    ///
    /// </details>
    pub const DEFAULT_SIZE: f32 = 16.0;

    /// 创建一个新的 [`Toggler`]。
    ///
    /// 它需要：
    ///   * 一个布尔值，描述 [`Toggler`] 是否处于选中状态
    ///   * [`Toggler`] 的一个可选标签
    ///   * 一个在 [`Toggler`] 被切换时调用的函数。它
    ///     会收到 [`Toggler`] 的新状态，并且必须产生一个
    ///     `Message`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Toggler`].
    ///
    /// It expects:
    ///   * a boolean describing whether the [`Toggler`] is checked or not
    ///   * An optional label for the [`Toggler`]
    ///   * a function that will be called when the [`Toggler`] is toggled. It
    ///     will receive the new state of the [`Toggler`] and must produce a
    ///     `Message`.
    ///
    /// </details>
    pub fn new(is_toggled: bool) -> Self {
        Toggler {
            is_toggled,
            on_toggle: None,
            label: None,
            width: Length::Fit,
            size: Self::DEFAULT_SIZE,
            text_size: None,
            line_height: None,
            alignment: text::Alignment::Default,
            text_shaping: text::Shaping::default(),
            wrapping: text::Wrapping::default(),
            spacing: Self::DEFAULT_SIZE / 2.0,
            font: None,
            class: Theme::default(),
            last_status: None,
        }
    }

    /// 设置 [`Toggler`] 的标签。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the label of the [`Toggler`].
    ///
    /// </details>
    pub fn label(mut self, label: impl text::IntoFragment<'a>) -> Self {
        self.label = Some(label.into_fragment());
        self
    }

    /// 设置用户切换 [`Toggler`] 时
    /// 应该产生的消息。
    ///
    /// 如果没有调用这个方法，[`Toggler`] 会被禁用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the message that should be produced when a user toggles
    /// the [`Toggler`].
    ///
    /// If this method is not called, the [`Toggler`] will be disabled.
    ///
    /// </details>
    pub fn on_toggle(mut self, on_toggle: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_toggle = Some(Box::new(on_toggle));
        self
    }

    /// 如果为 `Some`，设置用户切换 [`Toggler`] 时
    /// 应该产生的消息。
    ///
    /// 如果为 `None`，[`Toggler`] 会被禁用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the message that should be produced when a user toggles
    /// the [`Toggler`], if `Some`.
    ///
    /// If `None`, the [`Toggler`] will be disabled.
    ///
    /// </details>
    pub fn on_toggle_maybe(mut self, on_toggle: Option<impl Fn(bool) -> Message + 'a>) -> Self {
        self.on_toggle = on_toggle.map(|on_toggle| Box::new(on_toggle) as _);
        self
    }

    /// 设置 [`Toggler`] 的大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the size of the [`Toggler`].
    ///
    /// </details>
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into().0;
        self
    }

    /// 设置 [`Toggler`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Toggler`].
    ///
    /// </details>
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// 设置 [`Toggler`] 的文本字号。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the text size o the [`Toggler`].
    ///
    /// </details>
    pub fn text_size(mut self, text_size: impl Into<Pixels>) -> Self {
        self.text_size = Some(text_size.into());
        self
    }

    /// 设置 [`Toggler`] 的文本 [`text::LineHeight`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the text [`text::LineHeight`] of the [`Toggler`].
    ///
    /// </details>
    pub fn line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    /// 设置 [`Toggler`] 文本的水平对齐方式
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the horizontal alignment of the text of the [`Toggler`]
    ///
    /// </details>
    pub fn alignment(mut self, alignment: impl Into<text::Alignment>) -> Self {
        self.alignment = alignment.into();
        self
    }

    /// 设置 [`Toggler`] 的 [`text::Shaping`] 策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`text::Shaping`] strategy of the [`Toggler`].
    ///
    /// </details>
    pub fn shaping(mut self, shaping: text::Shaping) -> Self {
        self.text_shaping = shaping;
        self
    }

    /// 设置 [`Toggler`] 的 [`text::Wrapping`] 策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`text::Wrapping`] strategy of the [`Toggler`].
    ///
    /// </details>
    pub fn wrapping(mut self, wrapping: text::Wrapping) -> Self {
        self.wrapping = wrapping;
        self
    }

    /// 设置 [`Toggler`] 与文本之间的间距。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the spacing between the [`Toggler`] and the text.
    ///
    /// </details>
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    /// 设置 [`Toggler`] 文本的 [`Font`]
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Font`] of the text of the [`Toggler`]
    ///
    /// </details>
    ///
    /// [`Font`]: crate::core::Font
    pub fn font(mut self, font: impl Into<Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// 设置 [`Toggler`] 的样式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style of the [`Toggler`].
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

    /// 设置 [`Toggler`] 的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style class of the [`Toggler`].
    ///
    /// </details>
    #[cfg(feature = "advanced")]
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<Message, Theme> widget::Meta for Toggler<'_, Message, Theme> where Theme: Catalog {}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Toggler<'_, Message, Theme>
where
    Theme: Catalog,
    Renderer: text::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<text::paragraph::Plain<Renderer::Paragraph>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(text::paragraph::Plain::<Renderer::Paragraph>::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Fit,
        }
    }

    fn diff(&mut self, tree: &mut Tree) {
        // The children of the tree are the track and the label; they only
        // carry their geometry, so no state is needed.
        // 树的子节点是轨道和标签；它们只
        // 携带各自的几何信息，因此不需要状态。
        tree.children.resize_with(2, Tree::empty);
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let limits = limits.width(self.width);

        let size = if renderer::CRISP {
            let scale_factor = renderer.hint_factor().unwrap_or(1.0);

            (self.size * scale_factor).round() / scale_factor
        } else {
            self.size
        };

        let track = Size::new(2.0 * size, size);

        let label = if let Some(label) = self.label.as_deref() {
            let state = tree
                .state
                .downcast_mut::<text::paragraph::Plain<Renderer::Paragraph>>();

            widget::text::layout(
                state,
                renderer,
                &limits.shrink(Size::new(track.width + self.spacing, 0.0)),
                label,
                widget::text::Format {
                    width: self.width,
                    height: Length::Fit,
                    line_height: self.line_height,
                    size: self.text_size,
                    font: self.font,
                    align_x: self.alignment,
                    align_y: alignment::Vertical::Top,
                    shaping: self.text_shaping,
                    wrapping: self.wrapping,
                    ellipsis: text::Ellipsis::None,
                },
            )
        } else {
            Size::ZERO
        };

        let spacing = if self.label.is_some() {
            self.spacing
        } else {
            0.0
        };

        layout::next_to_each_other(tree, track, label, spacing);
    }

    fn update(
        &mut self,
        _tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let Some(on_toggle) = &self.on_toggle else {
            return;
        };

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let mouse_over = cursor.is_over(layout.bounds());

                if mouse_over {
                    shell.publish(on_toggle(!self.is_toggled));
                    shell.capture_event();
                }
            }
            _ => {}
        }

        let current_status = if self.on_toggle.is_none() {
            Status::Disabled {
                is_toggled: self.is_toggled,
            }
        } else if cursor.is_over(layout.bounds()) {
            Status::Hovered {
                is_toggled: self.is_toggled,
            }
        } else {
            Status::Active {
                is_toggled: self.is_toggled,
            }
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.last_status = Some(current_status);
        } else if self
            .last_status
            .is_some_and(|status| status != current_status)
        {
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            if self.on_toggle.is_some() {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::NotAllowed
            }
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        defaults: &renderer::Style,
        layout: Layout,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let style = theme.style(
            &self.class,
            self.last_status.unwrap_or(Status::Disabled {
                is_toggled: self.is_toggled,
            }),
        );

        let mut children = layout.iter(&tree.children);
        let (track_layout, _) = children.next().unwrap();

        if self.label.is_some() {
            let (label_layout, _) = children.next().unwrap();

            let state = tree
                .state
                .downcast_ref::<text::paragraph::Plain<Renderer::Paragraph>>();

            crate::text::draw(
                renderer,
                defaults,
                label_layout.bounds(),
                state.raw(),
                crate::text::Style {
                    color: style.text_color,
                    selection: None,
                },
                theme.selection(),
                viewport,
            );
        }

        let bounds = track_layout.bounds();

        let border_radius = style
            .border_radius
            .unwrap_or_else(|| border::Radius::new(bounds.height / 2.0));

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    radius: border_radius,
                    width: style.background_border_width,
                    color: style.background_border_color,
                },
                ..renderer::Quad::default()
            },
            style.background,
        );

        let toggle_bounds = {
            // Try to align toggle to the pixel grid
            // 尽量把切换钮对齐到像素网格
            let bounds = if renderer::CRISP {
                let scale_factor = renderer.hint_factor().unwrap_or(1.0);
                bounds.hint(scale_factor)
            } else {
                bounds
            };

            let padding = (style.padding_ratio * bounds.height).round();

            Rectangle {
                x: bounds.x
                    + if self.is_toggled {
                        bounds.width - bounds.height + padding
                    } else {
                        padding
                    },
                y: bounds.y + padding,
                width: bounds.height - (2.0 * padding),
                height: bounds.height - (2.0 * padding),
            }
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: toggle_bounds,
                border: Border {
                    radius: border_radius,
                    width: style.foreground_border_width,
                    color: style.foreground_border_color,
                },
                ..renderer::Quad::default()
            },
            style.foreground,
        );
    }
}

/// [`Toggler`] 可能的交互状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The possible status of a [`Toggler`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// [`Toggler`] 可以交互。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Toggler`] can be interacted with.
    ///
    /// </details>
    Active {
        /// 表示 [`Toggler`] 是否处于开启状态。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Indicates whether the [`Toggler`] is toggled.
        ///
        /// </details>
        is_toggled: bool,
    },
    /// [`Toggler`] 正处于悬停状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Toggler`] is being hovered.
    ///
    /// </details>
    Hovered {
        /// 表示 [`Toggler`] 是否处于开启状态。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Indicates whether the [`Toggler`] is toggled.
        ///
        /// </details>
        is_toggled: bool,
    },
    /// [`Toggler`] 已被禁用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Toggler`] is disabled.
    ///
    /// </details>
    Disabled {
        /// 表示 [`Toggler`] 是否处于开启状态。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Indicates whether the [`Toggler`] is toggled.
        ///
        /// </details>
        is_toggled: bool,
    },
}

/// 开关的外观。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The appearance of a toggler.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// 开关的背景 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The background [`Color`] of the toggler.
    ///
    /// </details>
    pub background: Background,
    /// 开关背景边框的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The width of the background border of the toggler.
    ///
    /// </details>
    pub background_border_width: f32,
    /// 开关背景边框的 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Color`] of the background border of the toggler.
    ///
    /// </details>
    pub background_border_color: Color,
    /// 开关的前景 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The foreground [`Color`] of the toggler.
    ///
    /// </details>
    pub foreground: Background,
    /// 开关前景边框的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The width of the foreground border of the toggler.
    ///
    /// </details>
    pub foreground_border_width: f32,
    /// 开关前景边框的 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Color`] of the foreground border of the toggler.
    ///
    /// </details>
    pub foreground_border_color: Color,
    /// 开关的文本 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text [`Color`] of the toggler.
    ///
    /// </details>
    pub text_color: Option<Color>,
    /// 开关的边框半径。
    ///
    /// 如果为 `None`，开关会是完全圆润的。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The border radius of the toggler.
    ///
    /// If `None`, the toggler will be perfectly round.
    ///
    /// </details>
    pub border_radius: Option<border::Radius>,
    /// 背景与切换钮之间的间隔比例，相对于高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The ratio of separation between the background and the toggle in relative height.
    ///
    /// </details>
    pub padding_ratio: f32,
}

/// [`Toggler`] 的主题样式目录。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The theme catalog of a [`Toggler`].
///
/// </details>
pub trait Catalog: Sized {
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

    /// 全局的选区 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The global selection [`Color`].
    ///
    /// </details>
    fn selection(&self) -> Color;
}

/// [`Toggler`] 的样式函数。
///
/// 它只是一个装箱的闭包：`Fn(&Theme, Status) -> Style`。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A styling function for a [`Toggler`].
///
/// This is just a boxed closure: `Fn(&Theme, Status) -> Style`.
///
/// </details>
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }

    fn selection(&self) -> Color {
        self.palette().background.strongest.color
    }
}

/// [`Toggler`] 的默认样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The default style of a [`Toggler`].
///
/// </details>
pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();

    let background = match status {
        Status::Active { is_toggled } | Status::Hovered { is_toggled } => {
            if is_toggled {
                palette.primary.base.color
            } else {
                palette.background.strong.color
            }
        }
        Status::Disabled { is_toggled } => {
            if is_toggled {
                palette.background.strong.color
            } else {
                palette.background.weak.color
            }
        }
    };

    let foreground = match status {
        Status::Active { is_toggled } => {
            if is_toggled {
                palette.primary.base.text
            } else {
                palette.background.base.color
            }
        }
        Status::Hovered { is_toggled } => {
            if is_toggled {
                Color {
                    a: 0.5,
                    ..palette.primary.base.text
                }
            } else {
                palette.background.weak.color
            }
        }
        Status::Disabled { .. } => palette.background.weakest.color,
    };

    Style {
        background: background.into(),
        foreground: foreground.into(),
        foreground_border_width: 0.0,
        foreground_border_color: Color::TRANSPARENT,
        background_border_width: 0.0,
        background_border_color: Color::TRANSPARENT,
        text_color: None,
        border_radius: None,
        padding_ratio: 0.1,
    }
}
