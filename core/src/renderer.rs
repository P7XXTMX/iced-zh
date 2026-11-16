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
use crate::text;
use crate::{
    Background, Border, Color, Font, Pixels, Rectangle, Shadow, Size, Transformation, Vector,
};

/// 是否应该通过把图元坐标吸附到像素网格
/// 来避免抗锯齿。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Whether anti-aliasing should be avoided by snapping primitive coordinates to the
/// pixel grid.
///
/// </details>
pub const CRISP: bool = cfg!(feature = "crisp");

/// 部件可以用来把自己绘制到屏幕上的组件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A component that can be used by widgets to draw themselves on a screen.
///
/// </details>
pub trait Renderer: 'static {
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
    fn with_transformation(&mut self, transformation: Transformation, f: impl FnOnce(&mut Self)) {
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
    fn with_translation(&mut self, translation: Vector, f: impl FnOnce(&mut Self)) {
        self.with_transformation(Transformation::translate(translation.x, translation.y), f);
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

    /// 为给定的 [`image::Handle`] 创建一个 [`image::Allocation`]，并用它调用给定的回调。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates an [`image::Allocation`] for the given [`image::Handle`] and calls the given callback with it.
    ///
    /// </details>
    fn allocate_image(
        &self,
        handle: &image::Handle,
        callback: impl FnOnce(Result<image::Allocation, image::Error>) + Send + 'static,
    );

    /// 向 [`Renderer`] 提供有关渲染目标的提示。
    ///
    /// [`Renderer`] 内部可以利用它进行优化
    /// 和（或）提升渲染质量。
    ///
    /// 例如，提供一个 [`Scale`] 之后，某些渲染器可以
    /// 在内部以物理坐标执行度量微调（metrics hinting），同时让
    /// 布局坐标保持为逻辑坐标，从而保持线性。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Provides hints to the [`Renderer`] about the rendering target.
    ///
    /// This may be used internally by the [`Renderer`] to perform optimizations
    /// and/or improve rendering quality.
    ///
    /// For instance, providing a [`Scale`] may be used by some renderers to
    /// perform metrics hinting internally in physical coordinates while keeping
    /// layout coordinates logical and, therefore, maintain linearity.
    ///
    /// </details>
    fn hint(&mut self, scale: Scale);

    /// 返回最近一次作为 [`hint`](Self::hint) 提供的 [`Scale`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the last [`Scale`] provided as a [`hint`](Self::hint).
    ///
    /// </details>
    fn scale(&self) -> Option<Scale>;

    /// 返回最近一次作为 [`hint`](Self::hint) 提供的微调（hint）因子，
    /// 仅当 [`Settings::metrics_hinting`] 启用时才返回。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the last hint factor provided as a [`hint`](Self::hint),
    /// only if [`Settings::metrics_hinting`] is enabled.
    ///
    /// </details>
    fn hint_factor(&self) -> Option<f32> {
        if !self.settings().metrics_hinting {
            return None;
        }

        self.scale().map(Scale::total)
    }

    /// 重置 [`Renderer`]，以便在 `new_bounds` 中从头开始绘制。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Resets the [`Renderer`] to start drawing in the `new_bounds` from scratch.
    ///
    /// </details>
    fn reset(&mut self, new_bounds: Rectangle);

    /// 返回此 [`Renderer`] 的 [`Settings`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Settings`] of this [`Renderer`].
    ///
    /// </details>
    fn settings(&self) -> Settings;

    /// 轮询 [`Renderer`] 中可能尚未完成的并发计算。
    ///
    /// 默认情况下，它什么都不做。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Polls any concurrent computations that may be pending in the [`Renderer`].
    ///
    /// By default, it does nothing.
    ///
    /// </details>
    fn tick(&mut self) {}
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
            snap: CRISP,
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
    fn new(settings: Settings, backend: Option<&str>) -> impl Future<Output = Option<Self>>
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

/// [`Renderer`] 的设置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The settings of a [`Renderer`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    /// 要使用的默认 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default [`Font`] to use.
    ///
    /// </details>
    pub font: Font,

    /// 文本的默认大小。
    ///
    /// 默认情况下，它会被设置为 `16.0`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default size of text.
    ///
    /// By default, it will be set to `16.0`.
    ///
    /// </details>
    pub text_size: Pixels,

    /// 文本的默认行高。
    ///
    /// 默认情况下，它会被设置为 `LineHeight::Relative(1.375)`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default line height of text.
    ///
    /// By default, it will be set to `LineHeight::Relative(1.375)`.
    ///
    /// </details>
    pub line_height: text::LineHeight,

    /// [`Renderer`] 是否应该执行度量微调（metrics hinting）。
    ///
    /// 默认情况下，它是启用的。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the [`Renderer`] should perform metrics hinting.
    ///
    /// By default, it is enabled.
    ///
    /// </details>
    pub metrics_hinting: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            font: Font::DEFAULT,
            text_size: Pixels(16.0),
            line_height: text::LineHeight::default(),
            metrics_hinting: true,
        }
    }
}

/// [`Renderer`] 的缩放因子。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The scale factor of a [`Renderer`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scale {
    /// 窗口的全局缩放因子。
    ///
    /// 它通常由操作系统控制，并全局应用到所有应用程序。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The global scale factor of the window.
    ///
    /// This is normally controlled by the OS, and applied globally to all apps.
    ///
    /// </details>
    pub window: f32,

    /// 应用程序的局部缩放因子。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The local scale factor of the application.
    ///
    /// </details>
    pub application: f32,
}

impl Scale {
    /// 返回 [`Renderer`] 应用的总缩放因子。
    ///
    /// 它是窗口缩放因子与应用程序缩放因子的乘积。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the total scale factor applied by the [`Renderer`].
    ///
    /// This is the product of the window and application scale factors.
    ///
    /// </details>
    pub fn total(self) -> f32 {
        self.window * self.application
    }
}

impl Default for Scale {
    fn default() -> Self {
        Self {
            window: 1.0,
            application: 1.0,
        }
    }
}
