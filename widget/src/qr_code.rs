//! 二维码用一种二维矩阵条码来展示信息。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! QR codes display information in a type of two-dimensional matrix barcode.
//!
//! </details>
//!
//! # 示例（Example）
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
//! # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
//! #
//! use iced::widget::qr_code;
//!
//! struct State {
//!    data: qr_code::Data,
//! }
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     // ...
//! }
//!
//! fn view(state: &State) -> Element<'_, Message> {
//!     qr_code(&state.data).into()
//! }
//! ```
use crate::Renderer;
use crate::canvas;
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer::{self, Renderer as _};
use crate::core::widget::tree::{self, Tree};
use crate::core::{
    Color, Element, Layout, Length, Pixels, Point, Rectangle, Size, Theme,
    Vector, Widget,
};

use std::cell::RefCell;
use thiserror::Error;

const DEFAULT_CELL_SIZE: f32 = 4.0;
const QUIET_ZONE: usize = 2;

/// 一种由排列在网格中的方块组成的矩阵条码，
/// 可以被摄像头等成像设备读取。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A type of matrix barcode consisting of squares arranged in a grid which
/// can be read by an imaging device, such as a camera.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// #
/// use iced::widget::qr_code;
///
/// struct State {
///    data: qr_code::Data,
/// }
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     qr_code(&state.data).into()
/// }
/// ```
pub struct QRCode<'a, Theme = crate::Theme>
where
    Theme: Catalog,
{
    data: &'a Data,
    cell_size: f32,
    class: Theme::Class<'a>,
}

impl<'a, Theme> QRCode<'a, Theme>
where
    Theme: Catalog,
{
    /// 使用提供的 [`Data`] 创建一个新的 [`QRCode`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`QRCode`] with the provided [`Data`].
    ///
    /// </details>
    pub fn new(data: &'a Data) -> Self {
        Self {
            data,
            cell_size: DEFAULT_CELL_SIZE,
            class: Theme::default(),
        }
    }

    /// 设置 [`QRCode`] 网格单元格方块的大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the size of the squares of the grid cell of the [`QRCode`].
    ///
    /// </details>
    pub fn cell_size(mut self, cell_size: impl Into<Pixels>) -> Self {
        self.cell_size = cell_size.into().0;
        self
    }

    /// 设置整个 [`QRCode`] 的大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the size of the entire [`QRCode`].
    ///
    /// </details>
    pub fn total_size(mut self, total_size: impl Into<Pixels>) -> Self {
        self.cell_size =
            total_size.into().0 / (self.data.width + 2 * QUIET_ZONE) as f32;

        self
    }

    /// 设置 [`QRCode`] 的样式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style of the [`QRCode`].
    ///
    /// </details>
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// 设置 [`QRCode`] 的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style class of the [`QRCode`].
    ///
    /// </details>
    #[cfg(feature = "advanced")]
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<Message, Theme> Widget<Message, Theme, Renderer> for QRCode<'_, Theme>
where
    Theme: Catalog,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        let side_length =
            (self.data.width + 2 * QUIET_ZONE) as f32 * self.cell_size;

        layout::Node::new(Size::new(side_length, side_length))
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();

        let bounds = layout.bounds();
        let side_length = self.data.width + 2 * QUIET_ZONE;

        let style = theme.style(&self.class);
        let mut last_style = state.last_style.borrow_mut();

        if Some(style) != *last_style {
            self.data.cache.clear();

            *last_style = Some(style);
        }

        // Reuse cache if possible
        // 尽可能复用缓存
        let geometry = self.data.cache.draw(renderer, bounds.size(), |frame| {
            // Scale units to cell size
            // 把单位缩放到单元格大小
            frame.scale(self.cell_size);

            // Draw background
            // 绘制背景
            frame.fill_rectangle(
                Point::ORIGIN,
                Size::new(side_length as f32, side_length as f32),
                style.background,
            );

            // Avoid drawing on the quiet zone
            // 避免在静区上绘制
            frame.translate(Vector::new(QUIET_ZONE as f32, QUIET_ZONE as f32));

            // Draw contents
            // 绘制内容
            self.data
                .contents
                .iter()
                .enumerate()
                .filter(|(_, value)| **value == qrcode::Color::Dark)
                .for_each(|(index, _)| {
                    let row = index / self.data.width;
                    let column = index % self.data.width;

                    frame.fill_rectangle(
                        Point::new(column as f32, row as f32),
                        Size::UNIT,
                        style.cell,
                    );
                });
        });

        renderer.with_translation(
            bounds.position() - Point::ORIGIN,
            |renderer| {
                use crate::graphics::geometry::Renderer as _;

                renderer.draw_geometry(geometry);
            },
        );
    }
}

impl<'a, Message, Theme> From<QRCode<'a, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
{
    fn from(qr_code: QRCode<'a, Theme>) -> Self {
        Self::new(qr_code)
    }
}

/// [`QRCode`] 的数据。
///
/// 它存储将要显示的内容。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The data of a [`QRCode`].
///
/// It stores the contents that will be displayed.
///
/// </details>
#[derive(Debug)]
pub struct Data {
    contents: Vec<qrcode::Color>,
    width: usize,
    cache: canvas::Cache<Renderer>,
}

impl Data {
    /// 使用提供的数据创建一个新的 [`Data`]。
    ///
    /// 这个方法使用 [`ErrorCorrection::Medium`]，并选择能显示
    /// 这些数据的最小尺寸。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Data`] with the provided data.
    ///
    /// This method uses an [`ErrorCorrection::Medium`] and chooses the smallest
    /// size to display the data.
    ///
    /// </details>
    pub fn new(data: impl AsRef<[u8]>) -> Result<Self, Error> {
        let encoded = qrcode::QrCode::new(data)?;

        Ok(Self::build(encoded))
    }

    /// 使用提供的 [`ErrorCorrection`] 创建一个新的 [`Data`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Data`] with the provided [`ErrorCorrection`].
    ///
    /// </details>
    pub fn with_error_correction(
        data: impl AsRef<[u8]>,
        error_correction: ErrorCorrection,
    ) -> Result<Self, Error> {
        let encoded = qrcode::QrCode::with_error_correction_level(
            data,
            error_correction.into(),
        )?;

        Ok(Self::build(encoded))
    }

    /// 使用提供的 [`Version`] 和 [`ErrorCorrection`]
    /// 创建一个新的 [`Data`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Data`] with the provided [`Version`] and
    /// [`ErrorCorrection`].
    ///
    /// </details>
    pub fn with_version(
        data: impl AsRef<[u8]>,
        version: Version,
        error_correction: ErrorCorrection,
    ) -> Result<Self, Error> {
        let encoded = qrcode::QrCode::with_version(
            data,
            version.into(),
            error_correction.into(),
        )?;

        Ok(Self::build(encoded))
    }

    fn build(encoded: qrcode::QrCode) -> Self {
        let width = encoded.width();
        let contents = encoded.into_colors();

        Self {
            contents,
            width,
            cache: canvas::Cache::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// [`QRCode`] 的尺寸。
///
/// 版本越高，单元格网格越大，因此 [`QRCode`]
/// 能携带的信息也越多。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The size of a [`QRCode`].
///
/// The higher the version the larger the grid of cells, and therefore the more
/// information the [`QRCode`] can carry.
///
/// </details>
pub enum Version {
    /// 普通二维码版本。应在 1 到 40 之间。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A normal QR code version. It should be between 1 and 40.
    ///
    /// </details>
    Normal(u8),

    /// 微型二维码版本。应在 1 到 4 之间。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A micro QR code version. It should be between 1 and 4.
    ///
    /// </details>
    Micro(u8),
}

impl From<Version> for qrcode::Version {
    fn from(version: Version) -> Self {
        match version {
            Version::Normal(v) => qrcode::Version::Normal(i16::from(v)),
            Version::Micro(v) => qrcode::Version::Micro(i16::from(v)),
        }
    }
}

/// 纠错级别。
///
/// 它控制在仍然能够恢复原始信息的前提下，
/// 可以损坏的数据量。
///
/// 纠错级别越高，允许损坏的数据越多。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The error correction level.
///
/// It controls the amount of data that can be damaged while still being able
/// to recover the original information.
///
/// A higher error correction level allows for more corrupted data.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCorrection {
    /// 低纠错级别。可以恢复 7% 的数据。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Low error correction. 7% of the data can be restored.
    ///
    /// </details>
    Low,
    /// 中纠错级别。可以恢复 15% 的数据。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Medium error correction. 15% of the data can be restored.
    ///
    /// </details>
    Medium,
    /// 四分位纠错级别。可以恢复 25% 的数据。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Quartile error correction. 25% of the data can be restored.
    ///
    /// </details>
    Quartile,
    /// 高纠错级别。可以恢复 30% 的数据。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// High error correction. 30% of the data can be restored.
    ///
    /// </details>
    High,
}

impl From<ErrorCorrection> for qrcode::EcLevel {
    fn from(ec_level: ErrorCorrection) -> Self {
        match ec_level {
            ErrorCorrection::Low => qrcode::EcLevel::L,
            ErrorCorrection::Medium => qrcode::EcLevel::M,
            ErrorCorrection::Quartile => qrcode::EcLevel::Q,
            ErrorCorrection::High => qrcode::EcLevel::H,
        }
    }
}

/// 为 [`QRCode`] 构建 [`Data`] 时发生的错误。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An error that occurred when building a [`Data`] for a [`QRCode`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum Error {
    /// 数据太长，无法用所选的 [`Version`] 编码为二维码。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The data is too long to encode in a QR code for the chosen [`Version`].
    ///
    /// </details>
    #[error(
        "The data is too long to encode in a QR code for the chosen version"
    )]
    DataTooLong,

    /// 所选的 [`Version`] 与 [`ErrorCorrection`] 的组合无效。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The chosen [`Version`] and [`ErrorCorrection`] combination is invalid.
    ///
    /// </details>
    #[error(
        "The chosen version and error correction level combination is invalid."
    )]
    InvalidVersion,

    /// 提供的数据中有一个或多个字符不被所选的
    /// [`Version`] 支持。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// One or more characters in the provided data are not supported by the
    /// chosen [`Version`].
    ///
    /// </details>
    #[error(
        "One or more characters in the provided data are not supported by the \
        chosen version"
    )]
    UnsupportedCharacterSet,

    /// 所选的 ECI 指示符无效。有效的指示符应
    /// 在 0 到 999999 之间。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The chosen ECI designator is invalid. A valid designator should be
    /// between 0 and 999999.
    ///
    /// </details>
    #[error(
        "The chosen ECI designator is invalid. A valid designator should be \
        between 0 and 999999."
    )]
    InvalidEciDesignator,

    /// 发现了不属于该字符集的字符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A character that does not belong to the character set was found.
    ///
    /// </details>
    #[error("A character that does not belong to the character set was found")]
    InvalidCharacter,
}

impl From<qrcode::types::QrError> for Error {
    fn from(error: qrcode::types::QrError) -> Self {
        use qrcode::types::QrError;

        match error {
            QrError::DataTooLong => Error::DataTooLong,
            QrError::InvalidVersion => Error::InvalidVersion,
            QrError::UnsupportedCharacterSet => Error::UnsupportedCharacterSet,
            QrError::InvalidEciDesignator => Error::InvalidEciDesignator,
            QrError::InvalidCharacter => Error::InvalidCharacter,
        }
    }
}

#[derive(Default)]
struct State {
    last_style: RefCell<Option<Style>>,
}

/// 二维码的外观。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The appearance of a QR code.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// 二维码数据单元格的颜色
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The color of the QR code data cells
    ///
    /// </details>
    pub cell: Color,
    /// 二维码背景的颜色
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The color of the QR code background
    ///
    /// </details>
    pub background: Color,
}

/// [`QRCode`] 的主题样式目录。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The theme catalog of a [`QRCode`].
///
/// </details>
pub trait Catalog {
    /// [`Catalog`] 的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The item class of the [`Catalog`].
    ///
    /// </details>
    type Class<'a>;

    /// [`Catalog`] 产生的默认样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default class produced by the [`Catalog`].
    ///
    /// </details>
    fn default<'a>() -> Self::Class<'a>;

    /// 某个样式类在给定交互状态下的 [`Style`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Style`] of a class with the given status.
    ///
    /// </details>
    fn style(&self, class: &Self::Class<'_>) -> Style;
}

/// [`QRCode`] 的样式函数。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A styling function for a [`QRCode`].
///
/// </details>
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

/// [`QRCode`] 的默认样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The default style of a [`QRCode`].
///
/// </details>
pub fn default(theme: &Theme) -> Style {
    let palette = theme.palette();

    Style {
        cell: palette.text,
        background: palette.background,
    }
}
