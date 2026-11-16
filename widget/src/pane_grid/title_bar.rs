use crate::container;
use crate::core::layout;
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::widget::{self, Tree};
use crate::core::{self, Event, Layout, Padding, Point, Rectangle, Shell, Size, Vector, Widget};
use crate::pane_grid::controls::Controls;

/// [`Pane`] 的标题栏。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The title bar of a [`Pane`].
///
/// </details>
///
/// [`Pane`]: super::Pane
pub struct TitleBar<
    'a,
    Message,
    C = crate::Element<'a, Message>,
    Theme = crate::Theme,
    Renderer = crate::Renderer,
> where
    Theme: container::Catalog,
{
    content: C,
    controls: Option<Controls<'a, Message, Theme, Renderer>>,
    padding: Padding,
    always_show_controls: bool,
    class: Theme::Class<'a>,
}

impl<'a, Message, C, Theme, Renderer> TitleBar<'a, Message, C, Theme, Renderer>
where
    Theme: container::Catalog,
{
    /// 使用给定的内容创建一个新的 [`TitleBar`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`TitleBar`] with the given content.
    ///
    /// </details>
    pub fn new(content: C) -> Self {
        Self {
            content,
            controls: None,
            padding: Padding::ZERO,
            always_show_controls: false,
            class: Theme::default(),
        }
    }

    /// 设置 [`TitleBar`] 的控件。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the controls of the [`TitleBar`].
    ///
    /// </details>
    pub fn controls(mut self, controls: impl Into<Controls<'a, Message, Theme, Renderer>>) -> Self {
        self.controls = Some(controls.into());
        self
    }

    /// 设置 [`TitleBar`] 的 [`Padding`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Padding`] of the [`TitleBar`].
    ///
    /// </details>
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// 设置附加在这个 [`TitleBar`] 上的 [`controls`] 是否
    /// 始终可见。
    ///
    /// 默认情况下，只有当这个 [`TitleBar`] 所属的 [`Pane`]
    /// 被悬停时，控件才可见。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets whether or not the [`controls`] attached to this [`TitleBar`] are
    /// always visible.
    ///
    /// By default, the controls are only visible when the [`Pane`] of this
    /// [`TitleBar`] is hovered.
    ///
    /// </details>
    ///
    /// [`controls`]: Self::controls
    /// [`Pane`]: super::Pane
    pub fn always_show_controls(mut self) -> Self {
        self.always_show_controls = true;
        self
    }

    /// 设置 [`TitleBar`] 的样式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style of the [`TitleBar`].
    ///
    /// </details>
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme) -> container::Style + 'a) -> Self
    where
        Theme::Class<'a>: From<container::StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as container::StyleFn<'a, Theme>).into();
        self
    }

    /// 设置 [`TitleBar`] 的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style class of the [`TitleBar`].
    ///
    /// </details>
    #[cfg(feature = "advanced")]
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<Message, C, Theme, Renderer> TitleBar<'_, Message, C, Theme, Renderer>
where
    C: Widget<Message, Theme, Renderer>,
    Theme: container::Catalog,
    Renderer: core::Renderer,
{
    pub(super) fn state(&self) -> Tree {
        let children = match self.controls.as_ref() {
            Some(controls) => match controls.compact.as_ref() {
                Some(compact) => vec![
                    Tree::new(&self.content),
                    Tree::new(&controls.full),
                    Tree::new(compact),
                ],
                None => vec![
                    Tree::new(&self.content),
                    Tree::new(&controls.full),
                    Tree::empty(),
                ],
            },
            None => {
                vec![Tree::new(&self.content), Tree::empty(), Tree::empty()]
            }
        };

        Tree {
            children,
            ..Tree::empty()
        }
    }

    pub(super) fn diff(&mut self, tree: &mut Tree) {
        if tree.children.len() != 3 {
            *tree = self.state();
        }

        if let Some(controls) = &mut self.controls {
            if let Some(compact) = &mut controls.compact {
                tree.children[2].diff(compact);
            }

            tree.children[1].diff(&mut controls.full);
        }

        tree.children[0].diff(&mut self.content);
    }

    /// 使用提供的 [`Renderer`] 和 [`Layout`] 绘制 [`TitleBar`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the [`TitleBar`] with the provided [`Renderer`] and [`Layout`].
    ///
    /// </details>
    ///
    /// [`Renderer`]: core::Renderer
    pub fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        inherited_style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        show_controls: bool,
    ) {
        let bounds = layout.bounds();
        let style = theme.style(&self.class);

        let inherited_style = renderer::Style {
            text_color: style.text_color.unwrap_or(inherited_style.text_color),
        };

        container::draw_background(renderer, &style, bounds);

        let mut children = layout.iter(&tree.children);
        let (title_layout, title_tree) = children.next().unwrap();
        let (controls_layout, controls_tree) = children.next().unwrap();
        let (compact_layout, compact_tree) = children.next().unwrap();
        let mut show_title = true;

        let padded_width = layout.bounds().width - self.padding.left + self.padding.right;

        if let Some(controls) = &self.controls
            && (show_controls || self.always_show_controls)
        {
            if title_layout.bounds().width + controls_layout.bounds().width > padded_width {
                if let Some(compact) = controls.compact.as_ref() {
                    compact.draw(
                        compact_tree,
                        renderer,
                        theme,
                        &inherited_style,
                        compact_layout,
                        cursor,
                        viewport,
                    );
                } else {
                    show_title = false;

                    controls.full.draw(
                        controls_tree,
                        renderer,
                        theme,
                        &inherited_style,
                        controls_layout,
                        cursor,
                        viewport,
                    );
                }
            } else {
                controls.full.draw(
                    controls_tree,
                    renderer,
                    theme,
                    &inherited_style,
                    controls_layout,
                    cursor,
                    viewport,
                );
            }
        }

        if show_title {
            self.content.draw(
                title_tree,
                renderer,
                theme,
                &inherited_style,
                title_layout,
                cursor,
                viewport,
            );
        }
    }

    /// 返回鼠标光标是否位于 [`TitleBar`] 的
    /// 拾取区域之上。
    ///
    /// 除了控件之外，整个 [`TitleBar`] 都是拾取区域。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns whether the mouse cursor is over the pick area of the
    /// [`TitleBar`] or not.
    ///
    /// The whole [`TitleBar`] is a pick area, except its controls.
    ///
    /// </details>
    pub fn is_over_pick_area(&self, tree: &Tree, layout: Layout, cursor_position: Point) -> bool {
        if layout.bounds().contains(cursor_position) {
            let mut children = layout.iter(&tree.children);
            let (title_layout, _) = children.next().unwrap();
            let (controls_layout, _) = children.next().unwrap();
            let (compact_layout, _) = children.next().unwrap();

            let padded_width = layout.bounds().width - self.padding.left + self.padding.right;

            if let Some(controls) = self.controls.as_ref() {
                if title_layout.bounds().width + controls_layout.bounds().width > padded_width {
                    if controls.compact.is_some() {
                        !compact_layout.bounds().contains(cursor_position)
                            && !title_layout.bounds().contains(cursor_position)
                    } else {
                        !controls_layout.bounds().contains(cursor_position)
                    }
                } else {
                    !controls_layout.bounds().contains(cursor_position)
                        && !title_layout.bounds().contains(cursor_position)
                }
            } else {
                !title_layout.bounds().contains(cursor_position)
            }
        } else {
            false
        }
    }

    pub(crate) fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let limits = limits.shrink(self.padding);
        let max_size = limits.max;

        self.content.layout(
            &mut tree.children[0],
            renderer,
            &layout::Limits::new(Size::ZERO, max_size),
        );

        let title_size = tree.children[0].size;

        let inner_size = if let Some(controls) = &mut self.controls {
            controls.full.layout(
                &mut tree.children[1],
                renderer,
                &layout::Limits::new(Size::ZERO, max_size),
            );

            let controls_size = tree.children[1].size;

            if title_size.width + controls_size.width > max_size.width {
                if let Some(compact) = controls.compact.as_mut() {
                    compact.layout(
                        &mut tree.children[2],
                        renderer,
                        &layout::Limits::new(Size::ZERO, max_size),
                    );

                    let compact_size = tree.children[2].size;
                    let space_before_controls = max_size.width - compact_size.width;

                    tree.children[1].translation = Vector::ZERO;
                    tree.children[2].translation = Vector::new(space_before_controls, 0.0);

                    Size::new(max_size.width, title_size.height.max(compact_size.height))
                } else {
                    let space_before_controls = max_size.width - controls_size.width;

                    tree.children[1].translation = Vector::new(space_before_controls, 0.0);

                    Size::new(max_size.width, title_size.height.max(controls_size.height))
                }
            } else {
                let space_before_controls = max_size.width - controls_size.width;

                tree.children[1].translation = Vector::new(space_before_controls, 0.0);

                Size::new(max_size.width, title_size.height.max(controls_size.height))
            }
        } else {
            Size::new(max_size.width, title_size.height)
        };

        tree.children[0].translation = Vector::ZERO;

        for child in &mut tree.children {
            child.translation += Vector::new(self.padding.left, self.padding.top);
        }

        tree.size = inner_size.expand(self.padding);
    }

    pub(crate) fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        let mut children = layout.iter_mut(&mut tree.children);
        let (title_layout, title_tree) = children.next().unwrap();
        let (controls_layout, controls_tree) = children.next().unwrap();
        let (compact_layout, compact_tree) = children.next().unwrap();
        let padded_width = layout.bounds().width - self.padding.left + self.padding.right;
        let mut show_title = true;

        if let Some(controls) = &mut self.controls {
            if title_layout.bounds().width + controls_layout.bounds().width > padded_width {
                if let Some(compact) = controls.compact.as_mut() {
                    compact.operate(compact_tree, compact_layout, viewport, renderer, operation);
                } else {
                    show_title = false;

                    controls.full.operate(
                        controls_tree,
                        controls_layout,
                        viewport,
                        renderer,
                        operation,
                    );
                }
            } else {
                controls.full.operate(
                    controls_tree,
                    controls_layout,
                    viewport,
                    renderer,
                    operation,
                );
            }
        };

        if show_title {
            self.content
                .operate(title_tree, title_layout, viewport, renderer, operation);
        }
    }

    pub(crate) fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let mut children = layout.iter_mut(&mut tree.children);
        let (title_layout, title_tree) = children.next().unwrap();
        let (controls_layout, controls_tree) = children.next().unwrap();
        let (compact_layout, compact_tree) = children.next().unwrap();
        let padded_width = layout.bounds().width - self.padding.left + self.padding.right;
        let mut show_title = true;

        if let Some(controls) = &mut self.controls {
            if title_layout.bounds().width + controls_layout.bounds().width > padded_width {
                if let Some(compact) = controls.compact.as_mut() {
                    compact.update(
                        compact_tree,
                        event,
                        compact_layout,
                        cursor,
                        renderer,
                        shell,
                        viewport,
                    );
                } else {
                    show_title = false;

                    controls.full.update(
                        controls_tree,
                        event,
                        controls_layout,
                        cursor,
                        renderer,
                        shell,
                        viewport,
                    );
                }
            } else {
                controls.full.update(
                    controls_tree,
                    event,
                    controls_layout,
                    cursor,
                    renderer,
                    shell,
                    viewport,
                );
            }
        }

        if show_title {
            self.content.update(
                title_tree,
                event,
                title_layout,
                cursor,
                renderer,
                shell,
                viewport,
            );
        }
    }

    pub(crate) fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let mut children = layout.iter(&tree.children);
        let (title_layout, title_tree) = children.next().unwrap();
        let (controls_layout, controls_tree) = children.next().unwrap();
        let (compact_layout, compact_tree) = children.next().unwrap();
        let padded_width = layout.bounds().width - self.padding.left + self.padding.right;

        let title_interaction =
            self.content
                .mouse_interaction(title_tree, title_layout, cursor, viewport, renderer);

        if let Some(controls) = &self.controls {
            let controls_interaction = controls.full.mouse_interaction(
                controls_tree,
                controls_layout,
                cursor,
                viewport,
                renderer,
            );

            if title_layout.bounds().width + controls_layout.bounds().width > padded_width {
                if let Some(compact) = controls.compact.as_ref() {
                    let compact_interaction = compact.mouse_interaction(
                        compact_tree,
                        compact_layout,
                        cursor,
                        viewport,
                        renderer,
                    );

                    compact_interaction.max(title_interaction)
                } else {
                    controls_interaction
                }
            } else {
                controls_interaction.max(title_interaction)
            }
        } else {
            title_interaction
        }
    }

    pub(crate) fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Message, Theme, Renderer>> {
        let mut children = layout.iter_mut(&mut tree.children);
        let (title_layout, title_tree) = children.next().unwrap();
        let (controls_layout, controls_tree) = children.next().unwrap();
        let (compact_layout, compact_tree) = children.next().unwrap();
        let padded_width = layout.bounds().width - self.padding.left + self.padding.right;

        let Self {
            content, controls, ..
        } = self;

        let mut overlays = content.overlay(
            title_tree,
            title_layout,
            renderer,
            viewport,
            translation,
            window,
        );

        if let Some(controls) = controls {
            if title_layout.bounds().width + controls_layout.bounds().width > padded_width {
                if let Some(compact) = &mut controls.compact {
                    overlays.extend(compact.overlay(
                        compact_tree,
                        compact_layout,
                        renderer,
                        viewport,
                        translation,
                        window,
                    ));
                } else {
                    overlays.extend(controls.full.overlay(
                        controls_tree,
                        controls_layout,
                        renderer,
                        viewport,
                        translation,
                        window,
                    ));
                }
            } else {
                overlays.extend(controls.full.overlay(
                    controls_tree,
                    controls_layout,
                    renderer,
                    viewport,
                    translation,
                    window,
                ));
            }
        }

        overlays
    }
}
