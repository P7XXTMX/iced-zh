//! 容器让你可以在其边界内对齐一个部件。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Containers let you align a widget inside their boundaries.
//!
//! </details>
//!
//! # 示例（Example）
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } }
//! # pub type State = ();
//! # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
//! use iced::widget::container;
//!
//! enum Message {
//!     // ...
//! }
//!
//! fn view(state: &State) -> Element<'_, Message> {
//!     container("This text is centered inside a rounded box!")
//!         .padding(10)
//!         .center(800)
//!         .style(container::rounded_box)
//!         .into()
//! }
//! ```
use crate::core::alignment::{self, Alignment};
use crate::core::border::{self, Border};
use crate::core::gradient::{self, Gradient};
use crate::core::layout;
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::theme;
use crate::core::widget::tree::{self, Tree};
use crate::core::widget::{self, Operation};
use crate::core::{
    self, Background, Clipboard, Color, Element, Event, Layout, Length,
    Padding, Pixels, Rectangle, Shadow, Shell, Size, Theme, Vector, Widget,
    color,
};

/// 在自身边界内对齐其内容的部件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A widget that aligns its contents inside of its boundaries.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } }
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// use iced::widget::container;
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     container("This text is centered inside a rounded box!")
///         .padding(10)
///         .center(800)
///         .style(container::rounded_box)
///         .into()
/// }
/// ```
pub struct Container<
    'a,
    Message,
    Theme = crate::Theme,
    Renderer = crate::Renderer,
> where
    Theme: Catalog,
    Renderer: core::Renderer,
{
    id: Option<widget::Id>,
    padding: Padding,
    width: Length,
    height: Length,
    max_width: f32,
    max_height: f32,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
    clip: bool,
    content: Element<'a, Message, Theme, Renderer>,
    class: Theme::Class<'a>,
}

impl<'a, Message, Theme, Renderer> Container<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: core::Renderer,
{
    /// 使用给定的内容创建一个 [`Container`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Container`] with the given content.
    ///
    /// </details>
    pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        let content = content.into();
        let size = content.as_widget().size_hint();

        Container {
            id: None,
            padding: Padding::ZERO,
            width: size.width.fluid(),
            height: size.height.fluid(),
            max_width: f32::INFINITY,
            max_height: f32::INFINITY,
            horizontal_alignment: alignment::Horizontal::Left,
            vertical_alignment: alignment::Vertical::Top,
            clip: false,
            class: Theme::default(),
            content,
        }
    }

    /// 设置 [`Container`] 的 [`widget::Id`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`widget::Id`] of the [`Container`].
    ///
    /// </details>
    pub fn id(mut self, id: impl Into<widget::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// 设置 [`Container`] 的 [`Padding`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Padding`] of the [`Container`].
    ///
    /// </details>
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// 设置 [`Container`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Container`].
    ///
    /// </details>
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// 设置 [`Container`] 的高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the height of the [`Container`].
    ///
    /// </details>
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// 设置 [`Container`] 的最大宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the maximum width of the [`Container`].
    ///
    /// </details>
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = max_width.into().0;
        self
    }

    /// 设置 [`Container`] 的最大高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the maximum height of the [`Container`].
    ///
    /// </details>
    pub fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
        self.max_height = max_height.into().0;
        self
    }

    /// 设置 [`Container`] 的宽度，并让其内容水平居中。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Container`] and centers its contents horizontally.
    ///
    /// </details>
    pub fn center_x(self, width: impl Into<Length>) -> Self {
        self.width(width).align_x(alignment::Horizontal::Center)
    }

    /// 设置 [`Container`] 的高度，并让其内容垂直居中。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the height of the [`Container`] and centers its contents vertically.
    ///
    /// </details>
    pub fn center_y(self, height: impl Into<Length>) -> Self {
        self.height(height).align_y(alignment::Vertical::Center)
    }

    /// 设置 [`Container`] 的宽度和高度，并让其内容在
    /// 水平轴和垂直轴上都居中。
    ///
    /// 这等价于链式调用 [`center_x`] 和 [`center_y`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width and height of the [`Container`] and centers its contents in
    /// both the horizontal and vertical axes.
    ///
    /// This is equivalent to chaining [`center_x`] and [`center_y`].
    ///
    /// </details>
    ///
    /// [`center_x`]: Self::center_x
    /// [`center_y`]: Self::center_y
    pub fn center(self, length: impl Into<Length>) -> Self {
        let length = length.into();

        self.center_x(length).center_y(length)
    }

    /// 设置 [`Container`] 的宽度，并让其内容左对齐。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Container`] and aligns its contents to the left.
    ///
    /// </details>
    pub fn align_left(self, width: impl Into<Length>) -> Self {
        self.width(width).align_x(alignment::Horizontal::Left)
    }

    /// 设置 [`Container`] 的宽度，并让其内容右对齐。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Container`] and aligns its contents to the right.
    ///
    /// </details>
    pub fn align_right(self, width: impl Into<Length>) -> Self {
        self.width(width).align_x(alignment::Horizontal::Right)
    }

    /// 设置 [`Container`] 的高度，并让其内容顶部对齐。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the height of the [`Container`] and aligns its contents to the top.
    ///
    /// </details>
    pub fn align_top(self, height: impl Into<Length>) -> Self {
        self.height(height).align_y(alignment::Vertical::Top)
    }

    /// 设置 [`Container`] 的高度，并让其内容底部对齐。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the height of the [`Container`] and aligns its contents to the bottom.
    ///
    /// </details>
    pub fn align_bottom(self, height: impl Into<Length>) -> Self {
        self.height(height).align_y(alignment::Vertical::Bottom)
    }

    /// 设置 [`Container`] 水平轴上的内容对齐方式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the content alignment for the horizontal axis of the [`Container`].
    ///
    /// </details>
    pub fn align_x(
        mut self,
        alignment: impl Into<alignment::Horizontal>,
    ) -> Self {
        self.horizontal_alignment = alignment.into();
        self
    }

    /// 设置 [`Container`] 垂直轴上的内容对齐方式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the content alignment for the vertical axis of the [`Container`].
    ///
    /// </details>
    pub fn align_y(
        mut self,
        alignment: impl Into<alignment::Vertical>,
    ) -> Self {
        self.vertical_alignment = alignment.into();
        self
    }

    /// 设置 [`Container`] 的内容在溢出时是否应该
    /// 被裁剪。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets whether the contents of the [`Container`] should be clipped on
    /// overflow.
    ///
    /// </details>
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// 设置 [`Container`] 的样式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style of the [`Container`].
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

    /// 设置 [`Container`] 的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style class of the [`Container`].
    ///
    /// </details>
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Container<'_, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: core::Renderer,
{
    fn tag(&self) -> tree::Tag {
        self.content.as_widget().tag()
    }

    fn state(&self) -> tree::State {
        self.content.as_widget().state()
    }

    fn children(&self) -> Vec<Tree> {
        self.content.as_widget().children()
    }

    fn diff(&self, tree: &mut Tree) {
        self.content.as_widget().diff(tree);
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
            limits,
            self.width,
            self.height,
            self.max_width,
            self.max_height,
            self.padding,
            self.horizontal_alignment,
            self.vertical_alignment,
            |limits| {
                self.content.as_widget_mut().layout(tree, renderer, limits)
            },
        )
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(self.id.as_ref(), layout.bounds());
        operation.traverse(&mut |operation| {
            self.content.as_widget_mut().operate(
                tree,
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.content.as_widget_mut().update(
            tree,
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            tree,
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        renderer_style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let style = theme.style(&self.class);

        if let Some(clipped_viewport) = bounds.intersection(viewport) {
            draw_background(renderer, &style, bounds);

            self.content.as_widget().draw(
                tree,
                renderer,
                theme,
                &renderer::Style {
                    text_color: style
                        .text_color
                        .unwrap_or(renderer_style.text_color),
                },
                layout.children().next().unwrap(),
                cursor,
                if self.clip {
                    &clipped_viewport
                } else {
                    viewport
                },
            );
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            tree,
            layout.children().next().unwrap(),
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message, Theme, Renderer> From<Container<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::Renderer + 'a,
{
    fn from(
        container: Container<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(container)
    }
}

/// 计算 [`Container`] 的布局。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Computes the layout of a [`Container`].
///
/// </details>
pub fn layout(
    limits: &layout::Limits,
    width: Length,
    height: Length,
    max_width: f32,
    max_height: f32,
    padding: Padding,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
    layout_content: impl FnOnce(&layout::Limits) -> layout::Node,
) -> layout::Node {
    layout::positioned(
        &limits.max_width(max_width).max_height(max_height),
        width,
        height,
        padding,
        |limits| layout_content(&limits.loose()),
        |content, size| {
            content.align(
                Alignment::from(horizontal_alignment),
                Alignment::from(vertical_alignment),
                size,
            )
        },
    )
}

/// 根据给定的 [`Style`] 和 `bounds` 绘制 [`Container`] 的背景。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Draws the background of a [`Container`] given its [`Style`] and its `bounds`.
///
/// </details>
pub fn draw_background<Renderer>(
    renderer: &mut Renderer,
    style: &Style,
    bounds: Rectangle,
) where
    Renderer: core::Renderer,
{
    if style.background.is_some()
        || style.border.width > 0.0
        || style.shadow.color.a > 0.0
    {
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
}

/// 容器的外观。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The appearance of a container.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// 容器的文本 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text [`Color`] of the container.
    ///
    /// </details>
    pub text_color: Option<Color>,
    /// 容器的 [`Background`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Background`] of the container.
    ///
    /// </details>
    pub background: Option<Background>,
    /// 容器的 [`Border`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Border`] of the container.
    ///
    /// </details>
    pub border: Border,
    /// 容器的 [`Shadow`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Shadow`] of the container.
    ///
    /// </details>
    pub shadow: Shadow,
    /// 容器是否应该对齐到像素网格。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the container should be snapped to the pixel grid.
    ///
    /// </details>
    pub snap: bool,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            text_color: None,
            background: None,
            border: Border::default(),
            shadow: Shadow::default(),
            snap: cfg!(feature = "crisp"),
        }
    }
}

impl Style {
    /// 更新 [`Style`] 的文本颜色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Updates the text color of the [`Style`].
    ///
    /// </details>
    pub fn color(self, color: impl Into<Color>) -> Self {
        Self {
            text_color: Some(color.into()),
            ..self
        }
    }

    /// 更新 [`Style`] 的边框。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Updates the border of the [`Style`].
    ///
    /// </details>
    pub fn border(self, border: impl Into<Border>) -> Self {
        Self {
            border: border.into(),
            ..self
        }
    }

    /// 更新 [`Style`] 的背景。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Updates the background of the [`Style`].
    ///
    /// </details>
    pub fn background(self, background: impl Into<Background>) -> Self {
        Self {
            background: Some(background.into()),
            ..self
        }
    }

    /// 更新 [`Style`] 的阴影。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Updates the shadow of the [`Style`].
    ///
    /// </details>
    pub fn shadow(self, shadow: impl Into<Shadow>) -> Self {
        Self {
            shadow: shadow.into(),
            ..self
        }
    }
}

impl From<Color> for Style {
    fn from(color: Color) -> Self {
        Self::default().background(color)
    }
}

impl From<Gradient> for Style {
    fn from(gradient: Gradient) -> Self {
        Self::default().background(gradient)
    }
}

impl From<gradient::Linear> for Style {
    fn from(gradient: gradient::Linear) -> Self {
        Self::default().background(gradient)
    }
}

/// [`Container`] 的主题样式目录。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The theme catalog of a [`Container`].
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
    fn style(&self, class: &Self::Class<'_>) -> Style;
}

/// [`Container`] 的样式函数。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A styling function for a [`Container`].
///
/// </details>
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl<Theme> From<Style> for StyleFn<'_, Theme> {
    fn from(style: Style) -> Self {
        Box::new(move |_theme| style)
    }
}

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(transparent)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

/// 透明的 [`Container`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A transparent [`Container`].
///
/// </details>
pub fn transparent<Theme>(_theme: &Theme) -> Style {
    Style::default()
}

/// 具有给定 [`Background`] 的 [`Container`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A [`Container`] with the given [`Background`].
///
/// </details>
pub fn background(background: impl Into<Background>) -> Style {
    Style::default().background(background)
}

/// 带背景的圆角 [`Container`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A rounded [`Container`] with a background.
///
/// </details>
pub fn rounded_box(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: Some(palette.background.weak.color.into()),
        text_color: Some(palette.background.weak.text),
        border: border::rounded(2),
        ..Style::default()
    }
}

/// 带背景和边框的 [`Container`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A bordered [`Container`] with a background.
///
/// </details>
pub fn bordered_box(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: Some(palette.background.weakest.color.into()),
        text_color: Some(palette.background.weakest.text),
        border: Border {
            width: 1.0,
            radius: 5.0.into(),
            color: palette.background.weak.color,
        },
        ..Style::default()
    }
}

/// 深色背景、白色文本的 [`Container`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A [`Container`] with a dark background and white text.
///
/// </details>
pub fn dark(_theme: &Theme) -> Style {
    style(theme::palette::Pair {
        color: color!(0x111111),
        text: Color::WHITE,
    })
}

/// 使用主要背景色的 [`Container`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A [`Container`] with a primary background color.
///
/// </details>
pub fn primary(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    style(palette.primary.base)
}

/// 使用次要背景色的 [`Container`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A [`Container`] with a secondary background color.
///
/// </details>
pub fn secondary(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    style(palette.secondary.base)
}

/// 使用成功背景色的 [`Container`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A [`Container`] with a success background color.
///
/// </details>
pub fn success(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    style(palette.success.base)
}

/// 使用警告背景色的 [`Container`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A [`Container`] with a warning background color.
///
/// </details>
pub fn warning(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    style(palette.warning.base)
}

/// 使用危险背景色的 [`Container`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A [`Container`] with a danger background color.
///
/// </details>
pub fn danger(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    style(palette.danger.base)
}

fn style(pair: theme::palette::Pair) -> Style {
    Style {
        background: Some(pair.color.into()),
        text_color: Some(pair.text),
        border: border::rounded(2),
        ..Style::default()
    }
}
