//! 控制某些内容（例如图像）在一块空间内的适应方式。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Control the fit of some content (like an image) within a space.
//!
//! </details>
use crate::Size;

use std::fmt;

/// 让部件的内容适应其包围盒时所用的策略。
///
/// 这个枚举的每个变体都是一种策略，用来处理
/// 所显示的图像与显示它的空间之间
/// 在宽高比和尺寸上的差异。
///
/// 要以交互方式了解这些属性在 CSS 中的表现，
/// 请参阅 [Mozilla 的文档][1]，或运行 `tour` 示例
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The strategy used to fit the contents of a widget to its bounding box.
///
/// Each variant of this enum is a strategy that can be applied for resolving
/// differences in aspect ratio and size between the image being displayed and
/// the space its being displayed in.
///
/// For an interactive demonstration of these properties as they are implemented
/// in CSS, see [Mozilla's docs][1], or run the `tour` example
///
/// </details>
///
/// [1]: https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContentFit {
    /// 在不需要裁剪或隐藏任何部分的前提下尽可能放大。
    ///
    /// 图像会被缩放（保持宽高比），使它刚好能放进
    /// 窗口。这不会使图像变形，也不会裁剪或隐藏任何边缘，
    /// 但如果图像不能完全贴合，上下或左右
    /// 可能会留白。
    ///
    /// 当你需要完整显示一张图像、不丢失任何部分时，
    /// 尤其是当图像本身就是屏幕的焦点时，
    /// 这种方式非常合适。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Scale as big as it can be without needing to crop or hide parts.
    ///
    /// The image will be scaled (preserving aspect ratio) so that it just fits
    /// within the window.  This won't distort the image or crop/hide any edges,
    /// but if the image doesn't fit perfectly, there may be whitespace on the
    /// top/bottom or left/right.
    ///
    /// This is a great fit for when you need to display an image without losing
    /// any part of it, particularly when the image itself is the focus of the
    /// screen.
    ///
    /// </details>
    #[default]
    Contain,

    /// 缩放图像以覆盖整个包围盒，必要时进行裁剪。
    ///
    /// 这不会使图像变形，并且保证部件的区域被
    /// 完全覆盖，但可能会裁掉一些边缘，
    /// 尤其是当部件的宽高比与图像的宽高比
    /// 相差很大时。
    ///
    /// 最适合把图像用作背景或用来填充空间，
    /// 并且图像边缘的细节不太重要的
    /// 情况。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Scale the image to cover all of the bounding box, cropping if needed.
    ///
    /// This doesn't distort the image, and it ensures that the widget's area is
    /// completely covered, but it might crop off a bit of the edges of the
    /// widget, particularly when there is a big difference between the aspect
    /// ratio of the widget and the aspect ratio of the image.
    ///
    /// This is best for when you're using an image as a background, or to fill
    /// space, and any details of the image around the edge aren't too
    /// important.
    ///
    /// </details>
    Cover,

    /// 让图像变形，使部件在不裁剪的情况下被 100% 覆盖。
    ///
    /// 这会拉伸图像以适应部件，不留白也不裁剪。
    /// 不过由于拉伸，图像看起来可能会变形或被拉长，
    /// 尤其是在宽高比不匹配的时候。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Distort the image so the widget is 100% covered without cropping.
    ///
    /// This stretches the image to fit the widget, without any whitespace or
    /// cropping. However, because of the stretch, the image may look distorted
    /// or elongated, particularly when there's a mismatch of aspect ratios.
    ///
    /// </details>
    Fill,

    /// 完全不调整图像的大小或缩放。
    ///
    /// 这不会对给定的图像应用任何变换，但这也意味着，
    /// 除非你自己计算好，否则部件的区域不会被
    /// 完全覆盖，或者图像可能被裁剪。
    ///
    /// 最适合你已经自己调整好图像尺寸的情况。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Don't resize or scale the image at all.
    ///
    /// This will not apply any transformations to the provided image, but also
    /// means that unless you do the math yourself, the widget's area will not
    /// be completely covered, or the image might be cropped.
    ///
    /// This is best for when you've sized the image yourself.
    ///
    /// </details>
    None,

    /// 如果图像对于这块空间来说太大，就把它缩小，但绝不
    /// 放大。
    ///
    /// 它的行为与 [`Contain`](Self::Contain) 很相似，区别在于：如果图像
    /// 本来会被放大，它会保持原始分辨率，以避免
    /// 放大图像带来的模糊。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Scale the image down if it's too big for the space, but never scale it
    /// up.
    ///
    /// This works much like [`Contain`](Self::Contain), except that if the
    /// image would have been scaled up, it keeps its original resolution to
    /// avoid the bluring that accompanies upscaling images.
    ///
    /// </details>
    ScaleDown,
}

impl ContentFit {
    /// 尝试对某个边界内的内容尺寸应用给定的适应方式。
    ///
    /// 返回值是建议的内容缩放后尺寸。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Attempt to apply the given fit for a content size within some bounds.
    ///
    /// The returned value is the recommended scaled size of the content.
    ///
    /// </details>
    pub fn fit(&self, content: Size, bounds: Size) -> Size {
        let content_ar = content.width / content.height;
        let bounds_ar = bounds.width / bounds.height;

        match self {
            Self::Contain => {
                if bounds_ar > content_ar {
                    Size {
                        width: content.width * bounds.height / content.height,
                        ..bounds
                    }
                } else {
                    Size {
                        height: content.height * bounds.width / content.width,
                        ..bounds
                    }
                }
            }
            Self::Cover => {
                if bounds_ar < content_ar {
                    Size {
                        width: content.width * bounds.height / content.height,
                        ..bounds
                    }
                } else {
                    Size {
                        height: content.height * bounds.width / content.width,
                        ..bounds
                    }
                }
            }
            Self::Fill => bounds,
            Self::None => content,
            Self::ScaleDown => {
                if bounds_ar > content_ar && bounds.height < content.height {
                    Size {
                        width: content.width * bounds.height / content.height,
                        ..bounds
                    }
                } else if bounds.width < content.width {
                    Size {
                        height: content.height * bounds.width / content.width,
                        ..bounds
                    }
                } else {
                    content
                }
            }
        }
    }
}

impl fmt::Display for ContentFit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ContentFit::Contain => "Contain",
            ContentFit::Cover => "Cover",
            ContentFit::Fill => "Fill",
            ContentFit::None => "None",
            ContentFit::ScaleDown => "Scale Down",
        })
    }
}
