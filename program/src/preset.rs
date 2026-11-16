use crate::runtime::Task;

use std::borrow::Cow;
use std::fmt;

/// [`Program`](crate::Program) 的一种特定启动策略。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A specific boot strategy for a [`Program`](crate::Program).
///
/// </details>
pub struct Preset<State, Message> {
    name: Cow<'static, str>,
    boot: Box<dyn Fn() -> (State, Task<Message>)>,
}

impl<State, Message> Preset<State, Message> {
    /// 使用给定的名称和启动策略创建一个新的 [`Preset`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Preset`] with the given name and boot strategy.
    ///
    /// </details>
    pub fn new(
        name: impl Into<Cow<'static, str>>,
        boot: impl Fn() -> (State, Task<Message>) + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            boot: Box::new(boot),
        }
    }

    /// 返回 [`Preset`] 的名称。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the name of the [`Preset`].
    ///
    /// </details>
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 启动 [`Preset`]，返回 [`Program`](crate::Program) 的初始状态
    /// 以及一个用于并发启动的 [`Task`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Boots the [`Preset`], returning the initial [`Program`](crate::Program) state and
    /// a [`Task`] for concurrent booting.
    ///
    /// </details>
    pub fn boot(&self) -> (State, Task<Message>) {
        (self.boot)()
    }
}

impl<State, Message> fmt::Debug for Preset<State, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Preset")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}
