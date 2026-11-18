//! 创建并运行在后台运行的守护进程。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Create and run daemons that run in the background.
//!
//! </details>
use crate::application;
use crate::message;
use crate::program::{self, Program};
use crate::shell;
use crate::theme;
use crate::window;
use crate::{
    Element, Executor, Font, Preset, Result, Settings, Subscription, Task,
    Theme,
};

use iced_debug as debug;

use std::borrow::Cow;

/// 根据启动、更新和视图逻辑创建一个 iced [`Daemon`]。
///
/// [`Daemon`] 默认不会打开窗口，而是静默运行，
/// 直到它的更新逻辑返回一个来自 [`window::open`] 的 [`Task`]。
///
/// 此外，[`Daemon`] 在所有窗口都关闭后也不会停止运行。
/// 要彻底终止一个 [`Daemon`]，必须中断它的进程，或者
/// 让它的更新逻辑产生一个来自 [`exit`] 的 [`Task`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates an iced [`Daemon`] given its boot, update, and view logic.
///
/// A [`Daemon`] will not open a window by default, but will run silently
/// instead until a [`Task`] from [`window::open`] is returned by its update logic.
///
/// Furthermore, a [`Daemon`] will not stop running when all its windows are closed.
/// In order to completely terminate a [`Daemon`], its process must be interrupted or
/// its update logic must produce a [`Task`] from [`exit`].
///
/// </details>
///
/// [`exit`]: crate::exit
pub fn daemon<State, Message, Theme, Renderer>(
    boot: impl application::BootFn<State, Message>,
    update: impl application::UpdateFn<State, Message>,
    view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer>,
) -> Daemon<impl Program<State = State, Message = Message, Theme = Theme>>
where
    State: 'static,
    Message: Send + 'static,
    Theme: theme::Base,
    Renderer: program::Renderer,
{
    use std::marker::PhantomData;

    struct Instance<State, Message, Theme, Renderer, Boot, Update, View> {
        boot: Boot,
        update: Update,
        view: View,
        _state: PhantomData<State>,
        _message: PhantomData<Message>,
        _theme: PhantomData<Theme>,
        _renderer: PhantomData<Renderer>,
    }

    impl<State, Message, Theme, Renderer, Boot, Update, View> Program
        for Instance<State, Message, Theme, Renderer, Boot, Update, View>
    where
        Message: Send + 'static,
        Theme: theme::Base,
        Renderer: program::Renderer,
        Boot: application::BootFn<State, Message>,
        Update: application::UpdateFn<State, Message>,
        View: for<'a> self::ViewFn<'a, State, Message, Theme, Renderer>,
    {
        type State = State;
        type Message = Message;
        type Theme = Theme;
        type Renderer = Renderer;
        type Executor = iced_futures::backend::default::Executor;

        fn name() -> &'static str {
            let name = std::any::type_name::<State>();

            name.split("::").next().unwrap_or("a_cool_daemon")
        }

        fn settings(&self) -> Settings {
            Settings::default()
        }

        fn window(&self) -> Option<iced_core::window::Settings> {
            None
        }

        fn boot(&self) -> (Self::State, Task<Self::Message>) {
            self.boot.boot()
        }

        fn update(
            &self,
            state: &mut Self::State,
            message: Self::Message,
        ) -> Task<Self::Message> {
            self.update.update(state, message)
        }

        fn view<'a>(
            &self,
            state: &'a Self::State,
            window: window::Id,
        ) -> Element<'a, Self::Message, Self::Theme, Self::Renderer> {
            self.view.view(state, window)
        }
    }

    Daemon {
        raw: Instance {
            boot,
            update,
            view,
            _state: PhantomData,
            _message: PhantomData,
            _theme: PhantomData,
            _renderer: PhantomData,
        },
        settings: Settings::default(),
        presets: Vec::new(),
    }
}

/// iced 守护进程的底层定义与配置。
///
/// 你可以用这套 API 一步一步地创建并运行 iced 应用程序，
/// 而不必把你的逻辑与某个特质或某个特定类型
/// 耦合在一起。
///
/// 你可以用 [`daemon`] 辅助函数创建一个 [`Daemon`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The underlying definition and configuration of an iced daemon.
///
/// You can use this API to create and run iced applications
/// step by step—without coupling your logic to a trait
/// or a specific type.
///
/// You can create a [`Daemon`] with the [`daemon`] helper.
///
/// </details>
#[derive(Debug)]
pub struct Daemon<P: Program> {
    raw: P,
    settings: Settings,
    presets: Vec<Preset<P::State, P::Message>>,
}

impl<P: Program> Daemon<P> {
    /// 运行 [`Daemon`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Runs the [`Daemon`].
    ///
    /// </details>
    pub fn run(self) -> Result
    where
        Self: 'static,
        P::Message: message::MaybeDebug + message::MaybeClone,
    {
        #[cfg(feature = "debug")]
        iced_debug::init(iced_debug::Metadata {
            name: P::name(),
            theme: None,
            can_time_travel: cfg!(feature = "time-travel"),
        });

        #[cfg(feature = "tester")]
        let program = iced_tester::attach(self);

        #[cfg(all(feature = "debug", not(feature = "tester")))]
        let program = iced_devtools::attach(self);

        #[cfg(not(any(feature = "tester", feature = "debug")))]
        let program = self;

        Ok(shell::run(program)?)
    }

    /// 设置运行 [`Daemon`] 时使用的 [`Settings`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Settings`] that will be used to run the [`Daemon`].
    ///
    /// </details>
    pub fn settings(self, settings: Settings) -> Self {
        Self { settings, ..self }
    }

    /// 设置 [`Daemon`] 的 [`Settings::antialiasing`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Settings::antialiasing`] of the [`Daemon`].
    ///
    /// </details>
    pub fn antialiasing(self, antialiasing: bool) -> Self {
        Self {
            settings: Settings {
                antialiasing,
                ..self.settings
            },
            ..self
        }
    }

    /// 设置 [`Daemon`] 的默认 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the default [`Font`] of the [`Daemon`].
    ///
    /// </details>
    pub fn default_font(self, default_font: Font) -> Self {
        Self {
            settings: Settings {
                default_font,
                ..self.settings
            },
            ..self
        }
    }

    /// 向 [`Daemon`] 启动时要加载的字体列表中添加一个字体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adds a font to the list of fonts that will be loaded at the start of the [`Daemon`].
    ///
    /// </details>
    pub fn font(mut self, font: impl Into<Cow<'static, [u8]>>) -> Self {
        self.settings.fonts.push(font.into());
        self
    }

    /// 设置 [`Daemon`] 的标题。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the title of the [`Daemon`].
    ///
    /// </details>
    pub fn title(
        self,
        title: impl TitleFn<P::State>,
    ) -> Daemon<
        impl Program<State = P::State, Message = P::Message, Theme = P::Theme>,
    > {
        Daemon {
            raw: program::with_title(self.raw, move |state, window| {
                title.title(state, window)
            }),
            settings: self.settings,
            presets: self.presets,
        }
    }

    /// 设置 [`Daemon`] 的订阅逻辑。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the subscription logic of the [`Daemon`].
    ///
    /// </details>
    pub fn subscription(
        self,
        f: impl Fn(&P::State) -> Subscription<P::Message>,
    ) -> Daemon<
        impl Program<State = P::State, Message = P::Message, Theme = P::Theme>,
    > {
        Daemon {
            raw: program::with_subscription(self.raw, f),
            settings: self.settings,
            presets: self.presets,
        }
    }

    /// 设置 [`Daemon`] 的主题逻辑。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the theme logic of the [`Daemon`].
    ///
    /// </details>
    pub fn theme(
        self,
        f: impl ThemeFn<P::State, P::Theme>,
    ) -> Daemon<
        impl Program<State = P::State, Message = P::Message, Theme = P::Theme>,
    > {
        Daemon {
            raw: program::with_theme(self.raw, move |state, window| {
                f.theme(state, window)
            }),
            settings: self.settings,
            presets: self.presets,
        }
    }

    /// 设置 [`Daemon`] 的样式逻辑。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style logic of the [`Daemon`].
    ///
    /// </details>
    pub fn style(
        self,
        f: impl Fn(&P::State, &P::Theme) -> theme::Style,
    ) -> Daemon<
        impl Program<State = P::State, Message = P::Message, Theme = P::Theme>,
    > {
        Daemon {
            raw: program::with_style(self.raw, f),
            settings: self.settings,
            presets: self.presets,
        }
    }

    /// 设置 [`Daemon`] 的缩放因子。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the scale factor of the [`Daemon`].
    ///
    /// </details>
    pub fn scale_factor(
        self,
        f: impl Fn(&P::State, window::Id) -> f32,
    ) -> Daemon<
        impl Program<State = P::State, Message = P::Message, Theme = P::Theme>,
    > {
        Daemon {
            raw: program::with_scale_factor(self.raw, f),
            settings: self.settings,
            presets: self.presets,
        }
    }

    /// 设置 [`Daemon`] 的执行器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the executor of the [`Daemon`].
    ///
    /// </details>
    pub fn executor<E>(
        self,
    ) -> Daemon<
        impl Program<State = P::State, Message = P::Message, Theme = P::Theme>,
    >
    where
        E: Executor,
    {
        Daemon {
            raw: program::with_executor::<P, E>(self.raw),
            settings: self.settings,
            presets: self.presets,
        }
    }

    /// 设置 [`Daemon`] 的启动预设。
    ///
    /// 在测试时，可以用预设覆盖应用程序默认的
    /// 启动策略，从而创建可复现的
    /// 环境。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the boot presets of the [`Daemon`].
    ///
    /// Presets can be used to override the default booting strategy
    /// of your application during testing to create reproducible
    /// environments.
    ///
    /// </details>
    pub fn presets(
        self,
        presets: impl IntoIterator<Item = Preset<P::State, P::Message>>,
    ) -> Self {
        Self {
            presets: presets.into_iter().collect(),
            ..self
        }
    }
}

impl<P: Program> Program for Daemon<P> {
    type State = P::State;
    type Message = P::Message;
    type Theme = P::Theme;
    type Renderer = P::Renderer;
    type Executor = P::Executor;

    fn name() -> &'static str {
        P::name()
    }

    fn settings(&self) -> Settings {
        self.settings.clone()
    }

    fn window(&self) -> Option<window::Settings> {
        None
    }

    fn boot(&self) -> (Self::State, Task<Self::Message>) {
        self.raw.boot()
    }

    fn update(
        &self,
        state: &mut Self::State,
        message: Self::Message,
    ) -> Task<Self::Message> {
        debug::hot(|| self.raw.update(state, message))
    }

    fn view<'a>(
        &self,
        state: &'a Self::State,
        window: window::Id,
    ) -> Element<'a, Self::Message, Self::Theme, Self::Renderer> {
        debug::hot(|| self.raw.view(state, window))
    }

    fn title(&self, state: &Self::State, window: window::Id) -> String {
        debug::hot(|| self.raw.title(state, window))
    }

    fn subscription(&self, state: &Self::State) -> Subscription<Self::Message> {
        debug::hot(|| self.raw.subscription(state))
    }

    fn theme(
        &self,
        state: &Self::State,
        window: iced_core::window::Id,
    ) -> Option<Self::Theme> {
        debug::hot(|| self.raw.theme(state, window))
    }

    fn style(&self, state: &Self::State, theme: &Self::Theme) -> theme::Style {
        debug::hot(|| self.raw.style(state, theme))
    }

    fn scale_factor(&self, state: &Self::State, window: window::Id) -> f32 {
        debug::hot(|| self.raw.scale_factor(state, window))
    }

    fn presets(&self) -> &[Preset<Self::State, Self::Message>] {
        &self.presets
    }
}

/// 某个 [`Daemon`] 的标题逻辑。
///
/// `&static str` 和任何 `Fn(&State, window::Id) -> String` 闭包
/// 都实现了这个特质。
///
/// 这个特质让 [`daemon`] 构建器可以接受其中任何一种。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The title logic of some [`Daemon`].
///
/// This trait is implemented both for `&static str` and
/// any closure `Fn(&State, window::Id) -> String`.
///
/// This trait allows the [`daemon`] builder to take any of them.
///
/// </details>
pub trait TitleFn<State> {
    /// 生成 [`Daemon`] 的标题。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Produces the title of the [`Daemon`].
    ///
    /// </details>
    fn title(&self, state: &State, window: window::Id) -> String;
}

impl<State> TitleFn<State> for &'static str {
    fn title(&self, _state: &State, _window: window::Id) -> String {
        self.to_string()
    }
}

impl<T, State> TitleFn<State> for T
where
    T: Fn(&State, window::Id) -> String,
{
    fn title(&self, state: &State, window: window::Id) -> String {
        self(state, window)
    }
}

/// 某个 [`Daemon`] 的视图逻辑。
///
/// 这个特质让 [`daemon`] 构建器可以接受任何返回
/// `Into<Element<'_, Message>>` 的闭包。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The view logic of some [`Daemon`].
///
/// This trait allows the [`daemon`] builder to take any closure that
/// returns any `Into<Element<'_, Message>>`.
///
/// </details>
pub trait ViewFn<'a, State, Message, Theme, Renderer> {
    /// 生成 [`Daemon`] 的部件。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Produces the widget of the [`Daemon`].
    ///
    /// </details>
    fn view(
        &self,
        state: &'a State,
        window: window::Id,
    ) -> Element<'a, Message, Theme, Renderer>;
}

impl<'a, T, State, Message, Theme, Renderer, Widget>
    ViewFn<'a, State, Message, Theme, Renderer> for T
where
    T: Fn(&'a State, window::Id) -> Widget,
    State: 'static,
    Widget: Into<Element<'a, Message, Theme, Renderer>>,
{
    fn view(
        &self,
        state: &'a State,
        window: window::Id,
    ) -> Element<'a, Message, Theme, Renderer> {
        self(state, window).into()
    }
}

/// 某个 [`Daemon`] 的主题逻辑。
///
/// 任何实现了这个特质的类型都可以作为参数传给
/// [`Daemon::theme`]。
///
/// `iced` 提供了两种实现：
/// - 内置的 [`Theme`] 本身
/// - 以及任何 `Fn(&State, window::Id) -> impl Into<Option<Theme>>`。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The theme logic of some [`Daemon`].
///
/// Any implementors of this trait can be provided as an argument to
/// [`Daemon::theme`].
///
/// `iced` provides two implementors:
/// - the built-in [`Theme`] itself
/// - and any `Fn(&State, window::Id) -> impl Into<Option<Theme>>`.
///
/// </details>
pub trait ThemeFn<State, Theme> {
    /// 返回 [`Daemon`] 在当前状态和窗口下的主题。
    ///
    /// 如果返回 `None`，`iced` 会尝试使用与系统配色方案
    /// 相匹配的主题。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the theme of the [`Daemon`] for the current state and window.
    ///
    /// If `None` is returned, `iced` will try to use a theme that
    /// matches the system color scheme.
    ///
    /// </details>
    fn theme(&self, state: &State, window: window::Id) -> Option<Theme>;
}

impl<State> ThemeFn<State, Theme> for Theme {
    fn theme(&self, _state: &State, _window: window::Id) -> Option<Theme> {
        Some(self.clone())
    }
}

impl<F, T, State, Theme> ThemeFn<State, Theme> for F
where
    F: Fn(&State, window::Id) -> T,
    T: Into<Option<Theme>>,
{
    fn theme(&self, state: &State, window: window::Id) -> Option<Theme> {
        (self)(state, window).into()
    }
}
