//! 使用 Elm 架构构建并复用自定义部件。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Build and reuse custom widgets using The Elm Architecture.
//!
//! </details>
#![allow(deprecated)]
use crate::core::layout::{self, Layout};
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::widget;
use crate::core::widget::tree::{self, Tree};
use crate::core::{
    self, Clipboard, Element, Length, Rectangle, Shell, Size, Vector, Widget,
};

use ouroboros::self_referencing;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

/// 使用 Elm 架构的可复用自定义部件。
///
/// [`Component`] 让你可以像编写带有封装状态的 `iced` 应用程序
/// 那样实现自定义部件。
///
/// 换句话说，[`Component`] 让你可以把 `iced` 应用程序变成
/// 自定义部件，并且无需繁琐的接线就能嵌入它们。
///
/// [`Component`] 产生的部件可以触发 [`Event`](Component::Event)
/// 并更新 [`Component`] 的内部状态。
///
/// 此外，[`Component`] 还能产生 `Message`，把相关的交互
/// 通知给父应用程序。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A reusable, custom widget that uses The Elm Architecture.
///
/// A [`Component`] allows you to implement custom widgets as if they were
/// `iced` applications with encapsulated state.
///
/// In other words, a [`Component`] allows you to turn `iced` applications into
/// custom widgets and embed them without cumbersome wiring.
///
/// A [`Component`] produces widgets that may fire an [`Event`](Component::Event)
/// and update the internal state of the [`Component`].
///
/// Additionally, a [`Component`] is capable of producing a `Message` to notify
/// the parent application of any relevant interactions.
///
/// </details>
///
/// # 状态（State）
/// 组件可以用两种方式存储状态：要么作为特质实现者
/// 内部的数据，要么放在由运行时管理、并提供给特质方法的
/// [`State`][Component::State] 类型中。这两种方式并不
/// 互斥，而且优缺点正好相反。
///
/// 例如，如果某个状态被位于部件树不同分支的多个组件所需要，
/// 那么让它们的公共祖先存储该状态并向下传递
/// 会更方便。
///
/// 另一方面，如果某个状态只被组件自身需要，
/// 你可以把它存为其内部 [`State`][Component::State] 的一部分。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A component can store its state in one of two ways: either as data within the
/// implementor of the trait, or in a type [`State`][Component::State] that is managed
/// by the runtime and provided to the trait methods. These two approaches are not
/// mutually exclusive and have opposite pros and cons.
///
/// For instance, if a piece of state is needed by multiple components that reside
/// in different branches of the tree, then it's more convenient to let a common
/// ancestor store it and pass it down.
///
/// On the other hand, if a piece of state is only needed by the component itself,
/// you can store it as part of its internal [`State`][Component::State].
///
/// </details>
#[cfg(feature = "lazy")]
#[deprecated(
    since = "0.13.0",
    note = "components introduce encapsulated state and hamper the use of a single source of truth. \
    Instead, leverage the Elm Architecture directly, or implement a custom widget"
)]
pub trait Component<Message, Theme = crate::Theme, Renderer = crate::Renderer> {
    /// 此 [`Component`] 的内部状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The internal state of this [`Component`].
    ///
    /// </details>
    type State: Default;

    /// 此 [`Component`] 在内部处理的事件类型。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The type of event this [`Component`] handles internally.
    ///
    /// </details>
    type Event;

    /// 处理一个 [`Event`](Component::Event)，并相应地更新 [`Component`] 的状态。
    ///
    /// 它可以为父应用程序产生一条 `Message`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Processes an [`Event`](Component::Event) and updates the [`Component`] state accordingly.
    ///
    /// It can produce a `Message` for the parent application.
    ///
    /// </details>
    fn update(
        &mut self,
        state: &mut Self::State,
        event: Self::Event,
    ) -> Option<Message>;

    /// 产生 [`Component`] 的部件，这些部件可以在用户交互时
    /// 触发 [`Event`](Component::Event)。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Produces the widgets of the [`Component`], which may trigger an [`Event`](Component::Event)
    /// on user interaction.
    ///
    /// </details>
    fn view(
        &self,
        state: &Self::State,
    ) -> Element<'_, Self::Event, Theme, Renderer>;

    /// 根据给定的 [`Operation`](widget::Operation) 更新 [`Component`] 的状态
    ///
    /// 默认情况下，它什么都不做。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Update the [`Component`] state based on the provided [`Operation`](widget::Operation)
    ///
    /// By default, it does nothing.
    ///
    /// </details>
    fn operate(
        &self,
        _bounds: Rectangle,
        _state: &mut Self::State,
        _operation: &mut dyn widget::Operation,
    ) {
    }

    /// 返回用于布局 [`Component`] 的 [`Size`] 提示。
    ///
    /// 某些部件容器可能会在构造期间用这个提示来调整
    /// 它们的尺寸策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a [`Size`] hint for laying out the [`Component`].
    ///
    /// This hint may be used by some widget containers to adjust their sizing strategy
    /// during construction.
    ///
    /// </details>
    fn size_hint(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }
}

struct Tag<T>(T);

/// 把 [`Component`] 的实现者转换为可以嵌入任何应用程序的
/// [`Element`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Turns an implementor of [`Component`] into an [`Element`] that can be
/// embedded in any application.
///
/// </details>
pub fn view<'a, C, Message, Theme, Renderer>(
    component: C,
) -> Element<'a, Message, Theme, Renderer>
where
    C: Component<Message, Theme, Renderer> + 'a,
    C::State: 'static,
    Message: 'a,
    Theme: 'a,
    Renderer: core::Renderer + 'a,
{
    Element::new(Instance {
        state: RefCell::new(Some(
            StateBuilder {
                component: Box::new(component),
                message: PhantomData,
                state: PhantomData,
                element_builder: |_| None,
            }
            .build(),
        )),
        tree: RefCell::new(Rc::new(RefCell::new(None))),
    })
}

struct Instance<'a, Message, Theme, Renderer, Event, S> {
    state: RefCell<Option<State<'a, Message, Theme, Renderer, Event, S>>>,
    tree: RefCell<Rc<RefCell<Option<Tree>>>>,
}

#[self_referencing]
struct State<'a, Message: 'a, Theme: 'a, Renderer: 'a, Event: 'a, S: 'a> {
    component: Box<
        dyn Component<Message, Theme, Renderer, Event = Event, State = S> + 'a,
    >,
    message: PhantomData<Message>,
    state: PhantomData<S>,

    #[borrows(component)]
    #[covariant]
    element: Option<Element<'this, Event, Theme, Renderer>>,
}

impl<Message, Theme, Renderer, Event, S>
    Instance<'_, Message, Theme, Renderer, Event, S>
where
    S: Default + 'static,
    Renderer: renderer::Renderer,
{
    fn diff_self(&self) {
        self.with_element(|element| {
            self.tree
                .borrow_mut()
                .borrow_mut()
                .as_mut()
                .unwrap()
                .diff_children(std::slice::from_ref(&element));
        });
    }

    fn rebuild_element_if_necessary(&self) {
        let inner = self.state.borrow_mut().take().unwrap();
        if inner.borrow_element().is_none() {
            let heads = inner.into_heads();

            *self.state.borrow_mut() = Some(
                StateBuilder {
                    component: heads.component,
                    message: PhantomData,
                    state: PhantomData,
                    element_builder: |component| {
                        Some(
                            component.view(
                                self.tree
                                    .borrow()
                                    .borrow()
                                    .as_ref()
                                    .unwrap()
                                    .state
                                    .downcast_ref::<S>(),
                            ),
                        )
                    },
                }
                .build(),
            );
            self.diff_self();
        } else {
            *self.state.borrow_mut() = Some(inner);
        }
    }

    fn rebuild_element_with_operation(
        &self,
        layout: Layout<'_>,
        operation: &mut dyn widget::Operation,
    ) {
        let heads = self.state.borrow_mut().take().unwrap().into_heads();

        heads.component.operate(
            layout.bounds(),
            self.tree
                .borrow_mut()
                .borrow_mut()
                .as_mut()
                .unwrap()
                .state
                .downcast_mut(),
            operation,
        );

        *self.state.borrow_mut() = Some(
            StateBuilder {
                component: heads.component,
                message: PhantomData,
                state: PhantomData,
                element_builder: |component| {
                    Some(
                        component.view(
                            self.tree
                                .borrow()
                                .borrow()
                                .as_ref()
                                .unwrap()
                                .state
                                .downcast_ref(),
                        ),
                    )
                },
            }
            .build(),
        );
        self.diff_self();
    }

    fn with_element<T>(
        &self,
        f: impl FnOnce(&Element<'_, Event, Theme, Renderer>) -> T,
    ) -> T {
        self.with_element_mut(|element| f(element))
    }

    fn with_element_mut<T>(
        &self,
        f: impl FnOnce(&mut Element<'_, Event, Theme, Renderer>) -> T,
    ) -> T {
        self.rebuild_element_if_necessary();
        self.state
            .borrow_mut()
            .as_mut()
            .unwrap()
            .with_element_mut(|element| f(element.as_mut().unwrap()))
    }
}

impl<Message, Theme, Renderer, Event, S> Widget<Message, Theme, Renderer>
    for Instance<'_, Message, Theme, Renderer, Event, S>
where
    S: 'static + Default,
    Renderer: core::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Tag<S>>()
    }

    fn state(&self) -> tree::State {
        let state = Rc::new(RefCell::new(Some(Tree {
            tag: tree::Tag::of::<Tag<S>>(),
            state: tree::State::new(S::default()),
            children: vec![Tree::empty()],
        })));

        *self.tree.borrow_mut() = state.clone();
        self.diff_self();

        tree::State::new(state)
    }

    fn children(&self) -> Vec<Tree> {
        vec![]
    }

    fn diff(&self, tree: &mut Tree) {
        let tree = tree.state.downcast_ref::<Rc<RefCell<Option<Tree>>>>();
        *self.tree.borrow_mut() = tree.clone();
        self.rebuild_element_if_necessary();
    }

    fn size(&self) -> Size<Length> {
        self.with_element(|element| element.as_widget().size())
    }

    fn size_hint(&self) -> Size<Length> {
        self.state
            .borrow()
            .as_ref()
            .expect("Borrow instance state")
            .borrow_component()
            .size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let t = tree.state.downcast_mut::<Rc<RefCell<Option<Tree>>>>();

        self.with_element_mut(|element| {
            element.as_widget_mut().layout(
                &mut t.borrow_mut().as_mut().unwrap().children[0],
                renderer,
                limits,
            )
        })
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &core::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let mut local_messages = Vec::new();
        let mut local_shell = Shell::new(&mut local_messages);

        let t = tree.state.downcast_mut::<Rc<RefCell<Option<Tree>>>>();
        self.with_element_mut(|element| {
            element.as_widget_mut().update(
                &mut t.borrow_mut().as_mut().unwrap().children[0],
                event,
                layout,
                cursor,
                renderer,
                clipboard,
                &mut local_shell,
                viewport,
            );
        });

        if local_shell.is_event_captured() {
            shell.capture_event();
        }

        local_shell.revalidate_layout(|| shell.invalidate_layout());
        shell.request_redraw_at(local_shell.redraw_request());
        shell.request_input_method(local_shell.input_method());

        if !local_messages.is_empty() {
            let mut heads = self.state.take().unwrap().into_heads();

            for message in local_messages.into_iter().filter_map(|message| {
                heads.component.update(
                    t.borrow_mut().as_mut().unwrap().state.downcast_mut(),
                    message,
                )
            }) {
                shell.publish(message);
            }

            self.state = RefCell::new(Some(
                StateBuilder {
                    component: heads.component,
                    message: PhantomData,
                    state: PhantomData,
                    element_builder: |_| None,
                }
                .build(),
            ));

            shell.invalidate_layout();
            shell.request_redraw();
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.rebuild_element_with_operation(layout, operation);

        let tree = tree.state.downcast_mut::<Rc<RefCell<Option<Tree>>>>();
        self.with_element_mut(|element| {
            element.as_widget_mut().operate(
                &mut tree.borrow_mut().as_mut().unwrap().children[0],
                layout,
                renderer,
                operation,
            );
        });
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let tree = tree.state.downcast_ref::<Rc<RefCell<Option<Tree>>>>();
        self.with_element(|element| {
            element.as_widget().draw(
                &tree.borrow().as_ref().unwrap().children[0],
                renderer,
                theme,
                style,
                layout,
                cursor,
                viewport,
            );
        });
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let tree = tree.state.downcast_ref::<Rc<RefCell<Option<Tree>>>>();
        self.with_element(|element| {
            element.as_widget().mouse_interaction(
                &tree.borrow().as_ref().unwrap().children[0],
                layout,
                cursor,
                viewport,
                renderer,
            )
        })
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.rebuild_element_if_necessary();

        let state = tree.state.downcast_mut::<Rc<RefCell<Option<Tree>>>>();
        let tree = state.borrow_mut().take().unwrap();

        let overlay = InnerBuilder {
            instance: self,
            tree,
            types: PhantomData,
            overlay_builder: |instance, tree| {
                instance.state.get_mut().as_mut().unwrap().with_element_mut(
                    move |element| {
                        element
                            .as_mut()
                            .unwrap()
                            .as_widget_mut()
                            .overlay(
                                &mut tree.children[0],
                                layout,
                                renderer,
                                viewport,
                                translation,
                            )
                            .map(|overlay| {
                                RefCell::new(overlay::Nested::new(overlay))
                            })
                    },
                )
            },
        }
        .build();

        #[allow(clippy::redundant_closure_for_method_calls)]
        if overlay.with_overlay(|overlay| overlay.is_some()) {
            Some(overlay::Element::new(Box::new(OverlayInstance {
                overlay: Some(Overlay(Some(overlay))), // Beautiful, I know（很漂亮，我知道）
            })))
        } else {
            let heads = overlay.into_heads();

            // - You may not like it, but this is what peak performance looks like
            // - TODO: Get rid of ouroboros, for good
            // - What?!
            // - 你也许不喜欢，但这就是巅峰性能的样子
            // - 待办：彻底摆脱 ouroboros
            // - 什么？！
            *state.borrow_mut() = Some(heads.tree);

            None
        }
    }
}

struct Overlay<'a, 'b, Message, Theme, Renderer, Event, S>(
    Option<Inner<'a, 'b, Message, Theme, Renderer, Event, S>>,
);

impl<Message, Theme, Renderer, Event, S> Drop
    for Overlay<'_, '_, Message, Theme, Renderer, Event, S>
{
    fn drop(&mut self) {
        if let Some(heads) = self.0.take().map(Inner::into_heads) {
            *heads.instance.tree.borrow_mut().borrow_mut() = Some(heads.tree);
        }
    }
}

#[self_referencing]
struct Inner<'a, 'b, Message, Theme, Renderer, Event, S> {
    instance: &'a mut Instance<'b, Message, Theme, Renderer, Event, S>,
    tree: Tree,
    types: PhantomData<(Message, Event, S)>,

    #[borrows(mut instance, mut tree)]
    #[not_covariant]
    overlay: Option<RefCell<overlay::Nested<'this, Event, Theme, Renderer>>>,
}

struct OverlayInstance<'a, 'b, Message, Theme, Renderer, Event, S> {
    overlay: Option<Overlay<'a, 'b, Message, Theme, Renderer, Event, S>>,
}

impl<Message, Theme, Renderer, Event, S>
    OverlayInstance<'_, '_, Message, Theme, Renderer, Event, S>
{
    fn with_overlay_maybe<T>(
        &self,
        f: impl FnOnce(&mut overlay::Nested<'_, Event, Theme, Renderer>) -> T,
    ) -> Option<T> {
        self.overlay
            .as_ref()
            .unwrap()
            .0
            .as_ref()
            .unwrap()
            .with_overlay(|overlay| {
                overlay.as_ref().map(|nested| (f)(&mut nested.borrow_mut()))
            })
    }

    fn with_overlay_mut_maybe<T>(
        &mut self,
        f: impl FnOnce(&mut overlay::Nested<'_, Event, Theme, Renderer>) -> T,
    ) -> Option<T> {
        self.overlay
            .as_mut()
            .unwrap()
            .0
            .as_mut()
            .unwrap()
            .with_overlay_mut(|overlay| {
                overlay.as_mut().map(|nested| (f)(nested.get_mut()))
            })
    }
}

impl<Message, Theme, Renderer, Event, S>
    overlay::Overlay<Message, Theme, Renderer>
    for OverlayInstance<'_, '_, Message, Theme, Renderer, Event, S>
where
    Renderer: core::Renderer,
    S: 'static + Default,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        self.with_overlay_maybe(|overlay| overlay.layout(renderer, bounds))
            .unwrap_or_default()
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let _ = self.with_overlay_maybe(|overlay| {
            overlay.draw(renderer, theme, style, layout, cursor);
        });
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.with_overlay_maybe(|overlay| {
            overlay.mouse_interaction(layout, cursor, renderer)
        })
        .unwrap_or_default()
    }

    fn update(
        &mut self,
        event: &core::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let mut local_messages = Vec::new();
        let mut local_shell = Shell::new(&mut local_messages);

        let _ = self.with_overlay_mut_maybe(|overlay| {
            overlay.update(
                event,
                layout,
                cursor,
                renderer,
                clipboard,
                &mut local_shell,
            );
        });

        if local_shell.is_event_captured() {
            shell.capture_event();
        }

        local_shell.revalidate_layout(|| shell.invalidate_layout());
        shell.request_redraw_at(local_shell.redraw_request());
        shell.request_input_method(local_shell.input_method());

        if !local_messages.is_empty() {
            let mut inner =
                self.overlay.take().unwrap().0.take().unwrap().into_heads();
            let mut heads = inner.instance.state.take().unwrap().into_heads();

            for message in local_messages.into_iter().filter_map(|message| {
                heads
                    .component
                    .update(inner.tree.state.downcast_mut(), message)
            }) {
                shell.publish(message);
            }

            *inner.instance.state.borrow_mut() = Some(
                StateBuilder {
                    component: heads.component,
                    message: PhantomData,
                    state: PhantomData,
                    element_builder: |_| None,
                }
                .build(),
            );

            self.overlay = Some(Overlay(Some(
                InnerBuilder {
                    instance: inner.instance,
                    tree: inner.tree,
                    types: PhantomData,
                    overlay_builder: |_, _| None,
                }
                .build(),
            )));

            shell.invalidate_layout();
        }
    }
}
