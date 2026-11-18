//! 构建各种二维形状。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Build different kinds of 2D shapes.
//!
//! </details>
pub mod arc;

mod builder;

#[doc(no_inline)]
pub use arc::Arc;
pub use builder::Builder;

pub use lyon_path;

use crate::core::border;
use crate::core::{Point, Size};

/// 一组不可变的点，这些点可以相连，也可以不相连。
///
/// 单个 [`Path`] 就可以表示各种不同的二维形状！
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An immutable set of points that may or may not be connected.
///
/// A single [`Path`] can represent different kinds of 2D shapes!
///
/// </details>
#[derive(Debug, Clone)]
pub struct Path {
    raw: lyon_path::Path,
}

impl Path {
    /// 使用给定的闭包创建一个新的 [`Path`]。
    ///
    /// 使用 [`Builder`] 来配置你的 [`Path`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Path`] with the provided closure.
    ///
    /// Use the [`Builder`] to configure your [`Path`].
    ///
    /// </details>
    pub fn new(f: impl FnOnce(&mut Builder)) -> Self {
        let mut builder = Builder::new();

        // TODO: Make it pure instead of side-effect-based (?)
        // 待办：改为纯函数式，而不是基于副作用（？）
        f(&mut builder);

        builder.build()
    }

    /// 根据起点和终点创建一个表示线段的
    /// 新 [`Path`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Path`] representing a line segment given its starting
    /// and end points.
    ///
    /// </details>
    pub fn line(from: Point, to: Point) -> Self {
        Self::new(|p| {
            p.move_to(from);
            p.line_to(to);
        })
    }

    /// 根据左上角坐标和 `Size` 创建一个表示矩形的
    /// 新 [`Path`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Path`] representing a rectangle given its top-left
    /// corner coordinate and its `Size`.
    ///
    /// </details>
    pub fn rectangle(top_left: Point, size: Size) -> Self {
        Self::new(|p| p.rectangle(top_left, size))
    }

    /// 根据左上角坐标、[`Size`] 和 [`border::Radius`]
    /// 创建一个表示圆角矩形的新 [`Path`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Path`] representing a rounded rectangle given its top-left
    /// corner coordinate, its [`Size`] and [`border::Radius`].
    ///
    /// </details>
    pub fn rounded_rectangle(
        top_left: Point,
        size: Size,
        radius: border::Radius,
    ) -> Self {
        Self::new(|p| p.rounded_rectangle(top_left, size, radius))
    }

    /// 根据圆心坐标和半径创建一个表示圆的
    /// 新 [`Path`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Path`] representing a circle given its center
    /// coordinate and its radius.
    ///
    /// </details>
    pub fn circle(center: Point, radius: f32) -> Self {
        Self::new(|p| {
            p.circle(center, radius);
            p.close();
        })
    }

    /// 返回内部的 [`lyon_path::Path`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the internal [`lyon_path::Path`].
    ///
    /// </details>
    #[inline]
    pub fn raw(&self) -> &lyon_path::Path {
        &self.raw
    }

    /// 返回应用了给定变换之后的当前 [`Path`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current [`Path`] with the given transform applied to it.
    ///
    /// </details>
    #[inline]
    pub fn transform(&self, transform: &lyon_path::math::Transform) -> Path {
        Path {
            raw: self.raw.clone().transformed(transform),
        }
    }
}
