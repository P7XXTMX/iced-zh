//! 构建并绘制几何图形。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Build and draw geometry.
//!
//! </details>
pub mod fill;
pub mod frame;
pub mod path;
pub mod stroke;

mod cache;
mod style;
mod text;

pub use cache::Cache;
pub use fill::Fill;
pub use frame::Frame;
pub use path::Path;
pub use stroke::{LineCap, LineDash, LineJoin, Stroke};
pub use style::Style;
pub use text::Text;

pub use crate::core::{Image, Svg};
pub use crate::gradient::{self, Gradient};

use crate::cache::Cached;
use crate::core::{self, Rectangle};

/// 能够绘制某种 [`Self::Geometry`] 的渲染器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A renderer capable of drawing some [`Self::Geometry`].
///
/// </details>
pub trait Renderer: core::Renderer {
    /// 此渲染器能够绘制的几何图形种类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The kind of geometry this renderer can draw.
    ///
    /// </details>
    type Geometry: Cached;

    /// 此渲染器支持的 [`Frame`] 种类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The kind of [`Frame`] this renderer supports.
    ///
    /// </details>
    type Frame: frame::Backend<Geometry = Self::Geometry>;

    /// 创建一个新的 [`Self::Frame`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Self::Frame`].
    ///
    /// </details>
    fn new_frame(&self, bounds: Rectangle) -> Self::Frame;

    /// 绘制给定的 [`Self::Geometry`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the given [`Self::Geometry`].
    ///
    /// </details>
    fn draw_geometry(&mut self, geometry: Self::Geometry);
}

#[cfg(debug_assertions)]
impl Renderer for () {
    type Geometry = ();
    type Frame = ();

    fn new_frame(&self, _bounds: Rectangle) -> Self::Frame {}

    fn draw_geometry(&mut self, _geometry: Self::Geometry) {}
}
