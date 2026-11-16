//! 进度条用于直观地展示耗时较长的计算机操作的进度，例如下载、文件传输或安装。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Progress bars visualize the progression of an extended computer operation, such as a download, file transfer, or installation.
//!
//! </details>
//!
//! # 示例（Example）
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
//! # use iced::widget::Widget;
//! #
//! use iced::widget::progress_bar;
//!
//! struct State {
//!    progress: f32,
//! }
//!
//! enum Message {
//!     // ...
//! }
//!
//! fn view(state: &State) -> impl Widget<Message> {
//!     progress_bar(0.0..=100.0, state.progress)
//! }
//! ```
use crate::core::border::{self, Border};
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::widget::{Meta, Tree};
use crate::core::{self, Background, Color, Layout, Length, Rectangle, Size, Theme, Widget};

use std::ops::RangeInclusive;

/// 显示进度的条。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A bar that displays progress.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// #
/// use iced::widget::progress_bar;
///
/// struct State {
///    progress: f32,
/// }
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     progress_bar(0.0..=100.0, state.progress)
/// }
/// ```
pub struct ProgressBar<'a, Theme = crate::Theme>
where
    Theme: Catalog,
{
    range: RangeInclusive<f32>,
    value: f32,
    length: Length,
    girth: Length,
    is_vertical: bool,
    class: Theme::Class<'a>,
}

impl<'a, Theme> ProgressBar<'a, Theme>
where
    Theme: Catalog,
{
    /// [`ProgressBar`] 的默认粗细。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default girth of a [`ProgressBar`].
    ///
    /// </details>
    pub const DEFAULT_GIRTH: f32 = 30.0;

    /// 创建一个新的 [`ProgressBar`]。
    ///
    /// 它需要：
    ///   * 一个包含两端的可能取值范围
    ///   * [`ProgressBar`] 的当前值
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`ProgressBar`].
    ///
    /// It expects:
    ///   * an inclusive range of possible values
    ///   * the current value of the [`ProgressBar`]
    ///
    /// </details>
    pub fn new(range: RangeInclusive<f32>, value: f32) -> Self {
        ProgressBar {
            value: value.clamp(*range.start(), *range.end()),
            range,
            length: Length::Fill,
            girth: Length::from(Self::DEFAULT_GIRTH),
            is_vertical: false,
            class: Theme::default(),
        }
    }

    /// 设置 [`ProgressBar`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`ProgressBar`].
    ///
    /// </details>
    pub fn length(mut self, length: impl Into<Length>) -> Self {
        self.length = length.into();
        self
    }

    /// 设置 [`ProgressBar`] 的高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the height of the [`ProgressBar`].
    ///
    /// </details>
    pub fn girth(mut self, girth: impl Into<Length>) -> Self {
        self.girth = girth.into();
        self
    }

    /// 把 [`ProgressBar`] 变成垂直的 [`ProgressBar`]。
    ///
    /// 默认情况下，[`ProgressBar`] 是水平的。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Turns the [`ProgressBar`] into a vertical [`ProgressBar`].
    ///
    /// By default, a [`ProgressBar`] is horizontal.
    ///
    /// </details>
    pub fn vertical(mut self, vertical: bool) -> Self {
        self.is_vertical = vertical;
        self
    }

    /// 设置 [`ProgressBar`] 的样式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style of the [`ProgressBar`].
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

    /// 设置 [`ProgressBar`] 的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style class of the [`ProgressBar`].
    ///
    /// </details>
    #[cfg(feature = "advanced")]
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    fn width(&self) -> Length {
        if self.is_vertical {
            self.girth
        } else {
            self.length
        }
    }

    fn height(&self) -> Length {
        if self.is_vertical {
            self.length
        } else {
            self.girth
        }
    }
}

impl<Theme> Meta for ProgressBar<'_, Theme> where Theme: Catalog {}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for ProgressBar<'_, Theme>
where
    Theme: Catalog,
    Renderer: core::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width(),
            height: self.height(),
        }
    }

    fn layout(&mut self, tree: &mut Tree, _renderer: &Renderer, limits: &layout::Limits) {
        tree.size = layout::atomic(limits, self.width(), self.height());
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
        let (range_start, range_end) = self.range.clone().into_inner();

        let length = if self.is_vertical {
            bounds.height
        } else {
            bounds.width
        };

        let active_progress_length = if range_start >= range_end {
            0.0
        } else {
            length * (self.value - range_start) / (range_end - range_start)
        };

        let style = theme.style(&self.class);

        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle { ..bounds },
                border: style.border,
                ..renderer::Quad::default()
            },
            style.background,
        );

        if active_progress_length > 0.0 {
            let bounds = if self.is_vertical {
                Rectangle {
                    y: bounds.y + bounds.height - active_progress_length,
                    height: active_progress_length,
                    ..bounds
                }
            } else {
                Rectangle {
                    width: active_progress_length,
                    ..bounds
                }
            };

            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        color: Color::TRANSPARENT,
                        ..style.border
                    },
                    ..renderer::Quad::default()
                },
                style.bar,
            );
        }
    }
}

/// 进度条的外观。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The appearance of a progress bar.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// 进度条的 [`Background`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Background`] of the progress bar.
    ///
    /// </details>
    pub background: Background,
    /// 进度条中进度部分的 [`Background`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Background`] of the bar of the progress bar.
    ///
    /// </details>
    pub bar: Background,
    /// 进度条的 [`Border`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Border`] of the progress bar.
    ///
    /// </details>
    pub border: Border,
}

/// [`ProgressBar`] 的主题样式目录。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The theme catalog of a [`ProgressBar`].
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

/// [`ProgressBar`] 的样式函数。
///
/// 它只是一个装箱的闭包：`Fn(&Theme, Status) -> Style`。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A styling function for a [`ProgressBar`].
///
/// This is just a boxed closure: `Fn(&Theme, Status) -> Style`.
///
/// </details>
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

/// [`ProgressBar`] 的主要（primary）样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The primary style of a [`ProgressBar`].
///
/// </details>
pub fn primary(theme: &Theme) -> Style {
    let palette = theme.palette();

    styled(palette.background.strong.color, palette.primary.base.color)
}

/// [`ProgressBar`] 的次要（secondary）样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The secondary style of a [`ProgressBar`].
///
/// </details>
pub fn secondary(theme: &Theme) -> Style {
    let palette = theme.palette();

    styled(
        palette.background.strong.color,
        palette.secondary.base.color,
    )
}

/// [`ProgressBar`] 的成功（success）样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The success style of a [`ProgressBar`].
///
/// </details>
pub fn success(theme: &Theme) -> Style {
    let palette = theme.palette();

    styled(palette.background.strong.color, palette.success.base.color)
}

/// [`ProgressBar`] 的警告（warning）样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The warning style of a [`ProgressBar`].
///
/// </details>
pub fn warning(theme: &Theme) -> Style {
    let palette = theme.palette();

    styled(palette.background.strong.color, palette.warning.base.color)
}

/// [`ProgressBar`] 的危险（danger）样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The danger style of a [`ProgressBar`].
///
/// </details>
pub fn danger(theme: &Theme) -> Style {
    let palette = theme.palette();

    styled(palette.background.strong.color, palette.danger.base.color)
}

fn styled(background: impl Into<Background>, bar: impl Into<Background>) -> Style {
    Style {
        background: background.into(),
        bar: bar.into(),
        border: border::rounded(2),
    }
}
