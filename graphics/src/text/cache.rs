//! 缓存文本。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Cache text.
//!
//! </details>
use crate::core::{Font, Size};
use crate::text;

use rustc_hash::{FxHashMap, FxHashSet, FxHasher};
use std::collections::hash_map;
use std::hash::{Hash, Hasher};

/// 最近使用过的文本片段的存储。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A store of recently used sections of text.
///
/// </details>
#[derive(Debug, Default)]
pub struct Cache {
    entries: FxHashMap<KeyHash, Entry>,
    aliases: FxHashMap<KeyHash, KeyHash>,
    recently_used: FxHashSet<KeyHash>,
}

impl Cache {
    /// 创建一个新的空 [`Cache`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new empty [`Cache`].
    ///
    /// </details>
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取具有给定 [`KeyHash`] 的文本 [`Entry`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Gets the text [`Entry`] with the given [`KeyHash`].
    ///
    /// </details>
    pub fn get(&self, key: &KeyHash) -> Option<&Entry> {
        self.entries.get(key)
    }

    /// 如果 [`Cache`] 中尚不存在对应的文本 [`Entry`]，就分配一个。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Allocates a text [`Entry`] if it is not already present in the [`Cache`].
    ///
    /// </details>
    pub fn allocate(
        &mut self,
        font_system: &mut cosmic_text::FontSystem,
        key: Key<'_>,
    ) -> (KeyHash, &mut Entry) {
        let hash = key.hash(FxHasher::default());

        if let Some(hash) = self.aliases.get(&hash) {
            let _ = self.recently_used.insert(*hash);

            return (*hash, self.entries.get_mut(hash).unwrap());
        }

        if let hash_map::Entry::Vacant(entry) = self.entries.entry(hash) {
            let metrics =
                cosmic_text::Metrics::new(key.size, key.line_height.max(f32::MIN_POSITIVE));
            let mut buffer = cosmic_text::Buffer::new(font_system, metrics);

            let max_height = key.bounds.height.max(key.line_height);

            buffer.set_size(Some(key.bounds.width), Some(max_height));

            buffer.set_wrap(text::to_wrap(key.wrapping));
            buffer.set_ellipsize(text::to_ellipsize(key.ellipsis, max_height));

            buffer.set_text(
                key.content,
                &text::to_attributes(key.font),
                text::to_shaping(key.shaping, key.content),
                None,
            );
            buffer.shape_until_scroll(font_system, false);

            let bounds = text::align(&mut buffer, font_system, key.align_x);

            let _ = entry.insert(Entry {
                buffer,
                min_bounds: bounds,
            });

            for bounds in [
                bounds,
                Size {
                    width: key.bounds.width,
                    ..bounds
                },
            ] {
                if key.bounds != bounds {
                    let _ = self
                        .aliases
                        .insert(Key { bounds, ..key }.hash(FxHasher::default()), hash);
                }
            }
        }

        let _ = self.recently_used.insert(hash);

        (hash, self.entries.get_mut(&hash).unwrap())
    }

    /// 修剪 [`Cache`]。
    ///
    /// 这会清除自上一次 `trim` 以来未被使用过的文本片段。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Trims the [`Cache`].
    ///
    /// This will clear the sections of text that have not been used since the last `trim`.
    ///
    /// </details>
    pub fn trim(&mut self) {
        self.entries
            .retain(|key, _| self.recently_used.contains(key));

        self.aliases
            .retain(|_, value| self.recently_used.contains(value));

        self.recently_used.clear();
    }
}

/// 表示一段文本的缓存键。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A cache key representing a section of text.
///
/// </details>
#[derive(Debug, Clone, Copy)]
pub struct Key<'a> {
    /// 文本的内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The content of the text.
    ///
    /// </details>
    pub content: &'a str,
    /// 文本的大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The size of the text.
    ///
    /// </details>
    pub size: f32,
    /// 文本的行高。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The line height of the text.
    ///
    /// </details>
    pub line_height: f32,
    /// 文本的 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Font`] of the text.
    ///
    /// </details>
    pub font: Font,
    /// 文本的边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The bounds of the text.
    ///
    /// </details>
    pub bounds: Size,
    /// 文本的整形策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The shaping strategy of the text.
    ///
    /// </details>
    pub shaping: text::Shaping,
    /// 文本的对齐方式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The alignment of the text.
    ///
    /// </details>
    pub align_x: text::Alignment,
    /// 文本的换行策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The wrapping strategy of the text.
    ///
    /// </details>
    pub wrapping: text::Wrapping,
    /// 文本的省略策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The ellipsis strategy of the text.
    ///
    /// </details>
    pub ellipsis: text::Ellipsis,
}

impl Key<'_> {
    fn hash<H: Hasher>(self, mut hasher: H) -> KeyHash {
        self.content.hash(&mut hasher);
        self.size.to_bits().hash(&mut hasher);
        self.line_height.to_bits().hash(&mut hasher);
        self.font.hash(&mut hasher);
        self.bounds.width.to_bits().hash(&mut hasher);
        self.bounds.height.to_bits().hash(&mut hasher);
        self.shaping.hash(&mut hasher);
        self.align_x.hash(&mut hasher);

        hasher.finish()
    }
}

/// [`Key`] 的哈希值。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The hash of a [`Key`].
///
/// </details>
pub type KeyHash = u64;

/// 缓存条目。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A cache entry.
///
/// </details>
#[derive(Debug)]
pub struct Entry {
    /// 文本缓冲区，可直接用于绘制。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The buffer of text, ready for drawing.
    ///
    /// </details>
    pub buffer: cosmic_text::Buffer,
    /// 文本的最小边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The minimum bounds of the text.
    ///
    /// </details>
    pub min_bounds: Size,
}
