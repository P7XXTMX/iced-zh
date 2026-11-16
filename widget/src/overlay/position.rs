use crate::core::{Point, Rectangle, Size};

/// 弹出框（弹出层或工具提示）相对于它所锚定的元素
/// 的位置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The position of a popup (a popover or a tooltip) relative to the element
/// it is anchored to.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    /// 如果首选的 [`Side`] 放得下，弹出框就出现在那一侧，
    /// 否则出现在部件可用空间最大的一侧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The popup will appear on the preferred [`Side`] if it fits there, and
    /// on the side of the widget with the most available space otherwise.
    ///
    /// </details>
    Auto {
        /// 弹出框放得下时首选的 [`Side`]。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The [`Side`] to prefer when the popup fits there.
        ///
        /// </details>
        preference: Side,
    },
    /// 弹出框出现在部件的上方。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The popup will appear on the top of the widget.
    ///
    /// </details>
    Top,
    /// 弹出框出现在部件的下方。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The popup will appear on the bottom of the widget.
    ///
    /// </details>
    Bottom,
    /// 弹出框出现在部件的左侧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The popup will appear on the left of the widget.
    ///
    /// </details>
    Left,
    /// 弹出框出现在部件的右侧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The popup will appear on the right of the widget.
    ///
    /// </details>
    Right,
    /// 弹出框跟随光标。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The popup will follow the cursor.
    ///
    /// </details>
    FollowCursor,
}

impl Default for Position {
    fn default() -> Self {
        Position::Auto {
            preference: Side::default(),
        }
    }
}

impl Position {
    /// 计算弹出框在某个 [`Position`] 下所占据的 [`Rectangle`]。
    ///
    /// 所有参数都使用同一个坐标空间：
    /// 也就是对弹出框进行布局的空间。`content_bounds` 描述
    /// 弹出框所锚定的元素，`popup` 是弹出框自身的大小，
    /// `gap` 是在它周围留出的空间大小。
    ///
    /// `cursor_position` 只在 [`Position`] 为
    /// [`Position::FollowCursor`] 时使用；`viewport` 只在它为
    /// [`Position::Auto`] 或 `snap_within_viewport` 为 `true` 时使用，
    /// 后者会把计算出的矩形限制在 `viewport` 之内。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Computes the [`Rectangle`] that a popup occupies for a [`Position`].
    ///
    /// All the arguments are expressed in the same coordinate space: the
    /// space in which the popup is laid out. The `content_bounds` describe
    /// the element the popup is anchored to, `popup` is the size of the
    /// popup itself, and `gap` is the amount of space to leave around it.
    ///
    /// The `cursor_position` is only used when the [`Position`] is
    /// [`Position::FollowCursor`], and the `viewport` is only used when it is
    /// [`Position::Auto`] or when `snap_within_viewport` is `true`, which
    /// clamps the resolved rectangle into the `viewport`.
    ///
    /// </details>
    pub fn resolve(
        &self,
        content_bounds: Rectangle,
        popup: Size,
        gap: f32,
        cursor_position: Point,
        viewport: Rectangle,
        snap_within_viewport: bool,
    ) -> Rectangle {
        let offset = match self {
            // The popup follows the cursor.
            // 弹出框跟随光标。
            Position::FollowCursor => {
                Point::new(cursor_position.x, cursor_position.y - popup.height)
            }
            // The popup is placed on a side of the base; `Auto` resolves to
            // the preferred side if the popup fits there, and to the side
            // with the most available space otherwise.
            // 弹出框放在基底元素的某一侧；`Auto` 在首选一侧放得下时
            // 解析为首选的一侧，否则解析为
            // 可用空间最大的一侧。
            Position::Auto { preference } => {
                Side::with_most_available_space(content_bounds, viewport, popup, gap, *preference)
                    .offset(content_bounds, popup, gap)
            }
            Position::Top => Side::Top.offset(content_bounds, popup, gap),
            Position::Bottom => Side::Bottom.offset(content_bounds, popup, gap),
            Position::Left => Side::Left.offset(content_bounds, popup, gap),
            Position::Right => Side::Right.offset(content_bounds, popup, gap),
        };

        let mut rectangle = Rectangle {
            x: offset.x,
            y: offset.y,
            width: popup.width,
            height: popup.height,
        };

        if snap_within_viewport {
            if rectangle.x < viewport.x {
                rectangle.x = viewport.x;
            } else if viewport.x + viewport.width < rectangle.x + rectangle.width {
                rectangle.x = viewport.x + viewport.width - rectangle.width;
            }

            if rectangle.y < viewport.y {
                rectangle.y = viewport.y;
            } else if viewport.y + viewport.height < rectangle.y + rectangle.height {
                rectangle.y = viewport.y + viewport.height - rectangle.height;
            }
        }

        rectangle
    }
}

/// 弹出框可以出现的一侧。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A side that a popup can appear on.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Side {
    /// 上侧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The top side.
    ///
    /// </details>
    Top,
    /// 左侧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The left side.
    ///
    /// </details>
    Left,
    /// 右侧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The right side.
    ///
    /// </details>
    Right,
    /// 下侧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The bottom side.
    ///
    /// </details>
    #[default]
    Bottom,
}

impl Side {
    fn with_most_available_space(
        content_bounds: Rectangle,
        viewport: Rectangle,
        popup: Size,
        gap: f32,
        preference: Side,
    ) -> Side {
        // The popup is placed on the preferred side if it fits there.
        // 如果首选的一侧放得下，弹出框就放在那一侧。
        if preference.available_space(content_bounds, viewport, popup, gap) >= 0.0 {
            return preference;
        }

        // Otherwise, it is placed on the side with the most available space.
        // 否则，放在可用空间最大的一侧。
        [Side::Top, Side::Left, Side::Right, Side::Bottom]
            .into_iter()
            .max_by(|a, b| {
                a.available_space(content_bounds, viewport, popup, gap)
                    .total_cmp(&b.available_space(content_bounds, viewport, popup, gap))
            })
            .unwrap_or_default()
    }

    fn available_space(
        self,
        content_bounds: Rectangle,
        viewport: Rectangle,
        popup: Size,
        gap: f32,
    ) -> f32 {
        match self {
            Side::Top => content_bounds.y - viewport.y - popup.height - gap,
            Side::Bottom => {
                (viewport.y + viewport.height)
                    - (content_bounds.y + content_bounds.height)
                    - popup.height
                    - gap
            }
            Side::Left => content_bounds.x - viewport.x - popup.width - gap,
            Side::Right => {
                (viewport.x + viewport.width)
                    - (content_bounds.x + content_bounds.width)
                    - popup.width
                    - gap
            }
        }
    }

    fn offset(self, content_bounds: Rectangle, popup: Size, gap: f32) -> Point {
        let x_center = content_bounds.x + (content_bounds.width - popup.width) / 2.0;
        let y_center = content_bounds.y + (content_bounds.height - popup.height) / 2.0;

        match self {
            Side::Top => Point::new(x_center, content_bounds.y - popup.height - gap),
            Side::Bottom => Point::new(x_center, content_bounds.y + content_bounds.height + gap),
            Side::Left => Point::new(content_bounds.x - popup.width - gap, y_center),
            Side::Right => Point::new(content_bounds.x + content_bounds.width + gap, y_center),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Position, Side};
    use crate::core::{Point, Rectangle, Size};

    /// 通用输入：位于原点的 50x50 基底元素、80x80 的弹出框、没有间隙、
    /// 位于原点的光标，以及 1000x1000 的视口。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Common inputs: a 50x50 base at the origin, an 80x80 popup, no gap, a
    /// cursor at the origin, and a 1000x1000 viewport.
    ///
    /// </details>
    const BASE: Rectangle = Rectangle::new(Point::new(0.0, 0.0), Size::new(50.0, 50.0));
    const POPUP: Size = Size::new(80.0, 80.0);
    const GAP: f32 = 0.0;
    const CURSOR: Point = Point::new(0.0, 0.0);
    const VIEWPORT: Rectangle = Rectangle::new(Point::new(0.0, 0.0), Size::new(1000.0, 1000.0));

    #[test]
    fn auto_uses_the_preferred_side_when_it_fits() {
        // There is more space above (520) than below (270); the preference
        // still wins because the popup fits below.
        // 上方的空间（520）比下方（270）更大；但弹出框在下方放得下，
        // 所以首选项仍然生效。
        let base = Rectangle::new(Point::new(0.0, 600.0), Size::new(50.0, 50.0));
        let position = Position::Auto {
            preference: Side::Bottom,
        };

        let rect = position.resolve(base, POPUP, GAP, CURSOR, VIEWPORT, false);

        assert_eq!(
            rect,
            Rectangle::new(Point::new(-15.0, 650.0), Size::new(80.0, 80.0)),
            "Auto should use the preferred side when the popup fits there"
        );
    }

    #[test]
    fn auto_falls_back_to_the_side_with_the_most_space() {
        // The preferred side (`Bottom`) does not fit: the popup (960) is
        // taller than the space below the base (950), so the popup is placed
        // on the side with the most available space (right) instead.
        // 首选的一侧（`Bottom`）放不下：弹出框（960）比
        // 基底元素下方的空间（950）更高，因此弹出框改为放在
        // 可用空间最大的一侧（右侧）。
        let popup = Size::new(80.0, 960.0);
        let position = Position::Auto {
            preference: Side::Bottom,
        };

        let rect = position.resolve(BASE, popup, GAP, CURSOR, VIEWPORT, false);

        assert_eq!(
            rect,
            Rectangle::new(Point::new(50.0, -455.0), Size::new(80.0, 960.0)),
            "Auto should fall back to the side with the most available space"
        );
    }

    #[test]
    fn top_places_the_popup_above_the_base() {
        let rect = Position::Top.resolve(BASE, POPUP, GAP, CURSOR, VIEWPORT, false);

        assert_eq!(
            rect,
            Rectangle::new(Point::new(-15.0, -80.0), Size::new(80.0, 80.0)),
            "Top should place the popup above the base"
        );
    }

    #[test]
    fn bottom_places_the_popup_below_the_base() {
        let rect = Position::Bottom.resolve(BASE, POPUP, GAP, CURSOR, VIEWPORT, false);

        assert_eq!(
            rect,
            Rectangle::new(Point::new(-15.0, 50.0), Size::new(80.0, 80.0)),
            "Bottom should place the popup below the base"
        );
    }

    #[test]
    fn left_places_the_popup_left_of_the_base() {
        let rect = Position::Left.resolve(BASE, POPUP, GAP, CURSOR, VIEWPORT, false);

        assert_eq!(
            rect,
            Rectangle::new(Point::new(-80.0, -15.0), Size::new(80.0, 80.0)),
            "Left should place the popup to the left of the base"
        );
    }

    #[test]
    fn right_places_the_popup_right_of_the_base() {
        let rect = Position::Right.resolve(BASE, POPUP, GAP, CURSOR, VIEWPORT, false);

        assert_eq!(
            rect,
            Rectangle::new(Point::new(50.0, -15.0), Size::new(80.0, 80.0)),
            "Right should place the popup to the right of the base"
        );
    }

    #[test]
    fn follow_cursor_places_the_popup_at_the_cursor() {
        let cursor = Point::new(200.0, 300.0);

        // The popup's bottom edge is placed at the cursor.
        // 弹出框的下边缘放在光标处。
        let rect = Position::FollowCursor.resolve(BASE, POPUP, GAP, cursor, VIEWPORT, false);

        assert_eq!(
            rect,
            Rectangle::new(Point::new(200.0, 220.0), Size::new(80.0, 80.0)),
            "FollowCursor should place the popup at the cursor"
        );
    }

    #[test]
    fn snap_clamps_the_popup_into_the_viewport() {
        // Without snapping, `Top` places the popup at (-15, -80), outside of
        // the viewport; with snapping, it is clamped to the viewport's
        // origin.
        // 不吸附时，`Top` 会把弹出框放在 (-15, -80)，位于
        // 视口之外；吸附时，它会被限制到视口的
        // 原点。
        let rect = Position::Top.resolve(BASE, POPUP, GAP, CURSOR, VIEWPORT, true);

        assert_eq!(
            rect,
            Rectangle::new(Point::new(0.0, 0.0), Size::new(80.0, 80.0)),
            "Snap should clamp the popup into the viewport"
        );
    }

    #[test]
    fn snap_shifts_a_popup_that_overflows_the_viewport() {
        // A base flush with the right edge of the viewport (1000).
        // 紧贴视口右边缘（1000）的基底元素。
        let base = Rectangle::new(Point::new(950.0, 0.0), Size::new(50.0, 50.0));

        // Without snapping, `Right` places the popup at (1000, -15), beyond
        // the right edge of the viewport; with snapping, it is shifted back
        // so it fits.
        // 不吸附时，`Right` 会把弹出框放在 (1000, -15)，超出
        // 视口的右边缘；吸附时，它会被移回来，
        // 使其能够放得下。
        let rect = Position::Right.resolve(base, POPUP, GAP, CURSOR, VIEWPORT, true);

        assert_eq!(
            rect,
            Rectangle::new(Point::new(920.0, 0.0), Size::new(80.0, 80.0)),
            "Snap should shift a popup that overflows the viewport"
        );
    }
}
