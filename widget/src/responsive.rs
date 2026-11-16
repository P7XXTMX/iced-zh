use crate::core::layout::{self, Layout};
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::widget;
use crate::core::widget::Tree;
use crate::core::{Event, Length, Rectangle, Shell, Size, Vector, Widget};

/// 能够感知自身尺寸的部件。
///
/// [`Responsive`] 部件总是会尝试填满其父级的
/// 全部可用空间。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A widget that is aware of its dimensions.
///
/// A [`Responsive`] widget will always try to fill all the available space of
/// its parent.
///
/// </details>
pub struct Responsive<'a, W> {
    view: Box<dyn Fn(Size) -> W + 'a>,
    width: Length,
    height: Length,
    content: Option<W>,
}

impl<'a, W> Responsive<'a, W> {
    /// 使用一个产生其内容的闭包创建一个新的
    /// [`Responsive`] 部件。
    ///
    /// `view` 闭包会在布局期间收到 [`Responsive`] 的
    /// 最大可用空间。可以利用这个 [`Size`]
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
    pub fn new(view: impl Fn(Size) -> W + 'a) -> Self {
        Self {
            view: Box::new(view),
            width: Length::Fill,
            height: Length::Fill,
            content: None,
        }
    }

    /// 设置 [`Responsive`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Responsive`].
    ///
    /// </details>
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// 设置 [`Responsive`] 的高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the height of the [`Responsive`].
    ///
    /// </details>
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

impl<W> widget::Meta for Responsive<'_, W> {}

impl<W, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Responsive<'_, W>
where
    W: Widget<Message, Theme, Renderer>,
{
    fn diff(&mut self, _tree: &mut Tree) {
        // Diff is deferred to layout
        // 差异比较推迟到布局阶段进行
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let limits = limits.width(self.width).height(self.height);
        let size = limits.bounds();

        self.content = Some((self.view)(size));
        tree.diff_children(std::slice::from_mut(&mut self.content));

        self.content
            .layout(&mut tree.children[0], renderer, &limits.loose());

        tree.size = limits.resolve(self.width, self.height, tree.children[0].size);
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
        let (layout, tree) = layout.iter_mut(&mut tree.children).next().unwrap();

        self.content
            .update(tree, event, layout, cursor, renderer, shell, viewport);
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
        let (layout, tree) = layout.iter(&tree.children).next().unwrap();

        self.content
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
        let (layout, tree) = layout.iter(&tree.children).next().unwrap();

        self.content
            .mouse_interaction(tree, layout, cursor, viewport, renderer)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        let (layout, tree) = layout.iter_mut(&mut tree.children).next().unwrap();

        self.content
            .operate(tree, layout, viewport, renderer, operation);
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
        let (layout, tree) = layout.iter_mut(&mut tree.children).next().unwrap();

        self.content
            .overlay(tree, layout, renderer, viewport, translation, window)
    }
}
