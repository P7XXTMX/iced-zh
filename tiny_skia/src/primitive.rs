use crate::core::Rectangle;

#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    /// 用某种画笔（paint）填充的路径。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A path filled with some paint.
    ///
    /// </details>
    Fill {
        /// 要填充的路径。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The path to fill.
        ///
        /// </details>
        path: tiny_skia::Path,
        /// 要使用的画笔。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The paint to use.
        ///
        /// </details>
        paint: tiny_skia::Paint<'static>,
        /// 要遵循的填充规则。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The fill rule to follow.
        ///
        /// </details>
        rule: tiny_skia::FillRule,
    },
    /// 用某种画笔（paint）描边的路径。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A path stroked with some paint.
    ///
    /// </details>
    Stroke {
        /// 要描边的路径。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The path to stroke.
        ///
        /// </details>
        path: tiny_skia::Path,
        /// 要使用的画笔。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The paint to use.
        ///
        /// </details>
        paint: tiny_skia::Paint<'static>,
        /// 描边设置。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The stroke settings.
        ///
        /// </details>
        stroke: tiny_skia::Stroke,
    },
}

impl Primitive {
    /// 返回 [`Primitive`] 的可见边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the visible bounds of the [`Primitive`].
    ///
    /// </details>
    pub fn visible_bounds(&self) -> Rectangle {
        let bounds = match self {
            Primitive::Fill { path, .. } => path.bounds(),
            Primitive::Stroke { path, .. } => path.bounds(),
        };

        Rectangle {
            x: bounds.x(),
            y: bounds.y(),
            width: bounds.width(),
            height: bounds.height(),
        }
    }
}
