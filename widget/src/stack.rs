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
use crate::core::widget::{Operation, Tree};
use crate::core::{
    Clipboard, Element, Event, Layout, Length, Rectangle, Shell, Size, Vector,
    Widget,
};

/// 一个把子级层层叠放显示的容器。
///
/// 第一个 [`Element`] 决定 [`Stack`] 的固有 [`Size`]，
/// 并作为基础图层显示。之后的每个 [`Element`] 都会
/// 渲染在它上面，各占一个图层。
///
/// 你可以用 [`push_under`](Self::push_under) 把一个 [`Element`] 放到
/// 当前 [`Stack`] 的下面，而不影响它的固有 [`Size`]。
///
/// 请记住，过多的图层通常会带来糟糕的用户体验，
/// 还会引入一定的渲染开销。请节制地使用这个部件！
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A container that displays children on top of each other.
///
/// The first [`Element`] dictates the intrinsic [`Size`] of a [`Stack`] and
/// will be displayed as the base layer. Every consecutive [`Element`] will be
/// rendered on top; on its own layer.
///
/// You can use [`push_under`](Self::push_under) to push an [`Element`] under
/// the current [`Stack`] without affecting its intrinsic [`Size`].
///
/// Keep in mind that too much layering will normally produce bad UX as well as
/// introduce certain rendering overhead. Use this widget sparingly!
///
/// </details>
pub struct Stack<'a, Message, Theme = crate::Theme, Renderer = crate::Renderer>
{
    width: Length,
    height: Length,
    children: Vec<Element<'a, Message, Theme, Renderer>>,
    clip: bool,
    base_layer: usize,
}

impl<'a, Message, Theme, Renderer> Stack<'a, Message, Theme, Renderer>
where
    Renderer: crate::core::Renderer,
{
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

    /// 使用给定的元素创建一个 [`Stack`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Stack`] with the given elements.
    ///
    /// </details>
    pub fn with_children(
        children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        let iterator = children.into_iter();

        Self::with_capacity(iterator.size_hint().0).extend(iterator)
    }

    /// 从一个已经分配好的 [`Vec`] 创建 [`Stack`]。
    ///
    /// 请记住，[`Stack`] 不会检查这个 [`Vec`]，也就是说
    /// 它不会自动适配其内容的尺寸策略。
    ///
    /// 如果有任何子级使用 [`Length::Fill`] 策略，你需要相应地
    /// 调用 [`Stack::width`] 或 [`Stack::height`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Stack`] from an already allocated [`Vec`].
    ///
    /// Keep in mind that the [`Stack`] will not inspect the [`Vec`], which means
    /// it won't automatically adapt to the sizing strategy of its contents.
    ///
    /// If any of the children have a [`Length::Fill`] strategy, you will need to
    /// call [`Stack::width`] or [`Stack::height`] accordingly.
    ///
    /// </details>
    pub fn from_vec(
        children: Vec<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            width: Length::Shrink,
            height: Length::Shrink,
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

    /// 在 [`Stack`] 的上面添加一个元素。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adds an element on top of the [`Stack`].
    ///
    /// </details>
    pub fn push(
        mut self,
        child: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        let child = child.into();
        let child_size = child.as_widget().size_hint();

        if !child_size.is_void() {
            if self.children.is_empty() {
                self.width = self.width.enclose(child_size.width);
                self.height = self.height.enclose(child_size.height);
            }

            self.children.push(child);
        }

        self
    }

    /// 在 [`Stack`] 的下面添加一个元素。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adds an element under the [`Stack`].
    ///
    /// </details>
    pub fn push_under(
        mut self,
        child: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        self.children.insert(0, child.into());
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
    pub fn extend(
        self,
        children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
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

impl<Message, Renderer> Default for Stack<'_, Message, Renderer>
where
    Renderer: crate::core::Renderer,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Stack<'a, Message, Theme, Renderer>
where
    Renderer: crate::core::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);

        if self.children.len() <= self.base_layer {
            return layout::Node::new(limits.resolve(
                self.width,
                self.height,
                Size::ZERO,
            ));
        }

        let base = self.children[self.base_layer].as_widget_mut().layout(
            &mut tree.children[self.base_layer],
            renderer,
            &limits,
        );

        let size = limits.resolve(self.width, self.height, base.size());
        let limits = layout::Limits::new(Size::ZERO, size);

        let (under, above) = self.children.split_at_mut(self.base_layer);
        let (tree_under, tree_above) =
            tree.children.split_at_mut(self.base_layer);

        let nodes = under
            .iter_mut()
            .zip(tree_under)
            .map(|(layer, tree)| {
                layer.as_widget_mut().layout(tree, renderer, &limits)
            })
            .chain(std::iter::once(base))
            .chain(above[1..].iter_mut().zip(&mut tree_above[1..]).map(
                |(layer, tree)| {
                    layer.as_widget_mut().layout(tree, renderer, &limits)
                },
            ))
            .collect();

        layout::Node::with_children(size, nodes)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            self.children
                .iter_mut()
                .zip(&mut tree.children)
                .zip(layout.children())
                .for_each(|((child, state), layout)| {
                    child
                        .as_widget_mut()
                        .operate(state, layout, renderer, operation);
                });
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        mut cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if self.children.is_empty() {
            return;
        }

        let is_over = cursor.is_over(layout.bounds());
        let end = self.children.len() - 1;

        for (i, ((child, tree), layout)) in self
            .children
            .iter_mut()
            .rev()
            .zip(tree.children.iter_mut().rev())
            .zip(layout.children().rev())
            .enumerate()
        {
            child.as_widget_mut().update(
                tree, event, layout, cursor, renderer, clipboard, shell,
                viewport,
            );

            if shell.is_event_captured() {
                return;
            }

            if i < end && is_over && !cursor.is_levitating() {
                let interaction = child.as_widget().mouse_interaction(
                    tree, layout, cursor, viewport, renderer,
                );

                if interaction != mouse::Interaction::None {
                    cursor = cursor.levitate();
                }
            }
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.children
            .iter()
            .rev()
            .zip(tree.children.iter().rev())
            .zip(layout.children().rev())
            .map(|((child, tree), layout)| {
                child
                    .as_widget()
                    .mouse_interaction(tree, layout, cursor, viewport, renderer)
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
        layout: Layout<'_>,
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
                    .zip(tree.children.iter().rev())
                    .zip(layout.children().rev())
                    .position(|((layer, tree), layout)| {
                        let interaction = layer.as_widget().mouse_interaction(
                            tree, layout, cursor, viewport, renderer,
                        );

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
                .zip(&tree.children)
                .zip(layout.children())
                .enumerate();

            let layers = layers.by_ref();

            let mut draw_layer =
                |i,
                 layer: &Element<'a, Message, Theme, Renderer>,
                 tree,
                 layout,
                 cursor| {
                    if i > 0 {
                        renderer.with_layer(*viewport, |renderer| {
                            layer.as_widget().draw(
                                tree, renderer, theme, style, layout, cursor,
                                viewport,
                            );
                        });
                    } else {
                        layer.as_widget().draw(
                            tree, renderer, theme, style, layout, cursor,
                            viewport,
                        );
                    }
                };

            for (i, ((layer, tree), layout)) in layers.take(layers_under) {
                draw_layer(i, layer, tree, layout, mouse::Cursor::Unavailable);
            }

            for (i, ((layer, tree), layout)) in layers {
                draw_layer(i, layer, tree, layout, cursor);
            }
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        overlay::from_children(
            &mut self.children,
            tree,
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message, Theme, Renderer> From<Stack<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: crate::core::Renderer + 'a,
{
    fn from(stack: Stack<'a, Message, Theme, Renderer>) -> Self {
        Self::new(stack)
    }
}
