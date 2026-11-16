/// 一定数量的逻辑像素。
///
/// 通常用来表示空间的大小，或者某个东西的尺寸。
///
/// 这个类型通常以泛型的方式作为参数
/// （例如 `impl Into<Pixels>`），而且由于 `Pixels` 同时为
/// `f32` 和 `u16` 实现了 `From`，你可以根据需要
/// 传入整数或浮点数字面量。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An amount of logical pixels.
///
/// Normally used to represent an amount of space, or the size of something.
///
/// This type is normally asked as an argument in a generic way
/// (e.g. `impl Into<Pixels>`) and, since `Pixels` implements `From` both for
/// `f32` and `u16`, you should be able to provide both integers and float
/// literals as needed.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Pixels(pub f32);

impl Pixels {
    /// 零像素。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Zero pixels.
    ///
    /// </details>
    pub const ZERO: Self = Self(0.0);
}

impl From<f32> for Pixels {
    fn from(amount: f32) -> Self {
        Self(amount)
    }
}

impl From<u32> for Pixels {
    fn from(amount: u32) -> Self {
        Self(amount as f32)
    }
}

impl From<Pixels> for f32 {
    fn from(pixels: Pixels) -> Self {
        pixels.0
    }
}

impl std::ops::Add for Pixels {
    type Output = Pixels;

    fn add(self, rhs: Self) -> Self {
        Pixels(self.0 + rhs.0)
    }
}

impl std::ops::Add<f32> for Pixels {
    type Output = Pixels;

    fn add(self, rhs: f32) -> Self {
        Pixels(self.0 + rhs)
    }
}

impl std::ops::Mul for Pixels {
    type Output = Pixels;

    fn mul(self, rhs: Self) -> Self {
        Pixels(self.0 * rhs.0)
    }
}

impl std::ops::Mul<f32> for Pixels {
    type Output = Pixels;

    fn mul(self, rhs: f32) -> Self {
        Pixels(self.0 * rhs)
    }
}

impl std::ops::Div for Pixels {
    type Output = Pixels;

    fn div(self, rhs: Self) -> Self {
        Pixels(self.0 / rhs.0)
    }
}

impl std::ops::Div<f32> for Pixels {
    type Output = Pixels;

    fn div(self, rhs: f32) -> Self {
        Pixels(self.0 / rhs)
    }
}

impl std::ops::Div<u32> for Pixels {
    type Output = Pixels;

    fn div(self, rhs: u32) -> Self {
        Pixels(self.0 / rhs as f32)
    }
}
