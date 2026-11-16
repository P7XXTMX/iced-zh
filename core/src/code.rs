use crate::Theme;
use crate::font;
use crate::text::highlighter;

/// 代码中的一个特定区域。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A specific region of code.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Code {
    /// 注释。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A comment.
    ///
    /// </details>
    Comment,
    /// 字符串或字符字面量。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A string or character literal.
    ///
    /// </details>
    String,
    /// 关键字或存储类关键字。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A keyword or storage word.
    ///
    /// </details>
    Keyword,
    /// 常量、数值或布尔字面量。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A constant, numeric, or boolean literal.
    ///
    /// </details>
    Constant,
    /// 函数名或方法名。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A function or method name.
    ///
    /// </details>
    Function,
    /// 类型名、类名或标签名。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A type, class, or tag name.
    ///
    /// </details>
    Type,
    /// 变量。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A variable.
    ///
    /// </details>
    Variable,
    /// 内置符号或支持库符号。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A built-in or support symbol.
    ///
    /// </details>
    Support,
    /// 标点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Punctuation.
    ///
    /// </details>
    Punctuation,
    /// 路径的组成部分。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A path component.
    ///
    /// </details>
    Path,
    /// 无效或错误的结构。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An invalid or erroneous construct.
    ///
    /// </details>
    Invalid,
    /// 不属于其他任何类别的内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Anything that does not match another class.
    ///
    /// </details>
    Other,
}

impl Code {
    /// 使用给定的 [`Theme`] 高亮这个 [`Code`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Highlights the [`Code`] with the given [`Theme`].
    ///
    /// </details>
    pub fn highlight(self, theme: &Theme) -> highlighter::Style {
        let palette = theme.palette();

        let color = match self {
            Code::Keyword => Some(palette.primary.base.color),
            Code::Type | Code::Path | Code::Function => Some(palette.warning.base.color),

            Code::Variable => Some(palette.danger.base.color),
            Code::Constant => Some(palette.danger.base.color),
            Code::String => Some(palette.success.base.color),
            Code::Support => Some(palette.primary.base.color),

            Code::Punctuation => Some(palette.secondary.strong.color),
            Code::Comment => Some(palette.secondary.base.color),

            Code::Invalid => Some(palette.danger.base.color),
            Code::Other => None,
        };

        highlighter::Style {
            color,
            style: (self == Code::Comment).then_some(font::Style::Italic),
        }
    }
}
