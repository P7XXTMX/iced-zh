//! 访问剪贴板。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Access the clipboard.
//!
//! </details>
use std::path::PathBuf;
use std::sync::Arc;

/// 一组剪贴板请求。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A set of clipboard requests.
///
/// </details>
#[derive(Debug, Clone)]
pub struct Clipboard {
    /// 运行时必须完成的读取请求。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The read requests the runtime must fulfill.
    ///
    /// </details>
    pub reads: Vec<Kind>,
    /// 运行时必须写入剪贴板的内容
    /// （如果有）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The content that must be written to the clipboard by the runtime,
    /// if any.
    ///
    /// </details>
    pub write: Option<Content>,
}

impl Clipboard {
    /// 创建一组新的空 [`Clipboard`] 请求。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new empty set of [`Clipboard`] requests.
    ///
    /// </details>
    pub fn new() -> Self {
        Self {
            reads: Vec::new(),
            write: None,
        }
    }

    /// 把当前的 [`Clipboard`] 请求与另一组请求合并。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Merges the current [`Clipboard`] requests with others.
    ///
    /// </details>
    pub fn merge(&mut self, other: &mut Self) {
        self.reads.append(&mut other.reads);
        self.write = other.write.take().or(self.write.take());
    }
}

impl Default for Clipboard {
    fn default() -> Self {
        Self::new()
    }
}

/// 剪贴板事件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A clipboard event.
///
/// </details>
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// 剪贴板已被读取。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The clipboard was read.
    ///
    /// </details>
    Read(Result<Arc<Content>, Error>),

    /// 剪贴板已被写入。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The clipboard was written.
    ///
    /// </details>
    Written(Result<(), Error>),
}

/// 某种剪贴板内容。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Some clipboard content.
///
/// </details>
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum Content {
    Text(String),
    Html(String),
    #[cfg(feature = "image")]
    Image(Image),
    Files(Vec<PathBuf>),
}

impl From<String> for Content {
    fn from(text: String) -> Self {
        Self::Text(text)
    }
}

#[cfg(feature = "image")]
impl From<Image> for Content {
    fn from(image: Image) -> Self {
        Self::Image(image)
    }
}

impl From<Vec<PathBuf>> for Content {
    fn from(files: Vec<PathBuf>) -> Self {
        Self::Files(files)
    }
}

/// 某个剪贴板 [`Content`] 的种类。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The kind of some clipboard [`Content`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum Kind {
    Text,
    Html,
    #[cfg(feature = "image")]
    Image,
    Files,
}

/// 剪贴板图像。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A clipboard image.
///
/// </details>
#[cfg(feature = "image")]
#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    /// 图像的像素，RGBA 格式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The pixels of the image in RGBA format.
    ///
    /// </details>
    pub rgba: crate::Bytes,

    /// 图像的物理 [`Size`](crate::Size)。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The physical [`Size`](crate::Size) of the image.
    ///
    /// </details>
    pub size: crate::Size<u32>,
}

/// 剪贴板错误。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A clipboard error.
///
/// </details>
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// 当前环境中的剪贴板不存在或无法访问。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The clipboard in the current environment is either not present or could not be accessed.
    ///
    /// </details>
    ClipboardUnavailable,

    /// 原生剪贴板正被其他程序占用，因此无法访问。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The native clipboard is not accessible due to being held by another party.
    ///
    /// </details>
    ClipboardOccupied,

    /// 剪贴板内容无法以所请求的格式提供。
    /// 原因可能是剪贴板为空，也可能是剪贴板内容的格式
    /// 与所请求的格式不兼容
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The clipboard contents were not available in the requested format.
    /// This could either be due to the clipboard being empty or the clipboard contents having
    /// an incompatible format to the requested one
    ///
    /// </details>
    ContentNotAvailable,

    /// 要传入或传出剪贴板的图像或文本无法
    /// 转换为合适的格式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The image or the text that was about the be transferred to/from the clipboard could not be
    /// converted to the appropriate format.
    ///
    /// </details>
    ConversionFailure,

    /// 不属于其他错误类型的任何错误。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Any error that doesn't fit the other error types.
    ///
    /// </details>
    Unknown {
        /// 仅用于帮助开发者的描述信息，不应依赖它在运行时
        /// 识别具体的错误情形。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// A description only meant to help the developer that should not be relied on as a
        /// means to identify an error case during runtime.
        ///
        /// </details>
        description: Arc<String>,
    },
}
