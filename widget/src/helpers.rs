//! 用于创建纯部件的辅助函数。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Helper functions to create pure widgets.
//!
//! </details>
use crate::button::{self, Button};
use crate::checkbox::{self, Checkbox};
use crate::combo_box::{self, ComboBox};
use crate::container::{self, Container};
use crate::core;
use crate::core::theme;
use crate::core::time::Instant;
use crate::core::widget::Meta;
use crate::core::{Length, Size, Widget};
use crate::float::{self, Float};
use crate::keyed;
use crate::lazy::Lazy;
use crate::opaque::Opaque;
use crate::overlay;
use crate::pane_grid::{self, PaneGrid};
use crate::pick_list::{self, PickList};
use crate::popover::Popover;
use crate::progress_bar::{self, ProgressBar};
use crate::radio::{self, Radio};
use crate::scrollable::{self, Scrollable};
use crate::slider::{self, Slider};
use crate::sticky::Sticky;
use crate::text::{self, Text};
use crate::text_editor::{self, TextEditor};
use crate::text_input::{self, TextInput};
use crate::toggler::{self, Toggler};
use crate::tooltip::Tooltip;
use crate::transition::{self, Transition};
use crate::vertical_slider::{self, VerticalSlider};
use crate::{Column, Grid, Hover, MouseArea, Pin, Responsive, Row, Sensor, Space, Stack, Themer};

use std::borrow::Borrow;
use std::ops::RangeInclusive;

pub use crate::component::component;
pub use crate::table::table;

/// 使用给定的子元素创建一个 [`Column`]。
///
/// 列在垂直方向上分布其子元素。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a [`Column`] with the given children.
///
/// Columns distribute their children vertically.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// use iced::widget::{button, column};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     column![
///         "I am on top!",
///         button("I am in the center!"),
///         "I am below.",
///     ]
/// }
/// ```
#[macro_export]
macro_rules! column {
    () => (
        $crate::Column::new()
    );
    ($($x:expr),+ $(,)?) => (
        $crate::Column::with_children([$($crate::core::Widget::_boxed($x)),+])
    );
}

/// 使用给定的子元素创建一个 [`Row`]。
///
/// 行在水平方向上分布其子元素。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a [`Row`] with the given children.
///
/// Rows distribute their children horizontally.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// use iced::widget::{button, row};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     row![
///         "I am to the left!",
///         button("I am in the middle!"),
///         "I am to the right!",
///     ]
/// }
/// ```
#[macro_export]
macro_rules! row {
    () => (
        $crate::Row::new()
    );
    ($($x:expr),+ $(,)?) => (
        $crate::Row::with_children([$($crate::core::Widget::_boxed($x)),+])
    );
}

/// 使用给定的子元素创建一个 [`Stack`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a [`Stack`] with the given children.
///
/// </details>
///
/// [`Stack`]: crate::Stack
#[macro_export]
macro_rules! stack {
    () => (
        $crate::Stack::new()
    );
    ($($x:expr),+ $(,)?) => (
        $crate::Stack::with_children([$($crate::core::Widget::_boxed($x)),+])
    );
}

/// 使用给定的子元素创建一个 [`Grid`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a [`Grid`] with the given children.
///
/// </details>
///
/// [`Grid`]: crate::Grid
#[macro_export]
macro_rules! grid {
    () => (
        $crate::Grid::new()
    );
    ($($x:expr),+ $(,)?) => (
        $crate::Grid::with_children([$($crate::core::Widget::_boxed($x)),+])
    );
}

/// 使用给定的内容创建一个新的 [`Text`] 部件。
///
/// 此宏使用与 [`format!`] 相同的语法，但创建的是一个新的 [`Text`] 部件。
///
/// 宏参数语法的细节请参见
/// [`std::fmt` 中的格式化文档](std::fmt)。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Text`] widget with the provided content.
///
/// [`Text`]: core::widget::Text
///
/// This macro uses the same syntax as [`format!`], but creates a new [`Text`] widget instead.
///
/// See [the formatting documentation in `std::fmt`](std::fmt)
/// for details of the macro argument syntax.
///
/// </details>
///
/// # 示例（Examples）
///
/// ```no_run
/// # mod iced {
/// #     pub mod widget {
/// #         macro_rules! text {
/// #           ($($arg:tt)*) => {iced_widget::text::<iced_widget::Theme>("text")}
/// #         }
/// #         pub(crate) use text;
/// #     }
/// # }
/// # use iced_widget::Widget;
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::core::Theme, ()>;
/// use iced::widget::text;
///
/// enum Message {
///     // ...
/// }
///
/// fn view(_state: &State) -> impl Widget<Message> {
///     let simple = text!("Hello, world!");
///
///     let keyword = text!("Hello, {}", "world!");
///
///     let planet = "Earth";
///     let local_variable = text!("Hello, {planet}!");
///     // ...
///     # simple
/// }
/// ```
#[macro_export]
macro_rules! text {
    ($($arg:tt)*) => {
        $crate::Text::new(format!($($arg)*))
    };
}

/// 使用给定的文本片段创建 [`Rich`] 富文本。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates some [`Rich`] text with the given spans.
///
/// </details>
///
/// [`Rich`]: text::Rich
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// use iced::font;
/// use iced::widget::{rich_text, span};
/// use iced::{color, never, Font};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     rich_text![
///         span("I am red!").color(color!(0xff0000)),
///         span(" "),
///         span("And I am bold!").font(Font { weight: font::Weight::Bold, ..Font::default() }),
///     ]
///     .on_link_click(never)
///     .size(20)
/// }
/// ```
#[macro_export]
macro_rules! rich_text {
    () => (
        $crate::text::Rich::new()
    );
    ($($x:expr),+ $(,)?) => (
        $crate::text::Rich::from_iter([$($crate::text::Span::from($x)),+])
    );
}

/// 使用给定的内容创建一个新的 [`Container`]。
///
/// 容器让你可以在其边界内对齐一个部件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Container`] with the provided content.
///
/// Containers let you align a widget inside their boundaries.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// use iced::widget::container;
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     container("This text is centered inside a rounded box!")
///         .padding(10)
///         .center(800)
///         .style(container::rounded_box)
/// }
/// ```
pub fn container<'a, W, Theme>(content: W) -> Container<'a, W, Theme>
where
    Theme: container::Catalog + 'a,
{
    Container::new(content)
}

/// 创建一个新的 [`Container`]，它填满所有可用空间，
/// 并让其内容居中。
///
/// 这等价于：
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Container`] that fills all the available space
/// and centers its contents inside.
///
/// This is equivalent to:
///
/// </details>
///
/// ```rust,no_run
/// # use iced_widget::core::Length::Fill;
/// # use iced_widget::Container;
/// # fn container<A>(x: A) -> Container<'static, ()> { unreachable!() }
/// let center = container("Center!").center(Fill);
/// ```
///
/// [`Container`]: crate::Container
pub fn center<'a, W, Theme>(content: W) -> Container<'a, W, Theme>
where
    Theme: container::Catalog + 'a,
{
    container(content).center(Length::Fill)
}

/// 创建一个新的 [`Container`]，它在水平方向上填满所有可用空间，
/// 并让其内容居中。
///
/// 这等价于：
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Container`] that fills all the available space
/// horizontally and centers its contents inside.
///
/// This is equivalent to:
///
/// </details>
///
/// ```rust,no_run
/// # use iced_widget::core::Length::Fill;
/// # use iced_widget::Container;
/// # fn container<A>(x: A) -> Container<'static, ()> { unreachable!() }
/// let center_x = container("Horizontal Center!").center_x(Fill);
/// ```
///
/// [`Container`]: crate::Container
pub fn center_x<'a, W, Theme>(content: W) -> Container<'a, W, Theme>
where
    Theme: container::Catalog + 'a,
{
    container(content).center_x(Length::Fill)
}

/// 创建一个新的 [`Container`]，它在垂直方向上填满所有可用空间，
/// 并让其内容居中。
///
/// 这等价于：
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Container`] that fills all the available space
/// vertically and centers its contents inside.
///
/// This is equivalent to:
///
/// </details>
///
/// ```rust,no_run
/// # use iced_widget::core::Length::Fill;
/// # use iced_widget::Container;
/// # fn container<A>(x: A) -> Container<'static, ()> { unreachable!() }
/// let center_y = container("Vertical Center!").center_y(Fill);
/// ```
///
/// [`Container`]: crate::Container
pub fn center_y<'a, W, Theme>(content: W) -> Container<'a, W, Theme>
where
    Theme: container::Catalog + 'a,
{
    container(content).center_y(Length::Fill)
}

/// 创建一个新的 [`Container`]，它在水平方向上填满所有可用空间，
/// 并让其内容右对齐。
///
/// 这等价于：
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Container`] that fills all the available space
/// horizontally and right-aligns its contents inside.
///
/// This is equivalent to:
///
/// </details>
///
/// ```rust,no_run
/// # use iced_widget::core::Length::Fill;
/// # use iced_widget::Container;
/// # fn container<A>(x: A) -> Container<'static, ()> { unreachable!() }
/// let right = container("Right!").align_right(Fill);
/// ```
///
/// [`Container`]: crate::Container
pub fn right<'a, W, Theme>(content: W) -> Container<'a, W, Theme>
where
    Theme: container::Catalog + 'a,
{
    container(content).align_right(Length::Fill)
}

/// 创建一个新的 [`Container`]，它填满所有可用空间，
/// 并让其内容对齐到右侧居中的位置。
///
/// 这等价于：
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Container`] that fills all the available space
/// and aligns its contents inside to the right center.
///
/// This is equivalent to:
///
/// </details>
///
/// ```rust,no_run
/// # use iced_widget::core::Length::Fill;
/// # use iced_widget::Container;
/// # fn container<A>(x: A) -> Container<'static, ()> { unreachable!() }
/// let right_center = container("Bottom Center!").align_right(Fill).center_y(Fill);
/// ```
///
/// [`Container`]: crate::Container
pub fn right_center<'a, W, Theme>(content: W) -> Container<'a, W, Theme>
where
    Theme: container::Catalog + 'a,
{
    container(content)
        .align_right(Length::Fill)
        .center_y(Length::Fill)
}

/// 创建一个新的 [`Container`]，它在垂直方向上填满所有可用空间，
/// 并让其内容底部对齐。
///
/// 这等价于：
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Container`] that fills all the available space
/// vertically and bottom-aligns its contents inside.
///
/// This is equivalent to:
///
/// </details>
///
/// ```rust,no_run
/// # use iced_widget::core::Length::Fill;
/// # use iced_widget::Container;
/// # fn container<A>(x: A) -> Container<'static, ()> { unreachable!() }
/// let bottom = container("Bottom!").align_bottom(Fill);
/// ```
///
/// [`Container`]: crate::Container
pub fn bottom<'a, W, Theme>(content: W) -> Container<'a, W, Theme>
where
    Theme: container::Catalog + 'a,
{
    container(content).align_bottom(Length::Fill)
}

/// 创建一个新的 [`Container`]，它填满所有可用空间，
/// 并让其内容对齐到底部居中的位置。
///
/// 这等价于：
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Container`] that fills all the available space
/// and aligns its contents inside to the bottom center.
///
/// This is equivalent to:
///
/// </details>
///
/// ```rust,no_run
/// # use iced_widget::core::Length::Fill;
/// # use iced_widget::Container;
/// # fn container<A>(x: A) -> Container<'static, ()> { unreachable!() }
/// let bottom_center = container("Bottom Center!").center_x(Fill).align_bottom(Fill);
/// ```
///
/// [`Container`]: crate::Container
pub fn bottom_center<'a, W, Theme>(content: W) -> Container<'a, W, Theme>
where
    Theme: container::Catalog + 'a,
{
    container(content)
        .center_x(Length::Fill)
        .align_bottom(Length::Fill)
}

/// 创建一个新的 [`Container`]，它填满所有可用空间，
/// 并让其内容对齐到右下角。
///
/// 这等价于：
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Container`] that fills all the available space
/// and aligns its contents inside to the bottom right corner.
///
/// This is equivalent to:
///
/// </details>
///
/// ```rust,no_run
/// # use iced_widget::core::Length::Fill;
/// # use iced_widget::Container;
/// # fn container<A>(x: A) -> Container<'static, ()> { unreachable!() }
/// let bottom_right = container("Bottom!").align_right(Fill).align_bottom(Fill);
/// ```
///
/// [`Container`]: crate::Container
pub fn bottom_right<'a, W, Theme>(content: W) -> Container<'a, W, Theme>
where
    Theme: container::Catalog + 'a,
{
    container(content)
        .align_right(Length::Fill)
        .align_bottom(Length::Fill)
}

/// 使用给定的内容创建一个新的 [`Pin`] 部件。
///
/// [`Pin`] 部件把其内容放置在自身边界内的某个固定坐标处。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Pin`] widget with the given content.
///
/// A [`Pin`] widget positions its contents at some fixed coordinates inside of its boundaries.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; pub use iced_widget::core::Length::Fill; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// use iced::widget::pin;
/// use iced::Fill;
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     pin("This text is displayed at coordinates (50, 50)!")
///         .x(50)
///         .y(50)
/// }
/// ```
pub fn pin<W>(content: W) -> Pin<W> {
    Pin::new(content)
}

/// 使用给定的子元素创建一个新的 [`Column`]。
///
/// 列在垂直方向上分布其子元素。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Column`] with the given children.
///
/// Columns distribute their children vertically.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # pub type State = ();
/// use iced::widget::{column, text, Widget};
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     column((0..5).map(|i| text!("Item {i}")))
/// }
/// ```
pub fn column<W>(children: impl IntoIterator<Item = W>) -> Column<W>
where
    W: Meta,
{
    Column::with_children(children)
}

/// 从元素的迭代器创建一个新的 [`keyed::Column`]。
///
/// 带键的列在垂直方向上分布内容，同时保持连续性。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`keyed::Column`] from an iterator of elements.
///
/// Keyed columns distribute content vertically while keeping continuity.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// use iced::widget::{keyed_column, text};
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     keyed_column((0..=100).map(|i| {
///         (i, text!("Item {i}"))
///     }))
/// }
/// ```
pub fn keyed_column<Key, W>(children: impl IntoIterator<Item = (Key, W)>) -> keyed::Column<Key, W>
where
    Key: Copy + PartialEq,
    W: Meta,
{
    keyed::Column::with_children(children)
}

/// 从迭代器创建一个新的 [`Row`]。
///
/// 行在水平方向上分布其子元素。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Row`] from an iterator.
///
/// Rows distribute their children horizontally.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # pub type State = ();
/// use iced::widget::{row, text, Widget};
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     row((0..5).map(|i| text!("Item {i}")))
/// }
/// ```
pub fn row<W>(children: impl IntoIterator<Item = W>) -> Row<W>
where
    W: Meta,
{
    Row::with_children(children)
}

/// 从迭代器创建一个新的 [`Grid`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Grid`] from an iterator.
///
/// </details>
pub fn grid<W>(children: impl IntoIterator<Item = W>) -> Grid<W>
where
    W: Meta,
{
    Grid::with_children(children)
}

/// 使用给定的子元素创建一个新的 [`Stack`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Stack`] with the given children.
///
/// </details>
///
/// [`Stack`]: crate::Stack
pub fn stack<W>(children: impl IntoIterator<Item = W>) -> Stack<W>
where
    W: Meta,
{
    Stack::with_children(children)
}

/// 包装给定的部件，并捕获该部件边界内的所有鼠标按键
/// 按下事件——实际上就是让它变得*不透明*。
///
/// 此辅助函数用于标记 [`Stack`] 中的元素，或者
/// 覆盖层（例如 [`Popover`] 的弹出内容）中的元素，以免鼠标事件
/// 穿透到下方的元素。
///
/// 使用 [`Opaque::capture_scroll`] 可以同时捕获滚轮事件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Wraps the given widget and captures any mouse button presses inside the bounds of
/// the widget—effectively making it _opaque_.
///
/// This helper is meant to be used to mark elements in a [`Stack`], or inside an
/// overlay—like the popup of a [`Popover`]—to avoid mouse events from passing
/// through to the elements below.
///
/// Use [`Opaque::capture_scroll`] to also capture scroll wheel events.
///
/// </details>
///
/// [`Stack`]: crate::Stack
/// [`Popover`]: crate::popover::Popover
pub fn opaque<W>(content: W) -> Opaque<W> {
    Opaque::new(content)
}

/// 把一个部件显示在另一个部件之上，但仅当底层部件处于悬停状态时才显示。
///
/// 它的工作方式与 [`stack`] 类似，但只有当光标位于底层部件上时
/// 才会显示顶层。它可以用来减少视觉上的杂乱。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Displays a widget on top of another one, only when the base widget is hovered.
///
/// This works analogously to a [`stack`], but it will only display the layer on top
/// when the cursor is over the base. It can be useful for removing visual clutter.
///
/// </details>
///
/// [`stack`]: stack()
pub fn hover<W, V>(base: W, top: V) -> crate::Hover<W, V> {
    Hover::new(base, top)
}

/// 创建一个新的 [`Sensor`] 部件。
///
/// [`Sensor`] 部件可以在其内容显示、隐藏或
/// 尺寸变化时产生消息。
///
/// 它甚至可以在给定的距离处提前通知你！
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Sensor`] widget.
///
/// A [`Sensor`] widget can generate messages when its contents are shown,
/// hidden, or resized.
///
/// It can even notify you with anticipation at a given distance!
///
/// </details>
pub fn sensor<'a, Message, W>(content: W) -> Sensor<'a, (), Message, W> {
    Sensor::new(content)
}

/// 使用给定的内容创建一个新的 [`Scrollable`]。
///
/// 可滚动区域让用户可以借助滚动条浏览任意多的内容。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Scrollable`] with the provided content.
///
/// Scrollables let users navigate an endless amount of content with a scrollbar.
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
pub fn scrollable<'a, Message, W, Theme>(content: W) -> Scrollable<'a, Message, W, Theme>
where
    Theme: scrollable::Catalog + 'a,
{
    Scrollable::new(content)
}

/// 为给定的内容创建一个新的 [`Sticky`]。
///
/// 每当 [`Sticky`] 的内容本该滚出视野时，它们会显示在
/// 可见边界内的一个覆盖层上。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Sticky`] for the provided content.
///
/// The contents of a [`Sticky`] will be displayed on an overlay, inside the
/// visible bounds, whenever they would otherwise go out of view.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; pub use iced_widget::core::Length::Fill; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// use iced::widget::{column, container, scrollable, sticky, space};
/// use iced::Fill;
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     scrollable(column![
///         sticky(container("I always stay in view!").width(Fill).padding(10)),
///         space().height(3000),
///     ])
/// }
/// ```
pub fn sticky<W>(content: W) -> Sticky<W> {
    Sticky::new(content)
}

/// 使用给定的内容创建一个新的 [`Button`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Button`] with the provided content.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// use iced::widget::button;
///
/// #[derive(Clone)]
/// enum Message {
///     ButtonPressed,
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     button("Press me!").on_press(Message::ButtonPressed)
/// }
/// ```
pub fn button<'a, Message, W, Theme>(content: W) -> Button<'a, Message, W, Theme>
where
    Theme: button::Catalog + 'a,
{
    Button::new(content)
}

/// 使用给定的 [`Widget`] 为给定的内容创建一个新的 [`Popover`]。
///
/// 弹出层在某个元素之上显示一块浮动的内容。`popover`
/// 参数是一个 `Option`：`Some` 显示覆盖层（打开），`None`
/// 隐藏它（关闭）。底层部件始终存在。默认情况下，弹出层的
/// 位置为 [`crate::popover::Position::Auto`]；使用 [`Popover::position`]
/// 方法可以设置具体的位置。当用户点击其边界之外时，
/// 弹出层会通过 `on_close` 处理函数通知应用程序。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Popover`] for the provided content with the given [`Widget`].
///
/// Popovers display a floating piece of content over some element. The `popover`
/// argument is an `Option`: `Some` displays the overlay (open), and `None`
/// hides it (closed). The base is always present. By default, the popover is
/// positioned [`crate::popover::Position::Auto`]; use the [`Popover::position`]
/// method to set a specific position. When the user clicks outside of its
/// bounds, the popover notifies the application through its `on_close` handler.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// use iced::widget::{button, container, popover, text};
///
/// #[derive(Clone)]
/// enum Message {
///     Close,
/// }
///
/// struct State {
///     is_open: bool,
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     // The base is always present. The `popover` argument is `Some` when the
///     // 底层部件始终存在。弹出层打开时 `popover` 参数为 `Some`，
///     // popover is open and `None` when it is closed.
///     // 关闭时为 `None`。
///     popover(
///         button(text("Click me!")).on_press(Message::Close),
///         state.is_open.then(|| {
///             container(text("This is the popover contents!")).padding(10)
///         }),
///     )
///     .position(popover::Position::Bottom)
///     .on_close(Message::Close)
/// }
/// ```
pub fn popover<W, V, Message>(
    content: W,
    popover: Option<V>,
) -> crate::Popover<W, Opaque<V>, Message> {
    Popover::new(content, popover.map(opaque))
}

/// 使用给定的 [`Widget`] 为给定的内容创建一个新的
/// [`Tooltip`]。
///
/// 工具提示在某个元素处于悬停状态时，在其上方显示一条提示信息。
///
/// 默认情况下，工具提示的位置为 [`crate::tooltip::Position::Auto`]；
/// 使用 [`Tooltip::position`] 方法可以设置具体的位置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Tooltip`] for the provided content with the given
/// [`Widget`].
///
/// Tooltips display a hint of information over some element when hovered.
///
/// By default, the tooltip is positioned [`crate::tooltip::Position::Auto`];
/// use the [`Tooltip::position`] method to set a specific position.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// use iced::widget::{container, tooltip};
///
/// enum Message {
///     // ...
/// }
///
/// fn view(_state: &State) -> impl Widget<Message> {
///     tooltip(
///         "Hover me to display the tooltip!",
///         container("This is the tooltip contents!")
///             .padding(10)
///             .style(container::rounded_box),
///     )
///     .position(tooltip::Position::Bottom)
/// }
/// ```
pub fn tooltip<W, V>(content: W, tooltip: V) -> crate::Tooltip<W, V> {
    Tooltip::new(content, tooltip)
}

/// 使用给定的内容创建一个新的 [`Text`] 部件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Text`] widget with the provided content.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::core::Theme, ()>;
/// use iced::widget::text;
/// use iced::color;
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     text("Hello, this is iced!")
///         .size(20)
///         .color(color!(0x0000ff))
/// }
/// ```
pub fn text<'a, Theme>(text: impl text::IntoFragment<'a>) -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    Text::new(text)
}

/// 创建一个新的 [`Text`] 部件，显示给定的值。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Text`] widget that displays the provided value.
///
/// </details>
pub fn value<'a, Theme>(value: impl ToString) -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    Text::new(value.to_string())
}

/// 使用给定的文本片段创建一个新的 [`Rich`] 富文本部件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Rich`] text widget with the provided spans.
///
/// </details>
///
/// [`Rich`]: text::Rich
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// use iced::font;
/// use iced::widget::{rich_text, span};
/// use iced::{color, never, Font};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     LinkClicked(&'static str),
///     // ...
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     rich_text([
///         span("I am red!").color(color!(0xff0000)),
///         span(" "),
///         span("And I am bold!").font(Font { weight: font::Weight::Bold, ..Font::default() }),
///     ])
///     .on_link_click(never)
///     .size(20)
/// }
/// ```
pub fn rich_text<'a, Link, Message, Theme>(
    spans: impl AsRef<[text::Span<'a, Link>]> + 'a,
) -> text::Rich<'a, Link, Message, Theme>
where
    Link: Clone + 'static,
    Theme: text::Catalog + 'a,
{
    text::Rich::with_spans(spans)
}

/// 使用给定的内容创建一个新的文本 [`Span`]。
///
/// [`Span`] 是 [`Rich`] 富文本中的一个片段。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Span`] of text with the provided content.
///
/// A [`Span`] is a fragment of some [`Rich`] text.
///
/// </details>
///
/// [`Span`]: text::Span
/// [`Rich`]: text::Rich
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// use iced::font;
/// use iced::widget::{rich_text, span};
/// use iced::{color, never, Font};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     rich_text![
///         span("I am red!").color(color!(0xff0000)),
///         " ",
///         span("And I am bold!").font(Font { weight: font::Weight::Bold, ..Font::default() }),
///     ]
///     .on_link_click(never)
///     .size(20)
/// }
/// ```
pub fn span<'a, Link>(text: impl text::IntoFragment<'a>) -> text::Span<'a, Link> {
    text::Span::new(text)
}

#[cfg(feature = "markdown")]
#[doc(inline)]
pub use crate::markdown::view as markdown;

/// 创建一个新的 [`Checkbox`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Checkbox`].
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// #
/// use iced::widget::checkbox;
///
/// struct State {
///    is_checked: bool,
/// }
///
/// enum Message {
///     CheckboxToggled(bool),
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     checkbox(state.is_checked)
///         .label("Toggle me!")
///         .on_toggle(Message::CheckboxToggled)
/// }
///
/// fn update(state: &mut State, message: Message) {
///     match message {
///         Message::CheckboxToggled(is_checked) => {
///             state.is_checked = is_checked;
///         }
///     }
/// }
/// ```
/// ![由 `iced_wgpu` 绘制的复选框](https://github.com/iced-rs/iced/blob/7760618fb112074bc40b148944521f312152012a/docs/images/checkbox.png?raw=true)
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// ![Checkbox drawn by `iced_wgpu`](https://github.com/iced-rs/iced/blob/7760618fb112074bc40b148944521f312152012a/docs/images/checkbox.png?raw=true)
///
/// </details>
pub fn checkbox<'a, Message, Theme, Renderer>(
    is_checked: bool,
) -> Checkbox<'a, Message, Theme, Renderer>
where
    Theme: checkbox::Catalog + 'a,
    Renderer: core::text::Renderer,
{
    Checkbox::new(is_checked)
}

/// 创建一个新的 [`Radio`]。
///
/// 单选按钮让用户可以从一组选项中选择一项。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Radio`].
///
/// Radio buttons let users choose a single option from a bunch of options.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// #
/// use iced::widget::{column, radio};
///
/// struct State {
///    selection: Option<Choice>,
/// }
///
/// #[derive(Debug, Clone, Copy)]
/// enum Message {
///     RadioSelected(Choice),
/// }
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// enum Choice {
///     A,
///     B,
///     C,
///     All,
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     let a = radio(
///         "A",
///         Choice::A,
///         state.selection,
///         Message::RadioSelected,
///     );
///
///     let b = radio(
///         "B",
///         Choice::B,
///         state.selection,
///         Message::RadioSelected,
///     );
///
///     let c = radio(
///         "C",
///         Choice::C,
///         state.selection,
///         Message::RadioSelected,
///     );
///
///     let all = radio(
///         "All of the above",
///         Choice::All,
///         state.selection,
///         Message::RadioSelected
///     );
///
///     column![a, b, c, all]
/// }
/// ```
pub fn radio<'a, Message, Theme, V>(
    label: impl Into<String>,
    value: V,
    selected: Option<V>,
    on_click: impl FnOnce(V) -> Message,
) -> Radio<'a, Message, Theme>
where
    Message: Clone,
    Theme: radio::Catalog + 'a,
    V: Copy + Eq,
{
    Radio::new(label, value, selected, on_click)
}

/// 创建一个新的 [`Toggler`]。
///
/// 开关让用户可以通过拨动开关来做出二元选择。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Toggler`].
///
/// Togglers let users make binary choices by toggling a switch.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// #
/// use iced::widget::toggler;
///
/// struct State {
///    is_checked: bool,
/// }
///
/// enum Message {
///     TogglerToggled(bool),
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     toggler(state.is_checked)
///         .label("Toggle me!")
///         .on_toggle(Message::TogglerToggled)
/// }
///
/// fn update(state: &mut State, message: Message) {
///     match message {
///         Message::TogglerToggled(is_checked) => {
///             state.is_checked = is_checked;
///         }
///     }
/// }
/// ```
pub fn toggler<'a, Message, Theme>(is_checked: bool) -> Toggler<'a, Message, Theme>
where
    Theme: toggler::Catalog + 'a,
{
    Toggler::new(is_checked)
}

/// 创建一个新的 [`TextInput`]。
///
/// 文本输入框显示可以填入文本的字段。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`TextInput`].
///
/// Text inputs display fields that can be filled with text.
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
pub fn text_input<'a, Message, Theme>(
    placeholder: impl text::IntoFragment<'a>,
    value: impl text::IntoFragment<'a>,
) -> TextInput<'a, Message, Theme>
where
    Message: Clone,
    Theme: text_input::Catalog + 'a,
{
    TextInput::new(placeholder, value)
}

/// 创建一个新的 [`TextEditor`]。
///
/// 文本编辑器显示一个用于编辑文本的多行文本输入框。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`TextEditor`].
///
/// Text editors display a multi-line text input for text editing.
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
pub fn text_editor<'a, Message, Theme, Renderer>(
    content: &'a text_editor::Content<Renderer>,
) -> TextEditor<'a, core::text::parser::PlainText, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: text_editor::Catalog + 'a,
    Renderer: core::text::Renderer,
{
    TextEditor::new(content)
}

/// 创建一个新的 [`Slider`]。
///
/// 滑块让用户可以通过移动指示器来设置一个值。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Slider`].
///
/// Sliders let users set a value by moving an indicator.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// #
/// use iced::widget::slider;
///
/// struct State {
///    value: f32,
/// }
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     ValueChanged(f32),
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     slider(0.0..=100.0, state.value, Message::ValueChanged)
/// }
///
/// fn update(state: &mut State, message: Message) {
///     match message {
///         Message::ValueChanged(value) => {
///             state.value = value;
///         }
///     }
/// }
/// ```
pub fn slider<'a, T, Message, Theme>(
    range: std::ops::RangeInclusive<T>,
    value: T,
    on_change: impl Fn(T) -> Message + 'a,
) -> Slider<'a, T, Message, Theme>
where
    T: Copy + std::cmp::PartialOrd,
    Message: Clone,
    Theme: slider::Catalog + 'a,
{
    Slider::new(range, value, on_change)
}

/// 创建一个新的 [`VerticalSlider`]。
///
/// 滑块让用户可以通过移动指示器来设置一个值。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`VerticalSlider`].
///
/// Sliders let users set a value by moving an indicator.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// #
/// use iced::widget::vertical_slider;
///
/// struct State {
///    value: f32,
/// }
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     ValueChanged(f32),
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     vertical_slider(0.0..=100.0, state.value, Message::ValueChanged)
/// }
///
/// fn update(state: &mut State, message: Message) {
///     match message {
///         Message::ValueChanged(value) => {
///             state.value = value;
///         }
///     }
/// }
/// ```
pub fn vertical_slider<'a, T, Message, Theme>(
    range: std::ops::RangeInclusive<T>,
    value: T,
    on_change: impl Fn(T) -> Message + 'a,
) -> VerticalSlider<'a, T, Message, Theme>
where
    T: Copy + std::cmp::PartialOrd,
    Message: Clone,
    Theme: vertical_slider::Catalog + 'a,
{
    VerticalSlider::new(range, value, on_change)
}

/// 创建一个新的 [`PickList`]。
///
/// 选择列表显示一个可选择的选项下拉列表。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`PickList`].
///
/// Pick lists display a dropdown list of selectable options.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// #
/// use iced::widget::pick_list;
///
/// struct State {
///    favorite: Option<Fruit>,
/// }
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// enum Fruit {
///     Apple,
///     Orange,
///     Strawberry,
///     Tomato,
/// }
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     FruitSelected(Fruit),
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     let fruits = [
///         Fruit::Apple,
///         Fruit::Orange,
///         Fruit::Strawberry,
///         Fruit::Tomato,
///     ];
///
///     pick_list(
///         state.favorite,
///         fruits,
///         Fruit::to_string,
///     )
///     .on_select(Message::FruitSelected)
///     .placeholder("Select your favorite fruit...")
/// }
///
/// fn update(state: &mut State, message: Message) {
///     match message {
///         Message::FruitSelected(fruit) => {
///             state.favorite = Some(fruit);
///         }
///     }
/// }
///
/// impl std::fmt::Display for Fruit {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         f.write_str(match self {
///             Self::Apple => "Apple",
///             Self::Orange => "Orange",
///             Self::Strawberry => "Strawberry",
///             Self::Tomato => "Tomato",
///         })
///     }
/// }
/// ```
pub fn pick_list<'a, T, L, V, Message, Theme>(
    selected: Option<V>,
    options: L,
    to_string: impl Fn(&T) -> String + 'a,
) -> PickList<'a, T, L, V, Message, Theme>
where
    T: PartialEq + Clone + 'a,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    Message: Clone,
    Theme: pick_list::Catalog + overlay::menu::Catalog,
{
    PickList::new(selected, options, to_string)
}

/// 创建一个新的 [`ComboBox`]。
///
/// 组合框显示一个可搜索、可选择的选项下拉列表。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`ComboBox`].
///
/// Combo boxes display a dropdown list of searchable and selectable options.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// #
/// use iced::widget::combo_box;
///
/// struct State {
///    fruits: combo_box::State<Fruit>,
///    favorite: Option<Fruit>,
/// }
///
/// #[derive(Debug, Clone)]
/// enum Fruit {
///     Apple,
///     Orange,
///     Strawberry,
///     Tomato,
/// }
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     FruitSelected(Fruit),
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     combo_box(
///         &state.fruits,
///         "Select your favorite fruit...",
///         state.favorite.as_ref(),
///         Message::FruitSelected
///     )
/// }
///
/// fn update(state: &mut State, message: Message) {
///     match message {
///         Message::FruitSelected(fruit) => {
///             state.favorite = Some(fruit);
///         }
///     }
/// }
///
/// impl std::fmt::Display for Fruit {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         f.write_str(match self {
///             Self::Apple => "Apple",
///             Self::Orange => "Orange",
///             Self::Strawberry => "Strawberry",
///             Self::Tomato => "Tomato",
///         })
///     }
/// }
/// ```
pub fn combo_box<'a, T, Message, Theme>(
    state: &'a combo_box::State<T>,
    placeholder: impl text::IntoFragment<'a>,
    selection: Option<&T>,
    on_selected: impl Fn(T) -> Message + 'a,
) -> ComboBox<'a, T, Message, Theme>
where
    T: std::fmt::Display + Clone,
    Theme: combo_box::Catalog + 'a,
{
    ComboBox::new(state, placeholder, selection, on_selected)
}

/// 创建一个没有尺寸的空 [`Space`]。
///
/// 它被视为“恒等”部件。它不占用
/// 任何空间，也不做任何事。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates some empty [`Space`] with no size.
///
/// This is considered the "identity" widget. It will take
/// no space and do nothing.
///
/// </details>
pub fn space() -> Space {
    Space::new()
}

/// 创建一个新的 [`ProgressBar`]。
///
/// 进度条以可视化的方式展示耗时较长的计算机操作的进展，例如下载、文件传输或安装。
///
/// 它需要：
///   * 一个由可能取值构成的闭区间，以及
///   * [`ProgressBar`] 的当前值。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`ProgressBar`].
///
/// Progress bars visualize the progression of an extended computer operation, such as a download, file transfer, or installation.
///
/// It expects:
///   * an inclusive range of possible values, and
///   * the current value of the [`ProgressBar`].
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
pub fn progress_bar<'a, Theme>(range: RangeInclusive<f32>, value: f32) -> ProgressBar<'a, Theme>
where
    Theme: progress_bar::Catalog + 'a,
{
    ProgressBar::new(range, value)
}

/// 创建一个新的 [`Image`]。
///
/// 图像用于显示各种格式（PNG、JPG 等）的位图图形。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Image`].
///
/// Images display raster graphics in different formats (PNG, JPG, etc.).
///
/// </details>
///
/// [`Image`]: crate::Image
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// use iced::widget::image;
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     image("ferris.png")
/// }
/// ```
/// <img src="https://github.com/iced-rs/iced/blob/9712b319bb7a32848001b96bd84977430f14b623/examples/resources/ferris.png?raw=true" width="300">
#[cfg(feature = "image")]
pub fn image<Handle>(handle: impl Into<Handle>) -> crate::Image<Handle> {
    crate::Image::new(handle.into())
}

/// 从给定的 [`Handle`] 创建一个新的 [`Svg`] 部件。
///
/// Svg 部件在你的应用程序中显示矢量图形。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Svg`] widget from the given [`Handle`].
///
/// Svg widgets display vector graphics in your application.
///
/// </details>
///
/// [`Svg`]: crate::Svg
/// [`Handle`]: crate::svg::Handle
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// use iced::widget::svg;
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     svg("tiger.svg")
/// }
/// ```
#[cfg(feature = "svg")]
pub fn svg<'a, Theme>(handle: impl Into<core::svg::Handle>) -> crate::Svg<'a, Theme>
where
    Theme: crate::svg::Catalog,
{
    crate::Svg::new(handle)
}

/// 创建一个 [`Element`](crate::Element)，以给定的 `text_size`
/// 显示 iced 的徽标。
///
/// 例如，可以用它在“关于”界面里向你最喜欢的 GUI 库
/// 表达一点爱意。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates an [`Element`](crate::Element) that displays the iced logo with
/// the given `text_size`.
///
/// Useful for showing some love to your favorite GUI library in your "About" screen,
/// for instance.
///
/// </details>
pub fn iced<'a, Message, Theme, Renderer>(
    text_size: impl Into<core::Pixels>,
) -> impl Widget<Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: core::text::Renderer + 'a,
    Theme: text::Catalog + container::Catalog + 'a,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
    <Theme as text::Catalog>::Class<'a>: From<text::StyleFn<'a, Theme>>,
{
    use crate::core::border;
    use crate::core::color;
    use crate::core::gradient;
    use crate::core::{Alignment, Color, Font, Radians};

    let text_size = text_size.into();

    row![
        container(
            text(Renderer::ICED_LOGO)
                .line_height(1.0)
                .size(text_size)
                .font(Renderer::ICON_FONT)
                .color(Color::WHITE)
        )
        .padding(text_size * 0.15)
        .style(move |_| container::Style {
            background: Some(
                gradient::Linear::new(Radians::PI / 4.0)
                    .add_stop(0.0, color!(0x0033ff))
                    .add_stop(1.0, color!(0x1177ff))
                    .into()
            ),
            border: border::rounded(border::radius(text_size * 0.4)),
            ..container::Style::default()
        }),
        text("iced").size(text_size).font(Font::MONOSPACE)
    ]
    .spacing(text_size.0 / 3.0)
    .align_y(Alignment::Center)
}

/// 创建一个新的 [`Canvas`]。
///
/// 画布可以用来绘制可交互的 2D 图形。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Canvas`].
///
/// Canvases can be leveraged to draw interactive 2D graphics.
///
/// </details>
///
/// [`Canvas`]: crate::Canvas
///
/// # 示例：绘制一个简单的圆（Example: Drawing a Simple Circle）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// #
/// use iced::mouse;
/// use iced::widget::canvas;
/// use iced::{Color, Rectangle, Renderer, Theme};
///
/// // First, we define the data we need for drawing
/// // 首先，定义绘制所需的数据
/// #[derive(Debug)]
/// struct Circle {
///     radius: f32,
/// }
///
/// // Then, we implement the `Program` trait
/// // 然后，实现 `Program`特质
/// impl<Message> canvas::Program<Message> for Circle {
///     // No internal state
///     // 没有内部状态
///     type State = ();
///
///     fn draw(
///         &self,
///         _state: &(),
///         renderer: &Renderer,
///         _theme: &Theme,
///         bounds: Rectangle,
///         _cursor: mouse::Cursor
///     ) -> Vec<canvas::Geometry> {
///         // We prepare a new `Frame`
///         // 准备一个新的 `Frame`
///         let mut frame = canvas::Frame::new(renderer, bounds.size());
///
///         // We create a `Path` representing a simple circle
///         // 创建一个表示简单圆形的 `Path`
///         let circle = canvas::Path::circle(frame.center(), self.radius);
///
///         // And fill it with some color
///         // 并用某种颜色填充它
///         frame.fill(&circle, Color::BLACK);
///
///         // Then, we produce the geometry
///         // 然后，生成几何图形
///         vec![frame.into_geometry()]
///     }
/// }
///
/// // Finally, we simply use our `Circle` to create the `Canvas`!
/// // 最后，只需用我们的 `Circle` 创建 `Canvas`！
/// fn view<Message>(_state: &State) -> impl Widget<Message> {
///     canvas(Circle { radius: 50.0 })
/// }
/// ```
#[cfg(feature = "canvas")]
pub fn canvas<P, Message, Theme, Renderer>(program: P) -> crate::Canvas<P, Message, Theme, Renderer>
where
    Renderer: crate::graphics::geometry::Renderer,
    P: crate::canvas::Program<Message, Theme, Renderer>,
{
    crate::Canvas::new(program)
}

/// 从给定的 [`Data`] 创建一个新的 [`QRCode`] 部件。
///
/// 二维码用一种二维矩阵条码来显示信息。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`QRCode`] widget from the given [`Data`].
///
/// QR codes display information in a type of two-dimensional matrix barcode.
///
/// </details>
///
/// [`QRCode`]: crate::QRCode
/// [`Data`]: crate::qr_code::Data
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// #
/// use iced::widget::qr_code;
///
/// struct State {
///    data: qr_code::Data,
/// }
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     qr_code(&state.data)
/// }
/// ```
#[cfg(feature = "qr_code")]
pub fn qr_code<'a, Theme>(data: &'a crate::qr_code::Data) -> crate::QRCode<'a, Theme>
where
    Theme: crate::qr_code::Catalog + 'a,
{
    crate::QRCode::new(data)
}

/// 创建一个新的 [`Shader`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Shader`].
///
/// </details>
///
/// [`Shader`]: crate::Shader
#[cfg(feature = "wgpu")]
pub fn shader<Message, P>(program: P) -> crate::Shader<Message, P>
where
    P: crate::shader::Program<Message>,
{
    crate::Shader::new(program)
}

/// 创建一个新的 [`MouseArea`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`MouseArea`].
///
/// </details>
pub fn mouse_area<'a, Message, W>(widget: W) -> MouseArea<'a, Message, W> {
    MouseArea::new(widget)
}

/// 把任意 `Theme` 应用到其内容上的部件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A widget that applies any `Theme` to its contents.
///
/// </details>
pub fn themer<'a, Message, Theme, Renderer>(
    theme: Option<Theme>,
    content: impl Widget<Message, Theme, Renderer> + 'a,
) -> Themer<'a, Message, Theme, Renderer>
where
    Theme: theme::Base,
{
    Themer::new(theme, content)
}

/// 使用给定的 [`pane_grid::State`] 和视图函数创建一个 [`PaneGrid`]。
///
/// 窗格网格让用户可以分割应用程序的区域，并动态地组织布局。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a [`PaneGrid`] with the given [`pane_grid::State`] and view function.
///
/// Pane grids let your users split regions of your application and organize layout dynamically.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// #
/// use iced::widget::{pane_grid, text};
///
/// struct State {
///     panes: pane_grid::State<Pane>,
/// }
///
/// enum Pane {
///     SomePane,
///     AnotherKindOfPane,
/// }
///
/// enum Message {
///     PaneDragged(pane_grid::DragEvent),
///     PaneResized(pane_grid::ResizeEvent),
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     pane_grid(&state.panes, |pane, state, is_maximized| {
///         pane_grid::Content::new(match state {
///             Pane::SomePane => text("This is some pane"),
///             Pane::AnotherKindOfPane => text("This is another kind of pane"),
///         })
///     })
///     .on_drag(Message::PaneDragged)
///     .on_resize(10, Message::PaneResized)
/// }
/// ```
pub fn pane_grid<'a, T, Message, Title, W, Theme, Renderer>(
    state: &'a pane_grid::State<T>,
    view: impl Fn(
        pane_grid::Pane,
        &'a T,
        bool,
    ) -> pane_grid::Content<'a, Message, Title, W, Theme, Renderer>,
) -> PaneGrid<'a, Message, Title, W, Theme, Renderer>
where
    Theme: pane_grid::Catalog,
{
    PaneGrid::new(state, view)
}

/// 使用给定的内容创建一个新的 [`Float`] 部件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Float`] widget with the given content.
///
/// </details>
pub fn float<'a, W, Theme>(content: W) -> Float<'a, W, Theme>
where
    Theme: float::Catalog,
{
    Float::new(content)
}

/// 使用一个产生其内容的闭包创建一个新的 [`Responsive`]
/// 部件。
///
/// 布局期间，`view` 闭包会收到 [`Responsive`] 可用的
/// 最大空间。你可以利用这个 [`Size`]
/// 有条件地构建内容。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Responsive`] widget with a closure that produces its
/// contents.
///
/// The `view` closure will receive the maximum available space for
/// the [`Responsive`] during layout. You can use this [`Size`] to
/// conditionally build the contents.
///
/// </details>
pub fn responsive<'a, W>(f: impl Fn(Size) -> W + 'a) -> Responsive<'a, W> {
    Responsive::new(f)
}

/// 创建一个新的 [`Transition`]。
///
/// `init` 闭包用于初始化一个 [`Program`] 的实现者。它通常是
/// 一个 [`Animation`](crate::core::Animation)，但你也可以为自己的类型
/// 实现 [`Program`]。
///
/// `view` 闭包会收到 [`Program`] 和当前的 [`Instant`]，可以用它们对值进行插值。
/// 当 `value` 发生变化时，它会每帧被调用一次，直到 [`Program`] 停止动画。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Transition`].
///
/// The `init` closure will be used to initialize an implementor of [`Program`]. This is normally
/// an [`Animation`](crate::core::Animation), but you can implement [`Program`] on your own types
/// as well.
///
/// The `view` closure will receive the [`Program`] and the current [`Instant`], which can be used for interpolating values.
/// When the `value` changes, this will be called every frame, until the [`Program`] stops animating.
///
/// </details>
///
/// [`Program`]: transition::Program
///
/// # 示例（Example）
///
/// 下面演示如何实现一个平滑的进度条：
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Here is how you could implement a smooth progress bar:
///
/// </details>
///
/// ```
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// use iced::widget::{transition, progress_bar};
/// use iced::Animation;
///
/// fn smooth_progress_bar<Message>(progress: f32) -> impl Widget<Message> {
///     transition(progress, || Animation::new(0.).quick(), |animation, now| {
///         progress_bar(0.0..=1.0, animation.interpolate_with(std::convert::identity, now))
///     })
/// }
/// ```
pub fn transition<'a, Message, W, P>(
    value: P::Value,
    init: impl Fn() -> P + 'a,
    view: impl Fn(&P, Instant) -> W + 'a,
) -> Transition<'a, Message, W, P>
where
    P: transition::Program,
{
    Transition::new(init, value, view)
}

/// 创建一个零尺寸的 [`Widget`]，它什么都不做，并且会被
/// 容器过滤掉。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a zero-sized [`Widget`] that does nothing and will be filtered out by
/// containers.
///
/// </details>
pub fn void() -> core::widget::Void {
    core::widget::Void
}

/// 使用给定的数据 `Dependency` 和一个能把该数据转换为部件树的
/// 闭包，创建一个新的 [`Lazy`] 部件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Lazy`] widget with the given data `Dependency` and a
/// closure that can turn this data into a widget tree.
///
/// </details>
pub fn lazy<'a, W, Dependency>(
    dependency: Dependency,
    view: impl Fn(&Dependency) -> W + 'a,
) -> Lazy<'a, W, Dependency>
where
    Dependency: std::hash::Hash + 'a,
{
    Lazy::new(dependency, view)
}
