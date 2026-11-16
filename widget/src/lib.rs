//! 使用内置部件，或者创建你自己的部件。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Use the built-in widgets or create your own.
//!
//! </details>
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/iced-rs/iced/9ab6923e943f784985e9ef9ca28b10278297225d/docs/logo.svg"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
pub use iced_renderer as renderer;
pub use iced_renderer::core;
pub use iced_renderer::graphics;

pub use core::widget::{Id, Void};

mod action;
mod column;
mod hover;
mod mouse_area;
mod pin;
mod responsive;
mod stack;
mod themer;

pub mod button;
pub mod checkbox;
pub mod combo_box;
pub mod component;
pub mod container;
pub mod float;
pub mod grid;
pub mod keyed;
pub mod lazy;
pub mod opaque;
pub mod overlay;
pub mod pane_grid;
pub mod pick_list;
pub mod popover;
pub mod progress_bar;
pub mod radio;
pub mod row;
pub mod rule;
pub mod scrollable;
pub mod sensor;
pub mod slider;
pub mod space;
pub mod sticky;
pub mod table;
pub mod text;
pub mod text_editor;
pub mod text_input;
pub mod toggler;
pub mod tooltip;
pub mod transition;
pub mod vertical_slider;

mod helpers;

pub use helpers::*;

#[doc(no_inline)]
pub use button::Button;
#[doc(no_inline)]
pub use checkbox::Checkbox;
#[doc(no_inline)]
pub use column::Column;
#[doc(no_inline)]
pub use combo_box::ComboBox;
#[doc(no_inline)]
pub use component::Component;
#[doc(no_inline)]
pub use container::Container;
#[doc(no_inline)]
pub use float::Float;
#[doc(no_inline)]
pub use grid::Grid;
#[doc(no_inline)]
pub use hover::Hover;
#[doc(no_inline)]
pub use lazy::Lazy;
#[doc(no_inline)]
pub use mouse_area::MouseArea;
#[doc(no_inline)]
pub use opaque::Opaque;
#[doc(no_inline)]
pub use pane_grid::PaneGrid;
#[doc(no_inline)]
pub use pick_list::PickList;
#[doc(no_inline)]
pub use pin::Pin;
#[doc(no_inline)]
pub use popover::Popover;
#[doc(no_inline)]
pub use progress_bar::ProgressBar;
#[doc(no_inline)]
pub use radio::Radio;
#[doc(no_inline)]
pub use responsive::Responsive;
#[doc(no_inline)]
pub use row::Row;
#[doc(no_inline)]
pub use rule::Rule;
#[doc(no_inline)]
pub use scrollable::Scrollable;
#[doc(no_inline)]
pub use sensor::Sensor;
#[doc(no_inline)]
pub use slider::Slider;
#[doc(no_inline)]
pub use space::Space;
#[doc(no_inline)]
pub use stack::Stack;
#[doc(no_inline)]
pub use sticky::Sticky;
#[doc(no_inline)]
pub use text::Text;
#[doc(no_inline)]
pub use text_editor::TextEditor;
#[doc(no_inline)]
pub use text_input::TextInput;
#[doc(no_inline)]
pub use themer::Themer;
#[doc(no_inline)]
pub use toggler::Toggler;
#[doc(no_inline)]
pub use tooltip::Tooltip;
#[doc(no_inline)]
pub use vertical_slider::VerticalSlider;

#[cfg(feature = "wgpu")]
pub mod shader;

#[cfg(feature = "wgpu")]
#[doc(no_inline)]
pub use shader::Shader;

#[cfg(feature = "svg")]
pub mod svg;

#[cfg(feature = "svg")]
#[doc(no_inline)]
pub use svg::Svg;

#[cfg(feature = "image")]
pub mod image;

#[cfg(feature = "image")]
#[doc(no_inline)]
pub use image::Image;

#[cfg(feature = "canvas")]
pub mod canvas;

#[cfg(feature = "canvas")]
#[doc(no_inline)]
pub use canvas::Canvas;

#[cfg(feature = "qr_code")]
pub mod qr_code;

#[cfg(feature = "qr_code")]
#[doc(no_inline)]
pub use qr_code::QRCode;

#[cfg(feature = "markdown")]
pub mod markdown;

pub use crate::core::theme::{self, Theme};
pub use action::Action;
pub use renderer::Renderer;

use crate::core::layout::{self, Layout};
use crate::core::mouse;
use crate::core::widget::tree;
use crate::core::widget::{self, Tree};
use crate::core::{Border, Color, Event, Length, Rectangle, Shell, Size, Vector};

/// 装箱的 [`Widget`]。
///
/// 它是 [`core::Element`] 的别名，带有默认的 `Theme` 和
/// `Renderer` 参数。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A boxed [`Widget`].
///
/// This is an alias of [`core::Element`] with default `Theme` and
/// `Renderer` parameters.
///
/// </details>
pub type Element<'a, Message, Theme = crate::Theme, Renderer = crate::Renderer> =
    crate::core::Element<'a, Message, Theme, Renderer>;

/// 可以被 iced 渲染的部件。
///
/// 此特质为 [`core::Widget`] 扩展了几个便捷方法——例如
/// [`Widget::map`]、[`Widget::boxed`] 和 [`Widget::explain`]——并把
/// `Theme` 和 `Renderer` 参数默认设为 iced 自己的类型。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A widget that can be rendered by iced.
///
/// This trait extends [`core::Widget`] with a few convenient methods—like
/// [`Widget::map`], [`Widget::boxed`], and [`Widget::explain`]—and defaults
/// the `Theme` and `Renderer` parameters to iced's.
///
/// </details>
pub trait Widget<Message, Theme = crate::Theme, Renderer = crate::Renderer>:
    core::Widget<Message, Theme, Renderer>
{
    /// 对此 [`Widget`] 产生的消息应用一个变换。
    ///
    /// 当你想解耦 UI 的不同部分并让它们
    /// **可组合**时，此方法很有用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies a transformation to the produced message of this [`Widget`].
    ///
    /// This method is useful when you want to decouple different parts of your
    /// UI and make them __composable__.
    ///
    /// </details>
    ///
    /// # 示例（Example）
    /// 假设我们想使用[我们的计数器](index.html#usage)。但我们不是
    /// 只显示一个计数器，而是想显示很多个。我们可以原封不动地
    /// 复用 `Counter` 类型！
    ///
    /// 我们用组合来为新应用程序的**状态**建模：
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Imagine we want to use [our counter](index.html#usage). But instead of
    /// showing a single counter, we want to display many of them. We can reuse
    /// the `Counter` type as it is!
    ///
    /// We use composition to model the __state__ of our new application:
    ///
    /// </details>
    ///
    /// ```
    /// # mod counter {
    /// #     pub struct Counter;
    /// # }
    /// use counter::Counter;
    ///
    /// struct ManyCounters {
    ///     counters: Vec<Counter>,
    /// }
    /// ```
    ///
    /// 现在我们可以存储多个计数器的状态了。然而，
    /// 之前实现的**消息**描述的是**单个**计数器的
    /// 用户交互。现在，我们还需要分辨是哪个
    /// 计数器在接收用户交互。还能再用一次组合吗？
    /// 可以。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// We can store the state of multiple counters now. However, the
    /// __messages__ we implemented before describe the user interactions
    /// of a __single__ counter. Right now, we need to also identify which
    /// counter is receiving user interactions. Can we use composition again?
    /// Yes.
    ///
    /// </details>
    ///
    /// ```
    /// # mod counter {
    /// #     #[derive(Debug, Clone, Copy)]
    /// #     pub enum Message {}
    /// # }
    /// #[derive(Debug, Clone, Copy)]
    /// pub enum Message {
    ///     Counter(usize, counter::Message)
    /// }
    /// ```
    ///
    /// 我们把之前的**消息**与产生它们的计数器的索引
    /// 组合起来。现在来实现我们的**视图逻辑**：
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// We compose the previous __messages__ with the index of the counter
    /// producing them. Let's implement our __view logic__ now:
    ///
    /// </details>
    ///
    /// ```no_run
    /// # mod iced {
    /// #     pub use iced_widget::core::Function;
    /// #     pub use iced_widget::Widget;
    /// #     pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::core::Theme, ()>;
    /// #
    /// #     pub mod widget {
    /// #         pub fn row<'a, Message>(iter: impl IntoIterator<Item = super::Element<'a, Message>>) -> super::Element<'a, Message> {
    /// #             unimplemented!()
    /// #         }
    /// #     }
    /// # }
    /// #
    /// # mod counter {
    /// #     #[derive(Debug, Clone, Copy)]
    /// #     pub enum Message {}
    /// #     pub struct Counter;
    /// #
    /// #     pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::core::Theme, ()>;
    /// #
    /// #     impl Counter {
    /// #         pub fn view(&self) -> Element<Message> {
    /// #             unimplemented!()
    /// #         }
    /// #     }
    /// # }
    /// #
    /// use counter::Counter;
    ///
    /// use iced::widget::row;
    /// use iced::{Function, Widget};
    ///
    /// struct ManyCounters {
    ///     counters: Vec<Counter>,
    /// }
    ///
    /// #[derive(Debug, Clone, Copy)]
    /// pub enum Message {
    ///     Counter(usize, counter::Message),
    /// }
    ///
    /// impl ManyCounters {
    ///     pub fn view(&self) -> impl Widget<Message, iced_widget::Theme, ()> {
    ///         // We can quickly populate a `row` by mapping our counters
    ///         // 通过映射我们的计数器，可以快速填充一个 `row`
    ///         row(
    ///             self.counters
    ///                 .iter()
    ///                 .map(Counter::view)
    ///                 .enumerate()
    ///                 .map(|(index, counter)| {
    ///                     // Here we turn our `Element<counter::Message>` into
    ///                     // 在这里，我们把 `Element<counter::Message>` 转换为
    ///                     // an `Element<Message>` by combining the `index` and the
    ///                     // `Element<Message>`，方法是把 `index` 与
    ///                     // message of the `element`.
    ///                     // `element` 的消息组合起来。
    ///                     counter.map(Message::Counter.with(index)).boxed()
    ///                 }),
    ///         )
    ///     }
    /// }
    /// ```
    ///
    /// 最后，我们的**更新逻辑**非常直接：简单的
    /// 委托。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Finally, our __update logic__ is pretty straightforward: simple
    /// delegation.
    ///
    /// </details>
    ///
    /// ```
    /// # mod counter {
    /// #     #[derive(Debug, Clone, Copy)]
    /// #     pub enum Message {}
    /// #     pub struct Counter;
    /// #
    /// #     impl Counter {
    /// #         pub fn update(&mut self, _message: Message) {}
    /// #     }
    /// # }
    /// #
    /// # use counter::Counter;
    /// #
    /// # struct ManyCounters {
    /// #     counters: Vec<Counter>,
    /// # }
    /// #
    /// # #[derive(Debug, Clone, Copy)]
    /// # pub enum Message {
    /// #    Counter(usize, counter::Message)
    /// # }
    /// impl ManyCounters {
    ///     pub fn update(&mut self, message: Message) {
    ///         match message {
    ///             Message::Counter(index, counter_msg) => {
    ///                 if let Some(counter) = self.counters.get_mut(index) {
    ///                     counter.update(counter_msg);
    ///                 }
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    fn map<F, B>(self, f: F) -> widget::Map<Self, F, Message>
    where
        Self: Sized,
        F: Fn(Message) -> B,
    {
        core::Widget::_map(self, f)
    }

    /// 把此 [`Widget`] 装箱，转换为通用的 [`Element`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Boxes this [`Widget`], turning it into a generic [`Element`].
    ///
    /// </details>
    fn boxed<'a>(self) -> Element<'a, Message, Theme, Renderer>
    where
        Self: Sized + 'a,
    {
        core::Widget::_boxed(self)
    }

    /// 把此 [`Widget`] 标记为*待解释*。
    ///
    /// [`Renderer`] 会以图形方式解释它的布局。
    /// 这对调试布局非常有用！
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Marks this [`Widget`] as _to-be-explained_.
    ///
    /// The [`Renderer`] will explain its layout graphically.
    /// This can be very useful for debugging your layout!
    ///
    /// </details>
    ///
    /// [`Renderer`]: crate::Renderer
    fn explain(self, color: impl Into<Color>) -> impl core::Widget<Message, Theme, Renderer>
    where
        Self: Sized,
        Renderer: core::Renderer,
    {
        Explain {
            widget: self,
            color: color.into(),
        }
    }
}

impl<T, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for T where
    T: core::Widget<Message, Theme, Renderer>
{
}

/// 解释其子部件布局的 [`Widget`]。
///
/// [`Renderer`] 会以图形方式解释该布局。这对调试布局
/// 非常有用！
///
/// 此部件由 [`Widget::explain`] 返回。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A [`Widget`] that explains the layout of its child.
///
/// The [`Renderer`] will explain the layout graphically. This can be very
/// useful for debugging your layout!
///
/// This widget is returned by [`Widget::explain`].
///
/// </details>
///
/// [`Renderer`]: crate::Renderer
pub struct Explain<W> {
    widget: W,
    color: Color,
}

impl<W> widget::Meta for Explain<W> {}

impl<W, Message, Theme, Renderer> core::Widget<Message, Theme, Renderer> for Explain<W>
where
    W: core::Widget<Message, Theme, Renderer>,
    Renderer: core::Renderer,
{
    fn size(&self) -> Size<Length> {
        self.widget.size()
    }

    fn tag(&self) -> tree::Tag {
        self.widget.tag()
    }

    fn state(&self) -> tree::State {
        self.widget.state()
    }

    fn diff(&mut self, tree: &mut Tree) {
        self.widget.diff(tree);
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.widget.layout(tree, renderer, limits);
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.widget
            .operate(tree, layout, viewport, renderer, operation);
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
        self.widget
            .update(tree, event, layout, cursor, renderer, shell, viewport);
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &core::renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        fn explain_layout<Renderer: core::Renderer>(
            tree: &Tree,
            renderer: &mut Renderer,
            color: Color,
            layout: Layout,
        ) {
            renderer.fill_quad(
                core::renderer::Quad {
                    bounds: layout.bounds(),
                    border: Border {
                        color,
                        width: 1.0,
                        ..Border::default()
                    },
                    ..core::renderer::Quad::default()
                },
                Color::TRANSPARENT,
            );

            for (layout, tree) in layout.iter(&tree.children) {
                explain_layout(tree, renderer, color, layout);
            }
        }

        self.widget
            .draw(tree, renderer, theme, style, layout, cursor, viewport);

        renderer.with_layer(Rectangle::INFINITE, |renderer| {
            explain_layout(tree, renderer, self.color, layout);
        });
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.widget
            .mouse_interaction(tree, layout, cursor, viewport, renderer)
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<core::overlay::Element<'b, Message, Theme, Renderer>> {
        self.widget
            .overlay(tree, layout, renderer, viewport, translation, window)
    }
}
