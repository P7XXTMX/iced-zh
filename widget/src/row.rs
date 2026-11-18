//! 水平分布内容。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Distribute content horizontally.
//!
//! </details>
use crate::core::alignment::{self, Alignment};
use crate::core::layout::{self, Layout};
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::widget::{Operation, Tree};
use crate::core::{
    Clipboard, Element, Event, Length, Padding, Pixels, Rectangle, Shell, Size,
    Vector, Widget,
};

/// 水平分布其内容的容器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A container that distributes its contents horizontally.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } }
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// use iced::widget::{button, row};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     row![
///         "I am to the left!",
///         button("I am in the middle!"),
///         "I am to the right!",
///     ].into()
/// }
/// ```
pub struct Row<'a, Message, Theme = crate::Theme, Renderer = crate::Renderer> {
    spacing: f32,
    padding: Padding,
    width: Length,
    height: Length,
    align: Alignment,
    clip: bool,
    children: Vec<Element<'a, Message, Theme, Renderer>>,
}

impl<'a, Message, Theme, Renderer> Row<'a, Message, Theme, Renderer>
where
    Renderer: crate::core::Renderer,
{
    /// 创建一个空的 [`Row`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates an empty [`Row`].
    ///
    /// </details>
    pub fn new() -> Self {
        Self::from_vec(Vec::new())
    }

    /// 使用给定的容量创建一个 [`Row`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Row`] with the given capacity.
    ///
    /// </details>
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_vec(Vec::with_capacity(capacity))
    }

    /// 使用给定的元素创建一个 [`Row`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Row`] with the given elements.
    ///
    /// </details>
    pub fn with_children(
        children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        let iterator = children.into_iter();

        Self::with_capacity(iterator.size_hint().0).extend(iterator)
    }

    /// 根据一个已分配的 [`Vec`] 创建一个 [`Row`]。
    ///
    /// 请注意，[`Row`] 不会检查这个 [`Vec`]，这意味着
    /// 它不会自动适应其内容的尺寸策略。
    ///
    /// 如果任何子元素使用 [`Length::Fill`] 策略，你需要
    /// 相应地调用 [`Row::width`] 或 [`Row::height`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Row`] from an already allocated [`Vec`].
    ///
    /// Keep in mind that the [`Row`] will not inspect the [`Vec`], which means
    /// it won't automatically adapt to the sizing strategy of its contents.
    ///
    /// If any of the children have a [`Length::Fill`] strategy, you will need to
    /// call [`Row::width`] or [`Row::height`] accordingly.
    ///
    /// </details>
    pub fn from_vec(
        children: Vec<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            spacing: 0.0,
            padding: Padding::ZERO,
            width: Length::Shrink,
            height: Length::Shrink,
            align: Alignment::Start,
            clip: false,
            children,
        }
    }

    /// 设置元素 *之间* 的水平间距。
    ///
    /// iced 中不存在针对单个元素的自定义外边距。你应该改用
    /// 这个方法！它虽然不那么灵活，但有助于让元素之间的间距
    /// 保持一致。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the horizontal spacing _between_ elements.
    ///
    /// Custom margins per element do not exist in iced. You should use this
    /// method instead! While less flexible, it helps you keep spacing between
    /// elements consistent.
    ///
    /// </details>
    pub fn spacing(mut self, amount: impl Into<Pixels>) -> Self {
        self.spacing = amount.into().0;
        self
    }

    /// 设置 [`Row`] 的 [`Padding`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Padding`] of the [`Row`].
    ///
    /// </details>
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// 设置 [`Row`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Row`].
    ///
    /// </details>
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// 设置 [`Row`] 的高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the height of the [`Row`].
    ///
    /// </details>
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// 设置 [`Row`] 内容的垂直对齐方式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the vertical alignment of the contents of the [`Row`] .
    ///
    /// </details>
    pub fn align_y(mut self, align: impl Into<alignment::Vertical>) -> Self {
        self.align = Alignment::from(align.into());
        self
    }

    /// 设置 [`Row`] 的内容在溢出时是否应该
    /// 被裁剪。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets whether the contents of the [`Row`] should be clipped on
    /// overflow.
    ///
    /// </details>
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// 向 [`Row`] 添加一个 [`Element`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adds an [`Element`] to the [`Row`].
    ///
    /// </details>
    pub fn push(
        mut self,
        child: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        let child = child.into();
        let child_size = child.as_widget().size_hint();

        if !child_size.is_void() {
            self.width = self.width.enclose(child_size.width);
            self.height = self.height.enclose(child_size.height);
            self.children.push(child);
        }

        self
    }

    /// 用给定的子元素扩展 [`Row`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Extends the [`Row`] with the given children.
    ///
    /// </details>
    pub fn extend(
        self,
        children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// 把 [`Row`] 变成一个 [`Wrapping`] 行。
    ///
    /// [`Row`] 原有的对齐方式会在换行后的每一行中保留。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Turns the [`Row`] into a [`Wrapping`] row.
    ///
    /// The original alignment of the [`Row`] is preserved per row wrapped.
    ///
    /// </details>
    pub fn wrap(self) -> Wrapping<'a, Message, Theme, Renderer> {
        Wrapping {
            row: self,
            vertical_spacing: None,
            align_x: alignment::Horizontal::Left,
        }
    }
}

impl<Message, Renderer> Default for Row<'_, Message, Renderer>
where
    Renderer: crate::core::Renderer,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message, Theme, Renderer: crate::core::Renderer>
    FromIterator<Element<'a, Message, Theme, Renderer>>
    for Row<'a, Message, Theme, Renderer>
{
    fn from_iter<
        T: IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
    >(
        iter: T,
    ) -> Self {
        Self::with_children(iter)
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Row<'_, Message, Theme, Renderer>
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
        layout::flex::resolve(
            layout::flex::Axis::Horizontal,
            renderer,
            limits,
            self.width,
            self.height,
            self.padding,
            self.spacing,
            self.align,
            &mut self.children,
            &mut tree.children,
        )
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
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        for ((child, tree), layout) in self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
        {
            child.as_widget_mut().update(
                tree, event, layout, cursor, renderer, clipboard, shell,
                viewport,
            );
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
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((child, tree), layout)| {
                child
                    .as_widget()
                    .mouse_interaction(tree, layout, cursor, viewport, renderer)
            })
            .max()
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

            for ((child, tree), layout) in self
                .children
                .iter()
                .zip(&tree.children)
                .zip(layout.children())
                .filter(|(_, layout)| layout.bounds().intersects(viewport))
            {
                child.as_widget().draw(
                    tree, renderer, theme, style, layout, cursor, viewport,
                );
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

impl<'a, Message, Theme, Renderer> From<Row<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: crate::core::Renderer + 'a,
{
    fn from(row: Row<'a, Message, Theme, Renderer>) -> Self {
        Self::new(row)
    }
}

/// 会对其内容进行换行的 [`Row`]。
///
/// 先创建一个 [`Row`]，然后调用 [`Row::wrap`]，
/// 即可得到一个会对其内容进行换行的 [`Row`]。
///
/// [`Row`] 原有的对齐方式会在换行后的每一行中保留。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A [`Row`] that wraps its contents.
///
/// Create a [`Row`] first, and then call [`Row::wrap`] to
/// obtain a [`Row`] that wraps its contents.
///
/// The original alignment of the [`Row`] is preserved per row wrapped.
///
/// </details>
pub struct Wrapping<
    'a,
    Message,
    Theme = crate::Theme,
    Renderer = crate::Renderer,
> {
    row: Row<'a, Message, Theme, Renderer>,
    vertical_spacing: Option<f32>,
    align_x: alignment::Horizontal,
}

impl<Message, Theme, Renderer> Wrapping<'_, Message, Theme, Renderer> {
    /// 设置各行 *之间* 的垂直间距。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the vertical spacing _between_ lines.
    ///
    /// </details>
    pub fn vertical_spacing(mut self, amount: impl Into<Pixels>) -> Self {
        self.vertical_spacing = Some(amount.into().0);
        self
    }

    /// 设置换行 [`Row`] 的水平对齐方式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the horizontal alignment of the wrapping [`Row`].
    ///
    /// </details>
    pub fn align_x(
        mut self,
        align_x: impl Into<alignment::Horizontal>,
    ) -> Self {
        self.align_x = align_x.into();
        self
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Wrapping<'_, Message, Theme, Renderer>
where
    Renderer: crate::core::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        self.row.children()
    }

    fn diff(&self, tree: &mut Tree) {
        self.row.diff(tree);
    }

    fn size(&self) -> Size<Length> {
        self.row.size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits
            .width(self.row.width)
            .height(self.row.height)
            .shrink(self.row.padding);

        let child_limits = limits.loose();
        let spacing = self.row.spacing;
        let vertical_spacing = self.vertical_spacing.unwrap_or(spacing);
        let max_width = limits.max().width;

        let mut children: Vec<layout::Node> = Vec::new();
        let mut intrinsic_size = Size::ZERO;
        let mut row_start = 0;
        let mut row_height = 0.0;
        let mut x = 0.0;
        let mut y = 0.0;

        let align_factor = match self.row.align {
            Alignment::Start => 0.0,
            Alignment::Center => 2.0,
            Alignment::End => 1.0,
        };

        let align_y = |row_start: std::ops::Range<usize>,
                       row_height: f32,
                       children: &mut Vec<layout::Node>| {
            if align_factor != 0.0 {
                for node in &mut children[row_start] {
                    let height = node.size().height;

                    node.translate_mut(Vector::new(
                        0.0,
                        (row_height - height) / align_factor,
                    ));
                }
            }
        };

        for (i, child) in self.row.children.iter_mut().enumerate() {
            let node = child.as_widget_mut().layout(
                &mut tree.children[i],
                renderer,
                &child_limits,
            );

            let child_size = node.size();

            if x != 0.0 && x + child_size.width > max_width {
                intrinsic_size.width = intrinsic_size.width.max(x - spacing);

                align_y(row_start..i, row_height, &mut children);

                y += row_height + vertical_spacing;
                x = 0.0;
                row_start = i;
                row_height = 0.0;
            }

            row_height = row_height.max(child_size.height);

            children.push(node.move_to((
                x + self.row.padding.left,
                y + self.row.padding.top,
            )));

            x += child_size.width + spacing;
        }

        if x != 0.0 {
            intrinsic_size.width = intrinsic_size.width.max(x - spacing);
        }

        intrinsic_size.height = y + row_height;
        align_y(row_start..children.len(), row_height, &mut children);

        let align_factor = match self.align_x {
            alignment::Horizontal::Left => 0.0,
            alignment::Horizontal::Center => 2.0,
            alignment::Horizontal::Right => 1.0,
        };

        if align_factor != 0.0 {
            let total_width = intrinsic_size.width;

            let mut row_start = 0;

            for i in 0..children.len() {
                let bounds = children[i].bounds();
                let row_width = bounds.x + bounds.width;

                let next_x = children
                    .get(i + 1)
                    .map(|node| node.bounds().x)
                    .unwrap_or_default();

                if next_x == 0.0 {
                    let translation = Vector::new(
                        (total_width - row_width) / align_factor,
                        0.0,
                    );

                    for node in &mut children[row_start..=i] {
                        node.translate_mut(translation);
                    }

                    row_start = i + 1;
                }
            }
        }

        let size =
            limits.resolve(self.row.width, self.row.height, intrinsic_size);

        layout::Node::with_children(size.expand(self.row.padding), children)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        self.row.operate(tree, layout, renderer, operation);
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
        self.row.update(
            tree, event, layout, cursor, renderer, clipboard, shell, viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.row
            .mouse_interaction(tree, layout, cursor, viewport, renderer)
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
        self.row
            .draw(tree, renderer, theme, style, layout, cursor, viewport);
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.row
            .overlay(tree, layout, renderer, viewport, translation)
    }
}

impl<'a, Message, Theme, Renderer> From<Wrapping<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: crate::core::Renderer + 'a,
{
    fn from(row: Wrapping<'a, Message, Theme, Renderer>) -> Self {
        Self::new(row)
    }
}
