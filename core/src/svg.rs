//! 加载并绘制矢量图形。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Load and draw vector graphics.
//!
//! </details>
use crate::{Color, Radians, Rectangle, Size};

use rustc_hash::FxHasher;
use std::borrow::Cow;
use std::hash::{Hash, Hasher as _};
use std::path::PathBuf;
use std::sync::Arc;

/// 可以被绘制的位图图像。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A raster image that can be drawn.
///
/// </details>
#[derive(Debug, Clone, PartialEq)]
pub struct Svg<H = Handle> {
    /// [`Svg`] 的句柄。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The handle of the [`Svg`].
    ///
    /// </details>
    pub handle: H,

    /// 要应用到 [`Svg`] 上的 [`Color`] 滤镜。
    ///
    /// 如果设置了某个 [`Color`]，整个 [`Svg`] 都会
    /// 用它来绘制——忽略任何固有的颜色。
    ///
    /// 这对于以编程方式给图标上色很有用
    /// （例如使用主题）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Color`] filter to be applied to the [`Svg`].
    ///
    /// If some [`Color`] is set, the whole [`Svg`] will be
    /// painted with it—ignoring any intrinsic colors.
    ///
    /// This can be useful for coloring icons programmatically
    /// (e.g. with a theme).
    ///
    /// </details>
    pub color: Option<Color>,

    /// 要应用到图像上的旋转；以图像中心为轴。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The rotation to be applied to the image; on its center.
    ///
    /// </details>
    pub rotation: Radians,

    /// [`Svg`] 的不透明度。
    ///
    /// 0 表示透明，1 表示不透明。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The opacity of the [`Svg`].
    ///
    /// 0 means transparent. 1 means opaque.
    ///
    /// </details>
    pub opacity: f32,
}

impl Svg<Handle> {
    /// 使用给定的句柄创建一个新的 [`Svg`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Svg`] with the given handle.
    ///
    /// </details>
    pub fn new(handle: impl Into<Handle>) -> Self {
        Self {
            handle: handle.into(),
            color: None,
            rotation: Radians(0.0),
            opacity: 1.0,
        }
    }

    /// 设置 [`Svg`] 的 [`Color`] 滤镜。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Color`] filter of the [`Svg`].
    ///
    /// </details>
    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// 设置 [`Svg`] 的旋转。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the rotation of the [`Svg`].
    ///
    /// </details>
    pub fn rotation(mut self, rotation: impl Into<Radians>) -> Self {
        self.rotation = rotation.into();
        self
    }

    /// 设置 [`Svg`] 的不透明度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the opacity of the [`Svg`].
    ///
    /// </details>
    pub fn opacity(mut self, opacity: impl Into<f32>) -> Self {
        self.opacity = opacity.into();
        self
    }
}

impl From<&Handle> for Svg {
    fn from(handle: &Handle) -> Self {
        Svg::new(handle.clone())
    }
}

/// Svg 数据的句柄。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A handle of Svg data.
///
/// </details>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handle {
    id: u64,
    data: Arc<Data>,
}

impl Handle {
    /// 创建一个指向给定路径上矢量图像的
    /// SVG [`Handle`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates an SVG [`Handle`] pointing to the vector image of the given
    /// path.
    ///
    /// </details>
    pub fn from_path(path: impl Into<PathBuf>) -> Handle {
        Self::from_data(Data::Path(path.into()))
    }

    /// 根据原始字节创建一个 SVG [`Handle`]，这些字节可以是 SVG 字符串，
    /// 也可以是 gzip 压缩的数据。
    ///
    /// 如果你的 SVG 数据已经在内存中（比如
    /// 是下载得到的，或是程序化生成的），这会很有用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates an SVG [`Handle`] from raw bytes containing either an SVG string
    /// or gzip compressed data.
    ///
    /// This is useful if you already have your SVG data in-memory, maybe
    /// because you downloaded or generated it procedurally.
    ///
    /// </details>
    pub fn from_memory(bytes: impl Into<Cow<'static, [u8]>>) -> Handle {
        Self::from_data(Data::Bytes(bytes.into()))
    }

    fn from_data(data: Data) -> Handle {
        let mut hasher = FxHasher::default();
        data.hash(&mut hasher);

        Handle {
            id: hasher.finish(),
            data: Arc::new(data),
        }
    }

    /// 返回 [`Handle`] 的唯一标识符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the unique identifier of the [`Handle`].
    ///
    /// </details>
    pub fn id(&self) -> u64 {
        self.id
    }

    /// 返回 SVG [`Data`] 的引用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a reference to the SVG [`Data`].
    ///
    /// </details>
    pub fn data(&self) -> &Data {
        &self.data
    }
}

impl<T> From<T> for Handle
where
    T: Into<PathBuf>,
{
    fn from(path: T) -> Handle {
        Handle::from_path(path.into())
    }
}

impl Hash for Handle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// 矢量图像的数据。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The data of a vectorial image.
///
/// </details>
#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Data {
    /// 文件数据
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// File data
    ///
    /// </details>
    Path(PathBuf),

    /// 内存中的数据
    ///
    /// 可以包含 SVG 字符串或 gzip 压缩的数据。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// In-memory data
    ///
    /// Can contain an SVG string or a gzip compressed data.
    ///
    /// </details>
    Bytes(Cow<'static, [u8]>),
}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Path(path) => write!(f, "Path({path:?})"),
            Data::Bytes(_) => write!(f, "Bytes(...)"),
        }
    }
}

/// 可以渲染矢量图形的 [`Renderer`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A [`Renderer`] that can render vector graphics.
///
/// </details>
///
/// [renderer]: crate::renderer
pub trait Renderer: crate::Renderer {
    /// 返回给定 [`Handle`] 对应 SVG 的默认尺寸。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the default dimensions of an SVG for the given [`Handle`].
    ///
    /// </details>
    fn measure_svg(&self, handle: &Handle) -> Size<u32>;

    /// 使用给定的 [`Handle`] 和可选的 [`Color`] 滤镜，在给定的 `bounds` 内绘制 SVG。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws an SVG with the given [`Handle`], an optional [`Color`] filter, and inside the provided `bounds`.
    ///
    /// </details>
    fn draw_svg(&mut self, svg: Svg, bounds: Rectangle, clip_bounds: Rectangle);
}
