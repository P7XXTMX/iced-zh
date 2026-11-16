//! iced 的语法高亮器。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! A syntax highlighter for iced.
//!
//! </details>
use iced_core as core;

use crate::core::Code;
use crate::core::text;

use std::ops::Range;
use std::sync::LazyLock;

use syntect::parsing;
use two_face::re_exports::syntect;

static SYNTAXES: LazyLock<parsing::SyntaxSet> = LazyLock::new(two_face::syntax::extra_no_newlines);

const LINES_PER_SNAPSHOT: usize = 50;

/// 语法解析器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A syntax parser.
///
/// </details>
#[derive(Debug)]
pub struct Parser {
    syntax: &'static parsing::SyntaxReference,
    caches: Vec<(parsing::ParseState, parsing::ScopeStack)>,
    current_line: usize,
}

/// 遍历一行中各个高亮区域的迭代器。
///
/// 每一项是行内的一个字符范围，以及
/// 该区域对应的 [`Code`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An iterator over the highlighted regions of a line.
///
/// Each item is a character range within the line, paired with
/// the [`Code`] of the region.
///
/// </details>
pub type CodeIterator<'a> = Box<dyn Iterator<Item = (Range<usize>, Code)> + 'a>;

impl Parser {
    /// 使用给定的 [`Settings`] 创建一个新的 [`Parser`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Parser`] with the given [`Settings`].
    ///
    /// </details>
    pub fn new(settings: &Settings) -> Self {
        let syntax = SYNTAXES
            .find_syntax_by_token(&settings.token)
            .unwrap_or_else(|| SYNTAXES.find_syntax_plain_text());

        let parser = parsing::ParseState::new(syntax);
        let stack = parsing::ScopeStack::new();

        Parser {
            syntax,
            caches: vec![(parser, stack)],
            current_line: 0,
        }
    }

    /// 使用给定的 [`Settings`] 更新解析器，
    /// 并从第一行重新开始解析。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Updates the parser with the given [`Settings`],
    /// restarting it from the first line.
    ///
    /// </details>
    pub fn update(&mut self, new_settings: &Settings) {
        self.syntax = SYNTAXES
            .find_syntax_by_token(&new_settings.token)
            .unwrap_or_else(|| SYNTAXES.find_syntax_plain_text());

        // Restart the parser
        // 重启解析器
        self.change_line(0);
    }

    /// 更改解析器当前所在的行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Changes the line the parser is currently on.
    ///
    /// </details>
    pub fn change_line(&mut self, line: usize) {
        let snapshot = line / LINES_PER_SNAPSHOT;

        if snapshot <= self.caches.len() {
            self.caches.truncate(snapshot);
            self.current_line = snapshot * LINES_PER_SNAPSHOT;
        } else {
            self.caches.truncate(1);
            self.current_line = 0;
        }

        let (parser, stack) = self.caches.last().cloned().unwrap_or_else(|| {
            (
                parsing::ParseState::new(self.syntax),
                parsing::ScopeStack::new(),
            )
        });

        self.caches.push((parser, stack));
    }

    /// 高亮给定的行，返回一个 [`CodeIterator`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Highlights the given line, returning a [`CodeIterator`].
    ///
    /// </details>
    pub fn parse_line(&mut self, line: &str) -> CodeIterator<'_> {
        if self.current_line / LINES_PER_SNAPSHOT >= self.caches.len() {
            let (parser, stack) = self.caches.last().expect("Caches must not be empty");

            self.caches.push((parser.clone(), stack.clone()));
        }

        self.current_line += 1;

        let (parser, stack) = self.caches.last_mut().expect("Caches must not be empty");
        let ops = parser.parse_line(line, &SYNTAXES).unwrap_or_default();

        Box::new(code_iterator(ops, line, stack))
    }

    /// 返回解析器当前所在的行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the line the parser is currently on.
    ///
    /// </details>
    pub fn current_line(&self) -> usize {
        self.current_line
    }
}

impl text::Parser for Parser {
    type Settings = Settings;
    type Output = Code;
    type Iterator<'a> = CodeIterator<'a>;

    fn new(settings: &Self::Settings) -> Self {
        Self::new(settings)
    }

    fn update(&mut self, new_settings: &Self::Settings) {
        self.update(new_settings);
    }

    fn change_line(&mut self, line: usize) {
        self.change_line(line);
    }

    fn parse_line(&mut self, line: &str) -> Self::Iterator<'_> {
        self.parse_line(line)
    }

    fn current_line(&self) -> usize {
        self.current_line()
    }
}

fn code_iterator<'a>(
    ops: Vec<(usize, parsing::ScopeStackOp)>,
    line: &str,
    stack: &'a mut parsing::ScopeStack,
) -> impl Iterator<Item = (Range<usize>, Code)> + 'a {
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
            Some((range, scope_from_stack(&stack.scopes)))
        }
    })
}

/// 流式语法解析器。
///
/// 它可以高效地高亮一个不可变的词法单元流。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A streaming syntax parser.
///
/// It can efficiently highlight an immutable stream of tokens.
///
/// </details>
#[derive(Debug)]
pub struct Stream {
    syntax: &'static parsing::SyntaxReference,
    commit: (parsing::ParseState, parsing::ScopeStack),
    state: parsing::ParseState,
    stack: parsing::ScopeStack,
}

impl Stream {
    /// 创建一个新的 [`Stream`] 解析器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Stream`] parser.
    ///
    /// </details>
    pub fn new(settings: &Settings) -> Self {
        let syntax = SYNTAXES
            .find_syntax_by_token(&settings.token)
            .unwrap_or_else(|| SYNTAXES.find_syntax_plain_text());

        let state = parsing::ParseState::new(syntax);
        let stack = parsing::ScopeStack::new();

        Self {
            syntax,
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
    pub fn parse_line(&mut self, line: &str) -> impl Iterator<Item = (Range<usize>, Code)> + '_ {
        self.state = self.commit.0.clone();
        self.stack = self.commit.1.clone();

        let ops = self.state.parse_line(line, &SYNTAXES).unwrap_or_default();
        code_iterator(ops, line, &mut self.stack)
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

    /// 重置 [`Stream`] 解析器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Resets the [`Stream`] parser.
    ///
    /// </details>
    pub fn reset(&mut self) {
        self.state = parsing::ParseState::new(self.syntax);
        self.stack = parsing::ScopeStack::new();
        self.commit = (self.state.clone(), self.stack.clone());
    }
}

/// [`Parser`] 的设置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The settings of a [`Parser`].
///
/// </details>
#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    /// 要高亮的文件的扩展名或语言名称。
    ///
    /// [`Parser`] 会根据这个标记自动确定
    /// 高亮所用的语法。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The extension of the file or the name of the language to highlight.
    ///
    /// The [`Parser`] will use the token to automatically determine
    /// the grammar to use for highlighting.
    ///
    /// </details>
    pub token: String,
}

/// 作用域族及其对应的类别。
///
/// 各个族按从最具体到最不具体的顺序排列，例如
/// `entity.name.function` 优先于 `entity.name`。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The scope families and their classes.
///
/// The families are listed most specific first, so that e.g.
/// `entity.name.function` wins over `entity.name`.
///
/// </details>
static FAMILIES: LazyLock<Vec<(parsing::Scope, Code)>> = LazyLock::new(|| {
    [
        ("meta.path", Code::Path),
        ("invalid", Code::Invalid),
        ("constant", Code::Constant),
        ("string", Code::String),
        ("comment", Code::Comment),
        ("keyword", Code::Keyword),
        ("storage.type.", Code::Keyword),
        ("storage.type", Code::Type),
        ("storage", Code::Keyword),
        ("entity.name.function", Code::Function),
        ("entity.name", Code::Type),
        ("entity.other.inherited-class", Code::Type),
        ("support", Code::Support),
        ("variable.function", Code::Function),
        ("variable", Code::Variable),
        ("punctuation", Code::Punctuation),
    ]
    .into_iter()
    .map(|(name, class)| {
        (
            parsing::Scope::new(name).expect("scope family is valid"),
            class,
        )
    })
    .collect()
});

/// 对一个高亮区域的作用域栈进行分类。
///
/// 从最具体的作用域（最后一个）向最不具体的作用域
/// （第一个）遍历整个栈；第一个匹配到某个族的作用域
/// 决定类别。如果没有任何作用域匹配，该区域
/// 归类为 [`Code::Other`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Classifies the scope stack of a highlighted region.
///
/// The stack is walked from the most specific scope (last) to the
/// least specific (first); the first scope that matches a family
/// determines the class. If no scope matches, the region is
/// classified as [`Code::Other`].
///
/// </details>
fn scope_from_stack(stack: &[parsing::Scope]) -> Code {
    for scope in stack.iter().rev() {
        for (family, class) in FAMILIES.iter() {
            if family.is_prefix_of(*scope) {
                return *class;
            }
        }
    }

    Code::Other
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

#[cfg(test)]
mod tests {
    use super::*;

    /// 根据以点分隔的作用域名称构建作用域栈。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Builds a scope stack from dotted scope names.
    ///
    /// </details>
    fn stack(names: &[&str]) -> Vec<parsing::Scope> {
        names
            .iter()
            .map(|name| parsing::Scope::new(name).unwrap())
            .collect()
    }

    #[test]
    fn scopes_are_classified_by_family() {
        assert_eq!(
            scope_from_stack(&stack(&["source.rust", "comment.line"])),
            Code::Comment
        );
        assert_eq!(
            scope_from_stack(&stack(&["source.rust", "comment.block"])),
            Code::Comment
        );
        assert_eq!(
            scope_from_stack(&stack(&["source.rust", "string.quoted.double"])),
            Code::String
        );
        assert_eq!(
            scope_from_stack(&stack(&["source.rust", "keyword.control"])),
            Code::Keyword
        );
        assert_eq!(
            scope_from_stack(&stack(&["source.rust", "keyword.operator"])),
            Code::Keyword
        );
        assert_eq!(
            scope_from_stack(&stack(&["source.rust", "storage.type"])),
            Code::Keyword
        );
        assert_eq!(
            scope_from_stack(&stack(&["source.rust", "constant.numeric"])),
            Code::Constant
        );
        assert_eq!(
            scope_from_stack(&stack(&["source.rust", "entity.name.function"])),
            Code::Function
        );
        assert_eq!(
            scope_from_stack(&stack(&["source.rust", "entity.name.type"])),
            Code::Type
        );
        assert_eq!(
            scope_from_stack(&stack(&["source.rust", "entity.other.inherited-class"])),
            Code::Type
        );
        assert_eq!(
            scope_from_stack(&stack(&["source.rust", "variable.parameter"])),
            Code::Variable
        );
        assert_eq!(
            scope_from_stack(&stack(&["source.rust", "support.function.builtin"])),
            Code::Support
        );
        assert_eq!(
            scope_from_stack(&stack(&["source.rust", "punctuation.definition"])),
            Code::Punctuation
        );
        assert_eq!(
            scope_from_stack(&stack(&["source.rust", "invalid.illegal"])),
            Code::Invalid
        );
    }

    #[test]
    fn the_most_specific_scope_wins() {
        // An escape sequence inside a string is a constant, not a string.
        // 字符串内的转义序列是常量，而不是字符串。
        let names = [
            "source.rust",
            "string.quoted.double",
            "constant.character.escape",
        ];

        assert_eq!(scope_from_stack(&stack(&names)), Code::Constant);

        // A comment marker is punctuation, even inside a comment.
        // 注释标记属于标点，即使它位于注释内部。
        let names = [
            "source.rust",
            "comment.line",
            "punctuation.definition.comment",
        ];

        assert_eq!(scope_from_stack(&stack(&names)), Code::Punctuation);
    }

    #[test]
    fn the_walk_continues_to_shallower_scopes() {
        // The leaf scope is unmatched, so the classification falls back
        // to the next scope in the stack.
        // 叶子作用域没有匹配，因此分类会回退到
        // 栈中的下一个作用域。
        let names = ["source.rust", "string.quoted.double", "meta.embedded"];

        assert_eq!(scope_from_stack(&stack(&names)), Code::String);
    }

    #[test]
    fn unmatched_scopes_are_other() {
        assert_eq!(scope_from_stack(&[]), Code::Other);
        assert_eq!(
            scope_from_stack(&stack(&["source.rust", "meta.function"])),
            Code::Other
        );

        // A family must match whole scope atoms: `stringify` is not a
        // `string`.
        // 族必须匹配完整的作用域原子：`stringify` 不是
        // `string`。
        assert_eq!(
            scope_from_stack(&stack(&["source.rust", "stringify.call"])),
            Code::Other
        );
    }
}
