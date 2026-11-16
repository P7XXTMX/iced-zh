use crate::cache::{self, Cached};
use crate::core::{Rectangle, Size};
use crate::geometry::{self, Frame};

pub use cache::Group;

/// 一个简单的缓存，存储已生成的几何图形以避免重复计算。
///
/// 除非所在图层的尺寸发生变化，或者被显式清空，
/// 否则 [`Cache`] 不会重绘它的几何图形。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A simple cache that stores generated geometry to avoid recomputation.
///
/// A [`Cache`] will not redraw its geometry unless the dimensions of its layer
/// change or it is explicitly cleared.
///
/// </details>
pub struct Cache<Renderer>
where
    Renderer: geometry::Renderer,
{
    raw: crate::Cache<Data<<Renderer::Geometry as Cached>::Cache>>,
}

#[derive(Debug, Clone)]
struct Data<T> {
    bounds: Rectangle,
    geometry: T,
}

impl<Renderer> Cache<Renderer>
where
    Renderer: geometry::Renderer,
{
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
            raw: cache::Cache::new(),
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
        Cache {
            raw: crate::Cache::with_group(group),
        }
    }

    /// 清空 [`Cache`]，强制在下次使用时重绘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Clears the [`Cache`], forcing a redraw the next time it is used.
    ///
    /// </details>
    pub fn clear(&self) {
        self.raw.clear();
    }

    /// 使用给定的闭包绘制几何图形，并把它存入
    /// [`Cache`]。
    ///
    /// 只有在以下情况下才会调用闭包：
    /// - 尺寸自上一次绘制调用以来发生了变化。
    /// - [`Cache`] 为空，或者已被显式清空。
    ///
    /// 否则，将返回先前存储的几何图形。
    /// 这种情况下 [`Cache`] 不会被清空。换句话说，
    /// 只要有需要，它就会一直返回已存储的几何图形。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws geometry using the provided closure and stores it in the
    /// [`Cache`].
    ///
    /// The closure will only be called when
    /// - the size has changed since the previous draw call.
    /// - the [`Cache`] is empty or has been explicitly cleared.
    ///
    /// Otherwise, the previously stored geometry will be returned. The
    /// [`Cache`] is not cleared in this case. In other words, it will keep
    /// returning the stored geometry if needed.
    ///
    /// </details>
    pub fn draw(
        &self,
        renderer: &Renderer,
        size: Size,
        draw_fn: impl FnOnce(&mut Frame<Renderer>),
    ) -> Renderer::Geometry {
        self.draw_with_bounds(renderer, Rectangle::with_size(size), draw_fn)
    }

    /// 使用给定的闭包绘制几何图形，并把它存入
    /// [`Cache`]。
    ///
    /// 与 [`draw`](Self::draw) 类似，但接受一个裁剪用的 [`Rectangle`]，
    /// 而不是 [`Size`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws geometry using the provided closure and stores it in the
    /// [`Cache`].
    ///
    /// Analogous to [`draw`](Self::draw), but takes a clipping [`Rectangle`] instead of
    /// a [`Size`].
    ///
    /// </details>
    pub fn draw_with_bounds(
        &self,
        renderer: &Renderer,
        bounds: Rectangle,
        draw_fn: impl FnOnce(&mut Frame<Renderer>),
    ) -> Renderer::Geometry {
        use std::ops::Deref;

        let state = self.raw.state();

        let previous = match state.borrow().deref() {
            cache::State::Empty { previous } => previous.as_ref().map(|data| data.geometry.clone()),
            cache::State::Filled { current } => {
                if current.bounds == bounds {
                    return Cached::load(&current.geometry);
                }

                Some(current.geometry.clone())
            }
        };

        let mut frame = Frame::with_bounds(renderer, bounds);
        draw_fn(&mut frame);

        let geometry = frame.into_geometry().cache(self.raw.group(), previous);
        let result = Cached::load(&geometry);

        *state.borrow_mut() = cache::State::Filled {
            current: Data { bounds, geometry },
        };

        result
    }
}

impl<Renderer> std::fmt::Debug for Cache<Renderer>
where
    Renderer: geometry::Renderer,
    <Renderer::Geometry as Cached>::Cache: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.raw)
    }
}

impl<Renderer> Default for Cache<Renderer>
where
    Renderer: geometry::Renderer,
{
    fn default() -> Self {
        Self::new()
    }
}
