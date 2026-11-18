//! 绘制自定义图元。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Draw custom primitives.
//!
//! </details>
use crate::core::{self, Rectangle};
use crate::graphics::Viewport;
use crate::graphics::futures::{MaybeSend, MaybeSync};

use rustc_hash::FxHashMap;
use std::any::{Any, TypeId};
use std::fmt::Debug;

/// 一批图元。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A batch of primitives.
///
/// </details>
pub type Batch = Vec<Instance>;

/// 让 [`Primitive`] 能够被渲染的一组方法。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A set of methods which allows a [`Primitive`] to be rendered.
///
/// </details>
pub trait Primitive: Debug + MaybeSend + MaybeSync + 'static {
    /// 这个 [`Primitive`] 的共享渲染器。
    ///
    /// 通常，它会包含一些 [`wgpu`] 状态，例如
    /// 渲染管线、缓冲区和纹理。
    ///
    /// 这个 [`Primitive`] 类型的所有实例都共享同一个
    /// [`Renderer`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The shared renderer of this [`Primitive`].
    ///
    /// Normally, this will contain a bunch of [`wgpu`] state; like
    /// a rendering pipeline, buffers, and textures.
    ///
    /// All instances of this [`Primitive`] type will share the same
    /// [`Renderer`].
    ///
    /// </details>
    type Pipeline: Pipeline + MaybeSend + MaybeSync;

    /// 处理 [`Primitive`]，可以在这里分配 GPU 缓冲区。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Processes the [`Primitive`], allowing for GPU buffer allocation.
    ///
    /// </details>
    fn prepare(
        &self,
        pipeline: &mut Self::Pipeline,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bounds: &Rectangle,
        viewport: &Viewport,
    );

    /// 在给定的 [`wgpu::RenderPass`] 中绘制 [`Primitive`]。
    ///
    /// 只要可能，就应该优先实现它而不是 [`render`](Self::render)，
    /// 因为复用已有的渲染通道应该比新开一个
    /// 高效得多。
    ///
    /// 所提供的渲染通道的视口和裁剪矩形分别被设置为
    /// [`Primitive`] 的边界和裁剪边界。
    ///
    /// 如果你有复杂的合成需求，可以在这里返回 `false`，
    /// 从而改用 [`render`](Self::render)。
    ///
    /// 默认情况下，它什么也不做并返回 `false`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the [`Primitive`] in the given [`wgpu::RenderPass`].
    ///
    /// When possible, this should be implemented over [`render`](Self::render)
    /// since reusing the existing render pass should be considerably more
    /// efficient than issuing a new one.
    ///
    /// The viewport and scissor rect of the render pass provided is set
    /// to the bounds and clip bounds of the [`Primitive`], respectively.
    ///
    /// If you have complex composition needs, then you can leverage
    /// [`render`](Self::render) by returning `false` here.
    ///
    /// By default, it does nothing and returns `false`.
    ///
    /// </details>
    fn draw(
        &self,
        _pipeline: &Self::Pipeline,
        _render_pass: &mut wgpu::RenderPass<'_>,
    ) -> bool {
        false
    }

    /// 使用给定的 [`wgpu::CommandEncoder`] 渲染 [`Primitive`]。
    ///
    /// 只有在 [`draw`](Self::draw) 返回 `false` 时才会调用它。
    ///
    /// 默认情况下，它什么也不做。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Renders the [`Primitive`], using the given [`wgpu::CommandEncoder`].
    ///
    /// This will only be called if [`draw`](Self::draw) returns `false`.
    ///
    /// By default, it does nothing.
    ///
    /// </details>
    fn render(
        &self,
        _pipeline: &Self::Pipeline,
        _encoder: &mut wgpu::CommandEncoder,
        _target: &wgpu::TextureView,
        _clip_bounds: &Rectangle<u32>,
    ) {
    }
}

/// 图形 [`Primitive`] 的管线。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The pipeline of a graphics [`Primitive`].
///
/// </details>
pub trait Pipeline: Any + MaybeSend + MaybeSync {
    /// 创建 [`Primitive`] 的 [`Pipeline`]。
    ///
    /// 它只会被调用一次，即第一次遇到使用这种
    /// [`Pipeline`] 的 [`Primitive`] 时。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates the [`Pipeline`] of a [`Primitive`].
    ///
    /// This will only be called once, when the first [`Primitive`] with this kind
    /// of [`Pipeline`] is encountered.
    ///
    /// </details>
    fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
    ) -> Self
    where
        Self: Sized;

    /// 清理 [`Pipeline`] 中缓存的数据。
    ///
    /// 通常会在一帧结束时调用它。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Trims any cached data in the [`Pipeline`].
    ///
    /// This will normally be called at the end of a frame.
    ///
    /// </details>
    fn trim(&mut self) {}
}

pub(crate) trait Stored:
    Debug + MaybeSend + MaybeSync + 'static
{
    fn prepare(
        &self,
        storage: &mut Storage,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        bounds: &Rectangle,
        viewport: &Viewport,
    );

    fn draw(
        &self,
        storage: &Storage,
        render_pass: &mut wgpu::RenderPass<'_>,
    ) -> bool;

    fn render(
        &self,
        storage: &Storage,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        clip_bounds: &Rectangle<u32>,
    );
}

#[derive(Debug)]
struct BlackBox<P: Primitive> {
    primitive: P,
}

impl<P: Primitive> Stored for BlackBox<P> {
    fn prepare(
        &self,
        storage: &mut Storage,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        bounds: &Rectangle,
        viewport: &Viewport,
    ) {
        if !storage.has::<P>() {
            storage.store::<P, _>(P::Pipeline::new(device, queue, format));
        }

        let renderer = storage
            .get_mut::<P>()
            .expect("renderer should be initialized")
            .downcast_mut::<P::Pipeline>()
            .expect("renderer should have the proper type");

        self.primitive
            .prepare(renderer, device, queue, bounds, viewport);
    }

    fn draw(
        &self,
        storage: &Storage,
        render_pass: &mut wgpu::RenderPass<'_>,
    ) -> bool {
        let renderer = storage
            .get::<P>()
            .expect("renderer should be initialized")
            .downcast_ref::<P::Pipeline>()
            .expect("renderer should have the proper type");

        self.primitive.draw(renderer, render_pass)
    }

    fn render(
        &self,
        storage: &Storage,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        clip_bounds: &Rectangle<u32>,
    ) {
        let renderer = storage
            .get::<P>()
            .expect("renderer should be initialized")
            .downcast_ref::<P::Pipeline>()
            .expect("renderer should have the proper type");

        self.primitive
            .render(renderer, encoder, target, clip_bounds);
    }
}

#[derive(Debug)]
/// 某个具体 [`Primitive`] 的实例。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An instance of a specific [`Primitive`].
///
/// </details>
pub struct Instance {
    /// [`Instance`] 的边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The bounds of the [`Instance`].
    ///
    /// </details>
    pub(crate) bounds: Rectangle,

    /// 要渲染的 [`Primitive`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Primitive`] to render.
    ///
    /// </details>
    pub(crate) primitive: Box<dyn Stored>,
}

impl Instance {
    /// 使用给定的 [`Primitive`] 创建一个新的 [`Instance`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Instance`] with the given [`Primitive`].
    ///
    /// </details>
    pub fn new(bounds: Rectangle, primitive: impl Primitive) -> Self {
        Instance {
            bounds,
            primitive: Box::new(BlackBox { primitive }),
        }
    }
}

/// 能够绘制自定义图元的渲染器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A renderer than can draw custom primitives.
///
/// </details>
pub trait Renderer: core::Renderer {
    /// 绘制一个自定义图元。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws a custom primitive.
    ///
    /// </details>
    fn draw_primitive(&mut self, bounds: Rectangle, primitive: impl Primitive);
}

/// 存储用户提供的自定义类型。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Stores custom, user-provided types.
///
/// </details>
#[derive(Default)]
pub struct Storage {
    pipelines: FxHashMap<TypeId, Box<dyn Pipeline>>,
}

impl Storage {
    /// 如果 `Storage` 包含类型 `T`，返回 `true`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns `true` if `Storage` contains a type `T`.
    ///
    /// </details>
    pub fn has<T: 'static>(&self) -> bool {
        self.pipelines.contains_key(&TypeId::of::<T>())
    }

    /// 把数据 `T` 插入 [`Storage`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Inserts the data `T` in to [`Storage`].
    ///
    /// </details>
    pub fn store<T: 'static, P: Pipeline>(&mut self, pipeline: P) {
        let _ = self.pipelines.insert(TypeId::of::<T>(), Box::new(pipeline));
    }

    /// 如果 [`Storage`] 中存在类型为 `T` 的数据，返回它的引用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a reference to the data with type `T` if it exists in [`Storage`].
    ///
    /// </details>
    pub fn get<T: 'static>(&self) -> Option<&dyn Any> {
        self.pipelines
            .get(&TypeId::of::<T>())
            .map(|pipeline| pipeline.as_ref() as &dyn Any)
    }

    /// 如果 [`Storage`] 中存在类型为 `T` 的数据，返回它的可变引用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a mutable reference to the data with type `T` if it exists in [`Storage`].
    ///
    /// </details>
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut dyn Any> {
        self.pipelines
            .get_mut(&TypeId::of::<T>())
            .map(|pipeline| pipeline.as_mut() as &mut dyn Any)
    }

    /// 清理 [`Storage`] 中所有管线的缓存。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Trims the cache of all the pipelines in the [`Storage`].
    ///
    /// </details>
    pub fn trim(&mut self) {
        for pipeline in self.pipelines.values_mut() {
            pipeline.trim();
        }
    }
}
