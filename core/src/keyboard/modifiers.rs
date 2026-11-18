use bitflags::bitflags;

bitflags! {
    /// 键盘修饰键的当前状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The current state of the keyboard modifiers.
    ///
    /// </details>
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Modifiers: u32{
        /// “shift” 键。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The "shift" key.
        ///
        /// </details>
        const SHIFT = 0b100;
        // const LSHIFT = 0b010 << 0;
        // const RSHIFT = 0b001 << 0;
        //
        /// “control” 键。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The "control" key.
        ///
        /// </details>
        const CTRL = 0b100 << 3;
        // const LCTRL = 0b010 << 3;
        // const RCTRL = 0b001 << 3;
        //
        /// “alt” 键。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The "alt" key.
        ///
        /// </details>
        const ALT = 0b100 << 6;
        // const LALT = 0b010 << 6;
        // const RALT = 0b001 << 6;
        //
        /// Windows 上的 “windows” 键、Mac 上的 “command” 键，以及
        /// Linux 上的 “super” 键。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The "windows" key on Windows, "command" key on Mac, and
        /// "super" key on Linux.
        ///
        /// </details>
        const LOGO = 0b100 << 9;
        // const LLOGO = 0b010 << 9;
        // const RLOGO = 0b001 << 9;
        /// 没有修饰键
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// No modifiers
        ///
        /// </details>
        const NONE = 0;
    }
}

impl Modifiers {
    /// “command” 键。
    ///
    /// 通常是用于快捷键的主修饰键。
    ///
    /// 在 macOS 上，它等同于 `Self::LOGO`。
    /// 在其他平台上，它等同于 `Self::CTRL`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The "command" key.
    ///
    /// This is normally the main modifier to be used for hotkeys.
    ///
    /// On macOS, this is equivalent to `Self::LOGO`.
    /// Otherwise, this is equivalent to `Self::CTRL`.
    ///
    /// </details>
    pub const COMMAND: Self = if cfg!(target_os = "macos") {
        Self::LOGO
    } else {
        Self::CTRL
    };

    /// 如果 [`Modifiers`] 中 [`SHIFT`] 键处于按下状态，返回 true。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns true if the [`SHIFT`] key is pressed in the [`Modifiers`].
    ///
    /// </details>
    ///
    /// [`SHIFT`]: Self::SHIFT
    pub fn shift(self) -> bool {
        self.contains(Self::SHIFT)
    }

    /// 如果 [`Modifiers`] 中 [`CTRL`] 键处于按下状态，返回 true。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns true if the [`CTRL`] key is pressed in the [`Modifiers`].
    ///
    /// </details>
    ///
    /// [`CTRL`]: Self::CTRL
    pub fn control(self) -> bool {
        self.contains(Self::CTRL)
    }

    /// 如果 [`Modifiers`] 中 [`ALT`] 键处于按下状态，返回 true。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns true if the [`ALT`] key is pressed in the [`Modifiers`].
    ///
    /// </details>
    ///
    /// [`ALT`]: Self::ALT
    pub fn alt(self) -> bool {
        self.contains(Self::ALT)
    }

    /// 如果 [`Modifiers`] 中 [`LOGO`] 键处于按下状态，返回 true。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns true if the [`LOGO`] key is pressed in the [`Modifiers`].
    ///
    /// </details>
    ///
    /// [`LOGO`]: Self::LOGO
    pub fn logo(self) -> bool {
        self.contains(Self::LOGO)
    }

    /// 如果 [`Modifiers`] 中“命令键”处于按下状态，返回 true。
    ///
    /// “命令键”是当前平台上用于发出命令的主修饰键。
    /// 具体来说：
    ///
    /// - 在 macOS 上是 `logo` 键，即 command 键（⌘）
    /// - 在其他平台上是 `control` 键
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns true if a "command key" is pressed in the [`Modifiers`].
    ///
    /// The "command key" is the main modifier key used to issue commands in the
    /// current platform. Specifically:
    ///
    /// - It is the `logo` or command key (⌘) on macOS
    /// - It is the `control` key on other platforms
    ///
    /// </details>
    pub fn command(self) -> bool {
        #[cfg(target_os = "macos")]
        let is_pressed = self.logo();

        #[cfg(not(target_os = "macos"))]
        let is_pressed = self.control();

        is_pressed
    }

    /// 如果 [`Modifiers`] 中“跳转键”处于按下状态，返回 true。
    ///
    /// “跳转键”是用于扩大文本移动范围的修饰键。在 macOS 上是 `Alt`
    /// 键，在其他平台上是 `Ctrl` 键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns true if the "jump key" is pressed in the [`Modifiers`].
    ///
    /// The "jump key" is the modifier key used to widen text motions. It is the `Alt`
    /// key in macOS and the `Ctrl` key in other platforms.
    ///
    /// </details>
    pub fn jump(self) -> bool {
        if cfg!(target_os = "macos") {
            self.alt()
        } else {
            self.control()
        }
    }

    /// 如果在 macOS 设备上“命令键”处于按下状态，返回 true。
    ///
    /// 用于 macOS 特有的操作（例如 `⌘ + ArrowLeft` 把光标
    /// 移动到行首）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns true if the "command key" is pressed on a macOS device.
    ///
    /// This is relevant for macOS-specific actions (e.g. `⌘ + ArrowLeft` moves the cursor
    /// to the beginning of the line).
    ///
    /// </details>
    pub fn macos_command(self) -> bool {
        if cfg!(target_os = "macos") {
            self.logo()
        } else {
            false
        }
    }
}
