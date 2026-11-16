//! 构建并绘制曲线。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Build and draw curves.
//!
//! </details>
use iced_core::{Point, Radians, Vector};

/// 可微曲线的一段。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A segment of a differentiable curve.
///
/// </details>
#[derive(Debug, Clone, Copy)]
pub struct Arc {
    /// 圆弧的圆心。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The center of the arc.
    ///
    /// </details>
    pub center: Point,
    /// 圆弧的半径。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The radius of the arc.
    ///
    /// </details>
    pub radius: f32,
    /// 这段圆弧的起始角度，从 x 轴正方向起按顺时针旋转。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The start of the segment's angle, clockwise rotation from positive x-axis.
    ///
    /// </details>
    pub start_angle: Radians,
    /// 这段圆弧的结束角度，从 x 轴正方向起按顺时针旋转。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The end of the segment's angle, clockwise rotation from positive x-axis.
    ///
    /// </details>
    pub end_angle: Radians,
}

/// 椭圆形的 [`Arc`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An elliptical [`Arc`].
///
/// </details>
#[derive(Debug, Clone, Copy)]
pub struct Elliptical {
    /// 圆弧的中心。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The center of the arc.
    ///
    /// </details>
    pub center: Point,
    /// 圆弧所在椭圆的半径。椭圆的水平和垂直半轴长度分别对应半径向量的 x 值和 y 值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The radii of the arc's ellipse. The horizontal and vertical half-dimensions of the ellipse will match the x and y values of the radii vector.
    ///
    /// </details>
    pub radii: Vector,
    /// 圆弧所在椭圆的顺时针旋转角度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The clockwise rotation of the arc's ellipse.
    ///
    /// </details>
    pub rotation: Radians,
    /// 这段圆弧的起始角度，从 x 轴正方向起按顺时针旋转。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The start of the segment's angle, clockwise rotation from positive x-axis.
    ///
    /// </details>
    pub start_angle: Radians,
    /// 这段圆弧的结束角度，从 x 轴正方向起按顺时针旋转。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The end of the segment's angle, clockwise rotation from positive x-axis.
    ///
    /// </details>
    pub end_angle: Radians,
}

impl From<Arc> for Elliptical {
    fn from(arc: Arc) -> Elliptical {
        Elliptical {
            center: arc.center,
            radii: Vector::new(arc.radius, arc.radius),
            rotation: Radians(0.0),
            start_angle: arc.start_angle,
            end_angle: arc.end_angle,
        }
    }
}
