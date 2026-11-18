//! 把部件的内部状态保存在一棵状态树中，以保证连续性。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Store internal widget state in a state tree to ensure continuity.
//!
//! </details>
use crate::Widget;

use std::any::{self, Any};
use std::borrow::Borrow;
use std::fmt;

/// 持久化的部件状态树。
///
/// 一个 [`Tree`] 通常与部件树中的某个特定部件相关联。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A persistent state widget tree.
///
/// A [`Tree`] is normally associated with a specific widget in the widget tree.
///
/// </details>
#[derive(Debug)]
pub struct Tree {
    /// [`Tree`] 的标签。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The tag of the [`Tree`].
    ///
    /// </details>
    pub tag: Tag,

    /// [`Tree`] 的 [`State`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`State`] of the [`Tree`].
    ///
    /// </details>
    pub state: State,

    /// [`Tree`] 的根部件的各个子级。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The children of the root widget of the [`Tree`].
    ///
    /// </details>
    pub children: Vec<Tree>,
}

impl Tree {
    /// 创建一个空的、无状态且没有子节点的 [`Tree`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates an empty, stateless [`Tree`] with no children.
    ///
    /// </details>
    pub fn empty() -> Self {
        Self {
            tag: Tag::stateless(),
            state: State::None,
            children: Vec::new(),
        }
    }

    /// 为给定的 [`Widget`] 创建一个新的 [`Tree`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Tree`] for the provided [`Widget`].
    ///
    /// </details>
    pub fn new<'a, Message, Theme, Renderer>(
        widget: impl Borrow<dyn Widget<Message, Theme, Renderer> + 'a>,
    ) -> Self
    where
        Renderer: crate::Renderer,
    {
        let widget = widget.borrow();

        Self {
            tag: widget.tag(),
            state: widget.state(),
            children: widget.children(),
        }
    }

    /// 把当前的树与给定的 [`Widget`] 进行协调。
    ///
    /// 如果 [`Widget`] 的标签与 [`Tree`] 的标签匹配，那么
    /// [`Widget`] 会继续进行协调（即调用 [`Widget::diff`]）。
    ///
    /// 否则，整棵 [`Tree`] 会被重新创建。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Reconciles the current tree with the provided [`Widget`].
    ///
    /// If the tag of the [`Widget`] matches the tag of the [`Tree`], then the
    /// [`Widget`] proceeds with the reconciliation (i.e. [`Widget::diff`] is called).
    ///
    /// Otherwise, the whole [`Tree`] is recreated.
    ///
    /// </details>
    ///
    /// [`Widget::diff`]: crate::Widget::diff
    pub fn diff<'a, Message, Theme, Renderer>(
        &mut self,
        new: impl Borrow<dyn Widget<Message, Theme, Renderer> + 'a>,
    ) where
        Renderer: crate::Renderer,
    {
        if self.tag == new.borrow().tag() {
            new.borrow().diff(self);
        } else {
            *self = Self::new(new);
        }
    }

    /// 把这棵树的子节点与给定的部件列表进行协调。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Reconciles the children of the tree with the provided list of widgets.
    ///
    /// </details>
    pub fn diff_children<'a, Message, Theme, Renderer>(
        &mut self,
        new_children: &[impl Borrow<dyn Widget<Message, Theme, Renderer> + 'a>],
    ) where
        Renderer: crate::Renderer,
    {
        self.diff_children_custom(
            new_children,
            |tree, widget| tree.diff(widget.borrow()),
            |widget| Self::new(widget.borrow()),
        );
    }

    /// 把这棵树的子节点与给定的部件列表进行协调，差异比较和
    /// 创建新的部件状态都使用自定义逻辑。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Reconciles the children of the tree with the provided list of widgets using custom
    /// logic both for diffing and creating new widget state.
    ///
    /// </details>
    pub fn diff_children_custom<T>(
        &mut self,
        new_children: &[T],
        diff: impl Fn(&mut Tree, &T),
        new_state: impl Fn(&T) -> Self,
    ) {
        if self.children.len() > new_children.len() {
            self.children.truncate(new_children.len());
        }

        for (child_state, new) in
            self.children.iter_mut().zip(new_children.iter())
        {
            diff(child_state, new);
        }

        if self.children.len() < new_children.len() {
            self.children.extend(
                new_children[self.children.len()..].iter().map(new_state),
            );
        }
    }
}

/// 把 `current_children` 与给定的部件列表进行协调，
/// 差异比较和创建新的部件状态都使用自定义逻辑。
///
/// 该算法会通过查询 `maybe_changed` 闭包，
/// 尽量减小差异比较带来的影响。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Reconciles the `current_children` with the provided list of widgets using
/// custom logic both for diffing and creating new widget state.
///
/// The algorithm will try to minimize the impact of diffing by querying the
/// `maybe_changed` closure.
///
/// </details>
pub fn diff_children_custom_with_search<T>(
    current_children: &mut Vec<Tree>,
    new_children: &[T],
    diff: impl Fn(&mut Tree, &T),
    maybe_changed: impl Fn(usize) -> bool,
    new_state: impl Fn(&T) -> Tree,
) {
    if new_children.is_empty() {
        current_children.clear();
        return;
    }

    if current_children.is_empty() {
        current_children.extend(new_children.iter().map(new_state));
        return;
    }

    let first_maybe_changed = maybe_changed(0);
    let last_maybe_changed = maybe_changed(current_children.len() - 1);

    if current_children.len() > new_children.len() {
        if !first_maybe_changed && last_maybe_changed {
            current_children.truncate(new_children.len());
        } else {
            let difference_index = if first_maybe_changed {
                0
            } else {
                (1..current_children.len())
                    .find(|&i| maybe_changed(i))
                    .unwrap_or(0)
            };

            let _ = current_children.splice(
                difference_index
                    ..difference_index
                        + (current_children.len() - new_children.len()),
                std::iter::empty(),
            );
        }
    }

    if current_children.len() < new_children.len() {
        let first_maybe_changed = maybe_changed(0);
        let last_maybe_changed = maybe_changed(current_children.len() - 1);

        if !first_maybe_changed && last_maybe_changed {
            current_children.extend(
                new_children[current_children.len()..].iter().map(new_state),
            );
        } else {
            let difference_index = if first_maybe_changed {
                0
            } else {
                (1..current_children.len())
                    .find(|&i| maybe_changed(i))
                    .unwrap_or(0)
            };

            let _ = current_children.splice(
                difference_index..difference_index,
                new_children[difference_index
                    ..difference_index
                        + (new_children.len() - current_children.len())]
                    .iter()
                    .map(new_state),
            );
        }
    }

    // TODO: Merge loop with extend logic (?)
    // 待办：把这个循环与 extend 的逻辑合并（？）
    for (child_state, new) in
        current_children.iter_mut().zip(new_children.iter())
    {
        diff(child_state, new);
    }
}

/// 部件状态的标识符。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The identifier of some widget state.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Tag(any::TypeId);

impl Tag {
    /// 为类型 `T` 的状态创建一个 [`Tag`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Tag`] for a state of type `T`.
    ///
    /// </details>
    pub fn of<T>() -> Self
    where
        T: 'static,
    {
        Self(any::TypeId::of::<T>())
    }

    /// 为无状态的部件创建一个 [`Tag`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Tag`] for a stateless widget.
    ///
    /// </details>
    pub fn stateless() -> Self {
        Self::of::<()>()
    }
}

/// 部件的内部 [`State`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The internal [`State`] of a widget.
///
/// </details>
pub enum State {
    /// 没有有意义的内部状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// No meaningful internal state.
    ///
    /// </details>
    None,

    /// 有某种有意义的内部状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Some meaningful internal state.
    ///
    /// </details>
    Some(Box<dyn Any>),
}

impl State {
    /// 创建一个新的 [`State`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`State`].
    ///
    /// </details>
    pub fn new<T>(state: T) -> Self
    where
        T: 'static,
    {
        State::Some(Box::new(state))
    }

    /// 把 [`State`] 向下转型为 `T`，并返回它的引用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Downcasts the [`State`] to `T` and returns a reference to it.
    ///
    /// </details>
    ///
    /// # Panics
    /// 如果向下转型失败，或者 [`State`] 是 [`State::None`]，此方法会 panic。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// This method will panic if the downcast fails or the [`State`] is [`State::None`].
    ///
    /// </details>
    pub fn downcast_ref<T>(&self) -> &T
    where
        T: 'static,
    {
        match self {
            State::None => panic!("Downcast on stateless state"),
            State::Some(state) => {
                state.downcast_ref().expect("Downcast widget state")
            }
        }
    }

    /// 把 [`State`] 向下转型为 `T`，并返回它的可变引用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Downcasts the [`State`] to `T` and returns a mutable reference to it.
    ///
    /// </details>
    ///
    /// # Panics
    /// 如果向下转型失败，或者 [`State`] 是 [`State::None`]，此方法会 panic。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// This method will panic if the downcast fails or the [`State`] is [`State::None`].
    ///
    /// </details>
    pub fn downcast_mut<T>(&mut self) -> &mut T
    where
        T: 'static,
    {
        match self {
            State::None => panic!("Downcast on stateless state"),
            State::Some(state) => {
                state.downcast_mut().expect("Downcast widget state")
            }
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "State::None"),
            Self::Some(_) => write!(f, "State::Some"),
        }
    }
}
