//! 在不同的色彩空间中管理颜色。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Manage colors in different color spaces.
//!
//! </details>
use crate::animation::Interpolable;

/// `sRGB` 色彩空间中的颜色。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A color in the `sRGB` color space.
///
/// </details>
///
/// # 字符串表示（String Representation）
///
/// 颜色可以用以下任一有效格式表示：`#rrggbb`、`#rrggbbaa`、`#rgb` 和 `#rgba`。
/// 其中 `rgba` 代表十六进制数字。大写和小写字母都支持。
///
/// 如果没有指定 `a`（透明度），默认使用 `1.0`（完全不透明）。
///
/// 如果你有一个静态的颜色字符串，应优先使用 [`color!`] 宏，
/// 因为它直接利用了十六进制字面量记法和算术运算。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A color can be represented in either of the following valid formats: `#rrggbb`, `#rrggbbaa`, `#rgb`, and `#rgba`.
/// Where `rgba` represent hexadecimal digits. Both uppercase and lowercase letters are supported.
///
/// If `a` (transparency) is not specified, `1.0` (completely opaque) would be used by default.
///
/// If you have a static color string, using the [`color!`] macro should be preferred
/// since it leverages hexadecimal literal notation and arithmetic directly.
///
/// </details>
///
/// [`color!`]: crate::color!
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[must_use]
pub struct Color {
    /// 红色分量，0.0 - 1.0
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Red component, 0.0 - 1.0
    ///
    /// </details>
    pub r: f32,
    /// 绿色分量，0.0 - 1.0
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Green component, 0.0 - 1.0
    ///
    /// </details>
    pub g: f32,
    /// 蓝色分量，0.0 - 1.0
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Blue component, 0.0 - 1.0
    ///
    /// </details>
    pub b: f32,
    /// 透明度，0.0 - 1.0
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Transparency, 0.0 - 1.0
    ///
    /// </details>
    pub a: f32,
}

impl Color {
    /// 黑色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The black color.
    ///
    /// </details>
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    /// 白色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The white color.
    ///
    /// </details>
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    /// 完全透明的颜色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A color with no opacity.
    ///
    /// </details>
    pub const TRANSPARENT: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    /// 创建一个新的 [`Color`]。
    ///
    /// 在调试模式下，如果各个值不在正确的范围
    /// 0.0 - 1.0 内，它会 panic
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Color`].
    ///
    /// In debug mode, it will panic if the values are not in the correct
    /// range: 0.0 - 1.0
    ///
    /// </details>
    const fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        debug_assert!(
            r >= 0.0 && r <= 1.0,
            "Red component must be in [0, 1] range."
        );
        debug_assert!(
            g >= 0.0 && g <= 1.0,
            "Green component must be in [0, 1] range."
        );
        debug_assert!(
            b >= 0.0 && b <= 1.0,
            "Blue component must be in [0, 1] range."
        );

        Self { r, g, b, a }
    }

    /// 根据 RGB 分量创建一个 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Color`] from its RGB components.
    ///
    /// </details>
    pub const fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::from_rgba(r, g, b, 1.0f32)
    }

    /// 根据 RGBA 分量创建一个 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Color`] from its RGBA components.
    ///
    /// </details>
    pub const fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::new(r, g, b, a)
    }

    /// 根据 RGB8 分量创建一个 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Color`] from its RGB8 components.
    ///
    /// </details>
    pub const fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba8(r, g, b, 1.0)
    }

    /// 根据 RGB8 分量和一个 alpha 值创建一个 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Color`] from its RGB8 components and an alpha value.
    ///
    /// </details>
    pub const fn from_rgba8(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a)
    }

    /// 根据打包在 `u32` 低位中的 RGB8 分量创建一个 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Color`] from its RGB8 components packed in the lower bits of a `u32`.
    ///
    /// </details>
    pub const fn from_packed_rgb8(rgb: u32) -> Self {
        Self::from_packed_rgba8(rgb, 1.0)
    }

    /// 根据打包在 `u32` 低位中的 RGB8 分量和一个 alpha 值
    /// 创建一个 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Color`] from its RGB8 components packed in the lower bits of a `u32`
    /// and an alpha value.
    ///
    /// </details>
    pub const fn from_packed_rgba8(rgb: u32, a: f32) -> Self {
        let r = (rgb & 0xff0000) >> 16;
        let g = (rgb & 0xff00) >> 8;
        let b = rgb & 0xff;

        Self::from_rgba8(r as u8, g as u8, b as u8, a)
    }

    /// 根据线性 RGBA 分量创建一个 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Color`] from its linear RGBA components.
    ///
    /// </details>
    pub fn from_linear_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        // As described in:
        // https://en.wikipedia.org/wiki/SRGB
        // 出处见上面的链接
        fn gamma_component(u: f32) -> f32 {
            if u < 0.0031308 {
                12.92 * u
            } else {
                1.055 * u.powf(1.0 / 2.4) - 0.055
            }
        }

        Self::new(
            gamma_component(r),
            gamma_component(g),
            gamma_component(b),
            a,
        )
    }

    /// 根据 [`Oklch`] 表示创建最接近的 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates the most approximate [`Color`] from its [`Oklch`] representation.
    ///
    /// </details>
    pub fn from_oklch(oklch: Oklch) -> Color {
        // https://en.wikipedia.org/wiki/Oklab_color_space#Conversions_between_color_spaces
        let Oklch { l, c, h, a: alpha } = oklch;

        let a = c * h.cos();
        let b = c * h.sin();

        // Oklab → LMS (nonlinear)
        // Oklab → LMS（非线性）
        let l_ = l + 0.39633778 * a + 0.21580376 * b;
        let m_ = l - 0.105561346 * a - 0.06385417 * b;
        let s_ = l - 0.08948418 * a - 1.2914855 * b;

        // Cubing back
        // 还原立方
        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;

        let r = 4.0767417 * l - 3.3077116 * m + 0.23096994 * s;
        let g = -1.268438 * l + 2.6097574 * m - 0.34131938 * s;
        let b = -0.0041960863 * l - 0.7034186 * m + 1.7076147 * s;

        Color::from_linear_rgba(
            r.clamp(0.0, 1.0),
            g.clamp(0.0, 1.0),
            b.clamp(0.0, 1.0),
            alpha,
        )
    }

    /// 就地反转这个 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Inverts the [`Color`] in-place.
    ///
    /// </details>
    pub const fn invert(&mut self) {
        self.r = 1.0f32 - self.r;
        self.g = 1.0f32 - self.g;
        self.b = 1.0f32 - self.b;
    }

    /// 返回反转后的 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the inverted [`Color`].
    ///
    /// </details>
    pub const fn inverse(self) -> Self {
        Self::new(1.0f32 - self.r, 1.0f32 - self.g, 1.0f32 - self.b, self.a)
    }

    /// 按给定的系数缩放 [`Color`] 的 alpha 通道。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Scales the alpha channel of the [`Color`] by the given factor.
    ///
    /// </details>
    pub const fn scale_alpha(self, factor: f32) -> Self {
        Self {
            a: self.a * factor,
            ..self
        }
    }

    /// 按给定的系数把当前 [`Color`] 与另一种颜色混合。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Mixes the current [`Color`] with another one by the given factor.
    ///
    /// </details>
    pub fn mix(self, b: Color, factor: f32) -> Color {
        let b_amount = factor.clamp(0.0, 1.0);
        let a_amount = 1.0 - b_amount;

        let a_linear = self.into_linear().map(|c| c * a_amount);
        let b_linear = b.into_linear().map(|c| c * b_amount);

        Color::from_linear_rgba(
            a_linear[0] + b_linear[0],
            a_linear[1] + b_linear[1],
            a_linear[2] + b_linear[2],
            a_linear[3] + b_linear[3],
        )
    }

    /// 返回 [`Color`] 的相对亮度。
    /// <https://www.w3.org/TR/WCAG21/#dfn-relative-luminance>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the relative luminance of the [`Color`].
    /// <https://www.w3.org/TR/WCAG21/#dfn-relative-luminance>
    ///
    /// </details>
    #[must_use]
    pub fn relative_luminance(self) -> f32 {
        let linear = self.into_linear();
        0.2126 * linear[0] + 0.7152 * linear[1] + 0.0722 * linear[2]
    }

    /// 返回这个 [`Color`] 相对于另一种颜色的[相对对比度][relative contrast ratio]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [relative contrast ratio] of the [`Color`] against another one.
    ///
    /// </details>
    ///
    /// [relative contrast ratio]: https://www.w3.org/TR/WCAG21/#dfn-contrast-ratio
    #[must_use]
    pub fn relative_contrast(self, b: Self) -> f32 {
        let lum_a = self.relative_luminance();
        let lum_b = b.relative_luminance();

        (lum_a.max(lum_b) + 0.05) / (lum_a.min(lum_b) + 0.05)
    }

    /// 如果当前 [`Color`] 放在给定的背景 [`Color`] 之上
    /// 是可读的，返回 true。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns true if the current [`Color`] is readable on top
    /// of the given background [`Color`].
    ///
    /// </details>
    #[must_use]
    pub fn is_readable_on(self, background: Self) -> bool {
        background.relative_contrast(self) >= 6.0
    }

    /// 把 [`Color`] 转换为等价的 RGBA8 值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Converts the [`Color`] into its RGBA8 equivalent.
    ///
    /// </details>
    #[must_use]
    pub const fn into_rgba8(self) -> [u8; 4] {
        [
            (self.r * 255.0).round() as u8,
            (self.g * 255.0).round() as u8,
            (self.b * 255.0).round() as u8,
            (self.a * 255.0).round() as u8,
        ]
    }

    /// 把 [`Color`] 转换为线性值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Converts the [`Color`] into its linear values.
    ///
    /// </details>
    #[must_use]
    pub fn into_linear(self) -> [f32; 4] {
        // As described in:
        // https://en.wikipedia.org/wiki/SRGB#The_reverse_transformation
        // 出处见上面的链接
        fn linear_component(u: f32) -> f32 {
            if u < 0.04045 {
                u / 12.92
            } else {
                ((u + 0.055) / 1.055).powf(2.4)
            }
        }

        [
            linear_component(self.r),
            linear_component(self.g),
            linear_component(self.b),
            self.a,
        ]
    }

    /// 把 [`Color`] 转换为 [`Oklch`] 表示。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Converts the [`Color`] into its [`Oklch`] representation.
    ///
    /// </details>
    pub fn into_oklch(self) -> Oklch {
        // https://en.wikipedia.org/wiki/Oklab_color_space#Conversions_between_color_spaces
        let [r, g, b, alpha] = self.into_linear();

        // linear RGB → LMS
        // 线性 RGB → LMS
        let l = 0.41222146 * r + 0.53633255 * g + 0.051445995 * b;
        let m = 0.2119035 * r + 0.6806995 * g + 0.10739696 * b;
        let s = 0.08830246 * r + 0.28171885 * g + 0.6299787 * b;

        // Nonlinear transform (cube root)
        // 非线性变换（立方根）
        let l_ = l.cbrt();
        let m_ = m.cbrt();
        let s_ = s.cbrt();

        // LMS → Oklab
        let l = 0.21045426 * l_ + 0.7936178 * m_ - 0.004072047 * s_;
        let a = 1.9779985 * l_ - 2.4285922 * m_ + 0.4505937 * s_;
        let b = 0.025904037 * l_ + 0.78277177 * m_ - 0.80867577 * s_;

        // Oklab → Oklch
        let c = (a * a + b * b).sqrt();
        let h = b.atan2(a); // radians（弧度）

        Oklch { l, c, h, a: alpha }
    }
}

impl From<[f32; 3]> for Color {
    fn from([r, g, b]: [f32; 3]) -> Self {
        Color::new(r, g, b, 1.0)
    }
}

impl From<[f32; 4]> for Color {
    fn from([r, g, b, a]: [f32; 4]) -> Self {
        Color::new(r, g, b, a)
    }
}

impl From<Oklch> for Color {
    fn from(oklch: Oklch) -> Self {
        Self::from_oklch(oklch)
    }
}

impl From<Color> for Oklch {
    fn from(color: Color) -> Self {
        color.into_oklch()
    }
}

/// 从 RGB 十六进制字符串解析颜色时可能返回的错误。
///
/// 字符串的规范见 [`Color`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An error which can be returned when parsing color from an RGB hexadecimal string.
///
/// See [`Color`] for specifications for the string.
///
/// </details>
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// 字符串无法解析为有效的整数。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The string could not be parsed to valid integers.
    ///
    /// </details>
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    /// 字符串的长度无效。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The string is of invalid length.
    ///
    /// </details>
    #[error("expected hex string of length 3, 4, 6 or 8 excluding optional prefix '#', found {0}")]
    InvalidLength(usize),
}

impl std::str::FromStr for Color {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.strip_prefix('#').unwrap_or(s);

        let parse_channel = |from: usize, to: usize| -> Result<f32, std::num::ParseIntError> {
            let num = usize::from_str_radix(&hex[from..=to], 16)? as f32 / 255.0;

            // If we only got half a byte (one letter), expand it into a full byte (two letters)
            // 如果只得到半个字节（一个字母），就把它扩展为一个完整的字节（两个字母）
            Ok(if from == to { num + num * 16.0 } else { num })
        };

        let val = match hex.len() {
            3 => Color::from_rgb(
                parse_channel(0, 0)?,
                parse_channel(1, 1)?,
                parse_channel(2, 2)?,
            ),
            4 => Color::from_rgba(
                parse_channel(0, 0)?,
                parse_channel(1, 1)?,
                parse_channel(2, 2)?,
                parse_channel(3, 3)?,
            ),
            6 => Color::from_rgb(
                parse_channel(0, 1)?,
                parse_channel(2, 3)?,
                parse_channel(4, 5)?,
            ),
            8 => Color::from_rgba(
                parse_channel(0, 1)?,
                parse_channel(2, 3)?,
                parse_channel(4, 5)?,
                parse_channel(6, 7)?,
            ),
            _ => return Err(ParseError::InvalidLength(hex.len())),
        };

        Ok(val)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [r, g, b, a] = self.into_rgba8();

        if self.a == 1.0 {
            return write!(f, "#{r:02x}{g:02x}{b:02x}");
        }

        write!(f, "#{r:02x}{g:02x}{b:02x}{a:02x}")
    }
}

impl Interpolable for Color {
    /// 对颜色进行插值。等价于 [`Color::mix`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Interpolates the color. Equivalent to [`Color::mix`].
    ///
    /// </details>
    fn interpolated(&self, other: Self, ratio: f32) -> Self {
        self.mix(other, ratio)
    }
}

/// [Oklab 色彩空间](https://en.wikipedia.org/wiki/Oklab_color_space)中的颜色，
/// 以 Oklch 表示。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A color in the [Oklab color space](https://en.wikipedia.org/wiki/Oklab_color_space),
/// represented as Oklch.
///
/// </details>
pub struct Oklch {
    /// 感知亮度：0 是纯黑，1 是纯白。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Perceptual lightness: 0 is pure black, 1 is pure white.
    ///
    /// </details>
    pub l: f32,
    /// 色度强度：0 表示无彩色，+0.5 通常是上限。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Chromatic intensity: 0 is achromatic, +0.5 is usually the upper limit.
    ///
    /// </details>
    pub c: f32,
    /// 色相角，以弧度为单位。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Hue angle, in radians.
    ///
    /// </details>
    pub h: f32,
    /// alpha 通道。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Alpha channel.
    ///
    /// </details>
    pub a: f32,
}

/// 用更简短、更清晰的语法创建一个 [`Color`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a [`Color`] with shorter and cleaner syntax.
///
/// </details>
///
/// # 示例（Examples）
///
/// ```
/// # use iced_core::{Color, color};
/// assert_eq!(color!(0, 0, 0), Color::BLACK);
/// assert_eq!(color!(0, 0, 0, 0.0), Color::TRANSPARENT);
/// assert_eq!(color!(0xffffff), Color::from_rgb(1.0, 1.0, 1.0));
/// assert_eq!(color!(0xffffff, 0.), Color::from_rgba(1.0, 1.0, 1.0, 0.0));
/// assert_eq!(color!(0x0000ff), Color::from_rgba(0.0, 0.0, 1.0, 1.0));
/// ```
#[macro_export]
macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => {
        $crate::Color::from_rgb8($r, $g, $b)
    };
    ($r:expr, $g:expr, $b:expr, $a:expr) => {{ $crate::Color::from_rgba8($r, $g, $b, $a) }};
    ($hex:literal) => {{ $crate::color!($hex, 1.0) }};
    ($hex:literal, $a:expr) => {{
        let mut hex = $hex as u32;

        // Shorthand notation: 0x123
        // 简写记法：0x123
        if stringify!($hex).len() == 5 {
            let r = hex & 0xF00;
            let g = hex & 0xF0;
            let b = hex & 0xF;

            hex = (r << 12) | (r << 8) | (g << 8) | (g << 4) | (b << 4) | b;
        }

        debug_assert!(hex <= 0xffffff, "color! value must not exceed 0xffffff");

        $crate::Color::from_packed_rgba8(hex, $a)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let tests = [
            ("#ff0000", [255, 0, 0, 255], "#ff0000"),
            ("00ff0080", [0, 255, 0, 128], "#00ff0080"),
            ("#F80", [255, 136, 0, 255], "#ff8800"),
            ("#00f1", [0, 0, 255, 17], "#0000ff11"),
            ("#00ff", [0, 0, 255, 255], "#0000ff"),
        ];

        for (arg, expected_rgba8, expected_str) in tests {
            let color = arg.parse::<Color>().expect("color must parse");

            assert_eq!(color.into_rgba8(), expected_rgba8);
            assert_eq!(color.to_string(), expected_str);
        }

        assert!("invalid".parse::<Color>().is_err());
    }

    const SHORTHAND: Color = color!(0x123);

    #[test]
    fn shorthand_notation() {
        assert_eq!(SHORTHAND, Color::from_rgb8(0x11, 0x22, 0x33));
    }
}
