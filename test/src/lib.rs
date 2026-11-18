//! 以无头模式测试你的 `iced` 应用程序。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Test your `iced` applications in headless mode.
//!
//! </details>
//!
//! # 基本用法（Basic Usage）
//! 假设我们想测试[经典的计数器界面][the classical counter interface]。
//!
//! 首先，我们要为界面创建一个 [`Simulator`]：
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Let's assume we want to test [the classical counter interface].
//!
//! First, we will want to create a [`Simulator`] of our interface:
//!
//! </details>
//!
//! ```rust,no_run
//! # struct Counter { value: i64 }
//! # impl Counter {
//! #    pub fn view(&self) -> iced_runtime::core::Element<(), iced_runtime::core::Theme, iced_renderer::Renderer> { unimplemented!() }
//! # }
//! use iced_test::simulator;
//!
//! let mut counter = Counter { value: 0 };
//! let mut ui = simulator(counter.view());
//! ```
//!
//! 现在我们可以模拟用户与界面的交互了。让我们用 [`Simulator::click`] 来点击
//! 计数器的按钮：
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Now we can simulate a user interacting with our interface. Let's use [`Simulator::click`] to click
//! the counter buttons:
//!
//! </details>
//!
//! ```rust,no_run
//! # struct Counter { value: i64 }
//! # impl Counter {
//! #    pub fn view(&self) -> iced_runtime::core::Element<(), iced_runtime::core::Theme, iced_renderer::Renderer> { unimplemented!() }
//! # }
//! # use iced_test::simulator;
//! #
//! # let mut counter = Counter { value: 0 };
//! # let mut ui = simulator(counter.view());
//! #
//! let _ = ui.click("+");
//! let _ = ui.click("+");
//! let _ = ui.click("-");
//! ```
//!
//! [`Simulator::click`] 接受一个实现了 [`Selector`]特质的类型。[`Selector`] 描述了一种查询界面中部件的方式。
//! 这里我们利用了 `&str` 的 [`Selector`] 实现，它根据部件包含的文本来选择部件。
//!
//! 现在我们可以处理这些交互产生的消息，然后断言计数器的最终值
//! 确实是 `1`！
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! [`Simulator::click`] takes a type implementing the [`Selector`] trait. A [`Selector`] describes a way to query the widgets of an interface.
//! In this case, we leverage the [`Selector`] implementation of `&str`, which selects a widget by the text it contains.
//!
//! We can now process any messages produced by these interactions and then assert that the final value of our counter is
//! indeed `1`!
//!
//! </details>
//!
//! ```rust,no_run
//! # struct Counter { value: i64 }
//! # impl Counter {
//! #    pub fn update(&mut self, message: ()) {}
//! #    pub fn view(&self) -> iced_runtime::core::Element<(), iced_runtime::core::Theme, iced_renderer::Renderer> { unimplemented!() }
//! # }
//! # use iced_test::simulator;
//! #
//! # let mut counter = Counter { value: 0 };
//! # let mut ui = simulator(counter.view());
//! #
//! # let _ = ui.click("+");
//! # let _ = ui.click("+");
//! # let _ = ui.click("-");
//! #
//! for message in ui.into_messages() {
//!     counter.update(message);
//! }
//!
//! assert_eq!(counter.value, 1);
//! ```
//!
//! 我们甚至可以重建界面，用 [`Simulator::find`] 确认计数器 *显示* 的值是正确的：
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! We can even rebuild the interface to make sure the counter _displays_ the proper value with [`Simulator::find`]:
//!
//! </details>
//!
//! ```rust,no_run
//! # struct Counter { value: i64 }
//! # impl Counter {
//! #    pub fn view(&self) -> iced_runtime::core::Element<(), iced_runtime::core::Theme, iced_renderer::Renderer> { unimplemented!() }
//! # }
//! # use iced_test::simulator;
//! #
//! # let mut counter = Counter { value: 0 };
//! let mut ui = simulator(counter.view());
//!
//! assert!(ui.find("1").is_ok(), "Counter should display 1!");
//! ```
//!
//! 就是这样！这就是测试 `iced` 应用程序的要点！
//!
//! [`Simulator`] 还包含其他操作，可以用来模拟更多交互，例如 [`tap_key`](Simulator::tap_key) 或
//! [`typewrite`](Simulator::typewrite)，甚至可以进行[*快照测试*](Simulator::snapshot)！
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! And that's it! That's the gist of testing `iced` applications!
//!
//! [`Simulator`] contains additional operations you can use to simulate more interactions—like [`tap_key`](Simulator::tap_key) or
//! [`typewrite`](Simulator::typewrite)—and even perform [_snapshot testing_](Simulator::snapshot)!
//!
//! </details>
//!
//! [the classical counter interface]: https://book.iced.rs/architecture.html#dissecting-an-interface
pub use iced_futures as futures;
pub use iced_program as program;
pub use iced_renderer as renderer;
pub use iced_runtime as runtime;
pub use iced_runtime::core;

pub use iced_selector as selector;

pub mod emulator;
pub mod ice;
pub mod instruction;
pub mod simulator;

mod error;

pub use emulator::Emulator;
pub use error::Error;
pub use ice::Ice;
pub use instruction::Instruction;
pub use selector::Selector;
pub use simulator::{Simulator, simulator};

use crate::core::Size;
use crate::core::time::{Duration, Instant};
use crate::core::window;

use std::path::Path;

/// 为给定的 [`Program`](program::Program) 运行一个 [`Ice`] 测试套件。
///
/// 会从给定目录中解析所有 `.ice` 测试，并在给定
/// [`Program`](program::Program) 的 [`Emulator`] 中执行。
///
/// 请记住，[`Emulator`] 执行的是真实的程序！副作用 *一定会*
/// 发生。你需要自己利用 [`Preset`][program::Preset]
/// 来保证测试环境可复现。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Runs an [`Ice`] test suite for the given [`Program`](program::Program).
///
/// Any `.ice` tests will be parsed from the given directory and executed in
/// an [`Emulator`] of the given [`Program`](program::Program).
///
/// Remember that an [`Emulator`] executes the real thing! Side effects _will_
/// take place. It is up to you to ensure your tests have reproducible environments
/// by leveraging [`Preset`][program::Preset].
///
/// </details>
pub fn run(
    program: impl program::Program + 'static,
    tests_dir: impl AsRef<Path>,
) -> Result<(), Error> {
    use crate::futures::futures::StreamExt;
    use crate::futures::futures::channel::mpsc;
    use crate::futures::futures::executor;

    use std::ffi::OsStr;
    use std::fs;

    let files = fs::read_dir(tests_dir)?;
    let mut tests = Vec::new();

    for file in files {
        let file = file?;

        if file.path().extension().and_then(OsStr::to_str) != Some("ice") {
            continue;
        }

        let content = fs::read_to_string(file.path())?;

        match Ice::parse(&content) {
            Ok(ice) => {
                let preset = if let Some(preset) = &ice.preset {
                    let Some(preset) = program
                        .presets()
                        .iter()
                        .find(|candidate| candidate.name() == preset)
                    else {
                        return Err(Error::PresetNotFound {
                            name: preset.to_owned(),
                            available: program
                                .presets()
                                .iter()
                                .map(program::Preset::name)
                                .map(str::to_owned)
                                .collect(),
                        });
                    };

                    Some(preset)
                } else {
                    None
                };

                tests.push((file, ice, preset));
            }
            Err(error) => {
                return Err(Error::IceParsingFailed {
                    file: file.path().to_path_buf(),
                    error,
                });
            }
        }
    }

    // TODO: Concurrent runtimes
    // 待办：并发的运行时
    for (file, ice, preset) in tests {
        let (sender, mut receiver) = mpsc::channel(1);

        let mut emulator = Emulator::with_preset(
            sender,
            &program,
            ice.mode,
            ice.viewport,
            preset,
        );

        let mut instructions = ice.instructions.into_iter();

        loop {
            let event = executor::block_on(receiver.next())
                .expect("emulator runtime should never stop on its own");

            match event {
                emulator::Event::Action(action) => {
                    emulator.perform(&program, action);
                }
                emulator::Event::Failed(instruction) => {
                    return Err(Error::IceTestingFailed {
                        file: file.path().to_path_buf(),
                        instruction,
                    });
                }
                emulator::Event::Ready => {
                    let Some(instruction) = instructions.next() else {
                        break;
                    };

                    emulator.run(&program, instruction);
                }
            }
        }
    }

    Ok(())
}

/// 让给定的 [`Program`](program::Program) 运行给定的 [`Duration`] 之后，
/// 按给定的主题、视口和缩放因子为它截图。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Takes a screenshot of the given [`Program`](program::Program) with the given theme, viewport,
/// and scale factor after running it for the given [`Duration`].
///
/// </details>
pub fn screenshot<P: program::Program + 'static>(
    program: &P,
    theme: &P::Theme,
    viewport: impl Into<Size>,
    scale_factor: f32,
    duration: Duration,
) -> window::Screenshot {
    use crate::runtime::futures::futures::channel::mpsc;

    let (sender, mut receiver) = mpsc::channel(100);

    let mut emulator = Emulator::new(
        sender,
        program,
        emulator::Mode::Immediate,
        viewport.into(),
    );

    let start = Instant::now();

    loop {
        if let Some(event) = receiver.try_next().ok().flatten() {
            match event {
                emulator::Event::Action(action) => {
                    emulator.perform(program, action);
                }
                emulator::Event::Failed(_) => {
                    unreachable!(
                        "no instructions should be executed during a screenshot"
                    );
                }
                emulator::Event::Ready => {}
            }
        }

        if start.elapsed() >= duration {
            break;
        }

        std::thread::sleep(Duration::from_millis(1));
    }

    emulator.screenshot(program, theme, scale_factor)
}
