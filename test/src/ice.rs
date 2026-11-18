//! 一种可分享的、简单的端到端测试格式。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! A shareable, simple format of end-to-end tests.
//!
//! </details>
use crate::Instruction;
use crate::core::Size;
use crate::emulator;
use crate::instruction;

/// iced 应用程序的端到端测试。
///
/// ice 测试把某个配置和一个指令序列编码在一起。
/// 如果所有指令都能成功执行，ice 测试就通过。
///
/// 通常，ice 测试由 [`Emulator`](crate::Emulator) 在持续
/// 集成流水线中运行。
///
/// 要运行 ice 测试很简单：把它们保存为某个文件夹里的 `.ice` 文件，然后
/// 调用 [`run`](crate::run) 即可。启用根 crate 的 `tester`
/// 特性标志后，就可以录制这些测试文件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An end-to-end test for iced applications.
///
/// Ice tests encode a certain configuration together with a sequence of instructions.
/// An ice test passes if all the instructions can be executed successfully.
///
/// Normally, ice tests are run by an [`Emulator`](crate::Emulator) in continuous
/// integration pipelines.
///
/// Ice tests can be easily run by saving them as `.ice` files in a folder and simply
/// calling [`run`](crate::run). These test files can be recorded by enabling the `tester`
/// feature flag in the root crate.
///
/// </details>
#[derive(Debug, Clone, PartialEq)]
pub struct Ice {
    /// 测试必须使用的视口 [`Size`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The viewport [`Size`] that must be used for the test.
    ///
    /// </details>
    pub viewport: Size,
    /// 测试必须使用的 [`emulator::Mode`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`emulator::Mode`] that must be used for the test.
    ///
    /// </details>
    pub mode: emulator::Mode,
    /// 测试必须使用的 [`Preset`](crate::program::Preset) 的名称。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The name of the [`Preset`](crate::program::Preset) that must be used for the test.
    ///
    /// </details>
    pub preset: Option<String>,
    /// 测试的指令序列。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The sequence of instructions of the test.
    ///
    /// </details>
    pub instructions: Vec<Instruction>,
}

impl Ice {
    /// 从文本表示解析出一个 [`Ice`] 测试。
    ///
    /// 下面是 [`Ice`] 测试语法的一个示例：
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Parses an [`Ice`] test from its textual representation.
    ///
    /// Here is an example of the [`Ice`] test syntax:
    ///
    /// </details>
    ///
    /// ```text
    /// viewport: 500x800
    /// mode: Immediate
    /// preset: Empty
    /// -----
    /// click "What needs to be done?"
    /// type "Create the universe"
    /// type enter
    /// type "Make an apple pie"
    /// type enter
    /// expect "2 tasks left"
    /// click "Create the universe"
    /// expect "1 task left"
    /// click "Make an apple pie"
    /// expect "0 tasks left"
    /// ```
    ///
    /// 这套语法 *非常* 实验性，极有可能经常变动。
    /// 因此，它只留给想要提前试用的高级用户。
    ///
    /// 目前，要使用它，你需要通过阅读代码来赢得这个资格，
    /// 并证明你理解它的实验性质！
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// This syntax is _very_ experimental and extremely likely to change often.
    /// For this reason, it is reserved for advanced users that want to early test it.
    ///
    /// Currently, in order to use it, you will need to earn the right and prove you understand
    /// its experimental nature by reading the code!
    ///
    /// </details>
    pub fn parse(content: &str) -> Result<Self, ParseError> {
        let Some((metadata, rest)) = content.split_once("-") else {
            return Err(ParseError::NoMetadata);
        };

        let mut viewport = None;
        let mut mode = None;
        let mut preset = None;

        for (i, line) in metadata.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }

            let Some((field, value)) = line.split_once(':') else {
                return Err(ParseError::InvalidMetadata {
                    line: i,
                    content: line.to_owned(),
                });
            };

            match field.trim() {
                "viewport" => {
                    viewport = Some(
                        if let Some((width, height)) =
                            value.trim().split_once('x')
                            && let Ok(width) = width.parse()
                            && let Ok(height) = height.parse()
                        {
                            Size::new(width, height)
                        } else {
                            return Err(ParseError::InvalidViewport {
                                line: i,
                                value: value.to_owned(),
                            });
                        },
                    );
                }
                "mode" => {
                    mode = Some(match value.trim().to_lowercase().as_str() {
                        "zen" => emulator::Mode::Zen,
                        "patient" => emulator::Mode::Patient,
                        "immediate" => emulator::Mode::Immediate,
                        _ => {
                            return Err(ParseError::InvalidMode {
                                line: i,
                                value: value.to_owned(),
                            });
                        }
                    });
                }
                "preset" => {
                    preset = Some(value.trim().to_owned());
                }
                field => {
                    return Err(ParseError::UnknownField {
                        line: i,
                        field: field.to_owned(),
                    });
                }
            }
        }

        let Some(viewport) = viewport else {
            return Err(ParseError::MissingViewport);
        };

        let Some(mode) = mode else {
            return Err(ParseError::MissingMode);
        };

        let instructions = rest
            .lines()
            .skip(1)
            .enumerate()
            .map(|(i, line)| {
                Instruction::parse(line).map_err(|error| {
                    ParseError::InvalidInstruction {
                        line: metadata.lines().count() + 1 + i,
                        error,
                    }
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            viewport,
            mode,
            preset,
            instructions,
        })
    }
}

impl std::fmt::Display for Ice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "viewport: {width}x{height}",
            width = self.viewport.width as u32, // TODO（待办）
            height = self.viewport.height as u32, // TODO（待办）
        )?;

        writeln!(f, "mode: {}", self.mode)?;

        if let Some(preset) = &self.preset {
            writeln!(f, "preset: {preset}")?;
        }

        f.write_str("-----\n")?;

        for instruction in &self.instructions {
            instruction.fmt(f)?;
            f.write_str("\n")?;
        }

        Ok(())
    }
}

/// [`Ice::parse`] 过程中产生的错误。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An error produced during [`Ice::parse`].
///
/// </details>
#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseError {
    /// 没有元数据。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// No metadata is present.
    ///
    /// </details>
    #[error("the ice test has no metadata")]
    NoMetadata,

    /// 元数据无效。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The metadata is invalid.
    ///
    /// </details>
    #[error("invalid metadata in line {line}: \"{content}\"")]
    InvalidMetadata {
        /// 无效行的行号。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The number of the invalid line.
        ///
        /// </details>
        line: usize,
        /// 无效行的内容。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The content of the invalid line.
        ///
        /// </details>
        content: String,
    },

    /// 视口无效。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The viewport is invalid.
    ///
    /// </details>
    #[error("invalid viewport in line {line}: \"{value}\"")]
    InvalidViewport {
        /// 无效行的行号。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The number of the invalid line.
        ///
        /// </details>
        line: usize,

        /// 无效的值。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The invalid value.
        ///
        /// </details>
        value: String,
    },

    /// [`emulator::Mode`] 无效。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`emulator::Mode`] is invalid.
    ///
    /// </details>
    #[error("invalid mode in line {line}: \"{value}\"")]
    InvalidMode {
        /// 无效行的行号。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The number of the invalid line.
        ///
        /// </details>
        line: usize,
        /// 无效的值。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The invalid value.
        ///
        /// </details>
        value: String,
    },

    /// 某个元数据字段未知。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A metadata field is unknown.
    ///
    /// </details>
    #[error("unknown metadata field in line {line}: \"{field}\"")]
    UnknownField {
        /// 无效行的行号。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The number of the invalid line.
        ///
        /// </details>
        line: usize,
        /// 未知字段的名称。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The name of the unknown field.
        ///
        /// </details>
        field: String,
    },

    /// 缺少视口元数据。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Viewport metadata is missing.
    ///
    /// </details>
    #[error("metadata is missing the viewport field")]
    MissingViewport,

    /// 缺少 [`emulator::Mode`] 元数据。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// [`emulator::Mode`] metadata is missing.
    ///
    /// </details>
    #[error("metadata is missing the mode field")]
    MissingMode,

    /// 某条 [`Instruction`] 解析失败。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An [`Instruction`] failed to parse.
    ///
    /// </details>
    #[error("invalid instruction in line {line}: {error}")]
    InvalidInstruction {
        /// 无效行的行号。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The number of the invalid line.
        ///
        /// </details>
        line: usize,
        /// 解析错误。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The parse error.
        ///
        /// </details>
        error: instruction::ParseError,
    },
}
