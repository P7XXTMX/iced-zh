//! 绘制文本。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Draw text.
//!
//! </details>
pub mod cache;
pub mod editor;
pub mod paragraph;

pub use cache::Cache;
pub use editor::Editor;
pub use paragraph::Paragraph;

pub use cosmic_text;

use crate::core::alignment;
use crate::core::font::{self, Font};
use crate::core::text::{Alignment, Ellipsis, Shaping, Wrapping};
use crate::core::{Color, Pixels, Point, Rectangle, Size, Transformation};

use std::borrow::Cow;
use std::collections::HashSet;
use std::sync::{Arc, OnceLock, RwLock, Weak};

/// 文本图元。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A text primitive.
///
/// </details>
#[derive(Debug, Clone, PartialEq)]
pub enum Text {
    /// 段落。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A paragraph.
    ///
    /// </details>
    #[allow(missing_docs)]
    Paragraph {
        paragraph: paragraph::Weak,
        position: Point,
        color: Color,
        clip_bounds: Rectangle,
        transformation: Transformation,
    },
    /// 编辑器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An editor.
    ///
    /// </details>
    #[allow(missing_docs)]
    Editor {
        editor: editor::Weak,
        position: Point,
        color: Color,
        clip_bounds: Rectangle,
        transformation: Transformation,
    },
    /// 缓存的文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Some cached text.
    ///
    /// </details>
    Cached {
        /// 文本的内容。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The contents of the text.
        ///
        /// </details>
        content: String,
        /// 文本的边界。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The bounds of the text.
        ///
        /// </details>
        bounds: Rectangle,
        /// 文本的颜色。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The color of the text.
        ///
        /// </details>
        color: Color,
        /// 文本的大小，以逻辑像素为单位。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The size of the text in logical pixels.
        ///
        /// </details>
        size: Pixels,
        /// 文本的行高。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The line height of the text.
        ///
        /// </details>
        line_height: Pixels,
        /// 文本的字体。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The font of the text.
        ///
        /// </details>
        font: Font,
        /// 文本的水平对齐方式。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The horizontal alignment of the text.
        ///
        /// </details>
        align_x: Alignment,
        /// 文本的垂直对齐方式。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The vertical alignment of the text.
        ///
        /// </details>
        align_y: alignment::Vertical,
        /// 文本的整形策略。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The shaping strategy of the text.
        ///
        /// </details>
        shaping: Shaping,
        /// 文本的换行策略。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The wrapping strategy of the text.
        ///
        /// </details>
        wrapping: Wrapping,
        /// 文本的省略策略。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The ellipsis strategy of the text.
        ///
        /// </details>
        ellipsis: Ellipsis,
        /// 文本的裁剪边界。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The clip bounds of the text.
        ///
        /// </details>
        clip_bounds: Rectangle,
    },
    /// 原始文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Some raw text.
    ///
    /// </details>
    #[allow(missing_docs)]
    Raw {
        raw: Raw,
        transformation: Transformation,
    },
}

impl Text {
    /// 返回 [`Text`] 的可见边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the visible bounds of the [`Text`].
    ///
    /// </details>
    pub fn visible_bounds(&self) -> Option<Rectangle> {
        match self {
            Text::Paragraph {
                position,
                paragraph,
                clip_bounds,
                transformation,
                ..
            } => Rectangle::new(*position, paragraph.min_bounds)
                .intersection(clip_bounds)
                .map(|bounds| bounds * *transformation),
            Text::Editor {
                editor,
                position,
                clip_bounds,
                transformation,
                ..
            } => Rectangle::new(*position, editor.bounds)
                .intersection(clip_bounds)
                .map(|bounds| bounds * *transformation),
            Text::Cached {
                bounds,
                clip_bounds,
                ..
            } => bounds.intersection(clip_bounds),
            Text::Raw { raw, .. } => Some(raw.clip_bounds),
        }
    }
}

/// [Fira Sans] 字体的常规（regular）变体。
///
/// 启用 `fira-sans` 特性时，它会作为默认字体的一部分
/// 被加载。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The regular variant of the [Fira Sans] font.
///
/// It is loaded as part of the default fonts when the `fira-sans`
/// feature is enabled.
///
/// </details>
///
/// [Fira Sans]: https://mozilla.github.io/Fira/
#[cfg(feature = "fira-sans")]
pub const FIRA_SANS_REGULAR: &[u8] = include_bytes!("../fonts/FiraSans-Regular.ttf").as_slice();

/// 返回全局的 [`FontSystem`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns the global [`FontSystem`].
///
/// </details>
pub fn font_system() -> &'static RwLock<FontSystem> {
    static FONT_SYSTEM: OnceLock<RwLock<FontSystem>> = OnceLock::new();

    FONT_SYSTEM.get_or_init(|| {
        #[allow(unused_mut)]
        let mut raw = cosmic_text::FontSystem::new_with_fonts([
            cosmic_text::fontdb::Source::Binary(Arc::new(
                include_bytes!("../fonts/Iced-Icons.ttf").as_slice(),
            )),
            #[cfg(feature = "fira-sans")]
            cosmic_text::fontdb::Source::Binary(Arc::new(
                include_bytes!("../fonts/FiraSans-Regular.ttf").as_slice(),
            )),
        ]);

        #[cfg(feature = "fira-sans")]
        raw.db_mut().set_sans_serif_family("Fira Sans");

        #[cfg(target_os = "macos")]
        {
            #[cfg(not(feature = "fira-sans"))]
            raw.db_mut().set_sans_serif_family(".SF NS");
            raw.db_mut().set_serif_family("Times New Roman");
            raw.db_mut().set_monospace_family("Menlo");
        }

        #[cfg(target_os = "windows")]
        {
            #[cfg(not(feature = "fira-sans"))]
            raw.db_mut().set_sans_serif_family("Segoe UI");
            raw.db_mut().set_serif_family("Times New Roman");
            raw.db_mut().set_monospace_family("Consolas");
        }

        RwLock::new(FontSystem {
            raw,
            loaded_fonts: HashSet::new(),
            version: Version::default(),
        })
    })
}

/// 一组系统字体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A set of system fonts.
///
/// </details>
pub struct FontSystem {
    raw: cosmic_text::FontSystem,
    loaded_fonts: HashSet<usize>,
    version: Version,
}

impl FontSystem {
    /// 返回原始的 [`cosmic_text::FontSystem`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the raw [`cosmic_text::FontSystem`].
    ///
    /// </details>
    pub fn raw(&mut self) -> &mut cosmic_text::FontSystem {
        &mut self.raw
    }

    /// 从字节数据加载字体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Loads a font from its bytes.
    ///
    /// </details>
    pub fn load_font(&mut self, bytes: Cow<'static, [u8]>) {
        if let Cow::Borrowed(bytes) = bytes {
            let address = bytes.as_ptr() as usize;

            if !self.loaded_fonts.insert(address) {
                return;
            }
        }

        let _ = self
            .raw
            .db_mut()
            .load_font_source(cosmic_text::fontdb::Source::Binary(Arc::new(
                bytes.into_owned(),
            )));

        self.version = Version(self.version.0 + 1);
    }

    /// 返回一个迭代器，遍历字体数据库中
    /// 所有字体的字体族名称。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns an iterator over the family names of all font faces
    /// in the font database.
    ///
    /// </details>
    pub fn families(&self) -> impl Iterator<Item = &str> {
        self.raw
            .db()
            .faces()
            .filter_map(|face| face.families.first())
            .map(|(name, _)| name.as_str())
    }

    /// 返回 [`FontSystem`] 的当前 [`Version`]。
    ///
    /// 加载字体会使 [`FontSystem`] 的版本递增。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current [`Version`] of the [`FontSystem`].
    ///
    /// Loading a font will increase the version of a [`FontSystem`].
    ///
    /// </details>
    pub fn version(&self) -> Version {
        self.version
    }
}

/// 版本号。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A version number.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Version(u32);

/// 可以被绘制的 [`cosmic_text::Buffer`] 的弱引用。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A weak reference to a [`cosmic_text::Buffer`] that can be drawn.
///
/// </details>
#[derive(Debug, Clone)]
pub struct Raw {
    /// [`cosmic_text::Buffer`] 的弱引用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A weak reference to a [`cosmic_text::Buffer`].
    ///
    /// </details>
    pub buffer: Weak<cosmic_text::Buffer>,
    /// 文本的位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The position of the text.
    ///
    /// </details>
    pub position: Point,
    /// 文本的颜色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The color of the text.
    ///
    /// </details>
    pub color: Color,
    /// 文本的裁剪边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The clip bounds of the text.
    ///
    /// </details>
    pub clip_bounds: Rectangle,
}

impl PartialEq for Raw {
    fn eq(&self, _other: &Self) -> bool {
        // TODO: There is no proper way to compare raw buffers
        // For now, no two instances of `Raw` text will be equal.
        // This should be fine, but could trigger unnecessary redraws
        // in the future.
        // 待办：目前没有比较原始缓冲区的合适办法
        // 暂时让任意两个 `Raw` 文本实例都不相等。
        // 这应该没什么问题，但将来可能会触发
        // 不必要的重绘。
        false
    }
}

/// 测量给定 [`cosmic_text::Buffer`] 的尺寸。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Measures the dimensions of the given [`cosmic_text::Buffer`].
///
/// </details>
pub fn measure(buffer: &cosmic_text::Buffer) -> (Size, bool) {
    let (width, height, has_rtl) =
        buffer
            .layout_runs()
            .fold((0.0, 0.0, false), |(width, height, has_rtl), run| {
                (
                    run.line_w.max(width),
                    height + run.line_height,
                    has_rtl || run.rtl,
                )
            });

    (Size::new(width, height), has_rtl)
}

/// 按给定的 [`Alignment`] 对齐给定的 [`cosmic_text::Buffer`]，
/// 并返回它的最小 [`Size`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Aligns the given [`cosmic_text::Buffer`] with the given [`Alignment`]
/// and returns its minimum [`Size`].
///
/// </details>
pub fn align(
    buffer: &mut cosmic_text::Buffer,
    font_system: &mut cosmic_text::FontSystem,
    alignment: Alignment,
) -> Size {
    let (min_bounds, has_rtl) = measure(buffer);
    let mut needs_relayout = has_rtl;

    if let Some(align) = to_align(alignment) {
        let has_multiple_lines = buffer.lines.len() > 1
            || buffer
                .lines
                .first()
                .is_some_and(|line| line.layout_opt().is_some_and(|layout| layout.len() > 1));

        if has_multiple_lines {
            for line in &mut buffer.lines {
                let _ = line.set_align(Some(align));
            }

            needs_relayout = true;
        } else if let Some(line) = buffer.lines.first_mut() {
            needs_relayout |= line.set_align(None);
        }
    }

    // TODO: Avoid relayout with some changes to `cosmic-text` (?)
    // 待办：对 `cosmic-text` 做一些修改，以避免重新布局（？）
    if needs_relayout {
        log::trace!("Relayouting paragraph...");

        buffer.set_size(Some(min_bounds.width), Some(min_bounds.height));
        buffer.shape_until_scroll(font_system, false);
    }

    min_bounds
}

/// 返回给定 [`Font`] 的属性。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns the attributes of the given [`Font`].
///
/// </details>
pub fn to_attributes(font: Font) -> cosmic_text::Attrs<'static> {
    cosmic_text::Attrs::new()
        .family(to_family(font.family))
        .weight(to_weight(font.weight))
        .stretch(to_stretch(font.stretch))
        .style(to_style(font.style))
}

fn to_family(family: font::Family) -> cosmic_text::Family<'static> {
    match family {
        font::Family::Name(name) => cosmic_text::Family::Name(name),
        font::Family::SansSerif => cosmic_text::Family::SansSerif,
        font::Family::Serif => cosmic_text::Family::Serif,
        font::Family::Cursive => cosmic_text::Family::Cursive,
        font::Family::Fantasy => cosmic_text::Family::Fantasy,
        font::Family::Monospace => cosmic_text::Family::Monospace,
    }
}

fn to_weight(weight: font::Weight) -> cosmic_text::Weight {
    match weight {
        font::Weight::Thin => cosmic_text::Weight::THIN,
        font::Weight::ExtraLight => cosmic_text::Weight::EXTRA_LIGHT,
        font::Weight::Light => cosmic_text::Weight::LIGHT,
        font::Weight::Normal => cosmic_text::Weight::NORMAL,
        font::Weight::Medium => cosmic_text::Weight::MEDIUM,
        font::Weight::Semibold => cosmic_text::Weight::SEMIBOLD,
        font::Weight::Bold => cosmic_text::Weight::BOLD,
        font::Weight::ExtraBold => cosmic_text::Weight::EXTRA_BOLD,
        font::Weight::Black => cosmic_text::Weight::BLACK,
    }
}

fn to_stretch(stretch: font::Stretch) -> cosmic_text::Stretch {
    match stretch {
        font::Stretch::UltraCondensed => cosmic_text::Stretch::UltraCondensed,
        font::Stretch::ExtraCondensed => cosmic_text::Stretch::ExtraCondensed,
        font::Stretch::Condensed => cosmic_text::Stretch::Condensed,
        font::Stretch::SemiCondensed => cosmic_text::Stretch::SemiCondensed,
        font::Stretch::Normal => cosmic_text::Stretch::Normal,
        font::Stretch::SemiExpanded => cosmic_text::Stretch::SemiExpanded,
        font::Stretch::Expanded => cosmic_text::Stretch::Expanded,
        font::Stretch::ExtraExpanded => cosmic_text::Stretch::ExtraExpanded,
        font::Stretch::UltraExpanded => cosmic_text::Stretch::UltraExpanded,
    }
}

fn to_style(style: font::Style) -> cosmic_text::Style {
    match style {
        font::Style::Normal => cosmic_text::Style::Normal,
        font::Style::Italic => cosmic_text::Style::Italic,
        font::Style::Oblique => cosmic_text::Style::Oblique,
    }
}

fn to_align(alignment: Alignment) -> Option<cosmic_text::Align> {
    match alignment {
        Alignment::Default => None,
        Alignment::Left => Some(cosmic_text::Align::Left),
        Alignment::Center => Some(cosmic_text::Align::Center),
        Alignment::Right => Some(cosmic_text::Align::Right),
        Alignment::Justified => Some(cosmic_text::Align::Justified),
    }
}

/// 把 [`Shaping`] 策略转换为 [`cosmic_text::Shaping`] 策略。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Converts some [`Shaping`] strategy to a [`cosmic_text::Shaping`] strategy.
///
/// </details>
pub fn to_shaping(shaping: Shaping, text: &str) -> cosmic_text::Shaping {
    match shaping {
        Shaping::Auto => {
            if text.is_ascii() {
                cosmic_text::Shaping::Basic
            } else {
                cosmic_text::Shaping::Advanced
            }
        }
        Shaping::Basic => cosmic_text::Shaping::Basic,
        Shaping::Advanced => cosmic_text::Shaping::Advanced,
    }
}

/// 把 [`Wrapping`] 策略转换为 [`cosmic_text::Wrap`] 策略。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Converts some [`Wrapping`] strategy to a [`cosmic_text::Wrap`] strategy.
///
/// </details>
pub fn to_wrap(wrapping: Wrapping) -> cosmic_text::Wrap {
    match wrapping {
        Wrapping::None => cosmic_text::Wrap::None,
        Wrapping::Word => cosmic_text::Wrap::Word,
        Wrapping::Glyph => cosmic_text::Wrap::Glyph,
        Wrapping::WordOrGlyph => cosmic_text::Wrap::WordOrGlyph,
    }
}

/// 把 [`Ellipsis`] 策略转换为 [`cosmic_text::Ellipsize`] 策略。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Converts some [`Ellipsis`] strategy to a [`cosmic_text::Ellipsize`] strategy.
///
/// </details>
pub fn to_ellipsize(ellipsis: Ellipsis, max_height: f32) -> cosmic_text::Ellipsize {
    let limit = cosmic_text::EllipsizeHeightLimit::Height(max_height);

    match ellipsis {
        Ellipsis::None => cosmic_text::Ellipsize::None,
        Ellipsis::Start => cosmic_text::Ellipsize::Start(limit),
        Ellipsis::Middle => cosmic_text::Ellipsize::Middle(limit),
        Ellipsis::End => cosmic_text::Ellipsize::End(limit),
    }
}

/// 把 [`Color`] 转换为 [`cosmic_text::Color`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Converts some [`Color`] to a [`cosmic_text::Color`].
///
/// </details>
pub fn to_color(color: Color) -> cosmic_text::Color {
    let [r, g, b, a] = color.into_rgba8();

    cosmic_text::Color::rgba(r, g, b, a)
}

/// 根据文本的大小和缩放因子，返回理想的微调（hint）因子。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns the ideal hint factor given the size and scale factor of some text.
///
/// </details>
pub fn hint_factor(_size: Pixels, _scale_factor: Option<f32>) -> Option<f32> {
    // TODO: Fix hinting in `cosmic-text`
    // const MAX_HINTING_SIZE: f32 = 18.0;
    // 待办：修复 `cosmic-text` 中的字体微调（hinting）

    // let hint_factor = scale_factor?;

    // if size.0 * hint_factor < MAX_HINTING_SIZE {
    //     Some(hint_factor)
    // } else {
    //     None
    // }

    None // Disable all text hinting for now（暂时禁用所有文本微调）
}

/// 计算给定缓冲区中的选区区域。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Computes selection regions in the given buffer.
///
/// </details>
pub fn regions(
    buffer: &cosmic_text::Buffer,
    start: cosmic_text::Cursor,
    end: cosmic_text::Cursor,
) -> Vec<Rectangle> {
    let mut regions = Vec::new();
    let scroll = buffer.scroll();

    for run in buffer.layout_runs() {
        let line_i = run.line_i;
        let line_w = run.line_w;
        let line_top = run.line_top;
        let line_height = run.line_height;
        let line_break = run.line_height / 2.0;

        if line_i >= start.line && line_i <= end.line {
            let mut highlights = run.highlight(start, end).peekable();

            if highlights.peek().is_none() && run.glyphs.is_empty() && end.line > line_i {
                regions.push(Rectangle {
                    x: 0.0,
                    y: line_top,
                    width: line_break,
                    height: line_height,
                });
            }

            while let Some((x, width)) = highlights.next() {
                let mut min = x;
                let mut max = x + width;

                // Extend the last rect to the line edge for
                // multi-line selections
                // 对于多行选区，把最后一个矩形
                // 延伸到行的边缘
                if highlights.peek().is_none() && end.line > line_i {
                    if run.rtl {
                        min = 0.0;
                    } else {
                        max = line_w + line_break;
                    }
                }

                regions.push(Rectangle {
                    x: min - scroll.horizontal,
                    y: line_top,
                    width: (max - min).max(0.0),
                    height: line_height,
                });
            }
        }
    }

    regions
}

/// 与 `iced_graphics` 耦合的文本渲染器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A text renderer coupled to `iced_graphics`.
///
/// </details>
pub trait Renderer {
    /// 绘制给定的 [`Raw`] 文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the given [`Raw`] text.
    ///
    /// </details>
    fn fill_raw(&mut self, raw: Raw);
}
