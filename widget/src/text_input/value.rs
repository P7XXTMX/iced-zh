use unicode_segmentation::UnicodeSegmentation;

/// [`TextInput`] 的值。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The value of a [`TextInput`].
///
/// </details>
///
/// [`TextInput`]: super::TextInput
// TODO: Reduce allocations, cache results (?)
// 待办：减少分配，缓存结果（？）
#[derive(Debug, Clone)]
pub struct Value {
    graphemes: Vec<String>,
}

impl Value {
    /// 从字符串切片创建一个新的 [`Value`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Value`] from a string slice.
    ///
    /// </details>
    pub fn new(string: &str) -> Self {
        let graphemes = UnicodeSegmentation::graphemes(string, true)
            .map(String::from)
            .collect();

        Self { graphemes }
    }

    /// 返回 [`Value`] 是否为空。
    ///
    /// 不包含任何字素的 [`Value`] 是空的。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns whether the [`Value`] is empty or not.
    ///
    /// A [`Value`] is empty when it contains no graphemes.
    ///
    /// </details>
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 返回 [`Value`] 中字素的总数。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the total amount of graphemes in the [`Value`].
    ///
    /// </details>
    pub fn len(&self) -> usize {
        self.graphemes.len()
    }

    /// 返回从给定字素 `index` 起，上一个单词开头的
    /// 位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the position of the previous start of a word from the given
    /// grapheme `index`.
    ///
    /// </details>
    pub fn previous_start_of_word(&self, index: usize) -> usize {
        let previous_string =
            &self.graphemes[..index.min(self.graphemes.len())].concat();

        UnicodeSegmentation::split_word_bound_indices(previous_string as &str)
            .filter(|(_, word)| !word.trim_start().is_empty())
            .next_back()
            .map(|(i, previous_word)| {
                index
                    - UnicodeSegmentation::graphemes(previous_word, true)
                        .count()
                    - UnicodeSegmentation::graphemes(
                        &previous_string[i + previous_word.len()..] as &str,
                        true,
                    )
                    .count()
            })
            .unwrap_or(0)
    }

    /// 返回从给定字素 `index` 起，下一个单词结尾的
    /// 位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the position of the next end of a word from the given grapheme
    /// `index`.
    ///
    /// </details>
    pub fn next_end_of_word(&self, index: usize) -> usize {
        let next_string = &self.graphemes[index..].concat();

        UnicodeSegmentation::split_word_bound_indices(next_string as &str)
            .find(|(_, word)| !word.trim_start().is_empty())
            .map(|(i, next_word)| {
                index
                    + UnicodeSegmentation::graphemes(next_word, true).count()
                    + UnicodeSegmentation::graphemes(
                        &next_string[..i] as &str,
                        true,
                    )
                    .count()
            })
            .unwrap_or(self.len())
    }

    /// 返回一个新的 [`Value`]，包含从 `start` 到
    /// 给定 `end` 的字素。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a new [`Value`] containing the graphemes from `start` until the
    /// given `end`.
    ///
    /// </details>
    pub fn select(&self, start: usize, end: usize) -> Self {
        let graphemes =
            self.graphemes[start.min(self.len())..end.min(self.len())].to_vec();

        Self { graphemes }
    }

    /// 返回一个新的 [`Value`]，包含给定 `index`
    /// 之前的字素。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a new [`Value`] containing the graphemes until the given
    /// `index`.
    ///
    /// </details>
    pub fn until(&self, index: usize) -> Self {
        let graphemes = self.graphemes[..index.min(self.len())].to_vec();

        Self { graphemes }
    }

    /// 在给定的字素 `index` 处插入一个新的 `char`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Inserts a new `char` at the given grapheme `index`.
    ///
    /// </details>
    pub fn insert(&mut self, index: usize, c: char) {
        self.graphemes.insert(index, c.to_string());

        self.graphemes =
            UnicodeSegmentation::graphemes(&self.to_string() as &str, true)
                .map(String::from)
                .collect();
    }

    /// 在给定的字素 `index` 处插入一批字素。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Inserts a bunch of graphemes at the given grapheme `index`.
    ///
    /// </details>
    pub fn insert_many(&mut self, index: usize, mut value: Value) {
        let _ = self
            .graphemes
            .splice(index..index, value.graphemes.drain(..));
    }

    /// 移除给定 `index` 处的字素。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Removes the grapheme at the given `index`.
    ///
    /// </details>
    pub fn remove(&mut self, index: usize) {
        let _ = self.graphemes.remove(index);
    }

    /// 移除从 `start` 到 `end` 的字素。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Removes the graphemes from `start` to `end`.
    ///
    /// </details>
    pub fn remove_many(&mut self, start: usize, end: usize) {
        let _ = self.graphemes.splice(start..end, std::iter::empty());
    }

    /// 返回一个新的 [`Value`]，其中所有字素都被替换为
    /// 圆点（'•'）字符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a new [`Value`] with all its graphemes replaced with the
    /// dot ('•') character.
    ///
    /// </details>
    pub fn secure(&self) -> Self {
        Self {
            graphemes: std::iter::repeat_n(
                String::from("•"),
                self.graphemes.len(),
            )
            .collect(),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.graphemes.concat())
    }
}
