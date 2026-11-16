//! 合成器负责初始化渲染器并管理窗口
//! 表面。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! A compositor is responsible for initializing a renderer and managing window
//! surfaces.
//!
//! </details>
use crate::core::Color;
use crate::core::backend;
use crate::core::font;
use crate::core::renderer;
use crate::futures::{MaybeSend, MaybeSync};
use crate::{Shell, Viewport};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use thiserror::Error;

use std::borrow::Cow;
use std::fmt::Debug;

/// 可以绘制到窗口的图形合成器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A graphics compositor that can draw to windows.
///
/// </details>
pub trait Compositor: Sized {
    /// 后端的 iced 渲染器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The iced renderer of the backend.
    ///
    /// </details>
    type Renderer;

    /// 后端的表面。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The surface of the backend.
    ///
    /// </details>
    type Surface;

    /// 使用给定的 [`backend::Settings`] 创建一个新的 [`Compositor`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Compositor`] with the given [`backend::Settings`].
    ///
    /// </details>
    fn new(
        settings: backend::Settings,
        display: impl Display + Clone,
        compatible_window: impl Window + Clone,
        shell: Shell,
    ) -> impl Future<Output = Result<Self, backend::Error>>;

    /// 为 [`Compositor`] 创建一个 [`Self::Renderer`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Self::Renderer`] for the [`Compositor`].
    ///
    /// </details>
    fn create_renderer(&self, settings: renderer::Settings) -> Self::Renderer;

    /// 为给定的窗口创建一个新的 [`Surface`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Crates a new [`Surface`] for the given window.
    ///
    /// </details>
    ///
    /// [`Surface`]: Self::Surface
    fn create_surface(&mut self, window: impl Window + Clone) -> Self::Surface;

    /// 使用给定的尺寸配置一个新的 [`Surface`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Configures a new [`Surface`] with the given dimensions.
    ///
    /// </details>
    ///
    /// [`Surface`]: Self::Surface
    fn configure_surface(&mut self, surface: &mut Self::Surface, width: u32, height: u32);

    /// 返回此 [`Compositor`] 使用的 [`Information`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns [`Information`] used by this [`Compositor`].
    ///
    /// </details>
    fn information(&self) -> Information;

    /// 从字节数据加载字体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Loads a font from its bytes.
    ///
    /// </details>
    fn load_font(&mut self, font: Cow<'static, [u8]>) -> Result<(), font::Error> {
        crate::text::font_system()
            .write()
            .expect("Write to font system")
            .load_font(font);

        // TODO: Error handling
        // 待办：错误处理
        Ok(())
    }

    /// 列出所有可用的字体族。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Lists all the available font families.
    ///
    /// </details>
    fn list_fonts(&mut self) -> Result<Vec<font::Family>, font::Error> {
        use std::collections::BTreeSet;

        let font_system = crate::text::font_system()
            .read()
            .expect("Read from font system");

        let families = BTreeSet::from_iter(font_system.families());

        Ok(families.into_iter().map(font::Family::name).collect())
    }

    /// 把 [`Renderer`] 的图元呈现到给定 [`Surface`] 的下一帧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Presents the [`Renderer`] primitives to the next frame of the given [`Surface`].
    ///
    /// </details>
    ///
    /// [`Renderer`]: Self::Renderer
    /// [`Surface`]: Self::Surface
    fn present(
        &mut self,
        renderer: &mut Self::Renderer,
        surface: &mut Self::Surface,
        viewport: &Viewport,
        background_color: Color,
        on_pre_present: impl FnOnce(),
    ) -> Result<(), SurfaceError>;

    /// 把 [`Renderer`] 当前的图元截图到一个离屏纹理，并返回该纹理的字节数据，
    /// 按 `sRGB` 色彩空间中的 `RGBA` 顺序排列。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Screenshots the current [`Renderer`] primitives to an offscreen texture, and returns the bytes of
    /// the texture ordered as `RGBA` in the `sRGB` color space.
    ///
    /// </details>
    ///
    /// [`Renderer`]: Self::Renderer
    fn screenshot(
        &mut self,
        renderer: &mut Self::Renderer,
        viewport: &Viewport,
        background_color: Color,
    ) -> Vec<u8>;
}

/// 可以在 [`Compositor`] 中使用的窗口。
///
/// 这只是 `raw-window-handle` 各个特质的一个
/// 便捷的父特质。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A window that can be used in a [`Compositor`].
///
/// This is just a convenient super trait of the `raw-window-handle`
/// traits.
///
/// </details>
pub trait Window: HasWindowHandle + Debug + MaybeSend + MaybeSync + 'static {}

impl<T> Window for T where T: HasWindowHandle + Debug + MaybeSend + MaybeSync + 'static {}

/// 可以在 [`Compositor`] 中使用的、拥有所有权的显示句柄。
///
/// 这只是 `raw-window-handle` 的特质的一个
/// 便捷的父特质。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An owned display handle that can be used in a [`Compositor`].
///
/// This is just a convenient super trait of the `raw-window-handle`
/// trait.
///
/// </details>
pub trait Display: HasDisplayHandle + Debug + Send + Sync + 'static {}

impl<T> Display for T where T: HasDisplayHandle + Debug + Send + Sync + 'static {}

/// 定义渲染器的默认合成器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Defines the default compositor of a renderer.
///
/// </details>
pub trait Default {
    /// 渲染器的合成器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The compositor of the renderer.
    ///
    /// </details>
    type Compositor: Compositor<Renderer = Self>;
}

/// 调用 [`Compositor::present`] 失败时的结果。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Result of an unsuccessful call to [`Compositor::present`].
///
/// </details>
#[derive(Clone, PartialEq, Eq, Debug, Error)]
pub enum SurfaceError {
    /// 尝试获取下一帧时超时。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A timeout was encountered while trying to acquire the next frame.
    ///
    /// </details>
    #[error("A timeout was encountered while trying to acquire the next frame")]
    Timeout,
    /// 底层表面已经改变，因此必须更新表面。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The underlying surface has changed, and therefore the surface must be updated.
    ///
    /// </details>
    #[error("The underlying surface has changed, and therefore the surface must be updated.")]
    Outdated,
    /// 交换链已丢失，需要重新创建。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The swap chain has been lost and needs to be recreated.
    ///
    /// </details>
    #[error("The surface has been lost and needs to be recreated")]
    Lost,
    /// 没有剩余内存可用于分配新的帧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// There is no more memory left to allocate a new frame.
    ///
    /// </details>
    #[error("There is no more memory left to allocate a new frame")]
    OutOfMemory,
    /// 表面被遮挡，不得向其绘制。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The surface is occluded and must not be drawn to.
    ///
    /// </details>
    #[error("The surface is occluded and must not be drawn to")]
    Occluded,
    /// 获取纹理失败，出现了一般性错误。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Acquiring a texture failed with a generic error.
    ///
    /// </details>
    #[error("Acquiring a texture failed with a generic error")]
    Other,
}

/// 包含图形相关的信息（例如图形适配器、图形后端）。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Contains information about the graphics (e.g. graphics adapter, graphics backend).
///
/// </details>
#[derive(Debug)]
pub struct Information {
    /// 图形适配器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Contains the graphics adapter.
    ///
    /// </details>
    pub adapter: String,
    /// 图形后端。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Contains the graphics backend.
    ///
    /// </details>
    pub backend: String,
}

#[cfg(debug_assertions)]
impl Compositor for () {
    type Renderer = ();
    type Surface = ();

    async fn new(
        _settings: backend::Settings,
        _display: impl Display,
        _compatible_window: impl Window + Clone,
        _shell: Shell,
    ) -> Result<Self, backend::Error> {
        Ok(())
    }

    fn create_renderer(&self, _settings: renderer::Settings) -> Self::Renderer {}

    fn create_surface(&mut self, _window: impl Window + Clone) -> Self::Surface {}

    fn configure_surface(&mut self, _surface: &mut Self::Surface, _width: u32, _height: u32) {}

    fn load_font(&mut self, _font: Cow<'static, [u8]>) -> Result<(), font::Error> {
        Ok(())
    }

    fn list_fonts(&mut self) -> Result<Vec<font::Family>, font::Error> {
        Ok(Vec::new())
    }

    fn information(&self) -> Information {
        Information {
            adapter: String::from("Null Renderer"),
            backend: String::from("Null"),
        }
    }

    fn present(
        &mut self,
        _renderer: &mut Self::Renderer,
        _surface: &mut Self::Surface,
        _viewport: &Viewport,
        _background_color: Color,
        _on_pre_present: impl FnOnce(),
    ) -> Result<(), SurfaceError> {
        Ok(())
    }

    fn screenshot(
        &mut self,
        _renderer: &mut Self::Renderer,
        _viewport: &Viewport,
        _background_color: Color,
    ) -> Vec<u8> {
        vec![]
    }
}

#[cfg(debug_assertions)]
impl Default for () {
    type Compositor = ();
}
