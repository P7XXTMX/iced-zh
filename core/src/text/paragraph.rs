//! 绘制段落。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Draw paragraphs.
//!
//! </details>
use crate::alignment;
use crate::text::{
    Alignment, Difference, Ellipsis, Hit, LineHeight, Shaping, Span, Target, Text, Wrapping,
};
use crate::{Font, Pixels, Point, Rectangle, Size};

/// 文本段落。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A text paragraph.
///
/// </details>
pub trait Paragraph: Sized + Default {
    /// 使用给定的 [`Text`] 创建一个已完成布局的新 [`Paragraph`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Paragraph`] laid out with the given [`Text`].
    ///
    /// </details>
    fn with_text(text: Text<&str>) -> Self;

    /// 使用给定的 [`Text`] 创建一个已完成布局的新 [`Paragraph`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Paragraph`] laid out with the given [`Text`].
    ///
    /// </details>
    fn with_spans<Link>(text: Text<&[Span<'_, Link>]>) -> Self;

    /// 使用新的边界对 [`Paragraph`] 进行布局。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Lays out the [`Paragraph`] with some new boundaries.
    ///
    /// </details>
    fn resize(&mut self, new_bounds: Size);

    /// 把 [`Paragraph`] 与期望的 [`Text`] 进行比较，并返回
    /// [`Difference`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Compares the [`Paragraph`] with some desired [`Text`] and returns the
    /// [`Difference`].
    ///
    /// </details>
    fn compare(&self, text: Text<()>) -> Difference;

    /// 返回 [`Paragraph`] 的文本大小，单位为 [`Pixels`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the text size of the [`Paragraph`] in [`Pixels`].
    ///
    /// </details>
    fn size(&self) -> Pixels;

    /// 返回 [`Paragraph`] 的微调（hint）因子。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the hint factor of the [`Paragraph`].
    ///
    /// </details>
    fn hint_factor(&self) -> Option<f32>;

    /// 返回 [`Paragraph`] 的 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Font`] of the [`Paragraph`].
    ///
    /// </details>
    fn font(&self) -> Font;

    /// 返回 [`Paragraph`] 的 [`LineHeight`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`LineHeight`] of the [`Paragraph`].
    ///
    /// </details>
    fn line_height(&self) -> LineHeight;

    /// 返回 [`Paragraph`] 的水平对齐方式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the horizontal alignment of the [`Paragraph`].
    ///
    /// </details>
    fn align_x(&self) -> Alignment;

    /// 返回 [`Paragraph`] 的垂直对齐方式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the vertical alignment of the [`Paragraph`].
    ///
    /// </details>
    fn align_y(&self) -> alignment::Vertical;

    /// 返回 [`Paragraph`] 的 [`Wrapping`] 策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Wrapping`] strategy of the [`Paragraph`]>
    ///
    /// </details>
    fn wrapping(&self) -> Wrapping;

    /// 返回 [`Paragraph`] 的 [`Ellipsis`] 策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Ellipsis`] strategy of the [`Paragraph`]>
    ///
    /// </details>
    fn ellipsis(&self) -> Ellipsis;

    /// 返回 [`Paragraph`] 的 [`Shaping`] 策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Shaping`] strategy of the [`Paragraph`]>
    ///
    /// </details>
    fn shaping(&self) -> Shaping;

    /// 返回用于对 [`Paragraph`] 进行布局的可用边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the available bounds used to layout the [`Paragraph`].
    ///
    /// </details>
    fn bounds(&self) -> Size;

    /// 返回能容纳 [`Paragraph`] 内容的
    /// 最小边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the minimum boundaries that can fit the contents of the
    /// [`Paragraph`].
    ///
    /// </details>
    fn min_bounds(&self) -> Size;

    /// 测试给定的点是否位于 [`Paragraph`] 的边界之内，
    /// 并返回最近字符的相关信息。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Tests whether the provided point is within the boundaries of the
    /// [`Paragraph`], returning information about the nearest character.
    ///
    /// </details>
    fn hit_test(&self, point: Point) -> Option<Hit>;

    /// 测试给定的点是否位于 [`Paragraph`] 中某个
    /// [`Span`] 的边界之内，并返回被命中的 [`Span`]
    /// 的索引。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Tests whether the provided point is within the boundaries of a
    /// [`Span`] in the [`Paragraph`], returning the index of the [`Span`]
    /// that was hit.
    ///
    /// </details>
    fn hit_span(&self, point: Point) -> Option<usize>;

    /// 测试给定的点是否位于 [`Paragraph`] 中某个
    /// 字形的边界之内。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Tests whether the provided point is inside the boundaries of a
    /// glyph in the [`Paragraph`].
    ///
    /// </details>
    fn hit_glyph(&self, point: Point) -> bool {
        self.hit_test(point).is_some()
    }

    /// 返回 [`Paragraph`] 中给定索引的 [`Span`] 的所有边界。
    /// 一个 [`Span`] 在它所跨的每一行上都可以有一个边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns all bounds for the provided [`Span`] index of the [`Paragraph`].
    /// A [`Span`] can have multiple bounds for each line it's on.
    ///
    /// </details>
    fn span_bounds(&self, index: usize) -> Vec<Rectangle>;

    /// 选择给定位置之间的文本。
    ///
    /// 这些位置相对于 [`Paragraph`] 的原点。
    /// `start` 是选区开始的位置，而 `end`
    /// 是它的最终位置。[`Target`] 决定选区如何
    /// 围绕这些位置扩展。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Selects the text between the given positions.
    ///
    /// The positions are relative to the origin of the [`Paragraph`].
    /// `start` is the position where the selection began, while `end`
    /// is its final position. The [`Target`] determines how the
    /// selection is expanded around these positions.
    ///
    /// </details>
    fn select(&mut self, start: Point, end: Point, target: Target);

    /// 选择 [`Paragraph`] 的全部文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Selects all the text of the [`Paragraph`].
    ///
    /// </details>
    fn select_all(&mut self);

    /// 取消选择所有已选中的文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Deselects any selected text.
    ///
    /// </details>
    fn deselect(&mut self);

    /// 返回 [`Paragraph`] 当前选区的各个区域。
    ///
    /// 每个 [`Rectangle`] 对应选区中的一行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the regions of the current selection of the [`Paragraph`].
    ///
    /// Each [`Rectangle`] corresponds to a line of the selection.
    ///
    /// </details>
    fn selection(&self) -> &[Rectangle];

    /// 返回 [`Paragraph`] 当前选中的文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current selected text of the [`Paragraph`].
    ///
    /// </details>
    ///
    // TODO: Make immutable
    // 待办：改为不可变
    fn copy(&mut self) -> Option<String>;

    /// 返回能容纳 [`Paragraph`] 内容的最小宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the minimum width that can fit the contents of the [`Paragraph`].
    ///
    /// </details>
    fn min_width(&self) -> f32 {
        self.min_bounds().width
    }

    /// 返回能容纳 [`Paragraph`] 内容的最小高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the minimum height that can fit the contents of the [`Paragraph`].
    ///
    /// </details>
    fn min_height(&self) -> f32 {
        self.min_bounds().height
    }
}

/// 纯文本的 [`Paragraph`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A [`Paragraph`] of plain text.
///
/// </details>
#[derive(Debug, Clone, Default)]
pub struct Plain<P: Paragraph> {
    raw: P,
    content: String,
}

impl<P: Paragraph> Plain<P> {
    /// 创建一个新的 [`Plain`] 段落。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Plain`] paragraph.
    ///
    /// </details>
    pub fn new(text: Text<String>) -> Self {
        Self {
            raw: P::with_text(text.as_ref()),
            content: text.content,
        }
    }

    /// 在需要时更新纯文本 [`Paragraph`]，使其与给定的 [`Text`] 一致。
    ///
    /// 如果 [`Paragraph`] 发生了变化，返回 true。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Updates the plain [`Paragraph`] to match the given [`Text`], if needed.
    ///
    /// Returns true if the [`Paragraph`] changed.
    ///
    /// </details>
    pub fn update(&mut self, text: Text<&str>) -> bool {
        if self.content != text.content {
            text.content.clone_into(&mut self.content);
            self.raw = P::with_text(text);
            return true;
        }

        match self.raw.compare(text.with_content(())) {
            Difference::None => false,
            Difference::Bounds => {
                self.raw.resize(text.bounds);
                true
            }
            Difference::Shape => {
                self.raw = P::with_text(text);
                true
            }
        }
    }

    /// 返回 [`Paragraph`] 的水平对齐方式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the horizontal alignment of the [`Paragraph`].
    ///
    /// </details>
    pub fn align_x(&self) -> Alignment {
        self.raw.align_x()
    }

    /// 返回 [`Paragraph`] 的垂直对齐方式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the vertical alignment of the [`Paragraph`].
    ///
    /// </details>
    pub fn align_y(&self) -> alignment::Vertical {
        self.raw.align_y()
    }

    /// 返回能容纳 [`Paragraph`] 内容的
    /// 最小边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the minimum boundaries that can fit the contents of the
    /// [`Paragraph`].
    ///
    /// </details>
    pub fn min_bounds(&self) -> Size {
        self.raw.min_bounds()
    }

    /// 返回能容纳 [`Paragraph`] 内容的
    /// 最小宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the minimum width that can fit the contents of the
    /// [`Paragraph`].
    ///
    /// </details>
    pub fn min_width(&self) -> f32 {
        self.raw.min_width()
    }

    /// 返回能容纳 [`Paragraph`] 内容的
    /// 最小高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the minimum height that can fit the contents of the
    /// [`Paragraph`].
    ///
    /// </details>
    pub fn min_height(&self) -> f32 {
        self.raw.min_height()
    }

    /// 返回缓存的 [`Paragraph`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the cached [`Paragraph`].
    ///
    /// </details>
    pub fn raw(&self) -> &P {
        &self.raw
    }

    /// 返回缓存的 [`Paragraph`] 的可变引用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the cached [`Paragraph`], mutably.
    ///
    /// </details>
    pub fn raw_mut(&mut self) -> &mut P {
        &mut self.raw
    }

    /// 返回纯文本 [`Paragraph`] 当前的内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current content of the plain [`Paragraph`].
    ///
    /// </details>
    pub fn content(&self) -> &str {
        &self.content
    }

    /// 以 [`Text`] 定义的形式返回 [`Paragraph`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Paragraph`] as a [`Text`] definition.
    ///
    /// </details>
    pub fn as_text(&self) -> Text<&str> {
        Text {
            content: &self.content,
            bounds: self.raw.bounds(),
            size: self.raw.size(),
            line_height: self.raw.line_height(),
            font: self.raw.font(),
            align_x: self.raw.align_x(),
            align_y: self.raw.align_y(),
            shaping: self.raw.shaping(),
            wrapping: self.raw.wrapping(),
            ellipsis: self.raw.ellipsis(),
            hint_factor: self.raw.hint_factor(),
        }
    }
}
