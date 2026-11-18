use crate::core::{self, Element};
use crate::lazy::component;

use std::hash::Hash;

#[allow(deprecated)]
pub use crate::lazy::{Component, Lazy};

/// 使用给定的数据 `Dependency` 和一个能把该数据转换为部件树的
/// 闭包，创建一个新的 [`Lazy`] 部件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Lazy`] widget with the given data `Dependency` and a
/// closure that can turn this data into a widget tree.
///
/// </details>
#[cfg(feature = "lazy")]
pub fn lazy<'a, Message, Theme, Renderer, Dependency, View>(
    dependency: Dependency,
    view: impl Fn(&Dependency) -> View + 'a,
) -> Lazy<'a, Message, Theme, Renderer, Dependency, View>
where
    Dependency: Hash + 'a,
    View: Into<Element<'static, Message, Theme, Renderer>>,
{
    Lazy::new(dependency, view)
}

/// 把 [`Component`] 的实现者转换成一个 [`Element`]，
/// 它可以嵌入任何应用程序。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Turns an implementor of [`Component`] into an [`Element`] that can be
/// embedded in any application.
///
/// </details>
#[cfg(feature = "lazy")]
#[deprecated(
    since = "0.13.0",
    note = "components introduce encapsulated state and hamper the use of a single source of truth. \
    Instead, leverage the Elm Architecture directly, or implement a custom widget"
)]
#[allow(deprecated)]
pub fn component<'a, C, Message, Theme, Renderer>(
    component: C,
) -> Element<'a, Message, Theme, Renderer>
where
    C: Component<Message, Theme, Renderer> + 'a,
    C::State: 'static,
    Message: 'a,
    Theme: 'a,
    Renderer: core::Renderer + 'a,
{
    component::view(component)
}
