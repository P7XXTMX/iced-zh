use crate::core::alignment;
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::text::{Paragraph, Span};
use crate::core::widget::text::{
    self, Alignment, Catalog, LineHeight, Shaping, Style, StyleFn, Wrapping,
};
use crate::core::widget::tree::{self, Tree};
use crate::core::{
    self, Clipboard, Color, Element, Event, Layout, Length, Pixels, Point,
    Rectangle, Shell, Size, Vector, Widget,
};

/// 一段 [`Rich`] 文本。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A bunch of [`Rich`] text.
///
/// </details>
pub struct Rich<
    'a,
    Link,
    Message,
    Theme = crate::Theme,
    Renderer = crate::Renderer,
> where
    Link: Clone + 'static,
    Theme: Catalog,
    Renderer: core::text::Renderer,
{
    spans: Box<dyn AsRef<[Span<'a, Link, Renderer::Font>]> + 'a>,
    size: Option<Pixels>,
    line_height: LineHeight,
    width: Length,
    height: Length,
    font: Option<Renderer::Font>,
    align_x: Alignment,
    align_y: alignment::Vertical,
    wrapping: Wrapping,
    class: Theme::Class<'a>,
    hovered_link: Option<usize>,
    on_link_click: Option<Box<dyn Fn(Link) -> Message + 'a>>,
}

impl<'a, Link, Message, Theme, Renderer>
    Rich<'a, Link, Message, Theme, Renderer>
where
    Link: Clone + 'static,
    Theme: Catalog,
    Renderer: core::text::Renderer,
    Renderer::Font: 'a,
{
    /// 创建一个新的空 [`Rich`] 文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new empty [`Rich`] text.
    ///
    /// </details>
    pub fn new() -> Self {
        Self {
            spans: Box::new([]),
            size: None,
            line_height: LineHeight::default(),
            width: Length::Shrink,
            height: Length::Shrink,
            font: None,
            align_x: Alignment::Default,
            align_y: alignment::Vertical::Top,
            wrapping: Wrapping::default(),
            class: Theme::default(),
            hovered_link: None,
            on_link_click: None,
        }
    }

    /// 使用给定的文本片段创建一个新的 [`Rich`] 文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Rich`] text with the given text spans.
    ///
    /// </details>
    pub fn with_spans(
        spans: impl AsRef<[Span<'a, Link, Renderer::Font>]> + 'a,
    ) -> Self {
        Self {
            spans: Box::new(spans),
            ..Self::new()
        }
    }

    /// 设置 [`Rich`] 文本的默认字号。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the default size of the [`Rich`] text.
    ///
    /// </details>
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// 设置 [`Rich`] 文本的默认 [`LineHeight`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the default [`LineHeight`] of the [`Rich`] text.
    ///
    /// </details>
    pub fn line_height(mut self, line_height: impl Into<LineHeight>) -> Self {
        self.line_height = line_height.into();
        self
    }

    /// 设置 [`Rich`] 文本的默认字体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the default font of the [`Rich`] text.
    ///
    /// </details>
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// 设置 [`Rich`] 文本边界的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Rich`] text boundaries.
    ///
    /// </details>
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// 设置 [`Rich`] 文本边界的高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the height of the [`Rich`] text boundaries.
    ///
    /// </details>
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// 让 [`Rich`] 文本在水平和垂直方向上都居中。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Centers the [`Rich`] text, both horizontally and vertically.
    ///
    /// </details>
    pub fn center(self) -> Self {
        self.align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
    }

    /// 设置 [`Rich`] 文本的 [`alignment::Horizontal`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`alignment::Horizontal`] of the [`Rich`] text.
    ///
    /// </details>
    pub fn align_x(mut self, alignment: impl Into<Alignment>) -> Self {
        self.align_x = alignment.into();
        self
    }

    /// 设置 [`Rich`] 文本的 [`alignment::Vertical`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`alignment::Vertical`] of the [`Rich`] text.
    ///
    /// </details>
    pub fn align_y(
        mut self,
        alignment: impl Into<alignment::Vertical>,
    ) -> Self {
        self.align_y = alignment.into();
        self
    }

    /// 设置 [`Rich`] 文本的 [`Wrapping`] 策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Wrapping`] strategy of the [`Rich`] text.
    ///
    /// </details>
    pub fn wrapping(mut self, wrapping: Wrapping) -> Self {
        self.wrapping = wrapping;
        self
    }

    /// 设置 [`Rich`] 文本中的链接被点击时
    /// 要产生的消息。
    ///
    /// 如果 [`Rich`] 文本的片段中不包含链接，你可能需要用
    /// `on_link_click(never)` 调用这个方法，以便编译器推断出
    /// 正确的 `Link` 泛型类型。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the message that will be produced when a link of the [`Rich`] text
    /// is clicked.
    ///
    /// If the spans of the [`Rich`] text contain no links, you may need to call
    /// this method with `on_link_click(never)` in order for the compiler to infer
    /// the proper `Link` generic type.
    ///
    /// </details>
    pub fn on_link_click(
        mut self,
        on_link_click: impl Fn(Link) -> Message + 'a,
    ) -> Self {
        self.on_link_click = Some(Box::new(on_link_click));
        self
    }

    /// 设置 [`Rich`] 文本的默认样式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the default style of the [`Rich`] text.
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

    /// 设置 [`Rich`] 文本的默认 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the default [`Color`] of the [`Rich`] text.
    ///
    /// </details>
    pub fn color(self, color: impl Into<Color>) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.color_maybe(Some(color))
    }

    /// 如果为 `Some`，设置 [`Rich`] 文本的默认 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the default [`Color`] of the [`Rich`] text, if `Some`.
    ///
    /// </details>
    pub fn color_maybe(self, color: Option<impl Into<Color>>) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        let color = color.map(Into::into);

        self.style(move |_theme| Style { color })
    }

    /// 设置 [`Rich`] 文本的默认样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the default style class of the [`Rich`] text.
    ///
    /// </details>
    #[cfg(feature = "advanced")]
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<'a, Link, Message, Theme, Renderer> Default
    for Rich<'a, Link, Message, Theme, Renderer>
where
    Link: Clone + 'a,
    Theme: Catalog,
    Renderer: core::text::Renderer,
    Renderer::Font: 'a,
{
    fn default() -> Self {
        Self::new()
    }
}

struct State<Link, P: Paragraph> {
    spans: Vec<Span<'static, Link, P::Font>>,
    span_pressed: Option<usize>,
    paragraph: P,
}

impl<Link, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Rich<'_, Link, Message, Theme, Renderer>
where
    Link: Clone + 'static,
    Theme: Catalog,
    Renderer: core::text::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<Link, Renderer::Paragraph>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::<Link, _> {
            spans: Vec::new(),
            span_pressed: None,
            paragraph: Renderer::Paragraph::default(),
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
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout(
            tree.state
                .downcast_mut::<State<Link, Renderer::Paragraph>>(),
            renderer,
            limits,
            self.width,
            self.height,
            self.spans.as_ref().as_ref(),
            self.line_height,
            self.size,
            self.font,
            self.align_x,
            self.align_y,
            self.wrapping,
        )
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
        if !layout.bounds().intersects(viewport) {
            return;
        }

        let state = tree
            .state
            .downcast_ref::<State<Link, Renderer::Paragraph>>();

        let style = theme.style(&self.class);

        for (index, span) in self.spans.as_ref().as_ref().iter().enumerate() {
            let is_hovered_link = self.on_link_click.is_some()
                && Some(index) == self.hovered_link;

            if span.highlight.is_some()
                || span.underline
                || span.strikethrough
                || is_hovered_link
            {
                let translation = layout.position() - Point::ORIGIN;
                let regions = state.paragraph.span_bounds(index);

                if let Some(highlight) = span.highlight {
                    for bounds in &regions {
                        let bounds = Rectangle::new(
                            bounds.position()
                                - Vector::new(
                                    span.padding.left,
                                    span.padding.top,
                                ),
                            bounds.size()
                                + Size::new(span.padding.x(), span.padding.y()),
                        );

                        renderer.fill_quad(
                            renderer::Quad {
                                bounds: bounds + translation,
                                border: highlight.border,
                                ..Default::default()
                            },
                            highlight.background,
                        );
                    }
                }

                if span.underline || span.strikethrough || is_hovered_link {
                    let size = span
                        .size
                        .or(self.size)
                        .unwrap_or(renderer.default_size());

                    let line_height = span
                        .line_height
                        .unwrap_or(self.line_height)
                        .to_absolute(size);

                    let color = span
                        .color
                        .or(style.color)
                        .unwrap_or(defaults.text_color);

                    let baseline = translation
                        + Vector::new(
                            0.0,
                            size.0 + (line_height.0 - size.0) / 2.0,
                        );

                    if span.underline || is_hovered_link {
                        for bounds in &regions {
                            renderer.fill_quad(
                                renderer::Quad {
                                    bounds: Rectangle::new(
                                        bounds.position() + baseline
                                            - Vector::new(0.0, size.0 * 0.08),
                                        Size::new(bounds.width, 1.0),
                                    ),
                                    ..Default::default()
                                },
                                color,
                            );
                        }
                    }

                    if span.strikethrough {
                        for bounds in &regions {
                            renderer.fill_quad(
                                renderer::Quad {
                                    bounds: Rectangle::new(
                                        bounds.position() + baseline
                                            - Vector::new(0.0, size.0 / 2.0),
                                        Size::new(bounds.width, 1.0),
                                    ),
                                    ..Default::default()
                                },
                                color,
                            );
                        }
                    }
                }
            }
        }

        text::draw(
            renderer,
            defaults,
            layout.bounds(),
            &state.paragraph,
            style,
            viewport,
        );
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let Some(on_link_clicked) = &self.on_link_click else {
            return;
        };

        let was_hovered = self.hovered_link.is_some();

        if let Some(position) = cursor.position_in(layout.bounds()) {
            let state = tree
                .state
                .downcast_ref::<State<Link, Renderer::Paragraph>>();

            self.hovered_link =
                state.paragraph.hit_span(position).and_then(|span| {
                    if self.spans.as_ref().as_ref().get(span)?.link.is_some() {
                        Some(span)
                    } else {
                        None
                    }
                });
        } else {
            self.hovered_link = None;
        }

        if was_hovered != self.hovered_link.is_some() {
            shell.request_redraw();
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let state = tree
                    .state
                    .downcast_mut::<State<Link, Renderer::Paragraph>>();

                if self.hovered_link.is_some() {
                    state.span_pressed = self.hovered_link;
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                let state = tree
                    .state
                    .downcast_mut::<State<Link, Renderer::Paragraph>>();

                match state.span_pressed {
                    Some(span) if Some(span) == self.hovered_link => {
                        if let Some(link) = self
                            .spans
                            .as_ref()
                            .as_ref()
                            .get(span)
                            .and_then(|span| span.link.clone())
                        {
                            shell.publish(on_link_clicked(link));
                        }
                    }
                    _ => {}
                }

                state.span_pressed = None;
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if self.hovered_link.is_some() {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::None
        }
    }
}

fn layout<Link, Renderer>(
    state: &mut State<Link, Renderer::Paragraph>,
    renderer: &Renderer,
    limits: &layout::Limits,
    width: Length,
    height: Length,
    spans: &[Span<'_, Link, Renderer::Font>],
    line_height: LineHeight,
    size: Option<Pixels>,
    font: Option<Renderer::Font>,
    align_x: Alignment,
    align_y: alignment::Vertical,
    wrapping: Wrapping,
) -> layout::Node
where
    Link: Clone,
    Renderer: core::text::Renderer,
{
    layout::sized(limits, width, height, |limits| {
        let bounds = limits.max();

        let size = size.unwrap_or_else(|| renderer.default_size());
        let font = font.unwrap_or_else(|| renderer.default_font());

        let text_with_spans = || core::Text {
            content: spans,
            bounds,
            size,
            line_height,
            font,
            align_x,
            align_y,
            shaping: Shaping::Advanced,
            wrapping,
        };

        if state.spans != spans {
            state.paragraph =
                Renderer::Paragraph::with_spans(text_with_spans());
            state.spans = spans.iter().cloned().map(Span::to_static).collect();
        } else {
            match state.paragraph.compare(core::Text {
                content: (),
                bounds,
                size,
                line_height,
                font,
                align_x,
                align_y,
                shaping: Shaping::Advanced,
                wrapping,
            }) {
                core::text::Difference::None => {}
                core::text::Difference::Bounds => {
                    state.paragraph.resize(bounds);
                }
                core::text::Difference::Shape => {
                    state.paragraph =
                        Renderer::Paragraph::with_spans(text_with_spans());
                }
            }
        }

        state.paragraph.min_bounds()
    })
}

impl<'a, Link, Message, Theme, Renderer>
    FromIterator<Span<'a, Link, Renderer::Font>>
    for Rich<'a, Link, Message, Theme, Renderer>
where
    Link: Clone + 'a,
    Theme: Catalog,
    Renderer: core::text::Renderer,
    Renderer::Font: 'a,
{
    fn from_iter<T: IntoIterator<Item = Span<'a, Link, Renderer::Font>>>(
        spans: T,
    ) -> Self {
        Self::with_spans(spans.into_iter().collect::<Vec<_>>())
    }
}

impl<'a, Link, Message, Theme, Renderer>
    From<Rich<'a, Link, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Link: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer + 'a,
{
    fn from(
        text: Rich<'a, Link, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(text)
    }
}
