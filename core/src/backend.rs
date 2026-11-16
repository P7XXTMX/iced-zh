//! 图形后端用于辅助把计算机图形渲染到显示器上。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Graphical backends are designed to aid in rendering computer graphics to a monitor.
//!
//! </details>
use std::env;
use std::fmt;

/// 图形后端。
///
/// 可以通过设置 `ICED_BACKEND` 环境变量来覆盖默认策略。
/// 如果你使用的是默认渲染器，可用的选项有：
///   - `wgpu`：硬件加速后端。
///   - `tiny-skia`：基于软件的后端。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A graphical backend.
///
/// You can override the default strategy by setting the `ICED_BACKEND` environment variable.
/// If you are using the default renderer, the available options are:
///   - `wgpu` for the hardware accelerated backend.
///   - `tiny-skia` for the software-based backend.
///
/// </details>
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Backend {
    /// 自动检测并选择可用的最佳 [`Backend`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Auto-detect and choose the best available [`Backend`].
    ///
    /// </details>
    Best,
    /// 使用给定 [`Api`] 的硬件加速图形后端。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Hardware accelerated graphics backend with the given [`Api`].
    ///
    /// </details>
    Hardware(Api),
    /// 软件图形后端；比基于硬件的后端慢不少，但兼容性更好。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sofware graphics backend; quite slower than hardware-based backends, but more compatible.
    ///
    /// </details>
    Software,
    /// 具有给定名称的自定义渲染后端。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A custom rendering backend with the given name.
    ///
    /// </details>
    Custom(String),
}

impl Backend {
    /// [`Backend`] 所有已知的可能组合。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// All the possible known combinations of [`Backend`].
    ///
    /// </details>
    pub const ALL: &[Self] = &[
        Self::Best,
        Self::Hardware(Api::Best),
        Self::Hardware(Api::Vulkan),
        Self::Hardware(Api::Metal),
        Self::Hardware(Api::DirectX12),
        Self::Hardware(Api::OpenGL),
        Self::Hardware(Api::WebGPU),
        Self::Software,
    ];

    /// 如果 [`Backend`] 是 [`Backend::Hardware`]，返回 true。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns true if the [`Backend`] is [`Backend::Hardware`].
    ///
    /// </details>
    pub fn hardware(&self) -> Option<Api> {
        match self {
            Backend::Hardware(api) => Some(*api),
            _ => None,
        }
    }

    /// 如果 [`Backend`] 是 [`Backend::Software`]，返回 true。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns true if the [`Backend`] is [`Backend::Software`].
    ///
    /// </details>
    pub fn is_software(&self) -> bool {
        matches!(self, Self::Software)
    }

    /// 如果 [`Backend`] 是 [`Backend::Best`] 或与给定的名称匹配，返回 true。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns true if the [`Backend`] is [`Backend::Best`] or matches the given name.
    ///
    /// </details>
    pub fn matches(&self, target: &str) -> bool {
        match self {
            Backend::Best => true,
            Backend::Custom(name) => name == target || name == &target.replace("-", "_"),
            _ => false,
        }
    }
}

impl From<String> for Backend {
    fn from(backend: String) -> Self {
        Self::Custom(backend)
    }
}

impl From<&str> for Backend {
    fn from(backend: &str) -> Self {
        Self::Custom(backend.to_owned())
    }
}

impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Backend::Best => write!(f, "Best Backend"),
            Backend::Hardware(api) => write!(f, "Hardware Backend ({api})"),
            Backend::Software => write!(f, "Software Backend"),
            Backend::Custom(name) => write!(f, "Custom Backend ({name})"),
        }
    }
}

/// 硬件图形 API。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A hardware graphics API.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Api {
    /// 自动检测并选择可用的最佳图形 [`Api`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Auto-detect and choose the best available graphics [`Api`].
    ///
    /// </details>
    #[default]
    Best,
    /// Vulkan API（Windows、Linux、Android，以及通过 vulkan-portability/MoltenVK 支持的 MacOS）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Vulkan API (Windows, Linux, Android, MacOS via vulkan-portability/MoltenVK)
    ///
    /// </details>
    Vulkan,
    /// Metal API（Apple 平台）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Metal API (Apple platforms)
    ///
    /// </details>
    Metal,
    /// Direct3D-12（Windows）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Direct3D-12 (Windows)
    ///
    /// </details>
    DirectX12,
    /// OpenGL 3.3+（Windows）、OpenGL ES 3.0+（Linux、Android，以及通过 Angle 支持的 MacOS）和 WebGL2
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// OpenGL 3.3+ (Windows), OpenGL ES 3.0+ (Linux, Android, MacOS via Angle), and WebGL2
    ///
    /// </details>
    OpenGL,
    /// WebGPU（Web 浏览器）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// WebGPU (Web Browser)
    ///
    /// </details>
    WebGPU,
}

impl Default for Backend {
    fn default() -> Self {
        let Ok(backend) = env::var("ICED_BACKEND") else {
            return Self::Best;
        };

        Self::Custom(backend.to_owned())
    }
}

impl fmt::Display for Api {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Api::Best => "Best",
            Api::Vulkan => "Vulkan",
            Api::Metal => "Metal",
            Api::DirectX12 => "DirectX 12",
            Api::OpenGL => "OpenGL",
            Api::WebGPU => "WebGPU",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Default)]
/// 图形适配器的功耗偏好。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The power-usage preference for graphics adapters.
///
/// </details>
pub enum PowerPreference {
    /// 选择适配器时没有功耗偏好。
    ///
    /// 这是默认值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// No power preference in which adapter should be chosen.
    ///
    /// This is the default.
    ///
    /// </details>
    #[default]
    None,

    /// 后端会优先选择低功耗适配器，而不是高性能适配器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The backend will prefer low power adapters over high performance ones.
    ///
    /// </details>
    LowPower,

    /// 后端会优先选择高性能适配器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The backend will prefer adapters that are high performance.
    ///
    /// </details>
    HighPerformance,
}

/// 用于配置 [`Backend`] 的设置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The settings usted to configure a [`Backend`].
///
/// </details>
#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    /// 要使用的图形后端。
    ///
    /// 默认为 [`Backend::Best`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The graphical backend to use.
    ///
    /// By default, it is [`Backend::Best`].
    ///
    /// </details>
    pub backend: Backend,

    /// 后端的 [`PowerPreference`]。
    ///
    /// 默认为 [`PowerPreference::None`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`PowerPreference`] of the backend.
    ///
    /// By default, it is [`PowerPreference::None`].
    ///
    /// </details>
    pub power_preference: PowerPreference,

    /// 如果设置为 true，渲染器会尝试对某些图元
    /// 进行抗锯齿。
    ///
    /// 启用后，某些部件（例如 `Canvas`）的显示效果会更平滑，
    /// 但会有性能开销。
    ///
    /// 默认为 `true`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// If set to true, the renderer will try to perform antialiasing for some
    /// primitives.
    ///
    /// Enabling it can produce a smoother result in some widgets, like the
    /// `Canvas`, at a performance cost.
    ///
    /// By default, it is `true`.
    ///
    /// </details>
    pub antialiasing: bool,

    /// 是否同步帧。
    ///
    /// 默认为 `true`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether or not to synchronize frames.
    ///
    /// By default, it is `true`.
    ///
    /// </details>
    pub vsync: bool,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            backend: Backend::Best,
            antialiasing: true,
            vsync: true,
            power_preference: PowerPreference::None,
        }
    }
}

impl From<&crate::Settings> for Settings {
    fn from(settings: &crate::Settings) -> Self {
        Self {
            backend: settings.backend.clone(),
            antialiasing: settings.antialiasing,
            vsync: settings.vsync,
            power_preference: settings.power_preference,
        }
    }
}

/// 创建应用程序的图形上下文时发生的错误。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An error that occurred while creating an application's graphical context.
///
/// </details>
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// 不支持所请求的后端版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The requested backend version is not supported.
    ///
    /// </details>
    #[error("the requested backend version is not supported")]
    VersionNotSupported,

    /// 找不到任何符合条件的像素格式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Failed to find any pixel format that matches the criteria.
    ///
    /// </details>
    #[error("failed to find any pixel format that matches the criteria")]
    NoAvailablePixelFormat,

    /// 找不到合适的图形适配器或设备。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A suitable graphics adapter or device could not be found.
    ///
    /// </details>
    #[error("a suitable graphics adapter could not be found: {reason}")]
    GraphicsAdapterNotFound {
        /// 发生错误的后端的名称
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The name of the backend where the error happened
        ///
        /// </details>
        backend: &'static str,
        /// 这个后端无法使用的原因
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The reason why this backend could not be used
        ///
        /// </details>
        reason: Reason,
    },

    /// 上下文的内部后端发生了错误
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An error occurred in the context's internal backend
    ///
    /// </details>
    #[error("an error occurred in the context's internal backend")]
    BackendError(String),

    /// 发生了多个错误
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Multiple errors occurred
    ///
    /// </details>
    #[error("multiple errors occurred:\n{}", error_list(.0))]
    List(Vec<Self>),
}

/// 找不到图形适配器的原因
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The reason why a graphics adapter could not be found
///
/// </details>
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reason {
    /// 后端与偏好不匹配
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The backend did not match the preference
    ///
    /// </details>
    DidNotMatch {
        /// 偏好的后端
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The preferred backend
        ///
        /// </details>
        preferred_backend: Backend,
    },
    /// 创建后端的请求失败
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The request to create the backend failed
    ///
    /// </details>
    RequestFailed(String),
}

impl fmt::Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Reason::DidNotMatch { preferred_backend } => {
                write!(
                    f,
                    "the backend did not match the preference: {preferred_backend}"
                )
            }
            Reason::RequestFailed(error) => f.write_str(error),
        }
    }
}

fn error_list(errors: &Vec<Error>) -> String {
    let mut list = String::new();

    for error in errors {
        list.push_str("- ");
        list.push_str(&error.to_string());
        list.push('\n');
    }

    list
}
