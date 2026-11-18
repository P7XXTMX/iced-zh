//! 可用作某些几何图形填充的渐变。
//!
//! 如果需要可用作部件背景的渐变，参见 [`Gradient`]。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! A gradient that can be used as a fill for some geometry.
//!
//! For a gradient that you can use as a background variant for a widget, see [`Gradient`].
//!
//! </details>
use crate::color;
use crate::core::gradient::ColorStop;
use crate::core::{self, Color, Point, Rectangle};

use bytemuck::{Pod, Zeroable};
use half::f16;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
/// 沿某个方向对颜色进行线性插值的填充。
///
/// 如果需要可用作部件背景填充的渐变，参见 [`crate::core::Gradient`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A fill which linearly interpolates colors along a direction.
///
/// For a gradient which can be used as a fill for a background of a widget, see [`crate::core::Gradient`].
///
/// </details>
pub enum Gradient {
    /// 线性渐变沿着从 `start` 点到 `end` 点的方向
    /// 对颜色进行插值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A linear gradient interpolates colors along a direction from its `start` to its `end`
    /// point.
    ///
    /// </details>
    Linear(Linear),
}

impl From<Linear> for Gradient {
    fn from(gradient: Linear) -> Self {
        Self::Linear(gradient)
    }
}

impl Gradient {
    /// 打包 [`Gradient`]，以便在着色器代码中使用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Packs the [`Gradient`] for use in shader code.
    ///
    /// </details>
    pub fn pack(&self) -> Packed {
        match self {
            Gradient::Linear(linear) => linear.pack(),
        }
    }
}

/// 线性渐变。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A linear gradient.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Linear {
    /// 渐变的绝对起始位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The absolute starting position of the gradient.
    ///
    /// </details>
    pub start: Point,

    /// 渐变的绝对结束位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The absolute ending position of the gradient.
    ///
    /// </details>
    pub end: Point,

    /// 沿线性渐变方向分布的 [`ColorStop`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// [`ColorStop`]s along the linear gradient direction.
    ///
    /// </details>
    pub stops: [Option<ColorStop>; 8],
}

impl Linear {
    /// 创建一个新的 [`Linear`] 构建器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Linear`] builder.
    ///
    /// </details>
    pub fn new(start: Point, end: Point) -> Self {
        Self {
            start,
            end,
            stops: [None; 8],
        }
    }

    /// 向渐变添加一个新的 [`ColorStop`]，由偏移量和颜色定义。
    ///
    /// 任何不在 `0.0..=1.0` 范围内的 `offset` 都会被静默忽略。
    ///
    /// 第 8 个之后添加的色标都会被静默忽略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adds a new [`ColorStop`], defined by an offset and a color, to the gradient.
    ///
    /// Any `offset` that is not within `0.0..=1.0` will be silently ignored.
    ///
    /// Any stop added after the 8th will be silently ignored.
    ///
    /// </details>
    pub fn add_stop(mut self, offset: f32, color: Color) -> Self {
        if offset.is_finite() && (0.0..=1.0).contains(&offset) {
            let (Ok(index) | Err(index)) =
                self.stops.binary_search_by(|stop| match stop {
                    None => Ordering::Greater,
                    Some(stop) => stop.offset.partial_cmp(&offset).unwrap(),
                });

            if index < 8 {
                self.stops[index] = Some(ColorStop { offset, color });
            }
        } else {
            log::warn!("Gradient: ColorStop must be within 0.0..=1.0 range.");
        };

        self
    }

    /// 向渐变添加多个 [`ColorStop`]。
    ///
    /// 第 8 个之后添加的色标都会被静默忽略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adds multiple [`ColorStop`]s to the gradient.
    ///
    /// Any stop added after the 8th will be silently ignored.
    ///
    /// </details>
    pub fn add_stops(
        mut self,
        stops: impl IntoIterator<Item = ColorStop>,
    ) -> Self {
        for stop in stops {
            self = self.add_stop(stop.offset, stop.color);
        }

        self
    }

    /// 打包 [`Gradient`]，以便在着色器代码中使用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Packs the [`Gradient`] for use in shader code.
    ///
    /// </details>
    pub fn pack(&self) -> Packed {
        let mut colors = [[0u32; 2]; 8];
        let mut offsets = [f16::from(0u8); 8];

        for (index, stop) in self.stops.iter().enumerate() {
            let [r, g, b, a] =
                color::pack(stop.map_or(Color::default(), |s| s.color))
                    .components();

            colors[index] = [
                pack_f16s([f16::from_f32(r), f16::from_f32(g)]),
                pack_f16s([f16::from_f32(b), f16::from_f32(a)]),
            ];

            offsets[index] =
                stop.map_or(f16::from_f32(2.0), |s| f16::from_f32(s.offset));
        }

        let offsets = [
            pack_f16s([offsets[0], offsets[1]]),
            pack_f16s([offsets[2], offsets[3]]),
            pack_f16s([offsets[4], offsets[5]]),
            pack_f16s([offsets[6], offsets[7]]),
        ];

        let direction = [self.start.x, self.start.y, self.end.x, self.end.y];

        Packed {
            colors,
            offsets,
            direction,
        }
    }
}

/// 打包后的 [`Gradient`] 数据，供着色器代码使用。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Packed [`Gradient`] data for use in shader code.
///
/// </details>
#[derive(Debug, Copy, Clone, PartialEq, Zeroable, Pod)]
#[repr(C)]
pub struct Packed {
    // 8 colors, each channel = 16 bit float, 2 colors packed into 1 u32
    // 8 个颜色，每个通道为 16 位浮点数，每 2 个颜色打包进 1 个 u32
    colors: [[u32; 2]; 8],
    // 8 offsets, 8x 16 bit floats packed into 4 u32s
    // 8 个偏移量，8 个 16 位浮点数打包进 4 个 u32
    offsets: [u32; 4],
    direction: [f32; 4],
}

/// 创建一个新的 [`Packed`] 渐变，供着色器代码使用。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Packed`] gradient for use in shader code.
///
/// </details>
pub fn pack(gradient: &core::Gradient, bounds: Rectangle) -> Packed {
    match gradient {
        core::Gradient::Linear(linear) => {
            let mut colors = [[0u32; 2]; 8];
            let mut offsets = [f16::from(0u8); 8];

            for (index, stop) in linear.stops.iter().enumerate() {
                let [r, g, b, a] =
                    color::pack(stop.map_or(Color::default(), |s| s.color))
                        .components();

                colors[index] = [
                    pack_f16s([f16::from_f32(r), f16::from_f32(g)]),
                    pack_f16s([f16::from_f32(b), f16::from_f32(a)]),
                ];

                offsets[index] = stop
                    .map_or(f16::from_f32(2.0), |s| f16::from_f32(s.offset));
            }

            let offsets = [
                pack_f16s([offsets[0], offsets[1]]),
                pack_f16s([offsets[2], offsets[3]]),
                pack_f16s([offsets[4], offsets[5]]),
                pack_f16s([offsets[6], offsets[7]]),
            ];

            let (start, end) = linear.angle.to_distance(&bounds);

            let direction = [start.x, start.y, end.x, end.y];

            Packed {
                colors,
                offsets,
                direction,
            }
        }
    }
}

/// 把两个 f16 打包进一个 u32。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Packs two f16s into one u32.
///
/// </details>
fn pack_f16s(f: [f16; 2]) -> u32 {
    let one = (f[0].to_bits() as u32) << 16;
    let two = f[1].to_bits() as u32;

    one | two
}
