//! 可滚动区域让用户借助滚动条浏览无限多的内容。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Scrollables let users navigate an endless amount of content with a scrollbar.
//!
//! </details>
//!
//! # 示例（Example）
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
//! # use iced::widget::Widget;
//! # pub type State = ();
//! use iced::widget::{column, scrollable, space};
//!
//! enum Message {
//!     // ...
//! }
//!
//! fn view(state: &State) -> impl Widget<Message> {
//!     scrollable(column![
//!         "Scroll me!",
//!         space().height(3000),
//!         "You did it!",
//!     ])
//! }
//! ```
use crate::container;
use crate::core::alignment;
use crate::core::border::{self, Border};
use crate::core::keyboard;
use crate::core::layout;
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::text;
use crate::core::time::{Duration, Instant};
use crate::core::touch;
use crate::core::widget;
use crate::core::widget::operation::{self, Animation, Operation};
use crate::core::widget::tree::{self, Tree};
use crate::core::window;
use crate::core::{
    self, Background, Color, Event, InputMethod, Layout, Length, Padding, Pixels, Point, Rectangle,
    Shadow, Shell, Size, Theme, Vector, Widget,
};

pub use operation::scrollable::{AbsoluteOffset, RelativeOffset};

/// 可以借助滚动条在垂直方向上显示无限多内容的
/// 部件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A widget that can vertically display an infinite amount of content with a
/// scrollbar.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// use iced::widget::{column, scrollable, space};
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     scrollable(column![
///         "Scroll me!",
///         space().height(3000),
///         "You did it!",
///     ])
/// }
/// ```
///
/// # 滚动条（Scrollbars）
///
/// 滚动条有两个可交互的部分：*滚动块*（scroller，可拖动的
/// 滑块）和*轨道*（rail，滚动块在其上滑动的轨道）。
///
/// * 按下并拖动**滚动块**时，它会与指针 1:1 同步移动。
/// * 按下**轨道**会滚动一页（启用 [`Self::smooth_scroll`] 时，
///   会像滚轮滚动一样带有动画）。按键保持按下期间，
///   滚动块随后会以恒定速度滑向指针，在其边缘到达指针时
///   停止；移动指针时，目标也会
///   随之移动。这与大多数工具包的轨道自动滚动行为一致。
/// * 按住 `Shift` 点击**轨道**则会让滚动块跳转，使其
///   中心位于指针之下，然后从那里开始拖动它。
///
/// 启用 [`Self::click_to_scroll`] 后，按下轨道与
/// 按住 `Shift` 点击的行为会互换。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A scrollbar has two interactive parts: the *scroller* (the draggable
/// thumb) and the *rail* (the track it slides on).
///
/// * Pressing and dragging the **scroller** moves it 1:1 with the pointer.
/// * Pressing the **rail** scrolls one page (animated like a wheel scroll
///   when [`Self::smooth_scroll`] is enabled). While the button is held, the
///   scroller then slides toward the pointer at a constant speed, stopping
///   when its edge reaches the pointer; moving the pointer moves the target
///   with it. This mirrors the track autoscroll of most toolkits.
/// * `Shift`-clicking the **rail** instead jumps the scroller so that its
///   center is under the pointer, and then drags it from there.
///
/// With [`Self::click_to_scroll`] enabled, the behavior of the rail press
/// and the `Shift`-click is inverted.
///
/// </details>
pub struct Scrollable<'a, Message, W = crate::Element<'a, Message>, Theme = crate::Theme>
where
    Theme: Catalog,
{
    id: Option<widget::Id>,
    width: Length,
    height: Length,
    direction: Direction,
    auto_scroll: bool,
    smooth_scroll: bool,
    click_to_scroll: bool,
    content: W,
    on_scroll: Option<Box<dyn Fn(Scroll) -> Action<Message> + 'a>>,
    class: Theme::Class<'a>,
}

impl<'a, Message, W, Theme> Scrollable<'a, Message, W, Theme>
where
    Theme: Catalog,
{
    /// 创建一个新的垂直 [`Scrollable`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new vertical [`Scrollable`].
    ///
    /// </details>
    pub fn new(content: W) -> Self {
        Self::with_direction(content, Direction::default())
    }

    /// 使用给定的 [`Direction`] 创建一个新的 [`Scrollable`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Scrollable`] with the given [`Direction`].
    ///
    /// </details>
    pub fn with_direction(content: W, direction: impl Into<Direction>) -> Self {
        Scrollable {
            id: None,
            width: Length::Fit,
            height: Length::Fit,
            direction: direction.into(),
            auto_scroll: false,
            smooth_scroll: true,
            click_to_scroll: false,
            content,
            on_scroll: None,
            class: Theme::default(),
        }
    }

    /// 让 [`Scrollable`] 水平滚动，使用默认的 [`Scrollbar`] 设置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Makes the [`Scrollable`] scroll horizontally, with default [`Scrollbar`] settings.
    ///
    /// </details>
    pub fn horizontal(self) -> Self {
        self.direction(Direction::Horizontal(Scrollbar::default()))
    }

    /// 设置 [`Scrollable`] 的 [`Direction`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Direction`] of the [`Scrollable`].
    ///
    /// </details>
    pub fn direction(mut self, direction: impl Into<Direction>) -> Self {
        self.direction = direction.into();
        self
    }

    /// 设置 [`Scrollable`] 的 [`widget::Id`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`widget::Id`] of the [`Scrollable`].
    ///
    /// </details>
    pub fn id(mut self, id: impl Into<widget::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// 设置 [`Scrollable`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Scrollable`].
    ///
    /// </details>
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// 设置 [`Scrollable`] 的高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the height of the [`Scrollable`].
    ///
    /// </details>
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// 设置 [`Scrollable`] 滚动时调用的处理函数。
    ///
    /// 这个函数接收一个 [`Scroll`]，其中包含
    /// [`Scrollable`] 的 [`Viewport`]，以及引起这次通知的
    /// 滚动的 [`Source`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets a handler to call when the [`Scrollable`] is scrolled.
    ///
    /// The function takes a [`Scroll`], which contains the [`Viewport`] of
    /// the [`Scrollable`] and the [`Source`] of the scroll that caused the
    /// notification.
    ///
    /// </details>
    pub fn on_scroll<T>(mut self, f: impl Fn(Scroll) -> T + 'a) -> Self
    where
        T: Into<Action<Message>>,
    {
        self.on_scroll = Some(Box::new(move |scroll| f(scroll).into()));
        self
    }

    /// 把 [`Scrollable`] 的垂直方向锚定到顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Anchors the vertical [`Scrollable`] direction to the top.
    ///
    /// </details>
    pub fn anchor_top(self) -> Self {
        self.anchor_y(Anchor::Start)
    }

    /// 把 [`Scrollable`] 的垂直方向锚定到底部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Anchors the vertical [`Scrollable`] direction to the bottom.
    ///
    /// </details>
    pub fn anchor_bottom(self) -> Self {
        self.anchor_y(Anchor::End)
    }

    /// 把 [`Scrollable`] 的水平方向锚定到左侧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Anchors the horizontal [`Scrollable`] direction to the left.
    ///
    /// </details>
    pub fn anchor_left(self) -> Self {
        self.anchor_x(Anchor::Start)
    }

    /// 把 [`Scrollable`] 的水平方向锚定到右侧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Anchors the horizontal [`Scrollable`] direction to the right.
    ///
    /// </details>
    pub fn anchor_right(self) -> Self {
        self.anchor_x(Anchor::End)
    }

    /// 设置 [`Scrollable`] 水平方向的 [`Anchor`]（如果适用）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Anchor`] of the horizontal direction of the [`Scrollable`], if applicable.
    ///
    /// </details>
    pub fn anchor_x(mut self, alignment: Anchor) -> Self {
        match &mut self.direction {
            Direction::Horizontal(horizontal) | Direction::Both { horizontal, .. } => {
                horizontal.alignment = alignment;
            }
            Direction::Vertical { .. } => {}
        }

        self
    }

    /// 设置 [`Scrollable`] 垂直方向的 [`Anchor`]（如果适用）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Anchor`] of the vertical direction of the [`Scrollable`], if applicable.
    ///
    /// </details>
    pub fn anchor_y(mut self, alignment: Anchor) -> Self {
        match &mut self.direction {
            Direction::Vertical(vertical) | Direction::Both { vertical, .. } => {
                vertical.alignment = alignment;
            }
            Direction::Horizontal { .. } => {}
        }

        self
    }

    /// 把 [`Scrollbar`] 嵌入 [`Scrollable`]，而不是让它浮动在
    /// 内容之上。
    ///
    /// 提供的 `spacing` 会用作 [`Scrollbar`] 与 [`Scrollable`]
    /// 内容之间的间距。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Embeds the [`Scrollbar`] into the [`Scrollable`], instead of floating on top of the
    /// content.
    ///
    /// The `spacing` provided will be used as space between the [`Scrollbar`] and the contents
    /// of the [`Scrollable`].
    ///
    /// </details>
    pub fn spacing(mut self, new_spacing: impl Into<Pixels>) -> Self {
        match &mut self.direction {
            Direction::Horizontal(scrollbar) | Direction::Vertical(scrollbar) => {
                scrollbar.spacing = Some(new_spacing.into().0);
            }
            Direction::Both { .. } => {}
        }

        self
    }

    /// 在 [`Scrollable`] 的各个 [`Scrollbar`] 两端添加内边距。
    ///
    /// 当滚动条可见时，提供的 `padding` 会用作
    /// 垂直 [`Scrollbar`] 顶部和底部的空间，以及水平
    /// [`Scrollbar`] 左右两端的空间。
    ///
    /// 与 [`Self::spacing`] 不同，这个内边距不会影响
    /// [`Scrollable`] 的布局。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adds padding at the ends of the [`Scrollbar`]s of the [`Scrollable`].
    ///
    /// The `padding` provided will be used as space at the top and bottom of a
    /// vertical [`Scrollbar`], and at the left and right ends of a horizontal
    /// [`Scrollbar`], when they are visible.
    ///
    /// Unlike [`Self::spacing`], the padding does not affect the layout of the
    /// [`Scrollable`].
    ///
    /// </details>
    pub fn padding(mut self, new_padding: impl Into<Pixels>) -> Self {
        let padding = new_padding.into().0;

        match &mut self.direction {
            Direction::Horizontal(scrollbar) | Direction::Vertical(scrollbar) => {
                scrollbar.padding = padding;
            }
            Direction::Both {
                horizontal,
                vertical,
            } => {
                horizontal.padding = padding;
                vertical.padding = padding;
            }
        }

        self
    }

    /// 设置是否允许用户用鼠标中键对 [`Scrollable`]
    /// 进行自动滚动。
    ///
    /// 默认禁用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets whether the user should be allowed to auto-scroll the [`Scrollable`]
    /// with the middle mouse button.
    ///
    /// By default, it is disabled.
    ///
    /// </details>
    pub fn auto_scroll(mut self, auto_scroll: bool) -> Self {
        self.auto_scroll = auto_scroll;
        self
    }

    /// 设置滚轮滚动是否应随时间平滑进行，而不是
    /// 立即移动 [`Scrollable`]。
    ///
    /// 启用后，离散的滚动（例如来自鼠标滚轮）会移动一个目标
    /// 滚动偏移量（它带有滚轮各格的动量），而
    /// [`Scrollable`] 会在几帧之内缓动到该目标。高精度的
    /// 滚动（例如来自触摸板）本身已经是平滑的，因此总是
    /// 立即生效。
    ///
    /// 这个设置同时也是各个滚动操作
    /// （`snap_to`、`scroll_to`、`scroll_by`）的默认行为：使用
    /// [`Animation::Auto`] 的操作当且仅当它启用时才会平滑滚动。
    ///
    /// 默认启用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets whether wheel scrolling should be smoothed out over time, instead of
    /// moving the [`Scrollable`] immediately.
    ///
    /// When enabled, discrete scrolls (e.g. from a mouse wheel) move a target
    /// scroll offset — carrying the momentum of the wheel notches — and the
    /// [`Scrollable`] eases towards it over a few frames. High-precision
    /// scrolls (e.g. from a touchpad), which are already smooth, are always
    /// applied immediately.
    ///
    /// The setting also acts as the default behavior of the scroll operations
    /// (`snap_to`, `scroll_to`, `scroll_by`): operations with
    /// [`Animation::Auto`] scroll smoothly if and only if it is enabled.
    ///
    /// By default, it is enabled.
    ///
    /// </details>
    pub fn smooth_scroll(mut self, smooth_scroll: bool) -> Self {
        self.smooth_scroll = smooth_scroll;
        self
    }

    /// 设置在滚动条轨道上的普通点击是否应让
    /// 滚动块跳转到点击位置，而不是滚动一页。
    ///
    /// 默认情况下，按下轨道会滚动一页（并在按键保持按下期间
    /// 自动滚动），而按住 `Shift` 点击会让滚动块跳转，
    /// 使其中心位于指针之下，并从那里开始拖动它。
    ///
    /// 启用后，行为会互换：普通点击会让
    /// 滚动块跳转，使其中心位于指针之下（并从那里
    /// 开始拖动它），而按住 `Shift` 点击会滚动一页（并在
    /// 按键保持按下期间自动滚动）。
    ///
    /// 默认禁用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets whether a plain click on the scrollbar rail should jump the
    /// scroller to the click position, instead of scrolling one page.
    ///
    /// By default, a press on the rail scrolls one page (and autoscrolls
    /// while the button is held), while a `Shift`-click jumps the scroller
    /// so that its center is under the pointer and drags it from there.
    ///
    /// When enabled, the behavior is inverted: a plain click jumps the
    /// scroller so that its center is under the pointer (and drags it from
    /// there), while a `Shift`-click scrolls one page (and autoscrolls
    /// while the button is held).
    ///
    /// By default, it is disabled.
    ///
    /// </details>
    pub fn click_to_scroll(mut self, click_to_scroll: bool) -> Self {
        self.click_to_scroll = click_to_scroll;
        self
    }

    /// 设置这个 [`Scrollable`] 的样式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style of this [`Scrollable`].
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

    /// 设置 [`Scrollable`] 的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style class of the [`Scrollable`].
    ///
    /// </details>
    #[cfg(feature = "advanced")]
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

/// [`Scrollable`] 的方向。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The direction of [`Scrollable`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    /// 垂直滚动
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Vertical scrolling
    ///
    /// </details>
    Vertical(Scrollbar),
    /// 水平滚动
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Horizontal scrolling
    ///
    /// </details>
    Horizontal(Scrollbar),
    /// 垂直和水平双向滚动
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Both vertical and horizontal scrolling
    ///
    /// </details>
    Both {
        /// 垂直滚动条的属性。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The properties of the vertical scrollbar.
        ///
        /// </details>
        vertical: Scrollbar,
        /// 水平滚动条的属性。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The properties of the horizontal scrollbar.
        ///
        /// </details>
        horizontal: Scrollbar,
    },
}

impl Direction {
    /// 返回水平 [`Scrollbar`]（如果有）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the horizontal [`Scrollbar`], if any.
    ///
    /// </details>
    pub fn horizontal(&self) -> Option<&Scrollbar> {
        match self {
            Self::Horizontal(scrollbar) => Some(scrollbar),
            Self::Both { horizontal, .. } => Some(horizontal),
            Self::Vertical(_) => None,
        }
    }

    /// 返回垂直 [`Scrollbar`]（如果有）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the vertical [`Scrollbar`], if any.
    ///
    /// </details>
    pub fn vertical(&self) -> Option<&Scrollbar> {
        match self {
            Self::Vertical(scrollbar) => Some(scrollbar),
            Self::Both { vertical, .. } => Some(vertical),
            Self::Horizontal(_) => None,
        }
    }

    fn align(&self, delta: Vector) -> Vector {
        let horizontal_alignment = self.horizontal().map(|p| p.alignment).unwrap_or_default();

        let vertical_alignment = self.vertical().map(|p| p.alignment).unwrap_or_default();

        let align = |alignment: Anchor, delta: f32| match alignment {
            Anchor::Start => delta,
            Anchor::End => -delta,
        };

        Vector::new(
            align(horizontal_alignment, delta.x),
            align(vertical_alignment, delta.y),
        )
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self::Vertical(Scrollbar::default())
    }
}

/// [`Scrollable`] 中的滚动条。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A scrollbar within a [`Scrollable`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scrollbar {
    width: f32,
    margin: f32,
    scroller_width: f32,
    alignment: Anchor,
    spacing: Option<f32>,
    padding: f32,
}

impl Default for Scrollbar {
    fn default() -> Self {
        Self {
            width: 10.0,
            margin: 0.0,
            scroller_width: 10.0,
            alignment: Anchor::Start,
            spacing: None,
            padding: 0.0,
        }
    }
}

impl Scrollbar {
    /// 创建一个用于 [`Scrollable`] 的新 [`Scrollbar`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates new [`Scrollbar`] for use in a [`Scrollable`].
    ///
    /// </details>
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建一个宽度为零的 [`Scrollbar`]，让 [`Scrollable`] 可以在没有可见
    /// 滚动块的情况下滚动。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Create a [`Scrollbar`] with zero width to allow a [`Scrollable`] to scroll without a visible
    /// scroller.
    ///
    /// </details>
    pub fn hidden() -> Self {
        Self::default().width(0).scroller_width(0)
    }

    /// 设置 [`Scrollbar`] 的滚动条宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the scrollbar width of the [`Scrollbar`] .
    ///
    /// </details>
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = width.into().0.max(0.0);
        self
    }

    /// 设置 [`Scrollbar`] 的滚动条外边距。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the scrollbar margin of the [`Scrollbar`] .
    ///
    /// </details>
    pub fn margin(mut self, margin: impl Into<Pixels>) -> Self {
        self.margin = margin.into().0;
        self
    }

    /// 设置 [`Scrollbar`] 的滚动块宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the scroller width of the [`Scrollbar`] .
    ///
    /// </details>
    pub fn scroller_width(mut self, scroller_width: impl Into<Pixels>) -> Self {
        self.scroller_width = scroller_width.into().0.max(0.0);
        self
    }

    /// 设置 [`Scrollbar`] 的 [`Anchor`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Anchor`] of the [`Scrollbar`] .
    ///
    /// </details>
    pub fn anchor(mut self, alignment: Anchor) -> Self {
        self.alignment = alignment;
        self
    }

    /// 设置 [`Scrollbar`] 是否应嵌入 [`Scrollable`] 之中，并在它与
    /// 内容之间使用给定的间距。
    ///
    /// 嵌入的 [`Scrollbar`] 会始终显示、占用布局空间，
    /// 并且不会浮动在内容之上。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets whether the [`Scrollbar`] should be embedded in the [`Scrollable`], using
    /// the given spacing between itself and the contents.
    ///
    /// An embedded [`Scrollbar`] will always be displayed, will take layout space,
    /// and will not float over the contents.
    ///
    /// </details>
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = Some(spacing.into().0);
        self
    }

    /// 设置 [`Scrollbar`] 的内边距。
    ///
    /// 当滚动条可见时，内边距会添加在它的顶部和底部
    /// （对于水平 [`Scrollbar`]，则是左右两端）。
    ///
    /// 与 [`Self::margin`] 和 [`Self::spacing`] 不同，内边距不会
    /// 影响 [`Scrollable`] 的布局。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the padding of the [`Scrollbar`].
    ///
    /// The padding is added at the top and bottom of the scrollbar (or at the
    /// left and right ends for a horizontal [`Scrollbar`]) when it is visible.
    ///
    /// Unlike [`Self::margin`] and [`Self::spacing`], the padding does not
    /// affect the layout of the [`Scrollable`].
    ///
    /// </details>
    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = padding.into().0;
        self
    }
}

/// [`Scrollable`] 的滚动块在给定轴上相对于其 [`Viewport`]
/// 的锚点。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The anchor of the scroller of the [`Scrollable`] relative to its [`Viewport`]
/// on a given axis.
///
/// </details>
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Anchor {
    /// 滚动块锚定到 [`Viewport`] 的起点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Scroller is anchored to the start of the [`Viewport`].
    ///
    /// </details>
    #[default]
    Start,
    /// 内容对齐到 [`Viewport`] 的终点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Content is aligned to the end of the [`Viewport`].
    ///
    /// </details>
    End,
}

impl<Message, W, Theme> widget::Meta for Scrollable<'_, Message, W, Theme> where Theme: Catalog {}

impl<Message, W, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Scrollable<'_, Message, W, Theme>
where
    Theme: Catalog,
    Renderer: text::Renderer,
    W: Widget<Message, Theme, Renderer>,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.content));

        let state = tree.state.downcast_mut::<State>();

        if state.last_id != self.id {
            *state = State {
                last_id: self.id.clone(),
                ..State::default()
            };
        }

        let size = self.content.size();

        self.width = self.width.stack(size.width);
        self.height = self.height.stack(size.height);
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let mut layout = |tree: &mut Tree, right_padding: f32, bottom_padding: f32| {
            let is_horizontal = self.direction.horizontal().is_some();
            let is_vertical = self.direction.vertical().is_some();

            layout::padded(
                tree,
                limits,
                self.width,
                self.height,
                Padding {
                    right: right_padding,
                    bottom: bottom_padding,
                    ..Padding::ZERO
                },
                |tree, limits| {
                    let child_limits = layout::Limits::with_flags(
                        limits.min,
                        limits.max,
                        limits.compression,
                        Size::new(
                            limits.infinite.width || is_horizontal,
                            limits.infinite.height || is_vertical,
                        ),
                    );

                    self.content.layout(tree, renderer, &child_limits);

                    tree.size
                },
            );
        };

        match self.direction {
            Direction::Vertical(Scrollbar {
                width,
                margin,
                spacing: Some(spacing),
                ..
            })
            | Direction::Horizontal(Scrollbar {
                width,
                margin,
                spacing: Some(spacing),
                ..
            }) => {
                let is_vertical = matches!(self.direction, Direction::Vertical(_));

                let padding = width + margin * 2.0 + spacing;
                let was_scrollbar_visible = tree.state.downcast_mut::<State>().is_scrollbar_visible;

                layout(
                    tree,
                    if is_vertical && was_scrollbar_visible {
                        padding
                    } else {
                        0.0
                    },
                    if !is_vertical && was_scrollbar_visible {
                        padding
                    } else {
                        0.0
                    },
                );

                let is_scrollbar_visible = if is_vertical {
                    tree.children[0].size.height > tree.size.height
                } else {
                    tree.children[0].size.width > tree.size.width
                };

                if was_scrollbar_visible != is_scrollbar_visible {
                    log::trace!("Scrollbar status quo has changed");
                    tree.state.downcast_mut::<State>().is_scrollbar_visible = is_scrollbar_visible;

                    layout(
                        tree,
                        if is_vertical && is_scrollbar_visible {
                            padding
                        } else {
                            0.0
                        },
                        if !is_vertical && is_scrollbar_visible {
                            padding
                        } else {
                            0.0
                        },
                    );
                }
            }
            _ => layout(tree, 0.0, 0.0),
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let bounds = layout.bounds();
        let (content_layout, content_tree) = layout.iter_mut(&mut tree.children).next().unwrap();
        let content_bounds = content_layout.bounds();
        let content = content_layout.size();

        let state = tree.state.downcast_mut::<State>();

        // `Animation::Auto` scroll operations resolve against this
        // `Animation::Auto` 的滚动操作会依据它来解析
        state.smooth_scroll = self.smooth_scroll;

        let translation = state.last_translation;
        let viewport = viewport.intersection(&bounds).unwrap_or_default() + translation;

        operation.scrollable(self.id.as_ref(), bounds, content, translation, state);
        operation.container(self.id.as_ref(), content_bounds, &viewport);

        operation.traverse(&mut |operation| {
            self.content
                .operate(content_tree, content_layout, &viewport, renderer, operation);
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let cursor_over_scrollable = cursor.position_over(bounds);

        let (content_layout, content_tree) = layout.iter_mut(&mut tree.children).next().unwrap();
        let content = content_layout.size();

        let state = tree.state.downcast_mut::<State>();
        let scrollbars = Scrollbars::new(state.last_translation, self.direction, bounds, content);
        let mouse_over_scrollbar = scrollbars.is_mouse_over(cursor);
        let last_offsets = (state.offset_x, state.offset_y);

        let interact = state.interact(
            event,
            bounds,
            content,
            cursor,
            cursor_over_scrollable,
            &scrollbars,
            mouse_over_scrollbar,
            self.direction,
            self.smooth_scroll,
            self.click_to_scroll,
        );

        if let Some(scroll) = interact.scroll
            && let Some(on_scroll) = &self.on_scroll
        {
            on_scroll(scroll).perform(state, bounds, content, shell);

            if let Event::Window(window::Event::RedrawRequested(_now)) = event {
                state.last_translation = state.translation(self.direction, bounds.size(), content);
            }
        }

        if interact.capture {
            shell.capture_event();
        }

        if interact.invalidate_layout {
            shell.invalidate_layout();
        }

        if interact.request_redraw {
            shell.request_redraw();
        }

        if !interact.stop {
            if let Some(Content { cursor }) = interact.content {
                let had_input_method = shell.input_method().is_enabled();

                let viewport =
                    bounds.intersection(viewport).unwrap_or_default() + state.last_translation;

                self.content.update(
                    content_tree,
                    event,
                    content_layout,
                    cursor,
                    renderer,
                    shell,
                    &viewport,
                );

                if !had_input_method
                    && let InputMethod::Enabled { cursor, .. } = shell.input_method_mut()
                {
                    *cursor -= state.last_translation;
                }
            }

            let update = state.update(
                event,
                bounds,
                content,
                cursor,
                cursor_over_scrollable,
                mouse_over_scrollbar,
                self.direction,
                self.smooth_scroll,
                self.auto_scroll,
                shell.is_event_captured(),
            );

            if let Some(scroll) = update.scroll
                && let Some(on_scroll) = &self.on_scroll
            {
                on_scroll(scroll).perform(state, bounds, content, shell);
            }

            if update.capture {
                shell.capture_event();
            }

            if update.invalidate_layout {
                shell.invalidate_layout();
            }

            if update.request_redraw {
                shell.request_redraw();
            }
        }

        let status = if let Some(axis) = state.interaction.axis() {
            Status::Dragged {
                is_horizontal_scrollbar_dragged: axis == Axis::X,
                is_vertical_scrollbar_dragged: axis == Axis::Y,
                is_horizontal_scrollbar_disabled: scrollbars.is_x_disabled(),
                is_vertical_scrollbar_disabled: scrollbars.is_y_disabled(),
            }
        } else if cursor_over_scrollable.is_some() {
            Status::Hovered {
                is_horizontal_scrollbar_hovered: mouse_over_scrollbar == Some(Axis::X),
                is_vertical_scrollbar_hovered: mouse_over_scrollbar == Some(Axis::Y),
                is_horizontal_scrollbar_disabled: scrollbars.is_x_disabled(),
                is_vertical_scrollbar_disabled: scrollbars.is_y_disabled(),
            }
        } else {
            Status::Active {
                is_horizontal_scrollbar_disabled: scrollbars.is_x_disabled(),
                is_vertical_scrollbar_disabled: scrollbars.is_y_disabled(),
            }
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            state.last_status = Some(status);
        }

        if last_offsets != (state.offset_x, state.offset_y)
            || state
                .last_status
                .is_some_and(|last_status| last_status != status)
        {
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        defaults: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();

        let Some(viewport) = viewport.intersection(&bounds) else {
            return;
        };

        let (content_layout, content_tree) = layout.iter(&tree.children).next().unwrap();
        let content = content_layout.size();

        let translation = state.last_translation;
        let scrollbars = Scrollbars::new(translation, self.direction, bounds, content);
        let cursor_over_scrollable = cursor.position_over(bounds);
        let mouse_over_scrollbar = scrollbars.is_mouse_over(cursor);

        let cursor = match cursor_over_scrollable {
            Some(cursor_position) if mouse_over_scrollbar.is_none() => {
                mouse::Cursor::Available(cursor_position + translation)
            }
            _ => cursor.obstruct() + translation,
        };

        let style = theme.style(
            &self.class,
            state.last_status.unwrap_or(Status::Active {
                is_horizontal_scrollbar_disabled: false,
                is_vertical_scrollbar_disabled: false,
            }),
        );

        container::draw_background(renderer, &style.container, layout.bounds());

        // Draw inner content
        // 绘制内部内容
        if scrollbars.active() {
            renderer.with_layer(viewport, |renderer| {
                renderer.with_translation(
                    -translation.hint(renderer.hint_factor().unwrap_or(1.0)),
                    |renderer| {
                        self.content.draw(
                            content_tree,
                            renderer,
                            theme,
                            defaults,
                            content_layout,
                            cursor,
                            &(viewport + translation),
                        );
                    },
                );
            });

            let draw_scrollbar =
                |renderer: &mut Renderer, style: Rail, scrollbar: &internals::Scrollbar| {
                    if scrollbar.bounds.width > 0.0
                        && scrollbar.bounds.height > 0.0
                        && (style.background.is_some()
                            || (style.border.color != Color::TRANSPARENT
                                && style.border.width > 0.0))
                    {
                        renderer.fill_quad(
                            renderer::Quad {
                                bounds: scrollbar.bounds,
                                border: style.border,
                                ..renderer::Quad::default()
                            },
                            style
                                .background
                                .unwrap_or(Background::Color(Color::TRANSPARENT)),
                        );
                    }

                    if let Some(scroller) = scrollbar.scroller
                        && scroller.bounds.width > 0.0
                        && scroller.bounds.height > 0.0
                        && (style.scroller.background != Background::Color(Color::TRANSPARENT)
                            || (style.scroller.border.color != Color::TRANSPARENT
                                && style.scroller.border.width > 0.0))
                    {
                        renderer.fill_quad(
                            renderer::Quad {
                                bounds: scroller.bounds,
                                border: style.scroller.border,
                                ..renderer::Quad::default()
                            },
                            style.scroller.background,
                        );
                    }
                };

            let has_floating_scrollbar = scrollbars.is_any_floating();

            if has_floating_scrollbar {
                renderer.start_layer(viewport);
            }

            if let Some(scrollbar) = scrollbars.y {
                draw_scrollbar(renderer, style.vertical_rail, &scrollbar);
            }

            if let Some(scrollbar) = scrollbars.x {
                draw_scrollbar(renderer, style.horizontal_rail, &scrollbar);
            }

            if let (Some(x), Some(y)) = (scrollbars.x, scrollbars.y) {
                let background = style.gap.or(style.container.background);

                if let Some(background) = background {
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: Rectangle {
                                x: y.bounds.x,
                                y: x.bounds.y,
                                width: y.bounds.width,
                                height: x.bounds.height,
                            },
                            ..renderer::Quad::default()
                        },
                        background,
                    );
                }
            }

            if has_floating_scrollbar {
                renderer.end_layer();
            }
        } else {
            self.content.draw(
                content_tree,
                renderer,
                theme,
                defaults,
                content_layout,
                cursor,
                &(viewport + translation),
            );
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let bounds = layout.bounds();
        let state = tree.state.downcast_ref::<State>();

        if state.interaction.scrollers_grabbed() {
            return mouse::Interaction::Idle;
        }

        let Some(viewport) = viewport.intersection(&bounds) else {
            return mouse::Interaction::None;
        };

        let cursor_over_scrollable = cursor.position_over(bounds);
        let (content_layout, content_tree) = layout.iter(&tree.children).next().unwrap();
        let content = content_layout.size();

        let translation = state.last_translation;
        let scrollbars = Scrollbars::new(translation, self.direction, bounds, content);
        let mouse_over_scrollbar = scrollbars.is_mouse_over(cursor);

        let cursor = match cursor_over_scrollable {
            Some(cursor_position) if mouse_over_scrollbar.is_none() => {
                mouse::Cursor::Available(cursor_position + translation)
            }
            _ => cursor.obstruct() + translation,
        };

        self.content.mouse_interaction(
            content_tree,
            content_layout,
            cursor,
            &(viewport + translation),
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Message, Theme, Renderer>> {
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let (content_layout, content_tree) = layout.iter_mut(&mut tree.children).next().unwrap();
        let content = content_layout.size();
        let viewport = viewport.intersection(&bounds).unwrap_or(*viewport);

        let offset = state.last_translation;

        let overlay = self.content.overlay(
            content_tree,
            content_layout,
            renderer,
            &(viewport + offset),
            translation - offset,
            window,
        );

        let icon = if let Interaction::AutoScrolling { origin, .. } = state.interaction {
            let scrollbars = Scrollbars::new(offset, self.direction, bounds, content);

            Some(overlay::Element::new(Box::new(AutoScrollIcon {
                origin,
                vertical: scrollbars.y.is_some(),
                horizontal: scrollbars.x.is_some(),
                class: &self.class,
            })))
        } else {
            None
        };

        overlay.into_iter().chain(icon).collect()
    }
}

struct AutoScrollIcon<'a, Class> {
    origin: Point,
    vertical: bool,
    horizontal: bool,
    class: &'a Class,
}

impl<Class> AutoScrollIcon<'_, Class> {
    const SIZE: f32 = 40.0;
    const DOT: f32 = Self::SIZE / 10.0;
    const PADDING: f32 = Self::SIZE / 10.0;
}

impl<Message, Theme, Renderer> core::Overlay<Message, Theme, Renderer>
    for AutoScrollIcon<'_, Theme::Class<'_>>
where
    Renderer: text::Renderer,
    Theme: Catalog,
{
    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        _cursor: mouse::Cursor,
    ) {
        let bounds = Rectangle::new(
            self.origin - Vector::new(Self::SIZE, Self::SIZE) / 2.0,
            Size::new(Self::SIZE, Self::SIZE),
        );

        let style = theme
            .style(
                self.class,
                Status::Active {
                    is_horizontal_scrollbar_disabled: false,
                    is_vertical_scrollbar_disabled: false,
                },
            )
            .auto_scroll;

        renderer.with_layer(bounds, |renderer| {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: style.border,
                    shadow: style.shadow,
                    snap: false,
                },
                style.background,
            );

            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle::new(
                        bounds.center() - Vector::new(Self::DOT, Self::DOT) / 2.0,
                        Size::new(Self::DOT, Self::DOT),
                    ),
                    border: border::rounded(bounds.width),
                    snap: false,
                    ..renderer::Quad::default()
                },
                style.icon,
            );

            let arrow = core::Text {
                content: String::new(),
                bounds: bounds.size(),
                size: Pixels::from(12),
                line_height: text::LineHeight::from(1.0),
                font: Renderer::ICON_FONT,
                align_x: text::Alignment::Center,
                align_y: alignment::Vertical::Center,
                shaping: text::Shaping::Basic,
                wrapping: text::Wrapping::None,
                ellipsis: text::Ellipsis::None,
                hint_factor: None,
            };

            if self.vertical {
                renderer.fill_text(
                    core::Text {
                        content: Renderer::SCROLL_UP_ICON.to_string(),
                        align_y: alignment::Vertical::Top,
                        ..arrow
                    },
                    Point::new(bounds.center_x(), bounds.y + Self::PADDING),
                    style.icon,
                    bounds,
                );

                renderer.fill_text(
                    core::Text {
                        content: Renderer::SCROLL_DOWN_ICON.to_string(),
                        align_y: alignment::Vertical::Bottom,
                        ..arrow
                    },
                    Point::new(
                        bounds.center_x(),
                        bounds.y + bounds.height - Self::PADDING - 0.5,
                    ),
                    style.icon,
                    bounds,
                );
            }

            if self.horizontal {
                renderer.fill_text(
                    core::Text {
                        content: Renderer::SCROLL_LEFT_ICON.to_string(),
                        align_x: text::Alignment::Left,
                        ..arrow
                    },
                    Point::new(bounds.x + Self::PADDING + 1.0, bounds.center_y() + 1.0),
                    style.icon,
                    bounds,
                );

                renderer.fill_text(
                    core::Text {
                        content: Renderer::SCROLL_RIGHT_ICON.to_string(),
                        align_x: text::Alignment::Right,
                        ..arrow
                    },
                    Point::new(
                        bounds.x + bounds.width - Self::PADDING - 1.0,
                        bounds.center_y() + 1.0,
                    ),
                    style.icon,
                    bounds,
                );
            }
        });
    }

    fn index(&self) -> f32 {
        f32::MAX
    }
}

/// 被按住的轨道上指针的新位置（如果它移动过）。
///
/// 指针移动后需要重新评估自动滚动的暂停和停止条件，
/// 因此它会通过一次重绘唤醒动画。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The new position of the pointer of a held rail, if it moved.
///
/// A moved pointer re-evaluates the autoscroll's pause and stop conditions,
/// so it wakes the animation with a redraw.
///
/// </details>
fn rail_moved(event: &Event, cursor: mouse::Cursor, last: Point) -> Option<Point> {
    if matches!(
        event,
        Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. })
    ) {
        cursor
            .observe()
            .position()
            .filter(|position| *position != last)
    } else {
        None
    }
}

/// [`State::interact`] 请求委托给子部件的
/// 内容更新。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The content update that [`State::interact`] requests to be delegated to the
/// child widget.
///
/// </details>
#[derive(Debug)]
struct Content {
    /// 传递给子部件的 [`mouse::Cursor`]
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`mouse::Cursor`] to pass to the child widget
    ///
    /// </details>
    cursor: mouse::Cursor,
}

/// [`State::interact`] 的结果：需要在 [`Shell`] 上落实的效果，
/// 以及事件是否必须委托给内容。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The outcome of [`State::interact`]: the [`Shell`] effects to materialize and
/// whether the event must be delegated to the content.
///
/// </details>
#[derive(Debug, Default)]
struct Interact {
    /// 滚动发生变化时要发布的滚动通知：事件之后的
    /// 最终状态，合并了所有中间变化
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The scroll notification to publish, if the scroll changed: the final
    /// state after the event, coalescing any intermediate changes
    ///
    /// </details>
    scroll: Option<Scroll>,

    /// 是否必须捕获该事件
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the event must be captured
    ///
    /// </details>
    capture: bool,

    /// 是否必须使布局失效
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the layout must be invalidated
    ///
    /// </details>
    invalidate_layout: bool,

    /// 是否必须请求重绘
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether a redraw must be requested
    ///
    /// </details>
    request_redraw: bool,

    /// 交互是否提前结束：事件既不能委托
    /// 给内容，也不能由 [`State::update`] 处理
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the interaction stopped early: the event must not be delegated
    /// to the content, nor processed by [`State::update`]
    ///
    /// </details>
    stop: bool,

    /// 如果为 `Some`，事件必须连同给定的光标和视口
    /// 委托给内容
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// If `Some`, the event must be delegated to the content with the given
    /// cursor and viewport
    ///
    /// </details>
    content: Option<Content>,
}

/// [`State::update`] 的结果：需要在 [`Shell`] 上落实的效果
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The outcome of [`State::update`]: the [`Shell`] effects to materialize
///
/// </details>
#[derive(Debug, Default)]
struct Update {
    /// 滚动发生变化时要发布的滚动通知：事件之后的
    /// 最终状态，合并了所有中间变化
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The scroll notification to publish, if the scroll changed: the final
    /// state after the event, coalescing any intermediate changes
    ///
    /// </details>
    scroll: Option<Scroll>,

    /// 是否必须捕获该事件
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the event must be captured
    ///
    /// </details>
    capture: bool,

    /// 是否必须使布局失效
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the layout must be invalidated
    ///
    /// </details>
    invalidate_layout: bool,

    /// 是否必须请求重绘
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether a redraw must be requested
    ///
    /// </details>
    request_redraw: bool,
}

#[derive(Debug, Clone)]
struct State {
    offset_y: Offset,
    offset_x: Offset,
    smooth_scroll: bool,
    target: Option<Target>,
    source: Option<Source>,
    segment: Segment,
    last_frame: Option<Instant>,
    last_translation: Vector,
    interaction: Interaction,
    keyboard_modifiers: keyboard::Modifiers,
    last_notified: Option<Viewport>,
    last_scrolled: Option<Instant>,
    is_scrollbar_visible: bool,
    last_status: Option<Status>,
    last_id: Option<widget::Id>,
}

#[derive(Debug, Clone, Copy)]
struct Segment {
    start: Point,
    started: Instant,
    duration: f32,
    slope: f32,
}

#[derive(Debug, Clone, Copy)]
enum Interaction {
    None,
    ScrollerGrabbed(Axis, f32),
    TouchScrolling(Point),
    AutoScrolling {
        origin: Point,
        current: Point,
        last_frame: Option<Instant>,
    },
    RailHeld(RailHeld),
}

impl Interaction {
    /// 当前滚动条交互（拖动滚动块或按住轨道）
    /// 所在的轴（如果有）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The axis of the active scrollbar interaction — a scroller drag or a
    /// held rail — if any.
    ///
    /// </details>
    fn axis(&self) -> Option<Axis> {
        match self {
            Interaction::ScrollerGrabbed(axis, _) => Some(*axis),
            Interaction::RailHeld(rail) => Some(rail.axis),
            _ => None,
        }
    }

    /// 用户是否正在与滚动条交互：拖动
    /// 滚动块或按住轨道。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the user is interacting with a scrollbar: dragging a
    /// scroller or holding a rail pressed.
    ///
    /// </details>
    fn scrollers_grabbed(&self) -> bool {
        self.axis().is_some()
    }

    /// 如果正在拖动滚动块，返回所在的轴以及
    /// 滚动块被抓取位置的比例。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The axis and the fraction of the scroller grabbed, if a scroller is
    /// being dragged.
    ///
    /// </details>
    fn scroller_grabbed(&self) -> Option<(Axis, f32)> {
        let Interaction::ScrollerGrabbed(axis, grabbed_at) = self else {
            return None;
        };

        Some((*axis, *grabbed_at))
    }
}

/// 轨道正被按住的 [`Scrollable`] 的状态。
///
/// 对轨道的普通按下会（平滑地）滚动一页，并在
/// [`State::RAIL_AUTOSCROLL_DELAY`] 之后以恒定速度自动滚动，
/// 直到滚动块到达指针，这与 Chromium 的轨道
/// 自动滚动（`cc::ScrollbarController`）一致。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The state of a [`Scrollable`] whose rail is being held pressed.
///
/// A plain rail press scrolls one page (smoothly) and, after
/// [`State::RAIL_AUTOSCROLL_DELAY`], autoscrolls at a constant velocity
/// until the scroller reaches the pointer, mirroring Chromium's track
/// autoscroll (`cc::ScrollbarController`).
///
/// </details>
#[derive(Debug, Clone, Copy)]
struct RailHeld {
    /// 轨道被按下时所在的轴
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The axis the rail was pressed on
    ///
    /// </details>
    axis: Axis,

    /// 自动滚动的方向，以偏移量单位计（`+1.0` 或
    /// `-1.0`），在轨道被按下时确定
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The direction of the autoscroll, in offset units (`+1.0` or
    /// `-1.0`), fixed when the rail was pressed
    ///
    /// </details>
    direction: f32,

    /// 轨道被按下的位置在滚动块的哪一侧，以光标
    /// 单位计：`+1.0` 表示在其下方，`-1.0` 表示在其上方。
    ///
    /// 它与锚点的镜像无关，并决定了
    /// 自动滚动停止时所对齐的滚动块边缘（它的前缘）以及
    /// 使自动滚动暂停的轨道部分。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Which side of the scroller the rail was pressed on, in cursor
    /// units: `+1.0` below it, `-1.0` above it.
    ///
    /// This is independent of the anchor's mirroring, and selects the
    /// scroller edge the autoscroll stops at (its leading edge) and the
    /// track part that pauses it.
    ///
    /// </details>
    cursor_direction: f32,

    /// 自动滚动的速度，单位为像素每秒
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The autoscroll velocity, in pixels per second
    ///
    /// </details>
    velocity: f32,

    /// 轨道被按下的时刻
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// When the rail was pressed
    ///
    /// </details>
    pressed_at: Instant,

    /// 指针的最新位置
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The latest pointer position
    ///
    /// </details>
    pointer: Point,

    /// 自动滚动上一次步进所对应的帧，用于防止
    /// 在同一时刻步进两次
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The last frame the autoscroll was stepped for, guarding against
    /// stepping twice for the same instant
    ///
    /// </details>
    last_frame: Option<Instant>,
}

/// 对被按住的轨道的自动滚动进行一次步进的结果。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The outcome of stepping a held rail's autoscroll.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RailStep {
    /// 偏移量发生了移动；应当发出滚动通知并重绘
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The offset moved; the scroll should be notified and redrawn
    ///
    /// </details>
    Moved,

    /// 自动滚动尚未开始（仍在按下延迟之内）；必须
    /// 继续产生帧
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The autoscroll has not started yet (within the press delay); frames
    /// must keep coming
    ///
    /// </details>
    Waiting,

    /// 在指针再次移动之前，没有任何东西可以移动
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Nothing can move until the pointer moves again
    ///
    /// </details>
    Idle,
}

/// [`Scrollable`] 内容的轴。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The axis of the [`Scrollable`] content.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Axis {
    /// 水平轴。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The horizontal axis.
    ///
    /// </details>
    X,

    /// 垂直轴。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The vertical axis.
    ///
    /// </details>
    Y,
}

impl Axis {
    /// 给定尺寸沿这个轴的长度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The length of the given size along this axis.
    ///
    /// </details>
    fn length(self, size: impl Into<Size>) -> f32 {
        let size = size.into();
        match self {
            Axis::X => size.width,
            Axis::Y => size.height,
        }
    }

    /// 给定的 `point` 沿这个轴的坐标。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The coordinate of the given `point` along this axis.
    ///
    /// </details>
    fn coordinate(self, point: Point) -> f32 {
        match self {
            Axis::X => point.x,
            Axis::Y => point.y,
        }
    }

    /// 在这个轴上为给定的 `value`、在另一个轴上为 `0.0` 的
    /// [`Vector`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A [`Vector`] with the given `value` on this axis and `0.0` on the
    /// other.
    ///
    /// </details>
    fn vector(self, value: f32) -> Vector {
        match self {
            Axis::X => Vector::new(value, 0.0),
            Axis::Y => Vector::new(0.0, value),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            offset_y: Offset::Absolute(0.0),
            offset_x: Offset::Absolute(0.0),
            smooth_scroll: true,
            target: None,
            source: None,
            segment: Segment {
                start: Point::ORIGIN,
                started: Instant::now(),
                duration: 0.0,
                slope: 0.0,
            },
            last_frame: None,
            last_translation: Vector::ZERO,
            interaction: Interaction::None,
            keyboard_modifiers: keyboard::Modifiers::default(),
            last_notified: None,
            last_scrolled: None,
            is_scrollbar_visible: true,
            last_status: None,
            last_id: None,
        }
    }
}

impl operation::Scrollable for State {
    fn snap_to(
        &mut self,
        offset: RelativeOffset<Option<f32>>,
        animation: Animation,
        bounds: Rectangle,
        content: Size,
    ) {
        State::snap_to(self, offset, animation, bounds, content, Source::Operation);
    }

    fn scroll_to(
        &mut self,
        offset: AbsoluteOffset<Option<f32>>,
        animation: Animation,
        bounds: Rectangle,
        content: Size,
    ) {
        State::scroll_to(self, offset, animation, bounds, content, Source::Operation);
    }

    fn scroll_by(
        &mut self,
        offset: AbsoluteOffset,
        animation: Animation,
        bounds: Rectangle,
        content: Size,
    ) {
        State::scroll_by(self, offset, animation, bounds, content);
    }
}

/// [`Scrollable`] 的偏移量，使用绝对单位或相对单位。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An offset of a [`Scrollable`], in either absolute or relative units.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Offset {
    /// 绝对偏移量，以像素为单位。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An absolute offset, in pixels.
    ///
    /// </details>
    Absolute(f32),

    /// 相对偏移量，表示为可滚动范围的比例。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A relative offset, as a fraction of the scrollable range.
    ///
    /// </details>
    Relative(f32),
}

impl Offset {
    /// 返回这个偏移量是否是吸附的，即相对的。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns whether this offset is snapped, i.e. relative.
    ///
    /// </details>
    pub fn is_snapped(self) -> bool {
        matches!(self, Offset::Relative(_))
    }

    fn absolute(self, viewport: f32, content: f32) -> f32 {
        match self {
            Offset::Absolute(absolute) => absolute.min((content - viewport).max(0.0)),
            Offset::Relative(percentage) => ((content - viewport) * percentage).max(0.0),
        }
    }

    fn translation(self, viewport: f32, content: f32, alignment: Anchor) -> f32 {
        let offset = self.absolute(viewport, content);

        match alignment {
            Anchor::Start => offset,
            Anchor::End => ((content - viewport).max(0.0) - offset).max(0.0),
        }
    }
}

/// 正在进行的平滑滚动的目标。
///
/// 这些偏移量是滚动最终停留的位置，因此相对（吸附）的
/// 偏移量会保持吸附。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The target of an in-progress smooth scroll.
///
/// The offsets are what the scroll settles at, so relative (snapped)
/// offsets stay snapped.
///
/// </details>
#[derive(Debug, Clone, Copy)]
pub struct Target {
    /// 目标的 X 偏移量，以 [`Offset`] 为单位。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The X offset of the target, in [`Offset`] units.
    ///
    /// </details>
    pub x: Offset,

    /// 目标的 Y 偏移量，以 [`Offset`] 为单位。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Y offset of the target, in [`Offset`] units.
    ///
    /// </details>
    pub y: Offset,

    /// 这些偏移量在当前边界下解析出的结果，也就是
    /// 当前运行的动画段所缓动到的点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The resolution of the offsets at the current bounds, the point the
    /// running segment eases towards.
    ///
    /// </details>
    pub destination: Vector,
}

impl Target {
    /// 把给定的偏移量解析为一个 [`Target`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Resolves the given offsets into a [`Target`].
    ///
    /// </details>
    fn new(x: Offset, y: Offset, bounds: Rectangle, content: Size) -> Self {
        let destination = Vector::new(
            x.absolute(bounds.width, content.width),
            y.absolute(bounds.height, content.height),
        );

        Self { x, y, destination }
    }

    /// 最终停留在给定的绝对 `destination` 处的 [`Target`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A [`Target`] that settles at the given absolute `destination`.
    ///
    /// </details>
    fn absolute(destination: Vector) -> Self {
        Self {
            x: Offset::Absolute(destination.x),
            y: Offset::Absolute(destination.y),
            destination,
        }
    }
}

/// 滚动通知的来源。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The source of a scroll notification.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    /// 用户使用鼠标滚轮或触摸板滚动。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The user scrolled with a mouse wheel or touchpad.
    ///
    /// </details>
    Wheel,

    /// 用户通过触摸滑动来滚动。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The user scrolled by swiping with a touch.
    ///
    /// </details>
    Touch,

    /// 用户通过拖动滚动条的滚动块来滚动。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The user scrolled by dragging the scrollbar's scroller.
    ///
    /// </details>
    Scrollbar,

    /// 用户通过鼠标中键自动滚动来滚动。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The user scrolled by auto-scrolling with the middle mouse button.
    ///
    /// </details>
    AutoScroll,

    /// 由滚动操作（`scroll_to`、`snap_to` 或 `scroll_by`）引起的滚动。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A scroll operation (`scroll_to`, `snap_to` or `scroll_by`) scrolled.
    ///
    /// </details>
    Operation,

    /// 由滚动 [`Action`] 引起的滚动。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A scroll [`Action`] scrolled.
    ///
    /// </details>
    Action,

    /// 内容的大小发生了变化。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The size of the content changed.
    ///
    /// </details>
    Content,

    /// [`Scrollable`] 的大小发生了变化。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The size of the [`Scrollable`] changed.
    ///
    /// </details>
    Resize,
}

/// 描述 [`Scrollable`] 的一次滚动的事件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An event describing a scroll of a [`Scrollable`].
///
/// </details>
#[derive(Debug, Clone, Copy)]
pub struct Scroll {
    /// [`Scrollable`] 的 [`Viewport`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Viewport`] of the [`Scrollable`].
    ///
    /// </details>
    pub viewport: Viewport,

    /// 滚动事件之前的原始 [`Viewport`]（如果有）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The original [`Viewport`] before the scroll event, if any.
    ///
    /// </details>
    pub origin: Option<Viewport>,

    /// 滚动的 [`Source`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Source`] of the scroll.
    ///
    /// </details>
    pub source: Source,

    /// 滚动的 [`Target`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Target`] of the scroll.
    ///
    /// </details>
    pub target: Option<Target>,
}

impl Scroll {
    /// 返回这次滚动最终停留的 [`Viewport`]。
    ///
    /// 平滑滚动动画进行期间，返回的
    /// [`Viewport`] 带有 [`Target`] 的偏移量（也就是
    /// 滚动最终停留的偏移量），而不是当前的偏移量。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Viewport`] that this scroll settles at.
    ///
    /// While a smooth scrolling animation is in progress, the returned
    /// [`Viewport`] carries the [`Target`]'s offsets — the offsets the
    /// scroll settles at — instead of the current ones.
    ///
    /// </details>
    pub fn destination(&self) -> Viewport {
        Viewport {
            x: self
                .target
                .map(|target| target.x)
                .unwrap_or(self.viewport.x),
            y: self
                .target
                .map(|target| target.y)
                .unwrap_or(self.viewport.y),
            ..self.viewport
        }
    }
}

/// [`Scrollable`] 当前的 [`Viewport`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The current [`Viewport`] of the [`Scrollable`].
///
/// </details>
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    /// 当前的 X 偏移量，以 [`Offset`] 为单位。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The current X offset, in [`Offset`] units.
    ///
    /// </details>
    pub x: Offset,

    /// 当前的 Y 偏移量，以 [`Offset`] 为单位。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The current Y offset, in [`Offset`] units.
    ///
    /// </details>
    pub y: Offset,

    /// [`Scrollable`] 的边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The bounds of the [`Scrollable`].
    ///
    /// </details>
    pub bounds: Rectangle,

    /// [`Scrollable`] 内容的大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The size of the content of the [`Scrollable`].
    ///
    /// </details>
    pub content: Size,
}

impl Viewport {
    /// 返回 [`Viewport`] 的终点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the end of the [`Viewport`].
    ///
    /// </details>
    pub fn end(&self) -> AbsoluteOffset {
        self.absolute_offset() + self.distance_to_end()
    }

    /// 返回一个与给定视口的坐标对齐的
    /// 新 [`Viewport`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a new [`Viewport`] aligned with the coordinates of the
    /// given one.
    ///
    /// </details>
    pub fn slide(self, other: Self) -> Self {
        Self {
            x: other.x,
            y: other.y,
            ..self
        }
    }

    /// 返回各个轴上从当前滚动位置到内容末尾的
    /// 距离，以像素为单位。
    ///
    /// 这是还可以滚动进入视野的内容量：
    /// 当内容放得下，或者已经滚动到末尾时，它为零。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the distance from the current scroll position to the end of
    /// the content, in pixels, per axis.
    ///
    /// This is the amount of content that can still be scrolled into view:
    /// it is zero when the content fits, or when the scroll is at the end.
    ///
    /// </details>
    pub fn distance_to_end(&self) -> Vector {
        let AbsoluteOffset { x, y } = self.absolute_offset();

        Vector::new(
            (self.content.width - self.bounds.width - x).max(0.0),
            (self.content.height - self.bounds.height - y).max(0.0),
        )
    }

    /// 返回当前 [`Viewport`] 的 [`AbsoluteOffset`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`AbsoluteOffset`] of the current [`Viewport`].
    ///
    /// </details>
    pub fn absolute_offset(&self) -> AbsoluteOffset {
        let x = self.x.absolute(self.bounds.width, self.content.width);
        let y = self.y.absolute(self.bounds.height, self.content.height);

        AbsoluteOffset { x, y }
    }

    /// 返回当前 [`Viewport`] 的 [`AbsoluteOffset`]，但把它的
    /// 对齐方式反转。
    ///
    /// 在切换 [`Scrollable`] 的对齐方式、同时保持其
    /// 滚动位置时，这个方法很有用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`AbsoluteOffset`] of the current [`Viewport`], but with its
    /// alignment reversed.
    ///
    /// This method can be useful to switch the alignment of a [`Scrollable`]
    /// while maintaining its scrolling position.
    ///
    /// </details>
    pub fn absolute_offset_reversed(&self) -> AbsoluteOffset {
        let AbsoluteOffset { x, y } = self.absolute_offset();

        AbsoluteOffset {
            x: (self.content.width - self.bounds.width).max(0.0) - x,
            y: (self.content.height - self.bounds.height).max(0.0) - y,
        }
    }

    /// 返回当前 [`Viewport`] 的 [`RelativeOffset`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`RelativeOffset`] of the current [`Viewport`].
    ///
    /// </details>
    pub fn relative_offset(&self) -> RelativeOffset {
        let AbsoluteOffset { x, y } = self.absolute_offset();

        let x = x / (self.content.width - self.bounds.width);
        let y = y / (self.content.height - self.bounds.height);

        RelativeOffset { x, y }
    }
}

/// 为响应 [`Scroll`] 而执行的动作。
///
/// 这是 [`Scrollable::on_scroll`] 处理函数的返回类型。它让
/// 处理函数可以通过进一步驱动 [`Scrollable`]，
/// 或者发布一条消息，来响应滚动通知。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An action to perform in response to a [`Scroll`].
///
/// This is the return type of the [`Scrollable::on_scroll`] handler. It lets
/// the handler react to a scroll notification by driving the [`Scrollable`]
/// further, or by publishing a message.
///
/// </details>
#[derive(Debug)]
pub enum Action<Message> {
    /// 什么也不做。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Do nothing.
    ///
    /// </details>
    None,

    /// 使用给定的 [`Animation`] 滚动到给定的 [`AbsoluteOffset`]。
    ///
    /// 设为 `None` 的轴保持当前位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Scroll to the given [`AbsoluteOffset`], with the given [`Animation`].
    ///
    /// An axis set to `None` keeps its current position.
    ///
    /// </details>
    ScrollTo(AbsoluteOffset<Option<f32>>, Animation),

    /// 使用给定的 [`Animation`] 吸附到给定的 [`RelativeOffset`]。
    ///
    /// 设为 `None` 的轴保持当前位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Snap to the given [`RelativeOffset`], with the given [`Animation`].
    ///
    /// An axis set to `None` keeps its current position.
    ///
    /// </details>
    SnapTo(RelativeOffset<Option<f32>>, Animation),

    /// 发布给定的消息。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Publish the given message.
    ///
    /// </details>
    Custom(Message),
}

impl<Message> Action<Message> {
    fn perform(
        self,
        state: &mut State,
        bounds: Rectangle,
        content: Size,
        shell: &mut Shell<'_, Message>,
    ) {
        match self {
            Action::None => {}
            Action::ScrollTo(absolute_offset, animation) => {
                state.scroll_to(absolute_offset, animation, bounds, content, Source::Action);
            }
            Action::SnapTo(relative_offset, animation) => {
                state.snap_to(relative_offset, animation, bounds, content, Source::Action);
            }
            Action::Custom(message) => {
                shell.publish(message);
            }
        }
    }
}

impl<Message> From<Message> for Action<Message> {
    fn from(message: Message) -> Self {
        Self::Custom(message)
    }
}

impl<Message> From<Option<Message>> for Action<Message> {
    fn from(message: Option<Message>) -> Self {
        match message {
            Some(message) => Self::Custom(message),
            None => Self::None,
        }
    }
}

/// 滚轮每滚动一行所对应的距离（以逻辑像素为单位）。
///
/// Chromium 对传统滚轮的每一格固定滚动 120 个 CSS 像素，
/// 与页面的行高无关：操作系统的增量被归一化为
/// 120 个单位（`ui::MouseWheelEvent::kWheelDelta`），并按 1:1 传递。
///
/// 这个值假定平台每一格报告一行（例如 X11）。
/// 在每一格报告三行的平台上，`40.0`（Chromium 的
/// `cc::kPixelsPerLineStep`）是与之等价的值。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The distance (in logical pixels) scrolled per wheel line.
///
/// Chromium scrolls a fixed 120 CSS pixels per classic wheel notch,
/// independent of the page's line height: the OS delta is normalized to
/// 120 units (`ui::MouseWheelEvent::kWheelDelta`) and passed through 1:1.
///
/// This value assumes the platform reports one line per notch (e.g. X11).
/// On platforms that report three lines per notch, `40.0` (Chromium's
/// `cc::kPixelsPerLineStep`) is the equivalent value.
///
/// </details>
const WHEEL_PX_PER_LINE: f32 = 120.0;

// The smooth scrolling behavior below (the easing curve, the animation
// durations, and the velocity-preserving retargeting) is derived from
// the Chromium project's wheel scroll animation
// (`cc/animation/scroll_offset_animation_curve.{h,cc}`), which is
// licensed under the BSD 3-Clause license:
// <https://chromium.googlesource.com/chromium/src/+/main/LICENSE>
// 下面的平滑滚动行为（缓动曲线、动画
// 时长，以及保持速度的重定目标）源自
// Chromium 项目的滚轮滚动动画
// （`cc/animation/scroll_offset_animation_curve.{h,cc}`），它
// 采用 BSD 3-Clause 许可证：
// <https://chromium.googlesource.com/chromium/src/+/main/LICENSE>

/// 平滑滚动缓动曲线的控制点。
///
/// 这是标准的 ease-in-out 三次贝塞尔曲线，Chromium 也把它用于
/// 滚轮滚动：滚动从静止开始，在中途达到峰值，
/// 并在目标处静止下来。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The control points of the smooth scrolling easing curve.
///
/// This is the standard ease-in-out cubic bezier, as used by Chromium for
/// wheel scrolling: the scroll starts from rest, peaks mid-way, and
/// settles on the target at rest.
///
/// </details>
const SMOOTH_SCROLL_BEZIER_X1: f32 = 0.42;
const SMOOTH_SCROLL_BEZIER_X2: f32 = 0.58;

/// 平滑滚动动画时长的除数，与
/// Chromium 的取值一致。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The divisor of the smooth scrolling animation duration, matching
/// Chromium's.
///
/// </details>
const SMOOTH_SCROLL_DURATION_DIVISOR: f32 = 60.0;

/// 滚轮事件与下一个绘制帧之间的延迟（以秒为单位）。
///
/// 滚轮事件在帧与帧之间处理，而平滑滚动
/// 动画只在绘制下一帧时才会步进。让
/// 动画在事件*之前*一个标称帧开始（一个标称帧
/// 是 `SMOOTH_SCROLL_DURATION_DIVISOR` 的一个单位），可以确保
/// 第一个绘制帧就已经显示出一些进展，而不是重复
/// 上一帧。
///
/// 这里使用一整帧，而不是半帧的平均延迟：它
/// 保证第一个绘制帧有可见的移动，即使
/// 事件恰好在一帧之后到达。这样动画会提前一帧
/// 静止下来，但由于曲线以静止结束，这是察觉不到的。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The delay (in seconds) between a wheel event and the next drawn frame.
///
/// Wheel events are processed between frames, and the smooth scrolling
/// animation is only stepped when the next frame is drawn. Starting the
/// animation one nominal frame *before* the event, where a nominal frame
/// is one unit of `SMOOTH_SCROLL_DURATION_DIVISOR`, ensures that the
/// first drawn frame already shows some progress, instead of repeating
/// the previous one.
///
/// A full frame is used rather than the mean delay of half a frame: it
/// guarantees that the first drawn frame visibly moves, even when the
/// event arrives right after a frame. The animation then settles one
/// frame early, which is imperceptible since the curve ends at rest.
///
/// </details>
const SMOOTH_SCROLL_FRAME_DELAY: f32 = 1.0 / SMOOTH_SCROLL_DURATION_DIVISOR;

/// 平滑滚动时长的渐变区间起止处的
/// 距离（以像素为单位）。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The distances (in pixels) at which the smooth scrolling duration ramp
/// starts and ends.
///
/// </details>
const SMOOTH_SCROLL_DURATION_RAMP_START: f32 = WHEEL_PX_PER_LINE;
const SMOOTH_SCROLL_DURATION_RAMP_END: f32 = 480.0;

/// 平滑滚动动画的最短和最长时长，以
/// `SMOOTH_SCROLL_DURATION_DIVISOR` 为单位，与 Chromium 的取值一致。
///
/// 在这个范围内，时长与距离成*反比*：
/// 短距离滚动的动画更长（更柔和），而长距离
/// 滚动的动画更短（更干脆）。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The shortest and longest smooth scrolling animation durations, in
/// `SMOOTH_SCROLL_DURATION_DIVISOR` units, matching Chromium's.
///
/// The duration is *inversely* proportional to the distance within these
/// bounds: short scrolls get a longer (softer) animation, while long
/// scrolls get a shorter (snappier) one.
///
/// </details>
const SMOOTH_SCROLL_DURATION_MIN: f32 = 6.0;
const SMOOTH_SCROLL_DURATION_MAX: f32 = 12.0;

/// 对正在运行的动画重定目标时，应用于“以当前速度
/// 走完新距离所需时间”的系数，与
/// Chromium 的取值一致。
///
/// 用它来限制新的时长，可以避免在高速滚动时加入一个
/// 很小的新增量而出现“橡皮筋”效应。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The factor applied to the time it would take to cover the new distance
/// at the current velocity when retargeting a running animation, matching
/// Chromium's.
///
/// Bounding the new duration by this keeps a fast scroll from "rubber
/// banding" when a small new delta is added at high velocity.
///
/// </details>
const SMOOTH_SCROLL_RETARGET_VELOCITY_BOUND: f32 = 2.5;

/// 重定目标后的动画初始斜率的钳制范围，与
/// Chromium 的取值一致。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The clamp for the initial slope of a retargeted animation, matching
/// Chromium's.
///
/// </details>
const SMOOTH_SCROLL_SLOPE_CLAMP: f32 = 1000.0;

// The held-rail behavior below (the press delay, the autoscroll
// velocity, and the page step) is derived from the Chromium
// project's compositor scrollbar controller (`cc/input/
// scrollbar_controller.{h,cc}` and `cc/input/scrollbar.h`), which is
// licensed under the BSD 3-Clause license:
// <https://chromium.googlesource.com/chromium/src/+/main/LICENSE>
// 下面的按住轨道行为（按下延迟、自动滚动
// 速度，以及翻页步长）源自 Chromium
// 项目的合成器滚动条控制器
// （`cc/input/scrollbar_controller.{h,cc}` 和 `cc/input/scrollbar.h`），它
// 采用 BSD 3-Clause 许可证：
// <https://chromium.googlesource.com/chromium/src/+/main/LICENSE>

/// 从按下轨道到开始按住轨道自动滚动之间的
/// 延迟，与 Chromium 的 `cc::kInitialAutoscrollTimerDelay` 一致。
///
/// 在延迟期间，只会应用最初的翻页步长（像滚轮滚动
/// 一样带有动画）；因此快速点击一次恰好滚动一页。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The delay between a rail press and the start of the held-rail
/// autoscroll, matching Chromium's `cc::kInitialAutoscrollTimerDelay`.
///
/// During the delay, only the initial page step (animated like a wheel
/// scroll) is applied; a quick click therefore scrolls exactly one page.
///
/// </details>
const RAIL_AUTOSCROLL_DELAY: Duration = Duration::from_millis(250);

/// 把轨道的翻页步长换算为按住轨道自动滚动
/// 速度的系数，与 Chromium 的 `cc::kAutoscrollMultiplier` 一致。
///
/// Chromium 的主线程自动滚动每 50 毫秒应用一次翻页步长；
/// 等价的恒定速度就是步长乘以 20。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The factor converting a rail page step into the held-rail autoscroll
/// velocity, matching Chromium's `cc::kAutoscrollMultiplier`.
///
/// Chromium's main thread autoscroll applies the page step every 50 ms;
/// the equivalent constant velocity is the step times 20.
///
/// </details>
const RAIL_AUTOSCROLL_MULTIPLIER: f32 = 20.0;

/// 点击轨道时最初的翻页步长所覆盖的视口
/// 比例，与 Chromium 的 `cc::kMinFractionToStepWhenPaging`
/// （非 Mac 平台的翻页步长）一致。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The fraction of the viewport covered by a rail click's initial page
/// step, matching Chromium's `cc::kMinFractionToStepWhenPaging` (the
/// non-Mac page step).
///
/// </details>
const RAIL_PAGE_STEP_FRACTION: f32 = 0.875;

/// 自动滚动（鼠标中键）时忽略移动的
/// 距离范围（以像素为单位）。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The distance (in pixels) within which the auto-scroll (middle mouse
/// button) movement is ignored.
///
/// </details>
const AUTOSCROLL_DEADZONE: f32 = 20.0;

/// 自动滚动速度曲线的指数，它让
/// 自动滚动随着与起点距离的增大而加速。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The exponent of the auto-scroll velocity curve, which makes the
/// auto-scroll accelerate with the distance from the origin.
///
/// </details>
const AUTOSCROLL_SMOOTHNESS: f32 = 1.5;

impl State {
    /// 给定可滚动视口的长度时，点击轨道最初的
    /// 翻页步长的距离（以像素为单位）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The distance (in pixels) of a rail click's initial page step, given
    /// the length of the scrollable viewport.
    ///
    /// </details>
    fn rail_page_step(viewport: f32) -> f32 {
        (viewport * RAIL_PAGE_STEP_FRACTION).max(1.0)
    }

    fn new() -> Self {
        State::default()
    }

    fn scroll(&mut self, delta: Vector<f32>, bounds: Rectangle, content: Size) {
        self.cancel();

        if bounds.height < content.height {
            self.offset_y = Offset::Absolute(
                (self.offset_y.absolute(bounds.height, content.height) + delta.y)
                    .clamp(0.0, content.height - bounds.height),
            );
        }

        if bounds.width < content.width {
            self.offset_x = Offset::Absolute(
                (self.offset_x.absolute(bounds.width, content.width) + delta.x)
                    .clamp(0.0, content.width - bounds.width),
            );
        }
    }

    /// 把*目标*滚动偏移量移动 `delta`，用于平滑滚动。
    ///
    /// 增量会累加到待处理的目标（如果有）上，这样
    /// 快速的滚轮移动就不会丢失（尚未滚动的）距离；
    /// 随后通过 [`State::scroll_smoothly_to`] 对目标进行动画。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Moves the *target* scroll offset by `delta`, for smooth scrolling.
    ///
    /// The delta is accumulated onto the pending target, if any, so that
    /// quick wheel movements do not lose their (not yet scrolled) distance;
    /// the target is then animated via [`State::scroll_smoothly_to`].
    ///
    /// </details>
    fn scroll_smoothly(
        &mut self,
        delta: Vector<f32>,
        bounds: Rectangle,
        content: Size,
        now: Instant,
    ) {
        let current = Point::new(
            self.offset_x.absolute(bounds.width, content.width),
            self.offset_y.absolute(bounds.height, content.height),
        );

        // Accumulate onto the pending target, if any, so that quick wheel
        // movements do not lose their (not yet scrolled) distance
        // 累加到待处理的目标（如果有）上，这样快速的滚轮
        // 移动就不会丢失（尚未滚动的）距离
        let target = match self.target {
            Some(target) => Vector::new(
                Self::clamp_offset(target.destination.x + delta.x, bounds.width, content.width),
                Self::clamp_offset(
                    target.destination.y + delta.y,
                    bounds.height,
                    content.height,
                ),
            ),
            None => Vector::new(
                Self::clamp_offset(current.x + delta.x, bounds.width, content.width),
                Self::clamp_offset(current.y + delta.y, bounds.height, content.height),
            ),
        };

        self.scroll_smoothly_to(Target::absolute(target), bounds, content, now);
    }

    /// 平滑滚动到给定的 `target`。
    ///
    /// 这个目标会替换任何待处理的目标。如果已有动画段正在运行，
    /// 就会从当前位置和速度对它重定目标，这样
    /// 滚动会连贯地进行，而不是重新开始。
    ///
    /// 随后在每一帧通过 [`State::step`] 向目标缓动，
    /// 使用 ease-in-out 动画，其时长取决于距离：
    /// 短距离滚动的动画更长（更柔和），而长距离滚动的
    /// 动画更短（更干脆）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Scrolls smoothly to the given `target`.
    ///
    /// The target replaces any pending one. If a segment is already running,
    /// it is retargeted from the current position and velocity, so that
    /// scrolling flows instead of restarting.
    ///
    /// The target is then eased towards on each frame, via [`State::step`],
    /// with an ease-in-out animation whose duration depends on the distance:
    /// short scrolls get a longer (softer) animation, while long scrolls get a
    /// shorter (snappier) one.
    ///
    /// </details>
    fn scroll_smoothly_to(
        &mut self,
        target: Target,
        bounds: Rectangle,
        content: Size,
        now: Instant,
    ) {
        // The scroll is requested between frames; start the animation one
        // nominal frame early so that the first drawn frame already shows
        // progress
        // 滚动是在帧与帧之间请求的；让动画提前一个
        // 标称帧开始，这样第一个绘制帧就已经显示出
        // 进展
        let now = now - Duration::from_secs_f32(SMOOTH_SCROLL_FRAME_DELAY);

        let current = Point::new(
            self.offset_x.absolute(bounds.width, content.width),
            self.offset_y.absolute(bounds.height, content.height),
        );

        // Nothing to animate: the content fits, or we're already at the target
        // 没有需要动画的内容：内容放得下，或者我们已经在目标处
        if target.destination.x == current.x && target.destination.y == current.y {
            self.offset_x = target.x;
            self.offset_y = target.y;
            self.target = None;
            self.last_frame = None;
            return;
        }

        let Some(ongoing) = self.target else {
            // A new scroll run: start a fresh segment from rest at the
            // current position
            // 一轮新的滚动：从当前位置、由静止状态开始
            // 一个全新的动画段
            let distance = (target.destination.x - current.x)
                .abs()
                .max((target.destination.y - current.y).abs());

            self.target = Some(target);
            self.segment = Segment {
                start: current,
                started: now,
                duration: Self::smooth_scroll_duration(distance),
                slope: 0.0,
            };
            self.last_frame = Some(now);
            return;
        };

        // The target is unchanged: keep the running segment as is
        // 目标没有变化：保持正在运行的动画段不变
        if ongoing.destination == target.destination {
            return;
        }

        self.retarget(target, now);
    }

    /// 从当前位置和速度，把正在运行的动画段重定目标到 `target`，
    /// 这样滚动会连贯地进行，而不是重新开始。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Retargets the running segment towards `target`, from the current
    /// position and velocity, so that scrolling flows instead of restarting.
    ///
    /// </details>
    fn retarget(&mut self, target: Target, now: Instant) {
        // Retarget the running segment from the current position, preserving
        // the current velocity
        // 从当前位置对正在运行的动画段重定目标，并保持
        // 当前速度
        let start = self.animated_position(now);
        let velocity = self.animated_velocity(now);
        let new = Vector::new(
            target.destination.x - start.x,
            target.destination.y - start.y,
        );

        // The signed dimension with the largest magnitude, like Chromium's
        // 绝对值最大的带符号分量，与 Chromium 的做法类似
        let max_dimension = if new.x.abs() > new.y.abs() {
            new.x
        } else {
            new.y
        };

        // The new duration, bounded so that a small delta added at high
        // velocity does not "rubber band"; a bound with the wrong sign means
        // the new delta is against the current motion, and does not apply
        // 新的时长，它受到限制，使得在高速下加入的小增量
        // 不会产生“橡皮筋”效应；符号相反的限制值表示
        // 新的增量与当前运动方向相反，此时该限制不适用
        let mut duration = Self::smooth_scroll_duration(max_dimension.abs());
        if velocity.abs() > 0.01 {
            let bound = SMOOTH_SCROLL_RETARGET_VELOCITY_BOUND * max_dimension / velocity;

            if bound > 0.0 {
                duration = duration.min(bound);
            }
        }

        if max_dimension.abs() < 0.01 || duration < 0.01 {
            // The new target is right on top of us: end the animation now
            // 新目标就在我们当前的位置上：立即结束动画
            self.offset_x = target.x;
            self.offset_y = target.y;
            self.target = None;
            self.last_frame = None;
            return;
        }

        // Adjust the initial slope of the new segment so that it starts with
        // the current velocity
        // 调整新动画段的初始斜率，使它以
        // 当前速度开始
        let slope = (velocity * (duration / max_dimension))
            .clamp(-SMOOTH_SCROLL_SLOPE_CLAMP, SMOOTH_SCROLL_SLOPE_CLAMP);

        self.target = Some(target);
        self.segment = Segment {
            start,
            started: now,
            duration,
            slope,
        };
        self.last_frame = Some(now);
    }

    /// 让平滑滚动动画朝目标偏移量
    /// 向前步进。
    ///
    /// 如果动画仍在进行中，返回 `true`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Steps the smooth scrolling animation forward, towards the target
    /// offset.
    ///
    /// Returns `true` if the animation is still in progress.
    ///
    /// </details>
    fn step(&mut self, now: Instant, bounds: Rectangle, content: Size) -> bool {
        let Some(target) = self.target else {
            return false;
        };

        // The bounds and content may have changed while the animation is
        // running; re-resolve the target, and retarget if it moved
        // 动画运行期间，边界和内容可能已经改变；
        // 重新解析目标，如果它移动了就重定目标
        let resolved = Target::new(target.x, target.y, bounds, content);
        if resolved.destination != target.destination {
            self.retarget(resolved, now);
        }

        let Some(target) = self.target else {
            return false;
        };
        let started = self.segment.started;

        let t = (now - started).as_secs_f32();
        let progress = (t / self.segment.duration).clamp(0.0, 1.0);

        if progress >= 1.0 {
            // Settled exactly on the target, keeping relative (snapped)
            // offsets
            // 恰好停在目标上，保持相对（吸附）的
            // 偏移量
            self.offset_x = target.x;
            self.offset_y = target.y;
            self.target = None;
            self.last_frame = None;
            return false;
        }

        let bez = Self::smooth_scroll_progress(progress, self.segment.slope);
        self.offset_x = Offset::Absolute(
            self.segment.start.x + (target.destination.x - self.segment.start.x) * bez,
        );
        self.offset_y = Offset::Absolute(
            self.segment.start.y + (target.destination.y - self.segment.start.y) * bez,
        );

        self.last_frame = Some(now);
        true
    }

    /// 取消任何正在进行的平滑滚动动画，保持当前的
    /// 偏移量。
    ///
    /// 这确保直接操作（拖动滚动条、触摸滚动、
    /// 自动滚动、以编程方式滚动）始终优先于
    /// 待处理的滚轮动画。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Cancels any in-progress smooth scrolling animation, keeping the current
    /// offsets.
    ///
    /// This ensures that direct manipulation (scrollbar drags, touch scrolling,
    /// auto-scrolling, programmatic scrolling) always takes priority over a
    /// pending wheel animation.
    ///
    /// </details>
    fn cancel(&mut self) {
        self.target = None;
        self.last_frame = None;
    }

    /// 把绝对滚动偏移量钳制到给定内容在给定视口内
    /// 可以滚动的范围之内。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Clamps an absolute scroll offset to the range in which the given content
    /// can be scrolled inside the given viewport.
    ///
    /// </details>
    fn clamp_offset(offset: f32, viewport: f32, content: f32) -> f32 {
        if content > viewport {
            offset.clamp(0.0, content - viewport)
        } else {
            0.0
        }
    }

    /// `now` 时刻的动画位置；如果没有动画段正在运行，
    /// 则为 [`Point::ORIGIN`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The animated position at `now`, or [`Point::ORIGIN`] if a segment is
    /// not running.
    ///
    /// </details>
    fn animated_position(&self, now: Instant) -> Point {
        let Some(target) = self.target else {
            return Point::ORIGIN;
        };

        let started = self.segment.started;

        let t = (now - started).as_secs_f32();
        let progress = (t / self.segment.duration).clamp(0.0, 1.0);
        let bez = Self::smooth_scroll_progress(progress, self.segment.slope);

        Point::new(
            self.segment.start.x + (target.destination.x - self.segment.start.x) * bez,
            self.segment.start.y + (target.destination.y - self.segment.start.y) * bez,
        )
    }

    /// `now` 时刻的动画速度，以像素每秒为单位，沿
    /// 动画段的最大分量方向；如果没有动画段正在运行，则为 `0.0`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The animated velocity at `now`, in pixels per second along the
    /// segment's largest dimension, or `0.0` if a segment is not running.
    ///
    /// </details>
    fn animated_velocity(&self, now: Instant) -> f32 {
        let Some(target) = self.target else {
            return 0.0;
        };

        let started = self.segment.started;

        let t = (now - started).as_secs_f32();
        let progress = (t / self.segment.duration).clamp(0.0, 1.0);

        if progress >= 1.0 {
            return 0.0;
        }

        let dx = target.destination.x - self.segment.start.x;
        let dy = target.destination.y - self.segment.start.y;
        let max_dimension = if dx.abs() > dy.abs() { dx } else { dy };

        Self::smooth_scroll_curve_slope(progress, self.segment.slope) * max_dimension
            / self.segment.duration
    }

    /// 覆盖给定距离（以像素为单位）的平滑滚动动画段的
    /// 时长（以秒为单位）。
    ///
    /// 在一个渐变区间内，时长与距离成反比：
    /// 短距离滚动的动画更长（更柔和），而长距离滚动的
    /// 动画更短（更干脆）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The duration (in seconds) of a smooth scrolling segment covering the
    /// given distance (in pixels).
    ///
    /// The duration is inversely proportional to the distance within a ramp:
    /// short scrolls get a longer (softer) animation, while long scrolls get a
    /// shorter (snappier) one.
    ///
    /// </details>
    fn smooth_scroll_duration(distance: f32) -> f32 {
        let slope = (SMOOTH_SCROLL_DURATION_MIN - SMOOTH_SCROLL_DURATION_MAX)
            / (SMOOTH_SCROLL_DURATION_RAMP_END - SMOOTH_SCROLL_DURATION_RAMP_START);
        let offset = SMOOTH_SCROLL_DURATION_MAX - SMOOTH_SCROLL_DURATION_RAMP_START * slope;

        (offset + distance * slope).clamp(SMOOTH_SCROLL_DURATION_MIN, SMOOTH_SCROLL_DURATION_MAX)
            / SMOOTH_SCROLL_DURATION_DIVISOR
    }

    /// 在给定的时间进度（位于 `[0, 1]`）和给定的初始斜率下，
    /// 平滑滚动缓动曲线的进度。
    ///
    /// 该曲线是一条 ease-in-out 三次贝塞尔曲线，它的起始控制点
    /// 按 `slope` 缩放（斜率为 `0.0` 时就是普通的 ease-in-out 曲线）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The progress of the smooth scrolling easing curve at the given time
    /// progress (in `[0, 1]`), with the given initial slope.
    ///
    /// The curve is an ease-in-out cubic bezier whose initial control point is
    /// scaled by `slope` (a slope of `0.0` is the plain ease-in-out curve).
    ///
    /// </details>
    fn smooth_scroll_progress(time: f32, slope: f32) -> f32 {
        if time <= 0.0 {
            return 0.0;
        }

        if time >= 1.0 {
            return 1.0;
        }

        let s = Self::bezier_solve(time);
        let y1 = SMOOTH_SCROLL_BEZIER_X1 * slope;
        let os = 1.0 - s;

        // y(s), with y2 = 1
        // y(s)，其中 y2 = 1
        3.0 * y1 * s * os * os + 3.0 * s * s * os + s * s * s
    }

    /// 在给定的时间进度（位于 `[0, 1]`）和给定的初始斜率下，
    /// 平滑滚动缓动曲线的斜率（dy/dx）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The slope of the smooth scrolling easing curve (dy/dx) at the given
    /// time progress (in `[0, 1]`), with the given initial slope.
    ///
    /// </details>
    fn smooth_scroll_curve_slope(time: f32, slope: f32) -> f32 {
        let s = if time <= 0.0 {
            0.0
        } else if time >= 1.0 {
            1.0
        } else {
            Self::bezier_solve(time)
        };

        let x1 = SMOOTH_SCROLL_BEZIER_X1;
        let x2 = SMOOTH_SCROLL_BEZIER_X2;
        let y1 = x1 * slope;
        let os = 1.0 - s;

        // x'(s) and y'(s), with y2 = 1
        // x'(s) 和 y'(s)，其中 y2 = 1
        let x_prime =
            3.0 * x1 * os * (1.0 - 3.0 * s) + 3.0 * x2 * s * (2.0 - 3.0 * s) + 3.0 * s * s;
        let y_prime = 3.0 * y1 * os * (1.0 - 3.0 * s) + 3.0 * s * (2.0 - 3.0 * s) + 3.0 * s * s;

        y_prime / x_prime
    }

    /// 针对平滑滚动贝塞尔曲线的 x 轴，在 `[0, 1]` 内
    /// 求解满足 `x(s) = time` 的 `s`。
    ///
    /// 对于这里使用的控制点，x 轴是严格递增的，因此
    /// 二分法无条件收敛。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Solves `x(s) = time` for `s` in `[0, 1]`, for the smooth scrolling
    /// bezier's x-axis.
    ///
    /// The x-axis is strictly increasing for the control points used here, so
    /// bisection converges unconditionally.
    ///
    /// </details>
    fn bezier_solve(time: f32) -> f32 {
        let x1 = SMOOTH_SCROLL_BEZIER_X1;
        let x2 = SMOOTH_SCROLL_BEZIER_X2;

        let x = |s: f32| {
            let os = 1.0 - s;
            3.0 * x1 * s * os * os + 3.0 * x2 * s * s * os + s * s * s
        };

        let mut low = 0.0;
        let mut high = 1.0;

        for _ in 0..40 {
            let mid = (low + high) / 2.0;

            if x(mid) < time {
                low = mid;
            } else {
                high = mid;
            }
        }

        (low + high) / 2.0
    }

    /// 如果当前视口自上次通知以来发生了变化，就为它构建一个
    /// [`Scroll`] 通知，并记录用户进行了滚动。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Builds a [`Scroll`] notification for the current viewport, if it changed
    /// since the last one, and records that the user scrolled.
    ///
    /// </details>
    fn notify_scroll(&mut self, bounds: Rectangle, content: Size) -> Option<Scroll> {
        let notification = self.notify_viewport(bounds, content);

        if notification.is_some() {
            self.last_scrolled = Some(Instant::now());
        }

        notification
    }

    /// 如果当前视口自上次通知以来发生了变化，就为它构建一个
    /// [`Scroll`] 通知，并把它记录下来，以免再次报告。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Builds a [`Scroll`] notification for the current viewport, if it changed
    /// since the last one, recording it so that it is not reported again.
    ///
    /// </details>
    fn notify_viewport(&mut self, bounds: Rectangle, content: Size) -> Option<Scroll> {
        if content.width <= bounds.width && content.height <= bounds.height {
            return None;
        }

        let viewport = Viewport {
            x: self.offset_x,
            y: self.offset_y,
            bounds,
            content,
        };

        // Don't publish redundant viewports to shell
        // 不要向外壳发布重复的视口
        if let Some(last_notified) = self.last_notified {
            let last_absolute_offset = last_notified.absolute_offset();
            let current_absolute_offset = viewport.absolute_offset();

            let unchanged =
                |a: f32, b: f32| (a - b).abs() <= f32::EPSILON || (a.is_nan() && b.is_nan());

            if last_notified.bounds == bounds
                && last_notified.content == content
                && unchanged(last_absolute_offset.x, current_absolute_offset.x)
                && unchanged(last_absolute_offset.y, current_absolute_offset.y)
            {
                return None;
            }
        }

        // The notification's source: the scroll in progress or pending, or,
        // when the scroll position changed on its own, what changed in the
        // layout
        // 通知的来源：正在进行或待处理的滚动；或者，
        // 当滚动位置自行发生变化时，布局中
        // 发生变化的部分
        let source = if self.target.is_some() {
            // An in-flight animation: report the segment's source, keeping it
            // so that the settle notification reports it too
            // 正在进行的动画：报告该动画段的来源，并保留它，
            // 这样静止时的通知也会报告它
            self.source
        } else {
            // A settled or pending scroll: consume the source
            // 已静止或待处理的滚动：消耗掉这个来源
            self.source.take()
        }
        .unwrap_or_else(|| {
            if let Some(last_notified) = self.last_notified {
                if last_notified.content != content {
                    Source::Content
                } else {
                    Source::Resize
                }
            } else {
                // The first notification: the content was laid out
                // 第一次通知：内容完成了布局
                Source::Content
            }
        });

        let origin = self.last_notified;
        self.last_notified = Some(viewport);

        Some(Scroll {
            viewport,
            origin,
            source,
            target: self.target,
        })
    }

    /// 让滚动沿给定的轴跳转到给定的 `percentage`，
    /// 并把它吸附到最近的逻辑位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Jumps the scroll to the given `percentage` along the given axis,
    /// snapping it to the nearest logical position.
    ///
    /// </details>
    fn scroll_to_percentage(
        &mut self,
        axis: Axis,
        percentage: f32,
        bounds: Rectangle,
        content: Size,
    ) {
        self.cancel();

        match axis {
            Axis::X => self.offset_x = Offset::Relative(percentage.clamp(0.0, 1.0)),
            Axis::Y => self.offset_y = Offset::Relative(percentage.clamp(0.0, 1.0)),
        }

        self.unsnap(axis, bounds, content);
    }

    /// 在 `cursor_position` 处按下给定轴的轨道，开始
    /// 按住轨道的自动滚动。
    ///
    /// 这与 Chromium 的轨道按下行为
    /// （`cc::ScrollbarController::HandlePointerDown`）一致：立即应用一个翻页步长，
    /// 像滚轮滚动一样带有动画（禁用平滑滚动时
    /// 则立即生效）；并且在按键保持按下期间，经过一段延迟后
    /// 以恒定速度自动滚动，直到滚动块到达
    /// 指针（在每一帧通过 [`Self::step_rail`] 步进）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Presses the rail of the given axis at `cursor_position`, starting
    /// the held-rail autoscroll.
    ///
    /// This mirrors Chromium's track press (`cc::ScrollbarController::
    /// HandlePointerDown`): a page step is applied immediately, animated
    /// like a wheel scroll (or immediately when smooth scrolling is
    /// disabled), and, while the button is held, a constant-velocity
    /// autoscroll runs after a delay until the scroller reaches the
    /// pointer (stepped on each frame, via [`Self::step_rail`]).
    ///
    /// </details>
    fn press_rail(
        &mut self,
        axis: Axis,
        scrollbar: &internals::Scrollbar,
        cursor_position: Point,
        smooth: bool,
        now: Instant,
        bounds: Rectangle,
        content: Size,
    ) {
        let Some(scroller) = scrollbar.scroller else {
            return;
        };

        // Which side of the scroller was pressed, in cursor space: valid
        // for both anchors, as the scroller's bounds already account for
        // the `Anchor::End` mirroring
        // 按下的位置在滚动块的哪一侧，以光标空间计：
        // 对两种锚点都有效，因为滚动块的边界已经考虑了
        // `Anchor::End` 的镜像
        let cursor_direction =
            if axis.coordinate(cursor_position) < axis.coordinate(scroller.bounds.position()) {
                -1.0
            } else {
                1.0
            };

        // The offset at which the scroller's leading edge (toward the
        // pointer) reaches the pointer: a `grabbed_at` of `0.0` is the
        // scroller's start edge, `1.0` its end edge
        // 滚动块的前缘（朝向指针的一侧）到达指针时的
        // 偏移量：`grabbed_at` 为 `0.0` 表示
        // 滚动块的起始边缘，`1.0` 表示它的结束边缘
        let grabbed_at = if cursor_direction < 0.0 { 0.0 } else { 1.0 };
        let stop = Offset::Relative(scrollbar.scroll_percentage(axis, grabbed_at, cursor_position))
            .absolute(axis.length(bounds), axis.length(content));

        // The direction, in offset units: the offset-space sign of the stop
        // line, which accounts for the anchor's mirroring
        // 方向，以偏移量单位计：停止线在偏移量空间中的
        // 符号，它考虑了锚点的镜像
        let direction = (stop - self.axis_offset(axis, bounds, content)).signum();
        if direction == 0.0 {
            // The pointer is on the stop line: there is nothing to scroll
            // 指针就在停止线上：没有可滚动的内容
            return;
        }

        let page_step = Self::rail_page_step(axis.length(bounds));

        // The initial page step: animated like a wheel scroll, or applied
        // immediately when smooth scrolling is disabled
        // 最初的翻页步长：像滚轮滚动一样带有动画，或者
        // 在禁用平滑滚动时立即应用
        if smooth {
            self.scroll_smoothly(axis.vector(direction * page_step), bounds, content, now);
        } else {
            self.scroll(axis.vector(direction * page_step), bounds, content);
        }

        self.interaction = Interaction::RailHeld(RailHeld {
            axis,
            direction,
            cursor_direction,
            velocity: page_step * RAIL_AUTOSCROLL_MULTIPLIER,
            pressed_at: now,
            pointer: cursor_position,
            last_frame: None,
        });
    }

    /// 在某一帧上步进给定轴的按住轨道自动滚动。
    ///
    /// 如果滚动条在轨道被按住期间消失，返回 `None`，
    /// 此时必须放弃这次交互（就像 Chromium 在
    /// 滚动条被注销时所做的那样）；否则返回这次步进的
    /// 结果。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Steps the held-rail autoscroll of the given axis on a frame.
    ///
    /// Returns `None` if the scrollbar disappeared while the rail was held,
    /// in which case the interaction must be dropped (like Chromium does
    /// when its scrollbar is unregistered); otherwise, the outcome of the
    /// step.
    ///
    /// </details>
    fn step_rail(
        &mut self,
        rail: &mut RailHeld,
        now: Instant,
        scrollbar: Option<&internals::Scrollbar>,
        bounds: Rectangle,
        content: Size,
    ) -> Option<RailStep> {
        let axis = rail.axis;

        let scrollbar = scrollbar?;

        // The autoscroll starts after the press delay, while the initial
        // page step is still easing
        // 自动滚动在按下延迟之后开始，此时最初的
        // 翻页步长仍在缓动之中
        if now - rail.pressed_at < RAIL_AUTOSCROLL_DELAY {
            return Some(RailStep::Waiting);
        }

        // `last_frame` guards against stepping twice for the same instant
        // `last_frame` 用于防止在同一时刻步进两次
        if rail.last_frame == Some(now) {
            return Some(RailStep::Waiting);
        }

        // The time since the last step; the first step starts from the
        // nominal start of the autoscroll, not from the press
        // 自上次步进以来经过的时间；第一次步进从
        // 自动滚动的标称起点开始计算，而不是从按下时开始
        let last_frame = rail.last_frame;
        rail.last_frame = Some(now);
        let time_delta = last_frame.map_or(
            (now - rail.pressed_at) - RAIL_AUTOSCROLL_DELAY,
            |last_frame| now - last_frame,
        );
        if time_delta.is_zero() {
            return Some(RailStep::Waiting);
        }

        let scroller = scrollbar.scroller?;

        let current = self.axis_offset(axis, bounds, content);

        // The offset at which the scroller's leading edge (the edge on the
        // pressed side, in cursor space) reaches the pointer
        // 滚动块的前缘（按下一侧的边缘，以光标空间计）
        // 到达指针时的偏移量
        let grabbed_at = if rail.cursor_direction > 0.0 {
            1.0
        } else {
            0.0
        };
        let stop = Offset::Relative(scrollbar.scroll_percentage(axis, grabbed_at, rail.pointer))
            .absolute(axis.length(bounds), axis.length(content));

        // The autoscroll pauses while the pointer is on the rail but no
        // longer on the pressed side of the (moving) scroller, and
        // continues while the pointer is off the rail, mirroring Chromium's
        // behavior when the pointer leaves the scrollbar layer
        // 当指针在轨道上、但已不在（移动中的）滚动块的
        // 按下一侧时，自动滚动会暂停；而当指针
        // 离开轨道时，自动滚动会继续，这与指针离开
        // 滚动条图层时 Chromium 的行为一致
        let on_pressed_side = if rail.cursor_direction > 0.0 {
            axis.coordinate(rail.pointer)
                > axis.coordinate(scroller.bounds.position()) + axis.length(scroller.bounds)
        } else {
            axis.coordinate(rail.pointer) < axis.coordinate(scroller.bounds.position())
        };
        if scrollbar.total_bounds.contains(rail.pointer) && !on_pressed_side {
            return Some(RailStep::Idle);
        }

        // ... and it stops while the scroller's leading edge has reached
        // the pointer
        // ……并且当滚动块的前缘已经到达指针时，
        // 它会停止
        let remaining = if rail.direction > 0.0 {
            stop - current
        } else {
            current - stop
        };
        if remaining <= 0.0 {
            return Some(RailStep::Idle);
        }

        // A constant-velocity step, clamped so that the scroller does not
        // cross the pointer within the frame
        // 一次恒定速度的步进，它受到钳制，使滚动块不会
        // 在这一帧内越过指针
        let delta = rail.direction * (rail.velocity * time_delta.as_secs_f32()).min(remaining);

        self.scroll(axis.vector(delta), bounds, content);

        Some(if self.axis_offset(axis, bounds, content) != current {
            RailStep::Moved
        } else {
            // The step was clamped away (e.g. at the end of the content):
            // nothing can move until the pointer moves again
            // 这次步进被钳制为零（例如在内容的末尾）：
            // 在指针再次移动之前，没有任何东西可以移动
            RailStep::Idle
        })
    }

    /// 使用给定的 [`Animation`] 把滚动吸附到给定的
    /// [`RelativeOffset`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Snaps the scroll to the given [`RelativeOffset`], with the given
    /// [`Animation`].
    ///
    /// </details>
    fn snap_to(
        &mut self,
        offset: RelativeOffset<Option<f32>>,
        animation: Animation,
        bounds: Rectangle,
        content: Size,
        source: Source,
    ) {
        self.source = Some(source);

        if !self.should_scroll_smoothly(animation) {
            self.cancel();

            if let Some(x) = offset.x {
                self.offset_x = Offset::Relative(x.clamp(0.0, 1.0));
            }

            if let Some(y) = offset.y {
                self.offset_y = Offset::Relative(y.clamp(0.0, 1.0));
            }

            return;
        }

        // Snap the targeted axes relative to the content, and keep the
        // current offsets of the axes the snap does not target
        // 把目标轴相对于内容进行吸附，并保持
        // 这次吸附未涉及的轴的当前偏移量
        let x = offset
            .x
            .map(|x| Offset::Relative(x.clamp(0.0, 1.0)))
            .unwrap_or(self.offset_x);
        let y = offset
            .y
            .map(|y| Offset::Relative(y.clamp(0.0, 1.0)))
            .unwrap_or(self.offset_y);

        self.scroll_smoothly_to(
            Target::new(x, y, bounds, content),
            bounds,
            content,
            Instant::now(),
        );
    }

    /// 使用给定的 [`Animation`] 滚动到给定的 [`AbsoluteOffset`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Scrolls to the given [`AbsoluteOffset`], with the given [`Animation`].
    ///
    /// </details>
    fn scroll_to(
        &mut self,
        offset: AbsoluteOffset<Option<f32>>,
        animation: Animation,
        bounds: Rectangle,
        content: Size,
        source: Source,
    ) {
        self.source = Some(source);

        if !self.should_scroll_smoothly(animation) {
            self.cancel();

            if let Some(x) = offset.x {
                self.offset_x = Offset::Absolute(x.max(0.0));
            }

            if let Some(y) = offset.y {
                self.offset_y = Offset::Absolute(y.max(0.0));
            }

            return;
        }

        // Scroll the targeted axes to their clamped absolute offsets, and
        // keep the current offsets of the axes the scroll does not target
        // 把目标轴滚动到钳制后的绝对偏移量，并
        // 保持这次滚动未涉及的轴的当前偏移量
        let x = offset
            .x
            .map(|x| Offset::Absolute(Self::clamp_offset(x, bounds.width, content.width)))
            .unwrap_or(self.offset_x);
        let y = offset
            .y
            .map(|y| Offset::Absolute(Self::clamp_offset(y, bounds.height, content.height)))
            .unwrap_or(self.offset_y);

        self.scroll_smoothly_to(
            Target::new(x, y, bounds, content),
            bounds,
            content,
            Instant::now(),
        );
    }

    /// 使用给定的 [`Animation`] 按提供的 [`AbsoluteOffset`]
    /// 进行滚动。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Scrolls by the provided [`AbsoluteOffset`], with the given
    /// [`Animation`].
    ///
    /// </details>
    fn scroll_by(
        &mut self,
        offset: AbsoluteOffset,
        animation: Animation,
        bounds: Rectangle,
        content: Size,
    ) {
        self.source = Some(Source::Operation);

        let delta = Vector::new(offset.x, offset.y);

        if self.should_scroll_smoothly(animation) {
            self.scroll_smoothly(delta, bounds, content, Instant::now());
        } else {
            self.scroll(delta, bounds, content);
        }
    }

    /// 结合部件的平滑滚动设置，判断给定的 [`Animation`]
    /// 是否请求平滑滚动。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the given [`Animation`] requests a smooth scroll, given the
    /// widget's smooth scrolling setting.
    ///
    /// </details>
    fn should_scroll_smoothly(&self, animation: Animation) -> bool {
        match animation {
            Animation::Auto => self.smooth_scroll,
            Animation::Instant => false,
            Animation::Smooth => true,
        }
    }

    /// 处理用户与 [`Scrollable`] 的交互，直到（但
    /// 不包括）把事件委托给内容。
    ///
    /// 返回需要在 [`Shell`] 上落实的 [`Interact`] 效果，以及
    /// 事件是否必须委托给内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Handles the interaction of the user with the [`Scrollable`], up to (but
    /// not including) the delegation of the event to the content.
    ///
    /// Returns the [`Interact`] effects to materialize on the [`Shell`] and
    /// whether the event must be delegated to the content.
    ///
    /// </details>
    fn interact(
        &mut self,
        event: &Event,
        bounds: Rectangle,
        content: Size,
        cursor: mouse::Cursor,
        cursor_over_scrollable: Option<Point>,
        scrollbars: &Scrollbars,
        mouse_over_scrollbar: Option<Axis>,
        direction: Direction,
        smooth_scroll: bool,
        click_to_scroll: bool,
    ) -> Interact {
        let mut interact = Interact::default();

        // Clear the scroll transaction, if the user is no longer scrolling
        // 如果用户不再滚动，就清除滚动事务
        if let Some(last_scrolled) = self.last_scrolled {
            let clear_transaction = match event {
                Event::Mouse(
                    mouse::Event::ButtonPressed(_)
                    | mouse::Event::ButtonReleased(_)
                    | mouse::Event::CursorLeft,
                ) => true,
                Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                    last_scrolled.elapsed() > Duration::from_millis(100)
                }
                _ => last_scrolled.elapsed() > Duration::from_millis(1500),
            };

            if clear_transaction {
                self.last_scrolled = None;
            }
        }

        let mut translation = self.last_translation;

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            // Step the smooth scrolling animation, if any; `last_frame`
            // guards against stepping twice for the same instant
            // 步进平滑滚动动画（如果有）；`last_frame`
            // 用于防止在同一时刻步进两次
            if self.last_frame != Some(*now)
                && self.step(*now, bounds, content)
                && let Some(notification) = self.notify_scroll(bounds, content)
            {
                interact.scroll = Some(notification);
            }

            translation = self.translation(direction, bounds.size(), content);
            self.last_translation = translation;

            // Step the held-rail autoscroll, if any; the geometry is rebuilt
            // from the current translation, so that the pause and stop
            // conditions are evaluated against the scroller as of this frame
            // 步进按住轨道的自动滚动（如果有）；几何信息会根据
            // 当前的平移量重新构建，这样暂停和停止
            // 条件就会针对这一帧的滚动块来评估
            if let Interaction::RailHeld(mut rail) = self.interaction {
                let scrollbars = Scrollbars::new(translation, direction, bounds, content);

                let scrollbar = match rail.axis {
                    Axis::X => scrollbars.x.as_ref(),
                    Axis::Y => scrollbars.y.as_ref(),
                };

                let step = self
                    .step_rail(&mut rail, *now, scrollbar, bounds, content)
                    .map(|step| (Interaction::RailHeld(rail), step));

                match step {
                    Some((interaction, step)) => {
                        self.interaction = interaction;

                        match step {
                            RailStep::Moved => {
                                self.source = Some(Source::Scrollbar);

                                if let Some(notification) = self.notify_scroll(bounds, content) {
                                    interact.scroll = Some(notification);
                                }

                                interact.request_redraw = true;
                            }
                            RailStep::Waiting => interact.request_redraw = true,
                            RailStep::Idle => {}
                        }
                    }
                    // The scrollbar disappeared while the rail was held: drop
                    // the interaction, like Chromium does when its scrollbar
                    // is unregistered
                    // 滚动条在轨道被按住期间消失：放弃
                    // 这次交互，就像 Chromium 在滚动条
                    // 被注销时所做的那样
                    None => self.interaction = Interaction::None,
                }
            }

            if self.target.is_some() {
                interact.request_redraw = true;
            } else if let Interaction::AutoScrolling {
                origin,
                current,
                last_frame,
            } = self.interaction
            {
                if last_frame == Some(*now) {
                    interact.request_redraw = true;
                } else {
                    self.interaction = Interaction::AutoScrolling {
                        origin,
                        current,
                        last_frame: None,
                    };

                    let mut delta = current - origin;

                    if delta.x.abs() < AUTOSCROLL_DEADZONE {
                        delta.x = 0.0;
                    }

                    if delta.y.abs() < AUTOSCROLL_DEADZONE {
                        delta.y = 0.0;
                    }

                    if delta.x != 0.0 || delta.y != 0.0 {
                        let time_delta =
                            last_frame.map_or(Duration::ZERO, |last_frame| *now - last_frame);

                        let scroll_factor = time_delta.as_secs_f32();

                        self.scroll(
                            direction.align(Vector::new(
                                delta.x.signum()
                                    * delta.x.abs().powf(AUTOSCROLL_SMOOTHNESS)
                                    * scroll_factor,
                                delta.y.signum()
                                    * delta.y.abs().powf(AUTOSCROLL_SMOOTHNESS)
                                    * scroll_factor,
                            )),
                            bounds,
                            content,
                        );

                        self.source = Some(Source::AutoScroll);

                        let notification = self.notify_scroll(bounds, content);
                        let has_scrolled = notification.is_some();

                        if let Some(notification) = notification {
                            interact.scroll = Some(notification);
                        }

                        if has_scrolled || time_delta.is_zero() {
                            self.interaction = Interaction::AutoScrolling {
                                origin,
                                current,
                                last_frame: Some(*now),
                            };

                            interact.request_redraw = true;
                        }
                    }
                }
            }

            if interact.scroll.is_none() {
                interact.scroll = self.notify_viewport(bounds, content);
            }

            // A source that did not produce a notification must not be
            // attributed to a later one
            // 没有产生通知的来源，不能被
            // 归到之后的通知上
            if self.target.is_none() {
                self.source = None;
            }
        }

        // A held rail: track the pointer for the autoscroll's pause and stop
        // conditions; a moved pointer re-evaluates them, waking the animation
        // 被按住的轨道：跟踪指针，用于自动滚动的暂停和停止
        // 条件；指针移动后会重新评估这些条件，并唤醒动画
        if let Interaction::RailHeld(mut rail) = self.interaction
            && let Some(position) = rail_moved(event, cursor, rail.pointer)
        {
            rail.pointer = position;
            self.interaction = Interaction::RailHeld(rail);
            interact.request_redraw = true;
        }

        // A scroller being dragged follows the pointer 1:1, until the button
        // is released
        // 被拖动的滚动块会 1:1 跟随指针，直到按键
        // 被松开
        if let Some((axis, scroller_grabbed_at)) = self.interaction.scroller_grabbed() {
            match event {
                Event::Mouse(mouse::Event::CursorMoved { .. })
                | Event::Touch(touch::Event::FingerMoved { .. }) => {
                    if let Some(scrollbar) = scrollbars.scrollbar(axis) {
                        let Some(cursor_position) = cursor.observe().position() else {
                            interact.stop = true;
                            return interact;
                        };

                        self.scroll_to_percentage(
                            axis,
                            scrollbar.scroll_percentage(axis, scroller_grabbed_at, cursor_position),
                            bounds,
                            content,
                        );

                        self.source = Some(Source::Scrollbar);

                        if let Some(notification) = self.notify_scroll(bounds, content) {
                            interact.scroll = Some(notification);
                        }

                        interact.capture = true;
                    }
                }
                _ => {}
            }
        } else if let Some(axis) = mouse_over_scrollbar {
            // Otherwise, a press on a scrollbar under the cursor grabs its
            // scroller, jump-drags it (with `Shift`, or with a plain press
            // when `click_to_scroll` is enabled), or presses the rail
            // 否则，在光标下方的滚动条上按下时，会抓取它的
            // 滚动块、跳转并拖动它（按住 `Shift`，或者在启用
            // `click_to_scroll` 时普通按下），或者按下轨道
            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    let Some(cursor_position) = cursor.position() else {
                        interact.stop = true;
                        return interact;
                    };

                    if let Some((scrollbar, hit)) = scrollbars.hit(axis, cursor_position) {
                        match hit {
                            // The scroller: jump to the click position and
                            // drag it from there
                            // 滚动块：跳转到点击位置，并
                            // 从那里开始拖动它
                            Hit::Scroller { grabbed_at } => {
                                self.scroll_to_percentage(
                                    axis,
                                    scrollbar.scroll_percentage(axis, grabbed_at, cursor_position),
                                    bounds,
                                    content,
                                );

                                self.interaction = Interaction::ScrollerGrabbed(axis, grabbed_at);
                            }
                            // A "jump click" on the rail — a `Shift`-click by
                            // default, or a plain click when `click_to_scroll`
                            // is enabled: jump the scroller to the click
                            // position and drag it from there, like
                            // Chromium's `Shift`+click on the track
                            // 在轨道上的“跳转点击”（默认是按住 `Shift`
                            // 点击，启用 `click_to_scroll` 时则是普通点击）：
                            // 让滚动块跳转到点击
                            // 位置，并从那里开始拖动它，类似于
                            // Chromium 在轨道上的 `Shift`+点击
                            Hit::Rail if self.keyboard_modifiers.shift() != click_to_scroll => {
                                self.scroll_to_percentage(
                                    axis,
                                    scrollbar.scroll_percentage(axis, 0.5, cursor_position),
                                    bounds,
                                    content,
                                );

                                self.interaction = Interaction::ScrollerGrabbed(axis, 0.5);
                            }
                            // A paging rail press — a plain click by default,
                            // or a `Shift`-click when `click_to_scroll` is
                            // enabled: a page step (animated like a wheel
                            // scroll) and, while the button is held, a
                            // constant-velocity autoscroll until the scroller
                            // reaches the pointer, like Chromium's track
                            // autoscroll
                            // 翻页式的轨道按下（默认是普通点击，
                            // 启用 `click_to_scroll` 时则是按住 `Shift`
                            // 点击）：一个翻页步长（像滚轮滚动一样
                            // 带有动画），并且在按键保持按下期间，
                            // 以恒定速度自动滚动，直到滚动块
                            // 到达指针，类似于 Chromium 的轨道
                            // 自动滚动
                            Hit::Rail => {
                                self.press_rail(
                                    axis,
                                    scrollbar,
                                    cursor_position,
                                    smooth_scroll,
                                    Instant::now(),
                                    bounds,
                                    content,
                                );

                                interact.request_redraw = true;
                            }
                        }

                        self.source = Some(Source::Scrollbar);

                        if let Some(notification) = self.notify_scroll(bounds, content) {
                            interact.scroll = Some(notification);
                        }

                        interact.capture = true;
                    }
                }
                _ => {}
            }
        }

        if matches!(self.interaction, Interaction::AutoScrolling { .. })
            && matches!(
                event,
                Event::Mouse(mouse::Event::ButtonPressed(_) | mouse::Event::WheelScrolled { .. })
                    | Event::Touch(_)
                    | Event::Keyboard(_)
            )
        {
            self.interaction = Interaction::None;
            interact.capture = true;
            interact.invalidate_layout = true;
            interact.request_redraw = true;
            interact.stop = true;
            return interact;
        }

        if self.last_scrolled.is_none()
            || !matches!(event, Event::Mouse(mouse::Event::WheelScrolled { .. }))
        {
            let cursor = match cursor_over_scrollable {
                Some(cursor_position)
                    if mouse_over_scrollbar.is_none() && !self.interaction.scrollers_grabbed() =>
                {
                    mouse::Cursor::Available(cursor_position + translation)
                }
                _ => cursor.obstruct() + translation,
            };

            interact.content = Some(Content { cursor });
        }

        interact
    }

    /// 处理交互中在事件委托给内容之后
    /// 发生的部分。
    ///
    /// 返回需要在 [`Shell`] 上落实的 [`Update`] 效果。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Handles the part of the interaction that happens after the event is
    /// delegated to the content.
    ///
    /// Returns the [`Update`] effects to materialize on the [`Shell`].
    ///
    /// </details>
    fn update(
        &mut self,
        event: &Event,
        bounds: Rectangle,
        content: Size,
        cursor: mouse::Cursor,
        cursor_over_scrollable: Option<Point>,
        mouse_over_scrollbar: Option<Axis>,
        direction: Direction,
        smooth_scroll: bool,
        auto_scroll: bool,
        is_event_captured: bool,
    ) -> Update {
        let mut update = Update::default();

        if matches!(
            event,
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. })
        ) {
            self.interaction = Interaction::None;
            return update;
        }

        if is_event_captured {
            return update;
        }

        match event {
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if !cursor.land().is_over(bounds) {
                    return update;
                }

                let (delta, is_significant) = match *delta {
                    mouse::ScrollDelta::Lines { x, y } => {
                        let is_shift_pressed = self.keyboard_modifiers.shift();

                        // macOS automatically inverts the axes when Shift is
                        // pressed
                        // 按下 Shift 时，macOS 会自动
                        // 交换两个轴
                        let (x, y) = if cfg!(target_os = "macos") && is_shift_pressed {
                            (y, x)
                        } else {
                            (x, y)
                        };

                        let movement = if !is_shift_pressed {
                            Vector::new(x, y)
                        } else {
                            Vector::new(y, x)
                        };

                        (
                            -movement * WHEEL_PX_PER_LINE,
                            x.abs() >= 0.5 || y.abs() >= 0.5,
                        )
                    }
                    // Pixel deltas (e.g. from high-precision touchpads) are
                    // already smooth, so scrolling them immediately avoids
                    // double-smoothing them
                    // 像素增量（例如来自高精度触摸板）本身
                    // 已经是平滑的，因此立即滚动可以避免
                    // 对它们进行二次平滑
                    mouse::ScrollDelta::Pixels { x, y } => (-Vector::new(x, y), false),
                };

                let delta = direction.align(delta);

                if smooth_scroll && is_significant {
                    self.scroll_smoothly(delta, bounds, content, Instant::now());
                } else {
                    self.scroll(delta, bounds, content);
                }

                self.source = Some(Source::Wheel);

                let notification = self.notify_scroll(bounds, content);
                let has_scrolled = notification.is_some();

                if let Some(notification) = notification {
                    update.scroll = Some(notification);
                }

                let in_transaction = self.last_scrolled.is_some() || self.target.is_some();

                if has_scrolled || in_transaction {
                    update.capture = true;
                }

                if self.target.is_some() {
                    update.request_redraw = true;
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Middle))
                if auto_scroll && matches!(self.interaction, Interaction::None) =>
            {
                let Some(origin) = cursor_over_scrollable else {
                    return update;
                };

                self.interaction = Interaction::AutoScrolling {
                    origin,
                    current: origin,
                    last_frame: None,
                };

                update.capture = true;
                update.invalidate_layout = true;
                update.request_redraw = true;
            }
            Event::Touch(event)
                if matches!(self.interaction, Interaction::TouchScrolling(_))
                    || mouse_over_scrollbar.is_none() =>
            {
                match event {
                    touch::Event::FingerPressed { .. } => {
                        let Some(position) = cursor_over_scrollable else {
                            return update;
                        };

                        self.interaction = Interaction::TouchScrolling(position);
                    }
                    touch::Event::FingerMoved { .. } => {
                        let Interaction::TouchScrolling(scroll_box_touched_at) = self.interaction
                        else {
                            return update;
                        };

                        let Some(cursor_position) = cursor.position() else {
                            return update;
                        };

                        let delta = Vector::new(
                            scroll_box_touched_at.x - cursor_position.x,
                            scroll_box_touched_at.y - cursor_position.y,
                        );

                        self.scroll(direction.align(delta), bounds, content);

                        self.interaction = Interaction::TouchScrolling(cursor_position);
                        self.source = Some(Source::Touch);

                        if let Some(notification) = self.notify_scroll(bounds, content) {
                            update.scroll = Some(notification);
                        }
                    }
                    _ => {}
                }

                update.capture = true;
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if let Interaction::AutoScrolling {
                    origin, last_frame, ..
                } = self.interaction
                {
                    let delta = *position - origin;

                    self.interaction = Interaction::AutoScrolling {
                        origin,
                        current: *position,
                        last_frame,
                    };

                    if (delta.x.abs() >= AUTOSCROLL_DEADZONE
                        || delta.y.abs() >= AUTOSCROLL_DEADZONE)
                        && last_frame.is_none()
                    {
                        update.request_redraw = true;
                    }
                }
            }
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                self.keyboard_modifiers = *modifiers;
            }
            _ => {}
        }

        update
    }

    /// 把给定轴的偏移量落实为绝对偏移量，这样它
    /// 就不再吸附到某个逻辑位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Materializes the given axis's offset into an absolute one, so that it
    /// is no longer snapped to a logical position.
    ///
    /// </details>
    fn unsnap(&mut self, axis: Axis, bounds: Rectangle, content: Size) {
        match axis {
            Axis::X => {
                self.offset_x =
                    Offset::Absolute(self.offset_x.absolute(bounds.width, content.width));
            }
            Axis::Y => {
                self.offset_y =
                    Offset::Absolute(self.offset_y.absolute(bounds.height, content.height));
            }
        }
    }

    /// 给定轴的绝对偏移量，以像素为单位。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The absolute offset of the given axis, in pixels.
    ///
    /// </details>
    fn axis_offset(&self, axis: Axis, bounds: Rectangle, content: Size) -> f32 {
        match axis {
            Axis::X => self.offset_x.absolute(bounds.width, content.width),
            Axis::Y => self.offset_y.absolute(bounds.height, content.height),
        }
    }

    /// 给定 [`Direction`]、[`Scrollable`] 的边界及其内容的边界，
    /// 返回 [`State`] 的滚动平移量。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the scrolling translation of the [`State`], given a [`Direction`],
    /// the bounds of the [`Scrollable`] and its contents.
    ///
    /// </details>
    fn translation(&self, direction: Direction, bounds: Size, content: Size) -> Vector {
        Vector::new(
            if let Some(horizontal) = direction.horizontal() {
                self.offset_x
                    .translation(bounds.width, content.width, horizontal.alignment)
            } else {
                0.0
            },
            if let Some(vertical) = direction.vertical() {
                self.offset_y
                    .translation(bounds.height, content.height, vertical.alignment)
            } else {
                0.0
            },
        )
    }
}

/// [`Scrollbar`] 被某个光标位置命中的部分。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The part of a [`Scrollbar`] hit by a cursor position.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
enum Hit {
    /// [`Scrollbar`] 的滚动块（滑块），在其长度的给定
    /// 比例处被抓取。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The scroller (thumb) of the [`Scrollbar`], grabbed at the given
    /// fraction of its length.
    ///
    /// </details>
    Scroller {
        /// 滚动块被抓取的位置，表示为从其起点算起、
        /// 占其长度的比例。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The fraction of the scroller's length, from its start, at which
        /// it was grabbed.
        ///
        /// </details>
        grabbed_at: f32,
    },

    /// [`Scrollbar`] 的轨道，即滚动块之外的部分。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The rail (track) of the [`Scrollbar`], outside of the scroller.
    ///
    /// </details>
    Rail,
}

#[derive(Debug)]
/// 两个 [`Scrollbar`] 的状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// State of both [`Scrollbar`]s.
///
/// </details>
struct Scrollbars {
    y: Option<internals::Scrollbar>,
    x: Option<internals::Scrollbar>,
}

impl Scrollbars {
    /// 如果内容溢出 [`Scrollable`] 的边界，就创建 y 和/或 x 滚动条。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Create y and/or x scrollbar(s) if content is overflowing the [`Scrollable`] bounds.
    ///
    /// </details>
    fn new(translation: Vector, direction: Direction, bounds: Rectangle, content: Size) -> Self {
        let show_scrollbar_x = direction
            .horizontal()
            .filter(|_scrollbar| content.width > bounds.width);

        let show_scrollbar_y = direction
            .vertical()
            .filter(|_scrollbar| content.height > bounds.height);

        let y_scrollbar = if let Some(vertical) = show_scrollbar_y {
            let Scrollbar {
                width,
                margin,
                scroller_width,
                spacing,
                padding,
                ..
            } = *vertical;

            // Adjust the height of the vertical scrollbar if the horizontal scrollbar
            // is present
            // 如果存在水平滚动条，就调整垂直滚动条的
            // 高度
            let x_scrollbar_height =
                show_scrollbar_x.map_or(0.0, |h| h.width.max(h.scroller_width) + h.margin);

            let total_scrollbar_width = width.max(scroller_width) + 2.0 * margin;

            // The padding is purely visual: it shrinks the top and bottom of the
            // scrollbar without affecting the layout
            // 内边距纯粹是视觉上的：它缩短滚动条的顶部和底部，
            // 而不影响布局
            let scrollbar_height = (bounds.height - x_scrollbar_height - 2.0 * padding).max(0.0);

            // Total bounds of the scrollbar + margin + scroller width
            // 滚动条 + 外边距 + 滚动块宽度的总边界
            let total_scrollbar_bounds = Rectangle {
                x: bounds.x + bounds.width - total_scrollbar_width,
                y: bounds.y + padding,
                width: total_scrollbar_width,
                height: scrollbar_height,
            };

            // Bounds of just the scrollbar
            // 仅滚动条本身的边界
            let scrollbar_bounds = Rectangle {
                x: bounds.x + bounds.width - total_scrollbar_width / 2.0 - width / 2.0,
                y: bounds.y + padding,
                width,
                height: scrollbar_height,
            };

            let ratio = bounds.height / content.height;

            let scroller = if ratio >= 1.0 {
                None
            } else {
                // min height for easier grabbing with super tall content
                // 最小高度，便于在内容特别高时抓取
                let scroller_height = (scrollbar_bounds.height * ratio).max(2.0);
                let scroller_offset =
                    translation.y * ratio * scrollbar_bounds.height / bounds.height;

                let scroller_bounds = Rectangle {
                    x: bounds.x + bounds.width - total_scrollbar_width / 2.0 - scroller_width / 2.0,
                    y: (scrollbar_bounds.y + scroller_offset).max(0.0),
                    width: scroller_width,
                    height: scroller_height,
                };

                Some(internals::Scroller {
                    bounds: scroller_bounds,
                })
            };

            Some(internals::Scrollbar {
                total_bounds: total_scrollbar_bounds,
                bounds: scrollbar_bounds,
                scroller,
                alignment: vertical.alignment,
                disabled: content.height <= bounds.height,
                floating: spacing.is_none(),
            })
        } else {
            None
        };

        let x_scrollbar = if let Some(horizontal) = show_scrollbar_x {
            let Scrollbar {
                width,
                margin,
                scroller_width,
                spacing,
                padding,
                ..
            } = *horizontal;

            // Need to adjust the width of the horizontal scrollbar if the vertical scrollbar
            // is present
            // 如果存在垂直滚动条，需要调整水平滚动条的
            // 宽度
            let scrollbar_y_width =
                y_scrollbar.map_or(0.0, |scrollbar| scrollbar.total_bounds.width);

            let total_scrollbar_height = width.max(scroller_width) + 2.0 * margin;

            // The padding is purely visual: it shrinks the left and right ends of
            // the scrollbar without affecting the layout
            // 内边距纯粹是视觉上的：它缩短滚动条的左右两端，
            // 而不影响布局
            let scrollbar_width = (bounds.width - scrollbar_y_width - 2.0 * padding).max(0.0);

            // Total bounds of the scrollbar + margin + scroller width
            // 滚动条 + 外边距 + 滚动块宽度的总边界
            let total_scrollbar_bounds = Rectangle {
                x: bounds.x + padding,
                y: bounds.y + bounds.height - total_scrollbar_height,
                width: scrollbar_width,
                height: total_scrollbar_height,
            };

            // Bounds of just the scrollbar
            // 仅滚动条本身的边界
            let scrollbar_bounds = Rectangle {
                x: bounds.x + padding,
                y: bounds.y + bounds.height - total_scrollbar_height / 2.0 - width / 2.0,
                width: scrollbar_width,
                height: width,
            };

            let ratio = bounds.width / content.width;

            let scroller = if ratio >= 1.0 {
                None
            } else {
                // min width for easier grabbing with extra wide content
                // 最小宽度，便于在内容特别宽时抓取
                let scroller_length = (scrollbar_bounds.width * ratio).max(2.0);
                let scroller_offset = translation.x * ratio * scrollbar_bounds.width / bounds.width;

                let scroller_bounds = Rectangle {
                    x: (scrollbar_bounds.x + scroller_offset).max(0.0),
                    y: bounds.y + bounds.height
                        - total_scrollbar_height / 2.0
                        - scroller_width / 2.0,
                    width: scroller_length,
                    height: scroller_width,
                };

                Some(internals::Scroller {
                    bounds: scroller_bounds,
                })
            };

            Some(internals::Scrollbar {
                total_bounds: total_scrollbar_bounds,
                bounds: scrollbar_bounds,
                scroller,
                alignment: horizontal.alignment,
                disabled: content.width <= bounds.width,
                floating: spacing.is_none(),
            })
        } else {
            None
        };

        Self {
            y: y_scrollbar,
            x: x_scrollbar,
        }
    }

    /// 给定轴的 [`internals::Scrollbar`]（如果它正在显示）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`internals::Scrollbar`] of the given axis, if it is shown.
    ///
    /// </details>
    fn scrollbar(&self, axis: Axis) -> Option<&internals::Scrollbar> {
        match axis {
            Axis::X => self.x.as_ref(),
            Axis::Y => self.y.as_ref(),
        }
    }

    /// 给定的 `cursor` 所在的滚动条的 [`Axis`]（如果有）。
    ///
    /// 两个滚动条的总边界互不重叠，因此光标最多
    /// 位于其中一个之上；垂直滚动条优先。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Axis`] of the scrollbar the given `cursor` is over, if any.
    ///
    /// The scrollbars' total bounds don't overlap, so the cursor is over at
    /// most one of them; the vertical one takes precedence.
    ///
    /// </details>
    fn is_mouse_over(&self, cursor: mouse::Cursor) -> Option<Axis> {
        let cursor_position = cursor.position()?;

        if self
            .y
            .as_ref()
            .is_some_and(|scrollbar| scrollbar.is_mouse_over(cursor_position))
        {
            Some(Axis::Y)
        } else if self
            .x
            .as_ref()
            .is_some_and(|scrollbar| scrollbar.is_mouse_over(cursor_position))
        {
            Some(Axis::X)
        } else {
            None
        }
    }

    fn is_y_disabled(&self) -> bool {
        self.y.map(|y| y.disabled).unwrap_or(false)
    }

    fn is_x_disabled(&self) -> bool {
        self.x.map(|x| x.disabled).unwrap_or(false)
    }

    /// 给定轴的 [`internals::Scrollbar`]，以及它被给定光标位置
    /// 命中的部分（如果有）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`internals::Scrollbar`] of the given axis and the part of it hit by
    /// the given cursor position, if any.
    ///
    /// </details>
    fn hit(&self, axis: Axis, cursor_position: Point) -> Option<(&internals::Scrollbar, Hit)> {
        let scrollbar = match axis {
            Axis::X => self.x.as_ref(),
            Axis::Y => self.y.as_ref(),
        }?;
        let scroller = scrollbar.scroller?;

        if !scrollbar.total_bounds.contains(cursor_position) {
            return None;
        }

        Some((
            scrollbar,
            if scroller.bounds.contains(cursor_position) {
                Hit::Scroller {
                    grabbed_at: (axis.coordinate(cursor_position)
                        - axis.coordinate(scroller.bounds.position()))
                        / axis.length(scroller.bounds),
                }
            } else {
                Hit::Rail
            },
        ))
    }

    fn is_any_floating(&self) -> bool {
        self.y.is_some_and(|scrollbar| scrollbar.floating)
            || self.x.is_some_and(|scrollbar| scrollbar.floating)
    }

    fn active(&self) -> bool {
        self.y.is_some() || self.x.is_some()
    }
}

pub(super) mod internals {
    use crate::core::{Point, Rectangle};

    use super::Anchor;

    #[derive(Debug, Copy, Clone)]
    pub struct Scrollbar {
        pub total_bounds: Rectangle,
        pub bounds: Rectangle,
        pub scroller: Option<Scroller>,
        pub alignment: Anchor,
        pub disabled: bool,
        pub floating: bool,
    }

    impl Scrollbar {
        /// 返回鼠标是否位于滚动条之上。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Returns whether the mouse is over the scrollbar or not.
        ///
        /// </details>
        pub fn is_mouse_over(&self, cursor_position: Point) -> bool {
            self.total_bounds.contains(cursor_position)
        }

        /// 根据光标位置，返回沿给定轴的
        /// 滚动百分比。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Returns the scrolled percentage from the cursor position, along
        /// the given axis.
        ///
        /// </details>
        pub fn scroll_percentage(
            &self,
            axis: super::Axis,
            grabbed_at: f32,
            cursor_position: Point,
        ) -> f32 {
            let Some(scroller) = self.scroller else {
                return 0.0;
            };

            let percentage = (axis.coordinate(cursor_position)
                - axis.coordinate(self.bounds.position())
                - axis.length(scroller.bounds) * grabbed_at)
                / (axis.length(self.bounds) - axis.length(scroller.bounds));

            match self.alignment {
                Anchor::Start => percentage,
                Anchor::End => 1.0 - percentage,
            }
        }
    }

    /// [`Scrollbar`] 的手柄。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The handle of a [`Scrollbar`].
    ///
    /// </details>
    #[derive(Debug, Clone, Copy)]
    pub struct Scroller {
        /// [`Scroller`] 的边界。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The bounds of the [`Scroller`].
        ///
        /// </details>
        pub bounds: Rectangle,
    }
}

/// [`Scrollable`] 可能的交互状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The possible status of a [`Scrollable`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// [`Scrollable`] 可以进行交互。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Scrollable`] can be interacted with.
    ///
    /// </details>
    Active {
        /// 水平滚动条是否被禁用，被禁用意味着内容没有溢出。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Whether or not the horizontal scrollbar is disabled meaning the content isn't overflowing.
        ///
        /// </details>
        is_horizontal_scrollbar_disabled: bool,
        /// 垂直滚动条是否被禁用，被禁用意味着内容没有溢出。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Whether or not the vertical scrollbar is disabled meaning the content isn't overflowing.
        ///
        /// </details>
        is_vertical_scrollbar_disabled: bool,
    },
    /// [`Scrollable`] 正被悬停。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Scrollable`] is being hovered.
    ///
    /// </details>
    Hovered {
        /// 表示水平滚动条是否正被悬停。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Indicates if the horizontal scrollbar is being hovered.
        ///
        /// </details>
        is_horizontal_scrollbar_hovered: bool,
        /// 表示垂直滚动条是否正被悬停。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Indicates if the vertical scrollbar is being hovered.
        ///
        /// </details>
        is_vertical_scrollbar_hovered: bool,
        /// 水平滚动条是否被禁用，被禁用意味着内容没有溢出。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Whether or not the horizontal scrollbar is disabled meaning the content isn't overflowing.
        ///
        /// </details>
        is_horizontal_scrollbar_disabled: bool,
        /// 垂直滚动条是否被禁用，被禁用意味着内容没有溢出。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Whether or not the vertical scrollbar is disabled meaning the content isn't overflowing.
        ///
        /// </details>
        is_vertical_scrollbar_disabled: bool,
    },
    /// [`Scrollable`] 正被拖动。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Scrollable`] is being dragged.
    ///
    /// </details>
    Dragged {
        /// 表示水平滚动条是否正被拖动。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Indicates if the horizontal scrollbar is being dragged.
        ///
        /// </details>
        is_horizontal_scrollbar_dragged: bool,
        /// 表示垂直滚动条是否正被拖动。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Indicates if the vertical scrollbar is being dragged.
        ///
        /// </details>
        is_vertical_scrollbar_dragged: bool,
        /// 水平滚动条是否被禁用，被禁用意味着内容没有溢出。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Whether or not the horizontal scrollbar is disabled meaning the content isn't overflowing.
        ///
        /// </details>
        is_horizontal_scrollbar_disabled: bool,
        /// 垂直滚动条是否被禁用，被禁用意味着内容没有溢出。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Whether or not the vertical scrollbar is disabled meaning the content isn't overflowing.
        ///
        /// </details>
        is_vertical_scrollbar_disabled: bool,
    },
}

/// 可滚动区域的外观。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The appearance of a scrollable.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// 可滚动区域的 [`container::Style`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`container::Style`] of a scrollable.
    ///
    /// </details>
    pub container: container::Style,
    /// 垂直 [`Rail`] 的外观。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The vertical [`Rail`] appearance.
    ///
    /// </details>
    pub vertical_rail: Rail,
    /// 水平 [`Rail`] 的外观。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The horizontal [`Rail`] appearance.
    ///
    /// </details>
    pub horizontal_rail: Rail,
    /// 水平滚动条与垂直滚动条之间空隙的 [`Background`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Background`] of the gap between a horizontal and vertical scrollbar.
    ///
    /// </details>
    pub gap: Option<Background>,
    /// [`AutoScroll`] 覆盖层的外观。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The appearance of the [`AutoScroll`] overlay.
    ///
    /// </details>
    pub auto_scroll: AutoScroll,
}

/// 可滚动区域的滚动条的外观。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The appearance of the scrollbar of a scrollable.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rail {
    /// 滚动条的 [`Background`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Background`] of a scrollbar.
    ///
    /// </details>
    pub background: Option<Background>,
    /// 滚动条的 [`Border`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Border`] of a scrollbar.
    ///
    /// </details>
    pub border: Border,
    /// 滚动条的 [`Scroller`] 的外观。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The appearance of the [`Scroller`] of a scrollbar.
    ///
    /// </details>
    pub scroller: Scroller,
}

/// 可滚动区域的滚动块的外观。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The appearance of the scroller of a scrollable.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scroller {
    /// 滚动块的 [`Background`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Background`] of the scroller.
    ///
    /// </details>
    pub background: Background,
    /// 滚动块的 [`Border`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Border`] of the scroller.
    ///
    /// </details>
    pub border: Border,
}

/// 可滚动区域的自动滚动覆盖层的外观。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The appearance of the autoscroll overlay of a scrollable.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoScroll {
    /// [`AutoScroll`] 覆盖层的 [`Background`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Background`] of the [`AutoScroll`] overlay.
    ///
    /// </details>
    pub background: Background,
    /// [`AutoScroll`] 覆盖层的 [`Border`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Border`] of the [`AutoScroll`] overlay.
    ///
    /// </details>
    pub border: Border,
    /// [`AutoScroll`] 覆盖层的 [`Shadow`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Thje [`Shadow`] of the [`AutoScroll`] overlay.
    ///
    /// </details>
    pub shadow: Shadow,
    /// [`AutoScroll`] 覆盖层箭头图标的 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Color`] for the arrow icons of the [`AutoScroll`] overlay.
    ///
    /// </details>
    pub icon: Color,
}

/// [`Scrollable`] 的主题样式目录。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The theme catalog of a [`Scrollable`].
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

/// [`Scrollable`] 的样式函数。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A styling function for a [`Scrollable`].
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

/// [`Scrollable`] 的默认样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The default style of a [`Scrollable`].
///
/// </details>
pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();

    let scrollbar = Rail {
        background: Some(palette.background.weak.color.into()),
        border: border::rounded(2),
        scroller: Scroller {
            background: palette.background.strongest.color.into(),
            border: border::rounded(2),
        },
    };

    let auto_scroll = AutoScroll {
        background: palette.background.base.color.scale_alpha(0.9).into(),
        border: border::rounded(u32::MAX)
            .width(1)
            .color(palette.background.base.text.scale_alpha(0.8)),
        shadow: Shadow {
            color: Color::BLACK.scale_alpha(0.7),
            offset: Vector::ZERO,
            blur_radius: 2.0,
        },
        icon: palette.background.base.text.scale_alpha(0.8),
    };

    match status {
        Status::Active { .. } => Style {
            container: container::Style::default(),
            vertical_rail: scrollbar,
            horizontal_rail: scrollbar,
            gap: None,
            auto_scroll,
        },
        Status::Hovered {
            is_horizontal_scrollbar_hovered,
            is_vertical_scrollbar_hovered,
            ..
        } => {
            let hovered_scrollbar = Rail {
                scroller: Scroller {
                    background: palette.primary.strong.color.into(),
                    ..scrollbar.scroller
                },
                ..scrollbar
            };

            Style {
                container: container::Style::default(),
                vertical_rail: if is_vertical_scrollbar_hovered {
                    hovered_scrollbar
                } else {
                    scrollbar
                },
                horizontal_rail: if is_horizontal_scrollbar_hovered {
                    hovered_scrollbar
                } else {
                    scrollbar
                },
                gap: None,
                auto_scroll,
            }
        }
        Status::Dragged {
            is_horizontal_scrollbar_dragged,
            is_vertical_scrollbar_dragged,
            ..
        } => {
            let dragged_scrollbar = Rail {
                scroller: Scroller {
                    background: palette.primary.base.color.into(),
                    ..scrollbar.scroller
                },
                ..scrollbar
            };

            Style {
                container: container::Style::default(),
                vertical_rail: if is_vertical_scrollbar_dragged {
                    dragged_scrollbar
                } else {
                    scrollbar
                },
                horizontal_rail: if is_horizontal_scrollbar_dragged {
                    dragged_scrollbar
                } else {
                    scrollbar
                },
                gap: None,
                auto_scroll,
            }
        }
    }
}
