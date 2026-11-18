//! 定义主题的颜色。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Define the colors of a theme.
//!
//! </details>
use crate::{Color, color};

use std::sync::LazyLock;

/// 一个调色板。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A color palette.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Palette {
    /// [`Palette`] 的背景 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The background [`Color`] of the [`Palette`].
    ///
    /// </details>
    pub background: Color,
    /// [`Palette`] 的文本 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text [`Color`] of the [`Palette`].
    ///
    /// </details>
    pub text: Color,
    /// [`Palette`] 的主 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The primary [`Color`] of the [`Palette`].
    ///
    /// </details>
    pub primary: Color,
    /// [`Palette`] 的成功 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The success [`Color`] of the [`Palette`].
    ///
    /// </details>
    pub success: Color,
    /// [`Palette`] 的警告 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The warning [`Color`] of the [`Palette`].
    ///
    /// </details>
    pub warning: Color,
    /// [`Palette`] 的危险 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The danger [`Color`] of the [`Palette`].
    ///
    /// </details>
    pub danger: Color,
}

impl Palette {
    /// [`Palette`] 内置的浅色变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in light variant of a [`Palette`].
    ///
    /// </details>
    pub const LIGHT: Self = Self {
        background: Color::WHITE,
        text: Color::BLACK,
        primary: color!(0x5865F2),
        success: color!(0x12664f),
        warning: color!(0xb77e33),
        danger: color!(0xc3423f),
    };

    /// [`Palette`] 内置的深色变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in dark variant of a [`Palette`].
    ///
    /// </details>
    pub const DARK: Self = Self {
        background: color!(0x2B2D31),
        text: Color::from_rgb(0.90, 0.90, 0.90),
        primary: color!(0x5865F2),
        success: color!(0x12664f),
        warning: color!(0xffc14e),
        danger: color!(0xc3423f),
    };

    /// [`Palette`] 内置的 [Dracula] 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in [Dracula] variant of a [`Palette`].
    ///
    /// </details>
    ///
    /// [Dracula]: https://draculatheme.com
    pub const DRACULA: Self = Self {
        background: color!(0x282A36), // BACKGROUND（背景）
        text: color!(0xf8f8f2),       // FOREGROUND（前景）
        primary: color!(0xbd93f9),    // PURPLE（紫色）
        success: color!(0x50fa7b),    // GREEN（绿色）
        warning: color!(0xf1fa8c),    // YELLOW（黄色）
        danger: color!(0xff5555),     // RED（红色）
    };

    /// [`Palette`] 内置的 [Nord] 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in [Nord] variant of a [`Palette`].
    ///
    /// </details>
    ///
    /// [Nord]: https://www.nordtheme.com/docs/colors-and-palettes
    pub const NORD: Self = Self {
        background: color!(0x2e3440), // nord0
        text: color!(0xeceff4),       // nord6
        primary: color!(0x8fbcbb),    // nord7
        success: color!(0xa3be8c),    // nord14
        warning: color!(0xebcb8b),    // nord13
        danger: color!(0xbf616a),     // nord11
    };

    /// [`Palette`] 内置的 [Solarized] Light 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in [Solarized] Light variant of a [`Palette`].
    ///
    /// </details>
    ///
    /// [Solarized]: https://ethanschoonover.com/solarized
    pub const SOLARIZED_LIGHT: Self = Self {
        background: color!(0xfdf6e3), // base3
        text: color!(0x657b83),       // base00
        primary: color!(0x2aa198),    // cyan（青色）
        success: color!(0x859900),    // green（绿色）
        warning: color!(0xb58900),    // yellow（黄色）
        danger: color!(0xdc322f),     // red（红色）
    };

    /// [`Palette`] 内置的 [Solarized] Dark 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in [Solarized] Dark variant of a [`Palette`].
    ///
    /// </details>
    ///
    /// [Solarized]: https://ethanschoonover.com/solarized
    pub const SOLARIZED_DARK: Self = Self {
        background: color!(0x002b36), // base03
        text: color!(0x839496),       // base0
        primary: color!(0x2aa198),    // cyan（青色）
        success: color!(0x859900),    // green（绿色）
        warning: color!(0xb58900),    // yellow（黄色）
        danger: color!(0xdc322f),     // red（红色）
    };

    /// [`Palette`] 内置的 [Gruvbox] Light 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in [Gruvbox] Light variant of a [`Palette`].
    ///
    /// </details>
    ///
    /// [Gruvbox]: https://github.com/morhetz/gruvbox
    pub const GRUVBOX_LIGHT: Self = Self {
        background: color!(0xfbf1c7), // light BG_0
        text: color!(0x282828),       // light FG0_29
        primary: color!(0x458588),    // light BLUE_4
        success: color!(0x98971a),    // light GREEN_2
        warning: color!(0xd79921),    // light YELLOW_3
        danger: color!(0xcc241d),     // light RED_1
    };

    /// [`Palette`] 内置的 [Gruvbox] Dark 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in [Gruvbox] Dark variant of a [`Palette`].
    ///
    /// </details>
    ///
    /// [Gruvbox]: https://github.com/morhetz/gruvbox
    pub const GRUVBOX_DARK: Self = Self {
        background: color!(0x282828), // dark BG_0
        text: color!(0xfbf1c7),       // dark FG0_29
        primary: color!(0x458588),    // dark BLUE_4
        success: color!(0x98971a),    // dark GREEN_2
        warning: color!(0xd79921),    // dark YELLOW_3
        danger: color!(0xcc241d),     // dark RED_1
    };

    /// [`Palette`] 内置的 [Catppuccin] Latte 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in [Catppuccin] Latte variant of a [`Palette`].
    ///
    /// </details>
    ///
    /// [Catppuccin]: https://github.com/catppuccin/catppuccin
    pub const CATPPUCCIN_LATTE: Self = Self {
        background: color!(0xeff1f5), // Base（基底色）
        text: color!(0x4c4f69),       // Text（文本）
        primary: color!(0x1e66f5),    // Blue（蓝色）
        success: color!(0x40a02b),    // Green（绿色）
        warning: color!(0xdf8e1d),    // Yellow（黄色）
        danger: color!(0xd20f39),     // Red（红色）
    };

    /// [`Palette`] 内置的 [Catppuccin] Frappé 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in [Catppuccin] Frappé variant of a [`Palette`].
    ///
    /// </details>
    ///
    /// [Catppuccin]: https://github.com/catppuccin/catppuccin
    pub const CATPPUCCIN_FRAPPE: Self = Self {
        background: color!(0x303446), // Base（基底色）
        text: color!(0xc6d0f5),       // Text（文本）
        primary: color!(0x8caaee),    // Blue（蓝色）
        success: color!(0xa6d189),    // Green（绿色）
        warning: color!(0xe5c890),    // Yellow（黄色）
        danger: color!(0xe78284),     // Red（红色）
    };

    /// [`Palette`] 内置的 [Catppuccin] Macchiato 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in [Catppuccin] Macchiato variant of a [`Palette`].
    ///
    /// </details>
    ///
    /// [Catppuccin]: https://github.com/catppuccin/catppuccin
    pub const CATPPUCCIN_MACCHIATO: Self = Self {
        background: color!(0x24273a), // Base（基底色）
        text: color!(0xcad3f5),       // Text（文本）
        primary: color!(0x8aadf4),    // Blue（蓝色）
        success: color!(0xa6da95),    // Green（绿色）
        warning: color!(0xeed49f),    // Yellow（黄色）
        danger: color!(0xed8796),     // Red（红色）
    };

    /// [`Palette`] 内置的 [Catppuccin] Mocha 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in [Catppuccin] Mocha variant of a [`Palette`].
    ///
    /// </details>
    ///
    /// [Catppuccin]: https://github.com/catppuccin/catppuccin
    pub const CATPPUCCIN_MOCHA: Self = Self {
        background: color!(0x1e1e2e), // Base（基底色）
        text: color!(0xcdd6f4),       // Text（文本）
        primary: color!(0x89b4fa),    // Blue（蓝色）
        success: color!(0xa6e3a1),    // Green（绿色）
        warning: color!(0xf9e2af),    // Yellow（黄色）
        danger: color!(0xf38ba8),     // Red（红色）
    };

    /// [`Palette`] 内置的 [Tokyo Night] 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in [Tokyo Night] variant of a [`Palette`].
    ///
    /// </details>
    ///
    /// [Tokyo Night]: https://github.com/enkia/tokyo-night-vscode-theme
    pub const TOKYO_NIGHT: Self = Self {
        background: color!(0x1a1b26), // Background (Night)（背景（Night））
        text: color!(0x9aa5ce),       // Text（文本）
        primary: color!(0x2ac3de),    // Blue（蓝色）
        success: color!(0x9ece6a),    // Green（绿色）
        warning: color!(0xe0af68),    // Yellow（黄色）
        danger: color!(0xf7768e),     // Red（红色）
    };

    /// [`Palette`] 内置的 [Tokyo Night] Storm 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in [Tokyo Night] Storm variant of a [`Palette`].
    ///
    /// </details>
    ///
    /// [Tokyo Night]: https://github.com/enkia/tokyo-night-vscode-theme
    pub const TOKYO_NIGHT_STORM: Self = Self {
        background: color!(0x24283b), // Background (Storm)（背景（Storm））
        text: color!(0x9aa5ce),       // Text（文本）
        primary: color!(0x2ac3de),    // Blue（蓝色）
        success: color!(0x9ece6a),    // Green（绿色）
        warning: color!(0xe0af68),    // Yellow（黄色）
        danger: color!(0xf7768e),     // Red（红色）
    };

    /// [`Palette`] 内置的 [Tokyo Night] Light 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in [Tokyo Night] Light variant of a [`Palette`].
    ///
    /// </details>
    ///
    /// [Tokyo Night]: https://github.com/enkia/tokyo-night-vscode-theme
    pub const TOKYO_NIGHT_LIGHT: Self = Self {
        background: color!(0xd5d6db), // Background（背景）
        text: color!(0x565a6e),       // Text（文本）
        primary: color!(0x166775),    // Blue（蓝色）
        success: color!(0x485e30),    // Green（绿色）
        warning: color!(0x8f5e15),    // Yellow（黄色）
        danger: color!(0x8c4351),     // Red（红色）
    };

    /// [`Palette`] 内置的 [Kanagawa] Wave 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in [Kanagawa] Wave variant of a [`Palette`].
    ///
    /// </details>
    ///
    /// [Kanagawa]: https://github.com/rebelot/kanagawa.nvim
    pub const KANAGAWA_WAVE: Self = Self {
        background: color!(0x1f1f28), // Sumi Ink 3
        text: color!(0xDCD7BA),       // Fuji White
        primary: color!(0x7FB4CA),    // Wave Blue
        success: color!(0x76946A),    // Autumn Green
        warning: color!(0xff9e3b),    // Ronin Yellow
        danger: color!(0xC34043),     // Autumn Red
    };

    /// [`Palette`] 内置的 [Kanagawa] Dragon 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in [Kanagawa] Dragon variant of a [`Palette`].
    ///
    /// </details>
    ///
    /// [Kanagawa]: https://github.com/rebelot/kanagawa.nvim
    pub const KANAGAWA_DRAGON: Self = Self {
        background: color!(0x181616), // Dragon Black 3
        text: color!(0xc5c9c5),       // Dragon White
        primary: color!(0x223249),    // Wave Blue 1
        success: color!(0x8a9a7b),    // Dragon Green 2
        warning: color!(0xff9e3b),    // Ronin Yellow
        danger: color!(0xc4746e),     // Dragon Red
    };

    /// [`Palette`] 内置的 [Kanagawa] Lotus 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in [Kanagawa] Lotus variant of a [`Palette`].
    ///
    /// </details>
    ///
    /// [Kanagawa]: https://github.com/rebelot/kanagawa.nvim
    pub const KANAGAWA_LOTUS: Self = Self {
        background: color!(0xf2ecbc), // Lotus White 3
        text: color!(0x545464),       // Lotus Ink 1
        primary: color!(0x4d699b),    // Lotus Blue
        success: color!(0x6f894e),    // Lotus Green
        warning: color!(0xe98a00),    // Lotus Orange 2
        danger: color!(0xc84053),     // Lotus Red
    };

    /// [`Palette`] 内置的 [Moonfly] 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in [Moonfly] variant of a [`Palette`].
    ///
    /// </details>
    ///
    /// [Moonfly]: https://github.com/bluz71/vim-moonfly-colors
    pub const MOONFLY: Self = Self {
        background: color!(0x080808), // Background（背景）
        text: color!(0xbdbdbd),       // Foreground（前景）
        primary: color!(0x80a0ff),    // Blue (normal)（蓝色（普通））
        success: color!(0x8cc85f),    // Green (normal)（绿色（普通））
        warning: color!(0xe3c78a),    // Yellow (normal)（黄色（普通））
        danger: color!(0xff5454),     // Red (normal)（红色（普通））
    };

    /// [`Palette`] 内置的 [Nightfly] 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in [Nightfly] variant of a [`Palette`].
    ///
    /// </details>
    ///
    /// [Nightfly]: https://github.com/bluz71/vim-nightfly-colors
    pub const NIGHTFLY: Self = Self {
        background: color!(0x011627), // Background（背景）
        text: color!(0xbdc1c6),       // Foreground（前景）
        primary: color!(0x82aaff),    // Blue (normal)（蓝色（普通））
        success: color!(0xa1cd5e),    // Green (normal)（绿色（普通））
        warning: color!(0xe3d18a),    // Yellow (normal)（黄色（普通））
        danger: color!(0xfc514e),     // Red (normal)（红色（普通））
    };

    /// [`Palette`] 内置的 [Oxocarbon] 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in [Oxocarbon] variant of a [`Palette`].
    ///
    /// </details>
    ///
    /// [Oxocarbon]: https://github.com/nyoom-engineering/oxocarbon.nvim
    pub const OXOCARBON: Self = Self {
        background: color!(0x232323),
        text: color!(0xd0d0d0),
        primary: color!(0x00b4ff),
        success: color!(0x00c15a),
        warning: color!(0xbe95ff), // Base 14
        danger: color!(0xf62d0f),
    };

    /// [`Palette`] 内置的 [Ferra] 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in [Ferra] variant of a [`Palette`].
    ///
    /// </details>
    ///
    /// [Ferra]: https://github.com/casperstorm/ferra
    pub const FERRA: Self = Self {
        background: color!(0x2b292d),
        text: color!(0xfecdb2),
        primary: color!(0xd1d1e0),
        success: color!(0xb1b695),
        warning: color!(0xf5d76e), // Honey
        danger: color!(0xe06b75),
    };
}

/// 根据 [`Palette`] 生成的一组扩展颜色。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An extended set of colors generated from a [`Palette`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Extended {
    /// 背景色组。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The set of background colors.
    ///
    /// </details>
    pub background: Background,
    /// 主色组。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The set of primary colors.
    ///
    /// </details>
    pub primary: Primary,
    /// 辅助色组。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The set of secondary colors.
    ///
    /// </details>
    pub secondary: Secondary,
    /// 成功色组。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The set of success colors.
    ///
    /// </details>
    pub success: Success,
    /// 警告色组。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The set of warning colors.
    ///
    /// </details>
    pub warning: Warning,
    /// 危险色组。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The set of danger colors.
    ///
    /// </details>
    pub danger: Danger,
    /// 调色板是否为深色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether the palette is dark or not.
    ///
    /// </details>
    pub is_dark: bool,
}

/// [`Extended`] 调色板的内置浅色变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in light variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_LIGHT: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::LIGHT));

/// [`Extended`] 调色板的内置深色变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in dark variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_DARK: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::DARK));

/// [`Extended`] 调色板的内置 Dracula 变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in Dracula variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_DRACULA: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::DRACULA));

/// [`Extended`] 调色板的内置 Nord 变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in Nord variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_NORD: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::NORD));

/// [`Extended`] 调色板的内置 Solarized Light 变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in Solarized Light variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_SOLARIZED_LIGHT: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::SOLARIZED_LIGHT));

/// [`Extended`] 调色板的内置 Solarized Dark 变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in Solarized Dark variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_SOLARIZED_DARK: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::SOLARIZED_DARK));

/// [`Extended`] 调色板的内置 Gruvbox Light 变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in Gruvbox Light variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_GRUVBOX_LIGHT: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::GRUVBOX_LIGHT));

/// [`Extended`] 调色板的内置 Gruvbox Dark 变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in Gruvbox Dark variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_GRUVBOX_DARK: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::GRUVBOX_DARK));

/// [`Extended`] 调色板的内置 Catppuccin Latte 变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in Catppuccin Latte variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_CATPPUCCIN_LATTE: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::CATPPUCCIN_LATTE));

/// [`Extended`] 调色板的内置 Catppuccin Frappé 变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in Catppuccin Frappé variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_CATPPUCCIN_FRAPPE: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::CATPPUCCIN_FRAPPE));

/// [`Extended`] 调色板的内置 Catppuccin Macchiato 变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in Catppuccin Macchiato variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_CATPPUCCIN_MACCHIATO: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::CATPPUCCIN_MACCHIATO));

/// [`Extended`] 调色板的内置 Catppuccin Mocha 变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in Catppuccin Mocha variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_CATPPUCCIN_MOCHA: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::CATPPUCCIN_MOCHA));

/// [`Extended`] 调色板的内置 Tokyo Night 变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in Tokyo Night variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_TOKYO_NIGHT: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::TOKYO_NIGHT));

/// [`Extended`] 调色板的内置 Tokyo Night Storm 变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in Tokyo Night Storm variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_TOKYO_NIGHT_STORM: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::TOKYO_NIGHT_STORM));

/// [`Extended`] 调色板的内置 Tokyo Night 变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in Tokyo Night variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_TOKYO_NIGHT_LIGHT: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::TOKYO_NIGHT_LIGHT));

/// [`Extended`] 调色板的内置 Kanagawa Wave 变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in Kanagawa Wave variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_KANAGAWA_WAVE: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::KANAGAWA_WAVE));

/// [`Extended`] 调色板的内置 Kanagawa Dragon 变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in Kanagawa Dragon variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_KANAGAWA_DRAGON: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::KANAGAWA_DRAGON));

/// [`Extended`] 调色板的内置 Kanagawa Lotus 变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in Kanagawa Lotus variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_KANAGAWA_LOTUS: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::KANAGAWA_LOTUS));

/// [`Extended`] 调色板的内置 Moonfly 变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in Moonfly variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_MOONFLY: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::MOONFLY));

/// [`Extended`] 调色板的内置 Nightfly 变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in Nightfly variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_NIGHTFLY: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::NIGHTFLY));

/// [`Extended`] 调色板的内置 Oxocarbon 变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in Oxocarbon variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_OXOCARBON: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::OXOCARBON));

/// [`Extended`] 调色板的内置 Ferra 变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The built-in Ferra variant of an [`Extended`] palette.
///
/// </details>
pub static EXTENDED_FERRA: LazyLock<Extended> =
    LazyLock::new(|| Extended::generate(Palette::FERRA));

impl Extended {
    /// 根据一个简单的 [`Palette`] 生成 [`Extended`] 调色板。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Generates an [`Extended`] palette from a simple [`Palette`].
    ///
    /// </details>
    pub fn generate(palette: Palette) -> Self {
        Self {
            background: Background::new(palette.background, palette.text),
            primary: Primary::generate(
                palette.primary,
                palette.background,
                palette.text,
            ),
            secondary: Secondary::generate(palette.background, palette.text),
            success: Success::generate(
                palette.success,
                palette.background,
                palette.text,
            ),
            warning: Warning::generate(
                palette.warning,
                palette.background,
                palette.text,
            ),
            danger: Danger::generate(
                palette.danger,
                palette.background,
                palette.text,
            ),
            is_dark: is_dark(palette.background),
        }
    }
}

/// 一对背景色和文本色。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A pair of background and text colors.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pair {
    /// 背景色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The background color.
    ///
    /// </details>
    pub color: Color,

    /// 文本色。
    ///
    /// 保证它在背景 [`color`] 之上清晰可读。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text color.
    ///
    /// It's guaranteed to be readable on top of the background [`color`].
    ///
    /// </details>
    ///
    /// [`color`]: Self::color
    pub text: Color,
}

impl Pair {
    /// 根据一个背景 [`Color`] 和一个文本 [`Color`] 创建一个新的 [`Pair`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Pair`] from a background [`Color`] and some text [`Color`].
    ///
    /// </details>
    pub fn new(color: Color, text: Color) -> Self {
        Self {
            color,
            text: readable(color, text),
        }
    }
}

/// 一组背景色。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A set of background colors.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Background {
    /// 基础背景色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The base background color.
    ///
    /// </details>
    pub base: Pair,
    /// 基础背景色最弱的版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The weakest version of the base background color.
    ///
    /// </details>
    pub weakest: Pair,
    /// 基础背景色更弱的版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A weaker version of the base background color.
    ///
    /// </details>
    pub weaker: Pair,
    /// 基础背景色较弱的版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A weak version of the base background color.
    ///
    /// </details>
    pub weak: Pair,
    /// 基础背景色的中性版本，介于弱与强之间。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A neutral version of the base background color, between weak and strong.
    ///
    /// </details>
    pub neutral: Pair,
    /// 基础背景色较强的版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A strong version of the base background color.
    ///
    /// </details>
    pub strong: Pair,
    /// 基础背景色更强的版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A stronger version of the base background color.
    ///
    /// </details>
    pub stronger: Pair,
    /// 基础背景色最强的版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The strongest version of the base background color.
    ///
    /// </details>
    pub strongest: Pair,
}

impl Background {
    /// 根据基础色和文本色生成一组 [`Background`] 颜色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Generates a set of [`Background`] colors from the base and text colors.
    ///
    /// </details>
    pub fn new(base: Color, text: Color) -> Self {
        let weakest = deviate(base, 0.03);
        let weaker = deviate(base, 0.07);
        let weak = deviate(base, 0.1);
        let neutral = deviate(base, 0.125);
        let strong = deviate(base, 0.15);
        let stronger = deviate(base, 0.175);
        let strongest = deviate(base, 0.20);

        Self {
            base: Pair::new(base, text),
            weakest: Pair::new(weakest, text),
            weaker: Pair::new(weaker, text),
            weak: Pair::new(weak, text),
            neutral: Pair::new(neutral, text),
            strong: Pair::new(strong, text),
            stronger: Pair::new(stronger, text),
            strongest: Pair::new(strongest, text),
        }
    }
}

/// 一组主色。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A set of primary colors.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Primary {
    /// 基础主色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The base primary color.
    ///
    /// </details>
    pub base: Pair,
    /// 基础主色的较弱版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A weaker version of the base primary color.
    ///
    /// </details>
    pub weak: Pair,
    /// 基础主色的较强版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A stronger version of the base primary color.
    ///
    /// </details>
    pub strong: Pair,
}

impl Primary {
    /// 根据基础色、背景色和文本颜色生成一组 [`Primary`] 颜色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Generates a set of [`Primary`] colors from the base, background, and text colors.
    ///
    /// </details>
    pub fn generate(base: Color, background: Color, text: Color) -> Self {
        let weak = mix(base, background, 0.4);
        let strong = deviate(base, 0.1);

        Self {
            base: Pair::new(base, text),
            weak: Pair::new(weak, text),
            strong: Pair::new(strong, text),
        }
    }
}

/// 一组次要色。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A set of secondary colors.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Secondary {
    /// 基础次要色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The base secondary color.
    ///
    /// </details>
    pub base: Pair,
    /// 基础次要色的较弱版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A weaker version of the base secondary color.
    ///
    /// </details>
    pub weak: Pair,
    /// 基础次要色的较强版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A stronger version of the base secondary color.
    ///
    /// </details>
    pub strong: Pair,
}

impl Secondary {
    /// 根据基础色和文本颜色生成一组 [`Secondary`] 颜色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Generates a set of [`Secondary`] colors from the base and text colors.
    ///
    /// </details>
    pub fn generate(base: Color, text: Color) -> Self {
        let factor = if is_dark(base) { 0.2 } else { 0.4 };

        let weak = mix(deviate(base, 0.1), text, factor);
        let strong = mix(deviate(base, 0.3), text, factor);
        let base = mix(deviate(base, 0.2), text, factor);

        Self {
            base: Pair::new(base, text),
            weak: Pair::new(weak, text),
            strong: Pair::new(strong, text),
        }
    }
}

/// 一组成功色。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A set of success colors.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Success {
    /// 基础成功色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The base success color.
    ///
    /// </details>
    pub base: Pair,
    /// 基础成功色的较弱版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A weaker version of the base success color.
    ///
    /// </details>
    pub weak: Pair,
    /// 基础成功色的较强版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A stronger version of the base success color.
    ///
    /// </details>
    pub strong: Pair,
}

impl Success {
    /// 根据基础色、背景色和文本颜色生成一组 [`Success`] 颜色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Generates a set of [`Success`] colors from the base, background, and text colors.
    ///
    /// </details>
    pub fn generate(base: Color, background: Color, text: Color) -> Self {
        let weak = mix(base, background, 0.4);
        let strong = deviate(base, 0.1);

        Self {
            base: Pair::new(base, text),
            weak: Pair::new(weak, text),
            strong: Pair::new(strong, text),
        }
    }
}

/// 一组警告色。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A set of warning colors.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Warning {
    /// 基础警告色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The base warning color.
    ///
    /// </details>
    pub base: Pair,
    /// 基础警告色的较弱版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A weaker version of the base warning color.
    ///
    /// </details>
    pub weak: Pair,
    /// 基础警告色的较强版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A stronger version of the base warning color.
    ///
    /// </details>
    pub strong: Pair,
}

impl Warning {
    /// 根据基础色、背景色和文本颜色生成一组 [`Warning`] 颜色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Generates a set of [`Warning`] colors from the base, background, and text colors.
    ///
    /// </details>
    pub fn generate(base: Color, background: Color, text: Color) -> Self {
        let weak = mix(base, background, 0.4);
        let strong = deviate(base, 0.1);

        Self {
            base: Pair::new(base, text),
            weak: Pair::new(weak, text),
            strong: Pair::new(strong, text),
        }
    }
}

/// 一组危险色。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A set of danger colors.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Danger {
    /// 基础危险色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The base danger color.
    ///
    /// </details>
    pub base: Pair,
    /// 基础危险色的较弱版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A weaker version of the base danger color.
    ///
    /// </details>
    pub weak: Pair,
    /// 基础危险色的较强版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A stronger version of the base danger color.
    ///
    /// </details>
    pub strong: Pair,
}

impl Danger {
    /// 根据基础色、背景色和文本颜色生成一组 [`Danger`] 颜色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Generates a set of [`Danger`] colors from the base, background, and text colors.
    ///
    /// </details>
    pub fn generate(base: Color, background: Color, text: Color) -> Self {
        let weak = mix(base, background, 0.4);
        let strong = deviate(base, 0.1);

        Self {
            base: Pair::new(base, text),
            weak: Pair::new(weak, text),
            strong: Pair::new(strong, text),
        }
    }
}

struct Oklch {
    l: f32,
    c: f32,
    h: f32,
    a: f32,
}

/// 按给定的系数加深一个 [`Color`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Darkens a [`Color`] by the given factor.
///
/// </details>
pub fn darken(color: Color, amount: f32) -> Color {
    let mut oklch = to_oklch(color);

    // We try to bump the chroma a bit for more colorful palettes
    // 我们尝试稍微提高色度，让调色板更鲜艳
    if oklch.c > 0.0 && oklch.c < (1.0 - oklch.l) / 2.0 {
        // Formula empirically and cluelessly derived
        // 公式是凭经验瞎试出来的
        oklch.c *= 1.0 + (0.2 / oklch.c).min(100.0) * amount;
    }

    oklch.l = if oklch.l - amount < 0.0 {
        0.0
    } else {
        oklch.l - amount
    };

    from_oklch(oklch)
}

/// 按给定的系数调亮一个 [`Color`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Lightens a [`Color`] by the given factor.
///
/// </details>
pub fn lighten(color: Color, amount: f32) -> Color {
    let mut oklch = to_oklch(color);

    // We try to bump the chroma a bit for more colorful palettes
    // Formula empirically and cluelessly derived
    // 我们尝试稍微提高色度，让调色板更鲜艳
    // 公式是凭经验瞎试出来的
    oklch.c *= 1.0 + 2.0 * amount / oklch.l.max(0.05);

    oklch.l = if oklch.l + amount > 1.0 {
        1.0
    } else {
        oklch.l + amount
    };

    from_oklch(oklch)
}

/// 按给定的系数偏移一个 [`Color`]。如果该 [`Color`] 是深色
/// 则提亮，否则加深。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Deviates a [`Color`] by the given factor. Lightens if the [`Color`] is
/// dark, darkens otherwise.
///
/// </details>
pub fn deviate(color: Color, amount: f32) -> Color {
    if is_dark(color) {
        lighten(color, amount)
    } else {
        darken(color, amount)
    }
}

/// 按给定的系数混合两种颜色。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Mixes two colors by the given factor.
///
/// </details>
pub fn mix(a: Color, b: Color, factor: f32) -> Color {
    let b_amount = factor.clamp(0.0, 1.0);
    let a_amount = 1.0 - b_amount;

    let a_linear = a.into_linear().map(|c| c * a_amount);
    let b_linear = b.into_linear().map(|c| c * b_amount);

    Color::from_linear_rgba(
        a_linear[0] + b_linear[0],
        a_linear[1] + b_linear[1],
        a_linear[2] + b_linear[2],
        a_linear[3] + b_linear[3],
    )
}

/// 根据给定的文本色计算出一个 [`Color`]，使其在
/// 给定的背景色之上清晰可读。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Computes a [`Color`] from the given text color that is
/// readable on top of the given background color.
///
/// </details>
pub fn readable(background: Color, text: Color) -> Color {
    if text.is_readable_on(background) {
        return text;
    }

    let improve = if is_dark(background) { lighten } else { darken };

    // TODO: Compute factor from relative contrast value
    // 待办：根据相对对比度的值计算系数
    let candidate = improve(text, 0.1);

    if candidate.is_readable_on(background) {
        return candidate;
    }

    let candidate = improve(text, 0.2);

    if candidate.is_readable_on(background) {
        return candidate;
    }

    let white_contrast = background.relative_contrast(Color::WHITE);
    let black_contrast = background.relative_contrast(Color::BLACK);

    if white_contrast >= black_contrast {
        mix(Color::WHITE, background, 0.05)
    } else {
        mix(Color::BLACK, background, 0.05)
    }
}

/// 如果 [`Color`] 是深色，返回 true。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns true if the [`Color`] is dark.
///
/// </details>
pub fn is_dark(color: Color) -> bool {
    to_oklch(color).l < 0.6
}

// https://en.wikipedia.org/wiki/Oklab_color_space#Conversions_between_color_spaces
fn to_oklch(color: Color) -> Oklch {
    let [r, g, b, alpha] = color.into_linear();

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

// https://en.wikipedia.org/wiki/Oklab_color_space#Conversions_between_color_spaces
fn from_oklch(oklch: Oklch) -> Color {
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
