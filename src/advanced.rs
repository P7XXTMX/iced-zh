//! 使用自定义部件等高级概念。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Leverage advanced concepts like custom widgets.
//!
//! </details>
pub mod subscription {
    //! 编写你自己的订阅。
    //!
    //! <details>
    //! <summary>显示英文原文</summary>
    //!
    //! Write your own subscriptions.
    //!
    //! </details>
    pub use crate::runtime::futures::subscription::{
        Event, EventStream, Hasher, Recipe, from_recipe, into_recipes,
    };
}

pub mod widget {
    //! 创建自定义部件并对它们执行操作。
    //!
    //! <details>
    //! <summary>显示英文原文</summary>
    //!
    //! Create custom widgets and operate on them.
    //!
    //! </details>
    pub use crate::core::widget::*;
    pub use crate::runtime::task::widget as operate;
}

pub use crate::core::Shell;
pub use crate::core::clipboard::{self, Clipboard};
pub use crate::core::image;
pub use crate::core::input_method::{self, InputMethod};
pub use crate::core::layout::{self, Layout};
pub use crate::core::mouse;
pub use crate::core::overlay::{self, Overlay};
pub use crate::core::renderer::{self, Renderer};
pub use crate::core::svg;
pub use crate::core::text::{self, Text};
pub use crate::renderer::graphics;

pub use widget::Widget;
