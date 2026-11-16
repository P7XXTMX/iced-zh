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
//! # use iced::widget::Widget;
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
//! fn view(state: &State) -> impl Widget<Message> {
//!     text_editor(&state.content)
//!         .placeholder("Type something here...")
//!         .on_action(Message::Edit)
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
use crate::core::clipboard;
use crate::core::layout::{self, Layout};
use crate::core::mouse;
use crate::core::renderer;
use crate::core::text::editor::{self, Editor as _};
use crate::core::text::highlighter;
use crate::core::text::parser;
use crate::core::text::{self, LineHeight, Text, Wrapping};
use crate::core::theme;
use crate::core::widget::{self, Widget};
use crate::core::window;
use crate::core::{
    Background, Border, Color, Event, Font, Length, Padding, Pixels, Rectangle, Shell, Size, Theme,
};

use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt;
use std::ops::DerefMut;

pub use text::Highlighter;
pub use text::editor::{
    Action, Binding, Cursor, Edit, KeyPress, Line, LineEnding, Motion, Selection,
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
/// # use iced::widget::Widget;
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
/// fn view(state: &State) -> impl Widget<Message> {
///     text_editor(&state.content)
///         .placeholder("Type something here...")
///         .on_action(Message::Edit)
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
pub struct TextEditor<'a, Parser, Message, Theme = crate::Theme, Renderer = crate::Renderer>
where
    Parser: text::Parser,
    Theme: Catalog,
    Renderer: text::Renderer,
{
    id: Option<widget::Id>,
    content: &'a Content<Renderer>,
    placeholder: Option<text::Fragment<'a>>,
    font: Option<Font>,
    text_size: Option<Pixels>,
    line_height: Option<LineHeight>,
    width: Length,
    height: Length,
    padding: Padding,
    wrapping: Wrapping,
    class: Theme::Class<'a>,
    key_binding: Option<Box<dyn Fn(KeyPress) -> Option<Binding<Message>> + 'a>>,
    on_edit: Option<Box<dyn Fn(Action) -> Message + 'a>>,
    parser_settings: Parser::Settings,
    highlighter: Option<Box<dyn text::Highlighter<Parser::Output, Theme> + 'a>>,
    last_status: Option<Status>,
}

impl<'a, Message, Theme, Renderer> TextEditor<'a, parser::PlainText, Message, Theme, Renderer>
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
            line_height: None,
            width: Length::Fill,
            height: Length::Fit,
            padding: Padding::new(5.0),
            wrapping: Wrapping::default(),
            class: <Theme as Catalog>::default(),
            key_binding: None,
            on_edit: None,
            parser_settings: (),
            highlighter: None,
            last_status: None,
        }
    }
}

impl<'a, Message, Renderer> TextEditor<'a, parser::PlainText, Message, crate::Theme, Renderer>
where
    Renderer: text::Renderer,
{
    /// 使用给定的语法为 [`TextEditor`] 进行高亮。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Highlights the [`TextEditor`] using the given syntax.
    ///
    /// </details>
    ///
    /// ```no_run
    /// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
    /// # use iced::widget::Widget;
    /// #
    /// use iced::color;
    /// use iced::widget::text;
    /// use iced::widget::text_editor;
    /// use iced::Theme;
    ///
    /// struct State {
    ///    content: text_editor::Content,
    /// }
    ///
    /// fn view(state: &State) -> impl Widget<()> {
    ///     text_editor(&state.content)
    ///         .highlight("rust")
    /// }
    /// ```
    #[cfg(feature = "highlighter")]
    pub fn highlight(
        self,
        syntax: &str,
    ) -> TextEditor<'a, iced_highlighter::Parser, Message, crate::Theme, Renderer>
    where
        Renderer: text::Renderer,
    {
        self.highlight_with(
            iced_highlighter::Settings {
                token: syntax.to_owned(),
            },
            crate::core::Code::highlight,
        )
    }
}

impl<'a, Parser, Message, Theme, Renderer> TextEditor<'a, Parser, Message, Theme, Renderer>
where
    Parser: text::Parser,
    Theme: Catalog,
    Renderer: text::Renderer,
{
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

    /// 设置 [`TextEditor`] 的占位符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the placeholder of the [`TextEditor`].
    ///
    /// </details>
    pub fn placeholder(mut self, placeholder: impl text::IntoFragment<'a>) -> Self {
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
    pub fn on_action(mut self, on_edit: impl Fn(Action) -> Message + 'a) -> Self {
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
    /// [`Font`]: crate::core::Font
    pub fn font(mut self, font: impl Into<Font>) -> Self {
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
    pub fn line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.line_height = Some(line_height.into());
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

    /// 使用给定的 [`text::Parser`] 和 [`text::Highlighter`]
    /// 为 [`TextEditor`] 进行高亮。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Highlights the [`TextEditor`] with the given [`text::Parser`] and
    /// [`text::Highlighter`].
    ///
    /// </details>
    pub fn highlight_with<P: text::Parser>(
        self,
        settings: P::Settings,
        highlighter: impl text::Highlighter<P::Output, Theme> + 'a,
    ) -> TextEditor<'a, P, Message, Theme, Renderer> {
        TextEditor {
            id: self.id,
            content: self.content,
            placeholder: self.placeholder,
            font: self.font,
            text_size: self.text_size,
            line_height: self.line_height,
            width: self.width,
            height: self.height,
            padding: self.padding,
            wrapping: self.wrapping,
            class: self.class,
            key_binding: self.key_binding,
            on_edit: self.on_edit,
            parser_settings: settings,
            highlighter: Some(Box::new(highlighter)),
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
}

struct State<Parser: text::Parser> {
    editor: editor::State,
    parser: RefCell<Parser>,
    parser_settings: Parser::Settings,
    last_theme: RefCell<Option<String>>,
}

impl<Parser, Message, Theme, Renderer> widget::Meta
    for TextEditor<'_, Parser, Message, Theme, Renderer>
where
    Parser: text::Parser,
    Theme: Catalog,
    Renderer: text::Renderer,
{
}

impl<Parser, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for TextEditor<'_, Parser, Message, Theme, Renderer>
where
    Parser: text::Parser,
    Theme: Catalog,
    Renderer: text::Renderer,
{
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<State<Parser>>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(State {
            editor: editor::State::new(),
            parser: RefCell::new(Parser::new(&self.parser_settings)),
            parser_settings: self.parser_settings.clone(),
            last_theme: RefCell::new(None),
        })
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&mut self, tree: &mut widget::Tree, renderer: &Renderer, limits: &layout::Limits) {
        let mut internal = self.content.0.borrow_mut();
        let state = tree.state.downcast_mut::<State<Parser>>();

        if state.parser_settings != self.parser_settings {
            state.parser.borrow_mut().update(&self.parser_settings);

            state.parser_settings = self.parser_settings.clone();
        }

        let limits = limits
            .width(self.width)
            .height(self.height)
            .shrink(self.padding);

        internal.editor.update(
            limits.bounds(),
            self.font.unwrap_or_else(|| renderer.font()),
            self.text_size.unwrap_or_else(|| renderer.text_size()),
            self.line_height.unwrap_or_else(|| renderer.line_height()),
            self.wrapping,
            text::Alignment::Default,
            renderer.hint_factor(),
            state.parser.borrow_mut().deref_mut(),
        );

        let bounds = limits.resolve(self.width, self.height, internal.editor.min_bounds());

        tree.size = bounds.expand(self.padding);
    }

    fn update(
        &mut self,
        tree: &mut widget::Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let Some(on_edit) = self.on_edit.as_ref() else {
            return;
        };

        let state = tree.state.downcast_mut::<State<Parser>>();
        let is_redraw = matches!(event, Event::Window(window::Event::RedrawRequested(_now)),);

        let editor = &self.content.0.borrow().editor;

        fn apply_update<Message>(
            update: editor::Update<Message>,
            shell: &mut Shell<'_, Message>,
            on_edit: &impl Fn(editor::Action) -> Message,
        ) {
            match update {
                editor::Update::Action(action) => {
                    shell.publish(on_edit(action));
                    shell.capture_event();
                }
                editor::Update::Release => {}
                editor::Update::Custom(message) => {
                    shell.publish(message);
                    shell.capture_event();
                }
                editor::Update::Sequence(updates) => {
                    for update in updates {
                        apply_update(update, shell, on_edit);
                    }
                }
                editor::Update::Copy(content) => {
                    shell.write_clipboard(clipboard::Content::Text(content));
                    shell.capture_event();
                }
                editor::Update::Paste => {
                    shell.read_clipboard(clipboard::Kind::Text);
                    shell.capture_event();
                }
                editor::Update::RedrawAt(at) => {
                    shell.request_redraw_at(at);
                }
                editor::Update::Focus | editor::Update::Unfocus | editor::Update::InputMethod => {
                    shell.request_redraw();
                }
            }
        }

        if let Some(update) = state.editor.update(
            &self.content.0.borrow().editor,
            event,
            layout.bounds(),
            self.padding,
            cursor,
            self.key_binding
                .as_deref()
                .unwrap_or(&Binding::from_key_press as _),
        ) {
            apply_update(update, shell, on_edit);
        }

        let status = {
            let is_disabled = self.on_edit.is_none();
            let is_hovered = cursor.is_over(layout.bounds());

            if is_disabled {
                Status::Disabled
            } else if state.editor.is_focused() {
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
                &state
                    .editor
                    .input_method(editor, layout.bounds().shrink(self.padding).position()),
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
        layout: Layout,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        let mut internal = self.content.0.borrow_mut();
        let state = tree.state.downcast_ref::<State<Parser>>();

        let font = self.font.unwrap_or_else(|| renderer.font());

        let theme_name = theme.name();

        if state
            .last_theme
            .borrow()
            .as_ref()
            .is_none_or(|last_theme| last_theme != theme_name)
        {
            state.parser.borrow_mut().change_line(0);
            let _ = state.last_theme.borrow_mut().replace(theme_name.to_owned());
        }

        internal
            .editor
            .highlight(font, state.parser.borrow_mut().deref_mut(), |output| {
                let Some(highlighter) = &self.highlighter else {
                    return highlighter::Style::default();
                };

                highlighter.highlight(output, theme)
            });

        let style = theme.style(&self.class, self.last_status.unwrap_or(Status::Active));

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: style.border,
                ..renderer::Quad::default()
            },
            style.background,
        );

        let text_bounds = bounds.shrink(self.padding);

        if internal.editor.is_empty()
            && let Some(placeholder) = &self.placeholder
        {
            renderer.fill_text(
                Text {
                    content: placeholder.clone().into_owned(),
                    bounds: text_bounds.size(),
                    size: self.text_size.unwrap_or_else(|| renderer.text_size()),
                    line_height: self.line_height.unwrap_or_else(|| renderer.line_height()),
                    font,
                    align_x: text::Alignment::Default,
                    align_y: alignment::Vertical::Top,
                    shaping: text::Shaping::Advanced,
                    wrapping: self.wrapping,
                    ellipsis: text::Ellipsis::None,
                    hint_factor: renderer.hint_factor(),
                },
                text_bounds.position(),
                style.placeholder,
                text_bounds,
            );
        }

        state.editor.draw(
            &internal.editor,
            renderer,
            text_bounds.position(),
            *viewport,
            editor::Style {
                value: style.value,
                selection: style.selection,
            },
        );
    }

    fn mouse_interaction(
        &self,
        _tree: &widget::Tree,
        layout: Layout,
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
        layout: Layout,
        _viewport: &Rectangle,
        _renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        let state = tree.state.downcast_mut::<State<Parser>>();

        operation.focusable(self.id.as_ref(), layout.bounds(), &mut state.editor);
        operation.text_input(
            self.id.as_ref(),
            layout.bounds(),
            &mut self.content.0.borrow_mut().editor,
        );
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
        self.0.borrow().editor.text()
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
    let palette = theme.palette();

    let active = Style {
        background: Background::Color(palette.background.weakest.color),
        border: Border {
            radius: 2.0.into(),
            width: 1.0,
            color: palette.background.strong.color,
        },
        placeholder: palette.secondary.base.color,
        value: palette.background.weakest.text,
        selection: palette.background.strongest.color,
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
            background: Background::Color(palette.background.base.color),
            value: active.placeholder,
            placeholder: palette.background.strongest.color,
            ..active
        },
    }
}
