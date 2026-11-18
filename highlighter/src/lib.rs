//! iced 的语法高亮器。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! A syntax highlighter for iced.
//!
//! </details>
use iced_core as core;

use crate::core::Color;
use crate::core::font::{self, Font};
use crate::core::text::highlighter::{self, Format};

use std::ops::Range;
use std::sync::LazyLock;

use syntect::highlighting;
use syntect::parsing;
use two_face::re_exports::syntect;

static SYNTAXES: LazyLock<parsing::SyntaxSet> =
    LazyLock::new(two_face::syntax::extra_no_newlines);

static THEMES: LazyLock<highlighting::ThemeSet> =
    LazyLock::new(highlighting::ThemeSet::load_defaults);

const LINES_PER_SNAPSHOT: usize = 50;

/// 一个语法高亮器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A syntax highlighter.
///
/// </details>
#[derive(Debug)]
pub struct Highlighter {
    syntax: &'static parsing::SyntaxReference,
    highlighter: highlighting::Highlighter<'static>,
    caches: Vec<(parsing::ParseState, parsing::ScopeStack)>,
    current_line: usize,
}

impl highlighter::Highlighter for Highlighter {
    type Settings = Settings;
    type Highlight = Highlight;

    type Iterator<'a> =
        Box<dyn Iterator<Item = (Range<usize>, Self::Highlight)> + 'a>;

    fn new(settings: &Self::Settings) -> Self {
        let syntax = SYNTAXES
            .find_syntax_by_token(&settings.token)
            .unwrap_or_else(|| SYNTAXES.find_syntax_plain_text());

        let highlighter = highlighting::Highlighter::new(
            &THEMES.themes[settings.theme.key()],
        );

        let parser = parsing::ParseState::new(syntax);
        let stack = parsing::ScopeStack::new();

        Highlighter {
            syntax,
            highlighter,
            caches: vec![(parser, stack)],
            current_line: 0,
        }
    }

    fn update(&mut self, new_settings: &Self::Settings) {
        self.syntax = SYNTAXES
            .find_syntax_by_token(&new_settings.token)
            .unwrap_or_else(|| SYNTAXES.find_syntax_plain_text());

        self.highlighter = highlighting::Highlighter::new(
            &THEMES.themes[new_settings.theme.key()],
        );

        // Restart the highlighter
        // 重启高亮器
        self.change_line(0);
    }

    fn change_line(&mut self, line: usize) {
        let snapshot = line / LINES_PER_SNAPSHOT;

        if snapshot <= self.caches.len() {
            self.caches.truncate(snapshot);
            self.current_line = snapshot * LINES_PER_SNAPSHOT;
        } else {
            self.caches.truncate(1);
            self.current_line = 0;
        }

        let (parser, stack) =
            self.caches.last().cloned().unwrap_or_else(|| {
                (
                    parsing::ParseState::new(self.syntax),
                    parsing::ScopeStack::new(),
                )
            });

        self.caches.push((parser, stack));
    }

    fn highlight_line(&mut self, line: &str) -> Self::Iterator<'_> {
        if self.current_line / LINES_PER_SNAPSHOT >= self.caches.len() {
            let (parser, stack) =
                self.caches.last().expect("Caches must not be empty");

            self.caches.push((parser.clone(), stack.clone()));
        }

        self.current_line += 1;

        let (parser, stack) =
            self.caches.last_mut().expect("Caches must not be empty");

        let ops = parser.parse_line(line, &SYNTAXES).unwrap_or_default();

        Box::new(scope_iterator(ops, line, stack, &self.highlighter))
    }

    fn current_line(&self) -> usize {
        self.current_line
    }
}

fn scope_iterator<'a>(
    ops: Vec<(usize, parsing::ScopeStackOp)>,
    line: &str,
    stack: &'a mut parsing::ScopeStack,
    highlighter: &'a highlighting::Highlighter<'static>,
) -> impl Iterator<Item = (Range<usize>, Highlight)> + 'a {
    ScopeRangeIterator {
        ops,
        line_length: line.len(),
        index: 0,
        last_str_index: 0,
    }
    .filter_map(move |(range, scope)| {
        let _ = stack.apply(&scope);

        if range.is_empty() {
            None
        } else {
            Some((
                range,
                Highlight(highlighter.style_mod_for_stack(&stack.scopes)),
            ))
        }
    })
}

/// 一个流式语法高亮器。
///
/// 它可以高效地高亮不可变的词法单元流。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A streaming syntax highlighter.
///
/// It can efficiently highlight an immutable stream of tokens.
///
/// </details>
#[derive(Debug)]
pub struct Stream {
    syntax: &'static parsing::SyntaxReference,
    highlighter: highlighting::Highlighter<'static>,
    commit: (parsing::ParseState, parsing::ScopeStack),
    state: parsing::ParseState,
    stack: parsing::ScopeStack,
}

impl Stream {
    /// 创建一个新的 [`Stream`] 高亮器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Stream`] highlighter.
    ///
    /// </details>
    pub fn new(settings: &Settings) -> Self {
        let syntax = SYNTAXES
            .find_syntax_by_token(&settings.token)
            .unwrap_or_else(|| SYNTAXES.find_syntax_plain_text());

        let highlighter = highlighting::Highlighter::new(
            &THEMES.themes[settings.theme.key()],
        );

        let state = parsing::ParseState::new(syntax);
        let stack = parsing::ScopeStack::new();

        Self {
            syntax,
            highlighter,
            commit: (state.clone(), stack.clone()),
            state,
            stack,
        }
    }

    /// 从上一次提交的位置开始高亮给定的行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Highlights the given line from the last commit.
    ///
    /// </details>
    pub fn highlight_line(
        &mut self,
        line: &str,
    ) -> impl Iterator<Item = (Range<usize>, Highlight)> + '_ {
        self.state = self.commit.0.clone();
        self.stack = self.commit.1.clone();

        let ops = self.state.parse_line(line, &SYNTAXES).unwrap_or_default();
        scope_iterator(ops, line, &mut self.stack, &self.highlighter)
    }

    /// 提交上一次高亮的行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Commits the last highlighted line.
    ///
    /// </details>
    pub fn commit(&mut self) {
        self.commit = (self.state.clone(), self.stack.clone());
    }

    /// 重置 [`Stream`] 高亮器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Resets the [`Stream`] highlighter.
    ///
    /// </details>
    pub fn reset(&mut self) {
        self.state = parsing::ParseState::new(self.syntax);
        self.stack = parsing::ScopeStack::new();
        self.commit = (self.state.clone(), self.stack.clone());
    }
}

/// [`Highlighter`] 的设置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The settings of a [`Highlighter`].
///
/// </details>
#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    /// [`Highlighter`] 的 [`Theme`]。
    ///
    /// 它决定高亮时使用的配色方案。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Theme`] of the [`Highlighter`].
    ///
    /// It dictates the color scheme that will be used for highlighting.
    ///
    /// </details>
    pub theme: Theme,
    /// 要高亮的文件的扩展名，或语言的名称。
    ///
    /// [`Highlighter`] 会用这个标记自动确定
    /// 高亮时使用的语法。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The extension of the file or the name of the language to highlight.
    ///
    /// The [`Highlighter`] will use the token to automatically determine
    /// the grammar to use for highlighting.
    ///
    /// </details>
    pub token: String,
}

/// [`Highlighter`] 产生的一个高亮。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A highlight produced by a [`Highlighter`].
///
/// </details>
#[derive(Debug)]
pub struct Highlight(highlighting::StyleModifier);

impl Highlight {
    /// 返回这个 [`Highlight`] 的颜色。
    ///
    /// 如果是 `None`，原来的文本颜色应保持不变。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the color of this [`Highlight`].
    ///
    /// If `None`, the original text color should be unchanged.
    ///
    /// </details>
    pub fn color(&self) -> Option<Color> {
        self.0.foreground.map(|color| {
            Color::from_rgba8(color.r, color.g, color.b, color.a as f32 / 255.0)
        })
    }

    /// 返回这个 [`Highlight`] 的字体。
    ///
    /// 如果是 `None`，原来的字体应保持不变。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the font of this [`Highlight`].
    ///
    /// If `None`, the original font should be unchanged.
    ///
    /// </details>
    pub fn font(&self) -> Option<Font> {
        self.0.font_style.and_then(|style| {
            let bold = style.contains(highlighting::FontStyle::BOLD);
            let italic = style.contains(highlighting::FontStyle::ITALIC);

            if bold || italic {
                Some(Font {
                    weight: if bold {
                        font::Weight::Bold
                    } else {
                        font::Weight::Normal
                    },
                    style: if italic {
                        font::Style::Italic
                    } else {
                        font::Style::Normal
                    },
                    ..Font::MONOSPACE
                })
            } else {
                None
            }
        })
    }

    /// 返回 [`Highlight`] 的 [`Format`]。
    ///
    /// 它同时包含 [`color`] 和 [`font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Format`] of the [`Highlight`].
    ///
    /// It contains both the [`color`] and the [`font`].
    ///
    /// </details>
    ///
    /// [`color`]: Self::color
    /// [`font`]: Self::font
    pub fn to_format(&self) -> Format<Font> {
        Format {
            color: self.color(),
            font: self.font(),
        }
    }
}

/// 一个高亮主题。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A highlighting theme.
///
/// </details>
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    SolarizedDark,
    Base16Mocha,
    Base16Ocean,
    Base16Eighties,
    InspiredGitHub,
}

impl Theme {
    /// 包含所有可用主题的静态切片。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A static slice containing all the available themes.
    ///
    /// </details>
    pub const ALL: &'static [Self] = &[
        Self::SolarizedDark,
        Self::Base16Mocha,
        Self::Base16Ocean,
        Self::Base16Eighties,
        Self::InspiredGitHub,
    ];

    /// 如果 [`Theme`] 是深色的，返回 `true`，否则返回 false。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns `true` if the [`Theme`] is dark, and false otherwise.
    ///
    /// </details>
    pub fn is_dark(self) -> bool {
        match self {
            Self::SolarizedDark
            | Self::Base16Mocha
            | Self::Base16Ocean
            | Self::Base16Eighties => true,
            Self::InspiredGitHub => false,
        }
    }

    fn key(self) -> &'static str {
        match self {
            Theme::SolarizedDark => "Solarized (dark)",
            Theme::Base16Mocha => "base16-mocha.dark",
            Theme::Base16Ocean => "base16-ocean.dark",
            Theme::Base16Eighties => "base16-eighties.dark",
            Theme::InspiredGitHub => "InspiredGitHub",
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::SolarizedDark => write!(f, "Solarized Dark"),
            Theme::Base16Mocha => write!(f, "Mocha"),
            Theme::Base16Ocean => write!(f, "Ocean"),
            Theme::Base16Eighties => write!(f, "Eighties"),
            Theme::InspiredGitHub => write!(f, "Inspired GitHub"),
        }
    }
}

struct ScopeRangeIterator {
    ops: Vec<(usize, parsing::ScopeStackOp)>,
    line_length: usize,
    index: usize,
    last_str_index: usize,
}

impl Iterator for ScopeRangeIterator {
    type Item = (std::ops::Range<usize>, parsing::ScopeStackOp);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.ops.len() {
            return None;
        }

        let next_str_i = if self.index == self.ops.len() {
            self.line_length
        } else {
            self.ops[self.index].0
        };

        let range = self.last_str_index..next_str_i;
        self.last_str_index = next_str_i;

        let op = if self.index == 0 {
            parsing::ScopeStackOp::Noop
        } else {
            self.ops[self.index - 1].1.clone()
        };

        self.index += 1;
        Some((range, op))
    }
}
