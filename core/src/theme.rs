//! 使用内置的主题和样式。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Use the built-in theme and styles.
//!
//! </details>
pub mod palette;

pub use palette::Palette;

use crate::Color;

use std::borrow::Cow;
use std::fmt;
use std::sync::Arc;

/// 内置主题。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A built-in theme.
///
/// </details>
#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    /// 内置的浅色变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in light variant.
    ///
    /// </details>
    Light,
    /// 内置的深色变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in dark variant.
    ///
    /// </details>
    Dark,
    /// 内置的 Dracula 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in Dracula variant.
    ///
    /// </details>
    Dracula,
    /// 内置的 Nord 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in Nord variant.
    ///
    /// </details>
    Nord,
    /// 内置的 Solarized Light 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in Solarized Light variant.
    ///
    /// </details>
    SolarizedLight,
    /// 内置的 Solarized Dark 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in Solarized Dark variant.
    ///
    /// </details>
    SolarizedDark,
    /// 内置的 Gruvbox Light 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in Gruvbox Light variant.
    ///
    /// </details>
    GruvboxLight,
    /// 内置的 Gruvbox Dark 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in Gruvbox Dark variant.
    ///
    /// </details>
    GruvboxDark,
    /// 内置的 Catppuccin Latte 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in Catppuccin Latte variant.
    ///
    /// </details>
    CatppuccinLatte,
    /// 内置的 Catppuccin Frappé 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in Catppuccin Frappé variant.
    ///
    /// </details>
    CatppuccinFrappe,
    /// 内置的 Catppuccin Macchiato 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in Catppuccin Macchiato variant.
    ///
    /// </details>
    CatppuccinMacchiato,
    /// 内置的 Catppuccin Mocha 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in Catppuccin Mocha variant.
    ///
    /// </details>
    CatppuccinMocha,
    /// 内置的 Tokyo Night 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in Tokyo Night variant.
    ///
    /// </details>
    TokyoNight,
    /// 内置的 Tokyo Night Storm 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in Tokyo Night Storm variant.
    ///
    /// </details>
    TokyoNightStorm,
    /// 内置的 Tokyo Night Light 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in Tokyo Night Light variant.
    ///
    /// </details>
    TokyoNightLight,
    /// 内置的 Kanagawa Wave 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in Kanagawa Wave variant.
    ///
    /// </details>
    KanagawaWave,
    /// 内置的 Kanagawa Dragon 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in Kanagawa Dragon variant.
    ///
    /// </details>
    KanagawaDragon,
    /// 内置的 Kanagawa Lotus 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in Kanagawa Lotus variant.
    ///
    /// </details>
    KanagawaLotus,
    /// 内置的 Moonfly 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in Moonfly variant.
    ///
    /// </details>
    Moonfly,
    /// 内置的 Nightfly 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in Nightfly variant.
    ///
    /// </details>
    Nightfly,
    /// 内置的 Oxocarbon 变体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in Oxocarbon variant.
    ///
    /// </details>
    Oxocarbon,
    /// 内置的 Ferra 变体：
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The built-in Ferra variant:
    ///
    /// </details>
    Ferra,
    /// 使用 [`Custom`] 调色板的 [`Theme`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A [`Theme`] that uses a [`Custom`] palette.
    ///
    /// </details>
    Custom(Arc<Custom>),
}

impl Theme {
    /// 包含所有已定义主题的列表。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A list with all the defined themes.
    ///
    /// </details>
    pub const ALL: &'static [Self] = &[
        Self::Light,
        Self::Dark,
        Self::Dracula,
        Self::Nord,
        Self::SolarizedLight,
        Self::SolarizedDark,
        Self::GruvboxLight,
        Self::GruvboxDark,
        Self::CatppuccinLatte,
        Self::CatppuccinFrappe,
        Self::CatppuccinMacchiato,
        Self::CatppuccinMocha,
        Self::TokyoNight,
        Self::TokyoNightStorm,
        Self::TokyoNightLight,
        Self::KanagawaWave,
        Self::KanagawaDragon,
        Self::KanagawaLotus,
        Self::Moonfly,
        Self::Nightfly,
        Self::Oxocarbon,
        Self::Ferra,
    ];

    /// 根据给定的 [`Palette`] 创建一个新的自定义 [`Theme`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new custom [`Theme`] from the given [`Palette`].
    ///
    /// </details>
    pub fn custom(
        name: impl Into<Cow<'static, str>>,
        palette: Palette,
    ) -> Self {
        Self::custom_with_fn(name, palette, palette::Extended::generate)
    }

    /// 根据给定的 [`Palette`] 创建一个新的自定义 [`Theme`]，
    /// 并使用自定义的 [`palette::Extended`] 生成器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new custom [`Theme`] from the given [`Palette`], with
    /// a custom generator of a [`palette::Extended`].
    ///
    /// </details>
    pub fn custom_with_fn(
        name: impl Into<Cow<'static, str>>,
        palette: Palette,
        generate: impl FnOnce(Palette) -> palette::Extended,
    ) -> Self {
        Self::Custom(Arc::new(Custom::with_fn(name, palette, generate)))
    }

    /// 返回 [`Theme`] 的 [`Palette`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Palette`] of the [`Theme`].
    ///
    /// </details>
    pub fn palette(&self) -> Palette {
        match self {
            Self::Light => Palette::LIGHT,
            Self::Dark => Palette::DARK,
            Self::Dracula => Palette::DRACULA,
            Self::Nord => Palette::NORD,
            Self::SolarizedLight => Palette::SOLARIZED_LIGHT,
            Self::SolarizedDark => Palette::SOLARIZED_DARK,
            Self::GruvboxLight => Palette::GRUVBOX_LIGHT,
            Self::GruvboxDark => Palette::GRUVBOX_DARK,
            Self::CatppuccinLatte => Palette::CATPPUCCIN_LATTE,
            Self::CatppuccinFrappe => Palette::CATPPUCCIN_FRAPPE,
            Self::CatppuccinMacchiato => Palette::CATPPUCCIN_MACCHIATO,
            Self::CatppuccinMocha => Palette::CATPPUCCIN_MOCHA,
            Self::TokyoNight => Palette::TOKYO_NIGHT,
            Self::TokyoNightStorm => Palette::TOKYO_NIGHT_STORM,
            Self::TokyoNightLight => Palette::TOKYO_NIGHT_LIGHT,
            Self::KanagawaWave => Palette::KANAGAWA_WAVE,
            Self::KanagawaDragon => Palette::KANAGAWA_DRAGON,
            Self::KanagawaLotus => Palette::KANAGAWA_LOTUS,
            Self::Moonfly => Palette::MOONFLY,
            Self::Nightfly => Palette::NIGHTFLY,
            Self::Oxocarbon => Palette::OXOCARBON,
            Self::Ferra => Palette::FERRA,
            Self::Custom(custom) => custom.palette,
        }
    }

    /// 返回 [`Theme`] 的 [`palette::Extended`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`palette::Extended`] of the [`Theme`].
    ///
    /// </details>
    pub fn extended_palette(&self) -> &palette::Extended {
        match self {
            Self::Light => &palette::EXTENDED_LIGHT,
            Self::Dark => &palette::EXTENDED_DARK,
            Self::Dracula => &palette::EXTENDED_DRACULA,
            Self::Nord => &palette::EXTENDED_NORD,
            Self::SolarizedLight => &palette::EXTENDED_SOLARIZED_LIGHT,
            Self::SolarizedDark => &palette::EXTENDED_SOLARIZED_DARK,
            Self::GruvboxLight => &palette::EXTENDED_GRUVBOX_LIGHT,
            Self::GruvboxDark => &palette::EXTENDED_GRUVBOX_DARK,
            Self::CatppuccinLatte => &palette::EXTENDED_CATPPUCCIN_LATTE,
            Self::CatppuccinFrappe => &palette::EXTENDED_CATPPUCCIN_FRAPPE,
            Self::CatppuccinMacchiato => {
                &palette::EXTENDED_CATPPUCCIN_MACCHIATO
            }
            Self::CatppuccinMocha => &palette::EXTENDED_CATPPUCCIN_MOCHA,
            Self::TokyoNight => &palette::EXTENDED_TOKYO_NIGHT,
            Self::TokyoNightStorm => &palette::EXTENDED_TOKYO_NIGHT_STORM,
            Self::TokyoNightLight => &palette::EXTENDED_TOKYO_NIGHT_LIGHT,
            Self::KanagawaWave => &palette::EXTENDED_KANAGAWA_WAVE,
            Self::KanagawaDragon => &palette::EXTENDED_KANAGAWA_DRAGON,
            Self::KanagawaLotus => &palette::EXTENDED_KANAGAWA_LOTUS,
            Self::Moonfly => &palette::EXTENDED_MOONFLY,
            Self::Nightfly => &palette::EXTENDED_NIGHTFLY,
            Self::Oxocarbon => &palette::EXTENDED_OXOCARBON,
            Self::Ferra => &palette::EXTENDED_FERRA,
            Self::Custom(custom) => &custom.extended,
        }
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// 带有自定义 [`Palette`] 的 [`Theme`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A [`Theme`] with a customized [`Palette`].
///
/// </details>
#[derive(Debug, Clone, PartialEq)]
pub struct Custom {
    name: Cow<'static, str>,
    palette: Palette,
    extended: palette::Extended,
}

impl Custom {
    /// 根据给定的 [`Palette`] 创建一个 [`Custom`] 主题。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Custom`] theme from the given [`Palette`].
    ///
    /// </details>
    pub fn new(name: String, palette: Palette) -> Self {
        Self::with_fn(name, palette, palette::Extended::generate)
    }

    /// 根据给定的 [`Palette`] 创建一个 [`Custom`] 主题，
    /// 并使用自定义的 [`palette::Extended`] 生成器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Custom`] theme from the given [`Palette`] with
    /// a custom generator of a [`palette::Extended`].
    ///
    /// </details>
    pub fn with_fn(
        name: impl Into<Cow<'static, str>>,
        palette: Palette,
        generate: impl FnOnce(Palette) -> palette::Extended,
    ) -> Self {
        Self {
            name: name.into(),
            palette,
            extended: generate(palette),
        }
    }
}

impl fmt::Display for Custom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// 主题模式，表示主题的色调或明暗。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A theme mode, denoting the tone or brightness of a theme.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    /// 没有特定的色调。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// No specific tone.
    ///
    /// </details>
    #[default]
    None,
    /// 指代浅色调主题的模式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A mode referring to themes with light tones.
    ///
    /// </details>
    Light,
    /// 指代深色调主题的模式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A mode referring to themes with dark tones.
    ///
    /// </details>
    Dark,
}

/// 主题的基础样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The base style of a theme.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// 应用程序的背景 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The background [`Color`] of the application.
    ///
    /// </details>
    pub background_color: Color,

    /// 应用程序的默认文本 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default text [`Color`] of the application.
    ///
    /// </details>
    pub text_color: Color,
}

/// 主题默认的空白样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The default blank style of a theme.
///
/// </details>
pub trait Base {
    /// 返回首选 [`Mode`] 对应的默认主题。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the default theme for the preferred [`Mode`].
    ///
    /// </details>
    fn default(preference: Mode) -> Self;

    /// 返回主题的 [`Mode`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Mode`] of the theme.
    ///
    /// </details>
    fn mode(&self) -> Mode;

    /// 返回主题默认的基础 [`Style`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the default base [`Style`] of the theme.
    ///
    /// </details>
    fn base(&self) -> Style;

    /// 返回主题的颜色 [`Palette`]。
    ///
    /// 运行时可能会把这个 [`Palette`] 用于
    /// 调试目的，例如显示性能
    /// 指标或开发者工具。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the color [`Palette`] of the theme.
    ///
    /// This [`Palette`] may be used by the runtime for
    /// debugging purposes; like displaying performance
    /// metrics or devtools.
    ///
    /// </details>
    fn palette(&self) -> Option<Palette>;

    /// 返回主题的唯一名称。
    ///
    /// 某些部件可以用这个名称来高效地检测主题
    /// 变化。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the unique name of the theme.
    ///
    /// This name may be used to efficiently detect theme
    /// changes in some widgets.
    ///
    /// </details>
    fn name(&self) -> &str;
}

impl Base for Theme {
    fn default(preference: Mode) -> Self {
        use std::env;
        use std::sync::OnceLock;

        static SYSTEM: OnceLock<Option<Theme>> = OnceLock::new();

        let system = SYSTEM.get_or_init(|| {
            let name = env::var("ICED_THEME").ok()?;

            Theme::ALL
                .iter()
                .find(|theme| theme.to_string() == name)
                .cloned()
        });

        if let Some(system) = system {
            return system.clone();
        }

        match preference {
            Mode::None | Mode::Light => Self::Light,
            Mode::Dark => Self::Dark,
        }
    }

    fn mode(&self) -> Mode {
        if self.extended_palette().is_dark {
            Mode::Dark
        } else {
            Mode::Light
        }
    }

    fn base(&self) -> Style {
        default(self)
    }

    fn palette(&self) -> Option<Palette> {
        Some(self.palette())
    }

    fn name(&self) -> &str {
        match self {
            Self::Light => "Light",
            Self::Dark => "Dark",
            Self::Dracula => "Dracula",
            Self::Nord => "Nord",
            Self::SolarizedLight => "Solarized Light",
            Self::SolarizedDark => "Solarized Dark",
            Self::GruvboxLight => "Gruvbox Light",
            Self::GruvboxDark => "Gruvbox Dark",
            Self::CatppuccinLatte => "Catppuccin Latte",
            Self::CatppuccinFrappe => "Catppuccin Frappé",
            Self::CatppuccinMacchiato => "Catppuccin Macchiato",
            Self::CatppuccinMocha => "Catppuccin Mocha",
            Self::TokyoNight => "Tokyo Night",
            Self::TokyoNightStorm => "Tokyo Night Storm",
            Self::TokyoNightLight => "Tokyo Night Light",
            Self::KanagawaWave => "Kanagawa Wave",
            Self::KanagawaDragon => "Kanagawa Dragon",
            Self::KanagawaLotus => "Kanagawa Lotus",
            Self::Moonfly => "Moonfly",
            Self::Nightfly => "Nightfly",
            Self::Oxocarbon => "Oxocarbon",
            Self::Ferra => "Ferra",
            Self::Custom(custom) => &custom.name,
        }
    }
}

/// 内置 [`Theme`] 的默认 [`Style`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The default [`Style`] of a built-in [`Theme`].
///
/// </details>
pub fn default(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        background_color: palette.background.base.color,
        text_color: palette.background.base.text,
    }
}
