//! 在没有副作用的情况下模拟运行你的应用程序。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Run a simulation of your application without side effects.
//!
//! </details>
use crate::core;
use crate::core::clipboard;
use crate::core::event;
use crate::core::keyboard;
use crate::core::mouse;
use crate::core::theme;
use crate::core::time;
use crate::core::widget;
use crate::core::window;
use crate::core::{Element, Event, Font, Point, Settings, Size, SmolStr};
use crate::renderer;
use crate::runtime::UserInterface;
use crate::runtime::user_interface;
use crate::selector::Bounded;
use crate::{Error, Selector};

use std::borrow::Cow;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// 可以通过编程方式交互和检查的用户界面。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A user interface that can be interacted with and inspected programmatically.
///
/// </details>
pub struct Simulator<
    'a,
    Message,
    Theme = core::Theme,
    Renderer = renderer::Renderer,
> {
    raw: UserInterface<'a, Message, Theme, Renderer>,
    renderer: Renderer,
    size: Size,
    cursor: mouse::Cursor,
    messages: Vec<Message>,
}

impl<'a, Message, Theme, Renderer> Simulator<'a, Message, Theme, Renderer>
where
    Theme: theme::Base,
    Renderer: core::Renderer + core::renderer::Headless,
{
    /// 使用默认的 [`Settings`] 和默认大小（1024x768）创建一个新的 [`Simulator`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Simulator`] with default [`Settings`] and a default size (1024x768).
    ///
    /// </details>
    pub fn new(
        element: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self::with_settings(Settings::default(), element)
    }

    /// 使用给定的 [`Settings`] 和默认大小（1024x768）创建一个新的 [`Simulator`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Simulator`] with the given [`Settings`] and a default size (1024x768).
    ///
    /// </details>
    pub fn with_settings(
        settings: Settings,
        element: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self::with_size(settings, window::Settings::default().size, element)
    }

    /// 使用给定的 [`Settings`] 和大小创建一个新的 [`Simulator`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Simulator`] with the given [`Settings`] and size.
    ///
    /// </details>
    pub fn with_size(
        settings: Settings,
        size: impl Into<Size>,
        element: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        let size = size.into();

        let default_font = match settings.default_font {
            Font::DEFAULT => Font::with_name("Fira Sans"),
            _ => settings.default_font,
        };

        for font in settings.fonts {
            load_font(font).expect("Font must be valid");
        }

        let mut renderer = {
            let backend = env::var("ICED_TEST_BACKEND").ok();

            crate::futures::futures::executor::block_on(Renderer::new(
                default_font,
                settings.default_text_size,
                backend.as_deref(),
            ))
            .expect("Create new headless renderer")
        };

        let raw = UserInterface::build(
            element,
            size,
            user_interface::Cache::default(),
            &mut renderer,
        );

        Simulator {
            raw,
            renderer,
            size,
            cursor: mouse::Cursor::Unavailable,
            messages: Vec::new(),
        }
    }

    /// 在 [`Simulator`] 中查找给定部件 [`Selector`] 的目标。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Finds the target of the given widget [`Selector`] in the [`Simulator`].
    ///
    /// </details>
    pub fn find<S>(&mut self, selector: S) -> Result<S::Output, Error>
    where
        S: Selector + Send,
        S::Output: Clone + Send,
    {
        use widget::Operation;

        let description = selector.description();
        let mut operation = selector.find();

        self.raw.operate(
            &self.renderer,
            &mut widget::operation::black_box(&mut operation),
        );

        match operation.finish() {
            widget::operation::Outcome::Some(output) => {
                output.ok_or(Error::SelectorNotFound {
                    selector: description,
                })
            }
            _ => Err(Error::SelectorNotFound {
                selector: description,
            }),
        }
    }

    /// 把鼠标光标指向 [`Simulator`] 中的给定位置。
    ///
    /// 这 *不会* 产生鼠标移动事件！
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Points the mouse cursor at the given position in the [`Simulator`].
    ///
    /// This does _not_ produce mouse movement events!
    ///
    /// </details>
    pub fn point_at(&mut self, position: impl Into<Point>) {
        self.cursor = mouse::Cursor::Available(position.into());
    }

    /// 点击给定 [`Selector`] 找到的 [`Bounded`] 目标（如果有）。
    ///
    /// 这包括：
    /// - 把鼠标光标指向 [`Bounded`] 目标的中心。
    /// - 模拟一次 [`click`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Clicks the [`Bounded`] target found by the given [`Selector`], if any.
    ///
    /// This consists in:
    /// - Pointing the mouse cursor at the center of the [`Bounded`] target.
    /// - Simulating a [`click`].
    ///
    /// </details>
    pub fn click<S>(&mut self, selector: S) -> Result<S::Output, Error>
    where
        S: Selector + Send,
        S::Output: Bounded + Clone + Send + Sync + 'static,
    {
        let target = self.find(selector)?;

        let Some(visible_bounds) = target.visible_bounds() else {
            return Err(Error::TargetNotVisible {
                target: Arc::new(target),
            });
        };

        self.point_at(visible_bounds.center());

        let _ = self.simulate(click());

        Ok(target)
    }

    /// 在 [`Simulator`] 中模拟一次按键按下，随后释放。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Simulates a key press, followed by a release, in the [`Simulator`].
    ///
    /// </details>
    pub fn tap_key(&mut self, key: impl Into<keyboard::Key>) -> event::Status {
        self.simulate(tap_key(key, None))
            .first()
            .copied()
            .unwrap_or(event::Status::Ignored)
    }

    /// 在 [`Simulator`] 中模拟用户用键盘输入给定的文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Simulates a user typing in the keyboard the given text in the [`Simulator`].
    ///
    /// </details>
    pub fn typewrite(&mut self, text: &str) -> event::Status {
        let statuses = self.simulate(typewrite(text));

        statuses
            .into_iter()
            .fold(event::Status::Ignored, event::Status::merge)
    }

    /// 在 [`Simulator`] 中模拟给定的原始事件序列。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Simulates the given raw sequence of events in the [`Simulator`].
    ///
    /// </details>
    pub fn simulate(
        &mut self,
        events: impl IntoIterator<Item = Event>,
    ) -> Vec<event::Status> {
        let events: Vec<Event> = events.into_iter().collect();

        let (_state, statuses) = self.raw.update(
            &events,
            self.cursor,
            &mut self.renderer,
            &mut clipboard::Null,
            &mut self.messages,
        );

        statuses
    }

    /// 绘制 [`Simulator`] 中的界面并为其生成一个 [`Snapshot`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws and takes a [`Snapshot`] of the interface in the [`Simulator`].
    ///
    /// </details>
    pub fn snapshot(&mut self, theme: &Theme) -> Result<Snapshot, Error> {
        let base = theme.base();

        let _ = self.raw.update(
            &[Event::Window(window::Event::RedrawRequested(
                time::Instant::now(),
            ))],
            self.cursor,
            &mut self.renderer,
            &mut clipboard::Null,
            &mut self.messages,
        );

        self.raw.draw(
            &mut self.renderer,
            theme,
            &core::renderer::Style {
                text_color: base.text_color,
            },
            self.cursor,
        );

        let scale_factor = 2.0;

        let physical_size = Size::new(
            (self.size.width * scale_factor).round() as u32,
            (self.size.height * scale_factor).round() as u32,
        );

        let rgba = self.renderer.screenshot(
            physical_size,
            scale_factor,
            base.background_color,
        );

        Ok(Snapshot {
            screenshot: window::Screenshot::new(
                rgba,
                physical_size,
                scale_factor,
            ),
            renderer: self.renderer.name(),
        })
    }

    /// 把 [`Simulator`] 转换为各种交互所产生的消息序列。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Turns the [`Simulator`] into the sequence of messages produced by any interactions.
    ///
    /// </details>
    pub fn into_messages(
        self,
    ) -> impl Iterator<Item = Message> + use<Message, Theme, Renderer> {
        self.messages.into_iter()
    }
}

/// 由 [`Simulator`] 渲染的一帧用户界面。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A frame of a user interface rendered by a [`Simulator`].
///
/// </details>
#[derive(Debug, Clone)]
pub struct Snapshot {
    screenshot: window::Screenshot,
    renderer: String,
}

impl Snapshot {
    /// 把 [`Snapshot`] 与给定路径下的 PNG 图像进行比较，
    /// 如果两者完全相同，返回 `true`。
    ///
    /// 如果 PNG 图像不存在，[`Snapshot`] 会创建它以供将来
    /// 测试使用，并返回 `true`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Compares the [`Snapshot`] with the PNG image found in the given path, returning
    /// `true` if they are identical.
    ///
    /// If the PNG image does not exist, it will be created by the [`Snapshot`] for future
    /// testing and `true` will be returned.
    ///
    /// </details>
    pub fn matches_image(&self, path: impl AsRef<Path>) -> Result<bool, Error> {
        let path = self.path(path, "png");

        if path.exists() {
            let file = fs::File::open(&path)?;
            let decoder = png::Decoder::new(io::BufReader::new(file));

            let mut reader = decoder.read_info()?;
            let n = reader
                .output_buffer_size()
                .expect("snapshot should fit in memory");
            let mut bytes = vec![0; n];
            let info = reader.next_frame(&mut bytes)?;

            Ok(self.screenshot.rgba == bytes[..info.buffer_size()])
        } else {
            if let Some(directory) = path.parent() {
                fs::create_dir_all(directory)?;
            }

            let file = fs::File::create(path)?;

            let mut encoder = png::Encoder::new(
                file,
                self.screenshot.size.width,
                self.screenshot.size.height,
            );
            encoder.set_color(png::ColorType::Rgba);

            let mut writer = encoder.write_header()?;
            writer.write_image_data(&self.screenshot.rgba)?;
            writer.finish()?;

            Ok(true)
        }
    }

    /// 把 [`Snapshot`] 与给定路径下的 SHA-256 哈希文件进行比较，
    /// 如果两者完全相同，返回 `true`。
    ///
    /// 如果哈希文件不存在，[`Snapshot`] 会创建它以供将来
    /// 测试使用，并返回 `true`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Compares the [`Snapshot`] with the SHA-256 hash file found in the given path, returning
    /// `true` if they are identical.
    ///
    /// If the hash file does not exist, it will be created by the [`Snapshot`] for future
    /// testing and `true` will be returned.
    ///
    /// </details>
    pub fn matches_hash(&self, path: impl AsRef<Path>) -> Result<bool, Error> {
        use sha2::{Digest, Sha256};

        let path = self.path(path, "sha256");

        let hash = {
            let mut hasher = Sha256::new();
            hasher.update(&self.screenshot.rgba);
            format!("{:x}", hasher.finalize())
        };

        if path.exists() {
            let saved_hash = fs::read_to_string(&path)?;

            Ok(hash == saved_hash)
        } else {
            if let Some(directory) = path.parent() {
                fs::create_dir_all(directory)?;
            }

            fs::write(path, hash)?;
            Ok(true)
        }
    }

    fn path(&self, path: impl AsRef<Path>, extension: &str) -> PathBuf {
        let path = path.as_ref();

        path.with_file_name(format!(
            "{name}-{renderer}",
            name = path
                .file_stem()
                .map(std::ffi::OsStr::to_string_lossy)
                .unwrap_or_default(),
            renderer = self.renderer
        ))
        .with_extension(extension)
    }
}

/// 创建一个新的 [`Simulator`]。
///
/// 这只是 [`Simulator::new`] 的函数版本。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Simulator`].
///
/// This is just a function version of [`Simulator::new`].
///
/// </details>
pub fn simulator<'a, Message, Theme, Renderer>(
    element: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Simulator<'a, Message, Theme, Renderer>
where
    Theme: theme::Base,
    Renderer: core::Renderer + core::renderer::Headless,
{
    Simulator::new(element)
}

/// 返回一次点击的事件序列。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns the sequence of events of a click.
///
/// </details>
pub fn click() -> impl Iterator<Item = Event> {
    [
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)),
    ]
    .into_iter()
}

/// 返回一次按键按下的事件序列。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns the sequence of events of a key press.
///
/// </details>
pub fn press_key(
    key: impl Into<keyboard::Key>,
    text: Option<SmolStr>,
) -> Event {
    let key = key.into();

    Event::Keyboard(keyboard::Event::KeyPressed {
        key: key.clone(),
        modified_key: key,
        physical_key: keyboard::key::Physical::Unidentified(
            keyboard::key::NativeCode::Unidentified,
        ),
        location: keyboard::Location::Standard,
        modifiers: keyboard::Modifiers::default(),
        repeat: false,
        text,
    })
}

/// 返回一次按键释放的事件序列。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns the sequence of events of a key release.
///
/// </details>
pub fn release_key(key: impl Into<keyboard::Key>) -> Event {
    let key = key.into();

    Event::Keyboard(keyboard::Event::KeyReleased {
        key: key.clone(),
        modified_key: key,
        physical_key: keyboard::key::Physical::Unidentified(
            keyboard::key::NativeCode::Unidentified,
        ),
        location: keyboard::Location::Standard,
        modifiers: keyboard::Modifiers::default(),
    })
}

/// 返回一次“敲击按键”（即按下并释放一个按键）的事件序列。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns the sequence of events of a "key tap" (i.e. pressing and releasing a key).
///
/// </details>
pub fn tap_key(
    key: impl Into<keyboard::Key>,
    text: Option<SmolStr>,
) -> impl Iterator<Item = Event> {
    let key = key.into();

    [press_key(key.clone(), text), release_key(key)].into_iter()
}

/// 返回在键盘上打出给定文本的事件序列。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns the sequence of events of typewriting the given text in a keyboard.
///
/// </details>
pub fn typewrite(text: &str) -> impl Iterator<Item = Event> + '_ {
    text.chars()
        .map(|c| SmolStr::new_inline(&c.to_string()))
        .flat_map(|c| tap_key(keyboard::Key::Character(c.clone()), Some(c)))
}

fn load_font(font: impl Into<Cow<'static, [u8]>>) -> Result<(), Error> {
    renderer::graphics::text::font_system()
        .write()
        .expect("Write to font system")
        .load_font(font.into());

    Ok(())
}
