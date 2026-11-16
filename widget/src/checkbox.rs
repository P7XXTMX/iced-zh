//! 复选框可以让用户做出二元选择。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Checkboxes can be used to let users make binary choices.
//!
//! </details>
//!
//! # 示例（Example）
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
//! # use iced::widget::Widget;
//! #
//! use iced::widget::checkbox;
//!
//! struct State {
//!    is_checked: bool,
//! }
//!
//! enum Message {
//!     CheckboxToggled(bool),
//! }
//!
//! fn view(state: &State) -> impl Widget<Message> {
//!     checkbox(state.is_checked)
//!         .label("Toggle me!")
//!         .on_toggle(Message::CheckboxToggled)
//! }
//!
//! fn update(state: &mut State, message: Message) {
//!     match message {
//!         Message::CheckboxToggled(is_checked) => {
//!             state.is_checked = is_checked;
//!         }
//!     }
//! }
//! ```
//! ![由 `iced_wgpu` 绘制的复选框](https://github.com/iced-rs/iced/blob/7760618fb112074bc40b148944521f312152012a/docs/images/checkbox.png?raw=true)
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! ![Checkbox drawn by `iced_wgpu`](https://github.com/iced-rs/iced/blob/7760618fb112074bc40b148944521f312152012a/docs/images/checkbox.png?raw=true)
//!
//! </details>
use std::marker::PhantomData;

use crate::core::alignment;
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::text;
use crate::core::theme::palette;
use crate::core::touch;
use crate::core::widget;
use crate::core::widget::tree::{self, Tree};
use crate::core::window;
use crate::core::{
    Background, Border, Color, Event, Font, Layout, Length, Pixels, Rectangle, Shell, Size, Theme,
    Widget,
};

/// 可以被勾选的方框。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A box that can be checked.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// #
/// use iced::widget::checkbox;
///
/// struct State {
///    is_checked: bool,
/// }
///
/// enum Message {
///     CheckboxToggled(bool),
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     checkbox(state.is_checked)
///         .label("Toggle me!")
///         .on_toggle(Message::CheckboxToggled)
/// }
///
/// fn update(state: &mut State, message: Message) {
///     match message {
///         Message::CheckboxToggled(is_checked) => {
///             state.is_checked = is_checked;
///         }
///     }
/// }
/// ```
/// ![由 `iced_wgpu` 绘制的复选框](https://github.com/iced-rs/iced/blob/7760618fb112074bc40b148944521f312152012a/docs/images/checkbox.png?raw=true)
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// ![Checkbox drawn by `iced_wgpu`](https://github.com/iced-rs/iced/blob/7760618fb112074bc40b148944521f312152012a/docs/images/checkbox.png?raw=true)
///
/// </details>
pub struct Checkbox<'a, Message, Theme = crate::Theme, Renderer = crate::Renderer>
where
    Theme: Catalog,
    Renderer: text::Renderer,
{
    is_checked: bool,
    on_toggle: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    label: Option<text::Fragment<'a>>,
    width: Length,
    size: f32,
    spacing: f32,
    text_size: Option<Pixels>,
    line_height: Option<text::LineHeight>,
    shaping: text::Shaping,
    wrapping: text::Wrapping,
    font: Option<Font>,
    icon: Icon,
    class: Theme::Class<'a>,
    last_status: Option<Status>,
    renderer_: PhantomData<Renderer>,
}

impl<'a, Message, Theme, Renderer> Checkbox<'a, Message, Theme, Renderer>
where
    Renderer: text::Renderer,
    Theme: Catalog,
{
    /// [`Checkbox`] 的默认尺寸。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default size of a [`Checkbox`].
    ///
    /// </details>
    const DEFAULT_SIZE: f32 = 16.0;

    /// 创建一个新的 [`Checkbox`]。
    ///
    /// 它需要：
    ///   * 一个布尔值，描述 [`Checkbox`] 是否被勾选
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Checkbox`].
    ///
    /// It expects:
    ///   * a boolean describing whether the [`Checkbox`] is checked or not
    ///
    /// </details>
    pub fn new(is_checked: bool) -> Self {
        Checkbox {
            is_checked,
            on_toggle: None,
            label: None,
            width: Length::Fit,
            size: Self::DEFAULT_SIZE,
            spacing: Self::DEFAULT_SIZE / 2.0,
            text_size: None,
            line_height: None,
            shaping: text::Shaping::default(),
            wrapping: text::Wrapping::default(),
            font: None,
            icon: Icon {
                font: Renderer::ICON_FONT,
                code_point: Renderer::CHECKMARK_ICON,
                size: None,
                line_height: None,
                shaping: text::Shaping::Basic,
            },
            class: Theme::default(),
            last_status: None,
            renderer_: PhantomData,
        }
    }

    /// 设置 [`Checkbox`] 的标签。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the label of the [`Checkbox`].
    ///
    /// </details>
    pub fn label(mut self, label: impl text::IntoFragment<'a>) -> Self {
        self.label = Some(label.into_fragment());
        self
    }

    /// 设置 [`Checkbox`] 被切换时调用的函数。
    /// 它会收到 [`Checkbox`] 的新状态，并且必须产生一条
    /// `Message`。
    ///
    /// 除非调用了 `on_toggle`，否则 [`Checkbox`] 将处于禁用状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the function that will be called when the [`Checkbox`] is toggled.
    /// It will receive the new state of the [`Checkbox`] and must produce a
    /// `Message`.
    ///
    /// Unless `on_toggle` is called, the [`Checkbox`] will be disabled.
    ///
    /// </details>
    pub fn on_toggle<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(bool) -> Message,
    {
        self.on_toggle = Some(Box::new(f));
        self
    }

    /// 设置 [`Checkbox`] 被切换时调用的函数，
    /// 如果为 `Some` 的话。
    ///
    /// 如果为 `None`，复选框将处于禁用状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the function that will be called when the [`Checkbox`] is toggled,
    /// if `Some`.
    ///
    /// If `None`, the checkbox will be disabled.
    ///
    /// </details>
    pub fn on_toggle_maybe<F>(mut self, f: Option<F>) -> Self
    where
        F: Fn(bool) -> Message + 'a,
    {
        self.on_toggle = f.map(|f| Box::new(f) as _);
        self
    }

    /// 设置 [`Checkbox`] 的尺寸。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the size of the [`Checkbox`].
    ///
    /// </details>
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into().0;
        self
    }

    /// 设置 [`Checkbox`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Checkbox`].
    ///
    /// </details>
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// 设置 [`Checkbox`] 与文本之间的间距。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the spacing between the [`Checkbox`] and the text.
    ///
    /// </details>
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    /// 设置 [`Checkbox`] 的文本大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the text size of the [`Checkbox`].
    ///
    /// </details>
    pub fn text_size(mut self, text_size: impl Into<Pixels>) -> Self {
        self.text_size = Some(text_size.into());
        self
    }

    /// 设置 [`Checkbox`] 的文本 [`text::LineHeight`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the text [`text::LineHeight`] of the [`Checkbox`].
    ///
    /// </details>
    pub fn line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    /// 设置 [`Checkbox`] 的 [`text::Shaping`] 策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`text::Shaping`] strategy of the [`Checkbox`].
    ///
    /// </details>
    pub fn shaping(mut self, shaping: text::Shaping) -> Self {
        self.shaping = shaping;
        self
    }

    /// 设置 [`Checkbox`] 的 [`text::Wrapping`] 策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`text::Wrapping`] strategy of the [`Checkbox`].
    ///
    /// </details>
    pub fn wrapping(mut self, wrapping: text::Wrapping) -> Self {
        self.wrapping = wrapping;
        self
    }

    /// 设置 [`Checkbox`] 文本的 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Font`] of the text of the [`Checkbox`].
    ///
    /// </details>
    ///
    /// [`Font`]: crate::core::Font
    pub fn font(mut self, font: impl Into<Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// 设置 [`Checkbox`] 的 [`Icon`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Icon`] of the [`Checkbox`].
    ///
    /// </details>
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = icon;
        self
    }

    /// 设置 [`Checkbox`] 的样式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style of the [`Checkbox`].
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

    /// 设置 [`Checkbox`] 的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style class of the [`Checkbox`].
    ///
    /// </details>
    #[cfg(feature = "advanced")]
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<Message, Theme, Renderer> widget::Meta for Checkbox<'_, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: text::Renderer,
{
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Checkbox<'_, Message, Theme, Renderer>
where
    Renderer: text::Renderer,
    Theme: Catalog,
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
        // The children of the tree are the box and the label; they only
        // carry their geometry, so no state is needed.
        // 部件树的子节点是方框和标签；它们只
        // 携带各自的几何信息，因此不需要状态。
        tree.children.resize_with(2, Tree::empty);
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let limits = limits.width(self.width);

        let checkbox = Size::new(self.size, self.size);
        let spacing = if self.label.is_some() {
            self.spacing
        } else {
            0.0
        };

        let label = if let Some(label) = self.label.as_deref() {
            let state = tree
                .state
                .downcast_mut::<text::paragraph::Plain<Renderer::Paragraph>>();

            widget::text::layout(
                state,
                renderer,
                &limits.shrink(Size::new(checkbox.width + spacing, 0.0)),
                label,
                widget::text::Format {
                    width: self.width,
                    height: Length::Fit,
                    line_height: self.line_height,
                    size: self.text_size,
                    font: self.font,
                    align_x: text::Alignment::Default,
                    align_y: alignment::Vertical::Top,
                    shaping: self.shaping,
                    wrapping: self.wrapping,
                    ellipsis: text::Ellipsis::None,
                },
            )
        } else {
            Size::ZERO
        };

        layout::next_to_each_other(tree, checkbox, label, spacing);
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
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let mouse_over = cursor.is_over(layout.bounds());

                if mouse_over && let Some(on_toggle) = &self.on_toggle {
                    shell.publish((on_toggle)(!self.is_checked));
                    shell.capture_event();
                }
            }
            _ => {}
        }

        let current_status = {
            let is_mouse_over = cursor.is_over(layout.bounds());
            let is_disabled = self.on_toggle.is_none();
            let is_checked = self.is_checked;

            if is_disabled {
                Status::Disabled { is_checked }
            } else if is_mouse_over {
                Status::Hovered { is_checked }
            } else {
                Status::Active { is_checked }
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
        if cursor.is_over(layout.bounds()) && self.on_toggle.is_some() {
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
        layout: Layout,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let mut children = layout.iter(&tree.children);

        let style = theme.style(
            &self.class,
            self.last_status.unwrap_or(Status::Disabled {
                is_checked: self.is_checked,
            }),
        );

        {
            let (checkbox_layout, _) = children.next().unwrap();
            let bounds = checkbox_layout.bounds();

            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: style.border,
                    ..renderer::Quad::default()
                },
                style.background,
            );

            let Icon {
                font,
                code_point,
                size,
                line_height,
                shaping,
            } = &self.icon;
            let size = size.unwrap_or(Pixels(bounds.height * 0.7));
            let line_height = line_height.unwrap_or_else(|| renderer.line_height());

            if self.is_checked {
                renderer.fill_text(
                    text::Text {
                        content: code_point.to_string(),
                        font: *font,
                        size,
                        line_height,
                        bounds: bounds.size(),
                        align_x: text::Alignment::Center,
                        align_y: alignment::Vertical::Center,
                        shaping: *shaping,
                        wrapping: text::Wrapping::default(),
                        ellipsis: text::Ellipsis::default(),
                        hint_factor: None,
                    },
                    bounds.center(),
                    style.icon_color,
                    *viewport,
                );
            }
        }

        if self.label.is_none() {
            return;
        }

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

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        _viewport: &Rectangle,
        _renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        if self.label.is_none() {
            return;
        }

        let paragraph = tree
            .state
            .downcast_mut::<text::paragraph::Plain<Renderer::Paragraph>>();

        let mut children = layout.iter(&tree.children);
        let (layout, _) = children.next().unwrap();

        operation.text(
            None,
            layout.bounds(),
            &mut widget::text::Operand {
                paragraph,
                layout,
                selectable: true,
            },
        );
    }
}

/// [`Checkbox`] 中的图标。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The icon in a [`Checkbox`].
///
/// </details>
#[derive(Debug, Clone, PartialEq)]
pub struct Icon {
    /// 用来显示 `code_point` 的字体，
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Font that will be used to display the `code_point`,
    ///
    /// </details>
    pub font: Font,
    /// 用作图标的 Unicode 码点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The unicode code point that will be used as the icon.
    ///
    /// </details>
    pub code_point: char,
    /// 内容的字体大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Font size of the content.
    ///
    /// </details>
    pub size: Option<Pixels>,
    /// 图标的行高。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The line height of the icon.
    ///
    /// </details>
    pub line_height: Option<text::LineHeight>,
    /// 图标的文本整形策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The shaping strategy of the icon.
    ///
    /// </details>
    pub shaping: text::Shaping,
}

/// [`Checkbox`] 可能的交互状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The possible status of a [`Checkbox`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// [`Checkbox`] 可以交互。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Checkbox`] can be interacted with.
    ///
    /// </details>
    Active {
        /// 表示 [`Checkbox`] 当前是否被勾选。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Indicates if the [`Checkbox`] is currently checked.
        ///
        /// </details>
        is_checked: bool,
    },
    /// [`Checkbox`] 可以交互，并且正处于悬停状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Checkbox`] can be interacted with and it is being hovered.
    ///
    /// </details>
    Hovered {
        /// 表示 [`Checkbox`] 当前是否被勾选。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Indicates if the [`Checkbox`] is currently checked.
        ///
        /// </details>
        is_checked: bool,
    },
    /// [`Checkbox`] 不能交互。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Checkbox`] cannot be interacted with.
    ///
    /// </details>
    Disabled {
        /// 表示 [`Checkbox`] 当前是否被勾选。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Indicates if the [`Checkbox`] is currently checked.
        ///
        /// </details>
        is_checked: bool,
    },
}

/// 复选框的样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The style of a checkbox.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// 复选框的 [`Background`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Background`] of the checkbox.
    ///
    /// </details>
    pub background: Background,
    /// 复选框的图标 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The icon [`Color`] of the checkbox.
    ///
    /// </details>
    pub icon_color: Color,
    /// 复选框的 [`Border`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Border`] of the checkbox.
    ///
    /// </details>
    pub border: Border,
    /// 复选框的文本 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text [`Color`] of the checkbox.
    ///
    /// </details>
    pub text_color: Option<Color>,
}

/// [`Checkbox`] 的主题样式目录。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The theme catalog of a [`Checkbox`].
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

    /// 全局的选中 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The global selection [`Color`].
    ///
    /// </details>
    fn selection(&self) -> Color;
}

/// [`Checkbox`] 的样式函数。
///
/// 它只是一个装箱的闭包：`Fn(&Theme, Status) -> Style`。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A styling function for a [`Checkbox`].
///
/// This is just a boxed closure: `Fn(&Theme, Status) -> Style`.
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

    fn selection(&self) -> Color {
        self.palette().background.strongest.color
    }
}

/// 主要复选框；表示主要的切换项。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A primary checkbox; denoting a main toggle.
///
/// </details>
pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();

    match status {
        Status::Active { is_checked } => styled(
            palette.background.strong.color,
            palette.background.base,
            palette.primary.base.text,
            palette.primary.base,
            is_checked,
        ),
        Status::Hovered { is_checked } => styled(
            palette.background.strong.color,
            palette.background.weak,
            palette.primary.base.text,
            palette.primary.strong,
            is_checked,
        ),
        Status::Disabled { is_checked } => styled(
            palette.background.weak.color,
            palette.background.weaker,
            palette.primary.base.text,
            palette.background.strong,
            is_checked,
        ),
    }
}

/// 次要复选框；表示补充性的切换项。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A secondary checkbox; denoting a complementary toggle.
///
/// </details>
pub fn secondary(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();

    match status {
        Status::Active { is_checked } => styled(
            palette.background.strong.color,
            palette.background.base,
            palette.background.base.text,
            palette.background.strong,
            is_checked,
        ),
        Status::Hovered { is_checked } => styled(
            palette.background.strong.color,
            palette.background.weak,
            palette.background.base.text,
            palette.background.strong,
            is_checked,
        ),
        Status::Disabled { is_checked } => styled(
            palette.background.weak.color,
            palette.background.weak,
            palette.background.base.text,
            palette.background.weak,
            is_checked,
        ),
    }
}

/// 成功复选框；表示正面的切换项。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A success checkbox; denoting a positive toggle.
///
/// </details>
pub fn success(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();

    match status {
        Status::Active { is_checked } => styled(
            palette.background.weak.color,
            palette.background.base,
            palette.success.base.text,
            palette.success.base,
            is_checked,
        ),
        Status::Hovered { is_checked } => styled(
            palette.background.strong.color,
            palette.background.weak,
            palette.success.base.text,
            palette.success.strong,
            is_checked,
        ),
        Status::Disabled { is_checked } => styled(
            palette.background.weak.color,
            palette.background.weak,
            palette.success.base.text,
            palette.success.weak,
            is_checked,
        ),
    }
}

/// 危险复选框；表示负面的切换项。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A danger checkbox; denoting a negative toggle.
///
/// </details>
pub fn danger(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();

    match status {
        Status::Active { is_checked } => styled(
            palette.background.strong.color,
            palette.background.base,
            palette.danger.base.text,
            palette.danger.base,
            is_checked,
        ),
        Status::Hovered { is_checked } => styled(
            palette.background.strong.color,
            palette.background.weak,
            palette.danger.base.text,
            palette.danger.strong,
            is_checked,
        ),
        Status::Disabled { is_checked } => styled(
            palette.background.weak.color,
            palette.background.weak,
            palette.danger.base.text,
            palette.danger.weak,
            is_checked,
        ),
    }
}

fn styled(
    border_color: Color,
    base: palette::Pair,
    icon_color: Color,
    accent: palette::Pair,
    is_checked: bool,
) -> Style {
    let (background, border) = if is_checked {
        (accent, accent.color)
    } else {
        (base, border_color)
    };

    Style {
        background: Background::Color(background.color),
        icon_color,
        border: Border {
            radius: 2.0.into(),
            width: 1.0,
            color: border,
        },
        text_color: None,
    }
}
