//! Svg 部件在你的应用程序中显示矢量图形。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Svg widgets display vector graphics in your application.
//!
//! </details>
//!
//! # 示例（Example）
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } }
//! # pub type State = ();
//! # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
//! use iced::widget::svg;
//!
//! enum Message {
//!     // ...
//! }
//!
//! fn view(state: &State) -> Element<'_, Message> {
//!     svg("tiger.svg").into()
//! }
//! ```
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::svg;
use crate::core::widget::Tree;
use crate::core::window;
use crate::core::{
    Clipboard, Color, ContentFit, Element, Event, Layout, Length, Point,
    Rectangle, Rotation, Shell, Size, Theme, Vector, Widget,
};

use std::path::PathBuf;

pub use crate::core::svg::Handle;

/// 矢量图形图像。
///
/// [`Svg`] 图像可以平滑地调整大小而不损失任何质量。
///
/// [`Svg`] 图像在调整大小时可能有相当大的渲染开销，
/// 尤其是当它们很复杂的时候。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A vector graphics image.
///
/// An [`Svg`] image resizes smoothly without losing any quality.
///
/// [`Svg`] images can have a considerable rendering cost when resized,
/// specially when they are complex.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } }
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// use iced::widget::svg;
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     svg("tiger.svg").into()
/// }
/// ```
pub struct Svg<'a, Theme = crate::Theme>
where
    Theme: Catalog,
{
    handle: Handle,
    width: Length,
    height: Length,
    content_fit: ContentFit,
    class: Theme::Class<'a>,
    rotation: Rotation,
    opacity: f32,
    status: Option<Status>,
}

impl<'a, Theme> Svg<'a, Theme>
where
    Theme: Catalog,
{
    /// 从给定的 [`Handle`] 创建一个新的 [`Svg`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Svg`] from the given [`Handle`].
    ///
    /// </details>
    pub fn new(handle: impl Into<Handle>) -> Self {
        Svg {
            handle: handle.into(),
            width: Length::Fill,
            height: Length::Shrink,
            content_fit: ContentFit::Contain,
            class: Theme::default(),
            rotation: Rotation::default(),
            opacity: 1.0,
            status: None,
        }
    }

    /// 创建一个新的 [`Svg`]，显示给定路径处的
    /// 文件内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Svg`] that will display the contents of the file at the
    /// provided path.
    ///
    /// </details>
    #[must_use]
    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        Self::new(Handle::from_path(path))
    }

    /// 设置 [`Svg`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Svg`].
    ///
    /// </details>
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// 设置 [`Svg`] 的高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the height of the [`Svg`].
    ///
    /// </details>
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// 设置 [`Svg`] 的 [`ContentFit`]。
    ///
    /// 默认为 [`ContentFit::Contain`]
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`ContentFit`] of the [`Svg`].
    ///
    /// Defaults to [`ContentFit::Contain`]
    ///
    /// </details>
    #[must_use]
    pub fn content_fit(self, content_fit: ContentFit) -> Self {
        Self {
            content_fit,
            ..self
        }
    }

    /// 设置 [`Svg`] 的样式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style of the [`Svg`].
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

    /// 设置 [`Svg`] 的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style class of the [`Svg`].
    ///
    /// </details>
    #[cfg(feature = "advanced")]
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    /// 对 [`Svg`] 应用给定的 [`Rotation`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies the given [`Rotation`] to the [`Svg`].
    ///
    /// </details>
    pub fn rotation(mut self, rotation: impl Into<Rotation>) -> Self {
        self.rotation = rotation.into();
        self
    }

    /// 设置 [`Svg`] 的不透明度。
    ///
    /// 它应该在 [0.0, 1.0] 范围内——`0.0` 表示完全透明，
    /// `1.0` 表示完全不透明。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the opacity of the [`Svg`].
    ///
    /// It should be in the [0.0, 1.0] range—`0.0` meaning completely transparent,
    /// and `1.0` meaning completely opaque.
    ///
    /// </details>
    pub fn opacity(mut self, opacity: impl Into<f32>) -> Self {
        self.opacity = opacity.into();
        self
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Svg<'_, Theme>
where
    Renderer: svg::Renderer,
    Theme: Catalog,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        // The raw w/h of the underlying image
        // 底层图像的原始宽高
        let Size { width, height } = renderer.measure_svg(&self.handle);
        let image_size = Size::new(width as f32, height as f32);

        // The rotated size of the svg
        // svg 旋转后的大小
        let rotated_size = self.rotation.apply(image_size);

        // The size to be available to the widget prior to `Shrink`ing
        // 在 `Shrink` 收缩之前可供部件使用的大小
        let raw_size = limits.resolve(self.width, self.height, rotated_size);

        // The uncropped size of the image when fit to the bounds above
        // 图像适配到上述边界后未裁剪的大小
        let full_size = self.content_fit.fit(rotated_size, raw_size);

        // Shrink the widget to fit the resized image, if requested
        // 如果有要求，把部件收缩到适合调整大小后的图像
        let final_size = Size {
            width: match self.width {
                Length::Shrink => f32::min(raw_size.width, full_size.width),
                _ => raw_size.width,
            },
            height: match self.height {
                Length::Shrink => f32::min(raw_size.height, full_size.height),
                _ => raw_size.height,
            },
        };

        layout::Node::new(final_size)
    }

    fn update(
        &mut self,
        _state: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let current_status = if cursor.is_over(layout.bounds()) {
            Status::Hovered
        } else {
            Status::Idle
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.status = Some(current_status);
        } else if self.status.is_some_and(|status| status != current_status) {
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let Size { width, height } = renderer.measure_svg(&self.handle);
        let image_size = Size::new(width as f32, height as f32);
        let rotated_size = self.rotation.apply(image_size);

        let bounds = layout.bounds();
        let adjusted_fit = self.content_fit.fit(rotated_size, bounds.size());
        let scale = Vector::new(
            adjusted_fit.width / rotated_size.width,
            adjusted_fit.height / rotated_size.height,
        );

        let final_size = image_size * scale;

        let position = match self.content_fit {
            ContentFit::None => Point::new(
                bounds.x + (rotated_size.width - adjusted_fit.width) / 2.0,
                bounds.y + (rotated_size.height - adjusted_fit.height) / 2.0,
            ),
            _ => Point::new(
                bounds.center_x() - final_size.width / 2.0,
                bounds.center_y() - final_size.height / 2.0,
            ),
        };

        let drawing_bounds = Rectangle::new(position, final_size);

        let style =
            theme.style(&self.class, self.status.unwrap_or(Status::Idle));

        renderer.draw_svg(
            svg::Svg {
                handle: self.handle.clone(),
                color: style.color,
                rotation: self.rotation.radians(),
                opacity: self.opacity,
            },
            drawing_bounds,
            bounds,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Svg<'a, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: svg::Renderer + 'a,
{
    fn from(icon: Svg<'a, Theme>) -> Element<'a, Message, Theme, Renderer> {
        Element::new(icon)
    }
}

/// [`Svg`] 可能的交互状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The possible status of an [`Svg`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// [`Svg`] 处于空闲状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Svg`] is idle.
    ///
    /// </details>
    Idle,
    /// [`Svg`] 正处于悬停状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Svg`] is being hovered.
    ///
    /// </details>
    Hovered,
}

/// [`Svg`] 的外观。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The appearance of an [`Svg`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Style {
    /// [`Svg`] 的 [`Color`] 滤镜。
    ///
    /// 适合用来给符号图标着色。
    ///
    /// `None` 保留原始颜色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Color`] filter of an [`Svg`].
    ///
    /// Useful for coloring a symbolic icon.
    ///
    /// `None` keeps the original color.
    ///
    /// </details>
    pub color: Option<Color>,
}

/// [`Svg`] 的主题样式目录。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The theme catalog of an [`Svg`].
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

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|_theme, _status| Style::default())
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// [`Svg`] 的样式函数。
///
/// 它只是一个装箱的闭包：`Fn(&Theme, Status) -> Style`。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A styling function for an [`Svg`].
///
/// This is just a boxed closure: `Fn(&Theme, Status) -> Style`.
///
/// </details>
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl<Theme> From<Style> for StyleFn<'_, Theme> {
    fn from(style: Style) -> Self {
        Box::new(move |_theme, _status| style)
    }
}
