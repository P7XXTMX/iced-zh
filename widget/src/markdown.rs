//! Markdown 部件可以解析并显示 Markdown。
//!
//! 你可以启用 `highlighter` 特性，为代码块提供
//! 语法高亮。
//!
//! 目前只支持 [`Item`] 的各个变体。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Markdown widgets can parse and display Markdown.
//!
//! You can enable the `highlighter` feature for syntax highlighting
//! in code blocks.
//!
//! Only the variants of [`Item`] are currently supported.
//!
//! </details>
//!
//! # 示例（Example）
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
//! # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
//! #
//! use iced::widget::markdown;
//! use iced::Theme;
//!
//! struct State {
//!    markdown: Vec<markdown::Item>,
//! }
//!
//! enum Message {
//!     LinkClicked(markdown::Uri),
//! }
//!
//! impl State {
//!     pub fn new() -> Self {
//!         Self {
//!             markdown: markdown::parse("This is some **Markdown**!").collect(),
//!         }
//!     }
//!
//!     fn view(&self) -> Element<'_, Message> {
//!         markdown::view(&self.markdown, Theme::TokyoNight)
//!             .map(Message::LinkClicked)
//!             .into()
//!     }
//!
//!     fn update(state: &mut State, message: Message) {
//!         match message {
//!             Message::LinkClicked(url) => {
//!                 println!("The following url was clicked: {url}");
//!             }
//!         }
//!     }
//! }
//! ```
use crate::core::alignment;
use crate::core::border;
use crate::core::font::{self, Font};
use crate::core::padding;
use crate::core::theme;
use crate::core::{
    self, Color, Element, Length, Padding, Pixels, Theme, color,
};
use crate::{
    checkbox, column, container, rich_text, row, rule, scrollable, span, text,
};

use std::borrow::BorrowMut;
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::mem;
use std::ops::Range;
use std::rc::Rc;
use std::sync::Arc;

pub use core::text::Highlight;
pub use pulldown_cmark::HeadingLevel;

/// 表示 Markdown 文档中某个 [URI] 的 [`String`]
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A [`String`] representing a [URI] in a Markdown document
///
/// </details>
///
/// [URI]: https://en.wikipedia.org/wiki/Uniform_Resource_Identifier
pub type Uri = String;

/// 一段已解析的 Markdown。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A bunch of Markdown that has been parsed.
///
/// </details>
#[derive(Debug, Default)]
pub struct Content {
    items: Vec<Item>,
    incomplete: HashMap<usize, Section>,
    state: State,
}

#[derive(Debug)]
struct Section {
    content: String,
    broken_links: HashSet<String>,
}

impl Content {
    /// 创建一个新的空 [`Content`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new empty [`Content`].
    ///
    /// </details>
    pub fn new() -> Self {
        Self::default()
    }

    /// 通过解析给定的 Markdown 创建新的 [`Content`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates some new [`Content`] by parsing the given Markdown.
    ///
    /// </details>
    pub fn parse(markdown: &str) -> Self {
        let mut content = Self::new();
        content.push_str(markdown);
        content
    }

    /// 向 [`Content`] 推入更多 Markdown，并以增量方式解析！
    ///
    /// 当你有很长的 Markdown 流时，这特别有用，例如
    /// 大文件或可能很长的回复。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Pushes more Markdown into the [`Content`]; parsing incrementally!
    ///
    /// This is specially useful when you have long streams of Markdown; like
    /// big files or potentially long replies.
    ///
    /// </details>
    pub fn push_str(&mut self, markdown: &str) {
        if markdown.is_empty() {
            return;
        }

        // Append to last leftover text
        // 追加到上次剩余的文本之后
        let mut leftover = std::mem::take(&mut self.state.leftover);
        leftover.push_str(markdown);

        let input = if leftover.trim_end().ends_with('|') {
            leftover.trim_end().trim_end_matches('|')
        } else {
            leftover.as_str()
        };

        // Pop the last item
        // 弹出最后一项
        let _ = self.items.pop();

        // Re-parse last item and new text
        // 重新解析最后一项和新文本
        for (item, source, broken_links) in parse_with(&mut self.state, input) {
            if !broken_links.is_empty() {
                let _ = self.incomplete.insert(
                    self.items.len(),
                    Section {
                        content: source.to_owned(),
                        broken_links,
                    },
                );
            }

            self.items.push(item);
        }

        self.state.leftover.push_str(&leftover[input.len()..]);

        // Re-parse incomplete sections if new references are available
        // 如果有新的引用可用，重新解析不完整的部分
        if !self.incomplete.is_empty() {
            self.incomplete.retain(|index, section| {
                if self.items.len() <= *index {
                    return false;
                }

                let broken_links_before = section.broken_links.len();

                section
                    .broken_links
                    .retain(|link| !self.state.references.contains_key(link));

                if broken_links_before != section.broken_links.len() {
                    let mut state = State {
                        leftover: String::new(),
                        references: self.state.references.clone(),
                        images: HashSet::new(),
                        #[cfg(feature = "highlighter")]
                        highlighter: None,
                    };

                    if let Some((item, _source, _broken_links)) =
                        parse_with(&mut state, &section.content).next()
                    {
                        self.items[*index] = item;
                    }

                    self.state.images.extend(state.images.drain());
                    drop(state);
                }

                !section.broken_links.is_empty()
            });
        }
    }

    /// 返回已经可以渲染的 Markdown 条目。
    ///
    /// 你可以用 [`view`] 把它们转换成 [`Element`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the Markdown items, ready to be rendered.
    ///
    /// You can use [`view`] to turn them into an [`Element`].
    ///
    /// </details>
    pub fn items(&self) -> &[Item] {
        &self.items
    }

    /// 返回 [`Content`] 中出现的 Markdown 图像的 URL。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the URLs of the Markdown images present in the [`Content`].
    ///
    /// </details>
    pub fn images(&self) -> &HashSet<Uri> {
        &self.state.images
    }
}

/// Markdown 条目。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A Markdown item.
///
/// </details>
#[derive(Debug, Clone)]
pub enum Item {
    /// 标题。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A heading.
    ///
    /// </details>
    Heading(pulldown_cmark::HeadingLevel, Text),
    /// 段落。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A paragraph.
    ///
    /// </details>
    Paragraph(Text),
    /// 代码块。
    ///
    /// 你可以启用 `highlighter` 特性来获得语法高亮。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A code block.
    ///
    /// You can enable the `highlighter` feature for syntax highlighting.
    ///
    /// </details>
    CodeBlock {
        /// 代码块的语言（如果有）。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The language of the code block, if any.
        ///
        /// </details>
        language: Option<String>,
        /// 代码块的原始代码。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The raw code of the code block.
        ///
        /// </details>
        code: String,
        /// 代码块中带样式的文本行。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The styled lines of text in the code block.
        ///
        /// </details>
        lines: Vec<Text>,
    },
    /// 列表。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A list.
    ///
    /// </details>
    List {
        /// 列表的起始编号，如果它是有序列表的话。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The first number of the list, if it is ordered.
        ///
        /// </details>
        start: Option<u64>,
        /// 列表的各项。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The items of the list.
        ///
        /// </details>
        bullets: Vec<Bullet>,
    },
    /// 图像。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An image.
    ///
    /// </details>
    Image {
        /// 图像的目标 URL。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The destination URL of the image.
        ///
        /// </details>
        url: Uri,
        /// 图像的标题。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The title of the image.
        ///
        /// </details>
        title: String,
        /// 图像的替代文本。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The alternative text of the image.
        ///
        /// </details>
        alt: Text,
    },
    /// 引用块。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A quote.
    ///
    /// </details>
    Quote(Vec<Item>),
    /// 水平分隔线。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A horizontal separator.
    ///
    /// </details>
    Rule,
    /// 表格。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A table.
    ///
    /// </details>
    Table {
        /// 表格的列。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The columns of the table.
        ///
        /// </details>
        columns: Vec<Column>,
        /// 表格的行。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The rows of the table.
        ///
        /// </details>
        rows: Vec<Row>,
    },
}

/// 表格的列。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The column of a table.
///
/// </details>
#[derive(Debug, Clone)]
pub struct Column {
    /// 列的表头。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The header of the column.
    ///
    /// </details>
    pub header: Vec<Item>,
    /// 列的对齐方式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The alignment of the column.
    ///
    /// </details>
    pub alignment: pulldown_cmark::Alignment,
}

/// 表格的行。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The row of a table.
///
/// </details>
#[derive(Debug, Clone)]
pub struct Row {
    /// 行中的单元格。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The cells of the row.
    ///
    /// </details>
    cells: Vec<Vec<Item>>,
}

/// 一段已解析的 Markdown 文本。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A bunch of parsed Markdown text.
///
/// </details>
#[derive(Debug, Clone)]
pub struct Text {
    spans: Vec<Span>,
    last_style: Cell<Option<Style>>,
    last_styled_spans: RefCell<Arc<[text::Span<'static, Uri>]>>,
}

impl Text {
    fn new(spans: Vec<Span>) -> Self {
        Self {
            spans,
            last_style: Cell::default(),
            last_styled_spans: RefCell::default(),
        }
    }

    /// 返回可用于给定样式的 [`rich_text()`] span。
    ///
    /// 此方法会为你做缓存。只有在给定的 [`Style`] 发生变化时，
    /// 它才会重新分配。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`rich_text()`] spans ready to be used for the given style.
    ///
    /// This method performs caching for you. It will only reallocate if the [`Style`]
    /// provided changes.
    ///
    /// </details>
    pub fn spans(&self, style: Style) -> Arc<[text::Span<'static, Uri>]> {
        if Some(style) != self.last_style.get() {
            *self.last_styled_spans.borrow_mut() =
                self.spans.iter().map(|span| span.view(&style)).collect();

            self.last_style.set(Some(style));
        }

        self.last_styled_spans.borrow().clone()
    }
}

#[derive(Debug, Clone)]
enum Span {
    Standard {
        text: String,
        strikethrough: bool,
        link: Option<Uri>,
        strong: bool,
        emphasis: bool,
        code: bool,
    },
    #[cfg(feature = "highlighter")]
    Highlight {
        text: String,
        color: Option<Color>,
        font: Option<Font>,
    },
}

impl Span {
    fn view(&self, style: &Style) -> text::Span<'static, Uri> {
        match self {
            Span::Standard {
                text,
                strikethrough,
                link,
                strong,
                emphasis,
                code,
            } => {
                let span = span(text.clone()).strikethrough(*strikethrough);

                let span = if *code {
                    span.font(style.inline_code_font)
                        .color(style.inline_code_color)
                        .background(style.inline_code_highlight.background)
                        .border(style.inline_code_highlight.border)
                        .padding(style.inline_code_padding)
                } else if *strong || *emphasis {
                    span.font(Font {
                        weight: if *strong {
                            font::Weight::Bold
                        } else {
                            font::Weight::Normal
                        },
                        style: if *emphasis {
                            font::Style::Italic
                        } else {
                            font::Style::Normal
                        },
                        ..style.font
                    })
                } else {
                    span.font(style.font)
                };

                if let Some(link) = link.as_ref() {
                    span.color(style.link_color).link(link.clone())
                } else {
                    span
                }
            }
            #[cfg(feature = "highlighter")]
            Span::Highlight { text, color, font } => {
                span(text.clone()).color_maybe(*color).font_maybe(*font)
            }
        }
    }
}

/// 列表的项。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The item of a list.
///
/// </details>
#[derive(Debug, Clone)]
pub enum Bullet {
    /// 简单的列表项。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A simple bullet point.
    ///
    /// </details>
    Point {
        /// 列表项的内容。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The contents of the bullet point.
        ///
        /// </details>
        items: Vec<Item>,
    },
    /// 任务项。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A task.
    ///
    /// </details>
    Task {
        /// 任务项的内容。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The contents of the task.
        ///
        /// </details>
        items: Vec<Item>,
        /// 任务是否已完成。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Whether the task is done or not.
        ///
        /// </details>
        done: bool,
    },
}

impl Bullet {
    fn items(&self) -> &[Item] {
        match self {
            Bullet::Point { items } | Bullet::Task { items, .. } => items,
        }
    }

    fn push(&mut self, item: Item) {
        let (Bullet::Point { items } | Bullet::Task { items, .. }) = self;

        items.push(item);
    }
}

/// 解析给定的 Markdown 内容。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Parse the given Markdown content.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// #
/// use iced::widget::markdown;
/// use iced::Theme;
///
/// struct State {
///    markdown: Vec<markdown::Item>,
/// }
///
/// enum Message {
///     LinkClicked(markdown::Uri),
/// }
///
/// impl State {
///     pub fn new() -> Self {
///         Self {
///             markdown: markdown::parse("This is some **Markdown**!").collect(),
///         }
///     }
///
///     fn view(&self) -> Element<'_, Message> {
///         markdown::view(&self.markdown, Theme::TokyoNight)
///             .map(Message::LinkClicked)
///             .into()
///     }
///
///     fn update(state: &mut State, message: Message) {
///         match message {
///             Message::LinkClicked(url) => {
///                 println!("The following url was clicked: {url}");
///             }
///         }
///     }
/// }
/// ```
pub fn parse(markdown: &str) -> impl Iterator<Item = Item> + '_ {
    parse_with(State::default(), markdown)
        .map(|(item, _source, _broken_links)| item)
}

#[derive(Debug, Default)]
struct State {
    leftover: String,
    references: HashMap<String, String>,
    images: HashSet<Uri>,
    #[cfg(feature = "highlighter")]
    highlighter: Option<Highlighter>,
}

#[cfg(feature = "highlighter")]
#[derive(Debug)]
struct Highlighter {
    lines: Vec<(String, Vec<Span>)>,
    language: String,
    parser: iced_highlighter::Stream,
    current: usize,
}

#[cfg(feature = "highlighter")]
impl Highlighter {
    pub fn new(language: &str) -> Self {
        Self {
            lines: Vec::new(),
            parser: iced_highlighter::Stream::new(
                &iced_highlighter::Settings {
                    theme: iced_highlighter::Theme::Base16Ocean,
                    token: language.to_owned(),
                },
            ),
            language: language.to_owned(),
            current: 0,
        }
    }

    pub fn prepare(&mut self) {
        self.current = 0;
    }

    pub fn highlight_line(&mut self, text: &str) -> &[Span] {
        match self.lines.get(self.current) {
            Some(line) if line.0 == text => {}
            _ => {
                if self.current + 1 < self.lines.len() {
                    log::debug!("Resetting highlighter...");
                    self.parser.reset();
                    self.lines.truncate(self.current);

                    for line in &self.lines {
                        log::debug!(
                            "Refeeding {n} lines",
                            n = self.lines.len()
                        );

                        let _ = self.parser.highlight_line(&line.0);
                    }
                }

                log::trace!("Parsing: {text}", text = text.trim_end());

                if self.current + 1 < self.lines.len() {
                    self.parser.commit();
                }

                let mut spans = Vec::new();

                for (range, highlight) in self.parser.highlight_line(text) {
                    spans.push(Span::Highlight {
                        text: text[range].to_owned(),
                        color: highlight.color(),
                        font: highlight.font(),
                    });
                }

                if self.current + 1 == self.lines.len() {
                    let _ = self.lines.pop();
                }

                self.lines.push((text.to_owned(), spans));
            }
        }

        self.current += 1;

        &self
            .lines
            .get(self.current - 1)
            .expect("Line must be parsed")
            .1
    }
}

fn parse_with<'a>(
    mut state: impl BorrowMut<State> + 'a,
    markdown: &'a str,
) -> impl Iterator<Item = (Item, &'a str, HashSet<String>)> + 'a {
    enum Scope {
        List(List),
        Quote(Vec<Item>),
        Table {
            alignment: Vec<pulldown_cmark::Alignment>,
            columns: Vec<Column>,
            rows: Vec<Row>,
            current: Vec<Item>,
        },
    }

    struct List {
        start: Option<u64>,
        bullets: Vec<Bullet>,
    }

    let broken_links = Rc::new(RefCell::new(HashSet::new()));

    let mut spans = Vec::new();
    let mut code = String::new();
    let mut code_language = None;
    let mut code_lines = Vec::new();
    let mut strong = false;
    let mut emphasis = false;
    let mut strikethrough = false;
    let mut metadata = false;
    let mut code_block = false;
    let mut link = None;
    let mut image = None;
    let mut stack = Vec::new();

    #[cfg(feature = "highlighter")]
    let mut highlighter = None;

    let parser = pulldown_cmark::Parser::new_with_broken_link_callback(
        markdown,
        pulldown_cmark::Options::ENABLE_YAML_STYLE_METADATA_BLOCKS
            | pulldown_cmark::Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS
            | pulldown_cmark::Options::ENABLE_TABLES
            | pulldown_cmark::Options::ENABLE_STRIKETHROUGH
            | pulldown_cmark::Options::ENABLE_TASKLISTS,
        {
            let references = state.borrow().references.clone();
            let broken_links = broken_links.clone();

            Some(move |broken_link: pulldown_cmark::BrokenLink<'_>| {
                if let Some(reference) =
                    references.get(broken_link.reference.as_ref())
                {
                    Some((
                        pulldown_cmark::CowStr::from(reference.to_owned()),
                        broken_link.reference.into_static(),
                    ))
                } else {
                    let _ = RefCell::borrow_mut(&broken_links)
                        .insert(broken_link.reference.into_string());

                    None
                }
            })
        },
    );

    let references = &mut state.borrow_mut().references;

    for reference in parser.reference_definitions().iter() {
        let _ = references
            .insert(reference.0.to_owned(), reference.1.dest.to_string());
    }

    let produce = move |state: &mut State,
                        stack: &mut Vec<Scope>,
                        item,
                        source: Range<usize>| {
        if let Some(scope) = stack.last_mut() {
            match scope {
                Scope::List(list) => {
                    list.bullets.last_mut().expect("item context").push(item);
                }
                Scope::Quote(items) => {
                    items.push(item);
                }
                Scope::Table { current, .. } => {
                    current.push(item);
                }
            }

            None
        } else {
            state.leftover = markdown[source.start..].to_owned();

            Some((
                item,
                &markdown[source.start..source.end],
                broken_links.take(),
            ))
        }
    };

    let parser = parser.into_offset_iter();

    // We want to keep the `spans` capacity
    // 我们希望保留 `spans` 的容量
    #[allow(clippy::drain_collect)]
    parser.filter_map(move |(event, source)| match event {
        pulldown_cmark::Event::Start(tag) => match tag {
            pulldown_cmark::Tag::Strong if !metadata => {
                strong = true;
                None
            }
            pulldown_cmark::Tag::Emphasis if !metadata => {
                emphasis = true;
                None
            }
            pulldown_cmark::Tag::Strikethrough if !metadata => {
                strikethrough = true;
                None
            }
            pulldown_cmark::Tag::Link { dest_url, .. } if !metadata => {
                link = Some(dest_url.into_string());
                None
            }
            pulldown_cmark::Tag::Image {
                dest_url, title, ..
            } if !metadata => {
                image = Some((dest_url.into_string(), title.into_string()));
                None
            }
            pulldown_cmark::Tag::List(first_item) if !metadata => {
                let prev = if spans.is_empty() {
                    None
                } else {
                    produce(
                        state.borrow_mut(),
                        &mut stack,
                        Item::Paragraph(Text::new(spans.drain(..).collect())),
                        source,
                    )
                };

                stack.push(Scope::List(List {
                    start: first_item,
                    bullets: Vec::new(),
                }));

                prev
            }
            pulldown_cmark::Tag::Item => {
                if let Some(Scope::List(list)) = stack.last_mut() {
                    list.bullets.push(Bullet::Point { items: Vec::new() });
                }

                None
            }
            pulldown_cmark::Tag::BlockQuote(_kind) if !metadata => {
                let prev = if spans.is_empty() {
                    None
                } else {
                    produce(
                        state.borrow_mut(),
                        &mut stack,
                        Item::Paragraph(Text::new(spans.drain(..).collect())),
                        source,
                    )
                };

                stack.push(Scope::Quote(Vec::new()));

                prev
            }
            pulldown_cmark::Tag::CodeBlock(
                pulldown_cmark::CodeBlockKind::Fenced(language),
            ) if !metadata => {
                #[cfg(feature = "highlighter")]
                {
                    highlighter = Some({
                        let mut highlighter = state
                            .borrow_mut()
                            .highlighter
                            .take()
                            .filter(|highlighter| {
                                highlighter.language == language.as_ref()
                            })
                            .unwrap_or_else(|| {
                                Highlighter::new(
                                    language
                                        .split(',')
                                        .next()
                                        .unwrap_or_default(),
                                )
                            });

                        highlighter.prepare();

                        highlighter
                    });
                }

                code_block = true;
                code_language =
                    (!language.is_empty()).then(|| language.into_string());

                if spans.is_empty() {
                    None
                } else {
                    produce(
                        state.borrow_mut(),
                        &mut stack,
                        Item::Paragraph(Text::new(spans.drain(..).collect())),
                        source,
                    )
                }
            }
            pulldown_cmark::Tag::MetadataBlock(_) => {
                metadata = true;
                None
            }
            pulldown_cmark::Tag::Table(alignment) => {
                stack.push(Scope::Table {
                    columns: Vec::with_capacity(alignment.len()),
                    alignment,
                    current: Vec::new(),
                    rows: Vec::new(),
                });

                None
            }
            pulldown_cmark::Tag::TableHead => {
                strong = true;
                None
            }
            pulldown_cmark::Tag::TableRow => {
                let Scope::Table { rows, .. } = stack.last_mut()? else {
                    return None;
                };

                rows.push(Row { cells: Vec::new() });
                None
            }
            _ => None,
        },
        pulldown_cmark::Event::End(tag) => match tag {
            pulldown_cmark::TagEnd::Heading(level) if !metadata => produce(
                state.borrow_mut(),
                &mut stack,
                Item::Heading(level, Text::new(spans.drain(..).collect())),
                source,
            ),
            pulldown_cmark::TagEnd::Strong if !metadata => {
                strong = false;
                None
            }
            pulldown_cmark::TagEnd::Emphasis if !metadata => {
                emphasis = false;
                None
            }
            pulldown_cmark::TagEnd::Strikethrough if !metadata => {
                strikethrough = false;
                None
            }
            pulldown_cmark::TagEnd::Link if !metadata => {
                link = None;
                None
            }
            pulldown_cmark::TagEnd::Paragraph if !metadata => {
                if spans.is_empty() {
                    None
                } else {
                    produce(
                        state.borrow_mut(),
                        &mut stack,
                        Item::Paragraph(Text::new(spans.drain(..).collect())),
                        source,
                    )
                }
            }
            pulldown_cmark::TagEnd::Item if !metadata => {
                if spans.is_empty() {
                    None
                } else {
                    produce(
                        state.borrow_mut(),
                        &mut stack,
                        Item::Paragraph(Text::new(spans.drain(..).collect())),
                        source,
                    )
                }
            }
            pulldown_cmark::TagEnd::List(_) if !metadata => {
                let scope = stack.pop()?;

                let Scope::List(list) = scope else {
                    return None;
                };

                produce(
                    state.borrow_mut(),
                    &mut stack,
                    Item::List {
                        start: list.start,
                        bullets: list.bullets,
                    },
                    source,
                )
            }
            pulldown_cmark::TagEnd::BlockQuote(_kind) if !metadata => {
                let scope = stack.pop()?;

                let Scope::Quote(quote) = scope else {
                    return None;
                };

                produce(
                    state.borrow_mut(),
                    &mut stack,
                    Item::Quote(quote),
                    source,
                )
            }
            pulldown_cmark::TagEnd::Image if !metadata => {
                let (url, title) = image.take()?;
                let alt = Text::new(spans.drain(..).collect());

                let state = state.borrow_mut();
                let _ = state.images.insert(url.clone());

                produce(
                    state,
                    &mut stack,
                    Item::Image { url, title, alt },
                    source,
                )
            }
            pulldown_cmark::TagEnd::CodeBlock if !metadata => {
                code_block = false;

                #[cfg(feature = "highlighter")]
                {
                    state.borrow_mut().highlighter = highlighter.take();
                }

                produce(
                    state.borrow_mut(),
                    &mut stack,
                    Item::CodeBlock {
                        language: code_language.take(),
                        code: mem::take(&mut code),
                        lines: code_lines.drain(..).collect(),
                    },
                    source,
                )
            }
            pulldown_cmark::TagEnd::MetadataBlock(_) => {
                metadata = false;
                None
            }
            pulldown_cmark::TagEnd::Table => {
                let scope = stack.pop()?;

                let Scope::Table { columns, rows, .. } = scope else {
                    return None;
                };

                produce(
                    state.borrow_mut(),
                    &mut stack,
                    Item::Table { columns, rows },
                    source,
                )
            }
            pulldown_cmark::TagEnd::TableHead => {
                strong = false;
                None
            }
            pulldown_cmark::TagEnd::TableCell => {
                if !spans.is_empty() {
                    let _ = produce(
                        state.borrow_mut(),
                        &mut stack,
                        Item::Paragraph(Text::new(spans.drain(..).collect())),
                        source,
                    );
                }

                let Scope::Table {
                    alignment,
                    columns,
                    rows,
                    current,
                } = stack.last_mut()?
                else {
                    return None;
                };

                if columns.len() < alignment.len() {
                    columns.push(Column {
                        header: std::mem::take(current),
                        alignment: alignment[columns.len()],
                    });
                } else {
                    rows.last_mut()
                        .expect("table row")
                        .cells
                        .push(std::mem::take(current));
                }

                None
            }
            _ => None,
        },
        pulldown_cmark::Event::Text(text) if !metadata => {
            if code_block {
                code.push_str(&text);

                #[cfg(feature = "highlighter")]
                if let Some(highlighter) = &mut highlighter {
                    for line in text.lines() {
                        code_lines.push(Text::new(
                            highlighter.highlight_line(line).to_vec(),
                        ));
                    }
                }

                #[cfg(not(feature = "highlighter"))]
                for line in text.lines() {
                    code_lines.push(Text::new(vec![Span::Standard {
                        text: line.to_owned(),
                        strong,
                        emphasis,
                        strikethrough,
                        link: link.clone(),
                        code: false,
                    }]));
                }

                return None;
            }

            let span = Span::Standard {
                text: text.into_string(),
                strong,
                emphasis,
                strikethrough,
                link: link.clone(),
                code: false,
            };

            spans.push(span);

            None
        }
        pulldown_cmark::Event::Code(code) if !metadata => {
            let span = Span::Standard {
                text: code.into_string(),
                strong,
                emphasis,
                strikethrough,
                link: link.clone(),
                code: true,
            };

            spans.push(span);
            None
        }
        pulldown_cmark::Event::SoftBreak if !metadata => {
            spans.push(Span::Standard {
                text: String::from(" "),
                strikethrough,
                strong,
                emphasis,
                link: link.clone(),
                code: false,
            });
            None
        }
        pulldown_cmark::Event::HardBreak if !metadata => {
            spans.push(Span::Standard {
                text: String::from("\n"),
                strikethrough,
                strong,
                emphasis,
                link: link.clone(),
                code: false,
            });
            None
        }
        pulldown_cmark::Event::Rule => {
            produce(state.borrow_mut(), &mut stack, Item::Rule, source)
        }
        pulldown_cmark::Event::TaskListMarker(done) => {
            if let Some(Scope::List(list)) = stack.last_mut()
                && let Some(item) = list.bullets.last_mut()
                && let Bullet::Point { items } = item
            {
                *item = Bullet::Task {
                    items: std::mem::take(items),
                    done,
                };
            }

            None
        }
        _ => None,
    })
}

/// 控制 [`view`] 中 Markdown 渲染方式的配置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Configuration controlling Markdown rendering in [`view`].
///
/// </details>
#[derive(Debug, Clone, Copy)]
pub struct Settings {
    /// 基础文本大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The base text size.
    ///
    /// </details>
    pub text_size: Pixels,
    /// 1 级标题的文本大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text size of level 1 heading.
    ///
    /// </details>
    pub h1_size: Pixels,
    /// 2 级标题的文本大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text size of level 2 heading.
    ///
    /// </details>
    pub h2_size: Pixels,
    /// 3 级标题的文本大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text size of level 3 heading.
    ///
    /// </details>
    pub h3_size: Pixels,
    /// 4 级标题的文本大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text size of level 4 heading.
    ///
    /// </details>
    pub h4_size: Pixels,
    /// 5 级标题的文本大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text size of level 5 heading.
    ///
    /// </details>
    pub h5_size: Pixels,
    /// 6 级标题的文本大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text size of level 6 heading.
    ///
    /// </details>
    pub h6_size: Pixels,
    /// 代码块使用的文本大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text size used in code blocks.
    ///
    /// </details>
    pub code_size: Pixels,
    /// 元素之间使用的间距。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The spacing to be used between elements.
    ///
    /// </details>
    pub spacing: Pixels,
    /// Markdown 的样式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The styling of the Markdown.
    ///
    /// </details>
    pub style: Style,
}

impl Settings {
    /// 使用默认文本大小和给定的 [`Style`] 创建新的 [`Settings`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates new [`Settings`] with default text size and the given [`Style`].
    ///
    /// </details>
    pub fn with_style(style: impl Into<Style>) -> Self {
        Self::with_text_size(16, style)
    }

    /// 使用给定的、以 [`Pixels`] 表示的基础文本大小创建新的 [`Settings`]。
    ///
    /// 各级标题的大小会自动调整。具体来说，
    /// 第一级是基础大小的两倍，之后每一级
    /// 都缩小 25%。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates new [`Settings`] with the given base text size in [`Pixels`].
    ///
    /// Heading levels will be adjusted automatically. Specifically,
    /// the first level will be twice the base size, and then every level
    /// after that will be 25% smaller.
    ///
    /// </details>
    pub fn with_text_size(
        text_size: impl Into<Pixels>,
        style: impl Into<Style>,
    ) -> Self {
        let text_size = text_size.into();

        Self {
            text_size,
            h1_size: text_size * 2.0,
            h2_size: text_size * 1.75,
            h3_size: text_size * 1.5,
            h4_size: text_size * 1.25,
            h5_size: text_size,
            h6_size: text_size,
            code_size: text_size * 0.75,
            spacing: text_size * 0.875,
            style: style.into(),
        }
    }
}

impl From<&Theme> for Settings {
    fn from(theme: &Theme) -> Self {
        Self::with_style(Style::from(theme))
    }
}

impl From<Theme> for Settings {
    fn from(theme: Theme) -> Self {
        Self::with_style(Style::from(theme))
    }
}

/// 在 [`view`] 中渲染 Markdown 时的文本样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The text styling of some Markdown rendering in [`view`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// 应用于普通文本的 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Font`] to be applied to basic text.
    ///
    /// </details>
    pub font: Font,
    /// 应用于行内代码背景的 [`Highlight`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Highlight`] to be applied to the background of inline code.
    ///
    /// </details>
    pub inline_code_highlight: Highlight,
    /// 应用于行内代码背景的 [`Padding`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Padding`] to be applied to the background of inline code.
    ///
    /// </details>
    pub inline_code_padding: Padding,
    /// 应用于行内代码的 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Color`] to be applied to inline code.
    ///
    /// </details>
    pub inline_code_color: Color,
    /// 应用于行内代码的 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Font`] to be applied to inline code.
    ///
    /// </details>
    pub inline_code_font: Font,
    /// 应用于代码块的 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Font`] to be applied to code blocks.
    ///
    /// </details>
    pub code_block_font: Font,
    /// 应用于链接的 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Color`] to be applied to links.
    ///
    /// </details>
    pub link_color: Color,
}

impl Style {
    /// 根据给定的 [`theme::Palette`] 创建一个新的 [`Style`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Style`] from the given [`theme::Palette`].
    ///
    /// </details>
    pub fn from_palette(palette: theme::Palette) -> Self {
        Self {
            font: Font::default(),
            inline_code_padding: padding::left(1).right(1),
            inline_code_highlight: Highlight {
                background: color!(0x111111).into(),
                border: border::rounded(4),
            },
            inline_code_color: Color::WHITE,
            inline_code_font: Font::MONOSPACE,
            code_block_font: Font::MONOSPACE,
            link_color: palette.primary,
        }
    }
}

impl From<theme::Palette> for Style {
    fn from(palette: theme::Palette) -> Self {
        Self::from_palette(palette)
    }
}

impl From<&Theme> for Style {
    fn from(theme: &Theme) -> Self {
        Self::from_palette(theme.palette())
    }
}

impl From<Theme> for Style {
    fn from(theme: Theme) -> Self {
        Self::from_palette(theme.palette())
    }
}

/// 显示一组 Markdown 条目。
///
/// 你可以通过 [`parse`] 获得这些条目。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Display a bunch of Markdown items.
///
/// You can obtain the items with [`parse`].
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// #
/// use iced::widget::markdown;
/// use iced::Theme;
///
/// struct State {
///    markdown: Vec<markdown::Item>,
/// }
///
/// enum Message {
///     LinkClicked(markdown::Uri),
/// }
///
/// impl State {
///     pub fn new() -> Self {
///         Self {
///             markdown: markdown::parse("This is some **Markdown**!").collect(),
///         }
///     }
///
///     fn view(&self) -> Element<'_, Message> {
///         markdown::view(&self.markdown, Theme::TokyoNight)
///             .map(Message::LinkClicked)
///             .into()
///     }
///
///     fn update(state: &mut State, message: Message) {
///         match message {
///             Message::LinkClicked(url) => {
///                 println!("The following url was clicked: {url}");
///             }
///         }
///     }
/// }
/// ```
pub fn view<'a, Theme, Renderer>(
    items: impl IntoIterator<Item = &'a Item>,
    settings: impl Into<Settings>,
) -> Element<'a, Uri, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer<Font = Font> + 'a,
{
    view_with(items, settings, &DefaultViewer)
}

/// 运行 [`view`]，但使用自定义的 [`Viewer`] 把 [`Item`] 转换成
/// [`Element`]。
///
/// 如果你想自定义某些 Markdown 元素的外观，
/// 这会很有用。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Runs [`view`] but with a custom [`Viewer`] to turn an [`Item`] into
/// an [`Element`].
///
/// This is useful if you want to customize the look of certain Markdown
/// elements.
///
/// </details>
pub fn view_with<'a, Message, Theme, Renderer>(
    items: impl IntoIterator<Item = &'a Item>,
    settings: impl Into<Settings>,
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer<Font = Font> + 'a,
{
    let settings = settings.into();

    let blocks = items
        .into_iter()
        .enumerate()
        .map(|(i, item_)| item(viewer, settings, item_, i));

    Element::new(column(blocks).spacing(settings.spacing))
}

/// 使用给定的 [`Viewer`] 显示一个 [`Item`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Displays an [`Item`] using the given [`Viewer`].
///
/// </details>
pub fn item<'a, Message, Theme, Renderer>(
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
    settings: Settings,
    item: &'a Item,
    index: usize,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer<Font = Font> + 'a,
{
    match item {
        Item::Image { url, title, alt } => {
            viewer.image(settings, url, title, alt)
        }
        Item::Heading(level, text) => {
            viewer.heading(settings, level, text, index)
        }
        Item::Paragraph(text) => viewer.paragraph(settings, text),
        Item::CodeBlock {
            language,
            code,
            lines,
        } => viewer.code_block(settings, language.as_deref(), code, lines),
        Item::List {
            start: None,
            bullets,
        } => viewer.unordered_list(settings, bullets),
        Item::List {
            start: Some(start),
            bullets,
        } => viewer.ordered_list(settings, *start, bullets),
        Item::Quote(quote) => viewer.quote(settings, quote),
        Item::Rule => viewer.rule(settings),
        Item::Table { columns, rows } => viewer.table(settings, columns, rows),
    }
}

/// 使用默认外观显示标题。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Displays a heading using the default look.
///
/// </details>
pub fn heading<'a, Message, Theme, Renderer>(
    settings: Settings,
    level: &'a HeadingLevel,
    text: &'a Text,
    index: usize,
    on_link_click: impl Fn(Uri) -> Message + 'a,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer<Font = Font> + 'a,
{
    let Settings {
        h1_size,
        h2_size,
        h3_size,
        h4_size,
        h5_size,
        h6_size,
        text_size,
        ..
    } = settings;

    container(
        rich_text(text.spans(settings.style))
            .on_link_click(on_link_click)
            .size(match level {
                pulldown_cmark::HeadingLevel::H1 => h1_size,
                pulldown_cmark::HeadingLevel::H2 => h2_size,
                pulldown_cmark::HeadingLevel::H3 => h3_size,
                pulldown_cmark::HeadingLevel::H4 => h4_size,
                pulldown_cmark::HeadingLevel::H5 => h5_size,
                pulldown_cmark::HeadingLevel::H6 => h6_size,
            }),
    )
    .padding(padding::top(if index > 0 {
        text_size / 2.0
    } else {
        Pixels::ZERO
    }))
    .into()
}

/// 使用默认外观显示段落。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Displays a paragraph using the default look.
///
/// </details>
pub fn paragraph<'a, Message, Theme, Renderer>(
    settings: Settings,
    text: &Text,
    on_link_click: impl Fn(Uri) -> Message + 'a,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer<Font = Font> + 'a,
{
    rich_text(text.spans(settings.style))
        .size(settings.text_size)
        .on_link_click(on_link_click)
        .into()
}

/// 使用默认外观显示无序列表，并为每个
/// 列表项调用 [`Viewer`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Displays an unordered list using the default look and
/// calling the [`Viewer`] for each bullet point item.
///
/// </details>
pub fn unordered_list<'a, Message, Theme, Renderer>(
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
    settings: Settings,
    bullets: &'a [Bullet],
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer<Font = Font> + 'a,
{
    column(bullets.iter().map(|bullet| {
        row![
            match bullet {
                Bullet::Point { .. } => {
                    text("•").size(settings.text_size).into()
                }
                Bullet::Task { done, .. } => {
                    Element::from(
                        container(checkbox(*done).size(settings.text_size))
                            .center_y(
                                text::LineHeight::default()
                                    .to_absolute(settings.text_size),
                            ),
                    )
                }
            },
            view_with(
                bullet.items(),
                Settings {
                    spacing: settings.spacing * 0.6,
                    ..settings
                },
                viewer,
            )
        ]
        .spacing(settings.spacing)
        .into()
    }))
    .spacing(settings.spacing * 0.75)
    .padding([0.0, settings.spacing.0])
    .into()
}

/// 使用默认外观显示有序列表，并为每个
/// 编号项调用 [`Viewer`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Displays an ordered list using the default look and
/// calling the [`Viewer`] for each numbered item.
///
/// </details>
pub fn ordered_list<'a, Message, Theme, Renderer>(
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
    settings: Settings,
    start: u64,
    bullets: &'a [Bullet],
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer<Font = Font> + 'a,
{
    let digits = ((start + bullets.len() as u64).max(1) as f32)
        .log10()
        .ceil();

    column(bullets.iter().enumerate().map(|(i, bullet)| {
        row![
            text!("{}.", i as u64 + start)
                .size(settings.text_size)
                .align_x(alignment::Horizontal::Right)
                .width(settings.text_size * ((digits / 2.0).ceil() + 1.0)),
            view_with(
                bullet.items(),
                Settings {
                    spacing: settings.spacing * 0.6,
                    ..settings
                },
                viewer,
            )
        ]
        .spacing(settings.spacing)
        .into()
    }))
    .spacing(settings.spacing * 0.75)
    .into()
}

/// 使用默认外观显示代码块。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Displays a code block using the default look.
///
/// </details>
pub fn code_block<'a, Message, Theme, Renderer>(
    settings: Settings,
    lines: &'a [Text],
    on_link_click: impl Fn(Uri) -> Message + Clone + 'a,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer<Font = Font> + 'a,
{
    container(
        scrollable(
            container(column(lines.iter().map(|line| {
                rich_text(line.spans(settings.style))
                    .on_link_click(on_link_click.clone())
                    .font(settings.style.code_block_font)
                    .size(settings.code_size)
                    .into()
            })))
            .padding(settings.code_size),
        )
        .direction(scrollable::Direction::Horizontal(
            scrollable::Scrollbar::default()
                .width(settings.code_size / 2)
                .scroller_width(settings.code_size / 2),
        )),
    )
    .width(Length::Fill)
    .padding(settings.code_size / 4)
    .class(Theme::code_block())
    .into()
}

/// 使用默认外观显示引用块。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Displays a quote using the default look.
///
/// </details>
pub fn quote<'a, Message, Theme, Renderer>(
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
    settings: Settings,
    contents: &'a [Item],
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer<Font = Font> + 'a,
{
    row![
        rule::vertical(4),
        column(
            contents
                .iter()
                .enumerate()
                .map(|(i, content)| item(viewer, settings, content, i)),
        )
        .spacing(settings.spacing.0),
    ]
    .height(Length::Shrink)
    .spacing(settings.spacing.0)
    .into()
}

/// 使用默认外观显示分隔线。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Displays a rule using the default look.
///
/// </details>
pub fn rule<'a, Message, Theme, Renderer>()
-> Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer<Font = Font> + 'a,
{
    rule::horizontal(2).into()
}

/// 使用默认外观显示表格。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Displays a table using the default look.
///
/// </details>
pub fn table<'a, Message, Theme, Renderer>(
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
    settings: Settings,
    columns: &'a [Column],
    rows: &'a [Row],
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer<Font = Font> + 'a,
{
    use crate::table;

    let table = table(
        columns.iter().enumerate().map(move |(i, column)| {
            table::column(
                items(viewer, settings, &column.header),
                move |row: &Row| {
                    if let Some(cells) = row.cells.get(i) {
                        items(viewer, settings, cells)
                    } else {
                        text("").into()
                    }
                },
            )
            .align_x(match column.alignment {
                pulldown_cmark::Alignment::None
                | pulldown_cmark::Alignment::Left => {
                    alignment::Horizontal::Left
                }
                pulldown_cmark::Alignment::Center => {
                    alignment::Horizontal::Center
                }
                pulldown_cmark::Alignment::Right => {
                    alignment::Horizontal::Right
                }
            })
        }),
        rows,
    )
    .padding_x(settings.spacing.0)
    .padding_y(settings.spacing.0 / 2.0)
    .separator_x(0);

    scrollable(table)
        .direction(scrollable::Direction::Horizontal(
            scrollable::Scrollbar::default(),
        ))
        .spacing(settings.spacing.0 / 2.0)
        .into()
}

/// 使用默认外观显示一列条目。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Displays a column of items with the default look.
///
/// </details>
pub fn items<'a, Message, Theme, Renderer>(
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
    settings: Settings,
    items: &'a [Item],
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer<Font = Font> + 'a,
{
    column(
        items
            .iter()
            .enumerate()
            .map(|(i, content)| item(viewer, settings, content, i)),
    )
    .spacing(settings.spacing.0)
    .into()
}

/// 用来显示 Markdown [`Item`] 的视图策略。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A view strategy to display a Markdown [`Item`].
///
/// </details>
pub trait Viewer<'a, Message, Theme = crate::Theme, Renderer = crate::Renderer>
where
    Self: Sized + 'a,
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer<Font = Font> + 'a,
{
    /// 当带有给定 [`Uri`] 的链接被点击时产生一条消息。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Produces a message when a link is clicked with the given [`Uri`].
    ///
    /// </details>
    fn on_link_click(url: Uri) -> Message;

    /// 显示图像。
    ///
    /// 默认情况下，它会显示一个带有图像标题的容器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Displays an image.
    ///
    /// By default, it will show a container with the image title.
    ///
    /// </details>
    fn image(
        &self,
        settings: Settings,
        url: &'a Uri,
        title: &'a str,
        alt: &Text,
    ) -> Element<'a, Message, Theme, Renderer> {
        let _url = url;
        let _title = title;

        container(
            rich_text(alt.spans(settings.style))
                .on_link_click(Self::on_link_click),
        )
        .padding(settings.spacing.0)
        .class(Theme::code_block())
        .into()
    }

    /// 显示标题。
    ///
    /// 默认情况下，它调用 [`heading`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Displays a heading.
    ///
    /// By default, it calls [`heading`].
    ///
    /// </details>
    fn heading(
        &self,
        settings: Settings,
        level: &'a HeadingLevel,
        text: &'a Text,
        index: usize,
    ) -> Element<'a, Message, Theme, Renderer> {
        heading(settings, level, text, index, Self::on_link_click)
    }

    /// 显示段落。
    ///
    /// 默认情况下，它调用 [`paragraph`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Displays a paragraph.
    ///
    /// By default, it calls [`paragraph`].
    ///
    /// </details>
    fn paragraph(
        &self,
        settings: Settings,
        text: &Text,
    ) -> Element<'a, Message, Theme, Renderer> {
        paragraph(settings, text, Self::on_link_click)
    }

    /// 显示代码块。
    ///
    /// 默认情况下，它调用 [`code_block`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Displays a code block.
    ///
    /// By default, it calls [`code_block`].
    ///
    /// </details>
    fn code_block(
        &self,
        settings: Settings,
        language: Option<&'a str>,
        code: &'a str,
        lines: &'a [Text],
    ) -> Element<'a, Message, Theme, Renderer> {
        let _language = language;
        let _code = code;

        code_block(settings, lines, Self::on_link_click)
    }

    /// 显示无序列表。
    ///
    /// 默认情况下，它调用 [`unordered_list`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Displays an unordered list.
    ///
    /// By default, it calls [`unordered_list`].
    ///
    /// </details>
    fn unordered_list(
        &self,
        settings: Settings,
        bullets: &'a [Bullet],
    ) -> Element<'a, Message, Theme, Renderer> {
        unordered_list(self, settings, bullets)
    }

    /// 显示有序列表。
    ///
    /// 默认情况下，它调用 [`ordered_list`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Displays an ordered list.
    ///
    /// By default, it calls [`ordered_list`].
    ///
    /// </details>
    fn ordered_list(
        &self,
        settings: Settings,
        start: u64,
        bullets: &'a [Bullet],
    ) -> Element<'a, Message, Theme, Renderer> {
        ordered_list(self, settings, start, bullets)
    }

    /// 显示引用块。
    ///
    /// 默认情况下，它调用 [`quote`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Displays a quote.
    ///
    /// By default, it calls [`quote`].
    ///
    /// </details>
    fn quote(
        &self,
        settings: Settings,
        contents: &'a [Item],
    ) -> Element<'a, Message, Theme, Renderer> {
        quote(self, settings, contents)
    }

    /// 显示分隔线。
    ///
    /// 默认情况下，它调用 [`rule`](self::rule())。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Displays a rule.
    ///
    /// By default, it calls [`rule`](self::rule()).
    ///
    /// </details>
    fn rule(
        &self,
        _settings: Settings,
    ) -> Element<'a, Message, Theme, Renderer> {
        rule()
    }

    /// 显示表格。
    ///
    /// 默认情况下，它调用 [`table`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Displays a table.
    ///
    /// By default, it calls [`table`].
    ///
    /// </details>
    fn table(
        &self,
        settings: Settings,
        columns: &'a [Column],
        rows: &'a [Row],
    ) -> Element<'a, Message, Theme, Renderer> {
        table(self, settings, columns, rows)
    }
}

#[derive(Debug, Clone, Copy)]
struct DefaultViewer;

impl<'a, Theme, Renderer> Viewer<'a, Uri, Theme, Renderer> for DefaultViewer
where
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer<Font = Font> + 'a,
{
    fn on_link_click(url: Uri) -> Uri {
        url
    }
}

/// Markdown 条目的主题样式目录。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The theme catalog of Markdown items.
///
/// </details>
pub trait Catalog:
    container::Catalog
    + scrollable::Catalog
    + text::Catalog
    + crate::rule::Catalog
    + checkbox::Catalog
    + crate::table::Catalog
{
    /// Markdown 代码块的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The styling class of a Markdown code block.
    ///
    /// </details>
    fn code_block<'a>() -> <Self as container::Catalog>::Class<'a>;
}

impl Catalog for Theme {
    fn code_block<'a>() -> <Self as container::Catalog>::Class<'a> {
        Box::new(container::dark)
    }
}
