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
//! # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::{Renderer, Widget}; pub use iced_widget::core::*; }
//! #
//! use iced::widget::markdown;
//! use iced::{Theme, Widget};
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
//!     fn view(&self) -> impl Widget<Message> {
//!         markdown::view(
//!             &self.markdown,
//!             markdown::Settings::default(),
//!             Theme::TokyoNight,
//!         )
//!             .map(Message::LinkClicked)
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
use crate::core;
use crate::core::alignment;
use crate::core::border;
use crate::core::font::{self, Font};
use crate::core::padding;
use crate::core::text::LineHeight;
use crate::core::theme;
use crate::core::{Code, Color, Length, Padding, Pixels, Theme};
use crate::{
    Widget, center_x, checkbox, column, container, rich_text, row, rule, scrollable, span, text,
};

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::mem;
use std::ops::Range;
use std::rc::Rc;
use std::sync::Arc;

pub use core::text::{Highlight, Highlighter};
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
    /// 到目前为止累积的原始 Markdown，由所有的
    /// 条目和区段共享。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The raw Markdown accumulated so far, shared between all the
    /// items and sections.
    ///
    /// </details>
    raw: String,
    /// 解析后的输出。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The parsed output.
    ///
    /// </details>
    items: Vec<Item>,
    /// 当该条目是最后一个条目时，需要重新解析的源文本的
    /// 起点：条目的起点；如果它是列表，则是其最后一个
    /// 列表项的起点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The start of the source that is re-parsed when the item is the
    /// last one: the start of the item, or the start of its last
    /// bullet, when it is a list.
    ///
    /// </details>
    starts: Vec<usize>,
    /// 条目源文本真正的起点；对于列表来说是整个列表的
    /// 起点，而不像 `starts` 那样是其最后一个列表项的
    /// 起点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The true start of the item's source; for a list, the start of
    /// the list, unlike `starts`, which is the start of its last
    /// bullet.
    ///
    /// </details>
    base: Vec<usize>,
    /// 下一次推入时将被重新解析的源文本的
    /// 起点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The start of the source that will be re-parsed on the next
    /// push.
    ///
    /// </details>
    window: usize,
    /// 上一次推入时是否存在尚未确定的元数据块；
    /// 如果存在，重新解析会从该块的起点开始，
    /// 这样一旦该块闭合，它就会被解析器吞掉
    /// （或者被重新解析为它最终所是的分隔线）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether a not-yet-settled metadata block was live on the last
    /// push; when it was, the re-parse starts at the start of the
    /// block, so that the block is swallowed by the parser once it is
    /// closed (or re-parsed as the rule it turned out to be).
    ///
    /// </details>
    pending_block: bool,
    /// 尚未确定的元数据块的起点（如果有）；
    /// 重新解析从这里开始，这样整个块（例如一条暂定的分隔线
    /// 及其内容）会作为一个整体被重新解析。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The start of the not-yet-settled metadata block, if any; the
    /// re-parse starts there, so that the block (a tentative rule and
    /// its content, for instance) is re-parsed as a whole.
    ///
    /// </details>
    pending_block_start: Option<usize>,
    incomplete: HashMap<usize, Section>,
    state: State,
}

#[derive(Debug)]
struct Section {
    /// 当某个引用变得可用或发生变化时，需要重新解析的
    /// 源文本的起点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The start of the source to re-parse when a reference becomes
    /// available or changes.
    ///
    /// </details>
    start: usize,
    /// 需要重新解析的源文本的终点；如果该条目仍然是
    /// 最后一个，则为 `None`，这样源文本还可以继续增长。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The end of the source to re-parse; `None` if the item is still
    /// the last one, so that the source can still grow.
    ///
    /// </details>
    end: Option<usize>,
    broken_links: HashSet<String>,
    /// 该条目上一次被重新解析时已解析的引用，
    /// 以及它们当时的目标地址。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The references that were resolved when the item was last
    /// re-parsed, along with their destination at that time.
    ///
    /// </details>
    references: HashMap<String, String>,
    /// 该条目在源文本重新解析结果中的索引。
    ///
    /// 一个源文本区域可以产生不止一个条目（例如一张图像和
    /// 它所属的段落）；这是本区段所指向的那个条目的
    /// 索引。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The index of the item in the re-parse of the source.
    ///
    /// A source region can produce more than one item (an image and
    /// the paragraph it belongs to, for instance); this is the index
    /// of the item that this section refers to.
    ///
    /// </details>
    item: usize,
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

    /// 向 [`Content`] 推入更多 Markdown；增量解析！
    ///
    /// 当你有很长的 Markdown 流时，这尤其有用；例如
    /// 大文件或可能很长的回复。
    ///
    /// 每次调用只会重新解析最后一个条目；并且，当最后一个
    /// 条目是列表时，只会重新解析它的最后一个列表项，这样向
    /// 长列表推入新的列表项时开销依然很低。
    ///
    /// 随着流的增长，结果会收敛到一次性解析整个流
    /// 所得到的结果。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Pushes more Markdown into the [`Content`]; parsing incrementally!
    ///
    /// This is specially useful when you have long streams of Markdown; like
    /// big files or potentially long replies.
    ///
    /// Only the last item is re-parsed on every call; and, when the last
    /// item is a list, only its last bullet is re-parsed, so that pushing
    /// new items to a long list stays cheap.
    ///
    /// The result converges to the one obtained by parsing the whole
    /// stream at once, as the stream grows.
    ///
    /// </details>
    pub fn push_str(&mut self, markdown: &str) {
        if markdown.is_empty() {
            return;
        }

        self.raw.push_str(markdown);

        // The text to re-parse: from the start of the source of the
        // last item (or its last bullet, when it is a list) to the
        // end. Unless a not-yet-settled metadata block is live (or
        // was live on the last push), in which case the re-parse
        // starts at the start of the block: it is swallowed by the
        // parser once it is closed, and its first line (a tentative
        // rule) must be re-parsed with it.
        // 需要重新解析的文本：从最后一个条目（如果它是列表，
        // 则是其最后一个列表项）的源文本起点开始，直到
        // 末尾。除非存在尚未确定的元数据块（或者
        // 上一次推入时存在），这种情况下重新解析
        // 从该块的起点开始：一旦该块闭合，它就会被
        // 解析器吞掉，而它的第一行（一条暂定的
        // 分隔线）必须和它一起被重新解析。
        let block_live = self
            .pending_block_start
            .map(|start| Self::metadata_block_live(&self.raw[start..]))
            .unwrap_or(false);
        // A not-yet-settled metadata block that was live on the last
        // push is now settled: the references registered while it was
        // open are swallowed by it, so they must be dropped.
        // 上一次推入时存在的尚未确定的元数据块现在
        // 已经确定：在它打开期间注册的引用
        // 被它吞掉了，因此必须丢弃这些引用。
        let block_closed = self.pending_block && !block_live;
        let mut input_start = if self.pending_block || block_live {
            self.pending_block_start.unwrap_or(self.window)
        } else {
            self.window
        };

        // When the last item is a list and the previous item is a list
        // or a quote, the last list may be a lazy continuation of the
        // previous item's last bullet once more of it is streamed: an
        // empty `2. two\n-` bullet list, for instance, becomes the
        // `--` continuation of the numbered item as the second dash
        // arrives, and a `-` outside a quoted list likewise becomes a
        // continuation of the quoted bullet. Re-parse from the
        // previous item's start so that this collapse is re-evaluated
        // (and the re-parse yields the merged item, not a trailing
        // paragraph or list).
        // 当最后一个条目是列表，而前一个条目是列表
        // 或引用块时，随着更多内容流入，最后的列表可能成为
        // 前一个条目最后一个列表项的惰性延续：
        // 例如，`2. two\n-` 中的空无序列表，在第二个短横线
        // 到来时会变成编号项的 `--` 延续；
        // 同样，引用块中的列表之外的 `-` 也会变成
        // 被引用列表项的延续。因此从前一个条目的
        // 起点开始重新解析，以便重新评估这种合并
        // （并让重新解析产生合并后的条目，而不是一个尾随的
        // 段落或列表）。
        if !self.pending_block
            && !block_live
            && let [.., prev, last] = self.items.as_slice()
            && matches!(last, Item::List { .. })
            && matches!(prev, Item::List { .. } | Item::Quote(_))
        {
            input_start = input_start.min(self.base[self.base.len() - 2]);
        }
        let tail = &self.raw[input_start..];
        let trimmed = tail.trim_end();
        let mut input = if trimmed.ends_with('|') {
            trimmed.trim_end_matches('|')
        } else {
            tail
        };

        // Pop the last item and the items whose source falls within
        // the text that will be re-parsed (an image and the paragraph
        // it belongs to, for instance); they will be re-parsed as
        // well.
        // 弹出最后一个条目，以及源文本落在即将被重新解析的
        // 文本范围内的条目（例如一张图像和它所属的
        // 段落）；它们也会被重新
        // 解析。
        let last = self.items.pop();
        let _ = self.starts.pop();
        // The true start of the source of the last item, if any; it
        // is the start of the merged list, when the last item is a
        // list.
        // 最后一个条目源文本真正的起点（如果有）；
        // 当最后一个条目是列表时，它是合并后列表的
        // 起点。
        let old_last_base = self.base.pop();
        while self
            .starts
            .last()
            .is_some_and(|start| *start >= input_start)
        {
            let _ = self.items.pop();
            let _ = self.starts.pop();
            let _ = self.base.pop();
        }

        // Re-parse the last item and the new text
        // 重新解析最后一个条目和新的文本
        let mut items: Vec<_> = parse_with(&mut self.state, input, input_start).collect();

        // We only re-parse the last bullet of a list, so merge the
        // re-parsed list into the old one, keeping the bullets that
        // were already parsed. This only applies when the re-parse
        // actually started after the start of the list (at its last
        // bullet); when it re-parsed the whole list, the re-parsed
        // list already contains all the bullets, and merging would
        // duplicate them.
        // 我们只重新解析列表的最后一个列表项，因此要把
        // 重新解析得到的列表合并进旧列表，保留已经解析过的
        // 列表项。这只适用于重新解析确实是从列表起点之后
        // （即其最后一个列表项处）开始的情况；
        // 如果重新解析的是整个列表，重新解析得到的
        // 列表已经包含所有列表项，合并反而会
        // 让它们重复。
        let last_is_list = matches!(last.as_ref(), Some(Item::List { .. }));
        let mut merged = false;
        if let Some(Item::List { start, bullets, .. }) = last
            && let Some((first_item, _, _)) = items.first_mut()
            && let Item::List { bullets: new, .. } = first_item
            && old_last_base.is_some_and(|base| input_start > base)
        {
            // The last bullet of the old list was re-parsed
            // 旧列表的最后一个列表项已被重新解析
            let mut bullets = bullets;
            let _ = bullets.pop();
            bullets.extend(mem::take(new));
            *first_item = Item::List { start, bullets };
            merged = true;
        } else if last_is_list {
            // The re-parse of the last bullet no longer produces a
            // list (the bullet grew into a rule or a heading, for
            // instance), so it cannot be merged into the old list:
            // re-parse from the start of the whole list, so its
            // source is re-parsed in full.
            //
            // The re-parse must not start after a not-yet-settled
            // metadata block opener (kept in `input_start`), or the
            // block would be dropped; use the earliest of the two.
            // 重新解析最后一个列表项的结果不再是
            // 列表（例如该列表项变成了分隔线或
            // 标题），因此无法合并进旧列表：
            // 改为从整个列表的起点开始重新解析，
            // 让它的源文本被完整地重新解析。
            //
            // 重新解析不能从尚未确定的元数据块开头标记
            // （保存在 `input_start` 中）之后开始，否则
            // 该块会被丢掉；因此取两者中较早的位置。
            let base = old_last_base.expect("a list has a base").min(input_start);
            let tail = &self.raw[base..];
            let trimmed = tail.trim_end();
            input = if trimmed.ends_with('|') {
                trimmed.trim_end_matches('|')
            } else {
                tail
            };
            input_start = base;
            items = parse_with(&mut self.state, input, input_start).collect();
        } else if let Some((Item::List { .. }, 0, _)) = items.first()
            && let Some(Item::List { .. }) = self.items.last()
            && let Some(base) = self.base.last().copied()
        {
            // The re-parse produced a list that starts where the old
            // last item (a lone paragraph, for instance) used to be,
            // right after a previous list: the paragraph grew into a
            // list item that continues the previous list. Re-parse
            // from the start of the previous list, so that the list
            // is not split in two; the parser decides whether the
            // two regions are one list.
            // 重新解析产生了一个列表，它的起点正是旧的
            // 最后一个条目（例如一个单独的段落）原来所在的位置，
            // 并且紧跟在前一个列表之后：该段落变成了
            // 延续前一个列表的列表项。因此从前一个列表的
            // 起点开始重新解析，以免列表被
            // 拆成两个；由解析器来判断这
            // 两个区域是否属于同一个列表。
            let tail = &self.raw[base..];
            let trimmed = tail.trim_end();
            input = if trimmed.ends_with('|') {
                trimmed.trim_end_matches('|')
            } else {
                tail
            };

            let reparsed: Vec<_> = parse_with(&mut self.state, input, base).collect();

            if let Some((Item::List { .. }, _, _)) = reparsed.first() {
                input_start = base;
                items = reparsed;
                // The previous list is covered by the re-parse
                // 前一个列表已被重新解析覆盖
                let _ = self.items.pop();
                let _ = self.starts.pop();
                let _ = self.base.pop();
            }
        }

        // The start of the most recent `---` or `+++` rule produced
        // by the re-parse, if any; it is a tentative metadata block
        // opener.
        // 重新解析产生的最近一条 `---` 或 `+++` 分隔线的
        // 起点（如果有）；它是一个暂定的元数据块
        // 开头标记。
        let mut newest_rule_start: Option<usize> = None;

        if items.is_empty() {
            // The new text did not produce any item (it completed a
            // reference definition or a metadata block, for
            // instance), so the last item was replaced by it; the
            // next push re-parses from the start of the new last
            // item.
            // 新的文本没有产生任何条目（例如它补全了
            // 一个引用定义或一个元数据块），
            // 因此最后一个条目被它取代了；
            // 下一次推入会从新的最后一个条目的起点
            // 开始重新解析。
            self.window = self.starts.last().copied().unwrap_or(input_start);
        } else {
            // Remember the start of the source of each re-parsed
            // item.
            // 记住每个被重新解析的条目的源文本
            // 起点。
            let starts: Vec<usize> = items
                .iter()
                .map(|(_, start, _)| input_start + *start)
                .collect();

            for (i, (item, _start, broken_links)) in items.into_iter().enumerate() {
                let start = starts[i];
                // The merged list is anchored at the start of the
                // whole list, unlike `start`, which is the start of
                // its last bullet.
                // 合并后的列表锚定在整个列表的
                // 起点，而不像 `start` 那样是
                // 其最后一个列表项的起点。
                let base = if i == 0 && merged {
                    old_last_base.expect("a merged list has a base")
                } else {
                    start
                };

                if !broken_links.is_empty() {
                    // The index of the item once it is pushed
                    // 该条目被推入后的索引
                    let index = self.items.len();

                    // The next item can cover this one (a paragraph
                    // and the image it contains, for instance); in
                    // that case, the source to re-parse spans both.
                    // 下一个条目可能覆盖当前条目（例如一个段落
                    // 和它包含的图像）；在这种情况下，
                    // 需要重新解析的源文本会横跨两者。
                    let covers = starts.get(i + 1).is_some_and(|next| *next <= start);

                    // The source to re-parse starts at the start of
                    // the covered group, if any, and ends where the
                    // item after the group starts; it grows with the
                    // source while the group is the last one.
                    // 需要重新解析的源文本从被覆盖的那组条目的起点
                    // （如果有）开始，到该组之后的条目的起点
                    // 结束；当该组是最后一组时，它会随着
                    // 源文本一起增长。
                    let (section_start, end) = if covers {
                        (starts[i + 1], starts.get(i + 2).copied())
                    } else {
                        (base, starts.get(i + 1).copied())
                    };

                    // A bullet that was not re-parsed can have broken
                    // links of its own, so they need to be kept
                    // 没有被重新解析的列表项可能有它自己的
                    // 断开的链接，因此需要保留它们
                    match self.incomplete.entry(index) {
                        Entry::Occupied(mut entry) => {
                            let section = entry.get_mut();
                            section.broken_links.extend(broken_links);
                            // The geometry can change (the item was
                            // not covered when the section was
                            // created, and its paragraph is now
                            // re-parsed as well)
                            // 几何范围可能发生变化（创建该区段时
                            // 这个条目还没有被覆盖，
                            // 而现在它所在的段落也会被
                            // 重新解析）
                            section.start = section_start;
                            section.end = end;
                        }
                        Entry::Vacant(entry) => {
                            // The re-parse of the section's source
                            // produces the items that fall within
                            // the section's range (an image and the
                            // paragraph it belongs to, for instance);
                            // remember the index of the one this
                            // section refers to.
                            // 重新解析该区段的源文本，会产生
                            // 落在该区段范围内的条目
                            // （例如一张图像和它所属的
                            // 段落）；
                            // 记住本区段所指向的那个条目的
                            // 索引。
                            let item = starts
                                .iter()
                                .take(i)
                                .copied()
                                .filter(|start| *start >= section_start)
                                .count();
                            let _ = entry.insert(Section {
                                start: section_start,
                                end,
                                broken_links,
                                references: HashMap::new(),
                                item,
                            });
                        }
                    }
                }

                if matches!(item, Item::Rule) && Self::metadata_delimiter(&self.raw, start) {
                    // A `---` or `+++` rule is a tentative metadata
                    // block opener; remember where it starts so that
                    // the block is re-parsed as a whole when it is
                    // closed.
                    // `---` 或 `+++` 分隔线是一个暂定的元数据
                    // 块开头标记；记住它的起点，这样
                    // 当该块闭合时，它会作为一个整体被
                    // 重新解析。
                    newest_rule_start = Some(start);
                }

                self.items.push(item);
                self.starts.push(start);
                self.base.push(base);
            }

            self.window = input_start + self.state.window.unwrap_or(input.len());
        }

        // Remember the tentative metadata block opener, if any, so
        // that the block is re-parsed as a whole as it grows, and
        // swallowed by the parser once it is closed.
        // 记住暂定的元数据块开头标记（如果有），这样
        // 该块在增长时会作为一个整体被重新解析，
        // 并在闭合后被解析器吞掉。
        self.pending_block_start = newest_rule_start;
        self.pending_block = newest_rule_start
            .map(|start| Self::metadata_block_live(&self.raw[start..]))
            .unwrap_or(false);

        // A metadata block that was open on the last push is now
        // settled: the references registered while it was open are
        // swallowed by it, so recompute the references from the whole
        // source, as the one-shot parse does.
        // 上一次推入时打开的元数据块现在已经
        // 确定：在它打开期间注册的引用
        // 被它吞掉了，因此要像一次性解析那样，
        // 根据整个源文本重新计算引用。
        if block_closed {
            self.recompute_references();
        }

        // The sections whose item is not the last one anymore have
        // a fixed source range
        // 对应条目不再是最后一个条目的区段，
        // 其源文本范围已经固定
        self.fix_section_ends();

        // The sections whose broken links became resolvable, or
        // whose references changed, are re-parsed
        // 断开的链接变得可以解析的区段，或者
        // 引用发生变化的区段，会被重新解析
        self.resolve_sections();

        // The images are those present in the items; recompute them,
        // as an image parsed while a metadata block was still open
        // can be swallowed by it once the block is closed.
        // 图像就是各条目中存在的那些图像；需要重新计算它们，
        // 因为在元数据块仍然打开时解析出的图像，
        // 可能在该块闭合后被它吞掉。
        self.state.images = self
            .items
            .iter()
            .filter_map(|item| match item {
                Item::Image { url, .. } => Some(url.clone()),
                _ => None,
            })
            .collect();
    }

    /// 如果源文本以一个尚未确定的元数据块开头，
    /// 返回 `true`：第一行是完整的 `---` 或
    /// `+++` 定界符，第二行不是空行
    /// （否则第一行就是一条分隔线），并且该块
    /// 尚未闭合。
    ///
    /// 当这样的块存在时，它的第一行是一条暂定的
    /// 分隔线，一旦该块闭合就会被解析器吞掉，因此
    /// 重新解析必须覆盖整个块。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns `true` if the source starts with a metadata block that
    /// is not settled yet: the first line is a complete `---` or
    /// `+++` delimiter, the second line is not a blank one
    /// (otherwise the first line is a rule), and the block has not
    /// been closed.
    ///
    /// While such a block is live, its first line is a tentative
    /// rule that the parser swallows once the block is closed, so
    /// the re-parse has to cover the block as a whole.
    ///
    /// </details>
    fn metadata_block_live(source: &str) -> bool {
        // The first line, which must be complete
        // 第一行，它必须是完整的
        let (first, rest) = match source.find('\n') {
            Some(end) => (&source[..end], &source[end + 1..]),
            None => return false,
        };

        // The delimiter line
        // 定界符行
        let first = first.trim_end();
        if first != "---" && first != "+++" {
            return false;
        }

        // The second line: a blank one makes the first line a rule,
        // not a metadata block; an incomplete one could still be
        // the start of a block
        // 第二行：如果是空行，第一行就是一条分隔线，
        // 而不是元数据块；如果是不完整的行，它仍然可能是
        // 一个块的开头
        let Some(second_end) = rest.find('\n') else {
            return true;
        };
        let second = &rest[..second_end];
        if second.trim().is_empty() {
            return false;
        }

        // The block is closed by a `---` or `...` line, or a `+++`
        // line, when it is delimited by `+++`
        // 该块由一行 `---` 或 `...` 闭合；如果它是用 `+++`
        // 定界的，则由一行 `+++` 闭合
        let closed = rest.lines().any(|line| {
            let line = line.trim_end();
            if first == "+++" {
                line == "+++"
            } else {
                line == "---" || line == "..."
            }
        });

        !closed
    }

    /// 如果从 `start` 开始的那一行是完整的
    /// `---` 或 `+++` 元数据块定界符，返回 `true`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns `true` if the line starting at `start` is a complete
    /// `---` or `+++` metadata block delimiter.
    ///
    /// </details>
    fn metadata_delimiter(source: &str, start: usize) -> bool {
        let line = &source[start..];
        let end = line.find('\n').unwrap_or(line.len());
        let line = line[..end].trim_end();
        line == "---" || line == "+++"
    }

    /// 重新解析整个源文本，并用一次性解析得到的
    /// 引用定义替换现有的引用定义。
    ///
    /// 在元数据块仍然打开时注册的引用，
    /// 会在该块确定后被它吞掉，因此它不应该
    /// 再用于解析链接；重新解析整个源文本会丢弃它，
    /// 就像一次性解析那样。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Re-parses the whole source and replaces the reference
    /// definitions with those of the one-shot parse.
    ///
    /// A reference registered while a metadata block was still open
    /// is swallowed by it once the block is settled, so it must not
    /// resolve links any more; re-parsing the whole source drops it,
    /// like the one-shot parse does.
    ///
    /// </details>
    fn recompute_references(&mut self) {
        let parser = pulldown_cmark::Parser::new_ext(&self.raw, options());
        let definitions = parser.reference_definitions();

        self.state.references.clear();
        self.state.references_staged.clear();

        absorb_references(
            &self.raw,
            definitions,
            &mut self.state.references,
            &mut self.state.references_staged,
        );
    }

    /// 结束那些对应条目不再是最后一个条目的区段：
    /// 它们的源文本范围现在已经固定，终点就是下一个
    /// 条目的起点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Ends the sections whose item is not the last one anymore:
    /// their source range is now fixed, and it ends where the next
    /// item starts.
    ///
    /// </details>
    fn fix_section_ends(&mut self) {
        if self.incomplete.is_empty() {
            return;
        }

        for (index, section) in self.incomplete.iter_mut() {
            if section.end.is_none() && *index + 1 < self.items.len() {
                // The next item can cover the section's item (a
                // paragraph and the image it contains), so the end
                // is the first start that is strictly after the
                // section's start
                // 下一个条目可能覆盖该区段的条目（一个
                // 段落和它包含的图像），因此终点
                // 是严格位于该区段起点之后的
                // 第一个起点
                section.end = self.starts[*index + 1..]
                    .iter()
                    .copied()
                    .find(|end| *end > section.start);
            }
        }
    }

    /// 重新解析那些断开的链接变得可以解析、
    /// 或者引用的目标地址发生变化的区段；
    /// 不再有任何内容需要监视的区段会被丢弃。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Re-parses the sections whose broken links became resolvable,
    /// or whose references changed destination; the sections that
    /// are left with nothing to watch are dropped.
    ///
    /// </details>
    fn resolve_sections(&mut self) {
        if self.incomplete.is_empty() {
            return;
        }

        self.incomplete.retain(|index, section| {
            if self.items.len() <= *index {
                // The section's item is gone
                // 该区段的条目已经不存在了
                return false;
            }

            // A link becomes resolvable...
            // 某个链接变得可以解析……
            let mut newly_resolved = Vec::new();
            section.broken_links.retain(|link| {
                if self.state.references.contains_key(link) {
                    newly_resolved.push(link.clone());
                    false
                } else {
                    true
                }
            });

            // ...or the destination of a resolved reference changes,
            // or the reference is dropped (its definition swallowed
            // by a metadata block, for instance)
            // ……或者某个已解析引用的目标地址发生了变化，
            // 或者该引用被丢弃（例如它的定义被
            // 元数据块吞掉）
            let needs_reparse = !newly_resolved.is_empty()
                || section.references.iter().any(|(link, dest)| {
                    match self.state.references.get(link) {
                        Some(new_dest) => new_dest != dest,
                        None => true,
                    }
                });

            if needs_reparse {
                let mut state = State {
                    window: None,
                    references: self.state.references.clone(),
                    references_staged: HashSet::new(),
                    images: HashSet::new(),
                    #[cfg(feature = "highlighter")]
                    parsers: HashMap::new(),
                };

                let end = section.end.unwrap_or(self.raw.len());
                let source = &self.raw[section.start..end];

                if let Some((item, _start, broken_links)) =
                    parse_with(&mut state, source, section.start).nth(section.item)
                {
                    self.items[*index] = item;

                    // Track the references that were resolved by the
                    // re-parse, so that a later change of their
                    // destination triggers a new re-parse
                    // 跟踪重新解析时已解析的引用，
                    // 这样它们的目标地址之后发生变化时
                    // 会触发新的重新解析
                    for link in newly_resolved {
                        if let Some(dest) = self.state.references.get(&link)
                            && !broken_links.contains(&link)
                        {
                            let _ = section.references.insert(link, dest.to_owned());
                        }
                    }

                    section.broken_links = broken_links;
                    section
                        .references
                        .retain(|link, _| !section.broken_links.contains(link));

                    for (link, dest) in &mut section.references {
                        if let Some(new_dest) = self.state.references.get(link) {
                            *dest = new_dest.clone();
                        }
                    }
                }

                self.state.images.extend(state.images);
            }

            // The section is kept while something is left to watch
            // 只要还有内容需要监视，就保留该区段
            !section.broken_links.is_empty() || !section.references.is_empty()
        });
    }

    /// 返回可供渲染的 Markdown 条目。
    ///
    /// 你可以使用 [`view`] 把它们转换为 [`Widget`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the Markdown items, ready to be rendered.
    ///
    /// You can use [`view`] to turn them into a [`Widget`].
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

    /// 返回原始 Markdown。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the raw Markdown.
    ///
    /// </details>
    pub fn raw(&self) -> &str {
        &self.raw
    }
}

/// 按 [`Item::Heading`] 对给定的 Markdown [`Item`] 进行分组。
///
/// 返回的迭代器为每个分组产生一个 `(Option<&Item>, &[Item])`
/// 二元组，不会克隆任何 [`Item`]：
///
/// * 第一个元素是开启该分组的标题（如果有）。对于
///   出现在第一个标题之前的那组条目（如果存在），
///   它是 [`None`]；
/// * 第二个元素是跟在该标题之后的条目切片，直到
///   下一个标题为止（不包含下一个标题）。
///
/// 给定切片中的每个条目都恰好被产生一次：标题
/// 作为它所开启的分组的第一个元素返回，其他每个
/// 条目则属于它之前最近一个标题之后的切片。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Groups the given Markdown [`Item`]s by [`Item::Heading`].
///
/// The returned iterator yields a `(Option<&Item>, &[Item])` pair for each
/// group, without cloning any [`Item`]:
///
/// * The first element is the heading that starts the group, if any. It is
///   [`None`] for the group of items that appears before the first heading,
///   if there is any;
/// * The second element is the slice of items that follow the heading, up to
///   (but not including) the next one.
///
/// Every item in the given slice is yielded exactly once: a heading is
/// returned as the first element of the group it starts, and every other
/// item is part of the slice that follows the last heading before it.
///
/// </details>
///
/// # 示例（Example）
/// ```
/// use iced_widget::markdown;
///
/// let items: Vec<_> = markdown::parse("# Title\n\nHello!\n\n# Subtitle\n\nMore!").collect();
///
/// let mut groups = markdown::sections(&items);
///
/// let (heading, contents) = groups.next().unwrap();
/// assert!(heading.is_some());
/// assert_eq!(contents.len(), 1);
///
/// let (heading, contents) = groups.next().unwrap();
/// assert!(heading.is_some());
/// assert_eq!(contents.len(), 1);
///
/// assert!(groups.next().is_none());
/// ```
pub fn sections<'a>(
    items: &'a [Item],
) -> impl Iterator<Item = (Option<&'a Item>, &'a [Item])> + 'a {
    struct Sections<'a> {
        /// 正在被分组的条目。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The items being grouped.
        ///
        /// </details>
        items: &'a [Item],
        /// 下一个分组第一个条目的索引。
        ///
        /// 它总是某个标题的索引，只有最开始的那个
        /// 分组例外，此时它可以指向任意条目（第一个
        /// 标题之前的内容）。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The index of the first item of the next group.
        ///
        /// This is always the index of a heading, except for the very first
        /// group, where it may point at any item (the content before the
        /// first heading).
        ///
        /// </details>
        start: usize,
    }

    impl<'a> Iterator for Sections<'a> {
        type Item = (Option<&'a Item>, &'a [Item]);

        fn next(&mut self) -> Option<Self::Item> {
            let Self { items, start } = self;

            if *start >= items.len() {
                return None;
            }

            // The heading of the current group, if any
            // 当前分组的标题（如果有）
            let heading = if matches!(items[*start], Item::Heading(..)) {
                Some(*start)
            } else {
                None
            };

            // The body of the group: the items after the heading, if any, up to
            // the next heading
            // 分组的正文：标题（如果有）之后、直到下一个标题
            // 为止的条目
            let body_start = heading.map_or(*start, |heading| heading + 1);
            let next_heading = (body_start..items.len())
                .find(|&index| matches!(items[index], Item::Heading(..)))
                .unwrap_or(items.len());

            // The next group starts at the next heading, if any
            // 下一个分组从下一个标题（如果有）开始
            *start = next_heading;

            Some((
                heading.map(|index| &items[index]),
                &items[body_start..next_heading],
            ))
        }
    }

    Sections { items, start: 0 }
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
    last_style: RefCell<Option<(Settings, String, String)>>,
    last_styled_spans: RefCell<Arc<[text::Span<'static, Uri>]>>,
}

impl Text {
    fn new(spans: Vec<Span>) -> Self {
        Self {
            spans,
            last_style: RefCell::default(),
            last_styled_spans: RefCell::default(),
        }
    }

    /// 返回可直接用于给定样式的 [`rich_text()`] 文本片段（span）。
    ///
    /// 此方法会替你进行缓存。只有当提供的 [`Settings`]
    /// 或 [`Catalog`] 发生变化时，它才会重新分配。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`rich_text()`] spans ready to be used for the given style.
    ///
    /// This method performs caching for you. It will only reallocate if the [`Settings`]
    /// or the [`Catalog`] provided changes.
    ///
    /// </details>
    pub fn spans<Theme: Catalog>(
        &self,
        settings: Settings,
        theme: &Theme,
        highlighter: &dyn text::Highlighter<Code, Theme>,
    ) -> Arc<[text::Span<'static, Uri>]> {
        let is_dirty = self.last_style.borrow().as_ref().is_none_or(
            |(last_settings, last_theme, last_highlighter)| {
                &settings != last_settings
                    || theme.id() != last_theme
                    || highlighter.id() != last_highlighter
            },
        );

        if is_dirty {
            *self.last_styled_spans.borrow_mut() = self
                .spans
                .iter()
                .map(|span| span.view(&settings, theme, highlighter))
                .collect();

            *self.last_style.borrow_mut() =
                Some((settings, theme.id().to_owned(), highlighter.id().to_owned()));
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
        inline_code: bool,
    },
    Code {
        text: String,
        code: Code,
    },
}

impl Span {
    fn view<Theme: Catalog>(
        &self,
        settings: &Settings,
        theme: &Theme,
        highlighter: &dyn text::Highlighter<Code, Theme>,
    ) -> text::Span<'static, Uri> {
        match self {
            Span::Standard {
                text,
                strikethrough,
                link,
                strong,
                emphasis,
                inline_code,
            } => {
                let span = span(text.clone()).strikethrough(*strikethrough);

                let weight = if *strong {
                    font::Weight::Bold
                } else {
                    settings.font.weight
                };

                let style = if *emphasis {
                    font::Style::Italic
                } else {
                    settings.font.style
                };

                let span = if *inline_code {
                    let code = theme.code();

                    span.font(Font {
                        weight,
                        style,
                        ..settings.inline_code_font
                    })
                    .size(settings.inline_code_size)
                    .color(code.color)
                    .background(code.highlight.background)
                    .border(code.highlight.border)
                    .padding(code.padding)
                } else {
                    span.font(Font {
                        weight,
                        style,
                        ..settings.font
                    })
                };

                if let Some(link) = link.as_ref() {
                    span.color(theme.link_color()).link(link.clone())
                } else {
                    span
                }
            }
            Span::Code { text, code } => {
                let format = highlighter.highlight(*code, theme);

                span(text.clone())
                    .color_maybe(format.color)
                    .font_maybe(format.style.map(|style| Font {
                        style,
                        ..settings.code_block_font
                    }))
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
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::{Renderer, Widget}; pub use iced_widget::core::*; }
/// #
/// use iced::widget::markdown;
/// use iced::{Theme, Widget};
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
///     fn view(&self) -> impl Widget<Message> {
///         markdown::view(
///             &self.markdown,
///             markdown::Settings::default(),
///             Theme::TokyoNight,
///         )
///             .map(Message::LinkClicked)
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
    parse_with(State::default(), markdown, 0).map(|(item, _start, _broken_links)| item)
}

#[derive(Debug, Default)]
struct State {
    /// 在本次解析之后，下一次将被重新解析的源文本的
    /// 起点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The start of the source that will be re-parsed next, after the
    /// current parse.
    ///
    /// </details>
    window: Option<usize>,
    /// 引用定义，把标签映射到它的目标地址。
    ///
    /// 和 CommonMark 一样，同一标签以第一个定义为准。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The reference definitions, mapping a label to its destination.
    ///
    /// The first definition of a label wins, like in CommonMark.
    ///
    /// </details>
    references: HashMap<String, String>,
    /// 这些标签在 `references` 中的目标地址来自
    /// 最后一行的定义，而该行尚未结束、
    /// 仍可能增长；它们的目标地址会在每次推入时更新，
    /// 直到该行结束。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The labels whose destination in `references` comes from the
    /// definition of the last line, which is not terminated yet and
    /// can still grow; their destination is updated on each push,
    /// until the line is terminated.
    ///
    /// </details>
    references_staged: HashSet<String>,
    images: HashSet<Uri>,
    /// 最近几次重新解析中出现的代码块的高亮器，
    /// 以代码块源文本在外层流中的起点作为键。
    ///
    /// 高亮器只会被同一个代码块（相同的
    /// 源文本起点和语言）复用，这样在重新解析一个仍在增长的
    /// 代码块时，它的行缓存依然有效。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The highlighters of the code blocks seen in the last re-parses,
    /// keyed by the start of the block's source in the outer stream.
    ///
    /// A highlighter is only reused for the same code block (the same
    /// source start and language), so that its line cache stays valid
    /// across the re-parses of a block that is still growing.
    ///
    /// </details>
    #[cfg(feature = "highlighter")]
    parsers: HashMap<usize, code::Parser>,
}

/// 解析器使用的选项。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The options used by the parser.
///
/// </details>
fn options() -> pulldown_cmark::Options {
    pulldown_cmark::Options::ENABLE_YAML_STYLE_METADATA_BLOCKS
        | pulldown_cmark::Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS
        | pulldown_cmark::Options::ENABLE_TABLES
        | pulldown_cmark::Options::ENABLE_STRIKETHROUGH
        | pulldown_cmark::Options::ENABLE_TASKLISTS
}

/// 把一次解析得到的引用定义并入 `references`，
/// 和 CommonMark 一样，同一标签保留第一个定义。
///
/// 最后一行的定义在该行尚未结束时
/// 仍可能增长，因此它的目标地址会在每次推入时更新，
/// 直到该行结束；正在增长的标签记录在
/// `growing_refs` 中。
///
/// `markdown` 是本次解析的源文本，`definitions` 是其中的
/// 引用定义。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Absorbs the reference definitions of a parse into `references`,
/// keeping the first definition of a label, like in CommonMark.
///
/// The definition of the last line, when that line is not terminated
/// yet, can still grow, so its destination is updated on each push,
/// until the line is terminated; the growing labels are tracked in
/// `growing_refs`.
///
/// `markdown` is the source of the parse, and `definitions` are its
/// reference definitions.
///
/// </details>
fn absorb_references(
    markdown: &str,
    definitions: &pulldown_cmark::RefDefs<'_>,
    references: &mut HashMap<String, String>,
    growing_refs: &mut HashSet<String>,
) {
    for reference in definitions.iter() {
        let name = reference.0.to_string();
        let dest = reference.1.dest.to_string();

        if markdown[reference.1.span.end..].contains('\n') {
            if !references.contains_key(&name) || growing_refs.remove(&name) {
                let _ = references.insert(name, dest);
            }
        } else if growing_refs.contains(&name) {
            // The map's value is the growing one: update it
            // 映射中的值就是正在增长的那个：更新它
            let _ = references.insert(name, dest);
        } else if !references.contains_key(&name) {
            // No terminated definition wins: the growing value is
            // provisional
            // 没有已结束的定义胜出：正在增长的值只是
            // 临时的
            let _ = growing_refs.insert(name.clone());
            let _ = references.insert(name, dest);
        }
    }
}

fn parse_with<'a>(
    mut state: impl BorrowMut<State> + 'a,
    markdown: &'a str,
    _offset: usize,
) -> impl Iterator<Item = (Item, usize, HashSet<String>)> + 'a {
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
        /// 列表最后一项的起点（如果有）。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The start of the last item of the list, if any.
        ///
        /// </details>
        last_item_start: Option<usize>,
    }

    // The broken links reported by the parser, along with their span
    // in the input.
    //
    // The broken links are reported before the items that contain them
    // are produced, so the links are attributed to an item by their
    // span.
    // 解析器报告的断开的链接，以及它们在输入中的
    // 范围。
    //
    // 断开的链接会在包含它们的条目产生之前
    // 被报告，因此要根据范围把这些链接归属到
    // 对应的条目。
    let broken_links = Rc::new(RefCell::new(Vec::new()));

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
    let mut paragraph_start = None;
    let mut stack = Vec::new();

    #[cfg(feature = "highlighter")]
    let mut code_block_key = None;

    let parser = pulldown_cmark::Parser::new_with_broken_link_callback(markdown, options(), {
        let references = state.borrow().references.clone();
        let broken_links = broken_links.clone();

        Some(move |broken_link: pulldown_cmark::BrokenLink<'_>| {
            if let Some(reference) = references.get(broken_link.reference.as_ref()) {
                Some((
                    pulldown_cmark::CowStr::from(reference.to_owned()),
                    broken_link.reference.into_static(),
                ))
            } else {
                RefCell::borrow_mut(&broken_links)
                    .push((broken_link.span, broken_link.reference.into_string()));

                None
            }
        })
    });

    {
        let state = state.borrow_mut();
        absorb_references(
            markdown,
            parser.reference_definitions(),
            &mut state.references,
            &mut state.references_staged,
        );
    }

    let produce = move |state: &mut State, stack: &mut Vec<Scope>, item, source: Range<usize>| {
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
            state.window = Some(source.start);

            // Attribute the broken links whose span falls within the
            // source of the item
            // 把范围落在该条目源文本之内的
            // 断开的链接归属给它
            let mut links = HashSet::new();
            for (span, reference) in RefCell::borrow(&broken_links).iter() {
                if source.contains(&span.start) {
                    let _ = links.insert(reference.clone());
                }
            }

            Some((item, source.start, links))
        }
    };

    // A reference link or image resolves with the first definition
    // of its label in the whole document, like in the one-shot
    // parse. A later definition that falls within the input wins
    // within the input, so, when known, prefer the global
    // definition.
    // 引用式的链接或图像，按其标签在整个文档中的
    // 第一个定义来解析，和一次性解析
    // 一样。落在输入范围内的较晚定义在输入内部
    // 会胜出，因此在已知的情况下，优先使用全局的
    // 定义。
    let resolve_reference = |state: &mut State,
                             link_type: pulldown_cmark::LinkType,
                             id: &str,
                             dest_url: &pulldown_cmark::CowStr<'a>|
     -> String {
        match link_type {
            pulldown_cmark::LinkType::Reference
            | pulldown_cmark::LinkType::ReferenceUnknown
            | pulldown_cmark::LinkType::Collapsed
            | pulldown_cmark::LinkType::CollapsedUnknown
            | pulldown_cmark::LinkType::Shortcut
            | pulldown_cmark::LinkType::ShortcutUnknown => state
                .references
                .get(id)
                .cloned()
                .unwrap_or_else(|| dest_url.to_string()),
            _ => dest_url.to_string(),
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
            pulldown_cmark::Tag::Link {
                link_type,
                dest_url,
                id,
                ..
            } if !metadata => {
                link = Some(resolve_reference(
                    state.borrow_mut(),
                    link_type,
                    &id,
                    &dest_url,
                ));
                None
            }
            pulldown_cmark::Tag::Paragraph if !metadata => {
                paragraph_start = Some(source.start);
                None
            }
            pulldown_cmark::Tag::Image {
                link_type,
                dest_url,
                title,
                id,
            } if !metadata => {
                image = Some((
                    resolve_reference(state.borrow_mut(), link_type, &id, &dest_url),
                    title.into_string(),
                    spans.len(),
                ));
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
                    last_item_start: None,
                }));

                prev
            }
            pulldown_cmark::Tag::Item => {
                if let Some(Scope::List(list)) = stack.last_mut() {
                    list.last_item_start = Some(source.start);
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
            pulldown_cmark::Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(language))
                if !metadata =>
            {
                #[cfg(feature = "highlighter")]
                {
                    // The start of the block in the outer stream; the
                    // key that identifies the block across re-parses,
                    // so that the highlighter's line cache is only
                    // reused for the same block.
                    // 代码块在外层流中的起点；它是在多次
                    // 重新解析之间标识该代码块的键，
                    // 这样高亮器的行缓存只会被
                    // 同一个代码块复用。
                    let key = _offset + source.start;
                    let state = state.borrow_mut();
                    let language = language.split(',').next().unwrap_or_default();

                    let mut parser = state
                        .parsers
                        .remove(&key)
                        .filter(|parser| parser.language() == language)
                        .unwrap_or_else(|| code::Parser::new(language));

                    parser.prepare();
                    let _ = state.parsers.insert(key, parser);

                    code_block_key = Some(key);
                }

                code_block = true;
                code_language = (!language.is_empty()).then(|| language.into_string());

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
                paragraph_start = None;

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

                let last_item_start = list.last_item_start;
                let produced = produce(
                    state.borrow_mut(),
                    &mut stack,
                    Item::List {
                        start: list.start,
                        bullets: list.bullets,
                    },
                    source,
                );

                // A list is re-parsed only from the start of its last
                // item, so that adding new items to a long list does not
                // require re-parsing the whole list.
                // 列表只从它最后一项的起点开始重新解析，
                // 这样向长列表添加新项时就不需要
                // 重新解析整个列表。
                if produced.is_some()
                    && let Some(start) = last_item_start
                {
                    state.borrow_mut().window = Some(start);
                }

                produced
            }
            pulldown_cmark::TagEnd::BlockQuote(_kind) if !metadata => {
                let scope = stack.pop()?;

                let Scope::Quote(quote) = scope else {
                    return None;
                };

                produce(state.borrow_mut(), &mut stack, Item::Quote(quote), source)
            }
            pulldown_cmark::TagEnd::Image if !metadata => {
                let (url, title, start) = image.take()?;
                let alt = Text::new(spans.drain(start..).collect());

                let state = state.borrow_mut();
                let _ = state.images.insert(url.clone());

                let produced = produce(state, &mut stack, Item::Image { url, title, alt }, source);

                // A top-level image is re-parsed from the start of the
                // line that contains it, as the rest of the line can
                // change how the image is parsed.
                // 顶层图像从包含它的那一行的起点开始
                // 重新解析，因为该行的其余部分可能
                // 改变图像的解析方式。
                if let Some(start) = paragraph_start.filter(|_| produced.is_some()) {
                    state.borrow_mut().window = Some(start);
                }

                produced
            }
            pulldown_cmark::TagEnd::CodeBlock if !metadata => {
                code_block = false;

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
                if let Some(key) = code_block_key
                    && let Some(highlighter) = state.borrow_mut().parsers.get_mut(&key)
                {
                    for line in text.lines() {
                        code_lines.push(Text::new(highlighter.parse_line(line).to_vec()));
                    }
                }

                #[cfg(not(feature = "highlighter"))]
                for line in text.lines() {
                    code_lines.push(Text::new(vec![Span::Code {
                        text: line.to_owned(),
                        code: Code::Other,
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
                inline_code: false,
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
                inline_code: true,
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
                inline_code: false,
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
                inline_code: false,
            });
            None
        }
        pulldown_cmark::Event::Rule => produce(state.borrow_mut(), &mut stack, Item::Rule, source),
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    /// 应用于普通文本的 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Font`] to be applied to basic text.
    ///
    /// </details>
    pub font: Font,
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
    /// 基础行高。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The base line height.
    ///
    /// </details>
    pub line_height: LineHeight,
    /// 基础文本大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The base text size.
    ///
    /// </details>
    pub text_size: Pixels,
    /// 代码块使用的文本大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text size used in code blocks.
    ///
    /// </details>
    pub code_block_size: Pixels,
    /// 行内代码使用的文本大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text size used in inline code.
    ///
    /// </details>
    pub inline_code_size: Pixels,
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
    /// 元素之间使用的间距。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The spacing to be used between elements.
    ///
    /// </details>
    pub spacing: Pixels,
}

impl Settings {
    /// 使用给定的基础文本大小（以 [`Pixels`] 为单位）创建新的 [`Settings`]。
    ///
    /// 各级标题会自动调整。具体来说，
    /// 第一级是基础大小的 1.5 倍，第二级是
    /// 1.25 倍，第三级是 1.125 倍，其余各级
    /// 使用基础大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates new [`Settings`] with the given base text size in [`Pixels`].
    ///
    /// Heading levels will be adjusted automatically. Specifically,
    /// the first level will be 1.5 times the base size, the second
    /// 1.25 times, the third 1.125 times, and the remaining levels
    /// will use the base size.
    ///
    /// </details>
    pub fn with_text_size(text_size: impl Into<Pixels>) -> Self {
        let text_size = text_size.into();
        let line_height = LineHeight::default();

        Self {
            font: Font::DEFAULT,
            inline_code_font: Font::MONOSPACE,
            code_block_font: Font::MONOSPACE,
            line_height,
            text_size,
            inline_code_size: text_size * 0.85,
            code_block_size: text_size * 0.85,
            h1_size: text_size * 1.5,
            h2_size: text_size * 1.25,
            h3_size: text_size * 1.125,
            h4_size: text_size,
            h5_size: text_size,
            h6_size: text_size,
            spacing: line_height.to_absolute(text_size) / 1.5,
        }
    }

    /// 设置 [`Settings`] 的 [`LineHeight`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`LineHeight`] of the [`Settings`].
    ///
    /// </details>
    pub fn line_height(self, line_height: impl Into<LineHeight>) -> Self {
        let line_height = line_height.into();

        Self {
            line_height,
            spacing: line_height.to_absolute(self.text_size) / 1.5,
            ..self
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::with_text_size(16)
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
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::{Renderer, Widget}; pub use iced_widget::core::*; }
/// #
/// use iced::widget::markdown;
/// use iced::{Theme, Widget};
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
///     fn view(&self) -> impl Widget<Message> {
///         markdown::view(
///             &self.markdown,
///             markdown::Settings::default(),
///             Theme::TokyoNight,
///         )
///             .map(Message::LinkClicked)
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
    items: &'a [Item],
    settings: impl Into<Settings>,
    theme: Theme,
) -> impl Widget<Uri, Theme, Renderer> + 'a
where
    Theme: Catalog + 'static,
    Renderer: core::text::Renderer,
{
    view_with(
        items,
        settings,
        &DefaultViewer {
            theme,
            highlighter: None,
        },
    )
    .boxed()
}

/// 运行 [`view`]，但使用自定义的 [`Viewer`] 把 [`Item`] 转换为部件。
///
/// 如果你想自定义某些 Markdown 元素的外观，
/// 这会很有用。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Runs [`view`] but with a custom [`Viewer`] to turn an [`Item`] into a widget.
///
/// This is useful if you want to customize the look of certain Markdown
/// elements.
///
/// </details>
pub fn view_with<'a, Message, Theme, Renderer>(
    items: &'a [Item],
    settings: impl Into<Settings>,
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
) -> impl Widget<Message, Theme, Renderer> + 'a
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer,
{
    self::items(viewer, settings.into(), items)
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
) -> impl Widget<Message, Theme, Renderer> + 'a
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer + 'a,
{
    match item {
        Item::Image { url, title, alt } => viewer.image(settings, url, title, alt).boxed(),
        Item::Heading(level, text) => viewer.heading(settings, level, text).boxed(),
        Item::Paragraph(text) => viewer.paragraph(settings, text).boxed(),
        Item::CodeBlock {
            language,
            code,
            lines,
        } => viewer
            .code_block(settings, language.as_deref(), code, lines)
            .boxed(),
        Item::List {
            start: None,
            bullets,
        } => viewer.unordered_list(settings, bullets).boxed(),
        Item::List {
            start: Some(start),
            bullets,
        } => viewer.ordered_list(settings, *start, bullets).boxed(),
        Item::Quote(quote) => viewer.quote(settings, quote).boxed(),
        Item::Rule => viewer.rule().boxed(),
        Item::Table { columns, rows } => viewer.table(settings, columns, rows).boxed(),
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
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
    settings: Settings,
    level: &'a HeadingLevel,
    text: &'a Text,
    on_link_click: impl Fn(Uri) -> Message + 'a,
) -> impl Widget<Message, Theme, Renderer> + 'a
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer + 'a,
{
    let Settings {
        h1_size,
        h2_size,
        h3_size,
        h4_size,
        h5_size,
        h6_size,
        ..
    } = settings;

    let size = match level {
        pulldown_cmark::HeadingLevel::H1 => h1_size,
        pulldown_cmark::HeadingLevel::H2 => h2_size,
        pulldown_cmark::HeadingLevel::H3 => h3_size,
        pulldown_cmark::HeadingLevel::H4 => h4_size,
        pulldown_cmark::HeadingLevel::H5 => h5_size,
        pulldown_cmark::HeadingLevel::H6 => h6_size,
    };

    container(
        rich_text(text.spans(
            Settings {
                font: Font {
                    weight: font::Weight::Bold,
                    ..settings.font
                },
                inline_code_font: Font {
                    weight: font::Weight::Bold,
                    ..settings.inline_code_font
                },
                inline_code_size: size * (settings.inline_code_size / settings.text_size),
                ..settings
            },
            viewer.theme(),
            viewer.highlighter(),
        ))
        .on_link_click(on_link_click)
        .size(size)
        .line_height(settings.line_height)
        .selectable(true),
    )
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
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
    settings: Settings,
    text: &Text,
    on_link_click: impl Fn(Uri) -> Message + 'a,
) -> impl Widget<Message, Theme, Renderer> + 'a
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer + 'a,
{
    rich_text(text.spans(settings, viewer.theme(), viewer.highlighter()))
        .size(settings.text_size)
        .line_height(settings.line_height)
        .on_link_click(on_link_click)
        .selectable(true)
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
) -> impl Widget<Message, Theme, Renderer> + 'a
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer + 'a,
{
    column(bullets.iter().map(|bullet| {
        row![
            match bullet {
                Bullet::Point { .. } => {
                    text("•")
                        .size(settings.text_size)
                        .line_height(settings.line_height)
                        .boxed()
                }
                Bullet::Task { done, .. } => {
                    container(checkbox(*done).size(settings.text_size))
                        .center_y(settings.line_height.to_absolute(settings.text_size))
                        .boxed()
                }
            },
            items(
                viewer,
                Settings {
                    spacing: settings.spacing / 2.0,
                    ..settings
                },
                bullet.items(),
            )
        ]
        .spacing(settings.text_size / 2.0)
    }))
    .spacing(settings.spacing / 2.0)
    .padding(padding::left(settings.text_size.0))
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
) -> impl Widget<Message, Theme, Renderer> + 'a
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer + 'a,
{
    let digits = (start + bullets.len() as u64).max(1).ilog10() + 1;

    column(bullets.iter().enumerate().map(|(i, bullet)| {
        row![
            text!("{}.", i as u64 + start)
                .line_height(settings.line_height)
                .size(settings.text_size)
                .align_x(alignment::Horizontal::Right)
                .width(settings.text_size * ((digits as f32 / 2.0).ceil() + 1.0)),
            items(
                viewer,
                Settings {
                    spacing: settings.spacing / 2.0,
                    ..settings
                },
                bullet.items(),
            )
        ]
        .spacing(settings.text_size / 2.0)
    }))
    .spacing(settings.spacing / 2.0)
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
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
    settings: Settings,
    lines: &'a [Text],
    on_link_click: impl Fn(Uri) -> Message + Clone + 'a,
) -> impl Widget<Message, Theme, Renderer> + 'a
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer + 'a,
{
    let padding = settings.code_block_size / 0.85 * 0.75;

    container(
        scrollable(column(lines.iter().map(|line| {
            rich_text(line.spans(settings, viewer.theme(), viewer.highlighter()))
                .on_link_click(on_link_click.clone())
                .font(settings.code_block_font)
                .size(settings.code_block_size)
                .line_height(settings.line_height)
                .selectable(true)
        })))
        .direction(scrollable::Direction::Horizontal(
            scrollable::Scrollbar::default()
                .width(padding / 2.0)
                .scroller_width(padding / 2.0),
        ))
        .spacing(padding),
    )
    .width(Length::Fill)
    .padding(padding)
    .class(Theme::code_block())
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
) -> impl Widget<Message, Theme, Renderer> + 'a
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer + 'a,
{
    container(
        column(
            contents
                .iter()
                .map(|content| item(viewer, settings, content)),
        )
        .spacing(settings.spacing.0),
    )
    .width(Length::Fill)
    .padding(settings.spacing.0)
    .class(Theme::quote())
}

/// 使用默认外观显示分隔线。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Displays a rule using the default look.
///
/// </details>
pub fn rule<'a, Message, Theme, Renderer>() -> impl Widget<Message, Theme, Renderer> + 'a
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer + 'a,
{
    rule::horizontal(2)
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
) -> impl Widget<Message, Theme, Renderer> + 'a
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer + 'a,
{
    use crate::table;

    let table = table(
        columns.iter().enumerate().map(move |(i, column)| {
            table::column(items(viewer, settings, &column.header), move |row: &Row| {
                if let Some(cells) = row.cells.get(i) {
                    items(viewer, settings, cells)
                } else {
                    items(viewer, settings, &[])
                }
            })
            .width(Length::Fit.max(300))
            .align_x(match column.alignment {
                pulldown_cmark::Alignment::None | pulldown_cmark::Alignment::Left => {
                    alignment::Horizontal::Left
                }
                pulldown_cmark::Alignment::Center => alignment::Horizontal::Center,
                pulldown_cmark::Alignment::Right => alignment::Horizontal::Right,
            })
        }),
        rows,
    )
    .padding_x(settings.spacing.0)
    .padding_y(settings.spacing.0 / 2.0)
    .separator_x(0);

    center_x(
        scrollable(table)
            .direction(scrollable::Direction::Horizontal(
                scrollable::Scrollbar::default(),
            ))
            .spacing(settings.spacing.0 / 2.0),
    )
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
) -> impl Widget<Message, Theme, Renderer> + 'a
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer + 'a,
{
    column(sections(items).map(|(heading, contents)| {
        let contents = column(
            contents
                .iter()
                .map(|content| item(viewer, settings, content)),
        )
        .spacing(settings.spacing);

        if let Some(heading) = heading {
            column![item(viewer, settings, heading), contents].spacing(settings.spacing / 2.0)
        } else {
            column![contents]
        }
    }))
    .spacing(settings.spacing * 1.5)
}

/// 用于显示 Markdown [`Item`] 的视图策略。
///
/// [`Viewer`] 负责把每个 [`Item`] 转换为部件。它
/// 还提供渲染所用的 [`Theme`] 和 [`text::Highlighter`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A view strategy to display a Markdown [`Item`].
///
/// A [`Viewer`] is in charge of turning each [`Item`] into a widget. It
/// also provides the [`Theme`] and [`text::Highlighter`] used for rendering.
///
/// </details>
pub trait Viewer<'a, Message, Theme = crate::Theme, Renderer = crate::Renderer>
where
    Self: Sized + 'a,
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer + 'a,
{
    /// 用于为 Markdown 元素设置样式的 [`Theme`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Theme`] used for styling the Markdown elements.
    ///
    /// </details>
    fn theme(&self) -> &Theme;

    /// 用于高亮 [`Code`] 区域的 [`text::Highlighter`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`text::Highlighter`] used for highligthing [`Code`] regions.
    ///
    /// </details>
    fn highlighter(&self) -> &dyn text::Highlighter<Code, Theme>;

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
    ) -> impl Widget<Message, Theme, Renderer> + 'a {
        let _url = url;
        let _title = title;

        container(
            rich_text(alt.spans(settings, self.theme(), self.highlighter()))
                .on_link_click(Self::on_link_click),
        )
        .padding(settings.spacing.0)
        .class(Theme::code_block())
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
    ) -> impl Widget<Message, Theme, Renderer> + 'a {
        heading(self, settings, level, text, Self::on_link_click)
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
    ) -> impl Widget<Message, Theme, Renderer> + 'a {
        paragraph(self, settings, text, Self::on_link_click)
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
    ) -> impl Widget<Message, Theme, Renderer> + 'a {
        let _language = language;
        let _code = code;

        code_block(self, settings, lines, Self::on_link_click)
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
    ) -> impl Widget<Message, Theme, Renderer> + 'a {
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
    ) -> impl Widget<Message, Theme, Renderer> + 'a {
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
    ) -> impl Widget<Message, Theme, Renderer> + 'a {
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
    fn rule(&self) -> impl Widget<Message, Theme, Renderer> + 'a {
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
    ) -> impl Widget<Message, Theme, Renderer> + 'a {
        table(self, settings, columns, rows)
    }
}

/// 默认的 [`Viewer`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The default [`Viewer`].
///
/// </details>
pub struct DefaultViewer<'a, Theme> {
    theme: Theme,
    highlighter: Option<Box<dyn text::Highlighter<Code, Theme> + 'a>>,
}

impl<'a, Theme> DefaultViewer<'a, Theme> {
    /// 使用给定的 [`Theme`] 创建一个新的 [`DefaultViewer`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`DefaultViewer`] with the given [`Theme`].
    ///
    /// </details>
    pub fn new(theme: Theme) -> Self {
        Self {
            theme,
            highlighter: None,
        }
    }

    /// 为 [`DefaultViewer`] 设置自定义的 [`text::Highlighter`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets a custom [`text::Highlighter`] for the [`DefaultViewer`].
    ///
    /// </details>
    pub fn highlighter(mut self, highlighter: impl text::Highlighter<Code, Theme> + 'a) -> Self {
        self.highlighter = Some(Box::new(highlighter));
        self
    }
}

impl<'a, Theme, Renderer> Viewer<'a, Uri, Theme, Renderer> for DefaultViewer<'a, Theme>
where
    Theme: Catalog + 'a,
    Renderer: core::text::Renderer + 'a,
{
    fn theme(&self) -> &Theme {
        &self.theme
    }

    fn highlighter(&self) -> &dyn text::Highlighter<Code, Theme> {
        self.highlighter
            .as_deref()
            .unwrap_or_else(|| self.theme.highlighter())
    }

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
    + Clone
    + PartialEq
{
    /// [`Catalog`] 的唯一标识符。
    ///
    /// 当主题变化时，它会被用来使文本片段的样式失效。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The unique identifier of the [`Catalog`].
    ///
    /// This will be used to invalidate span styling when a theme changes.
    ///
    /// </details>
    fn id(&self) -> &str;

    /// 链接的 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Color`] of some link.
    ///
    /// </details>
    fn link_color(&self) -> Color;

    /// 行内代码的 [`InlineCode`] 样式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`InlineCode`] style of some inline code.
    ///
    /// </details>
    fn code(&self) -> InlineCode;

    /// 代码块的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The styling class of a code block.
    ///
    /// </details>
    fn code_block<'a>() -> <Self as container::Catalog>::Class<'a>;

    /// 引用块的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The styling class of a quote.
    ///
    /// </details>
    fn quote<'a>() -> <Self as container::Catalog>::Class<'a>;

    /// 用于高亮代码的默认 [`text::Highlighter`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default [`text::Highlighter`] to use to highlight code.
    ///
    /// </details>
    fn highlighter(&self) -> &dyn text::Highlighter<Code, Self>;
}

/// 行内代码的样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The style of some inline code.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InlineCode {
    /// 应用在代码四周的 [`Padding`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Padding`] to apply around the code.
    ///
    /// </details>
    pub padding: Padding,
    /// 代码的 [`Highlight`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Highlight`] of the code.
    ///
    /// </details>
    pub highlight: Highlight,
    /// 代码的 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Color`] of the code.
    ///
    /// </details>
    pub color: Color,
}

impl Catalog for Theme {
    fn id(&self) -> &str {
        theme::Base::name(self)
    }

    fn link_color(&self) -> Color {
        self.seed().primary
    }

    fn code(&self) -> InlineCode {
        let palette = self.palette();

        InlineCode {
            padding: padding::horizontal(4).vertical(1),
            highlight: Highlight {
                background: palette.background.weaker.color.into(),
                border: border::rounded(4),
            },
            color: palette.background.weaker.text,
        }
    }

    fn code_block<'a>() -> <Self as container::Catalog>::Class<'a> {
        Box::new(|theme| container::dark(theme).border(border::rounded(5)))
    }

    fn quote<'a>() -> <Self as container::Catalog>::Class<'a> {
        Box::new(|theme| {
            let palette = theme.palette();

            container::Style {
                text_color: Some(palette.background.weakest.text),
                background: Some(palette.background.weakest.color.into()),
                border: border::rounded(5),
                ..container::Style::default()
            }
        })
    }

    fn highlighter(&self) -> &dyn text::Highlighter<Code, Self> {
        &Code::highlight
    }
}

#[cfg(feature = "highlighter")]
mod code {
    use super::Span;

    #[derive(Debug)]
    pub struct Parser {
        lines: Vec<(String, Vec<Span>)>,
        language: String,
        stream: iced_highlighter::Stream,
        current: usize,
    }

    impl Parser {
        pub fn new(language: &str) -> Self {
            Self {
                lines: Vec::new(),
                stream: iced_highlighter::Stream::new(&iced_highlighter::Settings {
                    token: language.to_owned(),
                }),
                language: language.to_owned(),
                current: 0,
            }
        }

        pub fn language(&self) -> &str {
            &self.language
        }

        pub fn prepare(&mut self) {
            self.current = 0;
        }

        pub fn parse_line(&mut self, text: &str) -> &[Span] {
            match self.lines.get(self.current) {
                Some(line) if line.0 == text => {}
                _ => {
                    if self.current + 1 < self.lines.len() {
                        log::debug!("Resetting highlighter...");
                        self.stream.reset();
                        self.lines.truncate(self.current);

                        for line in &self.lines {
                            log::debug!("Refeeding {n} lines", n = self.lines.len());

                            let _ = self.stream.parse_line(&line.0);
                            self.stream.commit();
                        }
                    }

                    log::trace!("Parsing: {text}", text = text.trim_end());

                    if self.current == self.lines.len() {
                        self.stream.commit();
                    }

                    let mut spans = Vec::new();

                    for (range, code) in self.stream.parse_line(text) {
                        spans.push(Span::Code {
                            text: text[range].to_owned(),
                            code,
                        });
                    }

                    if self.current == self.lines.len() {
                        self.lines.push((text.to_owned(), spans));
                    } else {
                        self.lines[self.current] = (text.to_owned(), spans);
                    }
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
}
