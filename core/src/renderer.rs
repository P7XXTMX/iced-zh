//! 编写你自己的渲染器。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Write your own renderer.
//!
//! </details>
#[cfg(debug_assertions)]
mod null;

use crate::image;
use crate::{
    Background, Border, Color, Font, Pixels, Rectangle, Shadow, Size,
    Transformation, Vector,
};

/// 部件可以用来把自己绘制到屏幕上的组件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A component that can be used by widgets to draw themselves on a screen.
///
/// </details>
pub trait Renderer {
    /// 开始记录一个新图层。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Starts recording a new layer.
    ///
    /// </details>
    fn start_layer(&mut self, bounds: Rectangle);

    /// 结束记录一个新图层。
    ///
    /// 新图层会把它的内容裁剪到给定的 `bounds`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Ends recording a new layer.
    ///
    /// The new layer will clip its contents to the provided `bounds`.
    ///
    /// </details>
    fn end_layer(&mut self);

    /// 在一个新图层中绘制给定闭包里记录的图元。
    ///
    /// 该图层会把它的内容裁剪到给定的 `bounds`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the primitives recorded in the given closure in a new layer.
    ///
    /// The layer will clip its contents to the provided `bounds`.
    ///
    /// </details>
    fn with_layer(&mut self, bounds: Rectangle, f: impl FnOnce(&mut Self)) {
        self.start_layer(bounds);
        f(self);
        self.end_layer();
    }

    /// 使用新的 [`Transformation`] 开始记录。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Starts recording with a new [`Transformation`].
    ///
    /// </details>
    fn start_transformation(&mut self, transformation: Transformation);

    /// 结束记录一个新图层。
    ///
    /// 新图层会把它的内容裁剪到给定的 `bounds`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Ends recording a new layer.
    ///
    /// The new layer will clip its contents to the provided `bounds`.
    ///
    /// </details>
    fn end_transformation(&mut self);

    /// 对给定闭包里记录的图元应用一个 [`Transformation`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies a [`Transformation`] to the primitives recorded in the given closure.
    ///
    /// </details>
    fn with_transformation(
        &mut self,
        transformation: Transformation,
        f: impl FnOnce(&mut Self),
    ) {
        self.start_transformation(transformation);
        f(self);
        self.end_transformation();
    }

    /// 对给定闭包里记录的图元应用一个平移。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies a translation to the primitives recorded in the given closure.
    ///
    /// </details>
    fn with_translation(
        &mut self,
        translation: Vector,
        f: impl FnOnce(&mut Self),
    ) {
        self.with_transformation(
            Transformation::translate(translation.x, translation.y),
            f,
        );
    }

    /// 使用给定的 [`Background`] 填充一个 [`Quad`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Fills a [`Quad`] with the provided [`Background`].
    ///
    /// </details>
    fn fill_quad(&mut self, quad: Quad, background: impl Into<Background>);

    /// 重置 [`Renderer`]，以便在 `new_bounds` 中从头开始绘制。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Resets the [`Renderer`] to start drawing in the `new_bounds` from scratch.
    ///
    /// </details>
    fn reset(&mut self, new_bounds: Rectangle);

    /// 为给定的 [`image::Handle`] 创建一个 [`image::Allocation`]，并用它调用给定的回调。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates an [`image::Allocation`] for the given [`image::Handle`] and calls the given callback with it.
    ///
    /// </details>
    fn allocate_image(
        &mut self,
        handle: &image::Handle,
        callback: impl FnOnce(Result<image::Allocation, image::Error>)
        + Send
        + 'static,
    );
}

/// 有四条边的多边形。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A polygon with four sides.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quad {
    /// [`Quad`] 的边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The bounds of the [`Quad`].
    ///
    /// </details>
    pub bounds: Rectangle,

    /// [`Quad`] 的 [`Border`]。边框绘制在 [`Quad`] 的内侧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Border`] of the [`Quad`]. The border is drawn on the inside of the [`Quad`].
    ///
    /// </details>
    pub border: Border,

    /// [`Quad`] 的 [`Shadow`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Shadow`] of the [`Quad`].
    ///
    /// </details>
    pub shadow: Shadow,

    /// [`Quad`] 是否应该吸附到像素网格。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the [`Quad`] should be snapped to the pixel grid.
    ///
    /// </details>
    pub snap: bool,
}

impl Default for Quad {
    fn default() -> Self {
        Self {
            bounds: Rectangle::with_size(Size::ZERO),
            border: Border::default(),
            shadow: Shadow::default(),
            snap: cfg!(feature = "crisp"),
        }
    }
}

/// [`Renderer`] 的样式属性。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The styling attributes of a [`Renderer`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// 文本颜色
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text color
    ///
    /// </details>
    pub text_color: Color,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            text_color: Color::BLACK,
        }
    }
}

/// 无头渲染器是一种不需要窗口和合成器、
/// 可以离屏渲染的渲染器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A headless renderer is a renderer that can render offscreen without
/// a window nor a compositor.
///
/// </details>
pub trait Headless {
    /// 创建一个新的 [`Headless`] 渲染器；
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Headless`] renderer;
    ///
    /// </details>
    fn new(
        default_font: Font,
        default_text_size: Pixels,
        backend: Option<&str>,
    ) -> impl Future<Output = Option<Self>>
    where
        Self: Sized;

    /// 返回渲染器的唯一名称。
    ///
    /// 测试库可以用这个名称来唯一地标识
    /// 快照。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the unique name of the renderer.
    ///
    /// This name may be used by testing libraries to uniquely identify
    /// snapshots.
    ///
    /// </details>
    fn name(&self) -> String;

    /// 离屏绘制到一张截图中，返回一组
    /// 按 RGBA 顺序表示已渲染像素的字节。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws offscreen into a screenshot, returning a collection of
    /// bytes representing the rendered pixels in RGBA order.
    ///
    /// </details>
    fn screenshot(
        &mut self,
        size: Size<u32>,
        scale_factor: f32,
        background_color: Color,
    ) -> Vec<u8>;
}
