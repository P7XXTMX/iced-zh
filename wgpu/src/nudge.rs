//! 把坐标吸附到物理像素网格。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Snapping coordinates to the physical pixel grid.
//!
//! </details>
//!
//! # 为什么需要微调（Why the nudge）
//!
//! 吸附的目的是让图元的边缘落在像素网格上，
//! 渲染成清晰、完全不透明的像素，而不是抗锯齿的接缝（参见
//! [`CRISP`]）。但是 `round` 的判定边界（半像素）在浮点运算下
//! 并不稳定：一个本应恰好落在半像素上的坐标
//! （例如 2 倍缩放下的 `5.25` 逻辑像素，
//! 即 `10.5` 物理像素）算出来可能略小一点（例如 `10.4998`），
//! 这时普通的 `round` 会向下取整。于是这条边就离开了网格，
//! 出现吸附本来要消除的抗锯齿接缝，并且与干净地算出同一边界的
//! 相邻图元相差一个像素。
//!
//! 而且半像素在 UI 中很常见，并不是极端情况：居中和
//! 等分空间在布局中随处可见，而 `.25` 逻辑像素
//! 在 2 倍缩放下恰好落在物理半像素上。
//!
//! 在取整之前把每条边微调 [`NUDGE`]，就把判定边界下移
//! 到了 `.499`：任何比半像素小不超过 `0.001` 的坐标现在都会向上
//! 取整到网格上，因此浮点噪声不再能把一条
//! 半像素边缘从网格上分离出去。离边界更远的坐标
//! 不受影响。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Snapping exists so that primitive edges land on the pixel grid and
//! render as crisp, fully opaque pixels instead of anti-aliased seams (see
//! [`CRISP`]). But `round`'s decision boundary — the half-pixel — is not
//! stable under floating-point arithmetic: a coordinate meant to land
//! exactly on a half-pixel (e.g. `5.25` logical pixels at 2× scale,
//! `10.5` physical) can be computed a hair below it (e.g. `10.4998`),
//! where plain `round` rounds down. The edge is then off the grid — the
//! anti-aliased seam that snapping exists to remove — and one pixel away
//! from its neighbours, which computed the same boundary cleanly.
//!
//! And half-pixels are common in UIs, not a corner case: centering and
//! equal space distribution are constant in layouts, and `.25` logical
//! pixels land exactly on physical half-pixels at 2× scale.
//!
//! Nudging each edge by [`NUDGE`] before rounding pulls the boundary down
//! to `.499`: any coordinate within `0.001` below a half-pixel now rounds
//! up onto the grid, so floating-point noise can no longer split a
//! half-pixel edge off the grid. Coordinates further from the boundary are
//! unaffected.
//!
//! </details>
//!
//! # 一致性（Consistency）
//!
//! 同样的约定（取整之前微调 [`NUDGE`]）也用于
//! `quad` 顶点着色器（参见 `shader/quad/snap.wgsl`），以及
//! CPU 一侧对这些坐标的每一处吸附（四边形、三角形、
//! 图像和文本通道的裁剪（scissor）边界）。两者绝不能出现分歧，
//! 否则裁剪边界可能与它所裁剪的已吸附边缘相差一个像素。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! The same convention — nudging by [`NUDGE`] before rounding — is applied
//! by the `quad` vertex shader (see `shader/quad/snap.wgsl`) and by every
//! CPU-side snapping of these coordinates (the clip (scissor) bounds of the
//! quad, triangle, image and text passes). The two must never diverge, or
//! clip bounds may end up one pixel off from the snapped edges they clip.
//!
//! </details>
//!
//! [`CRISP`]: crate::core::renderer::CRISP
use crate::core::{Rectangle, Vector};

/// 把坐标吸附到像素网格时施加的微调量。
///
/// 有了它，恰好落在半像素边界下方一点的坐标
/// 会向上取整。
///
/// 它必须与 `shader/quad/snap.wgsl` 中的 `nudge`
/// 保持同步。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The nudge applied when snapping a coordinate to the pixel grid.
///
/// Coordinates landing just below a half-pixel boundary round up because
/// of it.
///
/// This must stay in sync with `nudge` in
/// `shader/quad/snap.wgsl`.
///
/// </details>
pub const NUDGE: f32 = 0.001;

/// 把 `bounds` 吸附到物理像素网格。
///
/// 它使用与 `quad` 顶点着色器相同的取整约定
/// （参见 `shader/quad/snap.wgsl`）：每条边在取整之前先微调 [`NUDGE`]，
/// 这样恰好落在半像素边界下方一点的坐标
/// 会向上取整。
///
/// 这保证了裁剪（scissor）边界与在其中绘制的 [`Quad`] 图元
/// 吸附后的边缘完全对齐。
///
/// 不足一个像素的结果会被 [`Rectangle::snap`] 自身拒绝。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Snaps `bounds` to the physical pixel grid.
///
/// It uses the same rounding convention as the `quad` vertex shader
/// (see `shader/quad/snap.wgsl`): each edge is nudged by [`NUDGE`] before
/// being rounded, so that coordinates landing just below a half-pixel
/// boundary round up.
///
/// This ensures that clip (scissor) bounds align exactly with the snapped
/// edges of the [`Quad`] primitives drawn inside them.
///
/// Sub-pixel results are rejected, by [`Rectangle::snap`] itself.
///
/// </details>
///
/// [`Quad`]: crate::core::renderer::Quad
pub fn snap(bounds: Rectangle) -> Option<Rectangle<u32>> {
    (bounds + Vector::new(NUDGE, NUDGE)).snap()
}

/// 把 `bounds` 吸附到物理像素网格，并保留带符号的坐标。
///
/// 它使用与 [`snap`] 相同的“先微调再取整”约定，但保留
/// 浮点坐标，因此负值会被保留，而不是
/// 被饱和到零。
///
/// 它适用于可以合理地超出视口左上角的坐标，
/// 例如滚动到视野之外的内容的边界：
/// 这些内容之后会被其周围环境裁剪掉。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Snaps `bounds` to the physical pixel grid, keeping signed coordinates.
///
/// It applies the same nudge-and-round convention as [`snap`], but keeps
/// floating-point coordinates, so negative values are preserved instead of
/// being saturated to zero.
///
/// Use this for coordinates that can legitimately extend beyond the
/// top-left corner of the viewport, like the bounds of content scrolled out
/// of view: those are clipped afterwards by their surroundings.
///
/// </details>
#[cfg(any(feature = "image", feature = "svg"))]
pub fn round(bounds: Rectangle) -> Rectangle {
    (bounds + Vector::new(NUDGE, NUDGE)).round()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Point, Size};

    /// 滚动到视口上方、离开视野的图像在吸附时必须保留
    /// 负的位置；把它饱和到原点会让它
    /// 卡在可滚动区域的上边缘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An image scrolled out of view above the viewport must keep its
    /// negative position when snapped; saturating it to the origin would
    /// leave it stuck to the upper edge of the scrollable.
    ///
    /// </details>
    #[test]
    fn round_keeps_negative_coordinates() {
        let bounds = Rectangle::new(Point::new(10.0, -50.0), Size::new(100.0, 100.0));

        assert_eq!(
            round(bounds),
            Rectangle::new(Point::new(10.0, -50.0), Size::new(100.0, 100.0))
        );
    }

    /// 落在半像素边界下方一点点的坐标必须向上取整
    /// 到网格上，与 `quad` 顶点着色器一致。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Coordinates landing a hair below a half-pixel boundary must round
    /// up onto the grid, like the `quad` vertex shader.
    ///
    /// </details>
    #[test]
    fn round_applies_the_nudge() {
        let bounds = Rectangle::new(Point::new(0.0, 10.4998), Size::new(10.0, 1.0));

        // Without the nudge, plain `round` would land a pixel off:
        // 没有微调时，普通的 `round` 会偏差一个像素：
        assert_eq!(bounds.y.round(), 10.0);

        // With it, the boundary rounds up, matching the GPU:
        // 有了微调，边界处向上取整，与 GPU 一致：
        assert_eq!(round(bounds).y, 11.0);
    }

    /// `snap` 会把负坐标饱和到零，因此它只能
    /// 用于已知位于视口之内的边界，例如
    /// 裁剪矩形。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// `snap` saturates negative coordinates to zero, so it must only be
    /// used on bounds that are known to be within the viewport, like
    /// scissor rects.
    ///
    /// </details>
    #[test]
    fn snap_saturates_negative_coordinates() {
        let bounds = Rectangle::new(Point::new(10.0, -50.0), Size::new(100.0, 100.0));

        let snapped = snap(bounds).expect("Visible rectangle");

        assert_eq!(snapped.y, 0);
        assert_eq!(round(bounds).y, -50.0);
    }
}
