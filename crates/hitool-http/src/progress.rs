//! Stream progress callbacks aligned with Hutool `StreamProgress`.
//!
//! Java: `cn.hutool.core.io.StreamProgress` (used by HttpUtil download overloads).

/// Callback notified while downloading / copying a stream.
///
/// Java: `cn.hutool.core.io.StreamProgress`
pub trait StreamProgress: Send {
    /// Called once before transfer starts with the known total size (`-1` if unknown).
    ///
    /// Java: `StreamProgress.start()`
    fn start(&self) {}

    /// Called after each chunk with cumulative bytes transferred and total size.
    ///
    /// Java: `StreamProgress.progress(long total, long progressSize)`
    fn progress(&self, _total: i64, _progress_size: i64) {}

    /// Called when the transfer completes successfully.
    ///
    /// Java: `StreamProgress.finish()`
    fn finish(&self) {}
}

/// No-op progress implementation for callers that pass `null` in Hutool.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoopStreamProgress;

impl StreamProgress for NoopStreamProgress {}

/// Progress adapter over a mutable closure (tests / simple callbacks).
pub struct FnStreamProgress<F>
where
    F: Fn(i64, i64) + Send,
{
    on_progress: F,
}

impl<F> FnStreamProgress<F>
where
    F: Fn(i64, i64) + Send,
{
    /// Creates a progress callback that invokes `on_progress(total, progress_size)`.
    pub fn new(on_progress: F) -> Self {
        Self { on_progress }
    }
}

impl<F> StreamProgress for FnStreamProgress<F>
where
    F: Fn(i64, i64) + Send,
{
    fn progress(&self, total: i64, progress_size: i64) {
        (self.on_progress)(total, progress_size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicI64, Ordering};
    use std::sync::Arc;

    #[test]
    fn fn_stream_progress_reports_chunks() {
        let seen = Arc::new(AtomicI64::new(0));
        let seen2 = Arc::clone(&seen);
        let progress = FnStreamProgress::new(move |_total, size| {
            seen2.store(size, Ordering::SeqCst);
        });
        progress.start();
        progress.progress(10, 4);
        progress.progress(10, 10);
        progress.finish();
        assert_eq!(seen.load(Ordering::SeqCst), 10);
    }
}
