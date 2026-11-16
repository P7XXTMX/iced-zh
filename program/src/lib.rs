//! iced 程序的定义。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! The definition of an iced program.
//!
//! </details>
pub use iced_graphics as graphics;
pub use iced_runtime as runtime;
pub use iced_runtime::core;
pub use iced_runtime::futures;

pub mod message;

mod preset;

pub use preset::Preset;

use crate::core::renderer;
use crate::core::text;
use crate::core::theme;
use crate::core::window;
use crate::core::{Settings, Widget};
use crate::futures::{Executor, Subscription};
use crate::graphics::compositor;
use crate::runtime::Task;

/// 一个可交互的、原生的、跨平台的多窗口应用程序。
///
/// [`Program`] 可以通过在某些方法中返回 [`Task`]
/// 来执行异步操作。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An interactive, native, cross-platform, multi-windowed application.
///
/// A [`Program`] can execute asynchronous actions by returning a
/// [`Task`] in some of its methods.
///
/// </details>
#[allow(missing_docs)]
pub trait Program: Sized {
    /// 程序的状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The state of the program.
    ///
    /// </details>
    type State;

    /// 程序的消息。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The message of the program.
    ///
    /// </details>
    type Message: Send + 'static;

    /// 程序的主题。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The theme of the program.
    ///
    /// </details>
    type Theme: theme::Base + 'static;

    /// 程序的渲染器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The renderer of the program.
    ///
    /// </details>
    type Renderer: Renderer + 'static;

    /// 程序的执行器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The executor of the program.
    ///
    /// </details>
    type Executor: Executor;

    /// 返回 [`Program`] 的唯一名称。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the unique name of the [`Program`].
    ///
    /// </details>
    fn name() -> &'static str;

    fn settings(&self) -> Settings;

    fn window(&self) -> Option<window::Settings>;

    fn boot(&self) -> (Self::State, Task<Self::Message>);

    fn update(&self, state: &mut Self::State, message: Self::Message) -> Task<Self::Message>;

    fn view<'a>(
        &self,
        state: &'a Self::State,
        window: window::Id,
    ) -> impl Widget<Self::Message, Self::Theme, Self::Renderer> + 'a;

    fn title(&self, _state: &Self::State, _window: window::Id) -> String {
        let mut title = String::new();

        for (i, part) in Self::name().split("_").enumerate() {
            use std::borrow::Cow;

            let part = match part {
                "a" | "an" | "of" | "in" | "and" => Cow::Borrowed(part),
                _ => {
                    let mut part = part.to_owned();

                    if let Some(first_letter) = part.get_mut(0..1) {
                        first_letter.make_ascii_uppercase();
                    }

                    Cow::Owned(part)
                }
            };

            if i > 0 {
                title.push(' ');
            }

            title.push_str(&part);
        }

        format!("{title} - Iced")
    }

    fn subscription(&self, _state: &Self::State) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn theme(&self, _state: &Self::State, _window: window::Id) -> Option<Self::Theme> {
        None
    }

    fn style(&self, _state: &Self::State, theme: &Self::Theme) -> theme::Style {
        theme::Base::base(theme)
    }

    fn scale_factor(&self, _state: &Self::State, _window: window::Id) -> f32 {
        1.0
    }

    fn presets(&self) -> &[Preset<Self::State, Self::Message>] {
        &[]
    }
}

/// 使用给定的订阅函数装饰一个 [`Program`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Decorates a [`Program`] with the given subscription function.
///
/// </details>
pub fn with_subscription<P: Program>(
    program: P,
    f: impl Fn(&P::State) -> Subscription<P::Message>,
) -> impl Program<State = P::State, Message = P::Message, Theme = P::Theme> {
    struct WithSubscription<P, F> {
        program: P,
        subscription: F,
    }

    impl<P: Program, F> Program for WithSubscription<P, F>
    where
        F: Fn(&P::State) -> Subscription<P::Message>,
    {
        type State = P::State;
        type Message = P::Message;
        type Theme = P::Theme;
        type Renderer = P::Renderer;
        type Executor = P::Executor;

        #[inline]
        fn subscription(&self, state: &Self::State) -> Subscription<Self::Message> {
            (self.subscription)(state)
        }

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
            self.program.update(state, message)
        }

        #[inline]
        fn view<'a>(
            &self,
            state: &'a Self::State,
            window: window::Id,
        ) -> impl Widget<Self::Message, Self::Theme, Self::Renderer> + 'a {
            self.program.view(state, window)
        }

        #[inline]
        fn title(&self, state: &Self::State, window: window::Id) -> String {
            self.program.title(state, window)
        }

        #[inline]
        fn theme(&self, state: &Self::State, window: window::Id) -> Option<Self::Theme> {
            self.program.theme(state, window)
        }

        #[inline]
        fn style(&self, state: &Self::State, theme: &Self::Theme) -> theme::Style {
            self.program.style(state, theme)
        }

        #[inline]
        fn scale_factor(&self, state: &Self::State, window: window::Id) -> f32 {
            self.program.scale_factor(state, window)
        }
    }

    WithSubscription {
        program,
        subscription: f,
    }
}

/// 使用给定的样式函数装饰一个 [`Program`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Decorates a [`Program`] with the given style function.
///
/// </details>
pub fn with_style<P: Program>(
    program: P,
    f: impl Fn(&P::State, &P::Theme) -> theme::Style,
) -> impl Program<State = P::State, Message = P::Message, Theme = P::Theme> {
    struct WithStyle<P, F> {
        program: P,
        style: F,
    }

    impl<P: Program, F> Program for WithStyle<P, F>
    where
        F: Fn(&P::State, &P::Theme) -> theme::Style,
    {
        type State = P::State;
        type Message = P::Message;
        type Theme = P::Theme;
        type Renderer = P::Renderer;
        type Executor = P::Executor;

        #[inline]
        fn style(&self, state: &Self::State, theme: &Self::Theme) -> theme::Style {
            (self.style)(state, theme)
        }

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
        fn title(&self, state: &Self::State, window: window::Id) -> String {
            self.program.title(state, window)
        }

        #[inline]
        fn update(&self, state: &mut Self::State, message: Self::Message) -> Task<Self::Message> {
            self.program.update(state, message)
        }

        #[inline]
        fn view<'a>(
            &self,
            state: &'a Self::State,
            window: window::Id,
        ) -> impl Widget<Self::Message, Self::Theme, Self::Renderer> + 'a {
            self.program.view(state, window)
        }

        #[inline]
        fn subscription(&self, state: &Self::State) -> Subscription<Self::Message> {
            self.program.subscription(state)
        }

        #[inline]
        fn theme(&self, state: &Self::State, window: window::Id) -> Option<Self::Theme> {
            self.program.theme(state, window)
        }

        #[inline]
        fn scale_factor(&self, state: &Self::State, window: window::Id) -> f32 {
            self.program.scale_factor(state, window)
        }
    }

    WithStyle { program, style: f }
}

/// 使用给定的执行器函数装饰一个 [`Program`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Decorates a [`Program`] with the given executor function.
///
/// </details>
pub fn with_executor<P: Program, E: Executor>(
    program: P,
) -> impl Program<State = P::State, Message = P::Message, Theme = P::Theme> {
    use std::marker::PhantomData;

    struct WithExecutor<P, E> {
        program: P,
        executor: PhantomData<E>,
    }

    impl<P: Program, E> Program for WithExecutor<P, E>
    where
        E: Executor,
    {
        type State = P::State;
        type Message = P::Message;
        type Theme = P::Theme;
        type Renderer = P::Renderer;
        type Executor = E;

        #[inline]
        fn title(&self, state: &Self::State, window: window::Id) -> String {
            self.program.title(state, window)
        }

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
            self.program.update(state, message)
        }

        #[inline]
        fn view<'a>(
            &self,
            state: &'a Self::State,
            window: window::Id,
        ) -> impl Widget<Self::Message, Self::Theme, Self::Renderer> + 'a {
            self.program.view(state, window)
        }

        #[inline]
        fn subscription(&self, state: &Self::State) -> Subscription<Self::Message> {
            self.program.subscription(state)
        }

        #[inline]
        fn theme(&self, state: &Self::State, window: window::Id) -> Option<Self::Theme> {
            self.program.theme(state, window)
        }

        #[inline]
        fn style(&self, state: &Self::State, theme: &Self::Theme) -> theme::Style {
            self.program.style(state, theme)
        }

        #[inline]
        fn scale_factor(&self, state: &Self::State, window: window::Id) -> f32 {
            self.program.scale_factor(state, window)
        }
    }

    WithExecutor {
        program,
        executor: PhantomData::<E>,
    }
}

/// 某个 [`Program`] 的渲染器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The renderer of some [`Program`].
///
/// </details>
pub trait Renderer: text::Renderer + compositor::Default + renderer::Headless {}

impl<T> Renderer for T where T: text::Renderer + compositor::Default + renderer::Headless {}

/// 一个正在运行的 [`Program`] 的特定实例。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A particular instance of a running [`Program`].
///
/// </details>
pub struct Instance<P: Program> {
    program: P,
    state: P::State,
}

impl<P: Program> Instance<P> {
    /// 为给定的 [`Program`] 创建一个新的 [`Instance`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Instance`] of the given [`Program`].
    ///
    /// </details>
    pub fn new(program: P) -> (Self, Task<P::Message>) {
        let (state, task) = program.boot();

        (Self { program, state }, task)
    }

    /// 返回 [`Instance`] 当前的标题。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current title of the [`Instance`].
    ///
    /// </details>
    #[inline]
    pub fn title(&self, window: window::Id) -> String {
        self.program.title(&self.state, window)
    }

    /// 处理给定的消息并更新 [`Instance`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Processes the given message and updates the [`Instance`].
    ///
    /// </details>
    #[inline]
    pub fn update(&mut self, message: P::Message) -> Task<P::Message> {
        self.program.update(&mut self.state, message)
    }

    /// 生成 [`Instance`] 当前的部件树。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Produces the current widget tree of the [`Instance`].
    ///
    /// </details>
    #[inline]
    pub fn view(&self, window: window::Id) -> impl Widget<P::Message, P::Theme, P::Renderer> {
        self.program.view(&self.state, window)
    }

    /// 返回 [`Instance`] 当前的 [`Subscription`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current [`Subscription`] of the [`Instance`].
    ///
    /// </details>
    #[inline]
    pub fn subscription(&self) -> Subscription<P::Message> {
        self.program.subscription(&self.state)
    }

    /// 返回 [`Instance`] 当前的主题。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current theme of the [`Instance`].
    ///
    /// </details>
    #[inline]
    pub fn theme(&self, window: window::Id) -> Option<P::Theme> {
        self.program.theme(&self.state, window)
    }

    /// 返回 [`Instance`] 当前的 [`theme::Style`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current [`theme::Style`] of the [`Instance`].
    ///
    /// </details>
    #[inline]
    pub fn style(&self, theme: &P::Theme) -> theme::Style {
        self.program.style(&self.state, theme)
    }

    /// 返回 [`Instance`] 当前的缩放因子。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current scale factor of the [`Instance`].
    ///
    /// </details>
    #[inline]
    pub fn scale_factor(&self, window: window::Id) -> f32 {
        self.program.scale_factor(&self.state, window)
    }
}
