//! 缓存计算结果并高效地复用它们。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Cache computations and efficiently reuse them.
//!
//! </details>
use std::cell::RefCell;
use std::fmt;
use std::mem;
use std::sync::atomic::{self, AtomicU64};

/// 一个简单的缓存，存储已生成的值以避免重复计算。
///
/// 清空之后仍会记录最后一次生成的值。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A simple cache that stores generated values to avoid recomputation.
///
/// Keeps track of the last generated value after clearing.
///
/// </details>
pub struct Cache<T> {
    group: Group,
    state: RefCell<State<T>>,
}

impl<T> Cache<T> {
    /// 创建一个新的空 [`Cache`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new empty [`Cache`].
    ///
    /// </details>
    pub fn new() -> Self {
        Cache {
            group: Group::singleton(),
            state: RefCell::new(State::Empty { previous: None }),
        }
    }

    /// 使用给定的 [`Group`] 创建一个新的空 [`Cache`]。
    ///
    /// 同一分组内的缓存可以复用内部的渲染存储。
    ///
    /// 通常应当把很可能一起变化的缓存
    /// 归为一组。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new empty [`Cache`] with the given [`Group`].
    ///
    /// Caches within the same group may reuse internal rendering storage.
    ///
    /// You should generally group caches that are likely to change
    /// together.
    ///
    /// </details>
    pub fn with_group(group: Group) -> Self {
        assert!(
            !group.is_singleton(),
            "The group {group:?} cannot be shared!"
        );

        Cache {
            group,
            state: RefCell::new(State::Empty { previous: None }),
        }
    }

    /// 返回 [`Cache`] 的 [`Group`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Group`] of the [`Cache`].
    ///
    /// </details>
    pub fn group(&self) -> Group {
        self.group
    }

    /// 把给定的值放入 [`Cache`]。
    ///
    /// 注意，由于这是一个缓存，调用此方法并不需要
    /// 可变引用。你可以放心地在渲染代码中
    /// 更新缓存。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Puts the given value in the [`Cache`].
    ///
    /// Notice that, given this is a cache, a mutable reference is not
    /// necessary to call this method. You can safely update the cache in
    /// rendering code.
    ///
    /// </details>
    pub fn put(&self, value: T) {
        *self.state.borrow_mut() = State::Filled { current: value };
    }

    /// 返回指向 [`Cache`] 内部 [`State`] 的引用单元（reference cell）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a reference cell to the internal [`State`] of the [`Cache`].
    ///
    /// </details>
    pub fn state(&self) -> &RefCell<State<T>> {
        &self.state
    }

    /// 清空 [`Cache`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Clears the [`Cache`].
    ///
    /// </details>
    pub fn clear(&self) {
        let mut state = self.state.borrow_mut();

        let previous =
            mem::replace(&mut *state, State::Empty { previous: None });

        let previous = match previous {
            State::Empty { previous } => previous,
            State::Filled { current } => Some(current),
        };

        *state = State::Empty { previous };
    }
}

/// 缓存分组。
///
/// 属于同一分组的缓存通常会一起变化。
///
/// 缓存分组可用于在渲染时实现某些性能优化，
/// 例如批处理或共享图集。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A cache group.
///
/// Caches that share the same group generally change together.
///
/// A cache group can be used to implement certain performance
/// optimizations during rendering, like batching or sharing atlases.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Group {
    id: u64,
    is_singleton: bool,
}

impl Group {
    /// 生成一个新的唯一缓存 [`Group`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Generates a new unique cache [`Group`].
    ///
    /// </details>
    pub fn unique() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);

        Self {
            id: NEXT.fetch_add(1, atomic::Ordering::Relaxed),
            is_singleton: false,
        }
    }

    /// 如果 [`Group`] 中永远只可能有
    /// 一个 [`Cache`]，返回 `true`。
    ///
    /// 这是使用 [`Cache::new`] 时
    /// 默认分配的 [`Group`] 种类。
    ///
    /// 知道某个 [`Group`] 永远不会被共享，
    /// 可能有助于渲染后端执行额外的
    /// 优化。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns `true` if the [`Group`] can only ever have a
    /// single [`Cache`] in it.
    ///
    /// This is the default kind of [`Group`] assigned when using
    /// [`Cache::new`].
    ///
    /// Knowing that a [`Group`] will never be shared may be
    /// useful for rendering backends to perform additional
    /// optimizations.
    ///
    /// </details>
    pub fn is_singleton(self) -> bool {
        self.is_singleton
    }

    fn singleton() -> Self {
        Self {
            is_singleton: true,
            ..Self::unique()
        }
    }
}

impl<T> fmt::Debug for Cache<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::ops::Deref;

        let state = self.state.borrow();

        match state.deref() {
            State::Empty { previous } => {
                write!(f, "Cache::Empty {{ previous: {previous:?} }}")
            }
            State::Filled { current } => {
                write!(f, "Cache::Filled {{ current: {current:?} }}")
            }
        }
    }
}

impl<T> Default for Cache<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// [`Cache`] 的状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The state of a [`Cache`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State<T> {
    /// [`Cache`] 为空。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Cache`] is empty.
    ///
    /// </details>
    Empty {
        /// [`Cache`] 先前的值。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The previous value of the [`Cache`].
        ///
        /// </details>
        previous: Option<T>,
    },
    /// [`Cache`] 已填充。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Cache`] is filled.
    ///
    /// </details>
    Filled {
        /// [`Cache`] 当前的值
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The current value of the [`Cache`]
        ///
        /// </details>
        current: T,
    },
}

/// 可以被缓存的一份数据。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A piece of data that can be cached.
///
/// </details>
pub trait Cached: Sized {
    /// 所产生的缓存的类型。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The type of cache produced.
    ///
    /// </details>
    type Cache: Clone;

    /// 把 [`Cache`] 加载为一个正式的实例。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Loads the [`Cache`] into a proper instance.
    ///
    /// </details>
    ///
    /// [`Cache`]: Self::Cache
    fn load(cache: &Self::Cache) -> Self;

    /// 缓存这个值，产生与之对应的 [`Cache`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Caches this value, producing its corresponding [`Cache`].
    ///
    /// </details>
    ///
    /// [`Cache`]: Self::Cache
    fn cache(self, group: Group, previous: Option<Self::Cache>) -> Self::Cache;
}

#[cfg(debug_assertions)]
impl Cached for () {
    type Cache = ();

    fn load(_cache: &Self::Cache) -> Self {}

    fn cache(
        self,
        _group: Group,
        _previous: Option<Self::Cache>,
    ) -> Self::Cache {
    }
}
