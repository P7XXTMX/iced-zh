use crate::core;
use crate::core::alignment;
use crate::core::text::{Alignment, Ellipsis, LineHeight, Paragraph, Shaping, Wrapping};
use crate::core::{Color, Font, Pixels, Point, Size, Vector};
use crate::geometry::Path;
use crate::text;

/// 可以绘制到画布上的一段文本
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A bunch of text that can be drawn to a canvas
///
/// </details>
#[derive(Debug, Clone)]
pub struct Text {
    /// 文本的内容
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The contents of the text
    ///
    /// </details>
    pub content: String,
    /// 文本相对于对齐属性的位置。
    ///
    /// 默认情况下，这个位置相对于左上角坐标，也就是说，
    /// 如果水平和垂直对齐方式保持不变，这个属性指定的就是
    /// 文本左上角应当放置的位置。
    ///
    /// 通过修改 horizontal_alignment 和 vertical_alignment 属性，
    /// 可以改变文本的哪个部分被放置在这个位置。
    ///
    /// 例如，当 horizontal_alignment 和 vertical_alignment 都设置为 Center 时，
    /// 放置在给定位置的是文本的中心，而不是左上角坐标。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The position of the text relative to the alignment properties.
    ///
    /// By default, this position will be relative to the top-left corner coordinate meaning that
    /// if the horizontal and vertical alignments are unchanged, this property will tell where the
    /// top-left corner of the text should be placed.
    ///
    /// By changing the horizontal_alignment and vertical_alignment properties, you are are able to
    /// change what part of text is placed at this positions.
    ///
    /// For example, when the horizontal_alignment and vertical_alignment are set to Center, the
    /// center of the text will be placed at the given position NOT the top-left coordinate.
    ///
    /// </details>
    pub position: Point,
    /// 此 [`Text`] 可用的最大水平空间。
    ///
    /// 文本到达该宽度时会换行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The maximum horizontal space available for this [`Text`].
    ///
    /// Text will break into new lines when the width is reached.
    ///
    /// </details>
    pub max_width: f32,
    /// 文本的颜色
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The color of the text
    ///
    /// </details>
    pub color: Color,
    /// 文本的大小
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The size of the text
    ///
    /// </details>
    pub size: Pixels,
    /// 文本的行高。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The line height of the text.
    ///
    /// </details>
    pub line_height: LineHeight,
    /// 文本的字体
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The font of the text
    ///
    /// </details>
    pub font: Font,
    /// 文本的水平对齐方式
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The horizontal alignment of the text
    ///
    /// </details>
    pub align_x: Alignment,
    /// 文本的垂直对齐方式
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The vertical alignment of the text
    ///
    /// </details>
    pub align_y: alignment::Vertical,
    /// 文本的整形策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The shaping strategy of the text.
    ///
    /// </details>
    pub shaping: Shaping,
    /// 文本的换行策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The wrapping strategy of the text.
    ///
    /// </details>
    pub wrapping: Wrapping,
    /// 文本的省略策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The ellipsis strategy of the text.
    ///
    /// </details>
    pub ellipsis: Ellipsis,
}

impl Text {
    /// 计算 [`Text`] 的各个 [`Path`]，并使用给定的闭包
    /// 绘制它们。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Computes the [`Path`]s of the [`Text`] and draws them using
    /// the given closure.
    ///
    /// </details>
    pub fn draw_with(&self, mut f: impl FnMut(Path, Color)) {
        let paragraph = text::Paragraph::with_text(core::text::Text {
            content: &self.content,
            bounds: Size::new(self.max_width, f32::INFINITY),
            size: self.size,
            line_height: self.line_height,
            font: self.font,
            align_x: self.align_x,
            align_y: self.align_y,
            shaping: self.shaping,
            wrapping: Wrapping::default(),
            ellipsis: Ellipsis::default(),
            hint_factor: None,
        });

        let translation_x = match self.align_x {
            Alignment::Default | Alignment::Left | Alignment::Justified => self.position.x,
            Alignment::Center => self.position.x - paragraph.min_width() / 2.0,
            Alignment::Right => self.position.x - paragraph.min_width(),
        };

        let translation_y = {
            match self.align_y {
                alignment::Vertical::Top => self.position.y,
                alignment::Vertical::Center => self.position.y - paragraph.min_height() / 2.0,
                alignment::Vertical::Bottom => self.position.y - paragraph.min_height(),
            }
        };

        let buffer = paragraph.buffer();
        let mut swash_cache = cosmic_text::SwashCache::new();

        let mut font_system = text::font_system().write().expect("Write font system");

        for run in buffer.layout_runs() {
            for glyph in run.glyphs.iter() {
                let physical_glyph = glyph.physical((0.0, 0.0), 1.0);

                let start_x = translation_x + glyph.x + glyph.x_offset;
                let start_y = translation_y + glyph.y_offset + run.line_y;
                let offset = Vector::new(start_x, start_y);

                if let Some(commands) =
                    swash_cache.get_outline_commands(font_system.raw(), physical_glyph.cache_key)
                {
                    let glyph = Path::new(|path| {
                        use cosmic_text::Command;

                        for command in commands {
                            match command {
                                Command::MoveTo(p) => {
                                    path.move_to(Point::new(p.x, -p.y) + offset);
                                }
                                Command::LineTo(p) => {
                                    path.line_to(Point::new(p.x, -p.y) + offset);
                                }
                                Command::CurveTo(control_a, control_b, to) => {
                                    path.bezier_curve_to(
                                        Point::new(control_a.x, -control_a.y) + offset,
                                        Point::new(control_b.x, -control_b.y) + offset,
                                        Point::new(to.x, -to.y) + offset,
                                    );
                                }
                                Command::QuadTo(control, to) => {
                                    path.quadratic_curve_to(
                                        Point::new(control.x, -control.y) + offset,
                                        Point::new(to.x, -to.y) + offset,
                                    );
                                }
                                Command::Close => {
                                    path.close();
                                }
                            }
                        }
                    });

                    f(glyph, self.color);
                } else {
                    // TODO: Raster image support for `Canvas`
                    // 待办：为 `Canvas` 支持位图图像
                    let [r, g, b, a] = self.color.into_rgba8();

                    swash_cache.with_pixels(
                        font_system.raw(),
                        physical_glyph.cache_key,
                        cosmic_text::Color::rgba(r, g, b, a),
                        |x, y, color| {
                            f(
                                Path::rectangle(
                                    Point::new(x as f32, y as f32) + offset,
                                    Size::new(1.0, 1.0),
                                ),
                                Color::from_rgba8(
                                    color.r(),
                                    color.g(),
                                    color.b(),
                                    color.a() as f32 / 255.0,
                                ),
                            );
                        },
                    );
                }
            }
        }
    }
}

impl Default for Text {
    fn default() -> Text {
        Text {
            content: String::new(),
            position: Point::ORIGIN,
            max_width: f32::INFINITY,
            color: Color::BLACK,
            size: Pixels(16.0),
            line_height: LineHeight::Relative(1.2),
            font: Font::default(),
            align_x: Alignment::Default,
            align_y: alignment::Vertical::Top,
            shaping: Shaping::default(),
            wrapping: Wrapping::default(),
            ellipsis: Ellipsis::default(),
        }
    }
}

impl From<String> for Text {
    fn from(content: String) -> Text {
        Text {
            content,
            ..Default::default()
        }
    }
}

impl From<&str> for Text {
    fn from(content: &str) -> Text {
        String::from(content).into()
    }
}
