use crate::layout;
use crate::mouse;
use crate::overlay;
use crate::renderer;
use crate::widget;
use crate::widget::tree::{self, Tree};
use crate::{
    Border, Clipboard, Color, Event, Layout, Length, Rectangle, Shell, Size,
    Vector, Widget,
};

use std::borrow::Borrow;

/// 一个通用的 [`Widget`]。
///
/// 它适合用来构建可组合的用户界面，而不会在
/// **视图逻辑**中泄漏实现细节。
///
/// 如果你有一个[内置部件][built-in widget]，应该可以用 `Into<Element>`
/// 把它转换成 [`Element`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A generic [`Widget`].
///
/// It is useful to build composable user interfaces that do not leak
/// implementation details in their __view logic__.
///
/// If you have a [built-in widget], you should be able to use `Into<Element>`
/// to turn it into an [`Element`].
///
/// </details>
///
/// [built-in widget]: crate::widget
pub struct Element<'a, Message, Theme, Renderer> {
    widget: Box<dyn Widget<Message, Theme, Renderer> + 'a>,
}

impl<'a, Message, Theme, Renderer> Element<'a, Message, Theme, Renderer> {
    /// 创建一个包含给定 [`Widget`] 的新 [`Element`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Element`] containing the given [`Widget`].
    ///
    /// </details>
    pub fn new(widget: impl Widget<Message, Theme, Renderer> + 'a) -> Self
    where
        Renderer: crate::Renderer,
    {
        Self {
            widget: Box::new(widget),
        }
    }

    /// 返回 [`Element`] 的 [`Widget`] 的引用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a reference to the [`Widget`] of the [`Element`],
    ///
    /// </details>
    pub fn as_widget(&self) -> &dyn Widget<Message, Theme, Renderer> {
        self.widget.as_ref()
    }

    /// 返回 [`Element`] 的 [`Widget`] 的可变引用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a mutable reference to the [`Widget`] of the [`Element`],
    ///
    /// </details>
    pub fn as_widget_mut(
        &mut self,
    ) -> &mut dyn Widget<Message, Theme, Renderer> {
        self.widget.as_mut()
    }

    /// 对 [`Element`] 产生的消息应用一个变换。
    ///
    /// 当你想解耦界面的不同部分，并让它们
    /// **可组合**时，此方法很有用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies a transformation to the produced message of the [`Element`].
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
    /// #     pub use iced_core::Function;
    /// #     pub type Element<'a, Message> = iced_core::Element<'a, Message, iced_core::Theme, ()>;
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
    /// #     pub type Element<'a, Message> = iced_core::Element<'a, Message, iced_core::Theme, ()>;
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
    /// use iced::{Element, Function};
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
    ///     pub fn view(&self) -> Element<Message> {
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
    ///                     counter.map(Message::Counter.with(index))
    ///                 }),
    ///         )
    ///         .into()
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
    pub fn map<B>(
        self,
        f: impl Fn(Message) -> B + 'a,
    ) -> Element<'a, B, Theme, Renderer>
    where
        Message: 'a,
        Theme: 'a,
        Renderer: crate::Renderer + 'a,
        B: 'a,
    {
        Element::new(Map::new(self.widget, f))
    }

    /// 把 [`Element`] 标记为*待解释*。
    ///
    /// [`Renderer`] 会以图形方式解释 [`Element`] 的布局。
    /// 这对调试布局非常有用！
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Marks the [`Element`] as _to-be-explained_.
    ///
    /// The [`Renderer`] will explain the layout of the [`Element`] graphically.
    /// This can be very useful for debugging your layout!
    ///
    /// </details>
    ///
    /// [`Renderer`]: crate::Renderer
    pub fn explain<C: Into<Color>>(
        self,
        color: C,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Message: 'a,
        Theme: 'a,
        Renderer: crate::Renderer + 'a,
    {
        Element {
            widget: Box::new(Explain::new(self, color.into())),
        }
    }
}

impl<'a, Message, Theme, Renderer>
    Borrow<dyn Widget<Message, Theme, Renderer> + 'a>
    for Element<'a, Message, Theme, Renderer>
{
    fn borrow(&self) -> &(dyn Widget<Message, Theme, Renderer> + 'a) {
        self.widget.borrow()
    }
}

impl<'a, Message, Theme, Renderer>
    Borrow<dyn Widget<Message, Theme, Renderer> + 'a>
    for &Element<'a, Message, Theme, Renderer>
{
    fn borrow(&self) -> &(dyn Widget<Message, Theme, Renderer> + 'a) {
        self.widget.borrow()
    }
}

struct Map<'a, A, B, Theme, Renderer> {
    widget: Box<dyn Widget<A, Theme, Renderer> + 'a>,
    mapper: Box<dyn Fn(A) -> B + 'a>,
}

impl<'a, A, B, Theme, Renderer> Map<'a, A, B, Theme, Renderer> {
    pub fn new<F>(
        widget: Box<dyn Widget<A, Theme, Renderer> + 'a>,
        mapper: F,
    ) -> Map<'a, A, B, Theme, Renderer>
    where
        F: 'a + Fn(A) -> B,
    {
        Map {
            widget,
            mapper: Box::new(mapper),
        }
    }
}

impl<'a, A, B, Theme, Renderer> Widget<B, Theme, Renderer>
    for Map<'a, A, B, Theme, Renderer>
where
    Renderer: crate::Renderer + 'a,
    A: 'a,
    B: 'a,
{
    fn tag(&self) -> tree::Tag {
        self.widget.tag()
    }

    fn state(&self) -> tree::State {
        self.widget.state()
    }

    fn children(&self) -> Vec<Tree> {
        self.widget.children()
    }

    fn diff(&self, tree: &mut Tree) {
        self.widget.diff(tree);
    }

    fn size(&self) -> Size<Length> {
        self.widget.size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.widget.size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.widget.layout(tree, renderer, limits)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.widget.operate(tree, layout, renderer, operation);
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, B>,
        viewport: &Rectangle,
    ) {
        let mut local_messages = Vec::new();
        let mut local_shell = Shell::new(&mut local_messages);

        self.widget.update(
            tree,
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            &mut local_shell,
            viewport,
        );

        shell.merge(local_shell, &self.mapper);
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.widget
            .draw(tree, renderer, theme, style, layout, cursor, viewport);
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
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
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, B, Theme, Renderer>> {
        let mapper = &self.mapper;

        self.widget
            .overlay(tree, layout, renderer, viewport, translation)
            .map(move |overlay| overlay.map(mapper))
    }
}

struct Explain<'a, Message, Theme, Renderer: crate::Renderer> {
    element: Element<'a, Message, Theme, Renderer>,
    color: Color,
}

impl<'a, Message, Theme, Renderer> Explain<'a, Message, Theme, Renderer>
where
    Renderer: crate::Renderer,
{
    fn new(
        element: Element<'a, Message, Theme, Renderer>,
        color: Color,
    ) -> Self {
        Explain { element, color }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Explain<'_, Message, Theme, Renderer>
where
    Renderer: crate::Renderer,
{
    fn size(&self) -> Size<Length> {
        self.element.widget.size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.element.widget.size_hint()
    }

    fn tag(&self) -> tree::Tag {
        self.element.widget.tag()
    }

    fn state(&self) -> tree::State {
        self.element.widget.state()
    }

    fn children(&self) -> Vec<Tree> {
        self.element.widget.children()
    }

    fn diff(&self, tree: &mut Tree) {
        self.element.widget.diff(tree);
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.element.widget.layout(tree, renderer, limits)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.element
            .widget
            .operate(tree, layout, renderer, operation);
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
        self.element.widget.update(
            tree, event, layout, cursor, renderer, clipboard, shell, viewport,
        );
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        fn explain_layout<Renderer: crate::Renderer>(
            renderer: &mut Renderer,
            color: Color,
            layout: Layout<'_>,
        ) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border: Border {
                        color,
                        width: 1.0,
                        ..Border::default()
                    },
                    ..renderer::Quad::default()
                },
                Color::TRANSPARENT,
            );

            for child in layout.children() {
                explain_layout(renderer, color, child);
            }
        }

        self.element
            .widget
            .draw(tree, renderer, theme, style, layout, cursor, viewport);

        renderer.with_layer(Rectangle::INFINITE, |renderer| {
            explain_layout(renderer, self.color, layout);
        });
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.element
            .widget
            .mouse_interaction(tree, layout, cursor, viewport, renderer)
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.element.widget.overlay(
            tree,
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, T, Message, Theme, Renderer> From<Option<T>>
    for Element<'a, Message, Theme, Renderer>
where
    T: Into<Self>,
    Renderer: crate::Renderer,
{
    fn from(element: Option<T>) -> Self {
        struct Void;

        impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Void
        where
            Renderer: crate::Renderer,
        {
            fn size(&self) -> Size<Length> {
                Size {
                    width: Length::Fixed(0.0),
                    height: Length::Fixed(0.0),
                }
            }

            fn layout(
                &mut self,
                _tree: &mut Tree,
                _renderer: &Renderer,
                _limits: &layout::Limits,
            ) -> layout::Node {
                layout::Node::new(Size::ZERO)
            }

            fn draw(
                &self,
                _tree: &Tree,
                _renderer: &mut Renderer,
                _theme: &Theme,
                _style: &renderer::Style,
                _layout: Layout<'_>,
                _cursor: mouse::Cursor,
                _viewport: &Rectangle,
            ) {
            }
        }

        element.map(T::into).unwrap_or_else(|| Element::new(Void))
    }
}
