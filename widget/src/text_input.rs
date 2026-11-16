//! 文本输入框显示可以填入文本的字段。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Text inputs display fields that can be filled with text.
//!
//! </details>
//!
//! # 示例（Example）
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
//! # use iced::widget::Widget;
//! #
//! use iced::widget::text_input;
//!
//! struct State {
//!    content: String,
//! }
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     ContentChanged(String)
//! }
//!
//! fn view(state: &State) -> impl Widget<Message> {
//!     text_input("Type something here...", &state.content)
//!         .on_input(Message::ContentChanged)
//! }
//!
//! fn update(state: &mut State, message: Message) {
//!     match message {
//!         Message::ContentChanged(content) => {
//!             state.content = content;
//!         }
//!     }
//! }
//! ```
use crate::core::keyboard;
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::shell;
use crate::core::text;
use crate::core::text::editor;
use crate::core::text::input;
use crate::core::widget;
use crate::core::widget::operation::{self, Focusable, Operation};
use crate::core::widget::tree::{self, Tree};
use crate::core::window;
use crate::core::{
    Background, Border, Color, Event, Font, Layout, Length, Padding, Pixels, Rectangle, Shell,
    Size, Theme, Widget,
};

/// 可以填入文本的字段。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A field that can be filled with text.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// #
/// use iced::widget::text_input;
///
/// struct State {
///    content: String,
/// }
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     ContentChanged(String)
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     text_input("Type something here...", &state.content)
///         .on_input(Message::ContentChanged)
/// }
///
/// fn update(state: &mut State, message: Message) {
///     match message {
///         Message::ContentChanged(content) => {
///             state.content = content;
///         }
///     }
/// }
/// ```
pub struct TextInput<'a, Message, Theme = crate::Theme>
where
    Theme: Catalog,
{
    id: Option<widget::Id>,
    placeholder: text::Fragment<'a>,
    value: text::Fragment<'a>,
    is_secure: bool,
    font: Option<Font>,
    width: Length,
    height: Length,
    padding: Padding,
    size: Option<Pixels>,
    line_height: Option<text::LineHeight>,
    alignment: text::Alignment,
    multiline: Option<text::Wrapping>,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_paste: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_submit: Option<Message>,
    class: Theme::Class<'a>,
    last_status: Option<Status>,
}

/// [`TextInput`] 的默认 [`Padding`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The default [`Padding`] of a [`TextInput`].
///
/// </details>
pub const DEFAULT_PADDING: Padding = Padding::new(5.0);

impl<'a, Message, Theme> TextInput<'a, Message, Theme>
where
    Message: Clone,
    Theme: Catalog,
{
    /// 使用给定的占位符和当前值创建一个新的
    /// [`TextInput`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`TextInput`] with the given placeholder and
    /// its current value.
    ///
    /// </details>
    pub fn new(
        placeholder: impl text::IntoFragment<'a>,
        value: impl text::IntoFragment<'a>,
    ) -> Self {
        TextInput {
            id: None,
            placeholder: placeholder.into_fragment(),
            value: value.into_fragment(),
            is_secure: false,
            font: None,
            width: Length::Fill,
            height: Length::Fit,
            padding: DEFAULT_PADDING,
            size: None,
            line_height: None,
            alignment: text::Alignment::Default,
            multiline: None,
            on_input: None,
            on_paste: None,
            on_submit: None,
            class: Theme::default(),
            last_status: None,
        }
    }

    /// 设置 [`TextInput`] 的 [`widget::Id`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`widget::Id`] of the [`TextInput`].
    ///
    /// </details>
    pub fn id(mut self, id: impl Into<widget::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// 把 [`TextInput`] 转换为安全的密码输入框。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Converts the [`TextInput`] into a secure password input.
    ///
    /// </details>
    pub fn secure(mut self, is_secure: bool) -> Self {
        self.is_secure = is_secure;
        self
    }

    /// 设置在 [`TextInput`] 中输入文本时
    /// 应该产生的消息。
    ///
    /// 如果没有调用这个方法，[`TextInput`] 会被禁用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the message that should be produced when some text is typed into
    /// the [`TextInput`].
    ///
    /// If this method is not called, the [`TextInput`] will be disabled.
    ///
    /// </details>
    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'a) -> Self {
        self.on_input = Some(Box::new(on_input));
        self
    }

    /// 如果为 `Some`，设置在 [`TextInput`] 中输入文本时
    /// 应该产生的消息。
    ///
    /// 如果为 `None`，[`TextInput`] 会被禁用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the message that should be produced when some text is typed into
    /// the [`TextInput`], if `Some`.
    ///
    /// If `None`, the [`TextInput`] will be disabled.
    ///
    /// </details>
    pub fn on_input_maybe(mut self, on_input: Option<impl Fn(String) -> Message + 'a>) -> Self {
        self.on_input = on_input.map(|f| Box::new(f) as _);
        self
    }

    /// 设置 [`TextInput`] 获得焦点并且按下回车键时
    /// 应该产生的消息。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the message that should be produced when the [`TextInput`] is
    /// focused and the enter key is pressed.
    ///
    /// </details>
    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }

    /// 如果为 `Some`，设置 [`TextInput`] 获得焦点并且
    /// 按下回车键时应该产生的消息。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the message that should be produced when the [`TextInput`] is
    /// focused and the enter key is pressed, if `Some`.
    ///
    /// </details>
    pub fn on_submit_maybe(mut self, on_submit: Option<Message>) -> Self {
        self.on_submit = on_submit;
        self
    }

    /// 设置向 [`TextInput`] 中粘贴文本时
    /// 应该产生的消息。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the message that should be produced when some text is pasted into
    /// the [`TextInput`].
    ///
    /// </details>
    pub fn on_paste(mut self, on_paste: impl Fn(String) -> Message + 'a) -> Self {
        self.on_paste = Some(Box::new(on_paste));
        self
    }

    /// 如果为 `Some`，设置向 [`TextInput`] 中粘贴文本时
    /// 应该产生的消息。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the message that should be produced when some text is pasted into
    /// the [`TextInput`], if `Some`.
    ///
    /// </details>
    pub fn on_paste_maybe(mut self, on_paste: Option<impl Fn(String) -> Message + 'a>) -> Self {
        self.on_paste = on_paste.map(|f| Box::new(f) as _);
        self
    }

    /// 设置 [`TextInput`] 的 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Font`] of the [`TextInput`].
    ///
    /// </details>
    ///
    /// [`Font`]: crate::core::Font
    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    /// 设置 [`TextInput`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`TextInput`].
    ///
    /// </details>
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// 设置 [`TextInput`] 的 [`Padding`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Padding`] of the [`TextInput`].
    ///
    /// </details>
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// 设置 [`TextInput`] 的文本字号。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the text size of the [`TextInput`].
    ///
    /// </details>
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// 设置 [`TextInput`] 的 [`text::LineHeight`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`text::LineHeight`] of the [`TextInput`].
    ///
    /// </details>
    pub fn line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    /// 设置 [`TextInput`] 的水平对齐方式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the horizontal alignment of the [`TextInput`].
    ///
    /// </details>
    pub fn align_x(mut self, alignment: impl Into<text::Alignment>) -> Self {
        self.alignment = alignment.into();
        self
    }

    /// 设置 [`TextInput`] 的多行行为。
    ///
    /// `None` 表示作为单行输入框。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the multiline behavior of the [`TextInput`].
    ///
    /// `None` will behave as a single line input.
    ///
    /// </details>
    pub fn multiline(mut self, wrapping: Option<text::Wrapping>) -> Self {
        self.multiline = wrapping;
        self
    }

    /// 设置 [`TextInput`] 的样式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style of the [`TextInput`].
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

    /// 设置 [`TextInput`] 的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style class of the [`TextInput`].
    ///
    /// </details>
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<Message, Theme> widget::Meta for TextInput<'_, Message, Theme> where Theme: Catalog {}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for TextInput<'_, Message, Theme>
where
    Message: Clone,
    Theme: Catalog,
    Renderer: text::Renderer + 'static,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<Renderer>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::<Renderer>::new())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Fit,
        }
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let state = tree.state.downcast_mut::<State<Renderer>>();

        if state.value != self.value
            && state
                .transaction
                .as_ref()
                .is_none_or(shell::Tracking::is_processed)
        {
            state.input.overwrite(self.value.as_ref());
            state.value = self.value.clone().into_owned();
        }

        tree.size = state.input.layout(
            renderer,
            limits,
            input::Layout {
                width: self.width,
                height: self.height,
                padding: self.padding,
                placeholder: self.placeholder.as_ref(),
                font: self.font,
                size: self.size,
                line_height: self.line_height,
                alignment: self.alignment,
                multiline: self.multiline,
                is_secure: self.is_secure,
            },
        );
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        _viewport: &Rectangle,
        _renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let state = tree.state.downcast_mut::<State<Renderer>>();

        operation.text_input(self.id.as_ref(), layout.bounds(), state);
        operation.focusable(self.id.as_ref(), layout.bounds(), state);
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = state::<Renderer>(tree);
        let is_disabled = self.on_input.is_none();

        if let Some(on_input) = &self.on_input {
            let edit = state
                .input
                .update(event, layout.bounds(), cursor, shell, |key_press| {
                    if let Some(on_submit) = &self.on_submit
                        && key_press.is_focused
                        && key_press.modified_key
                            == keyboard::Key::Named(keyboard::key::Named::Enter)
                    {
                        return Some(editor::Binding::Custom(on_submit.clone()));
                    }

                    editor::Binding::from_key_press(key_press)
                });

            if let Some(edit) = edit {
                let on_input = if let Some(on_paste) = &self.on_paste
                    && edit.has_pasted
                {
                    on_paste
                } else {
                    on_input
                };

                state.value = state.input.value();
                state.transaction = Some(shell.publish_and_track(on_input(state.value.clone())));
            }
        }

        let status = if is_disabled {
            Status::Disabled
        } else if state.input.is_focused() {
            Status::Focused {
                is_hovered: cursor.is_over(layout.bounds()),
            }
        } else if cursor.is_over(layout.bounds()) {
            Status::Hovered
        } else {
            Status::Active
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.last_status = Some(status);

            shell.request_input_method(
                &state
                    .input
                    .input_method(layout.bounds().shrink(self.padding).position()),
            );
        } else if self
            .last_status
            .is_some_and(|last_status| status != last_status)
        {
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
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State<Renderer>>();
        let style = theme.style(&self.class, self.last_status.unwrap_or(Status::Disabled));
        let bounds = layout.bounds();

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: style.border,
                ..renderer::Quad::default()
            },
            style.background,
        );

        state.input.draw(
            renderer,
            bounds,
            *viewport,
            input::Style {
                value: style.value,
                selection: style.selection,
                placeholder: style.placeholder,
            },
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
        if cursor.is_over(layout.bounds()) {
            if self.on_input.is_none() {
                mouse::Interaction::Idle
            } else {
                mouse::Interaction::Text
            }
        } else {
            mouse::Interaction::default()
        }
    }
}

/// [`TextInput`] 的状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The state of a [`TextInput`].
///
/// </details>
struct State<R: text::Renderer> {
    input: text::Input<R>,
    value: String,
    transaction: Option<shell::Tracking>,
}

fn state<Renderer: text::Renderer + 'static>(tree: &mut Tree) -> &mut State<Renderer> {
    tree.state.downcast_mut::<State<Renderer>>()
}

impl<R: text::Renderer> State<R> {
    /// 创建一个新的 [`State`]，表示一个未获得焦点的 [`TextInput`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`State`], representing an unfocused [`TextInput`].
    ///
    /// </details>
    fn new() -> Self {
        Self {
            input: text::Input::new(),
            value: String::new(),
            transaction: None,
        }
    }
}

impl<R: text::Renderer> operation::Focusable for State<R> {
    fn is_focused(&self) -> bool {
        self.input.is_focused()
    }

    fn focus(&mut self) {
        self.input.focus();
    }

    fn unfocus(&mut self) {
        self.input.unfocus();
    }
}

impl<R: text::Renderer> operation::TextInput for State<R> {
    fn text(&self) -> text::Fragment<'_> {
        if self.input.is_empty() {
            text::Fragment::Borrowed(self.input.placeholder())
        } else {
            text::Fragment::Owned(self.input.value())
        }
    }

    fn move_cursor_to_front(&mut self) {
        self.input.move_cursor_to_front();
    }

    fn move_cursor_to_end(&mut self) {
        self.input.move_cursor_to_end();
    }

    fn move_cursor_to(&mut self, position: text::Position) {
        self.input.move_cursor_to(position);
    }

    fn select_all(&mut self) {
        self.input.select_all();
    }

    fn select_range(&mut self, start: text::Position, end: text::Position) {
        self.input.select_range(start, end);
    }
}

/// [`TextInput`] 可能的交互状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The possible status of a [`TextInput`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// [`TextInput`] 可以交互。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`TextInput`] can be interacted with.
    ///
    /// </details>
    Active,
    /// [`TextInput`] 正处于悬停状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`TextInput`] is being hovered.
    ///
    /// </details>
    Hovered,
    /// [`TextInput`] 已获得焦点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`TextInput`] is focused.
    ///
    /// </details>
    Focused {
        /// [`TextInput`] 在获得焦点的同时是否处于悬停状态。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Whether the [`TextInput`] is hovered, while focused.
        ///
        /// </details>
        is_hovered: bool,
    },
    /// [`TextInput`] 无法交互。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`TextInput`] cannot be interacted with.
    ///
    /// </details>
    Disabled,
}

/// 文本输入框的外观。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The appearance of a text input.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// 文本输入框的 [`Background`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Background`] of the text input.
    ///
    /// </details>
    pub background: Background,
    /// 文本输入框的 [`Border`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Border`] of the text input.
    ///
    /// </details>
    pub border: Border,
    /// 文本输入框占位符的 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Color`] of the placeholder of the text input.
    ///
    /// </details>
    pub placeholder: Color,
    /// 文本输入框值的 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Color`] of the value of the text input.
    ///
    /// </details>
    pub value: Color,
    /// 文本输入框选区的 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Color`] of the selection of the text input.
    ///
    /// </details>
    pub selection: Color,
}

/// [`TextInput`] 的主题样式目录。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The theme catalog of a [`TextInput`].
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
}

/// [`TextInput`] 的样式函数。
///
/// 它只是一个装箱的闭包：`Fn(&Theme, Status) -> Style`。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A styling function for a [`TextInput`].
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
}

/// [`TextInput`] 的默认样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The default style of a [`TextInput`].
///
/// </details>
pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();

    let active = Style {
        background: Background::Color(palette.background.weakest.color),
        border: Border {
            radius: 2.0.into(),
            width: 1.0,
            color: palette.background.strong.color,
        },
        placeholder: palette.secondary.base.color,
        value: palette.background.base.text,
        selection: palette.primary.weak.color,
    };

    match status {
        Status::Active => active,
        Status::Hovered => Style {
            border: Border {
                color: palette.background.weakest.text,
                ..active.border
            },
            ..active
        },
        Status::Focused { .. } => Style {
            border: Border {
                color: palette.primary.strong.color,
                ..active.border
            },
            ..active
        },
        Status::Disabled => Style {
            background: Background::Color(palette.background.weak.color),
            value: active.placeholder,
            placeholder: palette.background.base.color,
            ..active
        },
    }
}
