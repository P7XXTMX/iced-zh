//! 查询或更新部件的内部状态。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Query or update internal widget state.
//!
//! </details>
pub mod focusable;
pub mod scrollable;
pub mod text;
pub mod text_input;

pub use focusable::Focusable;
pub use scrollable::Scrollable;
pub use text::Text;
pub use text_input::TextInput;

use crate::widget::Id;
use crate::{Rectangle, Size, Vector};

use std::any::Any;
use std::fmt;
use std::sync::Arc;

/// 一段可以遍历应用程序部件树的逻辑，
/// 用于查询或更新某些部件状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A piece of logic that can traverse the widget tree of an application in
/// order to query or update some widget state.
///
/// </details>
pub trait Operation<T = ()>: Send {
    /// 请求继续遍历部件树，以便继续操作。
    ///
    /// [`Operation`] 可以调用给定的 `operate` 闭包，
    /// 把控制权交还给部件树并继续遍历。如果
    /// 没有调用该闭包，请求遍历的那个部件的子部件
    /// 将被跳过。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Requests further traversal of the widget tree to keep operating.
    ///
    /// The provided `operate` closure may be called by an [`Operation`]
    /// to return control to the widget tree and keep traversing it. If
    /// the closure is not called, the children of the widget asking for
    /// traversal will be skipped.
    ///
    /// </details>
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<T>));

    /// 对包含其他部件的部件进行操作。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Operates on a widget that contains other widgets.
    ///
    /// </details>
    fn container(&mut self, _id: Option<&Id>, _bounds: Rectangle, _viewport: &Rectangle) {}

    /// 对可以滚动的部件进行操作。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Operates on a widget that can be scrolled.
    ///
    /// </details>
    fn scrollable(
        &mut self,
        _id: Option<&Id>,
        _bounds: Rectangle,
        _content: Size,
        _translation: Vector,
        _state: &mut dyn Scrollable,
    ) {
    }

    /// 对可以获得焦点的部件进行操作。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Operates on a widget that can be focused.
    ///
    /// </details>
    fn focusable(&mut self, _id: Option<&Id>, _bounds: Rectangle, _state: &mut dyn Focusable) {}

    /// 对具有文本输入的部件进行操作。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Operates on a widget that has text input.
    ///
    /// </details>
    fn text_input(&mut self, _id: Option<&Id>, _bounds: Rectangle, _state: &mut dyn TextInput) {}

    /// 对包含文本的部件进行操作。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Operates on a widget that contains some text.
    ///
    /// </details>
    fn text(&mut self, _id: Option<&Id>, _bounds: Rectangle, _state: &mut dyn Text) {}

    /// 对带有某种状态的自定义部件进行操作。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Operates on a custom widget with some state.
    ///
    /// </details>
    fn custom(&mut self, _id: Option<&Id>, _bounds: Rectangle, _state: &mut dyn Any) {}

    /// 结束 [`Operation`] 并返回它的 [`Outcome`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Finishes the [`Operation`] and returns its [`Outcome`].
    ///
    /// </details>
    fn finish(&self) -> Outcome<T> {
        Outcome::None
    }
}

impl<T, O> Operation<O> for Box<T>
where
    T: Operation<O> + ?Sized,
{
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<O>)) {
        self.as_mut().traverse(operate);
    }

    fn container(&mut self, id: Option<&Id>, bounds: Rectangle, viewport: &Rectangle) {
        self.as_mut().container(id, bounds, viewport);
    }

    fn focusable(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Focusable) {
        self.as_mut().focusable(id, bounds, state);
    }

    fn scrollable(
        &mut self,
        id: Option<&Id>,
        bounds: Rectangle,
        content: Size,
        translation: Vector,
        state: &mut dyn Scrollable,
    ) {
        self.as_mut()
            .scrollable(id, bounds, content, translation, state);
    }

    fn text_input(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn TextInput) {
        self.as_mut().text_input(id, bounds, state);
    }

    fn text(&mut self, id: Option<&Id>, bounds: Rectangle, text: &mut dyn Text) {
        self.as_mut().text(id, bounds, text);
    }

    fn custom(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Any) {
        self.as_mut().custom(id, bounds, state);
    }

    fn finish(&self) -> Outcome<O> {
        self.as_ref().finish()
    }
}

/// [`Operation`] 的结果。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The result of an [`Operation`].
///
/// </details>
pub enum Outcome<T> {
    /// [`Operation`] 没有产生结果。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Operation`] produced no result.
    ///
    /// </details>
    None,

    /// [`Operation`] 产生了某个结果。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Operation`] produced some result.
    ///
    /// </details>
    Some(T),

    /// [`Operation`] 之后需要接着执行另一个 [`Operation`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Operation`] needs to be followed by another [`Operation`].
    ///
    /// </details>
    Chain(Box<dyn Operation<T>>),
}

impl<T> fmt::Debug for Outcome<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "Outcome::None"),
            Self::Some(output) => write!(f, "Outcome::Some({output:?})"),
            Self::Chain(_) => write!(f, "Outcome::Chain(...)"),
        }
    }
}

/// 应用于状态变化的动画，由
/// [`Operation`] 请求。
///
/// 有些部件既可以立即应用状态变化，也可以带
/// 动画地应用；对这些部件的操作接受一个 [`Animation`]
/// 来选择使用哪一种。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The animation to apply to a state change, as requested by an
/// [`Operation`].
///
/// Some widgets can apply a state change either immediately or with an
/// animation; operations on those widgets take an [`Animation`] to select
/// which.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Animation {
    /// 使用部件的默认动画。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Use the default animation of the widget.
    ///
    /// </details>
    #[default]
    Auto,

    /// 立即应用变化。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Apply the change immediately.
    ///
    /// </details>
    Instant,

    /// 以动画方式过渡到新状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Animate the change towards the new state.
    ///
    /// </details>
    Smooth,
}

/// 把 [`Operation`] 包装进一个黑盒，擦除它的返回类型。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Wraps the [`Operation`] in a black box, erasing its returning type.
///
/// </details>
pub fn black_box<'a, T, O>(operation: &'a mut dyn Operation<T>) -> impl Operation<O> + 'a
where
    T: 'a,
{
    struct BlackBox<'a, T> {
        operation: &'a mut dyn Operation<T>,
    }

    impl<T, O> Operation<O> for BlackBox<'_, T> {
        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<O>))
        where
            Self: Sized,
        {
            self.operation.traverse(&mut |operation| {
                operate(&mut BlackBox { operation });
            });
        }

        fn container(&mut self, id: Option<&Id>, bounds: Rectangle, viewport: &Rectangle) {
            self.operation.container(id, bounds, viewport);
        }

        fn focusable(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Focusable) {
            self.operation.focusable(id, bounds, state);
        }

        fn scrollable(
            &mut self,
            id: Option<&Id>,
            bounds: Rectangle,
            content: Size,
            translation: Vector,
            state: &mut dyn Scrollable,
        ) {
            self.operation
                .scrollable(id, bounds, content, translation, state);
        }

        fn text_input(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn TextInput) {
            self.operation.text_input(id, bounds, state);
        }

        fn text(&mut self, id: Option<&Id>, bounds: Rectangle, text: &mut dyn Text) {
            self.operation.text(id, bounds, text);
        }

        fn custom(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Any) {
            self.operation.custom(id, bounds, state);
        }

        fn finish(&self) -> Outcome<O> {
            Outcome::None
        }
    }

    BlackBox { operation }
}

/// 使用给定的函数映射 [`Operation`] 的输出。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Maps the output of an [`Operation`] using the given function.
///
/// </details>
pub fn map<A, B>(
    operation: impl Operation<A>,
    f: impl Fn(A) -> B + Send + Sync + 'static,
) -> impl Operation<B>
where
    A: 'static,
    B: 'static,
{
    struct Map<O, A, B> {
        operation: O,
        f: Arc<dyn Fn(A) -> B + Send + Sync>,
    }

    impl<O, A, B> Operation<B> for Map<O, A, B>
    where
        O: Operation<A>,
        A: 'static,
        B: 'static,
    {
        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<B>)) {
            struct MapRef<'a, A> {
                operation: &'a mut dyn Operation<A>,
            }

            impl<A, B> Operation<B> for MapRef<'_, A> {
                fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<B>)) {
                    self.operation.traverse(&mut |operation| {
                        operate(&mut MapRef { operation });
                    });
                }

                fn container(&mut self, id: Option<&Id>, bounds: Rectangle, viewport: &Rectangle) {
                    let Self { operation, .. } = self;

                    operation.container(id, bounds, viewport);
                }

                fn scrollable(
                    &mut self,
                    id: Option<&Id>,
                    bounds: Rectangle,
                    content: Size,
                    translation: Vector,
                    state: &mut dyn Scrollable,
                ) {
                    self.operation
                        .scrollable(id, bounds, content, translation, state);
                }

                fn focusable(
                    &mut self,
                    id: Option<&Id>,
                    bounds: Rectangle,
                    state: &mut dyn Focusable,
                ) {
                    self.operation.focusable(id, bounds, state);
                }

                fn text_input(
                    &mut self,
                    id: Option<&Id>,
                    bounds: Rectangle,
                    state: &mut dyn TextInput,
                ) {
                    self.operation.text_input(id, bounds, state);
                }

                fn text(&mut self, id: Option<&Id>, bounds: Rectangle, text: &mut dyn Text) {
                    self.operation.text(id, bounds, text);
                }

                fn custom(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Any) {
                    self.operation.custom(id, bounds, state);
                }
            }

            self.operation.traverse(&mut |operation| {
                operate(&mut MapRef { operation });
            });
        }

        fn container(&mut self, id: Option<&Id>, bounds: Rectangle, viewport: &Rectangle) {
            self.operation.container(id, bounds, viewport);
        }

        fn focusable(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Focusable) {
            self.operation.focusable(id, bounds, state);
        }

        fn scrollable(
            &mut self,
            id: Option<&Id>,
            bounds: Rectangle,
            content: Size,
            translation: Vector,
            state: &mut dyn Scrollable,
        ) {
            self.operation
                .scrollable(id, bounds, content, translation, state);
        }

        fn text_input(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn TextInput) {
            self.operation.text_input(id, bounds, state);
        }

        fn text(&mut self, id: Option<&Id>, bounds: Rectangle, text: &mut dyn Text) {
            self.operation.text(id, bounds, text);
        }

        fn custom(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Any) {
            self.operation.custom(id, bounds, state);
        }

        fn finish(&self) -> Outcome<B> {
            match self.operation.finish() {
                Outcome::None => Outcome::None,
                Outcome::Some(output) => Outcome::Some((self.f)(output)),
                Outcome::Chain(next) => Outcome::Chain(Box::new(Map {
                    operation: next,
                    f: self.f.clone(),
                })),
            }
        }
    }

    Map {
        operation,
        f: Arc::new(f),
    }
}

/// 把 [`Operation`] 的输出与给定的函数串联起来，
/// 构建一个新的 [`Operation`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Chains the output of an [`Operation`] with the provided function to
/// build a new [`Operation`].
///
/// </details>
pub fn then<A, B, O>(
    operation: impl Operation<A> + 'static,
    next: impl Fn(A) -> O + Send + Sync + 'static,
) -> impl Operation<B>
where
    A: 'static,
    B: 'static,
    O: Operation<B> + 'static,
{
    struct Chain<T, O, A> {
        operation: T,
        next: Arc<dyn Fn(A) -> O + Send + Sync>,
    }

    impl<T, O, A, B> Operation<B> for Chain<T, O, A>
    where
        T: Operation<A> + 'static,
        O: Operation<B> + 'static,
        A: 'static,
        B: 'static,
    {
        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<B>)) {
            self.operation.traverse(&mut |operation| {
                operate(&mut black_box(operation));
            });
        }

        fn container(&mut self, id: Option<&Id>, bounds: Rectangle, viewport: &Rectangle) {
            self.operation.container(id, bounds, viewport);
        }

        fn focusable(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Focusable) {
            self.operation.focusable(id, bounds, state);
        }

        fn scrollable(
            &mut self,
            id: Option<&Id>,
            bounds: Rectangle,
            content: Size,
            translation: crate::Vector,
            state: &mut dyn Scrollable,
        ) {
            self.operation
                .scrollable(id, bounds, content, translation, state);
        }

        fn text_input(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn TextInput) {
            self.operation.text_input(id, bounds, state);
        }

        fn text(&mut self, id: Option<&Id>, bounds: Rectangle, text: &mut dyn Text) {
            self.operation.text(id, bounds, text);
        }

        fn custom(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Any) {
            self.operation.custom(id, bounds, state);
        }

        fn finish(&self) -> Outcome<B> {
            match self.operation.finish() {
                Outcome::None => Outcome::None,
                Outcome::Some(value) => Outcome::Chain(Box::new((self.next)(value))),
                Outcome::Chain(next) => Outcome::Chain(Box::new(Chain {
                    operation: next,
                    next: self.next.clone(),
                })),
            }
        }
    }

    Chain {
        operation,
        next: Arc::new(next),
    }
}

/// 产生一个 [`Operation`]，它把给定的 [`Operation`] 应用到
/// 具有给定 [`Id`] 的容器的子部件上。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Produces an [`Operation`] that applies the given [`Operation`] to the
/// children of a container with the given [`Id`].
///
/// </details>
pub fn scope<T: 'static>(target: Id, operation: impl Operation<T> + 'static) -> impl Operation<T> {
    struct ScopedOperation<Message> {
        target: Id,
        current: Option<Id>,
        operation: Box<dyn Operation<Message>>,
    }

    impl<Message: 'static> Operation<Message> for ScopedOperation<Message> {
        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<Message>)) {
            if self.current.as_ref() == Some(&self.target) {
                self.operation.as_mut().traverse(operate);
            } else {
                operate(self);
            }

            self.current = None;
        }

        fn container(&mut self, id: Option<&Id>, _bounds: Rectangle, _viewport: &Rectangle) {
            self.current = id.cloned();
        }

        fn finish(&self) -> Outcome<Message> {
            match self.operation.finish() {
                Outcome::Chain(next) => Outcome::Chain(Box::new(ScopedOperation {
                    target: self.target.clone(),
                    current: None,
                    operation: next,
                })),
                outcome => outcome,
            }
        }
    }

    ScopedOperation {
        target,
        current: None,
        operation: Box::new(operation),
    }
}
