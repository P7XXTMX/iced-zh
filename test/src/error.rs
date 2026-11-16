use crate::Instruction;
use crate::ice;

use std::io;
use std::path::PathBuf;
use std::sync::Arc;

/// 测试错误。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A test error.
///
/// </details>
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    /// 没有找到与 [`Selector`](crate::Selector) 匹配的部件。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// No matching widget was found for the [`Selector`](crate::Selector).
    ///
    /// </details>
    #[error("no matching widget was found for the selector: {selector}")]
    SelectorNotFound {
        /// 选择器的描述。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// A description of the selector.
        ///
        /// </details>
        selector: String,
    },
    /// 有目标匹配，但它不可见。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A target matched, but is not visible.
    ///
    /// </details>
    #[error("the matching target is not visible: {target:?}")]
    TargetNotVisible {
        /// 目标
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The target
        ///
        /// </details>
        target: Arc<dyn std::fmt::Debug + Send + Sync>,
    },
    /// 某个 IO 操作失败。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An IO operation failed.
    ///
    /// </details>
    #[error("an IO operation failed: {0}")]
    IOFailed(Arc<io::Error>),
    /// 某个 PNG 图像解码失败。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The decoding of some PNG image failed.
    ///
    /// </details>
    #[error("the decoding of some PNG image failed: {0}")]
    PngDecodingFailed(Arc<png::DecodingError>),
    /// 某个 PNG 图像编码失败。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The encoding of some PNG image failed.
    ///
    /// </details>
    #[error("the encoding of some PNG image failed: {0}")]
    PngEncodingFailed(Arc<png::EncodingError>),
    /// 某个 [`Ice`](crate::Ice) 测试解析失败。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The parsing of an [`Ice`](crate::Ice) test failed.
    ///
    /// </details>
    #[error("the ice test ({file}) is invalid: {error}")]
    IceParsingFailed {
        /// 测试的路径。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The path of the test.
        ///
        /// </details>
        file: PathBuf,
        /// 解析错误。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The parse error.
        ///
        /// </details>
        error: ice::ParseError,
    },
    /// 某个 [`Ice`](crate::Ice) 测试执行失败。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The execution of an [`Ice`](crate::Ice) test failed.
    ///
    /// </details>
    #[error("the ice test ({file}) failed")]
    IceTestingFailed {
        /// 测试的路径。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The path of the test.
        ///
        /// </details>
        file: PathBuf,
        /// 失败的 [`Instruction`]。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The [`Instruction`] that failed.
        ///
        /// </details>
        instruction: Instruction,
    },
    /// 找不到程序的 [`Preset`](crate::program::Preset)。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Preset`](crate::program::Preset) of a program could not be found.
    ///
    /// </details>
    #[error("the preset \"{name}\" does not exist (available presets: {available:?})")]
    PresetNotFound {
        /// [`Preset`](crate::program::Preset) 的名称。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The name of the [`Preset`](crate::program::Preset).
        ///
        /// </details>
        name: String,
        /// 可用的预设集合。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The available set of presets.
        ///
        /// </details>
        available: Vec<String>,
    },
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::IOFailed(Arc::new(error))
    }
}

impl From<png::DecodingError> for Error {
    fn from(error: png::DecodingError) -> Self {
        Self::PngDecodingFailed(Arc::new(error))
    }
}

impl From<png::EncodingError> for Error {
    fn from(error: png::EncodingError) -> Self {
        Self::PngEncodingFailed(Arc::new(error))
    }
}
