//! 识别键盘按键。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Identify keyboard keys.
//!
//! </details>
use crate::SmolStr;

/// 键盘上的按键。
///
/// 基本上就是 [`winit`] 中的 `Key` 类型。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A key on the keyboard.
///
/// This is mostly the `Key` type found in [`winit`].
///
/// </details>
///
/// [`winit`]: https://docs.rs/winit/0.30/winit/keyboard/enum.Key.html
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Key<C = SmolStr> {
    /// 具有既定名称的按键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A key with an established name.
    ///
    /// </details>
    Named(Named),

    /// 与用户键入的字符相对应的按键字符串，其中考虑了
    /// 用户当前的区域设置，以及所有正在生效的系统级键盘映射
    /// 覆盖。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A key string that corresponds to the character typed by the user, taking into account the
    /// user’s current locale setting, and any system-level keyboard mapping overrides that are in
    /// effect.
    ///
    /// </details>
    Character(C),

    /// 无法识别的按键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An unidentified key.
    ///
    /// </details>
    Unidentified,
}

impl Key {
    /// 把 `Key::Character(SmolStr)` 转换为 `Key::Character(&str)`，以便更方便地对
    /// `Key` 进行匹配。其他变体保持不变。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Convert `Key::Character(SmolStr)` to `Key::Character(&str)` so you can more easily match on
    /// `Key`. All other variants remain unchanged.
    ///
    /// </details>
    pub fn as_ref(&self) -> Key<&str> {
        match self {
            Self::Named(named) => Key::Named(*named),
            Self::Character(c) => Key::Character(c.as_ref()),
            Self::Unidentified => Key::Unidentified,
        }
    }

    /// 尝试把这个逻辑 [`Key`] 转换为对应的拉丁字符；如果它本身不是拉丁字符，
    /// 则使用给定的 [`Physical`] 物理按键进行转换。
    ///
    /// 如果找不到对应的拉丁字符，返回 `None`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Tries to convert this logical [`Key`] into its latin character, using the
    /// [`Physical`] key provided for translation if it isn't already in latin.
    ///
    /// Returns `None` if no latin variant could be found.
    ///
    /// </details>
    ///
    /// ```
    /// use iced_core::keyboard::key::{Key, Named, Physical, Code};
    ///
    /// // Latin c
    /// // 拉丁字母 c
    /// assert_eq!(
    ///     Key::Character("c".into()).to_latin(Physical::Code(Code::KeyC)),
    ///     Some('c'),
    /// );
    ///
    /// // Cyrillic с
    /// // 西里尔字母 с
    /// assert_eq!(
    ///     Key::Character("с".into()).to_latin(Physical::Code(Code::KeyC)),
    ///     Some('c'),
    /// );
    ///
    /// // Arrow Left
    /// // 左方向键
    /// assert_eq!(
    ///     Key::Named(Named::ArrowLeft).to_latin(Physical::Code(Code::ArrowLeft)),
    ///     None,
    /// );
    /// ```
    pub fn to_latin(&self, physical_key: Physical) -> Option<char> {
        let Self::Character(s) = self else {
            return None;
        };

        let mut chars = s.chars();
        let c = chars.next()?;

        if chars.next().is_none() && c < '\u{370}' {
            return Some(c);
        }

        let Physical::Code(code) = physical_key else {
            return None;
        };

        let latin = match code {
            Code::KeyA => 'a',
            Code::KeyB => 'b',
            Code::KeyC => 'c',
            Code::KeyD => 'd',
            Code::KeyE => 'e',
            Code::KeyF => 'f',
            Code::KeyG => 'g',
            Code::KeyH => 'h',
            Code::KeyI => 'i',
            Code::KeyJ => 'j',
            Code::KeyK => 'k',
            Code::KeyL => 'l',
            Code::KeyM => 'm',
            Code::KeyN => 'n',
            Code::KeyO => 'o',
            Code::KeyP => 'p',
            Code::KeyQ => 'q',
            Code::KeyR => 'r',
            Code::KeyS => 's',
            Code::KeyT => 't',
            Code::KeyU => 'u',
            Code::KeyV => 'v',
            Code::KeyW => 'w',
            Code::KeyX => 'x',
            Code::KeyY => 'y',
            Code::KeyZ => 'z',
            Code::Digit0 => '0',
            Code::Digit1 => '1',
            Code::Digit2 => '2',
            Code::Digit3 => '3',
            Code::Digit4 => '4',
            Code::Digit5 => '5',
            Code::Digit6 => '6',
            Code::Digit7 => '7',
            Code::Digit8 => '8',
            Code::Digit9 => '9',
            _ => return None,
        };

        Some(latin)
    }
}

impl From<Named> for Key {
    fn from(named: Named) -> Self {
        Self::Named(named)
    }
}

/// 具名按键。
///
/// 基本上就是 [`winit`] 中的 `NamedKey` 类型。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A named key.
///
/// This is mostly the `NamedKey` type found in [`winit`].
///
/// </details>
///
/// [`winit`]: https://docs.rs/winit/0.30/winit/keyboard/enum.Key.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum Named {
    /// `Alt`（Alternative）键。
    ///
    /// 该键启用交替修饰功能，用于解释同时或随后的
    /// 键盘输入。Apple 的 <kbd>Option</kbd> 键也使用这个键值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The `Alt` (Alternative) key.
    ///
    /// This key enables the alternate modifier function for interpreting concurrent or subsequent
    /// keyboard input. This key value is also used for the Apple <kbd>Option</kbd> key.
    ///
    /// </details>
    Alt,
    /// Alternate Graphics（<kbd>AltGr</kbd> 或 <kbd>AltGraph</kbd>）键。
    ///
    /// 该键用于启用 ISO Level 3 shift 修饰键（标准的 `Shift` 键是
    /// level 2 修饰键）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Alternate Graphics (<kbd>AltGr</kbd> or <kbd>AltGraph</kbd>) key.
    ///
    /// This key is used enable the ISO Level 3 shift modifier (the standard `Shift` key is the
    /// level 2 modifier).
    ///
    /// </details>
    AltGraph,
    /// `Caps Lock`（Capital）键。
    ///
    /// 切换大写字符锁定功能，用于解释随后的键盘输入事件。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The `Caps Lock` (Capital) key.
    ///
    /// Toggle capital character lock function for interpreting subsequent keyboard input event.
    ///
    /// </details>
    CapsLock,
    /// `Control` 或 `Ctrl` 键。
    ///
    /// 用于启用 control 修饰功能，以解释同时或随后的键盘
    /// 输入。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The `Control` or `Ctrl` key.
    ///
    /// Used to enable control modifier function for interpreting concurrent or subsequent keyboard
    /// input.
    ///
    /// </details>
    Control,
    /// 功能切换键 `Fn`。与另一个键同时按下时，会把
    /// 那个键的值改为另一个字符或功能。该键通常由键盘硬件
    /// 直接处理，一般不会产生按键事件。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Function switch `Fn` key. Activating this key simultaneously with another key changes
    /// that key’s value to an alternate character or function. This key is often handled directly
    /// in the keyboard hardware and does not usually generate key events.
    ///
    /// </details>
    Fn,
    /// 功能锁定（`FnLock` 或 `F-Lock`）键。按下该键会切换
    /// 键盘的模式，把某些键的值改为另一个字符或功能。该键
    /// 通常由键盘硬件直接处理，一般不会产生按键事件。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Function-Lock (`FnLock` or `F-Lock`) key. Activating this key switches the mode of the
    /// keyboard to changes some keys' values to an alternate character or function. This key is
    /// often handled directly in the keyboard hardware and does not usually generate key events.
    ///
    /// </details>
    FnLock,
    /// `NumLock` 键，即数字锁定键。用于切换数字小键盘模式功能，以解释
    /// 随后的键盘输入。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The `NumLock` or Number Lock key. Used to toggle numpad mode function for interpreting
    /// subsequent keyboard input.
    ///
    /// </details>
    NumLock,
    /// 在滚动模式和光标移动模式之间切换。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle between scrolling and cursor movement modes.
    ///
    /// </details>
    ScrollLock,
    /// 用于启用 shift 修饰功能，以解释同时或随后的键盘
    /// 输入。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Used to enable shift modifier function for interpreting concurrent or subsequent keyboard
    /// input.
    ///
    /// </details>
    Shift,
    /// Symbol 修饰键（用于某些虚拟键盘）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Symbol modifier key (used on some virtual keyboards).
    ///
    /// </details>
    Symbol,
    SymbolLock,
    // Legacy modifier key. Also called "Super" in certain places.
    // 旧式修饰键。在某些场合也称为 “Super”。
    Meta,
    // Legacy modifier key.
    // 旧式修饰键。
    Hyper,
    /// 用于启用 “super” 修饰功能，以解释同时或随后的键盘
    /// 输入。“Windows 徽标”键和 Apple 的 `Command` 键（`⌘`）都使用这个键值。
    ///
    /// 注意：在某些环境中（例如 Web），它被称为 “Meta” 键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Used to enable "super" modifier function for interpreting concurrent or subsequent keyboard
    /// input. This key value is used for the "Windows Logo" key and the Apple `Command` or `⌘` key.
    ///
    /// Note: In some contexts (e.g. the Web) this is referred to as the "Meta" key.
    ///
    /// </details>
    Super,
    /// `Enter` 或 `↵` 键。用于激活当前选中项或确认当前输入。
    /// `Return` 键（Macintosh 数字小键盘）也使用这个键值。
    /// Android 的 `KEYCODE_DPAD_CENTER` 也使用这个键值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The `Enter` or `↵` key. Used to activate current selection or accept current input. This key
    /// value is also used for the `Return` (Macintosh numpad) key. This key value is also used for
    /// the Android `KEYCODE_DPAD_CENTER`.
    ///
    /// </details>
    Enter,
    /// 水平制表键 `Tab`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Horizontal Tabulation `Tab` key.
    ///
    /// </details>
    Tab,
    /// 在文本中用于在单词之间插入空格。通常位于字符键的下方。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Used in text to insert a space between words. Usually located below the character keys.
    ///
    /// </details>
    Space,
    /// 向下导航或遍历。（`KEYCODE_DPAD_DOWN`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Navigate or traverse downward. (`KEYCODE_DPAD_DOWN`)
    ///
    /// </details>
    ArrowDown,
    /// 向左导航或遍历。（`KEYCODE_DPAD_LEFT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Navigate or traverse leftward. (`KEYCODE_DPAD_LEFT`)
    ///
    /// </details>
    ArrowLeft,
    /// 向右导航或遍历。（`KEYCODE_DPAD_RIGHT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Navigate or traverse rightward. (`KEYCODE_DPAD_RIGHT`)
    ///
    /// </details>
    ArrowRight,
    /// 向上导航或遍历。（`KEYCODE_DPAD_UP`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Navigate or traverse upward. (`KEYCODE_DPAD_UP`)
    ///
    /// </details>
    ArrowUp,
    /// End 键，在键盘输入时用于跳到内容末尾（`KEYCODE_MOVE_END`）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The End key, used with keyboard entry to go to the end of content (`KEYCODE_MOVE_END`).
    ///
    /// </details>
    End,
    /// Home 键，在键盘输入时用于跳到内容开头（`KEYCODE_MOVE_HOME`）。
    /// 手机上的 `Home` 键（回到手机主屏幕）请使用 [`GoHome`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Home key, used with keyboard entry, to go to start of content (`KEYCODE_MOVE_HOME`).
    /// For the mobile phone `Home` key (which goes to the phone’s main screen), use [`GoHome`].
    ///
    /// </details>
    ///
    /// [`GoHome`]: Self::GoHome
    Home,
    /// 向下滚动或显示下一页内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Scroll down or display next page of content.
    ///
    /// </details>
    PageDown,
    /// 向上滚动或显示上一页内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Scroll up or display previous page of content.
    ///
    /// </details>
    PageUp,
    /// 用于删除光标左侧的字符。MacOS 键盘上标有 `Delete` 的键
    /// 也使用这个键值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Used to remove the character to the left of the cursor. This key value is also used for
    /// the key labeled `Delete` on MacOS keyboards.
    ///
    /// </details>
    Backspace,
    /// 移除当前选中的输入。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Remove the currently selected input.
    ///
    /// </details>
    Clear,
    /// 复制当前选中内容。（`APPCOMMAND_COPY`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Copy the current selection. (`APPCOMMAND_COPY`)
    ///
    /// </details>
    Copy,
    /// Cursor Select（光标选择）键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Cursor Select key.
    ///
    /// </details>
    CrSel,
    /// 剪切当前选中内容。（`APPCOMMAND_CUT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Cut the current selection. (`APPCOMMAND_CUT`)
    ///
    /// </details>
    Cut,
    /// 用于删除光标右侧的字符。在 MacOS 键盘上，`Fn` 生效时
    /// 标有 `Delete` 的键也使用这个键值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Used to delete the character to the right of the cursor. This key value is also used for the
    /// key labeled `Delete` on MacOS keyboards when `Fn` is active.
    ///
    /// </details>
    Delete,
    /// Erase to End of Field（擦除到字段末尾）键。该键删除从当前光标
    /// 位置到当前字段末尾的所有字符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Erase to End of Field key. This key deletes all characters from the current cursor
    /// position to the end of the current field.
    ///
    /// </details>
    EraseEof,
    /// Extend Selection（Exsel，扩展选择）键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Extend Selection (Exsel) key.
    ///
    /// </details>
    ExSel,
    /// 在插入和改写两种文本模式之间切换。
    /// （`KEYCODE_INSERT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle between text modes for insertion or overtyping.
    /// (`KEYCODE_INSERT`)
    ///
    /// </details>
    Insert,
    /// Paste（粘贴）键。（`APPCOMMAND_PASTE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Paste key. (`APPCOMMAND_PASTE`)
    ///
    /// </details>
    Paste,
    /// 重做上一个操作。（`APPCOMMAND_REDO`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Redo the last action. (`APPCOMMAND_REDO`)
    ///
    /// </details>
    Redo,
    /// 撤销上一个操作。（`APPCOMMAND_UNDO`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Undo the last action. (`APPCOMMAND_UNDO`)
    ///
    /// </details>
    Undo,
    /// Accept（Commit、OK）键。接受当前选项或输入法序列的转换结果。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Accept (Commit, OK) key. Accept current option or input method sequence conversion.
    ///
    /// </details>
    Accept,
    /// 重做或重复某个操作。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Redo or repeat an action.
    ///
    /// </details>
    Again,
    /// Attention（Attn）键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Attention (Attn) key.
    ///
    /// </details>
    Attn,
    Cancel,
    /// 显示应用程序的上下文菜单。
    /// 该键通常位于右 `Super` 键和右 `Control` 键之间。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Show the application’s context menu.
    /// This key is commonly found between the right `Super` key and the right `Control` key.
    ///
    /// </details>
    ContextMenu,
    /// `Esc` 键。该键最初用于发起转义序列，但
    /// 现在更普遍地用于退出（“escape”）当前上下文，例如关闭对话框
    /// 或退出全屏模式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The `Esc` key. This key was originally used to initiate an escape sequence, but is
    /// now more generally used to exit or "escape" the current context, such as closing a dialog
    /// or exiting full screen mode.
    ///
    /// </details>
    Escape,
    Execute,
    /// 打开查找对话框。（`APPCOMMAND_FIND`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Open the Find dialog. (`APPCOMMAND_FIND`)
    ///
    /// </details>
    Find,
    /// 打开帮助对话框，或切换帮助信息的显示。（`APPCOMMAND_HELP`、
    /// `KEYCODE_HELP`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Open a help dialog or toggle display of help information. (`APPCOMMAND_HELP`,
    /// `KEYCODE_HELP`)
    ///
    /// </details>
    Help,
    /// 暂停当前状态或应用程序（视情况而定）。
    ///
    /// 注意：不要把这个值用于媒体控制器上的 `Pause` 按钮。请改用
    /// `"MediaPause"`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Pause the current state or application (as appropriate).
    ///
    /// Note: Do not use this value for the `Pause` button on media controllers. Use `"MediaPause"`
    /// instead.
    ///
    /// </details>
    Pause,
    /// 播放或恢复当前状态或应用程序（视情况而定）。
    ///
    /// 注意：不要把这个值用于媒体控制器上的 `Play` 按钮。请改用
    /// `"MediaPlay"`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Play or resume the current state or application (as appropriate).
    ///
    /// Note: Do not use this value for the `Play` button on media controllers. Use `"MediaPlay"`
    /// instead.
    ///
    /// </details>
    Play,
    /// 属性（Props）键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The properties (Props) key.
    ///
    /// </details>
    Props,
    Select,
    /// ZoomIn（放大）键。（`KEYCODE_ZOOM_IN`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The ZoomIn key. (`KEYCODE_ZOOM_IN`)
    ///
    /// </details>
    ZoomIn,
    /// ZoomOut（缩小）键。（`KEYCODE_ZOOM_OUT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The ZoomOut key. (`KEYCODE_ZOOM_OUT`)
    ///
    /// </details>
    ZoomOut,
    /// Brightness Down（降低亮度）键。通常用于控制显示器亮度。
    /// （`KEYCODE_BRIGHTNESS_DOWN`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Brightness Down key. Typically controls the display brightness.
    /// (`KEYCODE_BRIGHTNESS_DOWN`)
    ///
    /// </details>
    BrightnessDown,
    /// Brightness Up（提高亮度）键。通常用于控制显示器亮度。（`KEYCODE_BRIGHTNESS_UP`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Brightness Up key. Typically controls the display brightness. (`KEYCODE_BRIGHTNESS_UP`)
    ///
    /// </details>
    BrightnessUp,
    /// 在弹出（打开）和插入（关闭）状态之间切换可移动介质。（`KEYCODE_MEDIA_EJECT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle removable media to eject (open) and insert (close) state. (`KEYCODE_MEDIA_EJECT`)
    ///
    /// </details>
    Eject,
    LogOff,
    /// 切换电源状态。（`KEYCODE_POWER`）
    /// 注意：某些设备可能不会把该键暴露给操作环境。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle power state. (`KEYCODE_POWER`)
    /// Note: Note: Some devices might not expose this key to the operating environment.
    ///
    /// </details>
    Power,
    /// `PowerOff` 键。有时也称为 `PowerDown`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The `PowerOff` key. Sometime called `PowerDown`.
    ///
    /// </details>
    PowerOff,
    /// 启动截屏（print-screen）功能。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Initiate print-screen function.
    ///
    /// </details>
    PrintScreen,
    /// Hibernate（休眠）键。该键把计算机的当前状态保存到磁盘，以便之后
    /// 恢复。随后计算机将关机。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Hibernate key. This key saves the current state of the computer to disk so that it can
    /// be restored. The computer will then shutdown.
    ///
    /// </details>
    Hibernate,
    /// Standby（待机）键。该键关闭显示器，并让计算机进入低功耗
    /// 模式而不完全关机。有时该键标为 `Suspend` 或 `Sleep`。
    /// （`KEYCODE_SLEEP`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Standby key. This key turns off the display and places the computer into a low-power
    /// mode without completely shutting down. It is sometimes labelled `Suspend` or `Sleep` key.
    /// (`KEYCODE_SLEEP`)
    ///
    /// </details>
    Standby,
    /// WakeUp（唤醒）键。（`KEYCODE_WAKEUP`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The WakeUp key. (`KEYCODE_WAKEUP`)
    ///
    /// </details>
    WakeUp,
    /// 启动多候选模式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Initiate the multi-candidate mode.
    ///
    /// </details>
    AllCandidates,
    Alphanumeric,
    /// 启动代码输入模式，允许通过码点
    /// 输入字符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Initiate the Code Input mode to allow characters to be entered by
    /// their code points.
    ///
    /// </details>
    CodeInput,
    /// Compose 键，在 X Window System 中也称为 “Multi_key”。该键的
    /// 行为类似于死键，会触发一种模式，把随后的按键组合起来
    /// 产生另一个字符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Compose key, also known as "Multi_key" on the X Window System. This key acts in a
    /// manner similar to a dead key, triggering a mode where subsequent key presses are combined to
    /// produce a different character.
    ///
    /// </details>
    Compose,
    /// 转换当前的输入法序列。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Convert the current input method sequence.
    ///
    /// </details>
    Convert,
    /// 某些亚洲键盘上的 Final Mode 键 `Final`，用于启用输入法（IME）的 final 模式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Final Mode `Final` key used on some Asian keyboards, to enable the final mode for IMEs.
    ///
    /// </details>
    FinalMode,
    /// 切换到第一个字符组。（ISO/IEC 9995）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Switch to the first character group. (ISO/IEC 9995)
    ///
    /// </details>
    GroupFirst,
    /// 切换到最后一个字符组。（ISO/IEC 9995）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Switch to the last character group. (ISO/IEC 9995)
    ///
    /// </details>
    GroupLast,
    /// 切换到下一个字符组。（ISO/IEC 9995）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Switch to the next character group. (ISO/IEC 9995)
    ///
    /// </details>
    GroupNext,
    /// 切换到上一个字符组。（ISO/IEC 9995）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Switch to the previous character group. (ISO/IEC 9995)
    ///
    /// </details>
    GroupPrevious,
    /// 在输入法（IME）的输入模式之间切换或循环切换。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle between or cycle through input modes of IMEs.
    ///
    /// </details>
    ModeChange,
    NextCandidate,
    /// 在输入法（IME）中不经转换，直接接受当前的
    /// 输入法序列。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Accept current input method sequence without
    /// conversion in IMEs.
    ///
    /// </details>
    NonConvert,
    PreviousCandidate,
    Process,
    SingleCandidate,
    /// 在韩文（Hangul）模式和英文模式之间切换。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle between Hangul and English modes.
    ///
    /// </details>
    HangulMode,
    HanjaMode,
    JunjaMode,
    /// Eisu（英数）键。该键可能会关闭输入法，但其用途由当前输入法决定。
    /// （`KEYCODE_EISU`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Eisu key. This key may close the IME, but its purpose is defined by the current IME.
    /// (`KEYCODE_EISU`)
    ///
    /// </details>
    Eisu,
    /// Hankaku（半角）字符键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The (Half-Width) Characters key.
    ///
    /// </details>
    Hankaku,
    /// Hiragana（平假名，日语假名字符）键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Hiragana (Japanese Kana characters) key.
    ///
    /// </details>
    Hiragana,
    /// Hiragana/Katakana（平假名/片假名）切换键。（`KEYCODE_KATAKANA_HIRAGANA`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Hiragana/Katakana toggle key. (`KEYCODE_KATAKANA_HIRAGANA`)
    ///
    /// </details>
    HiraganaKatakana,
    /// Kana Mode（假名模式，即 Kana Lock）键。该键用于进入平假名模式（通常是从
    /// 罗马字模式进入）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Kana Mode (Kana Lock) key. This key is used to enter hiragana mode (typically from
    /// romaji mode).
    ///
    /// </details>
    KanaMode,
    /// Kanji（汉字，日语中对源自中文的表意文字的称呼）Mode 键。该键
    /// 通常用于切换到平假名键盘，以便把输入转换为
    /// 汉字。（`KEYCODE_KANA`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Kanji (Japanese name for ideographic characters of Chinese origin) Mode key. This key is
    /// typically used to switch to a hiragana keyboard for the purpose of converting input into
    /// kanji. (`KEYCODE_KANA`)
    ///
    /// </details>
    KanjiMode,
    /// Katakana（片假名，日语假名字符）键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Katakana (Japanese Kana characters) key.
    ///
    /// </details>
    Katakana,
    /// Roman characters（罗马字）功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Roman characters function key.
    ///
    /// </details>
    Romaji,
    /// Zenkaku（全角）字符键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Zenkaku (Full-Width) Characters key.
    ///
    /// </details>
    Zenkaku,
    /// Zenkaku/Hankaku（全角/半角）切换键。（`KEYCODE_ZENKAKU_HANKAKU`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Zenkaku/Hankaku (full-width/half-width) toggle key. (`KEYCODE_ZENKAKU_HANKAKU`)
    ///
    /// </details>
    ZenkakuHankaku,
    /// 通用虚拟功能键，索引为 1。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General purpose virtual function key, as index 1.
    ///
    /// </details>
    Soft1,
    /// 通用虚拟功能键，索引为 2。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General purpose virtual function key, as index 2.
    ///
    /// </details>
    Soft2,
    /// 通用虚拟功能键，索引为 3。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General purpose virtual function key, as index 3.
    ///
    /// </details>
    Soft3,
    /// 通用虚拟功能键，索引为 4。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General purpose virtual function key, as index 4.
    ///
    /// </details>
    Soft4,
    /// 选择（按数字或逻辑顺序的）下一个更低的频道。（`APPCOMMAND_MEDIA_CHANNEL_DOWN`、
    /// `KEYCODE_CHANNEL_DOWN`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Select next (numerically or logically) lower channel. (`APPCOMMAND_MEDIA_CHANNEL_DOWN`,
    /// `KEYCODE_CHANNEL_DOWN`)
    ///
    /// </details>
    ChannelDown,
    /// 选择（按数字或逻辑顺序的）下一个更高的频道。（`APPCOMMAND_MEDIA_CHANNEL_UP`、
    /// `KEYCODE_CHANNEL_UP`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Select next (numerically or logically) higher channel. (`APPCOMMAND_MEDIA_CHANNEL_UP`,
    /// `KEYCODE_CHANNEL_UP`)
    ///
    /// </details>
    ChannelUp,
    /// 关闭当前文档或消息（注意：这不会关闭应用程序）。
    /// （`APPCOMMAND_CLOSE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Close the current document or message (Note: This doesn’t close the application).
    /// (`APPCOMMAND_CLOSE`)
    ///
    /// </details>
    Close,
    /// 打开编辑器以转发当前邮件。（`APPCOMMAND_FORWARD_MAIL`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Open an editor to forward the current message. (`APPCOMMAND_FORWARD_MAIL`)
    ///
    /// </details>
    MailForward,
    /// 打开编辑器以回复当前邮件。（`APPCOMMAND_REPLY_TO_MAIL`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Open an editor to reply to the current message. (`APPCOMMAND_REPLY_TO_MAIL`)
    ///
    /// </details>
    MailReply,
    /// 发送当前邮件。（`APPCOMMAND_SEND_MAIL`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Send the current message. (`APPCOMMAND_SEND_MAIL`)
    ///
    /// </details>
    MailSend,
    /// 关闭当前媒体，例如关闭 CD 或 DVD 托盘。（`KEYCODE_MEDIA_CLOSE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Close the current media, for example to close a CD or DVD tray. (`KEYCODE_MEDIA_CLOSE`)
    ///
    /// </details>
    MediaClose,
    /// 以高于正常的速度开始或继续正向播放；如果已经在快进，
    /// 则提高速度。（`APPCOMMAND_MEDIA_FAST_FORWARD`、`KEYCODE_MEDIA_FAST_FORWARD`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Initiate or continue forward playback at faster than normal speed, or increase speed if
    /// already fast forwarding. (`APPCOMMAND_MEDIA_FAST_FORWARD`, `KEYCODE_MEDIA_FAST_FORWARD`)
    ///
    /// </details>
    MediaFastForward,
    /// 暂停当前正在播放的媒体。（`APPCOMMAND_MEDIA_PAUSE`、`KEYCODE_MEDIA_PAUSE`）
    ///
    /// 注意：媒体控制器设备的暂停键应使用这个值，而不是
    /// `"Pause"`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Pause the currently playing media. (`APPCOMMAND_MEDIA_PAUSE`, `KEYCODE_MEDIA_PAUSE`)
    ///
    /// Note: Media controller devices should use this value rather than `"Pause"` for their pause
    /// keys.
    ///
    /// </details>
    MediaPause,
    /// 如果当前不是以正常速度播放，则以正常速度开始或继续播放
    /// 媒体。（`APPCOMMAND_MEDIA_PLAY`、`KEYCODE_MEDIA_PLAY`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Initiate or continue media playback at normal speed, if not currently playing at normal
    /// speed. (`APPCOMMAND_MEDIA_PLAY`, `KEYCODE_MEDIA_PLAY`)
    ///
    /// </details>
    MediaPlay,
    /// 在播放和暂停状态之间切换媒体。（`APPCOMMAND_MEDIA_PLAY_PAUSE`、
    /// `KEYCODE_MEDIA_PLAY_PAUSE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle media between play and pause states. (`APPCOMMAND_MEDIA_PLAY_PAUSE`,
    /// `KEYCODE_MEDIA_PLAY_PAUSE`)
    ///
    /// </details>
    MediaPlayPause,
    /// 开始或恢复录制当前选中的媒体。（`APPCOMMAND_MEDIA_RECORD`、
    /// `KEYCODE_MEDIA_RECORD`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Initiate or resume recording of currently selected media. (`APPCOMMAND_MEDIA_RECORD`,
    /// `KEYCODE_MEDIA_RECORD`)
    ///
    /// </details>
    MediaRecord,
    /// 以高于正常的速度开始或继续反向播放；如果已经在快退，
    /// 则提高速度。（`APPCOMMAND_MEDIA_REWIND`、`KEYCODE_MEDIA_REWIND`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Initiate or continue reverse playback at faster than normal speed, or increase speed if
    /// already rewinding. (`APPCOMMAND_MEDIA_REWIND`, `KEYCODE_MEDIA_REWIND`)
    ///
    /// </details>
    MediaRewind,
    /// 如果尚未停止，则停止媒体的播放、暂停、快进、快退或录制。
    /// （`APPCOMMAND_MEDIA_STOP`、`KEYCODE_MEDIA_STOP`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Stop media playing, pausing, forwarding, rewinding, or recording, if not already stopped.
    /// (`APPCOMMAND_MEDIA_STOP`, `KEYCODE_MEDIA_STOP`)
    ///
    /// </details>
    MediaStop,
    /// 跳到下一个媒体或节目曲目。（`APPCOMMAND_MEDIA_NEXTTRACK`、`KEYCODE_MEDIA_NEXT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Seek to next media or program track. (`APPCOMMAND_MEDIA_NEXTTRACK`, `KEYCODE_MEDIA_NEXT`)
    ///
    /// </details>
    MediaTrackNext,
    /// 跳到上一个媒体或节目曲目。（`APPCOMMAND_MEDIA_PREVIOUSTRACK`、
    /// `KEYCODE_MEDIA_PREVIOUS`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Seek to previous media or program track. (`APPCOMMAND_MEDIA_PREVIOUSTRACK`,
    /// `KEYCODE_MEDIA_PREVIOUS`)
    ///
    /// </details>
    MediaTrackPrevious,
    /// 新建文档或消息。（`APPCOMMAND_NEW`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Open a new document or message. (`APPCOMMAND_NEW`)
    ///
    /// </details>
    New,
    /// 打开已有的文档或消息。（`APPCOMMAND_OPEN`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Open an existing document or message. (`APPCOMMAND_OPEN`)
    ///
    /// </details>
    Open,
    /// 打印当前文档或消息。（`APPCOMMAND_PRINT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Print the current document or message. (`APPCOMMAND_PRINT`)
    ///
    /// </details>
    Print,
    /// 保存当前文档或消息。（`APPCOMMAND_SAVE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Save the current document or message. (`APPCOMMAND_SAVE`)
    ///
    /// </details>
    Save,
    /// 对当前文档或选中内容进行拼写检查。（`APPCOMMAND_SPELL_CHECK`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Spellcheck the current document or selection. (`APPCOMMAND_SPELL_CHECK`)
    ///
    /// </details>
    SpellCheck,
    /// `11` 键，见于按钮为 `1` ... `12` 的
    /// 媒体数字小键盘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The `11` key found on media numpads that
    /// have buttons from `1` ... `12`.
    ///
    /// </details>
    Key11,
    /// `12` 键，见于按钮为 `1` ... `12` 的
    /// 媒体数字小键盘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The `12` key found on media numpads that
    /// have buttons from `1` ... `12`.
    ///
    /// </details>
    Key12,
    /// 向左调整音频平衡。（`VK_AUDIO_BALANCE_LEFT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adjust audio balance leftward. (`VK_AUDIO_BALANCE_LEFT`)
    ///
    /// </details>
    AudioBalanceLeft,
    /// 向右调整音频平衡。（`VK_AUDIO_BALANCE_RIGHT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adjust audio balance rightward. (`VK_AUDIO_BALANCE_RIGHT`)
    ///
    /// </details>
    AudioBalanceRight,
    /// 减弱音频低音增强，或向下循环切换低音增强状态。（`APPCOMMAND_BASS_DOWN`、
    /// `VK_BASS_BOOST_DOWN`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Decrease audio bass boost or cycle down through bass boost states. (`APPCOMMAND_BASS_DOWN`,
    /// `VK_BASS_BOOST_DOWN`)
    ///
    /// </details>
    AudioBassBoostDown,
    /// 打开/关闭低音增强。（`APPCOMMAND_BASS_BOOST`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle bass boost on/off. (`APPCOMMAND_BASS_BOOST`)
    ///
    /// </details>
    AudioBassBoostToggle,
    /// 加强音频低音增强，或向上循环切换低音增强状态。（`APPCOMMAND_BASS_UP`、
    /// `VK_BASS_BOOST_UP`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Increase audio bass boost or cycle up through bass boost states. (`APPCOMMAND_BASS_UP`,
    /// `VK_BASS_BOOST_UP`)
    ///
    /// </details>
    AudioBassBoostUp,
    /// 把音频衰减器（fader）向前调整。（`VK_FADER_FRONT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adjust audio fader towards front. (`VK_FADER_FRONT`)
    ///
    /// </details>
    AudioFaderFront,
    /// 把音频衰减器（fader）向后调整。（`VK_FADER_REAR`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adjust audio fader towards rear. (`VK_FADER_REAR`)
    ///
    /// </details>
    AudioFaderRear,
    /// 把环绕声模式切换到下一个可用模式。（`VK_SURROUND_MODE_NEXT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Advance surround audio mode to next available mode. (`VK_SURROUND_MODE_NEXT`)
    ///
    /// </details>
    AudioSurroundModeNext,
    /// 降低高音。（`APPCOMMAND_TREBLE_DOWN`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Decrease treble. (`APPCOMMAND_TREBLE_DOWN`)
    ///
    /// </details>
    AudioTrebleDown,
    /// 提高高音。（`APPCOMMAND_TREBLE_UP`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Increase treble. (`APPCOMMAND_TREBLE_UP`)
    ///
    /// </details>
    AudioTrebleUp,
    /// 降低音量。（`APPCOMMAND_VOLUME_DOWN`、`KEYCODE_VOLUME_DOWN`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Decrease audio volume. (`APPCOMMAND_VOLUME_DOWN`, `KEYCODE_VOLUME_DOWN`)
    ///
    /// </details>
    AudioVolumeDown,
    /// 提高音量。（`APPCOMMAND_VOLUME_UP`、`KEYCODE_VOLUME_UP`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Increase audio volume. (`APPCOMMAND_VOLUME_UP`, `KEYCODE_VOLUME_UP`)
    ///
    /// </details>
    AudioVolumeUp,
    /// 在静音状态和之前的音量之间切换。（`APPCOMMAND_VOLUME_MUTE`、
    /// `KEYCODE_VOLUME_MUTE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle between muted state and prior volume level. (`APPCOMMAND_VOLUME_MUTE`,
    /// `KEYCODE_VOLUME_MUTE`)
    ///
    /// </details>
    AudioVolumeMute,
    /// 打开/关闭麦克风。（`APPCOMMAND_MIC_ON_OFF_TOGGLE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle the microphone on/off. (`APPCOMMAND_MIC_ON_OFF_TOGGLE`)
    ///
    /// </details>
    MicrophoneToggle,
    /// 降低麦克风音量。（`APPCOMMAND_MICROPHONE_VOLUME_DOWN`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Decrease microphone volume. (`APPCOMMAND_MICROPHONE_VOLUME_DOWN`)
    ///
    /// </details>
    MicrophoneVolumeDown,
    /// 提高麦克风音量。（`APPCOMMAND_MICROPHONE_VOLUME_UP`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Increase microphone volume. (`APPCOMMAND_MICROPHONE_VOLUME_UP`)
    ///
    /// </details>
    MicrophoneVolumeUp,
    /// 将麦克风静音。（`APPCOMMAND_MICROPHONE_VOLUME_MUTE`、`KEYCODE_MUTE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Mute the microphone. (`APPCOMMAND_MICROPHONE_VOLUME_MUTE`, `KEYCODE_MUTE`)
    ///
    /// </details>
    MicrophoneVolumeMute,
    /// 当某个词被错误识别时，显示更正列表。（`APPCOMMAND_CORRECTION_LIST`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Show correction list when a word is incorrectly identified. (`APPCOMMAND_CORRECTION_LIST`)
    ///
    /// </details>
    SpeechCorrectionList,
    /// 在听写模式和命令/控制模式之间切换。
    /// （`APPCOMMAND_DICTATE_OR_COMMAND_CONTROL_TOGGLE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle between dictation mode and command/control mode.
    /// (`APPCOMMAND_DICTATE_OR_COMMAND_CONTROL_TOGGLE`)
    ///
    /// </details>
    SpeechInputToggle,
    /// 第一个通用 “LaunchApplication” 键。通常用于启动“我的
    /// 电脑”，键上可能印有计算机符号。（`APPCOMMAND_LAUNCH_APP1`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The first generic "LaunchApplication" key. This is commonly associated with launching "My
    /// Computer", and may have a computer symbol on the key. (`APPCOMMAND_LAUNCH_APP1`)
    ///
    /// </details>
    LaunchApplication1,
    /// 第二个通用 “LaunchApplication” 键。通常用于启动
    /// “计算器”，键上可能印有计算器符号。（`APPCOMMAND_LAUNCH_APP2`、
    /// `KEYCODE_CALCULATOR`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The second generic "LaunchApplication" key. This is commonly associated with launching
    /// "Calculator", and may have a calculator symbol on the key. (`APPCOMMAND_LAUNCH_APP2`,
    /// `KEYCODE_CALCULATOR`)
    ///
    /// </details>
    LaunchApplication2,
    /// “Calendar”（日历）键。（`KEYCODE_CALENDAR`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The "Calendar" key. (`KEYCODE_CALENDAR`)
    ///
    /// </details>
    LaunchCalendar,
    /// “Contacts”（联系人）键。（`KEYCODE_CONTACTS`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The "Contacts" key. (`KEYCODE_CONTACTS`)
    ///
    /// </details>
    LaunchContacts,
    /// “Mail”（邮件）键。（`APPCOMMAND_LAUNCH_MAIL`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The "Mail" key. (`APPCOMMAND_LAUNCH_MAIL`)
    ///
    /// </details>
    LaunchMail,
    /// “Media Player”（媒体播放器）键。（`APPCOMMAND_LAUNCH_MEDIA_SELECT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The "Media Player" key. (`APPCOMMAND_LAUNCH_MEDIA_SELECT`)
    ///
    /// </details>
    LaunchMediaPlayer,
    LaunchMusicPlayer,
    LaunchPhone,
    LaunchScreenSaver,
    LaunchSpreadsheet,
    LaunchWebBrowser,
    LaunchWebCam,
    LaunchWordProcessor,
    /// 导航到当前历史记录中的上一个内容或页面。（`APPCOMMAND_BROWSER_BACKWARD`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Navigate to previous content or page in current history. (`APPCOMMAND_BROWSER_BACKWARD`)
    ///
    /// </details>
    BrowserBack,
    /// 打开浏览器收藏夹列表。（`APPCOMMAND_BROWSER_FAVORITES`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Open the list of browser favorites. (`APPCOMMAND_BROWSER_FAVORITES`)
    ///
    /// </details>
    BrowserFavorites,
    /// 导航到当前历史记录中的下一个内容或页面。（`APPCOMMAND_BROWSER_FORWARD`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Navigate to next content or page in current history. (`APPCOMMAND_BROWSER_FORWARD`)
    ///
    /// </details>
    BrowserForward,
    /// 前往用户首选的主页。（`APPCOMMAND_BROWSER_HOME`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Go to the user’s preferred home page. (`APPCOMMAND_BROWSER_HOME`)
    ///
    /// </details>
    BrowserHome,
    /// 刷新当前页面或内容。（`APPCOMMAND_BROWSER_REFRESH`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Refresh the current page or content. (`APPCOMMAND_BROWSER_REFRESH`)
    ///
    /// </details>
    BrowserRefresh,
    /// 调出用户首选的搜索页面。（`APPCOMMAND_BROWSER_SEARCH`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Call up the user’s preferred search page. (`APPCOMMAND_BROWSER_SEARCH`)
    ///
    /// </details>
    BrowserSearch,
    /// 停止加载当前页面或内容。（`APPCOMMAND_BROWSER_STOP`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Stop loading the current page or content. (`APPCOMMAND_BROWSER_STOP`)
    ///
    /// </details>
    BrowserStop,
    /// 应用切换键，提供一个可供切换的最近使用的应用列表。
    /// （`KEYCODE_APP_SWITCH`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Application switch key, which provides a list of recent apps to switch between.
    /// (`KEYCODE_APP_SWITCH`)
    ///
    /// </details>
    AppSwitch,
    /// Call（拨号）键。（`KEYCODE_CALL`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Call key. (`KEYCODE_CALL`)
    ///
    /// </details>
    Call,
    /// Camera（相机）键。（`KEYCODE_CAMERA`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Camera key. (`KEYCODE_CAMERA`)
    ///
    /// </details>
    Camera,
    /// Camera focus（相机对焦）键。（`KEYCODE_FOCUS`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Camera focus key. (`KEYCODE_FOCUS`)
    ///
    /// </details>
    CameraFocus,
    /// End Call（挂断）键。（`KEYCODE_ENDCALL`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The End Call key. (`KEYCODE_ENDCALL`)
    ///
    /// </details>
    EndCall,
    /// Back（返回）键。（`KEYCODE_BACK`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Back key. (`KEYCODE_BACK`)
    ///
    /// </details>
    GoBack,
    /// Home 键，回到手机的主屏幕。（`KEYCODE_HOME`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Home key, which goes to the phone’s main screen. (`KEYCODE_HOME`)
    ///
    /// </details>
    GoHome,
    /// Headset Hook（耳机挂钩）键。（`KEYCODE_HEADSETHOOK`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Headset Hook key. (`KEYCODE_HEADSETHOOK`)
    ///
    /// </details>
    HeadsetHook,
    LastNumberRedial,
    /// Notification（通知）键。（`KEYCODE_NOTIFICATION`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Notification key. (`KEYCODE_NOTIFICATION`)
    ///
    /// </details>
    Notification,
    /// 在礼仪模式的各状态之间切换：静音、振动、响铃……（`KEYCODE_MANNER_MODE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle between manner mode state: silent, vibrate, ring, ... (`KEYCODE_MANNER_MODE`)
    ///
    /// </details>
    MannerMode,
    VoiceDial,
    /// 切换到观看电视。（`KEYCODE_TV`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Switch to viewing TV. (`KEYCODE_TV`)
    ///
    /// </details>
    TV,
    /// 电视 3D 模式。（`KEYCODE_3D_MODE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// TV 3D Mode. (`KEYCODE_3D_MODE`)
    ///
    /// </details>
    TV3DMode,
    /// 在天线输入和有线输入之间切换。（`KEYCODE_TV_ANTENNA_CABLE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle between antenna and cable input. (`KEYCODE_TV_ANTENNA_CABLE`)
    ///
    /// </details>
    TVAntennaCable,
    /// 音频描述。（`KEYCODE_TV_AUDIO_DESCRIPTION`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Audio description. (`KEYCODE_TV_AUDIO_DESCRIPTION`)
    ///
    /// </details>
    TVAudioDescription,
    /// 降低音频描述的混音音量。（`KEYCODE_TV_AUDIO_DESCRIPTION_MIX_DOWN`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Audio description mixing volume down. (`KEYCODE_TV_AUDIO_DESCRIPTION_MIX_DOWN`)
    ///
    /// </details>
    TVAudioDescriptionMixDown,
    /// 提高音频描述的混音音量。（`KEYCODE_TV_AUDIO_DESCRIPTION_MIX_UP`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Audio description mixing volume up. (`KEYCODE_TV_AUDIO_DESCRIPTION_MIX_UP`)
    ///
    /// </details>
    TVAudioDescriptionMixUp,
    /// 内容菜单。（`KEYCODE_TV_CONTENTS_MENU`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Contents menu. (`KEYCODE_TV_CONTENTS_MENU`)
    ///
    /// </details>
    TVContentsMenu,
    /// 内容菜单。（`KEYCODE_TV_DATA_SERVICE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Contents menu. (`KEYCODE_TV_DATA_SERVICE`)
    ///
    /// </details>
    TVDataService,
    /// 切换外接电视的输入模式。（`KEYCODE_TV_INPUT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Switch the input mode on an external TV. (`KEYCODE_TV_INPUT`)
    ///
    /// </details>
    TVInput,
    /// 切换到分量输入 #1。（`KEYCODE_TV_INPUT_COMPONENT_1`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Switch to component input #1. (`KEYCODE_TV_INPUT_COMPONENT_1`)
    ///
    /// </details>
    TVInputComponent1,
    /// 切换到分量输入 #2。（`KEYCODE_TV_INPUT_COMPONENT_2`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Switch to component input #2. (`KEYCODE_TV_INPUT_COMPONENT_2`)
    ///
    /// </details>
    TVInputComponent2,
    /// 切换到复合输入 #1。（`KEYCODE_TV_INPUT_COMPOSITE_1`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Switch to composite input #1. (`KEYCODE_TV_INPUT_COMPOSITE_1`)
    ///
    /// </details>
    TVInputComposite1,
    /// 切换到复合输入 #2。（`KEYCODE_TV_INPUT_COMPOSITE_2`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Switch to composite input #2. (`KEYCODE_TV_INPUT_COMPOSITE_2`)
    ///
    /// </details>
    TVInputComposite2,
    /// 切换到 HDMI 输入 #1。（`KEYCODE_TV_INPUT_HDMI_1`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Switch to HDMI input #1. (`KEYCODE_TV_INPUT_HDMI_1`)
    ///
    /// </details>
    TVInputHDMI1,
    /// 切换到 HDMI 输入 #2。（`KEYCODE_TV_INPUT_HDMI_2`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Switch to HDMI input #2. (`KEYCODE_TV_INPUT_HDMI_2`)
    ///
    /// </details>
    TVInputHDMI2,
    /// 切换到 HDMI 输入 #3。（`KEYCODE_TV_INPUT_HDMI_3`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Switch to HDMI input #3. (`KEYCODE_TV_INPUT_HDMI_3`)
    ///
    /// </details>
    TVInputHDMI3,
    /// 切换到 HDMI 输入 #4。（`KEYCODE_TV_INPUT_HDMI_4`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Switch to HDMI input #4. (`KEYCODE_TV_INPUT_HDMI_4`)
    ///
    /// </details>
    TVInputHDMI4,
    /// 切换到 VGA 输入 #1。（`KEYCODE_TV_INPUT_VGA_1`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Switch to VGA input #1. (`KEYCODE_TV_INPUT_VGA_1`)
    ///
    /// </details>
    TVInputVGA1,
    /// 媒体上下文菜单。（`KEYCODE_TV_MEDIA_CONTEXT_MENU`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Media context menu. (`KEYCODE_TV_MEDIA_CONTEXT_MENU`)
    ///
    /// </details>
    TVMediaContext,
    /// 切换网络。（`KEYCODE_TV_NETWORK`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle network. (`KEYCODE_TV_NETWORK`)
    ///
    /// </details>
    TVNetwork,
    /// 数字输入。（`KEYCODE_TV_NUMBER_ENTRY`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Number entry. (`KEYCODE_TV_NUMBER_ENTRY`)
    ///
    /// </details>
    TVNumberEntry,
    /// 开关外接电视的电源。（`KEYCODE_TV_POWER`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle the power on an external TV. (`KEYCODE_TV_POWER`)
    ///
    /// </details>
    TVPower,
    /// 广播（Radio）。（`KEYCODE_TV_RADIO_SERVICE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Radio. (`KEYCODE_TV_RADIO_SERVICE`)
    ///
    /// </details>
    TVRadioService,
    /// 卫星。（`KEYCODE_TV_SATELLITE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Satellite. (`KEYCODE_TV_SATELLITE`)
    ///
    /// </details>
    TVSatellite,
    /// 广播卫星（BS）。（`KEYCODE_TV_SATELLITE_BS`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Broadcast Satellite. (`KEYCODE_TV_SATELLITE_BS`)
    ///
    /// </details>
    TVSatelliteBS,
    /// 通信卫星（CS）。（`KEYCODE_TV_SATELLITE_CS`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Communication Satellite. (`KEYCODE_TV_SATELLITE_CS`)
    ///
    /// </details>
    TVSatelliteCS,
    /// 在可用的卫星之间切换。（`KEYCODE_TV_SATELLITE_SERVICE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle between available satellites. (`KEYCODE_TV_SATELLITE_SERVICE`)
    ///
    /// </details>
    TVSatelliteToggle,
    /// 地面模拟信号。（`KEYCODE_TV_TERRESTRIAL_ANALOG`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Analog Terrestrial. (`KEYCODE_TV_TERRESTRIAL_ANALOG`)
    ///
    /// </details>
    TVTerrestrialAnalog,
    /// 地面数字信号。（`KEYCODE_TV_TERRESTRIAL_DIGITAL`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Digital Terrestrial. (`KEYCODE_TV_TERRESTRIAL_DIGITAL`)
    ///
    /// </details>
    TVTerrestrialDigital,
    /// 定时器编程。（`KEYCODE_TV_TIMER_PROGRAMMING`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Timer programming. (`KEYCODE_TV_TIMER_PROGRAMMING`)
    ///
    /// </details>
    TVTimer,
    /// 切换外接 AVR（音视频接收器）的输入模式。（`KEYCODE_AVR_INPUT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Switch the input mode on an external AVR (audio/video receiver). (`KEYCODE_AVR_INPUT`)
    ///
    /// </details>
    AVRInput,
    /// 开关外接 AVR（音视频接收器）的电源。（`KEYCODE_AVR_POWER`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle the power on an external AVR (audio/video receiver). (`KEYCODE_AVR_POWER`)
    ///
    /// </details>
    AVRPower,
    /// 通用彩色媒体功能键，索引为 0（红色）。（`VK_COLORED_KEY_0`、
    /// `KEYCODE_PROG_RED`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General purpose color-coded media function key, as index 0 (red). (`VK_COLORED_KEY_0`,
    /// `KEYCODE_PROG_RED`)
    ///
    /// </details>
    ColorF0Red,
    /// 通用彩色媒体功能键，索引为 1（绿色）。（`VK_COLORED_KEY_1`、
    /// `KEYCODE_PROG_GREEN`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General purpose color-coded media function key, as index 1 (green). (`VK_COLORED_KEY_1`,
    /// `KEYCODE_PROG_GREEN`)
    ///
    /// </details>
    ColorF1Green,
    /// 通用彩色媒体功能键，索引为 2（黄色）。（`VK_COLORED_KEY_2`、
    /// `KEYCODE_PROG_YELLOW`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General purpose color-coded media function key, as index 2 (yellow). (`VK_COLORED_KEY_2`,
    /// `KEYCODE_PROG_YELLOW`)
    ///
    /// </details>
    ColorF2Yellow,
    /// 通用彩色媒体功能键，索引为 3（蓝色）。（`VK_COLORED_KEY_3`、
    /// `KEYCODE_PROG_BLUE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General purpose color-coded media function key, as index 3 (blue). (`VK_COLORED_KEY_3`,
    /// `KEYCODE_PROG_BLUE`)
    ///
    /// </details>
    ColorF3Blue,
    /// 通用彩色媒体功能键，索引为 4（灰色）。（`VK_COLORED_KEY_4`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General purpose color-coded media function key, as index 4 (grey). (`VK_COLORED_KEY_4`)
    ///
    /// </details>
    ColorF4Grey,
    /// 通用彩色媒体功能键，索引为 5（棕色）。（`VK_COLORED_KEY_5`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General purpose color-coded media function key, as index 5 (brown). (`VK_COLORED_KEY_5`)
    ///
    /// </details>
    ColorF5Brown,
    /// 切换隐藏式字幕（Closed Captions）的显示。（`VK_CC`、`KEYCODE_CAPTIONS`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle the display of Closed Captions. (`VK_CC`, `KEYCODE_CAPTIONS`)
    ///
    /// </details>
    ClosedCaptionToggle,
    /// 通过在各状态之间切换或循环切换来调整设备亮度。（`VK_DIMMER`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adjust brightness of device, by toggling between or cycling through states. (`VK_DIMMER`)
    ///
    /// </details>
    Dimmer,
    /// 交换视频源。（`VK_DISPLAY_SWAP`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Swap video sources. (`VK_DISPLAY_SWAP`)
    ///
    /// </details>
    DisplaySwap,
    /// 选择数字视频录像机（DVR）。（`KEYCODE_DVR`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Select Digital Video Rrecorder. (`KEYCODE_DVR`)
    ///
    /// </details>
    DVR,
    /// 退出当前应用程序。（`VK_EXIT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Exit the current application. (`VK_EXIT`)
    ///
    /// </details>
    Exit,
    /// 清除保存为收藏 0 的节目或内容。（`VK_CLEAR_FAVORITE_0`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Clear program or content stored as favorite 0. (`VK_CLEAR_FAVORITE_0`)
    ///
    /// </details>
    FavoriteClear0,
    /// 清除保存为收藏 1 的节目或内容。（`VK_CLEAR_FAVORITE_1`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Clear program or content stored as favorite 1. (`VK_CLEAR_FAVORITE_1`)
    ///
    /// </details>
    FavoriteClear1,
    /// 清除保存为收藏 2 的节目或内容。（`VK_CLEAR_FAVORITE_2`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Clear program or content stored as favorite 2. (`VK_CLEAR_FAVORITE_2`)
    ///
    /// </details>
    FavoriteClear2,
    /// 清除保存为收藏 3 的节目或内容。（`VK_CLEAR_FAVORITE_3`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Clear program or content stored as favorite 3. (`VK_CLEAR_FAVORITE_3`)
    ///
    /// </details>
    FavoriteClear3,
    /// 选择（调出）保存为收藏 0 的节目或内容。（`VK_RECALL_FAVORITE_0`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Select (recall) program or content stored as favorite 0. (`VK_RECALL_FAVORITE_0`)
    ///
    /// </details>
    FavoriteRecall0,
    /// 选择（调出）保存为收藏 1 的节目或内容。（`VK_RECALL_FAVORITE_1`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Select (recall) program or content stored as favorite 1. (`VK_RECALL_FAVORITE_1`)
    ///
    /// </details>
    FavoriteRecall1,
    /// 选择（调出）保存为收藏 2 的节目或内容。（`VK_RECALL_FAVORITE_2`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Select (recall) program or content stored as favorite 2. (`VK_RECALL_FAVORITE_2`)
    ///
    /// </details>
    FavoriteRecall2,
    /// 选择（调出）保存为收藏 3 的节目或内容。（`VK_RECALL_FAVORITE_3`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Select (recall) program or content stored as favorite 3. (`VK_RECALL_FAVORITE_3`)
    ///
    /// </details>
    FavoriteRecall3,
    /// 把当前节目或内容保存为收藏 0。（`VK_STORE_FAVORITE_0`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Store current program or content as favorite 0. (`VK_STORE_FAVORITE_0`)
    ///
    /// </details>
    FavoriteStore0,
    /// 把当前节目或内容保存为收藏 1。（`VK_STORE_FAVORITE_1`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Store current program or content as favorite 1. (`VK_STORE_FAVORITE_1`)
    ///
    /// </details>
    FavoriteStore1,
    /// 把当前节目或内容保存为收藏 2。（`VK_STORE_FAVORITE_2`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Store current program or content as favorite 2. (`VK_STORE_FAVORITE_2`)
    ///
    /// </details>
    FavoriteStore2,
    /// 把当前节目或内容保存为收藏 3。（`VK_STORE_FAVORITE_3`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Store current program or content as favorite 3. (`VK_STORE_FAVORITE_3`)
    ///
    /// </details>
    FavoriteStore3,
    /// 切换节目或内容指南的显示。（`VK_GUIDE`、`KEYCODE_GUIDE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle display of program or content guide. (`VK_GUIDE`, `KEYCODE_GUIDE`)
    ///
    /// </details>
    Guide,
    /// 如果指南处于活动状态并已显示，则显示后一天的内容。（`VK_NEXT_DAY`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// If guide is active and displayed, then display next day’s content. (`VK_NEXT_DAY`)
    ///
    /// </details>
    GuideNextDay,
    /// 如果指南处于活动状态并已显示，则显示前一天的内容。（`VK_PREV_DAY`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// If guide is active and displayed, then display previous day’s content. (`VK_PREV_DAY`)
    ///
    /// </details>
    GuidePreviousDay,
    /// 切换当前选中的上下文或媒体的信息显示。（`VK_INFO`、
    /// `KEYCODE_INFO`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle display of information about currently selected context or media. (`VK_INFO`,
    /// `KEYCODE_INFO`)
    ///
    /// </details>
    Info,
    /// 切换即时回放。（`VK_INSTANT_REPLAY`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle instant replay. (`VK_INSTANT_REPLAY`)
    ///
    /// </details>
    InstantReplay,
    /// 在可用且合适时，启动链接的内容。（`VK_LINK`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Launch linked content, if available and appropriate. (`VK_LINK`)
    ///
    /// </details>
    Link,
    /// 列出当前节目。（`VK_LIST`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// List the current program. (`VK_LIST`)
    ///
    /// </details>
    ListProgram,
    /// 切换当前可用的直播内容或节目列表的显示。（`VK_LIVE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle display listing of currently available live content or programs. (`VK_LIVE`)
    ///
    /// </details>
    LiveContent,
    /// 锁定或解锁当前内容或节目。（`VK_LOCK`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Lock or unlock current content or program. (`VK_LOCK`)
    ///
    /// </details>
    Lock,
    /// 显示媒体应用程序列表：音频/视频播放器和图片查看器。（`VK_APPS`）
    ///
    /// 注意：不要把这个键值与 Windows 的 `VK_APPS` / `VK_CONTEXT_MENU` 键混淆，
    /// 后者被编码为 `"ContextMenu"`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Show a list of media applications: audio/video players and image viewers. (`VK_APPS`)
    ///
    /// Note: Do not confuse this key value with the Windows' `VK_APPS` / `VK_CONTEXT_MENU` key,
    /// which is encoded as `"ContextMenu"`.
    ///
    /// </details>
    MediaApps,
    /// 音轨键。（`KEYCODE_MEDIA_AUDIO_TRACK`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Audio track key. (`KEYCODE_MEDIA_AUDIO_TRACK`)
    ///
    /// </details>
    MediaAudioTrack,
    /// 选择上一次选中的频道或媒体。（`VK_LAST`、`KEYCODE_LAST_CHANNEL`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Select previously selected channel or media. (`VK_LAST`, `KEYCODE_LAST_CHANNEL`)
    ///
    /// </details>
    MediaLast,
    /// 向后跳到下一个内容或节目。（`KEYCODE_MEDIA_SKIP_BACKWARD`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Skip backward to next content or program. (`KEYCODE_MEDIA_SKIP_BACKWARD`)
    ///
    /// </details>
    MediaSkipBackward,
    /// 向前跳到下一个内容或节目。（`VK_SKIP`、`KEYCODE_MEDIA_SKIP_FORWARD`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Skip forward to next content or program. (`VK_SKIP`, `KEYCODE_MEDIA_SKIP_FORWARD`)
    ///
    /// </details>
    MediaSkipForward,
    /// 向后步进到下一个内容或节目。（`KEYCODE_MEDIA_STEP_BACKWARD`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Step backward to next content or program. (`KEYCODE_MEDIA_STEP_BACKWARD`)
    ///
    /// </details>
    MediaStepBackward,
    /// 向前步进到下一个内容或节目。（`KEYCODE_MEDIA_STEP_FORWARD`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Step forward to next content or program. (`KEYCODE_MEDIA_STEP_FORWARD`)
    ///
    /// </details>
    MediaStepForward,
    /// 媒体顶层菜单。（`KEYCODE_MEDIA_TOP_MENU`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Media top menu. (`KEYCODE_MEDIA_TOP_MENU`)
    ///
    /// </details>
    MediaTopMenu,
    /// 向内导航。（`KEYCODE_NAVIGATE_IN`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Navigate in. (`KEYCODE_NAVIGATE_IN`)
    ///
    /// </details>
    NavigateIn,
    /// 导航到下一个键。（`KEYCODE_NAVIGATE_NEXT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Navigate to next key. (`KEYCODE_NAVIGATE_NEXT`)
    ///
    /// </details>
    NavigateNext,
    /// 向外导航。（`KEYCODE_NAVIGATE_OUT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Navigate out. (`KEYCODE_NAVIGATE_OUT`)
    ///
    /// </details>
    NavigateOut,
    /// 导航到上一个键。（`KEYCODE_NAVIGATE_PREVIOUS`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Navigate to previous key. (`KEYCODE_NAVIGATE_PREVIOUS`)
    ///
    /// </details>
    NavigatePrevious,
    /// 循环切换到（收藏列表中的）下一个收藏频道。（`VK_NEXT_FAVORITE_CHANNEL`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Cycle to next favorite channel (in favorites list). (`VK_NEXT_FAVORITE_CHANNEL`)
    ///
    /// </details>
    NextFavoriteChannel,
    /// 循环切换到下一个用户配置（如果有多个用户配置）。（`VK_USER`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Cycle to next user profile (if there are multiple user profiles). (`VK_USER`)
    ///
    /// </details>
    NextUserProfile,
    /// 访问点播内容或节目。（`VK_ON_DEMAND`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Access on-demand content or programs. (`VK_ON_DEMAND`)
    ///
    /// </details>
    OnDemand,
    /// 用于配对设备的配对键。（`KEYCODE_PAIRING`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Pairing key to pair devices. (`KEYCODE_PAIRING`)
    ///
    /// </details>
    Pairing,
    /// 向下移动画中画窗口。（`VK_PINP_DOWN`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Move picture-in-picture window down. (`VK_PINP_DOWN`)
    ///
    /// </details>
    PinPDown,
    /// 移动画中画窗口。（`VK_PINP_MOVE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Move picture-in-picture window. (`VK_PINP_MOVE`)
    ///
    /// </details>
    PinPMove,
    /// 切换画中画窗口的显示。（`VK_PINP_TOGGLE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle display of picture-in-picture window. (`VK_PINP_TOGGLE`)
    ///
    /// </details>
    PinPToggle,
    /// 向上移动画中画窗口。（`VK_PINP_UP`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Move picture-in-picture window up. (`VK_PINP_UP`)
    ///
    /// </details>
    PinPUp,
    /// 降低媒体播放速度。（`VK_PLAY_SPEED_DOWN`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Decrease media playback speed. (`VK_PLAY_SPEED_DOWN`)
    ///
    /// </details>
    PlaySpeedDown,
    /// 把播放重置为正常速度。（`VK_PLAY_SPEED_RESET`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Reset playback to normal speed. (`VK_PLAY_SPEED_RESET`)
    ///
    /// </details>
    PlaySpeedReset,
    /// 提高媒体播放速度。（`VK_PLAY_SPEED_UP`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Increase media playback speed. (`VK_PLAY_SPEED_UP`)
    ///
    /// </details>
    PlaySpeedUp,
    /// 切换媒体或内容的随机播放模式。（`VK_RANDOM_TOGGLE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle random media or content shuffle mode. (`VK_RANDOM_TOGGLE`)
    ///
    /// </details>
    RandomToggle,
    /// 不是物理按键，但遥控器电池电量不足时会发送这个键码。
    /// （`VK_RC_LOW_BATTERY`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Not a physical key, but this key code is sent when the remote control battery is low.
    /// (`VK_RC_LOW_BATTERY`)
    ///
    /// </details>
    RcLowBattery,
    /// 在媒体录制速度之间切换或循环切换。（`VK_RECORD_SPEED_NEXT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle or cycle between media recording speeds. (`VK_RECORD_SPEED_NEXT`)
    ///
    /// </details>
    RecordSpeedNext,
    /// 切换 RF（射频）输入直通模式（把 RF 输入直接传给 RF 输出）。
    /// （`VK_RF_BYPASS`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle RF (radio frequency) input bypass mode (pass RF input directly to the RF output).
    /// (`VK_RF_BYPASS`)
    ///
    /// </details>
    RfBypass,
    /// 切换频道扫描模式。（`VK_SCAN_CHANNELS_TOGGLE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle scan channels mode. (`VK_SCAN_CHANNELS_TOGGLE`)
    ///
    /// </details>
    ScanChannelsToggle,
    /// 把屏幕显示模式切换到下一个可用模式。（`VK_SCREEN_MODE_NEXT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Advance display screen mode to next available mode. (`VK_SCREEN_MODE_NEXT`)
    ///
    /// </details>
    ScreenModeNext,
    /// 切换设备设置界面的显示。（`VK_SETTINGS`、`KEYCODE_SETTINGS`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle display of device settings screen. (`VK_SETTINGS`, `KEYCODE_SETTINGS`)
    ///
    /// </details>
    Settings,
    /// 切换分屏模式。（`VK_SPLIT_SCREEN_TOGGLE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle split screen mode. (`VK_SPLIT_SCREEN_TOGGLE`)
    ///
    /// </details>
    SplitScreenToggle,
    /// 切换外接 STB（机顶盒）的输入模式。（`KEYCODE_STB_INPUT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Switch the input mode on an external STB (set top box). (`KEYCODE_STB_INPUT`)
    ///
    /// </details>
    STBInput,
    /// 开关外接 STB（机顶盒）的电源。（`KEYCODE_STB_POWER`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle the power on an external STB (set top box). (`KEYCODE_STB_POWER`)
    ///
    /// </details>
    STBPower,
    /// 切换字幕的显示（如果有）。（`VK_SUBTITLE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle display of subtitles, if available. (`VK_SUBTITLE`)
    ///
    /// </details>
    Subtitle,
    /// 切换图文电视（teletext）的显示（如果有）（`VK_TELETEXT`、`KEYCODE_TV_TELETEXT`）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle display of teletext, if available (`VK_TELETEXT`, `KEYCODE_TV_TELETEXT`).
    ///
    /// </details>
    Teletext,
    /// 把视频模式切换到下一个可用模式。（`VK_VIDEO_MODE_NEXT`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Advance video mode to next available mode. (`VK_VIDEO_MODE_NEXT`)
    ///
    /// </details>
    VideoModeNext,
    /// 让设备以某种方式表明自己的身份，例如发出声音或可见提示。（`VK_WINK`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Cause device to identify itself in some manner, e.g., audibly or visibly. (`VK_WINK`)
    ///
    /// </details>
    Wink,
    /// 在全屏内容和缩放内容之间切换，或改变放大级别。（`VK_ZOOM`、
    /// `KEYCODE_TV_ZOOM_MODE`）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle between full-screen and scaled content, or alter magnification level. (`VK_ZOOM`,
    /// `KEYCODE_TV_ZOOM_MODE`)
    ///
    /// </details>
    ZoomToggle,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F1,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F2,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F3,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F4,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F5,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F6,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F7,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F8,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F9,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F10,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F11,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F12,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F13,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F14,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F15,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F16,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F17,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F18,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F19,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F20,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F21,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F22,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F23,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F24,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F25,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F26,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F27,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F28,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F29,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F30,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F31,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F32,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F33,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F34,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F35,
}

/// 表示物理按键位置的代码。
///
/// 它基本遵循 UI Events 规范的 [`KeyboardEvent.code`]，只有少数
/// 例外：
/// - 规范中称为 “MetaLeft” 和 “MetaRight” 的按键，在这里命名为 “SuperLeft” 和
///   “SuperRight”。
/// - 规范中称为 “Super” 的按键，在这里报告为 `Unidentified`。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Code representing the location of a physical key.
///
/// This mostly conforms to the UI Events Specification's [`KeyboardEvent.code`] with a few
/// exceptions:
/// - The keys that the specification calls "MetaLeft" and "MetaRight" are named "SuperLeft" and
///   "SuperRight" here.
/// - The key that the specification calls "Super" is reported as `Unidentified` here.
///
/// </details>
///
/// [`KeyboardEvent.code`]: https://w3c.github.io/uievents-code/#code-value-tables
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum Code {
    /// 美式键盘上的 <kbd>`</kbd>。也称为反引号（backtick）或重音符（grave）。
    /// 在日语键盘上是 <kbd>半角</kbd>/<kbd>全角</kbd>/<kbd>漢字</kbd>
    /// （hankaku/zenkaku/kanji）键
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>`</kbd> on a US keyboard. This is also called a backtick or grave.
    /// This is the <kbd>半角</kbd>/<kbd>全角</kbd>/<kbd>漢字</kbd>
    /// (hankaku/zenkaku/kanji) key on Japanese keyboards
    ///
    /// </details>
    Backquote,
    /// 既用于美式键盘（101 键布局）上的 <kbd>\\</kbd>，也用于
    /// 102、104 和 106 键布局中位于 C 行 <kbd>"</kbd> 和 <kbd>Enter</kbd> 键之间的
    /// 那个键。
    /// 在英式（102 键）键盘上标为 <kbd>#</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Used for both the US <kbd>\\</kbd> (on the 101-key layout) and also for the key
    /// located between the <kbd>"</kbd> and <kbd>Enter</kbd> keys on row C of the 102-,
    /// 104- and 106-key layouts.
    /// Labeled <kbd>#</kbd> on a UK (102) keyboard.
    ///
    /// </details>
    Backslash,
    /// 美式键盘上的 <kbd>[</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>[</kbd> on a US keyboard.
    ///
    /// </details>
    BracketLeft,
    /// 美式键盘上的 <kbd>]</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>]</kbd> on a US keyboard.
    ///
    /// </details>
    BracketRight,
    /// 美式键盘上的 <kbd>,</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>,</kbd> on a US keyboard.
    ///
    /// </details>
    Comma,
    /// 美式键盘上的 <kbd>0</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>0</kbd> on a US keyboard.
    ///
    /// </details>
    Digit0,
    /// 美式键盘上的 <kbd>1</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>1</kbd> on a US keyboard.
    ///
    /// </details>
    Digit1,
    /// 美式键盘上的 <kbd>2</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>2</kbd> on a US keyboard.
    ///
    /// </details>
    Digit2,
    /// 美式键盘上的 <kbd>3</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>3</kbd> on a US keyboard.
    ///
    /// </details>
    Digit3,
    /// 美式键盘上的 <kbd>4</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>4</kbd> on a US keyboard.
    ///
    /// </details>
    Digit4,
    /// 美式键盘上的 <kbd>5</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>5</kbd> on a US keyboard.
    ///
    /// </details>
    Digit5,
    /// 美式键盘上的 <kbd>6</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>6</kbd> on a US keyboard.
    ///
    /// </details>
    Digit6,
    /// 美式键盘上的 <kbd>7</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>7</kbd> on a US keyboard.
    ///
    /// </details>
    Digit7,
    /// 美式键盘上的 <kbd>8</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>8</kbd> on a US keyboard.
    ///
    /// </details>
    Digit8,
    /// 美式键盘上的 <kbd>9</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>9</kbd> on a US keyboard.
    ///
    /// </details>
    Digit9,
    /// 美式键盘上的 <kbd>=</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>=</kbd> on a US keyboard.
    ///
    /// </details>
    Equal,
    /// 位于左 <kbd>Shift</kbd> 键和 <kbd>Z</kbd> 键之间。
    /// 在英式键盘上标为 <kbd>\\</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Located between the left <kbd>Shift</kbd> and <kbd>Z</kbd> keys.
    /// Labeled <kbd>\\</kbd> on a UK keyboard.
    ///
    /// </details>
    IntlBackslash,
    /// 位于 <kbd>/</kbd> 键和右 <kbd>Shift</kbd> 键之间。
    /// 在日语键盘上标为 <kbd>\\</kbd>（ro）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Located between the <kbd>/</kbd> and right <kbd>Shift</kbd> keys.
    /// Labeled <kbd>\\</kbd> (ro) on a Japanese keyboard.
    ///
    /// </details>
    IntlRo,
    /// 位于 <kbd>=</kbd> 键和 <kbd>Backspace</kbd> 键之间。
    /// 在日语键盘上标为 <kbd>¥</kbd>（yen）。在俄语键盘上是
    /// <kbd>\\</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Located between the <kbd>=</kbd> and <kbd>Backspace</kbd> keys.
    /// Labeled <kbd>¥</kbd> (yen) on a Japanese keyboard. <kbd>\\</kbd> on a
    /// Russian keyboard.
    ///
    /// </details>
    IntlYen,
    /// 美式键盘上的 <kbd>a</kbd>。
    /// 在 AZERTY（例如法语）键盘上标为 <kbd>q</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>a</kbd> on a US keyboard.
    /// Labeled <kbd>q</kbd> on an AZERTY (e.g., French) keyboard.
    ///
    /// </details>
    KeyA,
    /// 美式键盘上的 <kbd>b</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>b</kbd> on a US keyboard.
    ///
    /// </details>
    KeyB,
    /// 美式键盘上的 <kbd>c</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>c</kbd> on a US keyboard.
    ///
    /// </details>
    KeyC,
    /// 美式键盘上的 <kbd>d</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>d</kbd> on a US keyboard.
    ///
    /// </details>
    KeyD,
    /// 美式键盘上的 <kbd>e</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>e</kbd> on a US keyboard.
    ///
    /// </details>
    KeyE,
    /// 美式键盘上的 <kbd>f</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>f</kbd> on a US keyboard.
    ///
    /// </details>
    KeyF,
    /// 美式键盘上的 <kbd>g</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>g</kbd> on a US keyboard.
    ///
    /// </details>
    KeyG,
    /// 美式键盘上的 <kbd>h</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>h</kbd> on a US keyboard.
    ///
    /// </details>
    KeyH,
    /// 美式键盘上的 <kbd>i</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>i</kbd> on a US keyboard.
    ///
    /// </details>
    KeyI,
    /// 美式键盘上的 <kbd>j</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>j</kbd> on a US keyboard.
    ///
    /// </details>
    KeyJ,
    /// 美式键盘上的 <kbd>k</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>k</kbd> on a US keyboard.
    ///
    /// </details>
    KeyK,
    /// 美式键盘上的 <kbd>l</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>l</kbd> on a US keyboard.
    ///
    /// </details>
    KeyL,
    /// 美式键盘上的 <kbd>m</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>m</kbd> on a US keyboard.
    ///
    /// </details>
    KeyM,
    /// 美式键盘上的 <kbd>n</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>n</kbd> on a US keyboard.
    ///
    /// </details>
    KeyN,
    /// 美式键盘上的 <kbd>o</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>o</kbd> on a US keyboard.
    ///
    /// </details>
    KeyO,
    /// 美式键盘上的 <kbd>p</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>p</kbd> on a US keyboard.
    ///
    /// </details>
    KeyP,
    /// 美式键盘上的 <kbd>q</kbd>。
    /// 在 AZERTY（例如法语）键盘上标为 <kbd>a</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>q</kbd> on a US keyboard.
    /// Labeled <kbd>a</kbd> on an AZERTY (e.g., French) keyboard.
    ///
    /// </details>
    KeyQ,
    /// 美式键盘上的 <kbd>r</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>r</kbd> on a US keyboard.
    ///
    /// </details>
    KeyR,
    /// 美式键盘上的 <kbd>s</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>s</kbd> on a US keyboard.
    ///
    /// </details>
    KeyS,
    /// 美式键盘上的 <kbd>t</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>t</kbd> on a US keyboard.
    ///
    /// </details>
    KeyT,
    /// 美式键盘上的 <kbd>u</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>u</kbd> on a US keyboard.
    ///
    /// </details>
    KeyU,
    /// 美式键盘上的 <kbd>v</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>v</kbd> on a US keyboard.
    ///
    /// </details>
    KeyV,
    /// 美式键盘上的 <kbd>w</kbd>。
    /// 在 AZERTY（例如法语）键盘上标为 <kbd>z</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>w</kbd> on a US keyboard.
    /// Labeled <kbd>z</kbd> on an AZERTY (e.g., French) keyboard.
    ///
    /// </details>
    KeyW,
    /// 美式键盘上的 <kbd>x</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>x</kbd> on a US keyboard.
    ///
    /// </details>
    KeyX,
    /// 美式键盘上的 <kbd>y</kbd>。
    /// 在 QWERTZ（例如德语）键盘上标为 <kbd>z</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>y</kbd> on a US keyboard.
    /// Labeled <kbd>z</kbd> on a QWERTZ (e.g., German) keyboard.
    ///
    /// </details>
    KeyY,
    /// 美式键盘上的 <kbd>z</kbd>。
    /// 在 AZERTY（例如法语）键盘上标为 <kbd>w</kbd>，在
    /// QWERTZ（例如德语）键盘上标为 <kbd>y</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>z</kbd> on a US keyboard.
    /// Labeled <kbd>w</kbd> on an AZERTY (e.g., French) keyboard, and <kbd>y</kbd> on a
    /// QWERTZ (e.g., German) keyboard.
    ///
    /// </details>
    KeyZ,
    /// 美式键盘上的 <kbd>-</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>-</kbd> on a US keyboard.
    ///
    /// </details>
    Minus,
    /// 美式键盘上的 <kbd>.</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>.</kbd> on a US keyboard.
    ///
    /// </details>
    Period,
    /// 美式键盘上的 <kbd>'</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>'</kbd> on a US keyboard.
    ///
    /// </details>
    Quote,
    /// 美式键盘上的 <kbd>;</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>;</kbd> on a US keyboard.
    ///
    /// </details>
    Semicolon,
    /// 美式键盘上的 <kbd>/</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>/</kbd> on a US keyboard.
    ///
    /// </details>
    Slash,
    /// <kbd>Alt</kbd>、<kbd>Option</kbd> 或 <kbd>⌥</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>.
    ///
    /// </details>
    AltLeft,
    /// <kbd>Alt</kbd>、<kbd>Option</kbd> 或 <kbd>⌥</kbd>。
    /// 在许多键盘布局上标为 <kbd>AltGr</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>.
    /// This is labeled <kbd>AltGr</kbd> on many keyboard layouts.
    ///
    /// </details>
    AltRight,
    /// <kbd>Backspace</kbd> 或 <kbd>⌫</kbd>。
    /// 在 Apple 键盘上标为 <kbd>Delete</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>Backspace</kbd> or <kbd>⌫</kbd>.
    /// Labeled <kbd>Delete</kbd> on Apple keyboards.
    ///
    /// </details>
    Backspace,
    /// <kbd>CapsLock</kbd> 或 <kbd>⇪</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>CapsLock</kbd> or <kbd>⇪</kbd>
    ///
    /// </details>
    CapsLock,
    /// 应用程序上下文菜单键，通常位于右
    /// <kbd>Super</kbd> 键和右 <kbd>Control</kbd> 键之间。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The application context menu key, which is typically found between the right
    /// <kbd>Super</kbd> key and the right <kbd>Control</kbd> key.
    ///
    /// </details>
    ContextMenu,
    /// <kbd>Control</kbd> 或 <kbd>⌃</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ///
    /// </details>
    ControlLeft,
    /// <kbd>Control</kbd> 或 <kbd>⌃</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ///
    /// </details>
    ControlRight,
    /// <kbd>Enter</kbd> 或 <kbd>↵</kbd>。在 Apple 键盘上标为 <kbd>Return</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>Enter</kbd> or <kbd>↵</kbd>. Labeled <kbd>Return</kbd> on Apple keyboards.
    ///
    /// </details>
    Enter,
    /// Windows 键、<kbd>⌘</kbd>、<kbd>Command</kbd> 或其他操作系统符号键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.
    ///
    /// </details>
    SuperLeft,
    /// Windows 键、<kbd>⌘</kbd>、<kbd>Command</kbd> 或其他操作系统符号键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.
    ///
    /// </details>
    SuperRight,
    /// <kbd>Shift</kbd> 或 <kbd>⇧</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>Shift</kbd> or <kbd>⇧</kbd>
    ///
    /// </details>
    ShiftLeft,
    /// <kbd>Shift</kbd> 或 <kbd>⇧</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>Shift</kbd> or <kbd>⇧</kbd>
    ///
    /// </details>
    ShiftRight,
    /// <kbd> </kbd>（空格）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd> </kbd> (space)
    ///
    /// </details>
    Space,
    /// <kbd>Tab</kbd> 或 <kbd>⇥</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>Tab</kbd> or <kbd>⇥</kbd>
    ///
    /// </details>
    Tab,
    /// 日语：<kbd>変</kbd>（henkan）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Japanese: <kbd>変</kbd> (henkan)
    ///
    /// </details>
    Convert,
    /// 日语：<kbd>カタカナ</kbd>/<kbd>ひらがな</kbd>/<kbd>ローマ字</kbd>
    /// （katakana/hiragana/romaji）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Japanese: <kbd>カタカナ</kbd>/<kbd>ひらがな</kbd>/<kbd>ローマ字</kbd>
    /// (katakana/hiragana/romaji)
    ///
    /// </details>
    KanaMode,
    /// 韩语：HangulMode <kbd>한/영</kbd>（han/yeong）
    ///
    /// 日语（Mac 键盘）：<kbd>か</kbd>（kana）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Korean: HangulMode <kbd>한/영</kbd> (han/yeong)
    ///
    /// Japanese (Mac keyboard): <kbd>か</kbd> (kana)
    ///
    /// </details>
    Lang1,
    /// 韩语：Hanja <kbd>한</kbd>（hanja）
    ///
    /// 日语（Mac 键盘）：<kbd>英</kbd>（eisu）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Korean: Hanja <kbd>한</kbd> (hanja)
    ///
    /// Japanese (Mac keyboard): <kbd>英</kbd> (eisu)
    ///
    /// </details>
    Lang2,
    /// 日语（文字处理键盘）：Katakana
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Japanese (word-processing keyboard): Katakana
    ///
    /// </details>
    Lang3,
    /// 日语（文字处理键盘）：Hiragana
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Japanese (word-processing keyboard): Hiragana
    ///
    /// </details>
    Lang4,
    /// 日语（文字处理键盘）：Zenkaku/Hankaku
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Japanese (word-processing keyboard): Zenkaku/Hankaku
    ///
    /// </details>
    Lang5,
    /// 日语：<kbd>無変換</kbd>（muhenkan）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Japanese: <kbd>無変換</kbd> (muhenkan)
    ///
    /// </details>
    NonConvert,
    /// <kbd>⌦</kbd>。向前删除键。
    /// 注意：在 Apple 键盘上，主键区标有 <kbd>Delete</kbd> 的键
    /// 被编码为 [`Backspace`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>⌦</kbd>. The forward delete key.
    /// Note that on Apple keyboards, the key labelled <kbd>Delete</kbd> on the main part of
    /// the keyboard is encoded as [`Backspace`].
    ///
    /// </details>
    ///
    /// [`Backspace`]: Self::Backspace
    Delete,
    /// <kbd>Page Down</kbd>、<kbd>End</kbd> 或 <kbd>↘</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>Page Down</kbd>, <kbd>End</kbd>, or <kbd>↘</kbd>
    ///
    /// </details>
    End,
    /// <kbd>Help</kbd>。标准 PC 键盘上没有这个键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>Help</kbd>. Not present on standard PC keyboards.
    ///
    /// </details>
    Help,
    /// <kbd>Home</kbd> 或 <kbd>↖</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>Home</kbd> or <kbd>↖</kbd>
    ///
    /// </details>
    Home,
    /// <kbd>Insert</kbd> 或 <kbd>Ins</kbd>。Apple 键盘上没有这个键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>Insert</kbd> or <kbd>Ins</kbd>. Not present on Apple keyboards.
    ///
    /// </details>
    Insert,
    /// <kbd>Page Down</kbd>、<kbd>PgDn</kbd> 或 <kbd>⇟</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>Page Down</kbd>, <kbd>PgDn</kbd>, or <kbd>⇟</kbd>
    ///
    /// </details>
    PageDown,
    /// <kbd>Page Up</kbd>、<kbd>PgUp</kbd> 或 <kbd>⇞</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>Page Up</kbd>, <kbd>PgUp</kbd>, or <kbd>⇞</kbd>
    ///
    /// </details>
    PageUp,
    /// <kbd>↓</kbd>
    ArrowDown,
    /// <kbd>←</kbd>
    ArrowLeft,
    /// <kbd>→</kbd>
    ArrowRight,
    /// <kbd>↑</kbd>
    ArrowUp,
    /// 在 Mac 上，它用于数字小键盘的 <kbd>Clear</kbd> 键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// On the Mac, this is used for the numpad <kbd>Clear</kbd> key.
    ///
    /// </details>
    NumLock,
    /// 键盘上的 <kbd>0 Ins</kbd>。手机或遥控器上的 <kbd>0</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>0 Ins</kbd> on a keyboard. <kbd>0</kbd> on a phone or remote control
    ///
    /// </details>
    Numpad0,
    /// 键盘上的 <kbd>1 End</kbd>。手机或遥控器上的 <kbd>1</kbd> 或
    /// <kbd>1 QZ</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>1 End</kbd> on a keyboard. <kbd>1</kbd> or <kbd>1 QZ</kbd> on a phone or remote
    /// control
    ///
    /// </details>
    Numpad1,
    /// 键盘上的 <kbd>2 ↓</kbd>。手机或遥控器上的 <kbd>2 ABC</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>2 ↓</kbd> on a keyboard. <kbd>2 ABC</kbd> on a phone or remote control
    ///
    /// </details>
    Numpad2,
    /// 键盘上的 <kbd>3 PgDn</kbd>。手机或遥控器上的 <kbd>3 DEF</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>3 PgDn</kbd> on a keyboard. <kbd>3 DEF</kbd> on a phone or remote control
    ///
    /// </details>
    Numpad3,
    /// 键盘上的 <kbd>4 ←</kbd>。手机或遥控器上的 <kbd>4 GHI</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>4 ←</kbd> on a keyboard. <kbd>4 GHI</kbd> on a phone or remote control
    ///
    /// </details>
    Numpad4,
    /// 键盘上的 <kbd>5</kbd>。手机或遥控器上的 <kbd>5 JKL</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>5</kbd> on a keyboard. <kbd>5 JKL</kbd> on a phone or remote control
    ///
    /// </details>
    Numpad5,
    /// 键盘上的 <kbd>6 →</kbd>。手机或遥控器上的 <kbd>6 MNO</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>6 →</kbd> on a keyboard. <kbd>6 MNO</kbd> on a phone or remote control
    ///
    /// </details>
    Numpad6,
    /// 键盘上的 <kbd>7 Home</kbd>。手机或遥控器上的 <kbd>7 PQRS</kbd> 或
    /// <kbd>7 PRS</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>7 Home</kbd> on a keyboard. <kbd>7 PQRS</kbd> or <kbd>7 PRS</kbd> on a phone
    /// or remote control
    ///
    /// </details>
    Numpad7,
    /// 键盘上的 <kbd>8 ↑</kbd>。手机或遥控器上的 <kbd>8 TUV</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>8 ↑</kbd> on a keyboard. <kbd>8 TUV</kbd> on a phone or remote control
    ///
    /// </details>
    Numpad8,
    /// 键盘上的 <kbd>9 PgUp</kbd>。手机或遥控器上的 <kbd>9 WXYZ</kbd> 或
    /// <kbd>9 WXY</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>9 PgUp</kbd> on a keyboard. <kbd>9 WXYZ</kbd> or <kbd>9 WXY</kbd> on a phone
    /// or remote control
    ///
    /// </details>
    Numpad9,
    /// <kbd>+</kbd>
    NumpadAdd,
    /// 见于 Microsoft Natural Keyboard。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Found on the Microsoft Natural Keyboard.
    ///
    /// </details>
    NumpadBackspace,
    /// <kbd>C</kbd> 或 <kbd>A</kbd>（All Clear，全部清除）。也用于
    /// <kbd>Clear</kbd> 键与 <kbd>NumLock</kbd> 键分开的数字小键盘。在 Mac 上，
    /// 数字小键盘的 <kbd>Clear</kbd> 键被编码为 [`NumLock`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>C</kbd> or <kbd>A</kbd> (All Clear). Also for use with numpads that have a
    /// <kbd>Clear</kbd> key that is separate from the <kbd>NumLock</kbd> key. On the Mac, the
    /// numpad <kbd>Clear</kbd> key is encoded as [`NumLock`].
    ///
    /// </details>
    ///
    /// [`NumLock`]: Self::NumLock
    NumpadClear,
    /// <kbd>C</kbd>（Clear Entry，清除输入）
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>C</kbd> (Clear Entry)
    ///
    /// </details>
    NumpadClearEntry,
    /// <kbd>,</kbd>（千位分隔符）。在千位分隔符为 “.” 的区域
    /// （例如巴西），该键可能产生 <kbd>.</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>,</kbd> (thousands separator). For locales where the thousands separator
    /// is a "." (e.g., Brazil), this key may generate a <kbd>.</kbd>.
    ///
    /// </details>
    NumpadComma,
    /// <kbd>. Del</kbd>。在小数分隔符为 “,” 的区域（例如
    /// 巴西），该键可能产生 <kbd>,</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>. Del</kbd>. For locales where the decimal separator is "," (e.g.,
    /// Brazil), this key may generate a <kbd>,</kbd>.
    ///
    /// </details>
    NumpadDecimal,
    /// <kbd>/</kbd>
    NumpadDivide,
    NumpadEnter,
    /// <kbd>=</kbd>
    NumpadEqual,
    /// 手机或遥控设备上的 <kbd>#</kbd>。该键通常位于
    /// <kbd>9</kbd> 键的下方、<kbd>0</kbd> 键的右侧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>#</kbd> on a phone or remote control device. This key is typically found
    /// below the <kbd>9</kbd> key and to the right of the <kbd>0</kbd> key.
    ///
    /// </details>
    NumpadHash,
    /// <kbd>M</kbd> 把当前输入加到存储器中保存的值上。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>M</kbd> Add current entry to the value stored in memory.
    ///
    /// </details>
    NumpadMemoryAdd,
    /// <kbd>M</kbd> 清除存储器中保存的值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>M</kbd> Clear the value stored in memory.
    ///
    /// </details>
    NumpadMemoryClear,
    /// <kbd>M</kbd> 用存储器中保存的值替换当前输入。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>M</kbd> Replace the current entry with the value stored in memory.
    ///
    /// </details>
    NumpadMemoryRecall,
    /// <kbd>M</kbd> 用当前输入替换存储器中保存的值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>M</kbd> Replace the value stored in memory with the current entry.
    ///
    /// </details>
    NumpadMemoryStore,
    /// <kbd>M</kbd> 从存储器中保存的值里减去当前输入。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>M</kbd> Subtract current entry from the value stored in memory.
    ///
    /// </details>
    NumpadMemorySubtract,
    /// 键盘上的 <kbd>&ast;</kbd>。用于提供数学运算
    /// （<kbd>+</kbd>、<kbd>-</kbd>、<kbd>&ast;</kbd> 和 <kbd>/</kbd>）的数字小键盘。
    ///
    /// 手机和遥控器上的 <kbd>&ast;</kbd> 键请使用 `NumpadStar`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>*</kbd> on a keyboard. For use with numpads that provide mathematical
    /// operations (<kbd>+</kbd>, <kbd>-</kbd> <kbd>*</kbd> and <kbd>/</kbd>).
    ///
    /// Use `NumpadStar` for the <kbd>*</kbd> key on phones and remote controls.
    ///
    /// </details>
    NumpadMultiply,
    /// <kbd>(</kbd> 见于 Microsoft Natural Keyboard。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>(</kbd> Found on the Microsoft Natural Keyboard.
    ///
    /// </details>
    NumpadParenLeft,
    /// <kbd>)</kbd> 见于 Microsoft Natural Keyboard。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>)</kbd> Found on the Microsoft Natural Keyboard.
    ///
    /// </details>
    NumpadParenRight,
    /// 手机或遥控设备上的 <kbd>&ast;</kbd>。
    ///
    /// 该键通常位于 <kbd>7</kbd> 键的下方、
    /// <kbd>0</kbd> 键的左侧。
    ///
    /// 数字小键盘上的 <kbd>&ast;</kbd> 键请使用
    /// <kbd>"NumpadMultiply"</kbd>。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>*</kbd> on a phone or remote control device.
    ///
    /// This key is typically found below the <kbd>7</kbd> key and to the left of
    /// the <kbd>0</kbd> key.
    ///
    /// Use <kbd>"NumpadMultiply"</kbd> for the <kbd>*</kbd> key on
    /// numeric keypads.
    ///
    /// </details>
    NumpadStar,
    /// <kbd>-</kbd>
    NumpadSubtract,
    /// <kbd>Esc</kbd> 或 <kbd>⎋</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>Esc</kbd> or <kbd>⎋</kbd>
    ///
    /// </details>
    Escape,
    /// <kbd>Fn</kbd> 通常是一个硬件按键，不会产生单独的代码。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>Fn</kbd> This is typically a hardware key that does not generate a separate code.
    ///
    /// </details>
    Fn,
    /// <kbd>FLock</kbd> 或 <kbd>FnLock</kbd>。功能锁定键。见于 Microsoft
    /// Natural Keyboard。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>FLock</kbd> or <kbd>FnLock</kbd>. Function Lock key. Found on the Microsoft
    /// Natural Keyboard.
    ///
    /// </details>
    FnLock,
    /// <kbd>PrtScr SysRq</kbd> 或 <kbd>Print Screen</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>PrtScr SysRq</kbd> or <kbd>Print Screen</kbd>
    ///
    /// </details>
    PrintScreen,
    /// <kbd>Scroll Lock</kbd>
    ScrollLock,
    /// <kbd>Pause Break</kbd>
    Pause,
    /// 某些笔记本电脑把这个键放在 <kbd>↑</kbd> 键的左侧。
    ///
    /// 它也是 Android 上的“返回”按钮（三角形）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Some laptops place this key to the left of the <kbd>↑</kbd> key.
    ///
    /// This also the "back" button (triangle) on Android.
    ///
    /// </details>
    BrowserBack,
    BrowserFavorites,
    /// 某些笔记本电脑把这个键放在 <kbd>↑</kbd> 键的右侧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Some laptops place this key to the right of the <kbd>↑</kbd> key.
    ///
    /// </details>
    BrowserForward,
    /// Android 上的“主页”按钮。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The "home" button on Android.
    ///
    /// </details>
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    /// <kbd>Eject</kbd> 或 <kbd>⏏</kbd>。在某些 Apple 键盘上，该键位于
    /// 功能键区。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// <kbd>Eject</kbd> or <kbd>⏏</kbd>. This key is placed in the function section on some Apple
    /// keyboards.
    ///
    /// </details>
    Eject,
    /// 在键盘上有时标为 <kbd>My Computer</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sometimes labelled <kbd>My Computer</kbd> on the keyboard
    ///
    /// </details>
    LaunchApp1,
    /// 在键盘上有时标为 <kbd>Calculator</kbd>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sometimes labelled <kbd>Calculator</kbd> on the keyboard
    ///
    /// </details>
    LaunchApp2,
    LaunchMail,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    /// 在某些 Apple 键盘上，该键位于功能键区，取代了
    /// <kbd>Eject</kbd> 键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// This key is placed in the function section on some Apple keyboards, replacing the
    /// <kbd>Eject</kbd> key.
    ///
    /// </details>
    Power,
    Sleep,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    WakeUp,
    // Legacy modifier key. Also called "Super" in certain places.
    // 旧式修饰键。在某些场合也称为 “Super”。
    Meta,
    // Legacy modifier key.
    // 旧式修饰键。
    Hyper,
    Turbo,
    Abort,
    Resume,
    Suspend,
    /// 见于 Sun 的 USB 键盘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Found on Sun’s USB keyboard.
    ///
    /// </details>
    Again,
    /// 见于 Sun 的 USB 键盘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Found on Sun’s USB keyboard.
    ///
    /// </details>
    Copy,
    /// 见于 Sun 的 USB 键盘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Found on Sun’s USB keyboard.
    ///
    /// </details>
    Cut,
    /// 见于 Sun 的 USB 键盘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Found on Sun’s USB keyboard.
    ///
    /// </details>
    Find,
    /// 见于 Sun 的 USB 键盘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Found on Sun’s USB keyboard.
    ///
    /// </details>
    Open,
    /// 见于 Sun 的 USB 键盘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Found on Sun’s USB keyboard.
    ///
    /// </details>
    Paste,
    /// 见于 Sun 的 USB 键盘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Found on Sun’s USB keyboard.
    ///
    /// </details>
    Props,
    /// 见于 Sun 的 USB 键盘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Found on Sun’s USB keyboard.
    ///
    /// </details>
    Select,
    /// 见于 Sun 的 USB 键盘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Found on Sun’s USB keyboard.
    ///
    /// </details>
    Undo,
    /// 用于某些日语文字处理键盘上专用的 <kbd>ひらがな</kbd> 键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Use for dedicated <kbd>ひらがな</kbd> key found on some Japanese word processing keyboards.
    ///
    /// </details>
    Hiragana,
    /// 用于某些日语文字处理键盘上专用的 <kbd>カタカナ</kbd> 键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Use for dedicated <kbd>カタカナ</kbd> key found on some Japanese word processing keyboards.
    ///
    /// </details>
    Katakana,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F1,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F2,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F3,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F4,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F5,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F6,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F7,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F8,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F9,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F10,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F11,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F12,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F13,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F14,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F15,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F16,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F17,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F18,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F19,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F20,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F21,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F22,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F23,
    /// 通用功能键。
    /// 通常位于键盘顶部。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    ///
    /// </details>
    F24,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F25,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F26,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F27,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F28,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F29,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F30,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F31,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F32,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F33,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F34,
    /// 通用功能键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// General-purpose function key.
    ///
    /// </details>
    F35,
}

/// 包含平台原生的物理按键标识符。
///
/// 具体的值因平台而异（这也是它按平台划分变体的原因
/// 之一），但这些值主要与按键在键盘上的物理位置相关。
///
/// 这个枚举主要用于在 Winit 没有把某个原生物理按键标识符映射到
/// 有意义的 [`Code`] 变体时保存原始键码。遇到我们尚未
/// 映射的标识符时，你仍然可以借助它像使用 [`Code`] 那样：
///
/// - 正确匹配按键按下和释放事件。
/// - 在非 Web 平台上，支持通过界面把快捷键绑定到几乎任意按键。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Contains the platform-native physical key identifier.
///
/// The exact values vary from platform to platform (which is part of why this is a per-platform
/// enum), but the values are primarily tied to the key's physical location on the keyboard.
///
/// This enum is primarily used to store raw keycodes when Winit doesn't map a given native
/// physical key identifier to a meaningful [`Code`] variant. In the presence of identifiers we
/// haven't mapped for you yet, this lets you use use [`Code`] to:
///
/// - Correctly match key press and release events.
/// - On non-web platforms, support assigning keybinds to virtually any key through a UI.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NativeCode {
    /// 无法识别的代码。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An unidentified code.
    ///
    /// </details>
    Unidentified,
    /// Android 的“扫描码”（scancode）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An Android "scancode".
    ///
    /// </details>
    Android(u32),
    /// macOS 的“扫描码”（scancode）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A macOS "scancode".
    ///
    /// </details>
    MacOS(u16),
    /// Windows 的“扫描码”（scancode）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A Windows "scancode".
    ///
    /// </details>
    Windows(u16),
    /// XKB 的“键码”（keycode）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An XKB "keycode".
    ///
    /// </details>
    Xkb(u32),
}

/// 表示物理按键的位置。
///
/// 这个类型是 [`Code`] 的超集，额外包含一个 [`Unidentified`][Self::Unidentified]
/// 变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Represents the location of a physical key.
///
/// This type is a superset of [`Code`], including an [`Unidentified`][Self::Unidentified]
/// variant.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Physical {
    /// 已知的按键代码
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A known key code
    ///
    /// </details>
    Code(Code),
    /// 当按键无法转换为 [`Code`] 时使用这个变体
    ///
    /// 此时会提供原生键码（如果有），这样你就可以通过对 [`Physical`] 按键
    /// 求哈希，更可靠地匹配按键按下和释放事件。也可以用它
    /// 为非标准按键设置快捷键，但这样的快捷键只适用于特定平台。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// This variant is used when the key cannot be translated to a [`Code`]
    ///
    /// The native keycode is provided (if available) so you're able to more reliably match
    /// key-press and key-release events by hashing the [`Physical`] key. It is also possible to use
    /// this for keybinds for non-standard keys, but such keybinds are tied to a given platform.
    ///
    /// </details>
    Unidentified(NativeCode),
}

impl PartialEq<Code> for Physical {
    #[inline]
    fn eq(&self, rhs: &Code) -> bool {
        match self {
            Physical::Code(code) => code == rhs,
            Physical::Unidentified(_) => false,
        }
    }
}

impl PartialEq<Physical> for Code {
    #[inline]
    fn eq(&self, rhs: &Physical) -> bool {
        rhs == self
    }
}

impl PartialEq<NativeCode> for Physical {
    #[inline]
    fn eq(&self, rhs: &NativeCode) -> bool {
        match self {
            Physical::Unidentified(code) => code == rhs,
            Physical::Code(_) => false,
        }
    }
}

impl PartialEq<Physical> for NativeCode {
    #[inline]
    fn eq(&self, rhs: &Physical) -> bool {
        rhs == self
    }
}
