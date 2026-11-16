//! 绘制三角形！
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Draw triangles!
//!
//! </details>
use crate::color;
use crate::core::{Rectangle, Transformation};
use crate::gradient;

use bytemuck::{Pod, Zeroable};

use std::sync::Arc;
use std::sync::atomic::{self, AtomicU64};

/// 用于渲染三角形网格的底层图元。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A low-level primitive to render a mesh of triangles.
///
/// </details>
#[derive(Debug, Clone, PartialEq)]
pub enum Mesh {
    /// 纯色网格。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A mesh with a solid color.
    ///
    /// </details>
    Solid {
        /// 网格的顶点和索引。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The vertices and indices of the mesh.
        ///
        /// </details>
        buffers: Indexed<SolidVertex2D>,

        /// 作用于 [`Mesh`] 顶点的 [`Transformation`]。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The [`Transformation`] for the vertices of the [`Mesh`].
        ///
        /// </details>
        transformation: Transformation,

        /// [`Mesh`] 的裁剪边界。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The clip bounds of the [`Mesh`].
        ///
        /// </details>
        clip_bounds: Rectangle,
    },
    /// 渐变网格。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A mesh with a gradient.
    ///
    /// </details>
    Gradient {
        /// 网格的顶点和索引。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The vertices and indices of the mesh.
        ///
        /// </details>
        buffers: Indexed<GradientVertex2D>,

        /// 作用于 [`Mesh`] 顶点的 [`Transformation`]。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The [`Transformation`] for the vertices of the [`Mesh`].
        ///
        /// </details>
        transformation: Transformation,

        /// [`Mesh`] 的裁剪边界。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The clip bounds of the [`Mesh`].
        ///
        /// </details>
        clip_bounds: Rectangle,
    },
}

impl Mesh {
    /// 返回 [`Mesh`] 的索引。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the indices of the [`Mesh`].
    ///
    /// </details>
    pub fn indices(&self) -> &[u32] {
        match self {
            Self::Solid { buffers, .. } => &buffers.indices,
            Self::Gradient { buffers, .. } => &buffers.indices,
        }
    }

    /// 返回 [`Mesh`] 的 [`Transformation`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Transformation`] of the [`Mesh`].
    ///
    /// </details>
    pub fn transformation(&self) -> Transformation {
        match self {
            Self::Solid { transformation, .. } | Self::Gradient { transformation, .. } => {
                *transformation
            }
        }
    }

    /// 返回 [`Mesh`] 的裁剪边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the clip bounds of the [`Mesh`].
    ///
    /// </details>
    pub fn clip_bounds(&self) -> Rectangle {
        match self {
            Self::Solid {
                clip_bounds,
                transformation,
                ..
            }
            | Self::Gradient {
                clip_bounds,
                transformation,
                ..
            } => *clip_bounds * *transformation,
        }
    }
}

/// 表示一组三角形的顶点和索引集合。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A set of vertices and indices representing a list of triangles.
///
/// </details>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Indexed<T> {
    /// 网格的顶点
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The vertices of the mesh
    ///
    /// </details>
    pub vertices: Vec<T>,

    /// 定义网格各个三角形的顶点索引列表。
    ///
    /// 因此，这个列表的长度应当始终是 3 的倍数。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The list of vertex indices that defines the triangles of the mesh.
    ///
    /// Therefore, this list should always have a length that is a multiple of 3.
    ///
    /// </details>
    pub indices: Vec<u32>,
}

/// 带颜色的二维顶点。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A two-dimensional vertex with a color.
///
/// </details>
#[derive(Copy, Clone, Debug, PartialEq, Zeroable, Pod)]
#[repr(C)]
pub struct SolidVertex2D {
    /// 顶点在二维空间中的位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The vertex position in 2D space.
    ///
    /// </details>
    pub position: [f32; 2],

    /// 顶点的颜色，采用 __线性__ RGBA。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The color of the vertex in __linear__ RGBA.
    ///
    /// </details>
    pub color: color::Packed,
}

/// 包含二维位置和打包后的渐变数据的顶点。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A vertex which contains 2D position & packed gradient data.
///
/// </details>
#[derive(Copy, Clone, Debug, PartialEq, Zeroable, Pod)]
#[repr(C)]
pub struct GradientVertex2D {
    /// 顶点在二维空间中的位置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The vertex position in 2D space.
    ///
    /// </details>
    pub position: [f32; 2],

    /// 渐变的打包顶点数据。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The packed vertex data of the gradient.
    ///
    /// </details>
    pub gradient: gradient::Packed,
}

/// 对一组网格的各项属性进行计数的结果。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The result of counting the attributes of a set of meshes.
///
/// </details>
#[derive(Debug, Clone, Copy, Default)]
pub struct AttributeCount {
    /// 纯色顶点的总数。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The total amount of solid vertices.
    ///
    /// </details>
    pub solid_vertices: usize,

    /// 纯色网格的总数。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The total amount of solid meshes.
    ///
    /// </details>
    pub solids: usize,

    /// 渐变顶点的总数。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The total amount of gradient vertices.
    ///
    /// </details>
    pub gradient_vertices: usize,

    /// 渐变网格的总数。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The total amount of gradient meshes.
    ///
    /// </details>
    pub gradients: usize,

    /// 索引的总数。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The total amount of indices.
    ///
    /// </details>
    pub indices: usize,
}

/// 返回所有 [`Mesh`] 的顶点总数和索引总数。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns the number of total vertices & total indices of all [`Mesh`]es.
///
/// </details>
pub fn attribute_count_of(meshes: &[Mesh]) -> AttributeCount {
    meshes
        .iter()
        .fold(AttributeCount::default(), |mut count, mesh| {
            match mesh {
                Mesh::Solid { buffers, .. } => {
                    count.solids += 1;
                    count.solid_vertices += buffers.vertices.len();
                    count.indices += buffers.indices.len();
                }
                Mesh::Gradient { buffers, .. } => {
                    count.gradients += 1;
                    count.gradient_vertices += buffers.vertices.len();
                    count.indices += buffers.indices.len();
                }
            }

            count
        })
}

/// 多个网格的缓存。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A cache of multiple meshes.
///
/// </details>
#[derive(Debug, Clone)]
pub struct Cache {
    id: Id,
    batch: Arc<[Mesh]>,
    version: usize,
}

/// [`Cache`] 的唯一 id。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The unique id of a [`Cache`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(u64);

impl Cache {
    /// 为给定的网格创建一个新的 [`Cache`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Cache`] for the given meshes.
    ///
    /// </details>
    pub fn new(meshes: Arc<[Mesh]>) -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);

        Self {
            id: Id(NEXT_ID.fetch_add(1, atomic::Ordering::Relaxed)),
            batch: meshes,
            version: 0,
        }
    }

    /// 返回 [`Cache`] 的 [`Id`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Id`] of the [`Cache`].
    ///
    /// </details>
    pub fn id(&self) -> Id {
        self.id
    }

    /// 返回 [`Cache`] 的当前版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current version of the [`Cache`].
    ///
    /// </details>
    pub fn version(&self) -> usize {
        self.version
    }

    /// 返回 [`Cache`] 中的那批网格。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the batch of meshes in the [`Cache`].
    ///
    /// </details>
    pub fn batch(&self) -> &Arc<[Mesh]> {
        &self.batch
    }

    /// 如果 [`Cache`] 为空，返回 true。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns true if the [`Cache`] is empty.
    ///
    /// </details>
    pub fn is_empty(&self) -> bool {
        self.batch.is_empty()
    }

    /// 使用给定的网格更新 [`Cache`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Updates the [`Cache`] with the given meshes.
    ///
    /// </details>
    pub fn update(&mut self, meshes: Arc<[Mesh]>) {
        self.batch = meshes;
        self.version += 1;
    }
}

/// 能够绘制 [`Mesh`] 的渲染器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A renderer capable of drawing a [`Mesh`].
///
/// </details>
pub trait Renderer {
    /// 绘制给定的 [`Mesh`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the given [`Mesh`].
    ///
    /// </details>
    fn draw_mesh(&mut self, mesh: Mesh);

    /// 绘制给定的 [`Cache`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the given [`Cache`].
    ///
    /// </details>
    fn draw_mesh_cache(&mut self, cache: Cache);
}
