use std::borrow;
use std::sync::atomic::{self, AtomicUsize};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// 通用部件的标识符。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The identifier of a generic widget.
///
/// </details>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(Internal);

impl Id {
    /// 根据一个静态 `str` 创建一个新的 [`Id`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Id`] from a static `str`.
    ///
    /// </details>
    pub const fn new(id: &'static str) -> Self {
        Self(Internal::Custom(borrow::Cow::Borrowed(id)))
    }

    /// 创建一个唯一的 [`Id`]。
    ///
    /// 此函数每次调用都会产生一个不同的 [`Id`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a unique [`Id`].
    ///
    /// This function produces a different [`Id`] every time it is called.
    ///
    /// </details>
    pub fn unique() -> Self {
        let id = NEXT_ID.fetch_add(1, atomic::Ordering::Relaxed);

        Self(Internal::Unique(id))
    }
}

impl From<&'static str> for Id {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Id {
    fn from(value: String) -> Self {
        Self(Internal::Custom(borrow::Cow::Owned(value)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Internal {
    Unique(usize),
    Custom(borrow::Cow<'static, str>),
}

#[cfg(test)]
mod tests {
    use super::Id;

    #[test]
    fn unique_generates_different_ids() {
        let a = Id::unique();
        let b = Id::unique();

        assert_ne!(a, b);
    }
}
