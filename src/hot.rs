//! 为 [`Program`] 添加热重载支持的装饰器。
//!
//! 这个模块仅在启用 `hot` 特性时可用。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Decorates a [`Program`] with hot reloading support.
//!
//! This module is only available when the `hot` feature is enabled.
//!
//! </details>
use crate::core::Widget;
use crate::program::Program;
use crate::theme;
use crate::window;
use crate::{Preset, Settings, Subscription, Task};

use iced_debug as debug;

/// 一个 [`Program`] 装饰器，使内部的程序可以热重载。
///
/// 程序的所有逻辑都被包装在 [`iced_debug::hot`] 中，
/// 因此可以在程序运行时进行热补丁。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A [`Program`] decorator that makes the inner program hot-reloadable.
///
/// All the logic of the program is wrapped in [`iced_debug::hot`],
/// so that it can be hot-patched while the program is running.
///
/// </details>
pub struct Hot<P: Program> {
    program: P,
}

impl<P: Program> Hot<P> {
    /// 为给定的 [`Program`] 包装上热重载支持。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Wraps the given [`Program`] with hot reloading support.
    ///
    /// </details>
    pub fn new(program: P) -> Self {
        Self { program }
    }
}

impl<P: Program> Program for Hot<P> {
    type State = P::State;
    type Message = P::Message;
    type Theme = P::Theme;
    type Renderer = P::Renderer;
    type Executor = P::Executor;

    #[inline]
    fn name() -> &'static str {
        P::name()
    }

    #[inline]
    fn settings(&self) -> Settings {
        self.program.settings()
    }

    #[inline]
    fn window(&self) -> Option<window::Settings> {
        self.program.window()
    }

    #[inline]
    fn boot(&self) -> (Self::State, Task<Self::Message>) {
        self.program.boot()
    }

    #[inline]
    fn update(&self, state: &mut Self::State, message: Self::Message) -> Task<Self::Message> {
        debug::hot(|| self.program.update(state, message))
    }

    #[inline]
    fn view<'a>(
        &self,
        state: &'a Self::State,
        window: window::Id,
    ) -> impl Widget<Self::Message, Self::Theme, Self::Renderer> + 'a {
        debug::hot(|| self.program.view(state, window))
    }

    #[inline]
    fn title(&self, state: &Self::State, window: window::Id) -> String {
        debug::hot(|| self.program.title(state, window))
    }

    #[inline]
    fn subscription(&self, state: &Self::State) -> Subscription<Self::Message> {
        debug::hot(|| self.program.subscription(state))
    }

    #[inline]
    fn theme(&self, state: &Self::State, window: window::Id) -> Option<Self::Theme> {
        debug::hot(|| self.program.theme(state, window))
    }

    #[inline]
    fn style(&self, state: &Self::State, theme: &Self::Theme) -> theme::Style {
        debug::hot(|| self.program.style(state, theme))
    }

    #[inline]
    fn scale_factor(&self, state: &Self::State, window: window::Id) -> f32 {
        debug::hot(|| self.program.scale_factor(state, window))
    }

    #[inline]
    fn presets(&self) -> &[Preset<Self::State, Self::Message>] {
        self.program.presets()
    }
}
