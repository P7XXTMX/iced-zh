//! 构建并显示下拉菜单。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Build and show dropdown menus.
//!
//! </details>
use crate::core::alignment;
use crate::core::border::{self, Border};
use crate::core::layout;
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::text::{self, Text};
use crate::core::touch;
use crate::core::widget::Meta;
use crate::core::widget::tree::{self, Tree};
use crate::core::window;
use crate::core::{
    Background, Color, Event, Font, Layout, Length, Padding, Pixels, Point, Rectangle, Shadow,
    Size, Theme, Vector,
};
use crate::core::{Shell, Widget};
use crate::scrollable::{self, Scrollable};

/// 可选择选项的列表。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A list of selectable options.
///
/// </details>
pub struct Menu<'a, 'b, T, Message, Theme = crate::Theme>
where
    Theme: Catalog,
    'b: 'a,
{
    state: &'a mut State,
    options: &'a [T],
    hovered_option: &'a mut Option<usize>,
    to_string: &'a dyn Fn(&T) -> String,
    on_selected: Box<dyn FnMut(T) -> Message + 'a>,
    on_option_hovered: Option<&'a dyn Fn(T) -> Message>,
    width: f32,
    height: Length,
    padding: Padding,
    text_size: Option<Pixels>,
    line_height: Option<text::LineHeight>,
    shaping: text::Shaping,
    ellipsis: text::Ellipsis,
    font: Option<Font>,
    class: &'a <Theme as Catalog>::Class<'b>,
}

impl<'a, 'b, T, Message, Theme> Menu<'a, 'b, T, Message, Theme>
where
    T: Clone,
    Message: 'a,
    Theme: Catalog + 'a,
    'b: 'a,
{
    /// 使用给定的 [`State`]、选项列表、选项被选中时产生的消息
    /// 以及 [`Style`] 创建一个新的 [`Menu`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Menu`] with the given [`State`], a list of options,
    /// the message to produced when an option is selected, and its [`Style`].
    ///
    /// </details>
    pub fn new(
        state: &'a mut State,
        options: &'a [T],
        hovered_option: &'a mut Option<usize>,
        to_string: &'a dyn Fn(&T) -> String,
        on_selected: impl FnMut(T) -> Message + 'a,
        on_option_hovered: Option<&'a dyn Fn(T) -> Message>,
        class: &'a <Theme as Catalog>::Class<'b>,
    ) -> Self {
        Menu {
            state,
            options,
            hovered_option,
            to_string,
            on_selected: Box::new(on_selected),
            on_option_hovered,
            width: 0.0,
            height: Length::Shrink,
            padding: Padding::ZERO,
            text_size: None,
            line_height: None,
            shaping: text::Shaping::default(),
            ellipsis: text::Ellipsis::default(),
            font: None,
            class,
        }
    }

    /// 设置 [`Menu`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Menu`].
    ///
    /// </details>
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// 设置 [`Menu`] 的高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the height of the [`Menu`].
    ///
    /// </details>
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// 设置 [`Menu`] 的 [`Padding`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Padding`] of the [`Menu`].
    ///
    /// </details>
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// 设置 [`Menu`] 的文本大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the text size of the [`Menu`].
    ///
    /// </details>
    pub fn text_size(mut self, text_size: impl Into<Pixels>) -> Self {
        self.text_size = Some(text_size.into());
        self
    }

    /// 设置 [`Menu`] 的文本 [`text::LineHeight`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the text [`text::LineHeight`] of the [`Menu`].
    ///
    /// </details>
    pub fn line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    /// 设置 [`Menu`] 的 [`text::Shaping`] 策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`text::Shaping`] strategy of the [`Menu`].
    ///
    /// </details>
    pub fn shaping(mut self, shaping: text::Shaping) -> Self {
        self.shaping = shaping;
        self
    }

    /// 设置 [`Menu`] 的 [`text::Ellipsis`] 策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`text::Ellipsis`] strategy of the [`Menu`].
    ///
    /// </details>
    pub fn ellipsis(mut self, ellipsis: text::Ellipsis) -> Self {
        self.ellipsis = ellipsis;
        self
    }

    /// 设置 [`Menu`] 的字体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the font of the [`Menu`].
    ///
    /// </details>
    pub fn font(mut self, font: impl Into<Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// 把 [`Menu`] 转换为位于给定目标位置的
    /// [`overlay::Element`]。
    ///
    /// `position` 是目标在屏幕坐标系中的位置，
    /// `window` 是窗口的大小。
    ///
    /// `target_height` 用于决定把菜单显示在目标的上方
    /// 还是下方，这取决于屏幕位置和
    /// [`Menu`] 的尺寸。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Turns the [`Menu`] into an [`overlay::Element`] at the given target
    /// position.
    ///
    /// `position` is the target's position in screen coordinates, and
    /// `window` is the size of the window.
    ///
    /// The `target_height` will be used to display the menu either on top
    /// of the target or under it, depending on the screen position and the
    /// dimensions of the [`Menu`].
    ///
    /// </details>
    pub fn overlay<Renderer>(
        self,
        renderer: &Renderer,
        position: Point,
        window: Size,
        target_height: f32,
    ) -> overlay::Element<'a, Message, Theme, Renderer>
    where
        Renderer: text::Renderer + 'a,
    {
        overlay::Element::new(Box::new(Overlay::new(
            position,
            window,
            self,
            target_height,
            renderer,
        )))
    }
}

/// [`Menu`] 的局部状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The local state of a [`Menu`].
///
/// </details>
#[derive(Debug)]
pub struct State {
    tree: Tree,
}

impl State {
    /// 为 [`Menu`] 创建一个新的 [`State`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`State`] for a [`Menu`].
    ///
    /// </details>
    pub fn new() -> Self {
        Self {
            tree: Tree::empty(),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

struct Overlay<'a, 'b, T, Message, Theme>
where
    Theme: Catalog,
{
    window: Size,
    layout: Layout,
    tree: &'a mut Tree,
    list: Scrollable<'a, Message, List<'a, 'b, T, Message, Theme>, Theme>,
    class: &'a <Theme as Catalog>::Class<'b>,
}

impl<'a, 'b, T, Message, Theme> Overlay<'a, 'b, T, Message, Theme>
where
    T: Clone,
    Message: 'a,
    Theme: Catalog + scrollable::Catalog + 'a,
    'b: 'a,
{
    pub fn new<Renderer>(
        position: Point,
        window: Size,
        menu: Menu<'a, 'b, T, Message, Theme>,
        target_height: f32,
        renderer: &Renderer,
    ) -> Self
    where
        Renderer: text::Renderer + 'a,
    {
        let Menu {
            state,
            options,
            hovered_option,
            to_string,
            on_selected,
            on_option_hovered,
            padding,
            font,
            text_size,
            line_height,
            shaping,
            ellipsis,
            class,
            height,
            width,
        } = menu;

        let mut list = Scrollable::new(List {
            options,
            hovered_option,
            to_string,
            on_selected,
            on_option_hovered,
            font,
            text_size,
            line_height,
            shaping,
            ellipsis,
            padding,
            class,
        })
        .height(height);

        let space_below = window.height - (position.y + target_height);
        let space_above = position.y;

        let limits = layout::Limits::new(
            Size::ZERO,
            Size::new(
                window.width - position.x,
                if space_below > space_above {
                    space_below
                } else {
                    space_above
                },
            ),
        )
        .width(width);

        state.tree.diff::<_, _, Renderer>(&mut list);
        list.layout(&mut state.tree, renderer, &limits);

        let layout = Layout::new(state.tree.size).move_to(if space_below > space_above {
            position + Vector::new(0.0, target_height)
        } else {
            position - Vector::new(0.0, state.tree.size.height)
        });

        Self {
            layout,
            window,
            tree: &mut state.tree,
            list,
            class,
        }
    }
}

impl<T, Message, Theme, Renderer> crate::core::Overlay<Message, Theme, Renderer>
    for Overlay<'_, '_, T, Message, Theme>
where
    T: Clone,
    Theme: Catalog + scrollable::Catalog,
    Renderer: text::Renderer,
{
    fn update(
        &mut self,
        event: &Event,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
    ) {
        self.list.update(
            self.tree,
            event,
            self.layout,
            cursor,
            renderer,
            shell,
            &self.layout.bounds(),
        );
    }

    fn mouse_interaction(&self, cursor: mouse::Cursor, renderer: &Renderer) -> mouse::Interaction {
        let interaction = self.list.mouse_interaction(
            self.tree,
            self.layout,
            cursor,
            &Rectangle::with_size(self.window),
            renderer,
        );

        if interaction == mouse::Interaction::None && cursor.is_over(self.layout.bounds()) {
            mouse::Interaction::Idle
        } else {
            interaction
        }
    }

    fn operate(&mut self, renderer: &Renderer, operation: &mut dyn crate::core::widget::Operation) {
        self.list.operate(
            self.tree,
            self.layout,
            &self.layout.bounds(),
            renderer,
            operation,
        );
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        defaults: &renderer::Style,
        cursor: mouse::Cursor,
    ) {
        let bounds = self.layout.bounds();

        let style = Catalog::style(theme, self.class);

        renderer.with_layer(bounds, |renderer| {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: style.border,
                    shadow: style.shadow,
                    ..renderer::Quad::default()
                },
                style.background,
            );

            self.list.draw(
                self.tree,
                renderer,
                theme,
                defaults,
                self.layout,
                cursor,
                &bounds,
            );
        });
    }
}

struct List<'a, 'b, T, Message, Theme>
where
    Theme: Catalog,
{
    options: &'a [T],
    hovered_option: &'a mut Option<usize>,
    to_string: &'a dyn Fn(&T) -> String,
    on_selected: Box<dyn FnMut(T) -> Message + 'a>,
    on_option_hovered: Option<&'a dyn Fn(T) -> Message>,
    padding: Padding,
    text_size: Option<Pixels>,
    line_height: Option<text::LineHeight>,
    shaping: text::Shaping,
    ellipsis: text::Ellipsis,
    font: Option<Font>,
    class: &'a <Theme as Catalog>::Class<'b>,
}

struct ListState {
    is_hovered: Option<bool>,
}

impl<T, Message, Theme> Meta for List<'_, '_, T, Message, Theme> where Theme: Catalog {}

impl<T, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for List<'_, '_, T, Message, Theme>
where
    T: Clone,
    Theme: Catalog,
    Renderer: text::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Option<bool>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(ListState { is_hovered: None })
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fit,
        }
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        use std::f32;

        let text_size = self.text_size.unwrap_or_else(|| renderer.text_size());
        let line_height = self.line_height.unwrap_or_else(|| renderer.line_height());

        let text_line_height = line_height.to_absolute(text_size);

        let size = {
            let intrinsic = Size::new(
                0.0,
                (f32::from(text_line_height) + self.padding.y()) * self.options.len() as f32,
            );

            limits.resolve(Length::Fill, Length::Fit, intrinsic)
        };

        tree.size = size;
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let hovered_option = self
            .hovered_option
            .unwrap_or_default()
            .min(self.options.len().saturating_sub(1));

        let text_size = self.text_size.unwrap_or_else(|| renderer.text_size());
        let line_height = self.line_height.unwrap_or_else(|| renderer.line_height());
        let option_height = f32::from(line_height.to_absolute(text_size)) + self.padding.y();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(cursor_position) = cursor.position_in(layout.bounds()) {
                    let option_index = (cursor_position.y / option_height) as usize;

                    *self.hovered_option = Some(option_index);

                    if let Some(option) = self.options.get(option_index) {
                        shell.publish((self.on_selected)(option.clone()));
                        shell.capture_event();
                    }
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if let Some(cursor_position) = cursor.position_in(layout.bounds()) {
                    let new_hovered_option = (cursor_position.y / option_height) as usize;

                    if hovered_option != new_hovered_option
                        && let Some(option) = self.options.get(new_hovered_option)
                    {
                        if let Some(on_option_hovered) = self.on_option_hovered {
                            shell.publish(on_option_hovered(option.clone()));
                        }

                        shell.request_redraw();
                    }

                    *self.hovered_option = Some(new_hovered_option);
                }
            }
            Event::Touch(touch::Event::FingerPressed { .. }) => {
                if let Some(cursor_position) = cursor.position_in(layout.bounds()) {
                    let option_index = (cursor_position.y / option_height) as usize;

                    *self.hovered_option = Some(option_index);

                    if let Some(option) = self.options.get(option_index) {
                        shell.publish((self.on_selected)(option.clone()));
                        shell.capture_event();
                    }
                }
            }
            _ => {}
        }

        let state = tree.state.downcast_mut::<ListState>();

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            state.is_hovered = Some(cursor.is_over(layout.bounds()));
        } else if state
            .is_hovered
            .is_some_and(|is_hovered| is_hovered != cursor.is_over(layout.bounds()))
        {
            shell.request_redraw();
        }
    }

    fn operate(
        &mut self,
        _tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn crate::core::widget::Operation,
    ) {
        let bounds = layout.bounds();

        let text_size = self.text_size.unwrap_or_else(|| renderer.text_size());
        let line_height = self.line_height.unwrap_or_else(|| renderer.line_height());
        let option_height = f32::from(line_height.to_absolute(text_size)) + self.padding.y();

        let len = self.options.len();

        let start = ((viewport.y - bounds.y) / option_height)
            .floor()
            .clamp(0.0, len as f32) as usize;
        let end = ((viewport.y + viewport.height - bounds.y) / option_height)
            .ceil()
            .clamp(0.0, len as f32) as usize;

        for (i, option) in self
            .options
            .iter()
            .enumerate()
            .skip(start)
            .take(end - start)
        {
            let mut text = (self.to_string)(option);

            operation.text(
                None,
                Rectangle {
                    x: bounds.x,
                    y: bounds.y + option_height * i as f32,
                    width: bounds.width,
                    height: option_height,
                },
                &mut text,
            );
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let is_mouse_over = cursor.is_over(layout.bounds());

        if is_mouse_over {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let style = Catalog::style(theme, self.class);
        let bounds = layout.bounds();

        let text_size = self.text_size.unwrap_or_else(|| renderer.text_size());
        let line_height = self.line_height.unwrap_or_else(|| renderer.line_height());
        let option_height = f32::from(line_height.to_absolute(text_size)) + self.padding.y();

        let offset = viewport.y - bounds.y;
        let start = (offset / option_height) as usize;
        let end = ((offset + viewport.height) / option_height).ceil() as usize;

        let visible_options = &self.options[start..end.min(self.options.len())];
        let hovered_option = self
            .hovered_option
            .map(|index| index.min(self.options.len().saturating_sub(1)));

        for (i, option) in visible_options.iter().enumerate() {
            let i = start + i;
            let is_selected = hovered_option == Some(i);

            let bounds = Rectangle {
                x: bounds.x,
                y: bounds.y + (option_height * i as f32),
                width: bounds.width,
                height: option_height,
            };

            if is_selected {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x: bounds.x + style.border.width,
                            width: bounds.width - style.border.width * 2.0,
                            ..bounds
                        },
                        border: border::rounded(style.border.radius),
                        ..renderer::Quad::default()
                    },
                    style.selected_background,
                );
            }

            renderer.fill_text(
                Text {
                    content: (self.to_string)(option),
                    bounds: Size::new(bounds.width - self.padding.x(), bounds.height),
                    size: text_size,
                    line_height,
                    font: self.font.unwrap_or_else(|| renderer.font()),
                    align_x: text::Alignment::Default,
                    align_y: alignment::Vertical::Center,
                    shaping: self.shaping,
                    wrapping: text::Wrapping::None,
                    ellipsis: self.ellipsis,
                    hint_factor: renderer.hint_factor(),
                },
                Point::new(bounds.x + self.padding.left, bounds.center_y()),
                if is_selected {
                    style.selected_text_color
                } else {
                    style.text_color
                },
                *viewport,
            );
        }
    }
}

/// [`Menu`] 的外观。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The appearance of a [`Menu`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// 菜单的 [`Background`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Background`] of the menu.
    ///
    /// </details>
    pub background: Background,
    /// 菜单的 [`Border`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Border`] of the menu.
    ///
    /// </details>
    pub border: Border,
    /// 菜单的文本 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text [`Color`] of the menu.
    ///
    /// </details>
    pub text_color: Color,
    /// 菜单中被选中选项的文本 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text [`Color`] of a selected option in the menu.
    ///
    /// </details>
    pub selected_text_color: Color,
    /// 菜单中被选中选项的背景 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The background [`Color`] of a selected option in the menu.
    ///
    /// </details>
    pub selected_background: Background,
    /// 菜单的 [`Shadow`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Shadow`] of the menu.
    ///
    /// </details>
    pub shadow: Shadow,
}

/// [`Menu`] 的主题样式目录。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The theme catalog of a [`Menu`].
///
/// </details>
pub trait Catalog: scrollable::Catalog {
    /// [`Catalog`] 的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The item class of the [`Catalog`].
    ///
    /// </details>
    type Class<'a>;

    /// [`Catalog`] 产生的默认样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default class produced by the [`Catalog`].
    ///
    /// </details>
    fn default<'a>() -> <Self as Catalog>::Class<'a>;

    /// [`Menu`] 的可滚动区域的默认样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default class for the scrollable of the [`Menu`].
    ///
    /// </details>
    fn default_scrollable<'a>() -> <Self as scrollable::Catalog>::Class<'a> {
        <Self as scrollable::Catalog>::default()
    }

    /// 某个样式类在给定交互状态下的 [`Style`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Style`] of a class with the given status.
    ///
    /// </details>
    fn style(&self, class: &<Self as Catalog>::Class<'_>) -> Style;
}

/// [`Menu`] 的样式函数。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A styling function for a [`Menu`].
///
/// </details>
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> StyleFn<'a, Self> {
        Box::new(default)
    }

    fn style(&self, class: &StyleFn<'_, Self>) -> Style {
        class(self)
    }
}

/// [`Menu`] 列表的默认样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The default style of the list of a [`Menu`].
///
/// </details>
pub fn default(theme: &Theme) -> Style {
    let palette = theme.palette();

    Style {
        background: palette.background.weak.color.into(),
        border: Border {
            width: 1.0,
            radius: 0.0.into(),
            color: palette.background.strong.color,
        },
        text_color: palette.background.weak.text,
        selected_text_color: palette.primary.strong.text,
        selected_background: palette.primary.strong.color.into(),
        shadow: Shadow::default(),
    }
}
