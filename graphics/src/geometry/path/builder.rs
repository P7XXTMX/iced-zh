use crate::geometry::path::{Arc, Path, arc};

use crate::core::border;
use crate::core::{Point, Radians, Size};

use lyon_path::builder::{self, SvgPathBuilder};
use lyon_path::geom;
use lyon_path::math;

/// [`Path`] 构建器。
///
/// [`Path`] 一旦构建完成，就不能再被修改。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A [`Path`] builder.
///
/// Once a [`Path`] is built, it can no longer be mutated.
///
/// </details>
pub struct Builder {
    raw: builder::WithSvg<lyon_path::path::BuilderImpl>,
}

impl Builder {
    /// 创建一个新的 [`Builder`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Builder`].
    ///
    /// </details>
    pub fn new() -> Builder {
        Builder {
            raw: lyon_path::Path::builder().with_svg(),
        }
    }

    /// 把新子路径的起点移动到给定的 `Point`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Moves the starting point of a new sub-path to the given `Point`.
    ///
    /// </details>
    #[inline]
    pub fn move_to(&mut self, point: Point) {
        let _ = self.raw.move_to(math::Point::new(point.x, point.y));
    }

    /// 用一条直线把 [`Path`] 中的最后一个点连接到
    /// 给定的 `Point`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Connects the last point in the [`Path`] to the given `Point` with a
    /// straight line.
    ///
    /// </details>
    #[inline]
    pub fn line_to(&mut self, point: Point) {
        let _ = self.raw.line_to(math::Point::new(point.x, point.y));
    }

    /// 向 [`Path`] 添加一段 [`Arc`]，沿顺时针方向
    /// 从 `start_angle` 到 `end_angle`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adds an [`Arc`] to the [`Path`] from `start_angle` to `end_angle` in
    /// a clockwise direction.
    ///
    /// </details>
    #[inline]
    pub fn arc(&mut self, arc: Arc) {
        self.ellipse(arc.into());
    }

    /// 使用给定的控制点和半径向 [`Path`] 添加一段
    /// 圆弧。
    ///
    /// 这实质上是从当前位置向 `a` 画一条直线段，
    /// 但会嵌入一段半径为 `radius` 的圆弧，该圆弧既与这条线段相切，
    /// 也与 `a` 和 `b` 之间的连线相切。
    ///
    /// 再调用一次 `.line_to(b)`，得到的路径就会连接
    /// 起点和 `b`：先是朝向 `a` 的直线段，
    /// 并由一段圆弧把 `a` 处的拐角变得平滑。
    ///
    /// 更多细节和示例参见
    /// [HTML5 规范中的 `arcTo`](https://html.spec.whatwg.org/multipage/canvas.html#building-paths:dom-context-2d-arcto)。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adds a circular arc to the [`Path`] with the given control points and
    /// radius.
    ///
    /// This essentially draws a straight line segment from the current
    /// position to `a`, but fits a circular arc of `radius` tangent to that
    /// segment and tangent to the line between `a` and `b`.
    ///
    /// With another `.line_to(b)`, the result will be a path connecting the
    /// starting point and `b` with straight line segments towards `a` and a
    /// circular arc smoothing out the corner at `a`.
    ///
    /// See [the HTML5 specification of `arcTo`](https://html.spec.whatwg.org/multipage/canvas.html#building-paths:dom-context-2d-arcto)
    /// for more details and examples.
    ///
    /// </details>
    pub fn arc_to(&mut self, a: Point, b: Point, radius: f32) {
        let start = self.raw.current_position();
        let mid = math::Point::new(a.x, a.y);
        let end = math::Point::new(b.x, b.y);

        if start == mid || mid == end || radius == 0.0 {
            let _ = self.raw.line_to(mid);
            return;
        }

        let double_area =
            start.x * (mid.y - end.y) + mid.x * (end.y - start.y) + end.x * (start.y - mid.y);

        if double_area == 0.0 {
            let _ = self.raw.line_to(mid);
            return;
        }

        let to_start = (start - mid).normalize();
        let to_end = (end - mid).normalize();

        let inner_angle = to_start.dot(to_end).acos();

        let origin_angle = inner_angle / 2.0;

        let origin_adjacent = radius / origin_angle.tan();

        let arc_start = mid + to_start * origin_adjacent;
        let arc_end = mid + to_end * origin_adjacent;

        let sweep = to_start.cross(to_end) < 0.0;

        let _ = self.raw.line_to(arc_start);

        self.raw.arc_to(
            math::Vector::new(radius, radius),
            math::Angle::radians(0.0),
            lyon_path::ArcFlags {
                large_arc: false,
                sweep,
            },
            arc_end,
        );
    }

    /// 沿顺时针方向向 [`Path`] 添加一个椭圆。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adds an ellipse to the [`Path`] using a clockwise direction.
    ///
    /// </details>
    pub fn ellipse(&mut self, arc: arc::Elliptical) {
        let arc = geom::Arc {
            center: math::Point::new(arc.center.x, arc.center.y),
            radii: math::Vector::new(arc.radii.x, arc.radii.y),
            x_rotation: math::Angle::radians(arc.rotation.0),
            start_angle: math::Angle::radians(arc.start_angle.0),
            sweep_angle: math::Angle::radians((arc.end_angle - arc.start_angle).0),
        };

        let _ = self.raw.move_to(arc.sample(0.0));

        arc.cast::<f64>().for_each_quadratic_bezier(&mut |curve| {
            let curve = curve.cast::<f32>();
            let _ = self.raw.quadratic_bezier_to(curve.ctrl, curve.to);
        });
    }

    /// 根据两个控制点和终点，向 [`Path`] 添加一条
    /// 三次贝塞尔曲线。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adds a cubic Bézier curve to the [`Path`] given its two control points
    /// and its end point.
    ///
    /// </details>
    #[inline]
    pub fn bezier_curve_to(&mut self, control_a: Point, control_b: Point, to: Point) {
        let _ = self.raw.cubic_bezier_to(
            math::Point::new(control_a.x, control_a.y),
            math::Point::new(control_b.x, control_b.y),
            math::Point::new(to.x, to.y),
        );
    }

    /// 根据控制点和终点，向 [`Path`] 添加一条
    /// 二次贝塞尔曲线。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adds a quadratic Bézier curve to the [`Path`] given its control point
    /// and its end point.
    ///
    /// </details>
    #[inline]
    pub fn quadratic_curve_to(&mut self, control: Point, to: Point) {
        let _ = self.raw.quadratic_bezier_to(
            math::Point::new(control.x, control.y),
            math::Point::new(to.x, to.y),
        );
    }

    /// 根据左上角坐标和 `Size`，向 [`Path`] 添加一个
    /// 矩形。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adds a rectangle to the [`Path`] given its top-left corner coordinate
    /// and its `Size`.
    ///
    /// </details>
    #[inline]
    pub fn rectangle(&mut self, top_left: Point, size: Size) {
        self.move_to(top_left);
        self.line_to(Point::new(top_left.x + size.width, top_left.y));
        self.line_to(Point::new(
            top_left.x + size.width,
            top_left.y + size.height,
        ));
        self.line_to(Point::new(top_left.x, top_left.y + size.height));
        self.close();
    }

    /// 根据左上角坐标、[`Size`] 和 [`border::Radius`]，
    /// 向 [`Path`] 添加一个圆角矩形。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adds a rounded rectangle to the [`Path`] given its top-left
    /// corner coordinate its [`Size`] and [`border::Radius`].
    ///
    /// </details>
    #[inline]
    pub fn rounded_rectangle(&mut self, top_left: Point, size: Size, radius: border::Radius) {
        let min_size = (size.height / 2.0).min(size.width / 2.0);
        let [
            top_left_corner,
            top_right_corner,
            bottom_right_corner,
            bottom_left_corner,
        ] = radius.into();

        self.move_to(Point::new(
            top_left.x + min_size.min(top_left_corner),
            top_left.y,
        ));
        self.line_to(Point::new(
            top_left.x + size.width - min_size.min(top_right_corner),
            top_left.y,
        ));
        self.arc_to(
            Point::new(top_left.x + size.width, top_left.y),
            Point::new(
                top_left.x + size.width,
                top_left.y + min_size.min(top_right_corner),
            ),
            min_size.min(top_right_corner),
        );
        self.line_to(Point::new(
            top_left.x + size.width,
            top_left.y + size.height - min_size.min(bottom_right_corner),
        ));
        self.arc_to(
            Point::new(top_left.x + size.width, top_left.y + size.height),
            Point::new(
                top_left.x + size.width - min_size.min(bottom_right_corner),
                top_left.y + size.height,
            ),
            min_size.min(bottom_right_corner),
        );
        self.line_to(Point::new(
            top_left.x + min_size.min(bottom_left_corner),
            top_left.y + size.height,
        ));
        self.arc_to(
            Point::new(top_left.x, top_left.y + size.height),
            Point::new(
                top_left.x,
                top_left.y + size.height - min_size.min(bottom_left_corner),
            ),
            min_size.min(bottom_left_corner),
        );
        self.line_to(Point::new(
            top_left.x,
            top_left.y + min_size.min(top_left_corner),
        ));
        self.arc_to(
            Point::new(top_left.x, top_left.y),
            Point::new(top_left.x + min_size.min(top_left_corner), top_left.y),
            min_size.min(top_left_corner),
        );
        self.close();
    }

    /// 根据圆心坐标和半径，向 [`Path`] 添加一个
    /// 圆。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adds a circle to the [`Path`] given its center coordinate and its
    /// radius.
    ///
    /// </details>
    #[inline]
    pub fn circle(&mut self, center: Point, radius: f32) {
        self.arc(Arc {
            center,
            radius,
            start_angle: Radians(0.0),
            end_angle: Radians(2.0 * std::f32::consts::PI),
        });
    }

    /// 用一条连到起点的直线闭合 [`Path`] 中的
    /// 当前子路径。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Closes the current sub-path in the [`Path`] with a straight line to
    /// the starting point.
    ///
    /// </details>
    #[inline]
    pub fn close(&mut self) {
        self.raw.close();
    }

    /// 构建此 [`Builder`] 的 [`Path`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Builds the [`Path`] of this [`Builder`].
    ///
    /// </details>
    #[inline]
    pub fn build(self) -> Path {
        Path {
            raw: self.raw.build(),
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}
