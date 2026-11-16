//! 把窗口与渲染器连接起来。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Connect a window with a renderer.
//!
//! </details>
use crate::core::Color;
use crate::core::backend;
use crate::core::renderer;
use crate::graphics::color;
use crate::graphics::compositor;
use crate::graphics::{self, Antialiasing, Shell, Viewport};
use crate::{Engine, Renderer};

/// 由 `wgpu` 驱动的 iced 窗口图形后端。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A window graphics backend for iced powered by `wgpu`.
///
/// </details>
pub struct Compositor {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    format: wgpu::TextureFormat,
    alpha_mode: wgpu::CompositeAlphaMode,
    engine: Engine,
    settings: Settings,
}

/// 合成器错误。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A compositor error.
///
/// </details>
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    /// 表面创建失败。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The surface creation failed.
    ///
    /// </details>
    #[error("the surface creation failed: {0}")]
    SurfaceCreationFailed(#[from] wgpu::CreateSurfaceError),
    /// 表面不兼容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The surface is not compatible.
    ///
    /// </details>
    #[error("the surface is not compatible")]
    IncompatibleSurface,
    /// 没有找到符合所请求选项的适配器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// No adapter was found for the options requested.
    ///
    /// </details>
    #[error("no adapter was found for the options requested: {0:?}")]
    NoAdapterFound(String),
    /// 没有任何设备请求成功。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// No device request succeeded.
    ///
    /// </details>
    #[error("no device request succeeded: {0:?}")]
    RequestDeviceFailed(Vec<(wgpu::Limits, wgpu::RequestDeviceError)>),
}

impl From<Error> for backend::Error {
    fn from(error: Error) -> Self {
        Self::GraphicsAdapterNotFound {
            backend: "wgpu",
            reason: backend::Reason::RequestFailed(error.to_string()),
        }
    }
}

impl Compositor {
    /// 使用给定的 [`Settings`] 请求一个新的 [`Compositor`]。
    ///
    /// 如果找不到兼容的图形适配器，返回 `None`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Requests a new [`Compositor`] with the given [`Settings`].
    ///
    /// Returns `None` if no compatible graphics adapter could be found.
    ///
    /// </details>
    pub async fn request(
        settings: Settings,
        display: impl compositor::Display,
        compatible_window: impl compositor::Window,
        shell: Shell,
    ) -> Result<Self, Error> {
        let instance = wgpu::util::new_instance_with_webgpu_detection(wgpu::InstanceDescriptor {
            backends: settings.backends,
            flags: if cfg!(feature = "strict-assertions") {
                wgpu::InstanceFlags::debugging()
            } else {
                wgpu::InstanceFlags::empty()
            },
            ..wgpu::InstanceDescriptor::new_with_display_handle(Box::new(display))
        })
        .await;

        log::info!("{settings:#?}");

        #[cfg(not(target_arch = "wasm32"))]
        if log::max_level() >= log::LevelFilter::Info {
            let available_adapters: Vec<_> = instance
                .enumerate_adapters(settings.backends)
                .await
                .iter()
                .map(wgpu::Adapter::get_info)
                .collect();
            log::info!("Available adapters: {available_adapters:#?}");
        }

        #[allow(unsafe_code)]
        let compatible_surface = instance
            .create_surface(wgpu::SurfaceTarget::Window(Box::new(compatible_window)))
            .ok();

        let power_preference = match settings.power_preference {
            backend::PowerPreference::None => wgpu::PowerPreference::None,
            backend::PowerPreference::LowPower => wgpu::PowerPreference::LowPower,
            backend::PowerPreference::HighPerformance => wgpu::PowerPreference::HighPerformance,
        };

        let adapter_options = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::from_env().unwrap_or(power_preference),
            compatible_surface: compatible_surface.as_ref(),
            force_fallback_adapter: false,
            apply_limit_buckets: false,
        };

        let adapter = instance
            .request_adapter(&adapter_options)
            .await
            .map_err(|_error| Error::NoAdapterFound(format!("{adapter_options:?}")))?;

        log::info!("Selected: {:#?}", adapter.get_info());

        let (format, alpha_mode) = compatible_surface
            .as_ref()
            .and_then(|surface| {
                let capabilities = surface.get_capabilities(&adapter);

                let formats = capabilities.formats.iter().copied();

                log::info!("Available formats: {formats:#?}");

                const BLACKLIST: &[wgpu::TextureFormat] = &[
                    wgpu::TextureFormat::Rgb10a2Unorm,
                    wgpu::TextureFormat::Rgb10a2Uint,
                    wgpu::TextureFormat::Rgba16Float,
                ];

                let mut formats = formats.filter(|format| {
                    format.required_features() == wgpu::Features::empty()
                        && !BLACKLIST.contains(format)
                });

                let format = if color::GAMMA_CORRECTION {
                    formats.find(wgpu::TextureFormat::is_srgb)
                } else {
                    formats.find(|format| !wgpu::TextureFormat::is_srgb(format))
                };

                let format = format.or_else(|| {
                    log::warn!("No format found!");

                    capabilities.formats.first().copied()
                });

                let alpha_modes = capabilities.alpha_modes;

                log::info!("Available alpha modes: {alpha_modes:#?}");

                let preferred_alpha =
                    if alpha_modes.contains(&wgpu::CompositeAlphaMode::PreMultiplied) {
                        wgpu::CompositeAlphaMode::PreMultiplied
                    } else {
                        wgpu::CompositeAlphaMode::Auto
                    };

                format.zip(Some(preferred_alpha))
            })
            .ok_or(Error::IncompatibleSurface)?;

        log::info!("Selected format: {format:?} with alpha mode: {alpha_mode:?}");

        #[cfg(target_arch = "wasm32")]
        let limits = [wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits())];

        #[cfg(not(target_arch = "wasm32"))]
        let limits = [
            wgpu::Limits::default().using_resolution(adapter.limits()),
            wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits()),
        ];

        let limits = limits.into_iter().map(|limits| wgpu::Limits {
            max_non_sampler_bindings: 2048,
            ..limits
        });

        // Request SHADER_F16 only if the adapter supports it (e.g., not available in WebGL2)
        // 只有适配器支持时才请求 SHADER_F16（例如 WebGL2 中不可用）
        let required_features = if adapter.features().contains(wgpu::Features::SHADER_F16) {
            wgpu::Features::SHADER_F16
        } else {
            wgpu::Features::empty()
        };

        let mut errors = Vec::new();

        for required_limits in limits {
            let result = adapter
                .request_device(&wgpu::DeviceDescriptor {
                    label: Some("iced_wgpu::window::compositor device descriptor"),
                    required_features,
                    required_limits: required_limits.clone(),
                    memory_hints: wgpu::MemoryHints::MemoryUsage,
                    trace: wgpu::Trace::Off,
                    experimental_features: wgpu::ExperimentalFeatures::disabled(),
                })
                .await;

            match result {
                Ok((device, queue)) => {
                    let engine = Engine::new(
                        &adapter,
                        device,
                        queue,
                        format,
                        settings.antialiasing,
                        shell,
                    );

                    return Ok(Compositor {
                        instance,
                        adapter,
                        format,
                        alpha_mode,
                        engine,
                        settings,
                    });
                }
                Err(error) => {
                    errors.push((required_limits, error));
                }
            }
        }

        Err(Error::RequestDeviceFailed(errors))
    }
}

/// 使用给定的 [`Settings`] 和窗口创建一个 [`Compositor`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a [`Compositor`] with the given [`Settings`] and window.
///
/// </details>
pub async fn new(
    settings: Settings,
    display: impl compositor::Display,
    compatible_window: impl compositor::Window,
    shell: Shell,
) -> Result<Compositor, Error> {
    Compositor::request(settings, display, compatible_window, shell).await
}

/// 使用给定的 [`Compositor`] 呈现给定的图元。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Presents the given primitives with the given [`Compositor`].
///
/// </details>
pub fn present(
    renderer: &mut Renderer,
    surface: &mut wgpu::Surface<'static>,
    viewport: &Viewport,
    background_color: Color,
    on_pre_present: impl FnOnce(),
) -> Result<(), compositor::SurfaceError> {
    match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame) => {
            let view = &frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let _submission = renderer.present(
                Some(background_color),
                frame.texture.format(),
                view,
                viewport,
            );

            // Present the frame
            // 呈现这一帧
            on_pre_present();
            renderer.engine.queue.present(frame);

            Ok(())
        }
        wgpu::CurrentSurfaceTexture::Suboptimal(_) | wgpu::CurrentSurfaceTexture::Outdated => {
            Err(compositor::SurfaceError::Outdated)
        }
        wgpu::CurrentSurfaceTexture::Timeout => Err(compositor::SurfaceError::Timeout),
        wgpu::CurrentSurfaceTexture::Occluded => Err(compositor::SurfaceError::Occluded),
        wgpu::CurrentSurfaceTexture::Lost => Err(compositor::SurfaceError::Lost),
        wgpu::CurrentSurfaceTexture::Validation => Err(compositor::SurfaceError::Other),
    }
}

impl graphics::Compositor for Compositor {
    type Renderer = Renderer;
    type Surface = wgpu::Surface<'static>;

    async fn new(
        settings: backend::Settings,
        display: impl compositor::Display,
        compatible_window: impl compositor::Window,
        shell: Shell,
    ) -> Result<Self, backend::Error> {
        if settings.backend.hardware().is_none() && !settings.backend.matches("wgpu") {
            return Err(backend::Error::GraphicsAdapterNotFound {
                backend: "wgpu",
                reason: backend::Reason::DidNotMatch {
                    preferred_backend: settings.backend,
                },
            });
        }

        let mut settings = Settings::from(settings);

        if let Some(backends) = wgpu::Backends::from_env() {
            settings.backends = backends;
        }

        if let Some(present_mode) = present_mode_from_env() {
            settings.present_mode = present_mode;
        }

        Ok(new(settings, display, compatible_window, shell).await?)
    }

    fn create_renderer(&self, settings: renderer::Settings) -> Self::Renderer {
        Renderer::new(self.engine.clone(), settings)
    }

    fn create_surface(&mut self, window: impl compositor::Window) -> Self::Surface {
        self.instance
            .create_surface(wgpu::SurfaceTarget::Window(Box::new(window)))
            .expect("Create surface")
    }

    fn configure_surface(&mut self, surface: &mut Self::Surface, width: u32, height: u32) {
        surface.configure(
            &self.engine.device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: self.format,
                present_mode: self.settings.present_mode,
                width,
                height,
                alpha_mode: self.alpha_mode,
                view_formats: vec![],
                desired_maximum_frame_latency: 1,
                color_space: wgpu::SurfaceColorSpace::Auto,
            },
        );
    }

    fn information(&self) -> compositor::Information {
        let information = self.adapter.get_info();

        compositor::Information {
            adapter: information.name,
            backend: format!("{:?}", information.backend),
        }
    }

    fn present(
        &mut self,
        renderer: &mut Self::Renderer,
        surface: &mut Self::Surface,
        viewport: &Viewport,
        background_color: Color,
        on_pre_present: impl FnOnce(),
    ) -> Result<(), compositor::SurfaceError> {
        present(
            renderer,
            surface,
            viewport,
            background_color,
            on_pre_present,
        )
    }

    fn screenshot(
        &mut self,
        renderer: &mut Self::Renderer,
        viewport: &Viewport,
        background_color: Color,
    ) -> Vec<u8> {
        renderer.screenshot(viewport, background_color)
    }
}

/// [`Compositor`] 的设置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The settings of a [`Compositor`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    /// [`Renderer`] 的呈现模式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The present mode of the [`Renderer`].
    ///
    /// </details>
    ///
    /// [`Renderer`]: crate::Renderer
    pub present_mode: wgpu::PresentMode,

    /// 要使用的图形后端。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The graphics backends to use.
    ///
    /// </details>
    pub backends: wgpu::Backends,

    /// 图形适配器的功耗偏好。
    ///
    /// 默认情况下，它是 [`backend::PowerPreference::None`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The power-usage preference for graphics adapters.
    ///
    /// By default, it is [`backend::PowerPreference::None`].
    ///
    /// </details>
    pub power_preference: backend::PowerPreference,

    /// 用于三角形图元的抗锯齿策略。
    ///
    /// 默认情况下，它是 `None`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The antialiasing strategy that will be used for triangle primitives.
    ///
    /// By default, it is `None`.
    ///
    /// </details>
    pub antialiasing: Option<Antialiasing>,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            present_mode: wgpu::PresentMode::AutoVsync,
            backends: wgpu::Backends::all(),
            power_preference: backend::PowerPreference::None,
            antialiasing: None,
        }
    }
}

impl From<backend::Settings> for Settings {
    fn from(settings: backend::Settings) -> Self {
        let backends = settings
            .backend
            .hardware()
            .map(|api| match api {
                backend::Api::Best => wgpu::Backends::all(),
                backend::Api::Vulkan => wgpu::Backends::VULKAN,
                backend::Api::Metal => wgpu::Backends::METAL,
                backend::Api::DirectX12 => wgpu::Backends::DX12,
                backend::Api::OpenGL => wgpu::Backends::GL,
                backend::Api::WebGPU => wgpu::Backends::BROWSER_WEBGPU,
            })
            .unwrap_or_else(wgpu::Backends::all);

        Self {
            present_mode: if settings.vsync {
                wgpu::PresentMode::AutoVsync
            } else {
                wgpu::PresentMode::AutoNoVsync
            },
            antialiasing: settings.antialiasing.then_some(Antialiasing::MSAAx4),
            backends,
            power_preference: settings.power_preference,
        }
    }
}

/// 从当前的环境配置中获取 [`wgpu::PresentMode`]
/// （如果已设置）。
///
/// 可以通过设置 `ICED_PRESENT_MODE` 环境变量来改变
/// 这个函数的返回值。可用的值有：
///
/// - `vsync` → [`wgpu::PresentMode::AutoVsync`]
/// - `no_vsync` → [`wgpu::PresentMode::AutoNoVsync`]
/// - `immediate` → [`wgpu::PresentMode::Immediate`]
/// - `fifo` → [`wgpu::PresentMode::Fifo`]
/// - `fifo_relaxed` → [`wgpu::PresentMode::FifoRelaxed`]
/// - `mailbox` → [`wgpu::PresentMode::Mailbox`]
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Obtains a [`wgpu::PresentMode`] from the current environment
/// configuration, if set.
///
/// The value returned by this function can be changed by setting
/// the `ICED_PRESENT_MODE` env variable. The possible values are:
///
/// - `vsync` → [`wgpu::PresentMode::AutoVsync`]
/// - `no_vsync` → [`wgpu::PresentMode::AutoNoVsync`]
/// - `immediate` → [`wgpu::PresentMode::Immediate`]
/// - `fifo` → [`wgpu::PresentMode::Fifo`]
/// - `fifo_relaxed` → [`wgpu::PresentMode::FifoRelaxed`]
/// - `mailbox` → [`wgpu::PresentMode::Mailbox`]
///
/// </details>
pub fn present_mode_from_env() -> Option<wgpu::PresentMode> {
    let present_mode = std::env::var("ICED_PRESENT_MODE").ok()?;

    match present_mode.to_lowercase().as_str() {
        "vsync" => Some(wgpu::PresentMode::AutoVsync),
        "no_vsync" => Some(wgpu::PresentMode::AutoNoVsync),
        "immediate" => Some(wgpu::PresentMode::Immediate),
        "fifo" => Some(wgpu::PresentMode::Fifo),
        "fifo_relaxed" => Some(wgpu::PresentMode::FifoRelaxed),
        "mailbox" => Some(wgpu::PresentMode::Mailbox),
        _ => None,
    }
}
