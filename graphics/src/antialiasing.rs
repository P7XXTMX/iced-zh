/// 抗锯齿策略。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An antialiasing strategy.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Antialiasing {
    /// 2 倍采样的多重采样抗锯齿
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Multisample AA with 2 samples
    ///
    /// </details>
    MSAAx2,
    /// 4 倍采样的多重采样抗锯齿
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Multisample AA with 4 samples
    ///
    /// </details>
    MSAAx4,
    /// 8 倍采样的多重采样抗锯齿
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Multisample AA with 8 samples
    ///
    /// </details>
    MSAAx8,
    /// 16 倍采样的多重采样抗锯齿
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Multisample AA with 16 samples
    ///
    /// </details>
    MSAAx16,
}

impl Antialiasing {
    /// 返回 [`Antialiasing`] 的采样数。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the amount of samples of the [`Antialiasing`].
    ///
    /// </details>
    pub fn sample_count(self) -> u32 {
        match self {
            Antialiasing::MSAAx2 => 2,
            Antialiasing::MSAAx4 => 4,
            Antialiasing::MSAAx8 => 8,
            Antialiasing::MSAAx16 => 16,
        }
    }
}
