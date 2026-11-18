use crate::time::Instant;

/// 重绘窗口的请求。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A request to redraw a window.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RedrawRequest {
    /// 在下一帧重绘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Redraw the next frame.
    ///
    /// </details>
    NextFrame,

    /// 在给定的时间重绘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Redraw at the given time.
    ///
    /// </details>
    At(Instant),

    /// 不需要重绘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// No redraw is needed.
    ///
    /// </details>
    Wait,
}

impl From<Instant> for RedrawRequest {
    fn from(time: Instant) -> Self {
        Self::At(time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::Duration;

    #[test]
    fn ordering() {
        let now = Instant::now();
        let later = now + Duration::from_millis(10);

        assert_eq!(RedrawRequest::NextFrame, RedrawRequest::NextFrame);
        assert_eq!(RedrawRequest::At(now), RedrawRequest::At(now));

        assert!(RedrawRequest::NextFrame < RedrawRequest::At(now));
        assert!(RedrawRequest::At(now) > RedrawRequest::NextFrame);
        assert!(RedrawRequest::At(now) < RedrawRequest::At(later));
        assert!(RedrawRequest::At(later) > RedrawRequest::At(now));

        assert!(RedrawRequest::NextFrame <= RedrawRequest::NextFrame);
        assert!(RedrawRequest::NextFrame <= RedrawRequest::At(now));
        assert!(RedrawRequest::At(now) >= RedrawRequest::NextFrame);
        assert!(RedrawRequest::At(now) <= RedrawRequest::At(now));
        assert!(RedrawRequest::At(now) <= RedrawRequest::At(later));
        assert!(RedrawRequest::At(later) >= RedrawRequest::At(now));

        assert!(RedrawRequest::Wait > RedrawRequest::NextFrame);
        assert!(RedrawRequest::Wait > RedrawRequest::At(later));
    }
}
