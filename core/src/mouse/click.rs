//! 跟踪鼠标点击。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Track mouse clicks.
//!
//! </details>
use crate::mouse::Button;
use crate::time::Instant;
use crate::{Point, Transformation};

use std::ops::Mul;

/// 鼠标点击。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A mouse click.
///
/// </details>
#[derive(Debug, Clone, Copy)]
pub struct Click {
    kind: Kind,
    button: Button,
    position: Point,
    time: Instant,
}

/// 鼠标点击的种类。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The kind of mouse click.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// 单击
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A single click
    ///
    /// </details>
    Single,

    /// 双击
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A double click
    ///
    /// </details>
    Double,

    /// 三击
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A triple click
    ///
    /// </details>
    Triple,
}

impl Kind {
    fn next(self) -> Kind {
        match self {
            Kind::Single => Kind::Double,
            Kind::Double => Kind::Triple,
            Kind::Triple => Kind::Double,
        }
    }
}

impl Click {
    /// 使用给定的位置和上一次的
    /// [`Click`] 创建一个新的 [`Click`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Click`] with the given position and previous last
    /// [`Click`].
    ///
    /// </details>
    pub fn new(position: Point, button: Button, previous: Option<Click>) -> Click {
        let time = Instant::now();

        let kind = if let Some(previous) = previous {
            if previous.is_consecutive(position, time) && button == previous.button {
                previous.kind.next()
            } else {
                Kind::Single
            }
        } else {
            Kind::Single
        };

        Click {
            kind,
            button,
            position,
            time,
        }
    }

    /// 返回 [`Click`] 的 [`Kind`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Kind`] of [`Click`].
    ///
    /// </details>
    pub fn kind(&self) -> Kind {
        self.kind
    }

    /// 返回 [`Click`] 的位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the position of the [`Click`].
    ///
    /// </details>
    pub fn position(&self) -> Point {
        self.position
    }

    fn is_consecutive(&self, new_position: Point, time: Instant) -> bool {
        let duration = if time > self.time {
            Some(time - self.time)
        } else {
            None
        };

        self.position.distance(new_position) < 6.0
            && duration
                .map(|duration| duration.as_millis() <= 300)
                .unwrap_or(false)
    }
}

impl Mul<Transformation> for Click {
    type Output = Click;

    fn mul(self, transformation: Transformation) -> Click {
        Click {
            kind: self.kind,
            button: self.button,
            position: self.position * transformation,
            time: self.time,
        }
    }
}
