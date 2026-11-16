//! 在其他内容之上显示内容。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Display content on top of other content.
//!
//! </details>
use crate::core::layout;
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::widget::{Meta, Operation, Tree};
use crate::core::{Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget};

/// 把子元素层叠显示的容器。
///
/// 第一个 [`Widget`] 决定 [`Stack`] 的固有 [`Size`]，并
/// 作为基础图层显示。之后的每个 [`Widget`] 都会
/// 渲染在上方；各自位于独立的图层。
///
/// 你可以使用 [`push_under`](Self::push_under) 把一个 [`Widget`] 压入
/// 当前 [`Stack`] 的下方，而不影响它的固有 [`Size`]。
///
/// 请记住，过多的层叠通常会带来糟糕的用户体验，
/// 还会引入一定的渲染开销。请节制地使用这个部件！
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A container that displays children on top of each other.
///
/// The first [`Widget`] dictates the intrinsic [`Size`] of a [`Stack`] and
/// will be displayed as the base layer. Every consecutive [`Widget`] will be
/// rendered on top; on its own layer.
///
/// You can use [`push_under`](Self::push_under) to push a [`Widget`] under
/// the current [`Stack`] without affecting its intrinsic [`Size`].
///
/// Keep in mind that too much layering will normally produce bad UX as well as
/// introduce certain rendering overhead. Use this widget sparingly!
///
/// </details>
pub struct Stack<W> {
    width: Length,
    height: Length,
    children: Vec<W>,
    clip: bool,
    base_layer: usize,
}

impl<W> Stack<W> {
    /// 创建一个空的 [`Stack`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates an empty [`Stack`].
    ///
    /// </details>
    pub fn new() -> Self {
        Self::from_vec(Vec::new())
    }

    /// 使用给定的容量创建一个 [`Stack`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Stack`] with the given capacity.
    ///
    /// </details>
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_vec(Vec::with_capacity(capacity))
    }

    /// 使用给定的部件创建一个 [`Stack`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Stack`] with the given widgets.
    ///
    /// </details>
    pub fn with_children(children: impl IntoIterator<Item = W>) -> Self
    where
        W: Meta,
    {
        let iterator = children.into_iter();

        Self::with_capacity(iterator.size_hint().0).extend(iterator)
    }

    /// 从一个已经分配好的 [`Vec`] 创建一个 [`Stack`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Stack`] from an already allocated [`Vec`].
    ///
    /// </details>
    pub fn from_vec(children: Vec<W>) -> Self {
        Self {
            width: Length::Fit,
            height: Length::Fit,
            children,
            clip: false,
            base_layer: 0,
        }
    }

    /// 设置 [`Stack`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Stack`].
    ///
    /// </details>
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// 设置 [`Stack`] 的高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the height of the [`Stack`].
    ///
    /// </details>
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// 在 [`Stack`] 的顶部添加一个部件。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adds a widget on top of the [`Stack`].
    ///
    /// </details>
    pub fn push(mut self, child: W) -> Self
    where
        W: Meta,
    {
        if !child.is_void() {
            self.children.push(child);
        }

        self
    }

    /// 在 [`Stack`] 的下方添加一个部件。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adds a widget under the [`Stack`].
    ///
    /// </details>
    pub fn push_under(mut self, child: W) -> Self {
        self.children.insert(0, child);
        self.base_layer += 1;
        self
    }

    /// 使用给定的子元素扩展 [`Stack`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Extends the [`Stack`] with the given children.
    ///
    /// </details>
    pub fn extend(self, children: impl IntoIterator<Item = W>) -> Self
    where
        W: Meta,
    {
        children.into_iter().fold(self, Self::push)
    }

    /// 设置 [`Stack`] 是否应该裁剪溢出的内容。
    ///
    /// 它在呈现时有轻微的性能开销。
    ///
    /// 默认设置为 `false`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets whether the [`Stack`] should clip overflowing content.
    ///
    /// It has a slight performance overhead during presentation.
    ///
    /// By default, it is set to `false`.
    ///
    /// </details>
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }
}

impl<W> Default for Stack<W> {
    fn default() -> Self {
        Self::new()
    }
}

impl<W> Meta for Stack<W> {}

impl<W, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Stack<W>
where
    Renderer: crate::core::Renderer,
    W: Widget<Message, Theme, Renderer>,
{
    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(&mut self.children);

        if let Some(base) = self.children.get(self.base_layer) {
            let size = base.size();

            self.width = self.width.cross(size.width);
            self.height = self.height.cross(size.height);
        }
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let limits = limits.width(self.width).height(self.height);

        if self.children.len() <= self.base_layer {
            tree.size = limits.resolve(self.width, self.height, Size::ZERO);
            return;
        }

        self.children[self.base_layer].layout(
            &mut tree.children[self.base_layer],
            renderer,
            &limits,
        );

        let size = limits.resolve(self.width, self.height, tree.children[self.base_layer].size);
        let limits = layout::Limits::new(Size::ZERO, size);

        let (under, above) = self.children.split_at_mut(self.base_layer);
        let (tree_under, tree_above) = tree.children.split_at_mut(self.base_layer);

        under.iter_mut().zip(tree_under).for_each(|(layer, tree)| {
            layer.layout(tree, renderer, &limits);
        });

        above[1..]
            .iter_mut()
            .zip(&mut tree_above[1..])
            .for_each(|(layer, tree)| layer.layout(tree, renderer, &limits));

        tree.size = size;
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds(), viewport);
        operation.traverse(&mut |operation| {
            self.children
                .iter_mut()
                .zip(layout.iter_mut(&mut tree.children))
                .for_each(|(child, (layout, state))| {
                    child.operate(state, layout, viewport, renderer, operation);
                });
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        mut cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if self.children.is_empty() {
            return;
        }

        let is_over = cursor.is_over(layout.bounds());
        let end = self.children.len() - 1;

        for (i, (child, (layout, tree))) in self
            .children
            .iter_mut()
            .rev()
            .zip(layout.iter_mut(&mut tree.children).rev())
            .enumerate()
        {
            child.update(tree, event, layout, cursor, renderer, shell, viewport);

            if shell.is_event_captured() {
                return;
            }

            if i < end && is_over && !cursor.is_levitating() {
                let interaction = child.mouse_interaction(tree, layout, cursor, viewport, renderer);

                if interaction != mouse::Interaction::None {
                    cursor = cursor.levitate();
                }
            }
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
        self.children
            .iter()
            .rev()
            .zip(layout.iter(&tree.children).rev())
            .map(|(child, (layout, tree))| {
                child.mouse_interaction(tree, layout, cursor, viewport, renderer)
            })
            .find(|&interaction| interaction != mouse::Interaction::None)
            .unwrap_or_default()
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
        if let Some(clipped_viewport) = layout.bounds().intersection(viewport) {
            let viewport = if self.clip {
                &clipped_viewport
            } else {
                viewport
            };

            let layers_under = if cursor.is_over(layout.bounds()) {
                self.children
                    .iter()
                    .rev()
                    .zip(layout.iter(&tree.children).rev())
                    .position(|(layer, (layout, tree))| {
                        let interaction =
                            layer.mouse_interaction(tree, layout, cursor, viewport, renderer);

                        interaction != mouse::Interaction::None
                    })
                    .map(|i| self.children.len() - i - 1)
                    .unwrap_or_default()
            } else {
                0
            };

            let mut layers = self
                .children
                .iter()
                .zip(layout.iter(&tree.children))
                .enumerate();

            let layers = layers.by_ref();

            let mut draw_layer = |i, layer: &W, tree, layout, cursor| {
                if i > 0 {
                    renderer.with_layer(*viewport, |renderer| {
                        layer.draw(tree, renderer, theme, style, layout, cursor, viewport);
                    });
                } else {
                    layer.draw(tree, renderer, theme, style, layout, cursor, viewport);
                }
            };

            for (i, (layer, (layout, tree))) in layers.take(layers_under) {
                draw_layer(i, layer, tree, layout, mouse::Cursor::Unavailable);
            }

            for (i, (layer, (layout, tree))) in layers {
                draw_layer(i, layer, tree, layout, cursor);
            }
        }
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
        overlay::from_children(
            &mut self.children,
            tree,
            layout,
            renderer,
            viewport,
            translation,
            window,
        )
    }
}
