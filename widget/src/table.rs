//! 显示表格。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Display tables.
//!
//! </details>
use crate::core;
use crate::core::alignment;
use crate::core::layout;
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::widget;
use crate::core::{
    Alignment, Background, Element, Layout, Length, Pixels, Rectangle, Size,
    Widget,
};

/// 使用给定的列和行创建一个新的 [`Table`]。
///
/// 列可以使用 [`column()`] 函数创建，而行可以是任意
/// 遍历某种数据类型 `T` 的迭代器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Table`] with the given columns and rows.
///
/// Columns can be created using the [`column()`] function, while rows can be any
/// iterator over some data type `T`.
///
/// </details>
pub fn table<'a, 'b, T, Message, Theme, Renderer>(
    columns: impl IntoIterator<Item = Column<'a, 'b, T, Message, Theme, Renderer>>,
    rows: impl IntoIterator<Item = T>,
) -> Table<'a, Message, Theme, Renderer>
where
    T: Clone,
    Theme: Catalog,
    Renderer: core::Renderer,
{
    Table::new(columns, rows)
}

/// 使用给定的表头和视图函数创建一个新的 [`Column`]。
///
/// 视图函数会为 [`Table`] 中的每一行调用一次，并且必须
/// 产生对应单元格的内容。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Column`] with the given header and view function.
///
/// The view function will be called for each row in a [`Table`] and it must
/// produce the resulting contents of a cell.
///
/// </details>
pub fn column<'a, 'b, T, E, Message, Theme, Renderer>(
    header: impl Into<Element<'a, Message, Theme, Renderer>>,
    view: impl Fn(T) -> E + 'b,
) -> Column<'a, 'b, T, Message, Theme, Renderer>
where
    T: 'a,
    E: Into<Element<'a, Message, Theme, Renderer>>,
{
    Column {
        header: header.into(),
        view: Box::new(move |data| view(data).into()),
        width: Length::Shrink,
        align_x: alignment::Horizontal::Left,
        align_y: alignment::Vertical::Top,
    }
}

/// 以网格形式直观呈现按列和行分布的数据。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A grid-like visual representation of data distributed in columns and rows.
///
/// </details>
pub struct Table<'a, Message, Theme = crate::Theme, Renderer = crate::Renderer>
where
    Theme: Catalog,
{
    columns: Vec<Column_>,
    cells: Vec<Element<'a, Message, Theme, Renderer>>,
    width: Length,
    height: Length,
    padding_x: f32,
    padding_y: f32,
    separator_x: f32,
    separator_y: f32,
    class: Theme::Class<'a>,
}

struct Column_ {
    width: Length,
    align_x: alignment::Horizontal,
    align_y: alignment::Vertical,
}

impl<'a, Message, Theme, Renderer> Table<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: core::Renderer,
{
    /// 使用给定的列和行创建一个新的 [`Table`]。
    ///
    /// 列可以使用 [`column()`] 函数创建，而行可以是任意
    /// 遍历某种数据类型 `T` 的迭代器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Table`] with the given columns and rows.
    ///
    /// Columns can be created using the [`column()`] function, while rows can be any
    /// iterator over some data type `T`.
    ///
    /// </details>
    pub fn new<'b, T>(
        columns: impl IntoIterator<
            Item = Column<'a, 'b, T, Message, Theme, Renderer>,
        >,
        rows: impl IntoIterator<Item = T>,
    ) -> Self
    where
        T: Clone,
    {
        let columns = columns.into_iter();
        let rows = rows.into_iter();

        let mut width = Length::Shrink;
        let mut height = Length::Shrink;

        let mut cells = Vec::with_capacity(
            columns.size_hint().0 * (1 + rows.size_hint().0),
        );

        let (mut columns, views): (Vec<_>, Vec<_>) = columns
            .map(|column| {
                width = width.enclose(column.width);

                cells.push(column.header);

                (
                    Column_ {
                        width: column.width,
                        align_x: column.align_x,
                        align_y: column.align_y,
                    },
                    column.view,
                )
            })
            .collect();

        for row in rows {
            for view in &views {
                let cell = view(row.clone());
                let size_hint = cell.as_widget().size_hint();

                height = height.enclose(size_hint.height);

                cells.push(cell);
            }
        }

        if width == Length::Shrink
            && let Some(first) = columns.first_mut()
        {
            first.width = Length::Fill;
        }

        Self {
            columns,
            cells,
            width,
            height,
            padding_x: 10.0,
            padding_y: 5.0,
            separator_x: 1.0,
            separator_y: 1.0,
            class: Theme::default(),
        }
    }

    /// 设置 [`Table`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Table`].
    ///
    /// </details>
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// 设置 [`Table`] 单元格的内边距。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the padding of the cells of the [`Table`].
    ///
    /// </details>
    pub fn padding(self, padding: impl Into<Pixels>) -> Self {
        let padding = padding.into();

        self.padding_x(padding).padding_y(padding)
    }

    /// 设置 [`Table`] 单元格的水平内边距。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the horizontal padding of the cells of the [`Table`].
    ///
    /// </details>
    pub fn padding_x(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding_x = padding.into().0;
        self
    }

    /// 设置 [`Table`] 单元格的垂直内边距。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the vertical padding of the cells of the [`Table`].
    ///
    /// </details>
    pub fn padding_y(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding_y = padding.into().0;
        self
    }

    /// 设置 [`Table`] 单元格之间分隔线的粗细。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the thickness of the line separator between the cells of the [`Table`].
    ///
    /// </details>
    pub fn separator(self, separator: impl Into<Pixels>) -> Self {
        let separator = separator.into();

        self.separator_x(separator).separator_y(separator)
    }

    /// 设置 [`Table`] 单元格之间水平分隔线的粗细。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the thickness of the horizontal line separator between the cells of the [`Table`].
    ///
    /// </details>
    pub fn separator_x(mut self, separator: impl Into<Pixels>) -> Self {
        self.separator_x = separator.into().0;
        self
    }

    /// 设置 [`Table`] 单元格之间垂直分隔线的粗细。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the thickness of the vertical line separator between the cells of the [`Table`].
    ///
    /// </details>
    pub fn separator_y(mut self, separator: impl Into<Pixels>) -> Self {
        self.separator_y = separator.into().0;
        self
    }
}

struct Metrics {
    columns: Vec<f32>,
    rows: Vec<f32>,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Table<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: core::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<Metrics>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(Metrics {
            columns: Vec::new(),
            rows: Vec::new(),
        })
    }

    fn children(&self) -> Vec<widget::Tree> {
        self.cells
            .iter()
            .map(|cell| widget::Tree::new(cell.as_widget()))
            .collect()
    }

    fn diff(&self, tree: &mut widget::Tree) {
        tree.diff_children(&self.cells);
    }

    fn layout(
        &mut self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let metrics = tree.state.downcast_mut::<Metrics>();
        let columns = self.columns.len();
        let rows = self.cells.len() / columns;

        let limits = limits.width(self.width).height(self.height);
        let available = limits.max();
        let table_fluid = self.width.fluid();

        let mut cells = Vec::with_capacity(self.cells.len());
        cells.resize(self.cells.len(), layout::Node::default());

        metrics.columns = vec![0.0; self.columns.len()];
        metrics.rows = vec![0.0; rows];

        let mut column_factors = vec![0; self.columns.len()];
        let mut total_row_factors = 0;
        let mut total_fluid_height = 0.0;
        let mut row_factor = 0;

        let spacing_x = self.padding_x * 2.0 + self.separator_x;
        let spacing_y = self.padding_y * 2.0 + self.separator_y;

        // FIRST PASS
        // Lay out non-fluid cells
        // 第一遍
        // 布局非流式单元格
        let mut x = self.padding_x;
        let mut y = self.padding_y;

        for (i, (cell, state)) in
            self.cells.iter_mut().zip(&mut tree.children).enumerate()
        {
            let row = i / columns;
            let column = i % columns;

            let width = self.columns[column].width;
            let size = cell.as_widget().size();

            if column == 0 {
                x = self.padding_x;

                if row > 0 {
                    y += metrics.rows[row - 1] + spacing_y;

                    if row_factor != 0 {
                        total_fluid_height += metrics.rows[row - 1];
                        total_row_factors += row_factor;

                        row_factor = 0;
                    }
                }
            }

            let width_factor = width.fill_factor();
            let height_factor = size.height.fill_factor();

            if width_factor != 0 || height_factor != 0 || size.width.is_fill() {
                column_factors[column] =
                    column_factors[column].max(width_factor);

                row_factor = row_factor.max(height_factor);

                continue;
            }

            let limits = layout::Limits::new(
                Size::ZERO,
                Size::new(available.width - x, available.height - y),
            )
            .width(width);

            let layout = cell.as_widget_mut().layout(state, renderer, &limits);
            let size = limits.resolve(width, Length::Shrink, layout.size());

            metrics.columns[column] = metrics.columns[column].max(size.width);
            metrics.rows[row] = metrics.rows[row].max(size.height);
            cells[i] = layout;

            x += size.width + spacing_x;
        }

        // SECOND PASS
        // Lay out fluid cells, using metrics from the first pass as limits
        // 第二遍
        // 布局流式单元格，使用第一遍得到的度量作为限制
        let left = Size::new(
            available.width
                - metrics
                    .columns
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| column_factors[*i] == 0)
                    .map(|(_, width)| width)
                    .sum::<f32>(),
            available.height - total_fluid_height,
        );

        let width_unit = (left.width
            - spacing_x * self.columns.len().saturating_sub(1) as f32
            - self.padding_x * 2.0)
            / column_factors.iter().sum::<u16>() as f32;

        let height_unit = (left.height
            - spacing_y * rows.saturating_sub(1) as f32
            - self.padding_y * 2.0)
            / total_row_factors as f32;

        let mut x = self.padding_x;
        let mut y = self.padding_y;

        for (i, (cell, state)) in
            self.cells.iter_mut().zip(&mut tree.children).enumerate()
        {
            let row = i / columns;
            let column = i % columns;

            let size = cell.as_widget().size();

            let width = self.columns[column].width;
            let width_factor = width.fill_factor();
            let height_factor = size.height.fill_factor();

            if column == 0 {
                x = self.padding_x;

                if row > 0 {
                    y += metrics.rows[row - 1] + spacing_y;
                }
            }

            if width_factor == 0
                && size.width.fill_factor() == 0
                && size.height.fill_factor() == 0
            {
                continue;
            }

            let max_width = if width_factor == 0 {
                if size.width.is_fill() {
                    metrics.columns[column]
                } else {
                    (available.width - x).max(0.0)
                }
            } else {
                width_unit * width_factor as f32
            };

            let max_height = if height_factor == 0 {
                if size.height.is_fill() {
                    metrics.rows[row]
                } else {
                    (available.height - y).max(0.0)
                }
            } else {
                height_unit * height_factor as f32
            };

            let limits = layout::Limits::new(
                Size::ZERO,
                Size::new(max_width, max_height),
            )
            .width(width);

            let layout = cell.as_widget_mut().layout(state, renderer, &limits);
            let size = limits.resolve(
                if let Length::Fixed(_) = width {
                    width
                } else {
                    table_fluid
                },
                Length::Shrink,
                layout.size(),
            );

            metrics.columns[column] = metrics.columns[column].max(size.width);
            metrics.rows[row] = metrics.rows[row].max(size.height);
            cells[i] = layout;

            x += size.width + spacing_x;
        }

        // THIRD PASS
        // Position each cell
        // 第三遍
        // 定位每个单元格
        let mut x = self.padding_x;
        let mut y = self.padding_y;

        for (i, cell) in cells.iter_mut().enumerate() {
            let row = i / columns;
            let column = i % columns;

            if column == 0 {
                x = self.padding_x;

                if row > 0 {
                    y += metrics.rows[row - 1] + spacing_y;
                }
            }

            let Column_ {
                align_x, align_y, ..
            } = &self.columns[column];

            cell.move_to_mut((x, y));
            cell.align_mut(
                Alignment::from(*align_x),
                Alignment::from(*align_y),
                Size::new(metrics.columns[column], metrics.rows[row]),
            );

            x += metrics.columns[column] + spacing_x;
        }

        let intrinsic = limits.resolve(
            self.width,
            self.height,
            Size::new(
                x - spacing_x + self.padding_x,
                y + metrics
                    .rows
                    .last()
                    .copied()
                    .map(|height| height + self.padding_y)
                    .unwrap_or_default(),
            ),
        );

        layout::Node::with_children(intrinsic, cells)
    }

    fn update(
        &mut self,
        tree: &mut widget::Tree,
        event: &core::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn core::Clipboard,
        shell: &mut core::Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        for ((cell, tree), layout) in self
            .cells
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
        {
            cell.as_widget_mut().update(
                tree, event, layout, cursor, renderer, clipboard, shell,
                viewport,
            );
        }
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        for ((cell, state), layout) in
            self.cells.iter().zip(&tree.children).zip(layout.children())
        {
            cell.as_widget()
                .draw(state, renderer, theme, style, layout, cursor, viewport);
        }

        let bounds = layout.bounds();
        let metrics = tree.state.downcast_ref::<Metrics>();
        let style = theme.style(&self.class);

        if self.separator_x > 0.0 {
            let mut x = self.padding_x;

            for width in
                &metrics.columns[..metrics.columns.len().saturating_sub(1)]
            {
                x += width + self.padding_x;

                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x: bounds.x + x,
                            y: bounds.y,
                            width: self.separator_x,
                            height: bounds.height,
                        },
                        snap: true,
                        ..renderer::Quad::default()
                    },
                    style.separator_x,
                );

                x += self.separator_x + self.padding_x;
            }
        }

        if self.separator_y > 0.0 {
            let mut y = self.padding_y;

            for height in &metrics.rows[..metrics.rows.len().saturating_sub(1)]
            {
                y += height + self.padding_y;

                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x: bounds.x,
                            y: bounds.y + y,
                            width: bounds.width,
                            height: self.separator_y,
                        },
                        snap: true,
                        ..renderer::Quad::default()
                    },
                    style.separator_y,
                );

                y += self.separator_y + self.padding_y;
            }
        }
    }

    fn mouse_interaction(
        &self,
        tree: &widget::Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.cells
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((cell, tree), layout)| {
                cell.as_widget()
                    .mouse_interaction(tree, layout, cursor, viewport, renderer)
            })
            .max()
            .unwrap_or_default()
    }

    fn operate(
        &mut self,
        tree: &mut widget::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        for ((cell, state), layout) in self
            .cells
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
        {
            cell.as_widget_mut()
                .operate(state, layout, renderer, operation);
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut widget::Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: core::Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        overlay::from_children(
            &mut self.cells,
            tree,
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message, Theme, Renderer> From<Table<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::Renderer + 'a,
{
    fn from(table: Table<'a, Message, Theme, Renderer>) -> Self {
        Element::new(table)
    }
}

/// 带有表头的一组数据的纵向呈现。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A vertical visualization of some data with a header.
///
/// </details>
pub struct Column<
    'a,
    'b,
    T,
    Message,
    Theme = crate::Theme,
    Renderer = crate::Renderer,
> {
    header: Element<'a, Message, Theme, Renderer>,
    view: Box<dyn Fn(T) -> Element<'a, Message, Theme, Renderer> + 'b>,
    width: Length,
    align_x: alignment::Horizontal,
    align_y: alignment::Vertical,
}

impl<'a, 'b, T, Message, Theme, Renderer>
    Column<'a, 'b, T, Message, Theme, Renderer>
{
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

    /// 设置 [`Column`] 水平轴上的对齐方式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the alignment for the horizontal axis of the [`Column`].
    ///
    /// </details>
    pub fn align_x(
        mut self,
        alignment: impl Into<alignment::Horizontal>,
    ) -> Self {
        self.align_x = alignment.into();
        self
    }

    /// 设置 [`Column`] 垂直轴上的对齐方式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the alignment for the vertical axis of the [`Column`].
    ///
    /// </details>
    pub fn align_y(
        mut self,
        alignment: impl Into<alignment::Vertical>,
    ) -> Self {
        self.align_y = alignment.into();
        self
    }
}

/// [`Table`] 的外观。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The appearance of a [`Table`].
///
/// </details>
#[derive(Debug, Clone, Copy)]
pub struct Style {
    /// 单元格之间水平分隔线的背景颜色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The background color of the horizontal line separator between cells.
    ///
    /// </details>
    pub separator_x: Background,
    /// 单元格之间垂直分隔线的背景颜色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The background color of the vertical line separator between cells.
    ///
    /// </details>
    pub separator_y: Background,
}

/// [`Table`] 的主题样式目录。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The theme catalog of a [`Table`].
///
/// </details>
pub trait Catalog {
    /// [`Catalog`] 的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The item class of the [`Catalog`].
    ///
    /// </details>
    type Class<'a>;

    /// [`Catalog`] 产生的默认样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default class produced by the [`Catalog`].
    ///
    /// </details>
    fn default<'a>() -> Self::Class<'a>;

    /// 某个样式类在给定交互状态下的 [`Style`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Style`] of a class with the given status.
    ///
    /// </details>
    fn style(&self, class: &Self::Class<'_>) -> Style;
}

/// [`Table`] 的样式函数。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A styling function for a [`Table`].
///
/// </details>
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl<Theme> From<Style> for StyleFn<'_, Theme> {
    fn from(style: Style) -> Self {
        Box::new(move |_theme| style)
    }
}

impl Catalog for crate::Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

/// [`Table`] 的默认样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The default style of a [`Table`].
///
/// </details>
pub fn default(theme: &crate::Theme) -> Style {
    let palette = theme.extended_palette();
    let separator = palette.background.strong.color.into();

    Style {
        separator_x: separator,
        separator_y: separator,
    }
}
