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
use crate::core::widget::{Meta, Operation, Tree, tree};
use crate::core::{Event, Length, Padding, Pixels, Rectangle, Shell, Size, Vector, Widget};

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
pub struct Row<W> {
    spacing: f32,
    padding: Padding,
    width: Length,
    height: Length,
    align: Alignment,
    clip: bool,
    children: Vec<W>,
}

impl<W> Row<W> {
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

    /// 使用给定的部件创建一个 [`Row`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Row`] with the given widgets.
    ///
    /// </details>
    pub fn with_children(children: impl IntoIterator<Item = W>) -> Self
    where
        W: Meta,
    {
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

    /// 设置元素 _之间_ 的水平间距。
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

    /// 向 [`Row`] 添加一个 [`Widget`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adds a [`Widget`] to the [`Row`].
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

    /// 用给定的子元素扩展 [`Row`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Extends the [`Row`] with the given children.
    ///
    /// </details>
    pub fn extend(self, children: impl IntoIterator<Item = W>) -> Self
    where
        W: Meta,
    {
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
    pub fn wrap(self) -> Wrapping<W> {
        Wrapping {
            row: self,
            vertical_spacing: None,
            align_x: alignment::Horizontal::Left,
        }
    }
}

impl<W> Default for Row<W> {
    fn default() -> Self {
        Self::new()
    }
}

impl<W> FromIterator<W> for Row<W>
where
    W: Meta,
{
    fn from_iter<T: IntoIterator<Item = W>>(iter: T) -> Self {
        Self::with_children(iter)
    }
}

impl<W> Meta for Row<W> {}

impl<W, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Row<W>
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

                self.width = self.width.stack(size.width);
                self.height = self.height.cross(size.height);
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
            layout::flex::Axis::Horizontal,
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
pub struct Wrapping<W> {
    row: Row<W>,
    vertical_spacing: Option<f32>,
    align_x: alignment::Horizontal,
}

impl<W> Wrapping<W> {
    /// 设置各行 _之间_ 的垂直间距。
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
    pub fn align_x(mut self, align_x: impl Into<alignment::Horizontal>) -> Self {
        self.align_x = align_x.into();
        self
    }
}

impl<W> Meta for Wrapping<W> {}

impl<W, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Wrapping<W>
where
    W: Widget<Message, Theme, Renderer>,
{
    fn diff(&mut self, tree: &mut Tree) {
        self.row.diff(tree);
    }

    fn size(&self) -> Size<Length> {
        self.row.size()
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let limits = limits
            .width(self.row.width)
            .height(self.row.height)
            .shrink(self.row.padding);

        let child_limits = limits.loose();
        let spacing = self.row.spacing;
        let vertical_spacing = self.vertical_spacing.unwrap_or(spacing);
        let max_width = limits.bounds().width;

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

        let align_y =
            |row_start: std::ops::Range<usize>, row_height: f32, children: &mut [Tree]| {
                if align_factor != 0.0 {
                    for child in &mut children[row_start] {
                        let height = child.size.height;

                        child.translation.y += (row_height - height) / align_factor;
                    }
                }
            };

        for (i, child) in self.row.children.iter_mut().enumerate() {
            child.layout(&mut tree.children[i], renderer, &child_limits);

            let child_size = tree.children[i].size;

            if x != 0.0 && x + child_size.width > max_width {
                intrinsic_size.width = intrinsic_size.width.max(x - spacing);

                align_y(row_start..i, row_height, &mut tree.children);

                y += row_height + vertical_spacing;
                x = 0.0;
                row_start = i;
                row_height = 0.0;
            }

            row_height = row_height.max(child_size.height);

            tree.children[i].translation =
                Vector::new(x + self.row.padding.left, y + self.row.padding.top);

            x += child_size.width + spacing;
        }

        if x != 0.0 {
            intrinsic_size.width = intrinsic_size.width.max(x - spacing);
        }

        intrinsic_size.height = y + row_height;
        align_y(
            row_start..tree.children.len(),
            row_height,
            &mut tree.children,
        );

        let align_factor = match self.align_x {
            alignment::Horizontal::Left => 0.0,
            alignment::Horizontal::Center => 2.0,
            alignment::Horizontal::Right => 1.0,
        };

        if align_factor != 0.0 {
            let total_width = intrinsic_size.width;

            let mut row_start = 0;

            for i in 0..tree.children.len() {
                let row_width = tree.children[i].translation.x + tree.children[i].size.width;

                let next_x = tree
                    .children
                    .get(i + 1)
                    .map(|child| child.translation.x)
                    .unwrap_or_default();

                if next_x == 0.0 {
                    let translation = Vector::new((total_width - row_width) / align_factor, 0.0);

                    for child in &mut tree.children[row_start..=i] {
                        child.translation += translation;
                    }

                    row_start = i + 1;
                }
            }
        }

        let size = limits.resolve(self.row.width, self.row.height, intrinsic_size);

        tree.size = size.expand(self.row.padding);
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        self.row
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
        self.row
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
        self.row
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
        self.row
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
        self.row
            .overlay(tree, layout, renderer, viewport, translation, window)
    }
}
