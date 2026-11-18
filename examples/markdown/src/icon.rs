// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../fonts/markdown-icons.toml
// dcd2f0c969d603e2ee9237a4b70fa86b1a6e84d86f4689046d8fdd10440b06b9
// 由 iced_fontello 在构建时自动生成。
// 请勿手动编辑。来源：../fonts/markdown-icons.toml
use iced::Font;
use iced::widget::{Text, text};

pub const FONT: &[u8] = include_bytes!("../fonts/markdown-icons.ttf");

pub fn copy<'a>() -> Text<'a> {
    icon("\u{F0C5}")
}

fn icon(codepoint: &str) -> Text<'_> {
    text(codepoint).font(Font::with_name("markdown-icons"))
}
