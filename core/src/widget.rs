//! 创建自定义部件并对它们进行操作。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Create custom widgets and operate on them.
//!
//! </details>
pub mod operation;
pub mod text;
pub mod tree;

mod element;
mod id;

pub use element::Element;
pub use id::Id;
pub use operation::Operation;
pub use text::Text;
pub use tree::Tree;

use crate::layout::{self, Layout};
use crate::mouse;
use crate::overlay;
use crate::renderer;
use crate::shell;
use crate::{Event, Length, Rectangle, Shell, Size, Vector};

/// 显示信息并允许交互的组件。
///
/// 如果你想构建自己的部件，就需要实现这个
/// 特质。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A component that displays information and allows interaction.
///
/// If you want to build your own widgets, you will need to implement this
/// trait.
///
/// </details>
///
/// # 示例（Examples）
/// 仓库中有一些[示例][examples]，展示了如何实现自定义
/// 部件：
///
/// - [`custom_widget`]：演示如何构建一个绘制圆形的
///   自定义部件。
/// - [`geometry`]：一个自定义部件，展示如何使用 [`iced_wgpu`] 中的
///   `Mesh2D` 图元绘制几何图形。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The repository has some [examples] showcasing how to implement a custom
/// widget:
///
/// - [`custom_widget`], a demonstration of how to build a custom widget that
///   draws a circle.
/// - [`geometry`], a custom widget showcasing how to draw geometry with the
///   `Mesh2D` primitive in [`iced_wgpu`].
///
/// </details>
///
/// [examples]: https://github.com/iced-rs/iced/tree/master/examples
/// [`custom_widget`]: https://github.com/iced-rs/iced/tree/master/examples/custom_widget
/// [`geometry`]: https://github.com/iced-rs/iced/tree/master/examples/geometry
/// [`iced_wgpu`]: https://github.com/iced-rs/iced/tree/master/wgpu
pub trait Widget<Message, Theme, Renderer>: Meta {
    /// 返回 [`Widget`] 以长度表示的 [`Size`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Size`] of the [`Widget`] in lengths.
    ///
    /// </details>
    fn size(&self) -> Size<Length>;

    /// 返回 [`Widget`] 的 [`Tag`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Tag`] of the [`Widget`].
    ///
    /// </details>
    ///
    /// [`Tag`]: tree::Tag
    fn tag(&self) -> tree::Tag {
        tree::Tag::stateless()
    }

    /// 返回 [`Widget`] 的 [`State`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`State`] of the [`Widget`].
    ///
    /// </details>
    ///
    /// [`State`]: tree::State
    fn state(&self) -> tree::State {
        tree::State::None
    }

    /// 把 [`Widget`] 与给定的 [`Tree`] 进行协调。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Reconciles the [`Widget`] with the provided [`Tree`].
    ///
    /// </details>
    fn diff(&mut self, tree: &mut Tree) {
        tree.children.clear();
    }

    /// 对 [`Widget`] 进行布局。
    ///
    /// 它会计算 [`Widget`] 的 [`Layout`]，并把结果存储
    /// 在给定的 [`Tree`] 中。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Lays out the [`Widget`].
    ///
    /// This computes the [`Layout`] of the [`Widget`] and stores the result
    /// in the provided [`Tree`].
    ///
    /// </details>
    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits);

    /// 使用关联的 `Renderer` 绘制 [`Widget`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the [`Widget`] using the associated `Renderer`.
    ///
    /// </details>
    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    );

    /// 对 [`Widget`] 应用一个 [`Operation`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies an [`Operation`] to the [`Widget`].
    ///
    /// </details>
    fn operate(
        &mut self,
        _tree: &mut Tree,
        _layout: Layout,
        _viewport: &Rectangle,
        _renderer: &Renderer,
        _operation: &mut dyn Operation,
    ) {
    }

    /// 处理运行时的 [`Event`]。
    ///
    /// 默认情况下，它什么都不做。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Processes a runtime [`Event`].
    ///
    /// By default, it does nothing.
    ///
    /// </details>
    fn update(
        &mut self,
        _tree: &mut Tree,
        _event: &Event,
        _layout: Layout,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
        _shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
    }

    /// 返回 [`Widget`] 当前的 [`mouse::Interaction`]。
    ///
    /// 默认情况下，它返回 [`mouse::Interaction::None`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current [`mouse::Interaction`] of the [`Widget`].
    ///
    /// By default, it returns [`mouse::Interaction::None`].
    ///
    /// </details>
    fn mouse_interaction(
        &self,
        _tree: &Tree,
        _layout: Layout,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        mouse::Interaction::None
    }

    /// 返回 [`Widget`] 的覆盖层。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the overlays of the [`Widget`].
    ///
    /// </details>
    fn overlay<'a>(
        &'a mut self,
        _tree: &'a mut Tree,
        _layout: Layout,
        _renderer: &Renderer,
        _viewport: &Rectangle,
        _translation: Vector,
        _window: Size,
    ) -> Vec<overlay::Element<'a, Message, Theme, Renderer>> {
        Vec::new()
    }

    /// 对此 [`Widget`] 产生的消息应用一个变换，
    /// 返回一个 [`Map`]。
    ///
    /// 当你想把 UI 的不同部分解耦，并让它们
    /// __可组合__ 时，此方法很有用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies a transformation to the produced message of this [`Widget`],
    /// returning a [`Map`].
    ///
    /// This method is useful when you want to decouple different parts of your
    /// UI and make them __composable__.
    ///
    /// </details>
    fn _map<F, B>(self, f: F) -> Map<Self, F, Message>
    where
        Self: Sized,
        F: Fn(Message) -> B,
    {
        Map {
            widget: self,
            mapper: f,
            _input: std::marker::PhantomData,
        }
    }

    /// 把此 [`Widget`] 装箱，转换为泛型的 [`Element`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Boxes this [`Widget`], turning it into a generic [`Element`].
    ///
    /// </details>
    fn _boxed<'a>(self) -> Element<'a, Message, Theme, Renderer>
    where
        Self: Sized + 'a,
    {
        Element::new(self)
    }
}

/// [`Widget`] 的元数据，与其泛型参数无关。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Metadata of a [`Widget`], independent of its generic parameters.
///
/// </details>
pub trait Meta {
    /// 返回 [`Widget`] 是否为 [`Void`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns whether the [`Widget`] is [`Void`].
    ///
    /// </details>
    fn is_void(&self) -> bool {
        false
    }
}

impl<T> Meta for &mut T
where
    T: Meta,
{
    fn is_void(&self) -> bool {
        T::is_void(self)
    }
}

impl<T, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for &mut T
where
    T: Widget<Message, Theme, Renderer>,
{
    fn size(&self) -> Size<Length> {
        T::size(self)
    }

    fn tag(&self) -> tree::Tag {
        T::tag(self)
    }

    fn state(&self) -> tree::State {
        T::state(self)
    }

    fn diff(&mut self, tree: &mut Tree) {
        T::diff(self, tree);
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        T::layout(self, tree, renderer, limits);
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        T::draw(self, tree, renderer, theme, style, layout, cursor, viewport);
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        T::operate(self, tree, layout, viewport, renderer, operation);
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
        T::update(self, tree, event, layout, cursor, renderer, shell, viewport);
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        T::mouse_interaction(self, tree, layout, cursor, viewport, renderer)
    }

    fn overlay<'a>(
        &'a mut self,
        tree: &'a mut Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'a, Message, Theme, Renderer>> {
        T::overlay(self, tree, layout, renderer, viewport, translation, window)
    }
}

/// 零大小的 [`Widget`]，它什么都不做，并且会被容器过滤掉。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A zero-sized [`Widget`] that does nothing and will be filtered out by containers.
///
/// </details>
pub struct Void;

impl Meta for Void {
    fn is_void(&self) -> bool {
        true
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Void {
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(&mut self, _tree: &mut Tree, _renderer: &Renderer, _limits: &layout::Limits) {}

    fn draw(
        &self,
        _tree: &Tree,
        _renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        _layout: Layout,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
    }
}

/// 对另一个部件产生的消息进行变换的 [`Widget`]。
///
/// 此部件由 [`Widget::map`](Widget::_map) 返回。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A [`Widget`] that transforms the produced message of another widget.
///
/// This widget is returned by [`Widget::map`](Widget::_map).
///
/// </details>
pub struct Map<W, F, A> {
    widget: W,
    mapper: F,
    _input: std::marker::PhantomData<A>,
}

impl<W, F, A> Meta for Map<W, F, A>
where
    W: Meta,
{
    fn is_void(&self) -> bool {
        self.widget.is_void()
    }
}

impl<W, A, B, F, Theme, Renderer> Widget<B, Theme, Renderer> for Map<W, F, A>
where
    W: Widget<A, Theme, Renderer>,
    B: 'static,
    F: Fn(A) -> B,
    Theme: 'static,
    Renderer: 'static,
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
        operation: &mut dyn Operation,
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
        shell: &mut Shell<'_, B>,
        viewport: &Rectangle,
    ) {
        let mut local_messages = shell::Bus::new();
        let mut local_shell = shell.local(&mut local_messages);

        self.widget.update(
            tree,
            event,
            layout,
            cursor,
            renderer,
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
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.widget
            .draw(tree, renderer, theme, style, layout, cursor, viewport);
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
    ) -> Vec<overlay::Element<'b, B, Theme, Renderer>> {
        let mapper = &self.mapper;

        self.widget
            .overlay(tree, layout, renderer, viewport, translation, window)
            .into_iter()
            .map(move |overlay| overlay.map(mapper))
            .collect()
    }
}

impl<T> Meta for Option<T>
where
    T: Meta,
{
    fn is_void(&self) -> bool {
        self.is_none()
    }
}

impl<T, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Option<T>
where
    T: Widget<Message, Theme, Renderer>,
{
    fn size(&self) -> Size<Length> {
        let Some(widget) = self else {
            return Size {
                width: Length::Shrink,
                height: Length::Shrink,
            };
        };

        widget.size()
    }

    fn tag(&self) -> tree::Tag {
        let Some(widget) = self else {
            return tree::Tag::stateless();
        };

        widget.tag()
    }

    fn state(&self) -> tree::State {
        let Some(widget) = self else {
            return tree::State::None;
        };

        widget.state()
    }

    fn diff(&mut self, tree: &mut Tree) {
        let Some(widget) = self else {
            tree.children.clear();
            return;
        };

        widget.diff(tree);
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let Some(widget) = self else {
            return;
        };

        widget.layout(tree, renderer, limits);
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let Some(widget) = self else {
            return;
        };

        widget.draw(tree, renderer, theme, style, layout, cursor, viewport);
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let Some(widget) = self else {
            return;
        };

        widget.operate(tree, layout, viewport, renderer, operation);
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
        let Some(widget) = self else {
            return;
        };

        widget.update(tree, event, layout, cursor, renderer, shell, viewport);
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let Some(widget) = self else {
            return mouse::Interaction::None;
        };

        widget.mouse_interaction(tree, layout, cursor, viewport, renderer)
    }

    fn overlay<'a>(
        &'a mut self,
        tree: &'a mut Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'a, Message, Theme, Renderer>> {
        let Some(widget) = self else {
            return Vec::new();
        };

        widget.overlay(tree, layout, renderer, viewport, translation, window)
    }
}
