//! 分隔线在水平或垂直方向上划分空间。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Rules divide space horizontally or vertically.
//!
//! </details>
//!
//! # 示例（Example）
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
//! # use iced::widget::Widget;
//! # pub type State = ();
//! use iced::widget::rule;
//!
//! #[derive(Clone)]
//! enum Message {
//!     // ...,
//! }
//!
//! fn view(state: &State) -> impl Widget<Message> {
//!     rule::horizontal(2)
//! }
//! ```
use crate::core;
use crate::core::border;
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::widget::{Meta, Tree};
use crate::core::{Color, Layout, Length, Pixels, Rectangle, Size, Theme, Widget};

/// 使用给定的高度创建一个新的水平 [`Rule`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new horizontal [`Rule`] with the given height.
///
/// </details>
pub fn horizontal<'a, Theme>(height: impl Into<Pixels>) -> Rule<'a, Theme>
where
    Theme: Catalog,
{
    Rule {
        thickness: Length::Fixed(height.into().0),
        is_vertical: false,
        class: Theme::default(),
    }
}

/// 使用给定的宽度创建一个新的垂直 [`Rule`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new vertical [`Rule`] with the given width.
///
/// </details>
pub fn vertical<'a, Theme>(width: impl Into<Pixels>) -> Rule<'a, Theme>
where
    Theme: Catalog,
{
    Rule {
        thickness: Length::Fixed(width.into().0),
        is_vertical: true,
        class: Theme::default(),
    }
}

/// 显示一条用于划分内容的水平或垂直分隔线。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Display a horizontal or vertical rule for dividing content.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// use iced::widget::rule;
///
/// #[derive(Clone)]
/// enum Message {
///     // ...,
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     rule::horizontal(2)
/// }
/// ```
pub struct Rule<'a, Theme = crate::Theme>
where
    Theme: Catalog,
{
    thickness: Length,
    is_vertical: bool,
    class: Theme::Class<'a>,
}

impl<'a, Theme> Rule<'a, Theme>
where
    Theme: Catalog,
{
    /// 设置 [`Rule`] 的样式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style of the [`Rule`].
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

    /// 设置 [`Rule`] 的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style class of the [`Rule`].
    ///
    /// </details>
    #[cfg(feature = "advanced")]
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<Theme> Meta for Rule<'_, Theme> where Theme: Catalog {}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Rule<'_, Theme>
where
    Renderer: core::Renderer,
    Theme: Catalog,
{
    fn size(&self) -> Size<Length> {
        if self.is_vertical {
            Size {
                width: self.thickness,
                height: Length::Fill,
            }
        } else {
            Size {
                width: Length::Fill,
                height: self.thickness,
            }
        }
    }

    fn layout(&mut self, tree: &mut Tree, _renderer: &Renderer, limits: &layout::Limits) {
        let size = <Self as Widget<(), Theme, Renderer>>::size(self);

        tree.size = layout::atomic(limits, size.width, size.height);
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let style = theme.style(&self.class);

        let mut bounds = if self.is_vertical {
            let line_x = bounds.x;

            let (offset, line_height) = style.fill_mode.fill(bounds.height);
            let line_y = bounds.y + offset;

            Rectangle {
                x: line_x,
                y: line_y,
                width: bounds.width,
                height: line_height,
            }
        } else {
            let line_y = bounds.y;

            let (offset, line_width) = style.fill_mode.fill(bounds.width);
            let line_x = bounds.x + offset;

            Rectangle {
                x: line_x,
                y: line_y,
                width: line_width,
                height: bounds.height,
            }
        };

        if style.snap {
            let unit = 1.0 / renderer.hint_factor().unwrap_or(1.0);

            bounds.width = bounds.width.max(unit);
            bounds.height = bounds.height.max(unit);
        }

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: border::rounded(style.radius),
                snap: style.snap,
                ..renderer::Quad::default()
            },
            style.color,
        );
    }
}

/// 分隔线的外观。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The appearance of a rule.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// 分隔线的颜色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The color of the rule.
    ///
    /// </details>
    pub color: Color,
    /// 线条拐角的半径。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The radius of the line corners.
    ///
    /// </details>
    pub radius: border::Radius,
    /// 分隔线的 [`FillMode`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`FillMode`] of the rule.
    ///
    /// </details>
    pub fill_mode: FillMode,
    /// 分隔线是否应该对齐到像素网格。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the rule should be snapped to the pixel grid.
    ///
    /// </details>
    pub snap: bool,
}

/// 分隔线的填充模式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The fill mode of a rule.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FillMode {
    /// 填满容器的整个长度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Fill the whole length of the container.
    ///
    /// </details>
    Full,
    /// 填充容器长度的一定百分比。分隔线
    /// 会在该容器中居中。
    ///
    /// 取值范围为 `[0.0, 100.0]`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Fill a percent of the length of the container. The rule
    /// will be centered in that container.
    ///
    /// The range is `[0.0, 100.0]`.
    ///
    /// </details>
    Percent(f32),
    /// 两端相同的偏移量，以长度单位计。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Uniform offset from each end, length units.
    ///
    /// </details>
    Padded(u16),
    /// 分隔线两端各自不同的偏移量，以长度单位计。
    /// 第一个值 = 上端或左端。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Different offset on each end of the rule, length units.
    /// First = top or left.
    ///
    /// </details>
    AsymmetricPadding(u16, u16),
}

impl FillMode {
    /// 返回分隔线的起始偏移量和长度。
    ///
    /// * `space` - 要填充的空间。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Return the starting offset and length of the rule.
    ///
    /// * `space` - The space to fill.
    ///
    /// </details>
    ///
    /// # 返回值（Returns）
    ///
    /// * (`starting_offset`, `length`)
    pub fn fill(&self, space: f32) -> (f32, f32) {
        match *self {
            FillMode::Full => (0.0, space),
            FillMode::Percent(percent) => {
                if percent >= 100.0 {
                    (0.0, space)
                } else {
                    let percent_width = (space * percent / 100.0).round();

                    (((space - percent_width) / 2.0).round(), percent_width)
                }
            }
            FillMode::Padded(padding) => {
                if padding == 0 {
                    (0.0, space)
                } else {
                    let padding = padding as f32;
                    let mut line_width = space - (padding * 2.0);
                    if line_width < 0.0 {
                        line_width = 0.0;
                    }

                    (padding, line_width)
                }
            }
            FillMode::AsymmetricPadding(first_pad, second_pad) => {
                let first_pad = first_pad as f32;
                let second_pad = second_pad as f32;
                let mut line_width = space - first_pad - second_pad;
                if line_width < 0.0 {
                    line_width = 0.0;
                }

                (first_pad, line_width)
            }
        }
    }
}

/// [`Rule`] 的主题样式目录。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The theme catalog of a [`Rule`].
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
    fn style(&self, class: &Self::Class<'_>) -> Style;
}

/// [`Rule`] 的样式函数。
///
/// 它只是一个装箱的闭包：`Fn(&Theme, Status) -> Style`。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A styling function for a [`Rule`].
///
/// This is just a boxed closure: `Fn(&Theme, Status) -> Style`.
///
/// </details>
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

/// [`Rule`] 的默认样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The default styling of a [`Rule`].
///
/// </details>
pub fn default(theme: &Theme) -> Style {
    let palette = theme.palette();

    Style {
        color: palette.background.strong.color,
        radius: 0.0.into(),
        fill_mode: FillMode::Full,
        snap: true,
    }
}

/// 使用弱背景色的 [`Rule`] 样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A [`Rule`] styling using the weak background color.
///
/// </details>
pub fn weak(theme: &Theme) -> Style {
    let palette = theme.palette();

    Style {
        color: palette.background.weak.color,
        radius: 0.0.into(),
        fill_mode: FillMode::Full,
        snap: true,
    }
}
