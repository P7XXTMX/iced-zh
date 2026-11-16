//! 为应用程序的窗口设置图标。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Attach an icon to the window of your application.
//!
//! </details>
pub use crate::core::window::icon::*;

use crate::core::window::icon;

use std::io;

#[cfg(feature = "image")]
use std::path::Path;

/// 根据图像文件创建图标。
///
/// 如果运行时文件不存在，它会返回错误。你可能更愿意使用 [`from_file_data`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates an icon from an image file.
///
/// This will return an error in case the file is missing at run-time. You may prefer [`from_file_data`] instead.
///
/// </details>
#[cfg(feature = "image")]
pub fn from_file<P: AsRef<Path>>(icon_path: P) -> Result<Icon, Error> {
    let icon = image::ImageReader::open(icon_path)?.decode()?.to_rgba8();

    Ok(icon::from_rgba(icon.to_vec(), icon.width(), icon.height())?)
}

/// 根据图像文件的内容创建图标。
///
/// 这些内容可以在编译期嵌入你的应用程序，例如使用 `include_bytes!` 宏。
/// 你可以显式传入文件格式。否则会在运行时猜测文件格式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates an icon from the content of an image file.
///
/// This content can be included in your application at compile-time, e.g. using the `include_bytes!` macro.
/// You can pass an explicit file format. Otherwise, the file format will be guessed at runtime.
///
/// </details>
#[cfg(feature = "image")]
pub fn from_file_data(
    data: &[u8],
    explicit_format: Option<image::ImageFormat>,
) -> Result<Icon, Error> {
    let mut icon = image::ImageReader::new(std::io::Cursor::new(data));

    let icon_with_format = match explicit_format {
        Some(format) => {
            icon.set_format(format);
            icon
        }
        None => icon.with_guessed_format()?,
    };

    let pixels = icon_with_format.decode()?.to_rgba8();

    Ok(icon::from_rgba(
        pixels.to_vec(),
        pixels.width(),
        pixels.height(),
    )?)
}

/// 创建 [`Icon`] 时产生的错误。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An error produced when creating an [`Icon`].
///
/// </details>
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// [`Icon`] 无效。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Icon`] is not valid.
    ///
    /// </details>
    #[error("The icon is invalid: {0}")]
    InvalidError(#[from] icon::Error),

    /// 底层操作系统创建图标失败。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The underlying OS failed to create the icon.
    ///
    /// </details>
    #[error("The underlying OS failed to create the window icon: {0}")]
    OsError(#[from] io::Error),

    /// `image` crate 报告了错误。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The `image` crate reported an error.
    ///
    /// </details>
    #[cfg(feature = "image")]
    #[error("Unable to create icon from a file: {0}")]
    ImageError(#[from] image::error::ImageError),
}
