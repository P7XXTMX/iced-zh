//! 渐进过渡的颜色。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Colors that transition progressively.
//!
//! </details>
use crate::{Color, Radians};

use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
/// 一种沿某个方向渐进过渡颜色的填充，可以是线性的、径向的（待定）
/// 或锥形的（待定）。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A fill which transitions colors progressively along a direction, either linearly, radially (TBD),
/// or conically (TBD).
///
/// </details>
pub enum Gradient {
    /// 线性渐变沿着特定角度的方向对颜色进行插值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A linear gradient interpolates colors along a direction at a specific angle.
    ///
    /// </details>
    Linear(Linear),
}

impl Gradient {
    /// 按给定的系数缩放 [`Gradient`] 的 alpha 通道。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Scales the alpha channel of the [`Gradient`] by the given factor.
    ///
    /// </details>
    pub fn scale_alpha(self, factor: f32) -> Self {
        match self {
            Gradient::Linear(linear) => Gradient::Linear(linear.scale_alpha(factor)),
        }
    }
}

impl From<Linear> for Gradient {
    fn from(gradient: Linear) -> Self {
        Self::Linear(gradient)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
/// 渐变向量上的一个点，在该点处指定的 [`color`] 不与其他颜色混合。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A point along the gradient vector where the specified [`color`] is unmixed.
///
/// </details>
///
/// [`color`]: Self::color
pub struct ColorStop {
    /// 沿渐变向量的偏移量。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Offset along the gradient vector.
    ///
    /// </details>
    pub offset: f32,

    /// 渐变在指定 [`offset`] 处的颜色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The color of the gradient at the specified [`offset`].
    ///
    /// </details>
    ///
    /// [`offset`]: Self::offset
    pub color: Color,
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
    /// [`Gradient`] 在其边界内的倾斜角度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// How the [`Gradient`] is angled within its bounds.
    ///
    /// </details>
    pub angle: Radians,
    /// 线性渐变路径上的各个 [`ColorStop`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// [`ColorStop`]s along the linear gradient path.
    ///
    /// </details>
    pub stops: [Option<ColorStop>; 8],
}

impl Linear {
    /// 使用给定的角度（[`Radians`]）创建一个新的 [`Linear`] 渐变。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Linear`] gradient with the given angle in [`Radians`].
    ///
    /// </details>
    pub fn new(angle: impl Into<Radians>) -> Self {
        Self {
            angle: angle.into(),
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
            let (Ok(index) | Err(index)) = self.stops.binary_search_by(|stop| match stop {
                None => Ordering::Greater,
                Some(stop) => stop.offset.partial_cmp(&offset).unwrap(),
            });

            if index < 8 {
                self.stops[index] = Some(ColorStop { offset, color });
            }
        } else {
            log::warn!("Gradient color stop must be within 0.0..=1.0 range.");
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
    pub fn add_stops(mut self, stops: impl IntoIterator<Item = ColorStop>) -> Self {
        for stop in stops {
            self = self.add_stop(stop.offset, stop.color);
        }

        self
    }

    /// 按给定的系数缩放 [`Linear`] 渐变的 alpha
    /// 通道。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Scales the alpha channel of the [`Linear`] gradient by the given
    /// factor.
    ///
    /// </details>
    pub fn scale_alpha(mut self, factor: f32) -> Self {
        for stop in self.stops.iter_mut().flatten() {
            stop.color.a *= factor;
        }

        self
    }
}
