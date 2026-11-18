//! 加载并绘制位图图像。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Load and draw raster graphics.
//!
//! </details>
use crate::border;
use crate::{Bytes, Radians, Rectangle, Size};

use rustc_hash::FxHasher;

use std::hash::{Hash, Hasher};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Weak};

/// 可以被绘制的位图图像。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A raster image that can be drawn.
///
/// </details>
#[derive(Debug, Clone, PartialEq)]
pub struct Image<H = Handle> {
    /// 图像的句柄。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The handle of the image.
    ///
    /// </details>
    pub handle: H,

    /// 图像的过滤方式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The filter method of the image.
    ///
    /// </details>
    pub filter_method: FilterMethod,

    /// 要应用到图像上的旋转；以图像中心为轴。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The rotation to be applied to the image; on its center.
    ///
    /// </details>
    pub rotation: Radians,

    /// [`Image`] 的边框半径。
    ///
    /// 目前，它只会应用到 `clip_bounds` 上。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The border radius of the [`Image`].
    ///
    /// Currently, this will only be applied to the `clip_bounds`.
    ///
    /// </details>
    pub border_radius: border::Radius,

    /// 图像的不透明度。
    ///
    /// 0 表示透明，1 表示不透明。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The opacity of the image.
    ///
    /// 0 means transparent. 1 means opaque.
    ///
    /// </details>
    pub opacity: f32,

    /// 如果设为 `true`，图像会被吸附到像素网格。
    ///
    /// 这可以避免图形瑕疵，尤其是在使用
    /// [`FilterMethod::Nearest`] 时。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// If set to `true`, the image will be snapped to the pixel grid.
    ///
    /// This can avoid graphical glitches, specially when using
    /// [`FilterMethod::Nearest`].
    ///
    /// </details>
    pub snap: bool,
}

impl Image<Handle> {
    /// 使用给定的句柄创建一个新的 [`Image`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Image`] with the given handle.
    ///
    /// </details>
    pub fn new(handle: impl Into<Handle>) -> Self {
        Self {
            handle: handle.into(),
            filter_method: FilterMethod::default(),
            rotation: Radians(0.0),
            border_radius: border::Radius::default(),
            opacity: 1.0,
            snap: false,
        }
    }

    /// 设置 [`Image`] 的过滤方式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the filter method of the [`Image`].
    ///
    /// </details>
    pub fn filter_method(mut self, filter_method: FilterMethod) -> Self {
        self.filter_method = filter_method;
        self
    }

    /// 设置 [`Image`] 的旋转。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the rotation of the [`Image`].
    ///
    /// </details>
    pub fn rotation(mut self, rotation: impl Into<Radians>) -> Self {
        self.rotation = rotation.into();
        self
    }

    /// 设置 [`Image`] 的不透明度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the opacity of the [`Image`].
    ///
    /// </details>
    pub fn opacity(mut self, opacity: impl Into<f32>) -> Self {
        self.opacity = opacity.into();
        self
    }

    /// 设置 [`Image`] 是否应该吸附到像素网格。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets whether the [`Image`] should be snapped to the pixel grid.
    ///
    /// </details>
    pub fn snap(mut self, snap: bool) -> Self {
        self.snap = snap;
        self
    }
}

impl From<&Handle> for Image {
    fn from(handle: &Handle) -> Self {
        Image::new(handle.clone())
    }
}

/// 某些图像数据的句柄。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A handle of some image data.
///
/// </details>
#[derive(Clone, PartialEq, Eq)]
pub enum Handle {
    /// 文件句柄。图像数据将从
    /// 文件路径读取。
    ///
    /// 使用 [`from_path`] 创建此变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A file handle. The image data will be read
    /// from the file path.
    ///
    /// Use [`from_path`] to create this variant.
    ///
    /// </details>
    ///
    /// [`from_path`]: Self::from_path
    Path(Id, PathBuf),

    /// 指向内存中某些已编码图像字节的句柄。
    ///
    /// 使用 [`from_bytes`] 创建此变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A handle pointing to some encoded image bytes in-memory.
    ///
    /// Use [`from_bytes`] to create this variant.
    ///
    /// </details>
    ///
    /// [`from_bytes`]: Self::from_bytes
    Bytes(Id, Bytes),

    /// 指向 RGBA 格式的已解码图像像素的句柄。
    ///
    /// 使用 [`from_rgba`] 创建此变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A handle pointing to decoded image pixels in RGBA format.
    ///
    /// Use [`from_rgba`] to create this variant.
    ///
    /// </details>
    ///
    /// [`from_rgba`]: Self::from_rgba
    Rgba {
        /// 此句柄的 id。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The id of this handle.
        ///
        /// </details>
        id: Id,
        /// 图像的宽度。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The width of the image.
        ///
        /// </details>
        width: u32,
        /// 图像的高度。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The height of the image.
        ///
        /// </details>
        height: u32,
        /// 像素数据。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The pixels.
        ///
        /// </details>
        pixels: Bytes,
    },
}

impl Handle {
    /// 创建一个指向给定路径上图像的图像 [`Handle`]。
    ///
    /// 通过检查文件中的数据，对图像格式做出合理的推测。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates an image [`Handle`] pointing to the image of the given path.
    ///
    /// Makes an educated guess about the image format by examining the data in the file.
    ///
    /// </details>
    pub fn from_path<T: Into<PathBuf>>(path: T) -> Handle {
        let path = path.into();

        Self::Path(Id::path(&path), path)
    }

    /// 创建一个直接包含已编码图像数据的图像 [`Handle`]。
    ///
    /// 通过检查给定的数据，对图像格式做出合理的推测。
    ///
    /// 如果你的图像已经加载到内存中（比如
    /// 是下载得到的，或是程序化生成的），这会很有用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates an image [`Handle`] containing the encoded image data directly.
    ///
    /// Makes an educated guess about the image format by examining the given data.
    ///
    /// This is useful if you already have your image loaded in-memory, maybe
    /// because you downloaded or generated it procedurally.
    ///
    /// </details>
    pub fn from_bytes(bytes: impl Into<Bytes>) -> Handle {
        Self::Bytes(Id::unique(), bytes.into())
    }

    /// 创建一个直接包含已解码图像像素的图像 [`Handle`]。
    ///
    /// 此函数要求像素数据以 RGBA 像素的 [`Bytes`] 集合的形式提供。
    /// 因此，像素数据的长度应该始终是
    /// `width * height * 4`。
    ///
    /// 如果你已经解码了图像，这会很有用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates an image [`Handle`] containing the decoded image pixels directly.
    ///
    /// This function expects the pixel data to be provided as a collection of [`Bytes`]
    /// of RGBA pixels. Therefore, the length of the pixel data should always be
    /// `width * height * 4`.
    ///
    /// This is useful if you have already decoded your image.
    ///
    /// </details>
    pub fn from_rgba(
        width: u32,
        height: u32,
        pixels: impl Into<Bytes>,
    ) -> Handle {
        Self::Rgba {
            id: Id::unique(),
            width,
            height,
            pixels: pixels.into(),
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
    pub fn id(&self) -> Id {
        match self {
            Handle::Path(id, _)
            | Handle::Bytes(id, _)
            | Handle::Rgba { id, .. } => *id,
        }
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

impl From<&Handle> for Handle {
    fn from(value: &Handle) -> Self {
        value.clone()
    }
}

impl std::fmt::Debug for Handle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Path(id, path) => write!(f, "Path({id:?}, {path:?})"),
            Self::Bytes(id, _) => write!(f, "Bytes({id:?}, ...)"),
            Self::Rgba {
                id, width, height, ..
            } => {
                write!(f, "Pixels({id:?}, {width} * {height})")
            }
        }
    }
}

/// 某些 [`Handle`] 数据的唯一标识符。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The unique identifier of some [`Handle`] data.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(_Id);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum _Id {
    Unique(u64),
    Hash(u64),
}

impl Id {
    fn unique() -> Self {
        use std::sync::atomic::{self, AtomicU64};

        static NEXT_ID: AtomicU64 = AtomicU64::new(0);

        Self(_Id::Unique(NEXT_ID.fetch_add(1, atomic::Ordering::Relaxed)))
    }

    fn path(path: impl AsRef<Path>) -> Self {
        let hash = {
            let mut hasher = FxHasher::default();
            path.as_ref().hash(&mut hasher);

            hasher.finish()
        };

        Self(_Id::Hash(hash))
    }
}

/// 图像过滤策略。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Image filtering strategy.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FilterMethod {
    /// 双线性插值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Bilinear interpolation.
    ///
    /// </details>
    #[default]
    Linear,
    /// 最近邻。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Nearest neighbor.
    ///
    /// </details>
    Nearest,
}

/// [`Handle`] 的内存分配，通常位于 GPU 内存中。
///
/// 渲染器往往会并发地解码并上传图像数据，以
/// 避免阻塞用户界面。这意味着当你在部件中使用
/// [`Handle`] 时，可能会有轻微的帧延迟，之后它才
/// 最终可见。如果你在为图像制作动画，这可能会导致
/// 不希望出现的闪烁。
///
/// 当你显式地获取一个 [`Allocation`] 时，你就得到了一个保证：
/// 使用 [`Handle`] 会在下一帧立即绘制出
/// 对应的 [`Image`]。
///
/// 只要你持有 [`Allocation`]，这个保证就有效。
/// 只有当你丢弃了它的所有克隆之后，渲染器才可能选择释放
/// [`Handle`] 的内存。请务必小心！
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A memory allocation of a [`Handle`], often in GPU memory.
///
/// Renderers tend to decode and upload image data concurrently to
/// avoid blocking the user interface. This means that when you use a
/// [`Handle`] in a widget, there may be a slight frame delay until it
/// is finally visible. If you are animating images, this can cause
/// undesirable flicker.
///
/// When you obtain an [`Allocation`] explicitly, you get the guarantee
/// that using a [`Handle`] will draw the corresponding [`Image`]
/// immediately in the next frame.
///
/// This guarantee is valid as long as you hold an [`Allocation`].
/// Only when you drop all its clones, the renderer may choose to free
/// the memory of the [`Handle`]. Be careful!
///
/// </details>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Allocation(Arc<Memory>);

/// [`Allocation`] 占用的某些内存。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Some memory taken by an [`Allocation`].
///
/// </details>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Memory {
    handle: Handle,
    size: Size<u32>,
}

impl Allocation {
    /// 返回 [`Allocation`] 的 [`Memory`] 的弱引用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a weak reference to the [`Memory`] of the [`Allocation`].
    ///
    /// </details>
    pub fn downgrade(&self) -> Weak<Memory> {
        Arc::downgrade(&self.0)
    }

    /// 把 [`Weak`] 内存引用升级为 [`Allocation`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Upgrades a [`Weak`] memory reference to an [`Allocation`].
    ///
    /// </details>
    pub fn upgrade(weak: &Weak<Memory>) -> Option<Allocation> {
        Weak::upgrade(weak).map(Allocation)
    }

    /// 返回此 [`Allocation`] 的 [`Handle`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Handle`] of this [`Allocation`].
    ///
    /// </details>
    pub fn handle(&self) -> &Handle {
        &self.0.handle
    }

    /// 返回此 [`Allocation`] 的图像的 [`Size`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Size`] of the image of this [`Allocation`].
    ///
    /// </details>
    pub fn size(&self) -> Size<u32> {
        self.0.size
    }
}

/// 为给定的句柄创建一个新的 [`Allocation`]。
///
/// 它只应该在渲染器实现的内部使用。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Allocation`] for the given handle.
///
/// This should only be used internally by renderer implementations.
///
/// </details>
///
/// # Safety
/// 只能在 [`Handle`] 已经在内存中分配之后创建。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Must only be created once the [`Handle`] is allocated in memory.
///
/// </details>
#[allow(unsafe_code)]
pub unsafe fn allocate(handle: &Handle, size: Size<u32>) -> Allocation {
    Allocation(Arc::new(Memory {
        handle: handle.clone(),
        size,
    }))
}

/// 可以渲染位图图像的 [`Renderer`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A [`Renderer`] that can render raster graphics.
///
/// </details>
///
/// [renderer]: crate::renderer
pub trait Renderer: crate::Renderer {
    /// 要显示的图像句柄。iced 公开了它自己的 [`Handle`] 默认实现
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The image Handle to be displayed. Iced exposes its own default implementation of a [`Handle`]
    ///
    /// </details>
    ///
    /// [`Handle`]: Self::Handle
    type Handle: Clone;

    /// 加载图像，并返回一个指向它的显式 [`Allocation`]。
    ///
    /// 如果图像尚未加载，此方法会阻塞！如果你想避免掉帧，
    /// 通常不应该在绘制逻辑中使用它。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Loads an image and returns an explicit [`Allocation`] to it.
    ///
    /// If the image is not already loaded, this method will block! You should
    /// generally not use it in drawing logic if you want to avoid frame drops.
    ///
    /// </details>
    fn load_image(&self, handle: &Self::Handle) -> Result<Allocation, Error>;

    /// 返回给定 [`Handle`] 对应图像的尺寸。
    ///
    /// 如果图像尚未加载，[`Renderer`] 可以选择返回
    /// `None`，在后台加载图像，然后触发一次重新布局。
    ///
    /// 如果你需要立即得到测量结果，可以考虑使用 [`Renderer::load_image`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the dimensions of an image for the given [`Handle`].
    ///
    /// If the image is not already loaded, the [`Renderer`] may choose to return
    /// `None`, load the image in the background, and then trigger a relayout.
    ///
    /// If you need a measurement right away, consider using [`Renderer::load_image`].
    ///
    /// </details>
    fn measure_image(&self, handle: &Self::Handle) -> Option<Size<u32>>;

    /// 在给定的 `bounds` 内绘制 [`Image`]。
    ///
    /// 如果图像尚未加载，[`Renderer`] 可以选择什么都不渲染，
    /// 在后台加载图像，然后触发一次重绘。
    ///
    /// 如果你需要立即绘制图像，可以考虑先使用 [`Renderer::load_image`]
    /// 并持有一个 [`Allocation`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws an [`Image`] inside the provided `bounds`.
    ///
    /// If the image is not already loaded, the [`Renderer`] may choose to render
    /// nothing, load the image in the background, and then trigger a redraw.
    ///
    /// If you need to draw an image right away, consider using [`Renderer::load_image`]
    /// and hold on to an [`Allocation`] first.
    ///
    /// </details>
    fn draw_image(
        &mut self,
        image: Image<Self::Handle>,
        bounds: Rectangle,
        clip_bounds: Rectangle,
    );
}

/// 图像加载错误。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An image loading error.
///
/// </details>
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    /// 图像数据无效，或者无法解码。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The image data was invalid or could not be decoded.
    ///
    /// </details>
    #[error("the image data was invalid or could not be decoded: {0}")]
    Invalid(Arc<dyn std::error::Error + Send + Sync>),
    /// 找不到图像文件。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The image file was not found.
    ///
    /// </details>
    #[error("the image file could not be opened: {0}")]
    Inaccessible(Arc<io::Error>),
    /// 不支持加载图像。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Loading images is unsupported.
    ///
    /// </details>
    #[error("loading images is unsupported")]
    Unsupported,
    /// 图像为空。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The image is empty.
    ///
    /// </details>
    #[error("the image is empty")]
    Empty,
    /// 内存不足，无法为图像分配内存。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Not enough memory to allocate the image.
    ///
    /// </details>
    #[error("not enough memory to allocate the image")]
    OutOfMemory,
}
