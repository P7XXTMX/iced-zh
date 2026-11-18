//! 文本编辑器显示一个用于编辑文本的多行文本输入框。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Text editors display a multi-line text input for text editing.
//!
//! </details>
//!
//! # 示例（Example）
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
//! # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
//! #
//! use iced::widget::text_editor;
//!
//! struct State {
//!    content: text_editor::Content,
//! }
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Edit(text_editor::Action)
//! }
//!
//! fn view(state: &State) -> Element<'_, Message> {
//!     text_editor(&state.content)
//!         .placeholder("Type something here...")
//!         .on_action(Message::Edit)
//!         .into()
//! }
//!
//! fn update(state: &mut State, message: Message) {
//!     match message {
//!         Message::Edit(action) => {
//!             state.content.perform(action);
//!         }
//!     }
//! }
//! ```
use crate::core::alignment;
use crate::core::clipboard::{self, Clipboard};
use crate::core::input_method;
use crate::core::keyboard;
use crate::core::keyboard::key;
use crate::core::layout::{self, Layout};
use crate::core::mouse;
use crate::core::renderer;
use crate::core::text::editor::Editor as _;
use crate::core::text::highlighter::{self, Highlighter};
use crate::core::text::{self, LineHeight, Text, Wrapping};
use crate::core::theme;
use crate::core::time::{Duration, Instant};
use crate::core::widget::operation;
use crate::core::widget::{self, Widget};
use crate::core::window;
use crate::core::{
    Background, Border, Color, Element, Event, InputMethod, Length, Padding,
    Pixels, Point, Rectangle, Shell, Size, SmolStr, Theme, Vector,
};

use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt;
use std::ops;
use std::ops::DerefMut;
use std::sync::Arc;

pub use text::editor::{
    Action, Cursor, Edit, Line, LineEnding, Motion, Position, Selection,
};

/// 多行文本输入框。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A multi-line text input.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// #
/// use iced::widget::text_editor;
///
/// struct State {
///    content: text_editor::Content,
/// }
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Edit(text_editor::Action)
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     text_editor(&state.content)
///         .placeholder("Type something here...")
///         .on_action(Message::Edit)
///         .into()
/// }
///
/// fn update(state: &mut State, message: Message) {
///     match message {
///         Message::Edit(action) => {
///             state.content.perform(action);
///         }
///     }
/// }
/// ```
pub struct TextEditor<
    'a,
    Highlighter,
    Message,
    Theme = crate::Theme,
    Renderer = crate::Renderer,
> where
    Highlighter: text::Highlighter,
    Theme: Catalog,
    Renderer: text::Renderer,
{
    id: Option<widget::Id>,
    content: &'a Content<Renderer>,
    placeholder: Option<text::Fragment<'a>>,
    font: Option<Renderer::Font>,
    text_size: Option<Pixels>,
    line_height: LineHeight,
    width: Length,
    height: Length,
    min_height: f32,
    max_height: f32,
    padding: Padding,
    wrapping: Wrapping,
    class: Theme::Class<'a>,
    key_binding: Option<Box<dyn Fn(KeyPress) -> Option<Binding<Message>> + 'a>>,
    on_edit: Option<Box<dyn Fn(Action) -> Message + 'a>>,
    highlighter_settings: Highlighter::Settings,
    highlighter_format: fn(
        &Highlighter::Highlight,
        &Theme,
    ) -> highlighter::Format<Renderer::Font>,
    last_status: Option<Status>,
}

impl<'a, Message, Theme, Renderer>
    TextEditor<'a, highlighter::PlainText, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: text::Renderer,
{
    /// 使用给定的 [`Content`] 创建一个新的 [`TextEditor`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates new [`TextEditor`] with the given [`Content`].
    ///
    /// </details>
    pub fn new(content: &'a Content<Renderer>) -> Self {
        Self {
            id: None,
            content,
            placeholder: None,
            font: None,
            text_size: None,
            line_height: LineHeight::default(),
            width: Length::Fill,
            height: Length::Shrink,
            min_height: 0.0,
            max_height: f32::INFINITY,
            padding: Padding::new(5.0),
            wrapping: Wrapping::default(),
            class: <Theme as Catalog>::default(),
            key_binding: None,
            on_edit: None,
            highlighter_settings: (),
            highlighter_format: |_highlight, _theme| {
                highlighter::Format::default()
            },
            last_status: None,
        }
    }

    /// 设置 [`TextEditor`] 的 [`Id`](widget::Id)。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Id`](widget::Id) of the [`TextEditor`].
    ///
    /// </details>
    pub fn id(mut self, id: impl Into<widget::Id>) -> Self {
        self.id = Some(id.into());
        self
    }
}

impl<'a, Highlighter, Message, Theme, Renderer>
    TextEditor<'a, Highlighter, Message, Theme, Renderer>
where
    Highlighter: text::Highlighter,
    Theme: Catalog,
    Renderer: text::Renderer,
{
    /// 设置 [`TextEditor`] 的占位符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the placeholder of the [`TextEditor`].
    ///
    /// </details>
    pub fn placeholder(
        mut self,
        placeholder: impl text::IntoFragment<'a>,
    ) -> Self {
        self.placeholder = Some(placeholder.into_fragment());
        self
    }

    /// 设置 [`TextEditor`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`TextEditor`].
    ///
    /// </details>
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Length::from(width.into());
        self
    }

    /// 设置 [`TextEditor`] 的高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the height of the [`TextEditor`].
    ///
    /// </details>
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// 设置 [`TextEditor`] 的最小高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the minimum height of the [`TextEditor`].
    ///
    /// </details>
    pub fn min_height(mut self, min_height: impl Into<Pixels>) -> Self {
        self.min_height = min_height.into().0;
        self
    }

    /// 设置 [`TextEditor`] 的最大高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the maximum height of the [`TextEditor`].
    ///
    /// </details>
    pub fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
        self.max_height = max_height.into().0;
        self
    }

    /// 设置在 [`TextEditor`] 中执行某个动作时
    /// 应该产生的消息。
    ///
    /// 如果没有调用这个方法，[`TextEditor`] 会被禁用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the message that should be produced when some action is performed in
    /// the [`TextEditor`].
    ///
    /// If this method is not called, the [`TextEditor`] will be disabled.
    ///
    /// </details>
    pub fn on_action(
        mut self,
        on_edit: impl Fn(Action) -> Message + 'a,
    ) -> Self {
        self.on_edit = Some(Box::new(on_edit));
        self
    }

    /// 设置 [`TextEditor`] 的 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Font`] of the [`TextEditor`].
    ///
    /// </details>
    ///
    /// [`Font`]: text::Renderer::Font
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// 设置 [`TextEditor`] 的文本字号。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the text size of the [`TextEditor`].
    ///
    /// </details>
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.text_size = Some(size.into());
        self
    }

    /// 设置 [`TextEditor`] 的 [`text::LineHeight`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`text::LineHeight`] of the [`TextEditor`].
    ///
    /// </details>
    pub fn line_height(
        mut self,
        line_height: impl Into<text::LineHeight>,
    ) -> Self {
        self.line_height = line_height.into();
        self
    }

    /// 设置 [`TextEditor`] 的 [`Padding`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Padding`] of the [`TextEditor`].
    ///
    /// </details>
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// 设置 [`TextEditor`] 的 [`Wrapping`] 策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Wrapping`] strategy of the [`TextEditor`].
    ///
    /// </details>
    pub fn wrapping(mut self, wrapping: Wrapping) -> Self {
        self.wrapping = wrapping;
        self
    }

    /// 使用给定的语法和主题高亮 [`TextEditor`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Highlights the [`TextEditor`] using the given syntax and theme.
    ///
    /// </details>
    #[cfg(feature = "highlighter")]
    pub fn highlight(
        self,
        syntax: &str,
        theme: iced_highlighter::Theme,
    ) -> TextEditor<'a, iced_highlighter::Highlighter, Message, Theme, Renderer>
    where
        Renderer: text::Renderer<Font = crate::core::Font>,
    {
        self.highlight_with::<iced_highlighter::Highlighter>(
            iced_highlighter::Settings {
                theme,
                token: syntax.to_owned(),
            },
            |highlight, _theme| highlight.to_format(),
        )
    }

    /// 使用给定的 [`Highlighter`]，以及一个把它的高亮结果
    /// 转换成某种文本格式的策略，高亮 [`TextEditor`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Highlights the [`TextEditor`] with the given [`Highlighter`] and
    /// a strategy to turn its highlights into some text format.
    ///
    /// </details>
    pub fn highlight_with<H: text::Highlighter>(
        self,
        settings: H::Settings,
        to_format: fn(
            &H::Highlight,
            &Theme,
        ) -> highlighter::Format<Renderer::Font>,
    ) -> TextEditor<'a, H, Message, Theme, Renderer> {
        TextEditor {
            id: self.id,
            content: self.content,
            placeholder: self.placeholder,
            font: self.font,
            text_size: self.text_size,
            line_height: self.line_height,
            width: self.width,
            height: self.height,
            min_height: self.min_height,
            max_height: self.max_height,
            padding: self.padding,
            wrapping: self.wrapping,
            class: self.class,
            key_binding: self.key_binding,
            on_edit: self.on_edit,
            highlighter_settings: settings,
            highlighter_format: to_format,
            last_status: self.last_status,
        }
    }

    /// 设置在按键时产生按键绑定的闭包。
    ///
    /// 可用的绑定列表见 [`Binding`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the closure to produce key bindings on key presses.
    ///
    /// See [`Binding`] for the list of available bindings.
    ///
    /// </details>
    pub fn key_binding(
        mut self,
        key_binding: impl Fn(KeyPress) -> Option<Binding<Message>> + 'a,
    ) -> Self {
        self.key_binding = Some(Box::new(key_binding));
        self
    }

    /// 设置 [`TextEditor`] 的样式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style of the [`TextEditor`].
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

    /// 设置 [`TextEditor`] 的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style class of the [`TextEditor`].
    ///
    /// </details>
    #[cfg(feature = "advanced")]
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    fn input_method<'b>(
        &self,
        state: &'b State<Highlighter>,
        renderer: &Renderer,
        layout: Layout<'_>,
    ) -> InputMethod<&'b str> {
        let Some(Focus {
            is_window_focused: true,
            ..
        }) = &state.focus
        else {
            return InputMethod::Disabled;
        };

        let bounds = layout.bounds();
        let internal = self.content.0.borrow_mut();

        let text_bounds = bounds.shrink(self.padding);
        let translation = text_bounds.position() - Point::ORIGIN;

        let cursor = match internal.editor.selection() {
            Selection::Caret(position) => position,
            Selection::Range(ranges) => {
                ranges.first().cloned().unwrap_or_default().position()
            }
        };

        let line_height = self.line_height.to_absolute(
            self.text_size.unwrap_or_else(|| renderer.default_size()),
        );

        let position = cursor + translation;

        InputMethod::Enabled {
            cursor: Rectangle::new(
                position,
                Size::new(1.0, f32::from(line_height)),
            ),
            purpose: input_method::Purpose::Normal,
            preedit: state.preedit.as_ref().map(input_method::Preedit::as_ref),
        }
    }
}

/// [`TextEditor`] 的内容。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The content of a [`TextEditor`].
///
/// </details>
pub struct Content<R = crate::Renderer>(RefCell<Internal<R>>)
where
    R: text::Renderer;

struct Internal<R>
where
    R: text::Renderer,
{
    editor: R::Editor,
}

impl<R> Content<R>
where
    R: text::Renderer,
{
    /// 创建一个空的 [`Content`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates an empty [`Content`].
    ///
    /// </details>
    pub fn new() -> Self {
        Self::with_text("")
    }

    /// 使用给定的文本创建一个 [`Content`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Content`] with the given text.
    ///
    /// </details>
    pub fn with_text(text: &str) -> Self {
        Self(RefCell::new(Internal {
            editor: R::Editor::with_text(text),
        }))
    }

    /// 对 [`Content`] 执行一个 [`Action`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Performs an [`Action`] on the [`Content`].
    ///
    /// </details>
    pub fn perform(&mut self, action: Action) {
        let internal = self.0.get_mut();

        internal.editor.perform(action);
    }

    /// 移动当前光标，使其与给定的光标一致。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Moves the current cursor to reflect the given one.
    ///
    /// </details>
    pub fn move_to(&mut self, cursor: Cursor) {
        let internal = self.0.get_mut();

        internal.editor.move_to(cursor);
    }

    /// 返回 [`Content`] 当前的光标位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current cursor position of the [`Content`].
    ///
    /// </details>
    pub fn cursor(&self) -> Cursor {
        self.0.borrow().editor.cursor()
    }

    /// 返回 [`Content`] 的行数。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the amount of lines of the [`Content`].
    ///
    /// </details>
    pub fn line_count(&self) -> usize {
        self.0.borrow().editor.line_count()
    }

    /// 返回给定索引处那一行的文本（如果存在）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the text of the line at the given index, if it exists.
    ///
    /// </details>
    pub fn line(&self, index: usize) -> Option<Line<'_>> {
        let internal = self.0.borrow();
        let line = internal.editor.line(index)?;

        Some(Line {
            text: Cow::Owned(line.text.into_owned()),
            ending: line.ending,
        })
    }

    /// 返回一个遍历 [`Content`] 中各行文本的迭代器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns an iterator of the text of the lines in the [`Content`].
    ///
    /// </details>
    pub fn lines(&self) -> impl Iterator<Item = Line<'_>> {
        (0..)
            .map(|i| self.line(i))
            .take_while(Option::is_some)
            .flatten()
    }

    /// 返回 [`Content`] 的文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the text of the [`Content`].
    ///
    /// </details>
    pub fn text(&self) -> String {
        let mut contents = String::new();
        let mut lines = self.lines().peekable();

        while let Some(line) = lines.next() {
            contents.push_str(&line.text);

            if lines.peek().is_some() {
                contents.push_str(if line.ending == LineEnding::None {
                    LineEnding::default().as_str()
                } else {
                    line.ending.as_str()
                });
            }
        }

        contents
    }

    /// 返回 [`Content`] 中被选中的文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the selected text of the [`Content`].
    ///
    /// </details>
    pub fn selection(&self) -> Option<String> {
        self.0.borrow().editor.copy()
    }

    /// 返回 [`Content`] 中用于分隔各行的 [`LineEnding`] 种类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the kind of [`LineEnding`] used for separating lines in the [`Content`].
    ///
    /// </details>
    pub fn line_ending(&self) -> Option<LineEnding> {
        Some(self.line(0)?.ending)
    }

    /// 返回 [`Content`] 是否为空。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns whether or not the the [`Content`] is empty.
    ///
    /// </details>
    pub fn is_empty(&self) -> bool {
        self.0.borrow().editor.is_empty()
    }
}

impl<Renderer> Clone for Content<Renderer>
where
    Renderer: text::Renderer,
{
    fn clone(&self) -> Self {
        Self::with_text(&self.text())
    }
}

impl<Renderer> Default for Content<Renderer>
where
    Renderer: text::Renderer,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Renderer> fmt::Debug for Content<Renderer>
where
    Renderer: text::Renderer,
    Renderer::Editor: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let internal = self.0.borrow();

        f.debug_struct("Content")
            .field("editor", &internal.editor)
            .finish()
    }
}

/// [`TextEditor`] 的状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The state of a [`TextEditor`].
///
/// </details>
#[derive(Debug)]
pub struct State<Highlighter: text::Highlighter> {
    focus: Option<Focus>,
    preedit: Option<input_method::Preedit>,
    last_click: Option<mouse::Click>,
    drag_click: Option<mouse::click::Kind>,
    partial_scroll: f32,
    last_theme: RefCell<Option<String>>,
    highlighter: RefCell<Highlighter>,
    highlighter_settings: Highlighter::Settings,
    highlighter_format_address: usize,
}

#[derive(Debug, Clone)]
struct Focus {
    updated_at: Instant,
    now: Instant,
    is_window_focused: bool,
}

impl Focus {
    const CURSOR_BLINK_INTERVAL_MILLIS: u128 = 500;

    fn now() -> Self {
        let now = Instant::now();

        Self {
            updated_at: now,
            now,
            is_window_focused: true,
        }
    }

    fn is_cursor_visible(&self) -> bool {
        self.is_window_focused
            && ((self.now - self.updated_at).as_millis()
                / Self::CURSOR_BLINK_INTERVAL_MILLIS)
                .is_multiple_of(2)
    }
}

impl<Highlighter: text::Highlighter> State<Highlighter> {
    /// 返回 [`TextEditor`] 当前是否获得焦点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns whether the [`TextEditor`] is currently focused or not.
    ///
    /// </details>
    pub fn is_focused(&self) -> bool {
        self.focus.is_some()
    }
}

impl<Highlighter: text::Highlighter> operation::Focusable
    for State<Highlighter>
{
    fn is_focused(&self) -> bool {
        self.focus.is_some()
    }

    fn focus(&mut self) {
        self.focus = Some(Focus::now());
    }

    fn unfocus(&mut self) {
        self.focus = None;
    }
}

impl<Highlighter, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for TextEditor<'_, Highlighter, Message, Theme, Renderer>
where
    Highlighter: text::Highlighter,
    Theme: Catalog,
    Renderer: text::Renderer,
{
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<State<Highlighter>>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(State {
            focus: None,
            preedit: None,
            last_click: None,
            drag_click: None,
            partial_scroll: 0.0,
            last_theme: RefCell::default(),
            highlighter: RefCell::new(Highlighter::new(
                &self.highlighter_settings,
            )),
            highlighter_settings: self.highlighter_settings.clone(),
            highlighter_format_address: self.highlighter_format as usize,
        })
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &mut self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> iced_renderer::core::layout::Node {
        let mut internal = self.content.0.borrow_mut();
        let state = tree.state.downcast_mut::<State<Highlighter>>();

        if state.highlighter_format_address != self.highlighter_format as usize
        {
            state.highlighter.borrow_mut().change_line(0);

            state.highlighter_format_address = self.highlighter_format as usize;
        }

        if state.highlighter_settings != self.highlighter_settings {
            state
                .highlighter
                .borrow_mut()
                .update(&self.highlighter_settings);

            state.highlighter_settings = self.highlighter_settings.clone();
        }

        let limits = limits
            .width(self.width)
            .height(self.height)
            .min_height(self.min_height)
            .max_height(self.max_height);

        internal.editor.update(
            limits.shrink(self.padding).max(),
            self.font.unwrap_or_else(|| renderer.default_font()),
            self.text_size.unwrap_or_else(|| renderer.default_size()),
            self.line_height,
            self.wrapping,
            state.highlighter.borrow_mut().deref_mut(),
        );

        match self.height {
            Length::Fill | Length::FillPortion(_) | Length::Fixed(_) => {
                layout::Node::new(limits.max())
            }
            Length::Shrink => {
                let min_bounds = internal.editor.min_bounds();

                layout::Node::new(
                    limits
                        .height(min_bounds.height)
                        .max()
                        .expand(Size::new(0.0, self.padding.y())),
                )
            }
        }
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
        _viewport: &Rectangle,
    ) {
        let Some(on_edit) = self.on_edit.as_ref() else {
            return;
        };

        let state = tree.state.downcast_mut::<State<Highlighter>>();
        let is_redraw = matches!(
            event,
            Event::Window(window::Event::RedrawRequested(_now)),
        );

        match event {
            Event::Window(window::Event::Unfocused) => {
                if let Some(focus) = &mut state.focus {
                    focus.is_window_focused = false;
                }
            }
            Event::Window(window::Event::Focused) => {
                if let Some(focus) = &mut state.focus {
                    focus.is_window_focused = true;
                    focus.updated_at = Instant::now();

                    shell.request_redraw();
                }
            }
            Event::Window(window::Event::RedrawRequested(now)) => {
                if let Some(focus) = &mut state.focus
                    && focus.is_window_focused
                {
                    focus.now = *now;

                    let millis_until_redraw =
                        Focus::CURSOR_BLINK_INTERVAL_MILLIS
                            - (focus.now - focus.updated_at).as_millis()
                                % Focus::CURSOR_BLINK_INTERVAL_MILLIS;

                    shell.request_redraw_at(
                        focus.now
                            + Duration::from_millis(millis_until_redraw as u64),
                    );
                }
            }
            _ => {}
        }

        if let Some(update) = Update::from_event(
            event,
            state,
            layout.bounds(),
            self.padding,
            cursor,
            self.key_binding.as_deref(),
        ) {
            match update {
                Update::Click(click) => {
                    let action = match click.kind() {
                        mouse::click::Kind::Single => {
                            Action::Click(click.position())
                        }
                        mouse::click::Kind::Double => Action::SelectWord,
                        mouse::click::Kind::Triple => Action::SelectLine,
                    };

                    state.focus = Some(Focus::now());
                    state.last_click = Some(click);
                    state.drag_click = Some(click.kind());

                    shell.publish(on_edit(action));
                    shell.capture_event();
                }
                Update::Drag(position) => {
                    shell.publish(on_edit(Action::Drag(position)));
                }
                Update::Release => {
                    state.drag_click = None;
                }
                Update::Scroll(lines) => {
                    let bounds = self.content.0.borrow().editor.bounds();

                    if bounds.height >= i32::MAX as f32 {
                        return;
                    }

                    let lines = lines + state.partial_scroll;
                    state.partial_scroll = lines.fract();

                    shell.publish(on_edit(Action::Scroll {
                        lines: lines as i32,
                    }));
                    shell.capture_event();
                }
                Update::InputMethod(update) => match update {
                    Ime::Toggle(is_open) => {
                        state.preedit =
                            is_open.then(input_method::Preedit::new);

                        shell.request_redraw();
                    }
                    Ime::Preedit { content, selection } => {
                        state.preedit = Some(input_method::Preedit {
                            content,
                            selection,
                            text_size: self.text_size,
                        });

                        shell.request_redraw();
                    }
                    Ime::Commit(text) => {
                        shell.publish(on_edit(Action::Edit(Edit::Paste(
                            Arc::new(text),
                        ))));
                    }
                },
                Update::Binding(binding) => {
                    fn apply_binding<
                        H: text::Highlighter,
                        R: text::Renderer,
                        Message,
                    >(
                        binding: Binding<Message>,
                        content: &Content<R>,
                        state: &mut State<H>,
                        on_edit: &dyn Fn(Action) -> Message,
                        clipboard: &mut dyn Clipboard,
                        shell: &mut Shell<'_, Message>,
                    ) {
                        let mut publish =
                            |action| shell.publish(on_edit(action));

                        match binding {
                            Binding::Unfocus => {
                                state.focus = None;
                                state.drag_click = None;
                            }
                            Binding::Copy => {
                                if let Some(selection) = content.selection() {
                                    clipboard.write(
                                        clipboard::Kind::Standard,
                                        selection,
                                    );
                                }
                            }
                            Binding::Cut => {
                                if let Some(selection) = content.selection() {
                                    clipboard.write(
                                        clipboard::Kind::Standard,
                                        selection,
                                    );

                                    publish(Action::Edit(Edit::Delete));
                                }
                            }
                            Binding::Paste => {
                                if let Some(contents) =
                                    clipboard.read(clipboard::Kind::Standard)
                                {
                                    publish(Action::Edit(Edit::Paste(
                                        Arc::new(contents),
                                    )));
                                }
                            }
                            Binding::Move(motion) => {
                                publish(Action::Move(motion));
                            }
                            Binding::Select(motion) => {
                                publish(Action::Select(motion));
                            }
                            Binding::SelectWord => {
                                publish(Action::SelectWord);
                            }
                            Binding::SelectLine => {
                                publish(Action::SelectLine);
                            }
                            Binding::SelectAll => {
                                publish(Action::SelectAll);
                            }
                            Binding::Insert(c) => {
                                publish(Action::Edit(Edit::Insert(c)));
                            }
                            Binding::Enter => {
                                publish(Action::Edit(Edit::Enter));
                            }
                            Binding::Backspace => {
                                publish(Action::Edit(Edit::Backspace));
                            }
                            Binding::Delete => {
                                publish(Action::Edit(Edit::Delete));
                            }
                            Binding::Sequence(sequence) => {
                                for binding in sequence {
                                    apply_binding(
                                        binding, content, state, on_edit,
                                        clipboard, shell,
                                    );
                                }
                            }
                            Binding::Custom(message) => {
                                shell.publish(message);
                            }
                        }
                    }

                    if !matches!(binding, Binding::Unfocus) {
                        shell.capture_event();
                    }

                    apply_binding(
                        binding,
                        self.content,
                        state,
                        on_edit,
                        clipboard,
                        shell,
                    );

                    if let Some(focus) = &mut state.focus {
                        focus.updated_at = Instant::now();
                    }
                }
            }
        }

        let status = {
            let is_disabled = self.on_edit.is_none();
            let is_hovered = cursor.is_over(layout.bounds());

            if is_disabled {
                Status::Disabled
            } else if state.focus.is_some() {
                Status::Focused { is_hovered }
            } else if is_hovered {
                Status::Hovered
            } else {
                Status::Active
            }
        };

        if is_redraw {
            self.last_status = Some(status);

            shell.request_input_method(
                &self.input_method(state, renderer, layout),
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
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _defaults: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        let mut internal = self.content.0.borrow_mut();
        let state = tree.state.downcast_ref::<State<Highlighter>>();

        let font = self.font.unwrap_or_else(|| renderer.default_font());

        let theme_name = theme.name();

        if state
            .last_theme
            .borrow()
            .as_ref()
            .is_none_or(|last_theme| last_theme != theme_name)
        {
            state.highlighter.borrow_mut().change_line(0);
            let _ =
                state.last_theme.borrow_mut().replace(theme_name.to_owned());
        }

        internal.editor.highlight(
            font,
            state.highlighter.borrow_mut().deref_mut(),
            |highlight| (self.highlighter_format)(highlight, theme),
        );

        let style = theme
            .style(&self.class, self.last_status.unwrap_or(Status::Active));

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: style.border,
                ..renderer::Quad::default()
            },
            style.background,
        );

        let text_bounds = bounds.shrink(self.padding);

        if internal.editor.is_empty() {
            if let Some(placeholder) = self.placeholder.clone() {
                renderer.fill_text(
                    Text {
                        content: placeholder.into_owned(),
                        bounds: text_bounds.size(),
                        size: self
                            .text_size
                            .unwrap_or_else(|| renderer.default_size()),
                        line_height: self.line_height,
                        font,
                        align_x: text::Alignment::Default,
                        align_y: alignment::Vertical::Top,
                        shaping: text::Shaping::Advanced,
                        wrapping: self.wrapping,
                    },
                    text_bounds.position(),
                    style.placeholder,
                    text_bounds,
                );
            }
        } else {
            renderer.fill_editor(
                &internal.editor,
                text_bounds.position(),
                style.value,
                text_bounds,
            );
        }

        let translation = text_bounds.position() - Point::ORIGIN;

        if let Some(focus) = state.focus.as_ref() {
            match internal.editor.selection() {
                Selection::Caret(position) if focus.is_cursor_visible() => {
                    let cursor =
                        Rectangle::new(
                            position + translation,
                            Size::new(
                                1.0,
                                self.line_height
                                    .to_absolute(self.text_size.unwrap_or_else(
                                        || renderer.default_size(),
                                    ))
                                    .into(),
                            ),
                        );

                    if let Some(clipped_cursor) =
                        text_bounds.intersection(&cursor)
                    {
                        renderer.fill_quad(
                            renderer::Quad {
                                bounds: clipped_cursor,
                                ..renderer::Quad::default()
                            },
                            style.value,
                        );
                    }
                }
                Selection::Range(ranges) => {
                    for range in ranges.into_iter().filter_map(|range| {
                        text_bounds.intersection(&(range + translation))
                    }) {
                        renderer.fill_quad(
                            renderer::Quad {
                                bounds: range,
                                ..renderer::Quad::default()
                            },
                            style.selection,
                        );
                    }
                }
                Selection::Caret(_) => {}
            }
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &widget::Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let is_disabled = self.on_edit.is_none();

        if cursor.is_over(layout.bounds()) {
            if is_disabled {
                mouse::Interaction::NotAllowed
            } else {
                mouse::Interaction::Text
            }
        } else {
            mouse::Interaction::default()
        }
    }

    fn operate(
        &mut self,
        tree: &mut widget::Tree,
        layout: Layout<'_>,
        _renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        let state = tree.state.downcast_mut::<State<Highlighter>>();

        operation.focusable(self.id.as_ref(), layout.bounds(), state);
    }
}

impl<'a, Highlighter, Message, Theme, Renderer>
    From<TextEditor<'a, Highlighter, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Highlighter: text::Highlighter,
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: text::Renderer,
{
    fn from(
        text_editor: TextEditor<'a, Highlighter, Message, Theme, Renderer>,
    ) -> Self {
        Self::new(text_editor)
    }
}

/// 到 [`TextEditor`] 中某个动作的绑定。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A binding to an action in the [`TextEditor`].
///
/// </details>
#[derive(Debug, Clone, PartialEq)]
pub enum Binding<Message> {
    /// 让 [`TextEditor`] 失去焦点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Unfocus the [`TextEditor`].
    ///
    /// </details>
    Unfocus,
    /// 复制 [`TextEditor`] 的选区。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Copy the selection of the [`TextEditor`].
    ///
    /// </details>
    Copy,
    /// 剪切 [`TextEditor`] 的选区。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Cut the selection of the [`TextEditor`].
    ///
    /// </details>
    Cut,
    /// 把剪贴板内容粘贴到 [`TextEditor`] 中。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Paste the clipboard contents in the [`TextEditor`].
    ///
    /// </details>
    Paste,
    /// 应用一个 [`Motion`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Apply a [`Motion`].
    ///
    /// </details>
    Move(Motion),
    /// 使用给定的 [`Motion`] 选择文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Select text with a given [`Motion`].
    ///
    /// </details>
    Select(Motion),
    /// 选择当前光标处的单词。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Select the word at the current cursor.
    ///
    /// </details>
    SelectWord,
    /// 选择当前光标处的行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Select the line at the current cursor.
    ///
    /// </details>
    SelectLine,
    /// 选择整个缓冲区。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Select the entire buffer.
    ///
    /// </details>
    SelectAll,
    /// 插入给定的字符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Insert the given character.
    ///
    /// </details>
    Insert(char),
    /// 在当前行换行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Break the current line.
    ///
    /// </details>
    Enter,
    /// 删除前一个字符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Delete the previous character.
    ///
    /// </details>
    Backspace,
    /// 删除后一个字符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Delete the next character.
    ///
    /// </details>
    Delete,
    /// 要依次执行的一系列绑定。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A sequence of bindings to execute.
    ///
    /// </details>
    Sequence(Vec<Self>),
    /// 产生给定的消息。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Produce the given message.
    ///
    /// </details>
    Custom(Message),
}

/// 一次按键。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A key press.
///
/// </details>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyPress {
    /// 按下的原始按键，未应用修饰键。
    ///
    /// 组合键（例如 Ctrl+C）应当使用这个按键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The original key pressed without modifiers applied to it.
    ///
    /// You should use this key for combinations (e.g. Ctrl+C).
    ///
    /// </details>
    pub key: keyboard::Key,
    /// 按下的按键，已应用修饰键。
    ///
    /// 任何单键绑定（例如光标移动）都应当使用这个按键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The key pressed with modifiers applied to it.
    ///
    /// You should use this key for any single key bindings (e.g. motions).
    ///
    /// </details>
    pub modified_key: keyboard::Key,
    /// 按下的物理按键。
    ///
    /// 与键盘布局无关的绑定应当使用这个按键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The physical key pressed.
    ///
    /// You should use this key for layout-independent bindings.
    ///
    /// </details>
    pub physical_key: keyboard::key::Physical,
    /// 键盘修饰键的状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The state of the keyboard modifiers.
    ///
    /// </details>
    pub modifiers: keyboard::Modifiers,
    /// 这次按键产生的文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text produced by the key press.
    ///
    /// </details>
    pub text: Option<SmolStr>,
    /// [`TextEditor`] 当前的 [`Status`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The current [`Status`] of the [`TextEditor`].
    ///
    /// </details>
    pub status: Status,
}

impl<Message> Binding<Message> {
    /// 返回给定按键对应的默认 [`Binding`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the default [`Binding`] for the given key press.
    ///
    /// </details>
    pub fn from_key_press(event: KeyPress) -> Option<Self> {
        let KeyPress {
            key,
            modified_key,
            physical_key,
            modifiers,
            text,
            status,
        } = event;

        if !matches!(status, Status::Focused { .. }) {
            return None;
        }

        let combination = match key.to_latin(physical_key) {
            Some('c') if modifiers.command() => Some(Self::Copy),
            Some('x') if modifiers.command() => Some(Self::Cut),
            Some('v') if modifiers.command() && !modifiers.alt() => {
                Some(Self::Paste)
            }
            Some('a') if modifiers.command() => Some(Self::SelectAll),
            _ => None,
        };

        if let Some(binding) = combination {
            return Some(binding);
        }

        #[cfg(target_os = "macos")]
        let modified_key =
            convert_macos_shortcut(&key, modifiers).unwrap_or(modified_key);

        match modified_key.as_ref() {
            keyboard::Key::Named(key::Named::Enter) => Some(Self::Enter),
            keyboard::Key::Named(key::Named::Backspace) => {
                Some(Self::Backspace)
            }
            keyboard::Key::Named(key::Named::Delete)
                if text.is_none() || text.as_deref() == Some("\u{7f}") =>
            {
                Some(Self::Delete)
            }
            keyboard::Key::Named(key::Named::Escape) => Some(Self::Unfocus),
            _ => {
                if let Some(text) = text {
                    let c = text.chars().find(|c| !c.is_control())?;

                    Some(Self::Insert(c))
                } else if let keyboard::Key::Named(named_key) = key.as_ref() {
                    let motion = motion(named_key)?;

                    let motion = if modifiers.macos_command() {
                        match motion {
                            Motion::Left => Motion::Home,
                            Motion::Right => Motion::End,
                            _ => motion,
                        }
                    } else {
                        motion
                    };

                    let motion = if modifiers.jump() {
                        motion.widen()
                    } else {
                        motion
                    };

                    Some(if modifiers.shift() {
                        Self::Select(motion)
                    } else {
                        Self::Move(motion)
                    })
                } else {
                    None
                }
            }
        }
    }
}

enum Update<Message> {
    Click(mouse::Click),
    Drag(Point),
    Release,
    Scroll(f32),
    InputMethod(Ime),
    Binding(Binding<Message>),
}

enum Ime {
    Toggle(bool),
    Preedit {
        content: String,
        selection: Option<ops::Range<usize>>,
    },
    Commit(String),
}

impl<Message> Update<Message> {
    fn from_event<H: Highlighter>(
        event: &Event,
        state: &State<H>,
        bounds: Rectangle,
        padding: Padding,
        cursor: mouse::Cursor,
        key_binding: Option<&dyn Fn(KeyPress) -> Option<Binding<Message>>>,
    ) -> Option<Self> {
        let binding = |binding| Some(Update::Binding(binding));

        match event {
            Event::Mouse(event) => match event {
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    if let Some(cursor_position) = cursor.position_in(bounds) {
                        let cursor_position = cursor_position
                            - Vector::new(padding.left, padding.top);

                        let click = mouse::Click::new(
                            cursor_position,
                            mouse::Button::Left,
                            state.last_click,
                        );

                        Some(Update::Click(click))
                    } else if state.focus.is_some() {
                        binding(Binding::Unfocus)
                    } else {
                        None
                    }
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    Some(Update::Release)
                }
                mouse::Event::CursorMoved { .. } => match state.drag_click {
                    Some(mouse::click::Kind::Single) => {
                        let cursor_position = cursor.position_in(bounds)?
                            - Vector::new(padding.left, padding.top);

                        Some(Update::Drag(cursor_position))
                    }
                    _ => None,
                },
                mouse::Event::WheelScrolled { delta }
                    if cursor.is_over(bounds) =>
                {
                    Some(Update::Scroll(match delta {
                        mouse::ScrollDelta::Lines { y, .. } => {
                            if y.abs() > 0.0 {
                                y.signum() * -(y.abs() * 4.0).max(1.0)
                            } else {
                                0.0
                            }
                        }
                        mouse::ScrollDelta::Pixels { y, .. } => -y / 4.0,
                    }))
                }
                _ => None,
            },
            Event::InputMethod(event) => match event {
                input_method::Event::Opened | input_method::Event::Closed => {
                    Some(Update::InputMethod(Ime::Toggle(matches!(
                        event,
                        input_method::Event::Opened
                    ))))
                }
                input_method::Event::Preedit(content, selection)
                    if state.focus.is_some() =>
                {
                    Some(Update::InputMethod(Ime::Preedit {
                        content: content.clone(),
                        selection: selection.clone(),
                    }))
                }
                input_method::Event::Commit(content)
                    if state.focus.is_some() =>
                {
                    Some(Update::InputMethod(Ime::Commit(content.clone())))
                }
                _ => None,
            },
            Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                modified_key,
                physical_key,
                modifiers,
                text,
                ..
            }) => {
                let status = if state.focus.is_some() {
                    Status::Focused {
                        is_hovered: cursor.is_over(bounds),
                    }
                } else {
                    Status::Active
                };

                let key_press = KeyPress {
                    key: key.clone(),
                    modified_key: modified_key.clone(),
                    physical_key: *physical_key,
                    modifiers: *modifiers,
                    text: text.clone(),
                    status,
                };

                if let Some(key_binding) = key_binding {
                    key_binding(key_press)
                } else {
                    Binding::from_key_press(key_press)
                }
                .map(Self::Binding)
            }
            _ => None,
        }
    }
}

fn motion(key: key::Named) -> Option<Motion> {
    match key {
        key::Named::ArrowLeft => Some(Motion::Left),
        key::Named::ArrowRight => Some(Motion::Right),
        key::Named::ArrowUp => Some(Motion::Up),
        key::Named::ArrowDown => Some(Motion::Down),
        key::Named::Home => Some(Motion::Home),
        key::Named::End => Some(Motion::End),
        key::Named::PageUp => Some(Motion::PageUp),
        key::Named::PageDown => Some(Motion::PageDown),
        _ => None,
    }
}

/// [`TextEditor`] 可能的交互状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The possible status of a [`TextEditor`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// [`TextEditor`] 可以交互。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`TextEditor`] can be interacted with.
    ///
    /// </details>
    Active,
    /// [`TextEditor`] 正处于悬停状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`TextEditor`] is being hovered.
    ///
    /// </details>
    Hovered,
    /// [`TextEditor`] 已获得焦点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`TextEditor`] is focused.
    ///
    /// </details>
    Focused {
        /// [`TextEditor`] 在获得焦点的同时是否处于悬停状态。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Whether the [`TextEditor`] is hovered, while focused.
        ///
        /// </details>
        is_hovered: bool,
    },
    /// [`TextEditor`] 无法交互。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`TextEditor`] cannot be interacted with.
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

/// [`TextEditor`] 的主题样式目录。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The theme catalog of a [`TextEditor`].
///
/// </details>
pub trait Catalog: theme::Base {
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

/// [`TextEditor`] 的样式函数。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A styling function for a [`TextEditor`].
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

/// [`TextEditor`] 的默认样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The default style of a [`TextEditor`].
///
/// </details>
pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let active = Style {
        background: Background::Color(palette.background.base.color),
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
                color: palette.background.base.text,
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
            placeholder: palette.background.strongest.color,
            ..active
        },
    }
}

#[cfg(target_os = "macos")]
pub(crate) fn convert_macos_shortcut(
    key: &keyboard::Key,
    modifiers: keyboard::Modifiers,
) -> Option<keyboard::Key> {
    if modifiers != keyboard::Modifiers::CTRL {
        return None;
    }

    let key = match key.as_ref() {
        keyboard::Key::Character("b") => key::Named::ArrowLeft,
        keyboard::Key::Character("f") => key::Named::ArrowRight,
        keyboard::Key::Character("a") => key::Named::Home,
        keyboard::Key::Character("e") => key::Named::End,
        keyboard::Key::Character("h") => key::Named::Backspace,
        keyboard::Key::Character("d") => key::Named::Delete,
        _ => return None,
    };

    Some(keyboard::Key::Named(key))
}
