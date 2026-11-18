use crate::{Point, Rectangle, Transformation, Vector};

/// 鼠标光标的状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The mouse cursor state.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Cursor {
    /// 光标有确定的位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The cursor has a defined position.
    ///
    /// </details>
    Available(Point),

    /// 光标有确定的位置，但它悬浮在上方的某个图层之上。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The cursor has a defined position, but it's levitating over a layer above.
    ///
    /// </details>
    Levitating(Point),

    /// 光标当前不可用（即超出边界或正忙）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The cursor is currently unavailable (i.e. out of bounds or busy).
    ///
    /// </details>
    #[default]
    Unavailable,
}

impl Cursor {
    /// 如果可用，返回 [`Cursor`] 的绝对位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the absolute position of the [`Cursor`], if available.
    ///
    /// </details>
    pub fn position(self) -> Option<Point> {
        match self {
            Cursor::Available(position) => Some(position),
            Cursor::Levitating(_) | Cursor::Unavailable => None,
        }
    }

    /// 如果 [`Cursor`] 可用并且位于给定的边界之内，
    /// 返回它的绝对位置。
    ///
    /// 如果 [`Cursor`] 不在给定的边界之上，此方法
    /// 返回 `None`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the absolute position of the [`Cursor`], if available and inside
    /// the given bounds.
    ///
    /// If the [`Cursor`] is not over the provided bounds, this method will
    /// return `None`.
    ///
    /// </details>
    pub fn position_over(self, bounds: Rectangle) -> Option<Point> {
        self.position().filter(|p| bounds.contains(*p))
    }

    /// 如果可用，返回 [`Cursor`] 在给定边界内的
    /// 相对位置。
    ///
    /// 如果 [`Cursor`] 不在给定的边界之上，此方法
    /// 返回 `None`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the relative position of the [`Cursor`] inside the given bounds,
    /// if available.
    ///
    /// If the [`Cursor`] is not over the provided bounds, this method will
    /// return `None`.
    ///
    /// </details>
    pub fn position_in(self, bounds: Rectangle) -> Option<Point> {
        self.position_over(bounds)
            .map(|p| p - Vector::new(bounds.x, bounds.y))
    }

    /// 如果可用，返回 [`Cursor`] 相对于给定原点的
    /// 相对位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the relative position of the [`Cursor`] from the given origin,
    /// if available.
    ///
    /// </details>
    pub fn position_from(self, origin: Point) -> Option<Point> {
        self.position().map(|p| p - Vector::new(origin.x, origin.y))
    }

    /// 如果 [`Cursor`] 位于给定的 `bounds` 之上，返回 true。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns true if the [`Cursor`] is over the given `bounds`.
    ///
    /// </details>
    pub fn is_over(self, bounds: Rectangle) -> bool {
        self.position_over(bounds).is_some()
    }

    /// 如果 [`Cursor`] 悬浮在上方的某个图层之上，返回 true。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns true if the [`Cursor`] is levitating over a layer above.
    ///
    /// </details>
    pub fn is_levitating(self) -> bool {
        matches!(self, Self::Levitating(_))
    }

    /// 让 [`Cursor`] 悬浮在上方的某个图层之上。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Makes the [`Cursor`] levitate over a layer above.
    ///
    /// </details>
    pub fn levitate(self) -> Self {
        match self {
            Self::Available(position) => Self::Levitating(position),
            _ => self,
        }
    }

    /// 把 [`Cursor`] 带回当前图层。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Brings the [`Cursor`] back to the current layer.
    ///
    /// </details>
    pub fn land(self) -> Self {
        match self {
            Cursor::Levitating(position) => Cursor::Available(position),
            _ => self,
        }
    }
}

impl std::ops::Add<Vector> for Cursor {
    type Output = Self;

    fn add(self, translation: Vector) -> Self::Output {
        match self {
            Cursor::Available(point) => Cursor::Available(point + translation),
            Cursor::Levitating(point) => {
                Cursor::Levitating(point + translation)
            }
            Cursor::Unavailable => Cursor::Unavailable,
        }
    }
}

impl std::ops::Sub<Vector> for Cursor {
    type Output = Self;

    fn sub(self, translation: Vector) -> Self::Output {
        match self {
            Cursor::Available(point) => Cursor::Available(point - translation),
            Cursor::Levitating(point) => {
                Cursor::Levitating(point - translation)
            }
            Cursor::Unavailable => Cursor::Unavailable,
        }
    }
}

impl std::ops::Mul<Transformation> for Cursor {
    type Output = Self;

    fn mul(self, transformation: Transformation) -> Self {
        match self {
            Self::Available(position) => {
                Self::Available(position * transformation)
            }
            Self::Levitating(position) => {
                Self::Levitating(position * transformation)
            }
            Self::Unavailable => Self::Unavailable,
        }
    }
}
