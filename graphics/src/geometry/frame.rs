//! 绘制并生成几何图形。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Draw and generate geometry.
//!
//! </details>
use crate::core::{Point, Radians, Rectangle, Size, Vector};
use crate::geometry::{self, Fill, Image, Path, Stroke, Svg, Text};

/// 表面上可用于绘制几何图形的区域。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The region of a surface that can be used to draw geometry.
///
/// </details>
pub struct Frame<Renderer>
where
    Renderer: geometry::Renderer,
{
    raw: Renderer::Frame,
}

impl<Renderer> Frame<Renderer>
where
    Renderer: geometry::Renderer,
{
    /// 使用给定的尺寸创建一个新的 [`Frame`]。
    ///
    /// 绘制在该尺寸之外的几何图形都会被裁剪。
    /// 如果需要更精细的控制，请使用 [`with_bounds`](Self::with_bounds)。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Frame`] with the given dimensions.
    ///
    /// Any geometry drawn outside of the dimensions will be clipped.
    /// If you need further control, use [`with_bounds`](Self::with_bounds).
    ///
    /// </details>
    pub fn new(renderer: &Renderer, size: Size) -> Self {
        Self::with_bounds(renderer, Rectangle::with_size(size))
    }

    /// 使用给定的裁剪边界创建一个新的 [`Frame`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Frame`] with the given clip bounds.
    ///
    /// </details>
    pub fn with_bounds(renderer: &Renderer, bounds: Rectangle) -> Self {
        Self {
            raw: renderer.new_frame(bounds),
        }
    }

    /// 返回 [`Frame`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the width of the [`Frame`].
    ///
    /// </details>
    pub fn width(&self) -> f32 {
        self.raw.width()
    }

    /// 返回 [`Frame`] 的高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the height of the [`Frame`].
    ///
    /// </details>
    pub fn height(&self) -> f32 {
        self.raw.height()
    }

    /// 返回 [`Frame`] 的尺寸。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the dimensions of the [`Frame`].
    ///
    /// </details>
    pub fn size(&self) -> Size {
        self.raw.size()
    }

    /// 返回 [`Frame`] 中心点的坐标。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the coordinate of the center of the [`Frame`].
    ///
    /// </details>
    pub fn center(&self) -> Point {
        self.raw.center()
    }

    /// 用给定的样式填充给定的 [`Path`]，
    /// 把它绘制到 [`Frame`] 上。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the given [`Path`] on the [`Frame`] by filling it with the
    /// provided style.
    ///
    /// </details>
    pub fn fill(&mut self, path: &Path, fill: impl Into<Fill>) {
        self.raw.fill(path, fill);
    }

    /// 根据左上角坐标和 `Size`，用给定的样式填充一个
    /// 与坐标轴对齐的矩形，把它绘制到 [`Frame`] 上。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws an axis-aligned rectangle given its top-left corner coordinate and
    /// its `Size` on the [`Frame`] by filling it with the provided style.
    ///
    /// </details>
    pub fn fill_rectangle(&mut self, top_left: Point, size: Size, fill: impl Into<Fill>) {
        self.raw.fill_rectangle(top_left, size, fill);
    }

    /// 用给定的样式在 [`Frame`] 上绘制给定 [`Path`] 的
    /// 描边。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the stroke of the given [`Path`] on the [`Frame`] with the
    /// provided style.
    ///
    /// </details>
    pub fn stroke<'a>(&mut self, path: &Path, stroke: impl Into<Stroke<'a>>) {
        self.raw.stroke(path, stroke);
    }

    /// 根据左上角坐标和 `Size`，用给定的样式在 [`Frame`] 上
    /// 绘制一个与坐标轴对齐的矩形的描边。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the stroke of an axis-aligned rectangle with the provided style
    /// given its top-left corner coordinate and its `Size` on the [`Frame`] .
    ///
    /// </details>
    pub fn stroke_rectangle<'a>(
        &mut self,
        top_left: Point,
        size: Size,
        stroke: impl Into<Stroke<'a>>,
    ) {
        self.raw.stroke_rectangle(top_left, size, stroke);
    }

    /// 在 [`Frame`] 上绘制给定 [`Text`] 的字符，
    /// 并用给定的颜色填充它们。
    ///
    /// __警告：__ 所有文本都会渲染在 `Canvas` 所有图层的
    /// 上方。因此，它目前只适合用于覆盖层，
    /// 这也是最常见的使用场景。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the characters of the given [`Text`] on the [`Frame`], filling
    /// them with the given color.
    ///
    /// __Warning:__ All text will be rendered on top of all the layers of
    /// a `Canvas`. Therefore, it is currently only meant to be used for
    /// overlays, which is the most common use case.
    ///
    /// </details>
    pub fn fill_text(&mut self, text: impl Into<Text>) {
        self.raw.fill_text(text);
    }

    /// 在 [`Frame`] 上给定的边界内绘制给定的 [`Image`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the given [`Image`] on the [`Frame`] inside the given bounds.
    ///
    /// </details>
    #[cfg(feature = "image")]
    pub fn draw_image(&mut self, bounds: Rectangle, image: impl Into<Image>) {
        self.raw.draw_image(bounds, image);
    }

    /// 在 [`Frame`] 上给定的边界内绘制给定的 [`Svg`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the given [`Svg`] on the [`Frame`] inside the given bounds.
    ///
    /// </details>
    #[cfg(feature = "svg")]
    pub fn draw_svg(&mut self, bounds: Rectangle, svg: impl Into<Svg>) {
        self.raw.draw_svg(bounds, svg);
    }

    /// 保存 [`Frame`] 当前的变换，执行给定的
    /// 绘制操作，之后再恢复该变换。
    ///
    /// 这个方法适合用来组合变换，
    /// 并在不同的坐标系中执行绘制操作。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Stores the current transform of the [`Frame`] and executes the given
    /// drawing operations, restoring the transform afterwards.
    ///
    /// This method is useful to compose transforms and perform drawing
    /// operations in different coordinate systems.
    ///
    /// </details>
    #[inline]
    pub fn with_save<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        self.push_transform();

        let result = f(self);

        self.pop_transform();

        result
    }

    /// 把当前变换压入变换栈。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Pushes the current transform in the transform stack.
    ///
    /// </details>
    pub fn push_transform(&mut self) {
        self.raw.push_transform();
    }

    /// 从变换栈弹出一个变换，并把它设为当前变换。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Pops a transform from the transform stack and sets it as the current transform.
    ///
    /// </details>
    pub fn pop_transform(&mut self) {
        self.raw.pop_transform();
    }

    /// 在一个 [`Rectangle`] 区域内执行给定的绘制操作，
    /// 裁剪掉所有超出其边界的几何图形。期间执行的任何变换
    /// 都只在给定的闭包内部生效。
    ///
    /// 这个方法适合用来执行需要裁剪的
    /// 绘制操作。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Executes the given drawing operations within a [`Rectangle`] region,
    /// clipping any geometry that overflows its bounds. Any transformations
    /// performed are local to the provided closure.
    ///
    /// This method is useful to perform drawing operations that need to be
    /// clipped.
    ///
    /// </details>
    #[inline]
    pub fn with_clip<R>(&mut self, region: Rectangle, f: impl FnOnce(&mut Self) -> R) -> R {
        let mut frame = self.draft(region);

        let result = f(&mut frame);
        self.paste(frame);

        result
    }

    /// 使用给定的 [`Size`] 创建一个新的 [`Frame`]。
    ///
    /// 使用 [`paste`] 把它的内容绘制回当前 [`Frame`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Frame`] with the given [`Size`].
    ///
    /// Draw its contents back to this [`Frame`] with [`paste`].
    ///
    /// </details>
    ///
    /// [`paste`]: Self::paste
    fn draft(&mut self, clip_bounds: Rectangle) -> Self {
        Self {
            raw: self.raw.draft(clip_bounds),
        }
    }

    /// 以给定的 [`Point`] 为原点，绘制给定 [`Frame`] 的内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the contents of the given [`Frame`] with origin at the given [`Point`].
    ///
    /// </details>
    fn paste(&mut self, frame: Self) {
        self.raw.paste(frame.raw);
    }

    /// 对 [`Frame`] 的当前变换应用平移。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies a translation to the current transform of the [`Frame`].
    ///
    /// </details>
    pub fn translate(&mut self, translation: Vector) {
        self.raw.translate(translation);
    }

    /// 对 [`Frame`] 的当前变换应用以弧度表示的旋转。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies a rotation in radians to the current transform of the [`Frame`].
    ///
    /// </details>
    pub fn rotate(&mut self, angle: impl Into<Radians>) {
        self.raw.rotate(angle);
    }

    /// 对 [`Frame`] 的当前变换应用等比缩放。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies a uniform scaling to the current transform of the [`Frame`].
    ///
    /// </details>
    pub fn scale(&mut self, scale: f32) {
        self.raw.scale(scale);
    }

    /// 对 [`Frame`] 的当前变换应用非等比缩放。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies a non-uniform scaling to the current transform of the [`Frame`].
    ///
    /// </details>
    pub fn scale_nonuniform(&mut self, scale: impl Into<Vector>) {
        self.raw.scale_nonuniform(scale);
    }

    /// 把 [`Frame`] 转换为其底层的几何图形。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Turns the [`Frame`] into its underlying geometry.
    ///
    /// </details>
    pub fn into_geometry(self) -> Renderer::Geometry {
        self.raw.into_geometry()
    }
}

/// [`Frame`] 的内部实现。
///
/// 与 [`Frame`] 类似。各个方法的文档
/// 请参见 [`Frame`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The internal implementation of a [`Frame`].
///
/// Analogous to [`Frame`]. See [`Frame`] for the documentation
/// of each method.
///
/// </details>
#[allow(missing_docs)]
pub trait Backend: Sized {
    type Geometry;

    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn size(&self) -> Size;
    fn center(&self) -> Point;

    fn push_transform(&mut self);
    fn pop_transform(&mut self);

    fn translate(&mut self, translation: Vector);
    fn rotate(&mut self, angle: impl Into<Radians>);
    fn scale(&mut self, scale: f32);
    fn scale_nonuniform(&mut self, scale: impl Into<Vector>);

    fn draft(&mut self, clip_bounds: Rectangle) -> Self;
    fn paste(&mut self, frame: Self);

    fn stroke<'a>(&mut self, path: &Path, stroke: impl Into<Stroke<'a>>);
    fn stroke_rectangle<'a>(&mut self, top_left: Point, size: Size, stroke: impl Into<Stroke<'a>>);
    fn stroke_text<'a>(&mut self, text: impl Into<Text>, stroke: impl Into<Stroke<'a>>);

    fn fill(&mut self, path: &Path, fill: impl Into<Fill>);
    fn fill_text(&mut self, text: impl Into<Text>);
    fn fill_rectangle(&mut self, top_left: Point, size: Size, fill: impl Into<Fill>);

    fn draw_image(&mut self, bounds: Rectangle, image: impl Into<Image>);
    fn draw_svg(&mut self, bounds: Rectangle, svg: impl Into<Svg>);

    fn into_geometry(self) -> Self::Geometry;
}

#[cfg(debug_assertions)]
impl Backend for () {
    type Geometry = ();

    fn width(&self) -> f32 {
        0.0
    }

    fn height(&self) -> f32 {
        0.0
    }

    fn size(&self) -> Size {
        Size::ZERO
    }

    fn center(&self) -> Point {
        Point::ORIGIN
    }

    fn push_transform(&mut self) {}
    fn pop_transform(&mut self) {}

    fn translate(&mut self, _translation: Vector) {}
    fn rotate(&mut self, _angle: impl Into<Radians>) {}
    fn scale(&mut self, _scale: f32) {}
    fn scale_nonuniform(&mut self, _scale: impl Into<Vector>) {}

    fn draft(&mut self, _clip_bounds: Rectangle) -> Self {}
    fn paste(&mut self, _frame: Self) {}

    fn stroke<'a>(&mut self, _path: &Path, _stroke: impl Into<Stroke<'a>>) {}
    fn stroke_rectangle<'a>(
        &mut self,
        _top_left: Point,
        _size: Size,
        _stroke: impl Into<Stroke<'a>>,
    ) {
    }
    fn stroke_text<'a>(&mut self, _text: impl Into<Text>, _stroke: impl Into<Stroke<'a>>) {}

    fn fill(&mut self, _path: &Path, _fill: impl Into<Fill>) {}
    fn fill_text(&mut self, _text: impl Into<Text>) {}
    fn fill_rectangle(&mut self, _top_left: Point, _size: Size, _fill: impl Into<Fill>) {}

    fn draw_image(&mut self, _bounds: Rectangle, _image: impl Into<Image>) {}
    fn draw_svg(&mut self, _bounds: Rectangle, _svg: impl Into<Svg>) {}

    fn into_geometry(self) -> Self::Geometry {}
}
