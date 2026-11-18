//! 文本部件通过文字展示信息。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Text widgets display information through writing.
//!
//! </details>
//!
//! # 示例（Example）
//! ```no_run
//! # mod iced { pub mod widget { pub fn text<T>(t: T) -> iced_core::widget::Text<'static, iced_core::Theme, ()> { unimplemented!() } }
//! #            pub use iced_core::color; }
//! # pub type State = ();
//! # pub type Element<'a, Message> = iced_core::Element<'a, Message, iced_core::Theme, ()>;
//! use iced::widget::text;
//! use iced::color;
//!
//! enum Message {
//!     // ...
//! }
//!
//! fn view(state: &State) -> Element<'_, Message> {
//!     text("Hello, this is iced!")
//!         .size(20)
//!         .color(color!(0x0000ff))
//!         .into()
//! }
//! ```
use crate::alignment;
use crate::layout;
use crate::mouse;
use crate::renderer;
use crate::text;
use crate::text::paragraph::{self, Paragraph};
use crate::widget::tree::{self, Tree};
use crate::{
    Color, Element, Layout, Length, Pixels, Rectangle, Size, Theme, Widget,
};

pub use text::{Alignment, LineHeight, Shaping, Wrapping};

/// 一段文本。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A bunch of text.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub fn text<T>(t: T) -> iced_core::widget::Text<'static, iced_core::Theme, ()> { unimplemented!() } }
/// #            pub use iced_core::color; }
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_core::Element<'a, Message, iced_core::Theme, ()>;
/// use iced::widget::text;
/// use iced::color;
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     text("Hello, this is iced!")
///         .size(20)
///         .color(color!(0x0000ff))
///         .into()
/// }
/// ```
pub struct Text<'a, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: text::Renderer,
{
    fragment: text::Fragment<'a>,
    format: Format<Renderer::Font>,
    class: Theme::Class<'a>,
}

impl<'a, Theme, Renderer> Text<'a, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: text::Renderer,
{
    /// 使用给定的内容创建一个新的 [`Text`] 片段。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Create a new fragment of [`Text`] with the given contents.
    ///
    /// </details>
    pub fn new(fragment: impl text::IntoFragment<'a>) -> Self {
        Text {
            fragment: fragment.into_fragment(),
            format: Format::default(),
            class: Theme::default(),
        }
    }

    /// 设置 [`Text`] 的大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the size of the [`Text`].
    ///
    /// </details>
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.format.size = Some(size.into());
        self
    }

    /// 设置 [`Text`] 的 [`LineHeight`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`LineHeight`] of the [`Text`].
    ///
    /// </details>
    pub fn line_height(mut self, line_height: impl Into<LineHeight>) -> Self {
        self.format.line_height = line_height.into();
        self
    }

    /// 设置 [`Text`] 的 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Font`] of the [`Text`].
    ///
    /// </details>
    ///
    /// [`Font`]: crate::text::Renderer::Font
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.format.font = Some(font.into());
        self
    }

    /// 如果为 `Some`，设置 [`Text`] 的 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Font`] of the [`Text`], if `Some`.
    ///
    /// </details>
    ///
    /// [`Font`]: crate::text::Renderer::Font
    pub fn font_maybe(
        mut self,
        font: Option<impl Into<Renderer::Font>>,
    ) -> Self {
        self.format.font = font.map(Into::into);
        self
    }

    /// 设置 [`Text`] 边界的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Text`] boundaries.
    ///
    /// </details>
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.format.width = width.into();
        self
    }

    /// 设置 [`Text`] 边界的高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the height of the [`Text`] boundaries.
    ///
    /// </details>
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.format.height = height.into();
        self
    }

    /// 让 [`Text`] 在水平和垂直方向上都居中。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Centers the [`Text`], both horizontally and vertically.
    ///
    /// </details>
    pub fn center(self) -> Self {
        self.align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
    }

    /// 设置 [`Text`] 的 [`alignment::Horizontal`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`alignment::Horizontal`] of the [`Text`].
    ///
    /// </details>
    pub fn align_x(mut self, alignment: impl Into<text::Alignment>) -> Self {
        self.format.align_x = alignment.into();
        self
    }

    /// 设置 [`Text`] 的 [`alignment::Vertical`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`alignment::Vertical`] of the [`Text`].
    ///
    /// </details>
    pub fn align_y(
        mut self,
        alignment: impl Into<alignment::Vertical>,
    ) -> Self {
        self.format.align_y = alignment.into();
        self
    }

    /// 设置 [`Text`] 的 [`Shaping`] 策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Shaping`] strategy of the [`Text`].
    ///
    /// </details>
    pub fn shaping(mut self, shaping: Shaping) -> Self {
        self.format.shaping = shaping;
        self
    }

    /// 设置 [`Text`] 的 [`Wrapping`] 策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Wrapping`] strategy of the [`Text`].
    ///
    /// </details>
    pub fn wrapping(mut self, wrapping: Wrapping) -> Self {
        self.format.wrapping = wrapping;
        self
    }

    /// 设置 [`Text`] 的样式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style of the [`Text`].
    ///
    /// </details>
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// 设置 [`Text`] 的 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Color`] of the [`Text`].
    ///
    /// </details>
    pub fn color(self, color: impl Into<Color>) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.color_maybe(Some(color))
    }

    /// 如果为 `Some`，设置 [`Text`] 的 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Color`] of the [`Text`], if `Some`.
    ///
    /// </details>
    pub fn color_maybe(self, color: Option<impl Into<Color>>) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        let color = color.map(Into::into);

        self.style(move |_theme| Style { color })
    }

    /// 设置 [`Text`] 的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style class of the [`Text`].
    ///
    /// </details>
    #[cfg(feature = "advanced")]
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

/// [`Text`] 部件的内部状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The internal state of a [`Text`] widget.
///
/// </details>
pub type State<P> = paragraph::Plain<P>;

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Text<'_, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: text::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<Renderer::Paragraph>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(paragraph::Plain::<Renderer::Paragraph>::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.format.width,
            height: self.format.height,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout(
            tree.state.downcast_mut::<State<Renderer::Paragraph>>(),
            renderer,
            limits,
            &self.fragment,
            self.format,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        defaults: &renderer::Style,
        layout: Layout<'_>,
        _cursor_position: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State<Renderer::Paragraph>>();
        let style = theme.style(&self.class);

        draw(
            renderer,
            defaults,
            layout.bounds(),
            state.raw(),
            style,
            viewport,
        );
    }

    fn operate(
        &mut self,
        _tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &Renderer,
        operation: &mut dyn super::Operation,
    ) {
        operation.text(None, layout.bounds(), &self.fragment);
    }
}

/// [`Text`] 的格式。
///
/// 查看 [`Text`] 部件的各个方法，
/// 可以进一步了解每个字段。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The format of some [`Text`].
///
/// Check out the methods of the [`Text`] widget
/// to learn more about each field.
///
/// </details>
#[derive(Debug, Clone, Copy)]
#[allow(missing_docs)]
pub struct Format<Font> {
    pub width: Length,
    pub height: Length,
    pub size: Option<Pixels>,
    pub font: Option<Font>,
    pub line_height: LineHeight,
    pub align_x: text::Alignment,
    pub align_y: alignment::Vertical,
    pub shaping: Shaping,
    pub wrapping: Wrapping,
}

impl<Font> Default for Format<Font> {
    fn default() -> Self {
        Self {
            size: None,
            line_height: LineHeight::default(),
            font: None,
            width: Length::Shrink,
            height: Length::Shrink,
            align_x: text::Alignment::Default,
            align_y: alignment::Vertical::Top,
            shaping: Shaping::default(),
            wrapping: Wrapping::default(),
        }
    }
}

/// 生成 [`Text`] 部件的 [`layout::Node`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Produces the [`layout::Node`] of a [`Text`] widget.
///
/// </details>
pub fn layout<Renderer>(
    paragraph: &mut paragraph::Plain<Renderer::Paragraph>,
    renderer: &Renderer,
    limits: &layout::Limits,
    content: &str,
    format: Format<Renderer::Font>,
) -> layout::Node
where
    Renderer: text::Renderer,
{
    layout::sized(limits, format.width, format.height, |limits| {
        let bounds = limits.max();

        let size = format.size.unwrap_or_else(|| renderer.default_size());
        let font = format.font.unwrap_or_else(|| renderer.default_font());

        let _ = paragraph.update(text::Text {
            content,
            bounds,
            size,
            line_height: format.line_height,
            font,
            align_x: format.align_x,
            align_y: format.align_y,
            shaping: format.shaping,
            wrapping: format.wrapping,
        });

        paragraph.min_bounds()
    })
}

/// 使用与 [`Text`] 部件相同的逻辑绘制文本。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Draws text using the same logic as the [`Text`] widget.
///
/// </details>
pub fn draw<Renderer>(
    renderer: &mut Renderer,
    style: &renderer::Style,
    bounds: Rectangle,
    paragraph: &Renderer::Paragraph,
    appearance: Style,
    viewport: &Rectangle,
) where
    Renderer: text::Renderer,
{
    let anchor = bounds.anchor(
        paragraph.min_bounds(),
        paragraph.align_x(),
        paragraph.align_y(),
    );

    renderer.fill_paragraph(
        paragraph,
        anchor,
        appearance.color.unwrap_or(style.text_color),
        *viewport,
    );
}

impl<'a, Message, Theme, Renderer> From<Text<'a, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: text::Renderer + 'a,
{
    fn from(
        text: Text<'a, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(text)
    }
}

impl<'a, Theme, Renderer> From<&'a str> for Text<'a, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: text::Renderer,
{
    fn from(content: &'a str) -> Self {
        Self::new(content)
    }
}

impl<'a, Message, Theme, Renderer> From<&'a str>
    for Element<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: text::Renderer + 'a,
{
    fn from(content: &'a str) -> Self {
        Text::from(content).into()
    }
}

/// 文本的外观。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The appearance of some text.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Style {
    /// 文本的 [`Color`]。
    ///
    /// 默认值 `None` 表示使用继承的颜色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Color`] of the text.
    ///
    /// The default, `None`, means using the inherited color.
    ///
    /// </details>
    pub color: Option<Color>,
}

/// [`Text`] 的主题样式目录。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The theme catalog of a [`Text`].
///
/// </details>
pub trait Catalog: Sized {
    /// 此 [`Catalog`] 的条目类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The item class of this [`Catalog`].
    ///
    /// </details>
    type Class<'a>;

    /// 此 [`Catalog`] 产生的默认类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default class produced by this [`Catalog`].
    ///
    /// </details>
    fn default<'a>() -> Self::Class<'a>;

    /// 某个类在给定交互状态下的 [`Style`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Style`] of a class with the given status.
    ///
    /// </details>
    fn style(&self, item: &Self::Class<'_>) -> Style;
}

/// [`Text`] 的样式函数。
///
/// 它只是一个装箱的闭包：`Fn(&Theme, Status) -> Style`。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A styling function for a [`Text`].
///
/// This is just a boxed closure: `Fn(&Theme, Status) -> Style`.
///
/// </details>
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|_theme| Style::default())
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

/// 默认的文本样式；颜色是继承而来的。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The default text styling; color is inherited.
///
/// </details>
pub fn default(_theme: &Theme) -> Style {
    Style { color: None }
}

/// 使用默认基础色的文本。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Text with the default base color.
///
/// </details>
pub fn base(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().text),
    }
}

/// 传达重要信息的文本，例如一个动作。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Text conveying some important information, like an action.
///
/// </details>
pub fn primary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().primary),
    }
}

/// 传达次要信息的文本，例如脚注。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Text conveying some secondary information, like a footnote.
///
/// </details>
pub fn secondary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.extended_palette().secondary.base.color),
    }
}

/// 传达正面信息的文本，例如某个成功的事件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Text conveying some positive information, like a successful event.
///
/// </details>
pub fn success(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().success),
    }
}

/// 传达轻微负面信息的文本，例如警告。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Text conveying some mildly negative information, like a warning.
///
/// </details>
pub fn warning(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().warning),
    }
}

/// 传达负面信息的文本，例如错误。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Text conveying some negative information, like an error.
///
/// </details>
pub fn danger(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().danger),
    }
}
