//! 在垂直方向上分布内容。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Distribute content vertically.
//!
//! </details>
use crate::core::alignment::{self, Alignment};
use crate::core::layout;
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::widget::{Meta, Operation, Tree, tree};
use crate::core::{Event, Layout, Length, Padding, Pixels, Rectangle, Shell, Size, Vector, Widget};

/// 在垂直方向上分布其内容的容器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A container that distributes its contents vertically.
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
pub struct Column<W> {
    spacing: f32,
    padding: Padding,
    width: Length,
    height: Length,
    align: Alignment,
    clip: bool,
    children: Vec<W>,
}

impl<W> Column<W> {
    /// 创建一个空的 [`Column`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates an empty [`Column`].
    ///
    /// </details>
    pub fn new() -> Self {
        Self::from_vec(Vec::new())
    }

    /// 使用给定的容量创建一个 [`Column`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Column`] with the given capacity.
    ///
    /// </details>
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_vec(Vec::with_capacity(capacity))
    }

    /// 使用给定的部件创建一个 [`Column`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Column`] with the given widgets.
    ///
    /// </details>
    pub fn with_children(children: impl IntoIterator<Item = W>) -> Self
    where
        W: Meta,
    {
        let iterator = children.into_iter();

        Self::with_capacity(iterator.size_hint().0).extend(iterator)
    }

    /// 从已分配的 [`Vec`] 创建一个 [`Column`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Column`] from an already allocated [`Vec`].
    ///
    /// </details>
    pub fn from_vec(children: Vec<W>) -> Self {
        Self {
            spacing: 0.0,
            padding: Padding::ZERO,
            width: Length::Fit,
            height: Length::Fit,
            align: Alignment::Start,
            clip: false,
            children,
        }
    }

    /// 设置元素*之间*的垂直间距。
    ///
    /// iced 中不存在针对单个元素的自定义外边距。你应该改用
    /// 此方法！虽然灵活性差一些，但它有助于让元素之间的
    /// 间距保持一致。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the vertical spacing _between_ elements.
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

    /// 设置 [`Column`] 的 [`Padding`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Padding`] of the [`Column`].
    ///
    /// </details>
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// 设置 [`Column`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Column`].
    ///
    /// </details>
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// 设置 [`Column`] 的高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the height of the [`Column`].
    ///
    /// </details>
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// 设置 [`Column`] 内容的水平对齐方式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the horizontal alignment of the contents of the [`Column`] .
    ///
    /// </details>
    pub fn align_x(mut self, align: impl Into<alignment::Horizontal>) -> Self {
        self.align = Alignment::from(align.into());
        self
    }

    /// 设置 [`Column`] 的内容在溢出时是否应该
    /// 被裁剪。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets whether the contents of the [`Column`] should be clipped on
    /// overflow.
    ///
    /// </details>
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// 向 [`Column`] 添加一个 [`Widget`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adds a [`Widget`] to the [`Column`].
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

    /// 用给定的子元素扩展 [`Column`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Extends the [`Column`] with the given children.
    ///
    /// </details>
    pub fn extend(self, children: impl IntoIterator<Item = W>) -> Self
    where
        W: Meta,
    {
        children.into_iter().fold(self, Self::push)
    }

    /// 把 [`Column`] 转换为 [`Wrapping`] 列。
    ///
    /// 换行后的每一列都会保留 [`Column`] 原本的对齐方式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Turns the [`Column`] into a [`Wrapping`] column.
    ///
    /// The original alignment of the [`Column`] is preserved per column wrapped.
    ///
    /// </details>
    pub fn wrap(self) -> Wrapping<W> {
        Wrapping {
            column: self,
            horizontal_spacing: None,
            align_y: alignment::Vertical::Top,
        }
    }
}

impl<W> Default for Column<W> {
    fn default() -> Self {
        Self::new()
    }
}

impl<W> FromIterator<W> for Column<W>
where
    W: Meta,
{
    fn from_iter<T: IntoIterator<Item = W>>(iter: T) -> Self {
        Self::with_children(iter)
    }
}

impl<W> Meta for Column<W> {}

impl<W, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Column<W>
where
    W: Widget<Message, Theme, Renderer>,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<layout::flex::Cache>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(layout::flex::Cache::default())
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(&mut self.children);

        if self.width.is_fit() || self.height.is_fit() {
            for child in &self.children {
                let size = child.size();

                self.width = self.width.cross(size.width);
                self.height = self.height.stack(size.height);
            }
        }
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let cache = tree.state.downcast_mut::<layout::flex::Cache>();

        tree.size = layout::flex::resolve(
            layout::flex::Axis::Vertical,
            renderer,
            limits,
            self.width,
            self.height,
            self.padding,
            self.spacing,
            self.align,
            &mut tree.children,
            &mut self.children,
            cache,
        );
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
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        for (child, (layout, tree)) in self
            .children
            .iter_mut()
            .zip(layout.iter_mut(&mut tree.children))
        {
            child.update(tree, event, layout, cursor, renderer, shell, viewport);
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
            .zip(layout.iter(&tree.children))
            .map(|(child, (layout, tree))| {
                child.mouse_interaction(tree, layout, cursor, viewport, renderer)
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

            for (child, (layout, tree)) in self
                .children
                .iter()
                .zip(layout.iter(&tree.children))
                .filter(|(_, (layout, _))| layout.bounds().intersects(viewport))
            {
                child.draw(tree, renderer, theme, style, layout, cursor, viewport);
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

/// 会对内容进行换行的 [`Column`]。
///
/// 先创建一个 [`Column`]，然后调用 [`Column::wrap`]，
/// 即可得到一个会对内容进行换行的 [`Column`]。
///
/// 换行后的每一列都会保留 [`Column`] 原本的对齐方式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A [`Column`] that wraps its contents.
///
/// Create a [`Column`] first, and then call [`Column::wrap`] to
/// obtain a [`Column`] that wraps its contents.
///
/// The original alignment of the [`Column`] is preserved per column wrapped.
///
/// </details>
#[allow(missing_debug_implementations)]
pub struct Wrapping<W> {
    column: Column<W>,
    horizontal_spacing: Option<f32>,
    align_y: alignment::Vertical,
}

impl<W> Wrapping<W> {
    /// 设置各列*之间*的水平间距。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the horizontal spacing _between_ columns.
    ///
    /// </details>
    pub fn horizontal_spacing(mut self, amount: impl Into<Pixels>) -> Self {
        self.horizontal_spacing = Some(amount.into().0);
        self
    }

    /// 设置换行 [`Column`] 的垂直对齐方式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the vertical alignment of the wrapping [`Column`].
    ///
    /// </details>
    pub fn align_y(mut self, align_y: impl Into<alignment::Vertical>) -> Self {
        self.align_y = align_y.into();
        self
    }
}

impl<W> Meta for Wrapping<W> {}

impl<W, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Wrapping<W>
where
    W: Widget<Message, Theme, Renderer>,
{
    fn diff(&mut self, tree: &mut Tree) {
        self.column.diff(tree);
    }

    fn size(&self) -> Size<Length> {
        self.column.size()
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let limits = limits
            .width(self.column.width)
            .height(self.column.height)
            .shrink(self.column.padding);

        let child_limits = limits.loose();
        let spacing = self.column.spacing;
        let horizontal_spacing = self.horizontal_spacing.unwrap_or(spacing);
        let max_height = limits.bounds().height;

        let mut intrinsic_size = Size::ZERO;
        let mut column_start = 0;
        let mut column_width = 0.0;
        let mut x = 0.0;
        let mut y = 0.0;

        let align_factor = match self.column.align {
            Alignment::Start => 0.0,
            Alignment::Center => 2.0,
            Alignment::End => 1.0,
        };

        let align_x =
            |column_start: std::ops::Range<usize>, column_width: f32, children: &mut [Tree]| {
                if align_factor != 0.0 {
                    for child in &mut children[column_start] {
                        let width = child.size.width;

                        child.translation.x += (column_width - width) / align_factor;
                    }
                }
            };

        for (i, child) in self.column.children.iter_mut().enumerate() {
            child.layout(&mut tree.children[i], renderer, &child_limits);

            let child_size = tree.children[i].size;

            if y != 0.0 && y + child_size.height > max_height {
                intrinsic_size.height = intrinsic_size.height.max(y - spacing);

                align_x(column_start..i, column_width, &mut tree.children);

                x += column_width + horizontal_spacing;
                y = 0.0;
                column_start = i;
                column_width = 0.0;
            }

            column_width = column_width.max(child_size.width);

            tree.children[i].translation =
                Vector::new(x + self.column.padding.left, y + self.column.padding.top);

            y += child_size.height + spacing;
        }

        if y != 0.0 {
            intrinsic_size.height = intrinsic_size.height.max(y - spacing);
        }

        intrinsic_size.width = x + column_width;
        align_x(
            column_start..tree.children.len(),
            column_width,
            &mut tree.children,
        );

        let align_factor = match self.align_y {
            alignment::Vertical::Top => 0.0,
            alignment::Vertical::Center => 2.0,
            alignment::Vertical::Bottom => 1.0,
        };

        if align_factor != 0.0 {
            let total_height = intrinsic_size.height;

            let mut column_start = 0;

            for i in 0..tree.children.len() {
                let column_height = tree.children[i].translation.y + tree.children[i].size.height;

                let next_y = tree
                    .children
                    .get(i + 1)
                    .map(|child| child.translation.y)
                    .unwrap_or_default();

                if next_y == 0.0 {
                    let translation =
                        Vector::new(0.0, (total_height - column_height) / align_factor);

                    for child in &mut tree.children[column_start..=i] {
                        child.translation += translation;
                    }

                    column_start = i + 1;
                }
            }
        }

        let size = limits.resolve(self.column.width, self.column.height, intrinsic_size);

        tree.size = size.expand(self.column.padding);
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        self.column
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
        self.column
            .update(tree, event, layout, cursor, renderer, shell, viewport);
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.column
            .mouse_interaction(tree, layout, cursor, viewport, renderer)
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
        self.column
            .draw(tree, renderer, theme, style, layout, cursor, viewport);
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
        self.column
            .overlay(tree, layout, renderer, viewport, translation, window)
    }
}
