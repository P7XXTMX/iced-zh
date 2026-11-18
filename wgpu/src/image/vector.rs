use crate::core::svg;
use crate::core::{Color, Size};
use crate::image::atlas::{self, Atlas};

use resvg::tiny_skia;
use resvg::usvg;
use rustc_hash::{FxHashMap, FxHashSet};
use std::fs;
use std::panic;
use std::sync::Arc;

/// 缓存中与某个 svg 句柄对应的条目
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Entry in cache corresponding to an svg handle
///
/// </details>
pub enum Svg {
    /// 已解析的 svg
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Parsed svg
    ///
    /// </details>
    Loaded(usvg::Tree),
    /// svg 未找到或解析失败
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Svg not found or failed to parse
    ///
    /// </details>
    NotFound,
}

impl Svg {
    /// 视口的宽度和高度
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Viewport width and height
    ///
    /// </details>
    pub fn viewport_dimensions(&self) -> Size<u32> {
        match self {
            Svg::Loaded(tree) => {
                let size = tree.size();

                Size::new(size.width() as u32, size.height() as u32)
            }
            Svg::NotFound => Size::new(1, 1),
        }
    }
}

/// 缓存 svg 的矢量数据和光栅数据
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Caches svg vector and raster data
///
/// </details>
#[derive(Debug, Default)]
pub struct Cache {
    svgs: FxHashMap<u64, Svg>,
    rasterized: FxHashMap<(u64, u32, u32, ColorFilter), atlas::Entry>,
    svg_hits: FxHashSet<u64>,
    rasterized_hits: FxHashSet<(u64, u32, u32, ColorFilter)>,
    should_trim: bool,
    fontdb: Option<Arc<usvg::fontdb::Database>>,
}

type ColorFilter = Option<[u8; 4]>;

impl Cache {
    /// 加载 svg
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Load svg
    ///
    /// </details>
    pub fn load(&mut self, handle: &svg::Handle) -> &Svg {
        if self.svgs.contains_key(&handle.id()) {
            return self.svgs.get(&handle.id()).unwrap();
        }

        // TODO: Reuse `cosmic-text` font database
        // 待办：复用 `cosmic-text` 的字体数据库
        if self.fontdb.is_none() {
            let mut fontdb = usvg::fontdb::Database::new();
            fontdb.load_system_fonts();

            self.fontdb = Some(Arc::new(fontdb));
        }

        let options = usvg::Options {
            fontdb: self
                .fontdb
                .as_ref()
                .expect("fontdb must be initialized")
                .clone(),
            ..usvg::Options::default()
        };

        let svg = match handle.data() {
            svg::Data::Path(path) => fs::read_to_string(path)
                .ok()
                .and_then(|contents| {
                    usvg::Tree::from_str(&contents, &options).ok()
                })
                .map(Svg::Loaded)
                .unwrap_or(Svg::NotFound),
            svg::Data::Bytes(bytes) => {
                match usvg::Tree::from_data(bytes, &options) {
                    Ok(tree) => Svg::Loaded(tree),
                    Err(_) => Svg::NotFound,
                }
            }
        };

        self.should_trim = true;

        let _ = self.svgs.insert(handle.id(), svg);
        self.svgs.get(&handle.id()).unwrap()
    }

    /// 加载 svg 并上传光栅数据
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Load svg and upload raster data
    ///
    /// </details>
    pub fn upload(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        belt: &mut wgpu::util::StagingBelt,
        handle: &svg::Handle,
        color: Option<Color>,
        size: Size,
        scale: f32,
        atlas: &mut Atlas,
    ) -> Option<&atlas::Entry> {
        let id = handle.id();

        let (width, height) = (
            (scale * size.width).ceil() as u32,
            (scale * size.height).ceil() as u32,
        );

        let color = color.map(Color::into_rgba8);
        let key = (id, width, height, color);

        // TODO: Optimize!
        // We currently rerasterize the SVG when its size changes. This is slow
        // as heck. A GPU rasterizer like `pathfinder` may perform better.
        // It would be cool to be able to smooth resize the `svg` example.
        // 待办：优化！
        // 目前 SVG 的大小一变，我们就重新光栅化它。这非常
        // 慢。像 `pathfinder` 这样的 GPU 光栅化器可能表现更好。
        // 要是能平滑地调整 `svg` 示例的大小就好了。
        if self.rasterized.contains_key(&key) {
            let _ = self.svg_hits.insert(id);
            let _ = self.rasterized_hits.insert(key);

            return self.rasterized.get(&key);
        }

        match self.load(handle) {
            Svg::Loaded(tree) => {
                if width == 0 || height == 0 {
                    return None;
                }

                // TODO: Optimize!
                // We currently rerasterize the SVG when its size changes. This is slow
                // as heck. A GPU rasterizer like `pathfinder` may perform better.
                // It would be cool to be able to smooth resize the `svg` example.
                // 待办：优化！
                // 目前 SVG 的大小一变，我们就重新光栅化它。这非常
                // 慢。像 `pathfinder` 这样的 GPU 光栅化器可能表现更好。
                // 要是能平滑地调整 `svg` 示例的大小就好了。
                let mut img = tiny_skia::Pixmap::new(width, height)?;

                let tree_size = tree.size().to_int_size();

                let target_size = if width > height {
                    tree_size.scale_to_width(width)
                } else {
                    tree_size.scale_to_height(height)
                };

                let transform = if let Some(target_size) = target_size {
                    let tree_size = tree_size.to_size();
                    let target_size = target_size.to_size();

                    tiny_skia::Transform::from_scale(
                        target_size.width() / tree_size.width(),
                        target_size.height() / tree_size.height(),
                    )
                } else {
                    tiny_skia::Transform::default()
                };

                // SVG rendering can panic on malformed or complex vectors.
                // We catch panics to prevent crashes and continue gracefully.
                // 遇到格式错误或过于复杂的矢量图时，SVG 渲染可能会 panic。
                // 我们捕获 panic 以避免崩溃，并平稳地继续运行。
                let render =
                    panic::catch_unwind(panic::AssertUnwindSafe(|| {
                        resvg::render(tree, transform, &mut img.as_mut());
                    }));

                if let Err(error) = render {
                    log::warn!(
                        "SVG rendering for {handle:?} panicked: {error:?}"
                    );
                }

                let mut rgba = img.take();

                if let Some(color) = color {
                    rgba.chunks_exact_mut(4).for_each(|rgba| {
                        if rgba[3] > 0 {
                            rgba[0] = color[0];
                            rgba[1] = color[1];
                            rgba[2] = color[2];
                        }
                    });
                }

                let allocation = atlas
                    .upload(device, encoder, belt, width, height, &rgba)?;

                log::debug!("allocating {id} {width}x{height}");

                let _ = self.svg_hits.insert(id);
                let _ = self.rasterized_hits.insert(key);
                let _ = self.rasterized.insert(key, allocation);
                self.should_trim = true;

                self.rasterized.get(&key)
            }
            Svg::NotFound => None,
        }
    }

    /// 加载 svg 并上传光栅数据
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Load svg and upload raster data
    ///
    /// </details>
    pub fn trim(&mut self, atlas: &mut Atlas) {
        if !self.should_trim {
            return;
        }

        let svg_hits = &self.svg_hits;
        let rasterized_hits = &self.rasterized_hits;

        self.svgs.retain(|k, _| svg_hits.contains(k));
        self.rasterized.retain(|k, entry| {
            let retain = rasterized_hits.contains(k);

            if !retain {
                atlas.remove(entry);
            }

            retain
        });
        self.svg_hits.clear();
        self.rasterized_hits.clear();
        self.should_trim = false;
    }
}

impl std::fmt::Debug for Svg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Svg::Loaded(_) => write!(f, "Svg::Loaded"),
            Svg::NotFound => write!(f, "Svg::NotFound"),
        }
    }
}
