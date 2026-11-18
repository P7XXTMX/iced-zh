//! 单选按钮让用户从一组选项中选择一个。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Radio buttons let users choose a single option from a bunch of options.
//!
//! </details>
//!
//! # 示例（Example）
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
//! # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
//! #
//! use iced::widget::{column, radio};
//!
//! struct State {
//!    selection: Option<Choice>,
//! }
//!
//! #[derive(Debug, Clone, Copy)]
//! enum Message {
//!     RadioSelected(Choice),
//! }
//!
//! #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//! enum Choice {
//!     A,
//!     B,
//!     C,
//!     All,
//! }
//!
//! fn view(state: &State) -> Element<'_, Message> {
//!     let a = radio(
//!         "A",
//!         Choice::A,
//!         state.selection,
//!         Message::RadioSelected,
//!     );
//!
//!     let b = radio(
//!         "B",
//!         Choice::B,
//!         state.selection,
//!         Message::RadioSelected,
//!     );
//!
//!     let c = radio(
//!         "C",
//!         Choice::C,
//!         state.selection,
//!         Message::RadioSelected,
//!     );
//!
//!     let all = radio(
//!         "All of the above",
//!         Choice::All,
//!         state.selection,
//!         Message::RadioSelected
//!     );
//!
//!     column![a, b, c, all].into()
//! }
//! ```
use crate::core::alignment;
use crate::core::border::{self, Border};
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::text;
use crate::core::touch;
use crate::core::widget;
use crate::core::widget::tree::{self, Tree};
use crate::core::window;
use crate::core::{
    Background, Clipboard, Color, Element, Event, Layout, Length, Pixels,
    Rectangle, Shell, Size, Theme, Widget,
};

/// 表示一个选项的圆形按钮。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A circular button representing a choice.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// #
/// use iced::widget::{column, radio};
///
/// struct State {
///    selection: Option<Choice>,
/// }
///
/// #[derive(Debug, Clone, Copy)]
/// enum Message {
///     RadioSelected(Choice),
/// }
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// enum Choice {
///     A,
///     B,
///     C,
///     All,
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     let a = radio(
///         "A",
///         Choice::A,
///         state.selection,
///         Message::RadioSelected,
///     );
///
///     let b = radio(
///         "B",
///         Choice::B,
///         state.selection,
///         Message::RadioSelected,
///     );
///
///     let c = radio(
///         "C",
///         Choice::C,
///         state.selection,
///         Message::RadioSelected,
///     );
///
///     let all = radio(
///         "All of the above",
///         Choice::All,
///         state.selection,
///         Message::RadioSelected
///     );
///
///     column![a, b, c, all].into()
/// }
/// ```
pub struct Radio<'a, Message, Theme = crate::Theme, Renderer = crate::Renderer>
where
    Theme: Catalog,
    Renderer: text::Renderer,
{
    is_selected: bool,
    on_click: Message,
    label: String,
    width: Length,
    size: f32,
    spacing: f32,
    text_size: Option<Pixels>,
    text_line_height: text::LineHeight,
    text_shaping: text::Shaping,
    text_wrapping: text::Wrapping,
    font: Option<Renderer::Font>,
    class: Theme::Class<'a>,
    last_status: Option<Status>,
}

impl<'a, Message, Theme, Renderer> Radio<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: Catalog,
    Renderer: text::Renderer,
{
    /// [`Radio`] 按钮的默认大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default size of a [`Radio`] button.
    ///
    /// </details>
    pub const DEFAULT_SIZE: f32 = 16.0;

    /// [`Radio`] 按钮的默认间距。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default spacing of a [`Radio`] button.
    ///
    /// </details>
    pub const DEFAULT_SPACING: f32 = 8.0;

    /// 创建一个新的 [`Radio`] 按钮。
    ///
    /// 它需要：
    ///   * 与 [`Radio`] 按钮关联的值
    ///   * [`Radio`] 按钮的标签
    ///   * 当前选中的值
    ///   * 一个在 [`Radio`] 被选中时调用的函数。它
    ///     接收该单选按钮的值，并且必须产生一个 `Message`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Radio`] button.
    ///
    /// It expects:
    ///   * the value related to the [`Radio`] button
    ///   * the label of the [`Radio`] button
    ///   * the current selected value
    ///   * a function that will be called when the [`Radio`] is selected. It
    ///     receives the value of the radio and must produce a `Message`.
    ///
    /// </details>
    pub fn new<F, V>(
        label: impl Into<String>,
        value: V,
        selected: Option<V>,
        f: F,
    ) -> Self
    where
        V: Eq + Copy,
        F: FnOnce(V) -> Message,
    {
        Radio {
            is_selected: Some(value) == selected,
            on_click: f(value),
            label: label.into(),
            width: Length::Shrink,
            size: Self::DEFAULT_SIZE,
            spacing: Self::DEFAULT_SPACING,
            text_size: None,
            text_line_height: text::LineHeight::default(),
            text_shaping: text::Shaping::default(),
            text_wrapping: text::Wrapping::default(),
            font: None,
            class: Theme::default(),
            last_status: None,
        }
    }

    /// 设置 [`Radio`] 按钮的大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the size of the [`Radio`] button.
    ///
    /// </details>
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into().0;
        self
    }

    /// 设置 [`Radio`] 按钮的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Radio`] button.
    ///
    /// </details>
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// 设置 [`Radio`] 按钮与文本之间的间距。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the spacing between the [`Radio`] button and the text.
    ///
    /// </details>
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    /// 设置 [`Radio`] 按钮的文本大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the text size of the [`Radio`] button.
    ///
    /// </details>
    pub fn text_size(mut self, text_size: impl Into<Pixels>) -> Self {
        self.text_size = Some(text_size.into());
        self
    }

    /// 设置 [`Radio`] 按钮的文本 [`text::LineHeight`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the text [`text::LineHeight`] of the [`Radio`] button.
    ///
    /// </details>
    pub fn text_line_height(
        mut self,
        line_height: impl Into<text::LineHeight>,
    ) -> Self {
        self.text_line_height = line_height.into();
        self
    }

    /// 设置 [`Radio`] 按钮的 [`text::Shaping`] 策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`text::Shaping`] strategy of the [`Radio`] button.
    ///
    /// </details>
    pub fn text_shaping(mut self, shaping: text::Shaping) -> Self {
        self.text_shaping = shaping;
        self
    }

    /// 设置 [`Radio`] 按钮的 [`text::Wrapping`] 策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`text::Wrapping`] strategy of the [`Radio`] button.
    ///
    /// </details>
    pub fn text_wrapping(mut self, wrapping: text::Wrapping) -> Self {
        self.text_wrapping = wrapping;
        self
    }

    /// 设置 [`Radio`] 按钮的文本字体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the text font of the [`Radio`] button.
    ///
    /// </details>
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// 设置 [`Radio`] 按钮的样式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style of the [`Radio`] button.
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

    /// 设置 [`Radio`] 按钮的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style class of the [`Radio`] button.
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
    for Radio<'_, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: Catalog,
    Renderer: text::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<widget::text::State<Renderer::Paragraph>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(widget::text::State::<Renderer::Paragraph>::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::next_to_each_other(
            &limits.width(self.width),
            self.spacing,
            |_| layout::Node::new(Size::new(self.size, self.size)),
            |limits| {
                let state = tree
                    .state
                    .downcast_mut::<widget::text::State<Renderer::Paragraph>>();

                widget::text::layout(
                    state,
                    renderer,
                    limits,
                    &self.label,
                    widget::text::Format {
                        width: self.width,
                        height: Length::Shrink,
                        line_height: self.text_line_height,
                        size: self.text_size,
                        font: self.font,
                        align_x: text::Alignment::Default,
                        align_y: alignment::Vertical::Top,
                        shaping: self.text_shaping,
                        wrapping: self.text_wrapping,
                    },
                )
            },
        )
    }

    fn update(
        &mut self,
        _tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(layout.bounds()) {
                    shell.publish(self.on_click.clone());
                    shell.capture_event();
                }
            }
            _ => {}
        }

        let current_status = {
            let is_mouse_over = cursor.is_over(layout.bounds());
            let is_selected = self.is_selected;

            if is_mouse_over {
                Status::Hovered { is_selected }
            } else {
                Status::Active { is_selected }
            }
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.last_status = Some(current_status);
        } else if self
            .last_status
            .is_some_and(|last_status| last_status != current_status)
        {
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
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
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let mut children = layout.children();

        let style = theme.style(
            &self.class,
            self.last_status.unwrap_or(Status::Active {
                is_selected: self.is_selected,
            }),
        );

        {
            let layout = children.next().unwrap();
            let bounds = layout.bounds();

            let size = bounds.width;
            let dot_size = size / 2.0;

            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        radius: (size / 2.0).into(),
                        width: style.border_width,
                        color: style.border_color,
                    },
                    ..renderer::Quad::default()
                },
                style.background,
            );

            if self.is_selected {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x: bounds.x + dot_size / 2.0,
                            y: bounds.y + dot_size / 2.0,
                            width: bounds.width - dot_size,
                            height: bounds.height - dot_size,
                        },
                        border: border::rounded(dot_size / 2.0),
                        ..renderer::Quad::default()
                    },
                    style.dot_color,
                );
            }
        }

        {
            let label_layout = children.next().unwrap();
            let state: &widget::text::State<Renderer::Paragraph> =
                tree.state.downcast_ref();

            crate::text::draw(
                renderer,
                defaults,
                label_layout.bounds(),
                state.raw(),
                crate::text::Style {
                    color: style.text_color,
                },
                viewport,
            );
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Radio<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + Catalog,
    Renderer: 'a + text::Renderer,
{
    fn from(
        radio: Radio<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(radio)
    }
}

/// [`Radio`] 按钮可能的交互状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The possible status of a [`Radio`] button.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// [`Radio`] 按钮可以进行交互。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Radio`] button can be interacted with.
    ///
    /// </details>
    Active {
        /// 表示 [`Radio`] 按钮当前是否被选中。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Indicates whether the [`Radio`] button is currently selected.
        ///
        /// </details>
        is_selected: bool,
    },
    /// [`Radio`] 按钮正被悬停。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Radio`] button is being hovered.
    ///
    /// </details>
    Hovered {
        /// 表示 [`Radio`] 按钮当前是否被选中。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Indicates whether the [`Radio`] button is currently selected.
        ///
        /// </details>
        is_selected: bool,
    },
}

/// 单选按钮的外观。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The appearance of a radio button.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// 单选按钮的 [`Background`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Background`] of the radio button.
    ///
    /// </details>
    pub background: Background,
    /// 单选按钮圆点的 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Color`] of the dot of the radio button.
    ///
    /// </details>
    pub dot_color: Color,
    /// 单选按钮的边框宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The border width of the radio button.
    ///
    /// </details>
    pub border_width: f32,
    /// 单选按钮的边框 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The border [`Color`] of the radio button.
    ///
    /// </details>
    pub border_color: Color,
    /// 单选按钮的文本 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text [`Color`] of the radio button.
    ///
    /// </details>
    pub text_color: Option<Color>,
}

/// [`Radio`] 的主题样式目录。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The theme catalog of a [`Radio`].
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

/// [`Radio`] 的样式函数。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A styling function for a [`Radio`].
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
}

/// [`Radio`] 按钮的默认样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The default style of a [`Radio`] button.
///
/// </details>
pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let active = Style {
        background: Color::TRANSPARENT.into(),
        dot_color: palette.primary.strong.color,
        border_width: 1.0,
        border_color: palette.primary.strong.color,
        text_color: None,
    };

    match status {
        Status::Active { .. } => active,
        Status::Hovered { .. } => Style {
            dot_color: palette.primary.strong.color,
            background: palette.primary.weak.color.into(),
            ..active
        },
    }
}
