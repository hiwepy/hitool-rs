//! Cron expression parsing and occurrence calculation.

#![forbid(unsafe_code)]

use chrono::{DateTime, TimeZone, Utc};
use cron::Schedule;
use std::str::FromStr;
use std::{fmt::Display, future::Future, sync::Arc, time::Duration};
use thiserror::Error;
use tokio::{sync::oneshot, task::JoinHandle, time};
use tracing::Instrument;

/// Errors returned by cron utilities.
#[derive(Debug, Error)]
pub enum CronError {
    /// The cron expression is invalid.
    #[error("invalid cron expression: {0}")]
    Parse(#[from] cron::error::Error),
    /// The timestamp cannot be represented by `chrono`.
    #[error("timestamp is outside the supported range")]
    InvalidTimestamp,
    /// A retry policy must contain at least one attempt and a valid delay range.
    #[error("retry policy must have at least one attempt and initial delay <= maximum delay")]
    InvalidRetryPolicy,
}

/// A parsed cron schedule that can be shared safely between threads.
#[derive(Debug, Clone)]
pub struct CronSchedule {
    expression: String,
    inner: Schedule,
}

impl CronSchedule {
    /// Parses a cron expression. The engine supports seconds as the first field.
    pub fn parse(expression: impl Into<String>) -> Result<Self, CronError> {
        let expression = expression.into();
        let inner = Schedule::from_str(&expression)?;
        Ok(Self { expression, inner })
    }

    /// Returns the original expression.
    #[must_use]
    pub fn expression(&self) -> &str {
        &self.expression
    }

    /// Returns the next occurrence after the supplied UTC time.
    #[must_use]
    pub fn next_after(&self, after: &DateTime<Utc>) -> Option<DateTime<Utc>> {
        self.inner.after(after).next()
    }

    /// Returns at most `limit` occurrences after the supplied UTC time.
    #[must_use]
    pub fn upcoming(&self, after: &DateTime<Utc>, limit: usize) -> Vec<DateTime<Utc>> {
        self.inner.after(after).take(limit).collect()
    }

    /// Resolves a millisecond timestamp and returns the next occurrence.
    pub fn next_after_millis(&self, timestamp: i64) -> Result<Option<DateTime<Utc>>, CronError> {
        let after = Utc
            .timestamp_millis_opt(timestamp)
            .single()
            .ok_or(CronError::InvalidTimestamp)?;
        Ok(self.next_after(&after))
    }
}

/// A running cron job. Dropping the handle aborts future executions.
#[derive(Debug)]
pub struct JobHandle {
    cancellation: Option<oneshot::Sender<()>>,
    task: JoinHandle<()>,
}

impl JobHandle {
    /// Requests cancellation and waits for the task to finish.
    pub async fn cancel(mut self) {
        if let Some(cancellation) = self.cancellation.take() {
            let _ = cancellation.send(());
        }
        let _ = (&mut self.task).await;
    }
}

impl Drop for JobHandle {
    fn drop(&mut self) {
        if let Some(cancellation) = self.cancellation.take() {
            let _ = cancellation.send(());
        }
        self.task.abort();
    }
}

/// Runtime execution policy for a cron job.
#[derive(Debug, Clone, Copy, Default)]
pub struct JobPolicy {
    /// Per-execution deadline. `None` allows the job to run until completion
    /// or cancellation.
    pub timeout: Option<Duration>,
}

/// Retry policy for fallible jobs, kept separate from the job implementation.
#[derive(Debug, Clone, Copy)]
pub struct RetryPolicy {
    max_attempts: u32,
    initial_delay: Duration,
    max_delay: Duration,
}

impl RetryPolicy {
    /// Creates a bounded exponential-backoff policy. `max_attempts` includes
    /// the first execution.
    pub fn new(
        max_attempts: u32,
        initial_delay: Duration,
        max_delay: Duration,
    ) -> Result<Self, CronError> {
        if max_attempts == 0 || initial_delay > max_delay {
            return Err(CronError::InvalidRetryPolicy);
        }
        Ok(Self {
            max_attempts,
            initial_delay,
            max_delay,
        })
    }

    /// Returns a policy that executes exactly once.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            max_attempts: 1,
            initial_delay: Duration::ZERO,
            max_delay: Duration::ZERO,
        }
    }

    fn delay_after(&self, failed_attempt: u32) -> Duration {
        let exponent = failed_attempt.saturating_sub(1).min(31);
        self.initial_delay
            .checked_mul(1_u32 << exponent)
            .unwrap_or(self.max_delay)
            .min(self.max_delay)
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self::none()
    }
}

/// Starts an asynchronous job for every future occurrence on the current
/// Tokio runtime.
///
/// A new run begins only after the previous run completes, preventing
/// accidental overlap. Callers can create independent handles for parallel
/// jobs when overlap is intentional.
#[must_use]
pub fn spawn<F, Fut>(schedule: CronSchedule, job: F) -> JobHandle
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    spawn_on(
        &tokio::runtime::Handle::current(),
        schedule,
        JobPolicy::default(),
        job,
    )
}

/// Starts a job on an explicitly supplied Tokio runtime handle.
#[must_use]
pub fn spawn_on<F, Fut>(
    runtime: &tokio::runtime::Handle,
    schedule: CronSchedule,
    policy: JobPolicy,
    job: F,
) -> JobHandle
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let job = Arc::new(job);
    let (cancellation, mut cancelled) = oneshot::channel();
    let task = runtime.spawn(async move {
        loop {
            let now = Utc::now();
            let Some(next) = schedule.next_after(&now) else {
                return;
            };
            let Ok(delay) = (next - now).to_std() else {
                continue;
            };
            tokio::select! {
                () = time::sleep(delay) => {},
                _ = &mut cancelled => return,
            }
            let span = tracing::info_span!(
                "hitool.cron.job",
                scheduled_at = %next,
                timed_out = tracing::field::Empty,
            );
            let run = (job)().instrument(span.clone());
            if let Some(timeout) = policy.timeout {
                tokio::select! {
                    result = time::timeout(timeout, run) => {
                        if result.is_err() {
                            span.record("timed_out", true);
                            tracing::warn!(parent: &span, "cron job exceeded its execution timeout");
                        }
                    },
                    _ = &mut cancelled => return,
                }
            } else {
                tokio::select! {
                    () = run => {},
                    _ = &mut cancelled => return,
                }
            }
        }
    });
    JobHandle {
        cancellation: Some(cancellation),
        task,
    }
}

/// Starts a fallible job on the current Tokio runtime with an independent
/// bounded retry policy.
///
/// Runs never overlap: a later scheduled occurrence is selected only after
/// the current occurrence, including retries, has completed.
#[must_use]
pub fn spawn_fallible<F, Fut, E>(
    schedule: CronSchedule,
    policy: JobPolicy,
    retry: RetryPolicy,
    job: F,
) -> JobHandle
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), E>> + Send + 'static,
    E: Display + Send + 'static,
{
    spawn_fallible_on(
        &tokio::runtime::Handle::current(),
        schedule,
        policy,
        retry,
        job,
    )
}

/// Starts a fallible job on an explicitly supplied Tokio runtime handle.
#[must_use]
pub fn spawn_fallible_on<F, Fut, E>(
    runtime: &tokio::runtime::Handle,
    schedule: CronSchedule,
    policy: JobPolicy,
    retry: RetryPolicy,
    job: F,
) -> JobHandle
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), E>> + Send + 'static,
    E: Display + Send + 'static,
{
    let job = Arc::new(job);
    let (cancellation, mut cancelled) = oneshot::channel();
    let task = runtime.spawn(async move {
        loop {
            let now = Utc::now();
            let Some(next) = schedule.next_after(&now) else {
                return;
            };
            let Ok(delay) = (next - now).to_std() else {
                continue;
            };
            tokio::select! {
                () = time::sleep(delay) => {},
                _ = &mut cancelled => return,
            }
            let span = tracing::info_span!(
                "hitool.cron.fallible_job",
                scheduled_at = %next,
                attempts = tracing::field::Empty,
                outcome = tracing::field::Empty,
            );
            tokio::select! {
                result = run_with_retry(&job, policy, retry, &span) => {
                    match result {
                        Ok(attempts) => {
                            span.record("attempts", attempts);
                            span.record("outcome", "success");
                        }
                        Err(failure) => {
                            span.record("attempts", failure.attempts);
                            span.record("outcome", failure.kind);
                            tracing::warn!(parent: &span, error = %failure.message, "cron job exhausted its retry policy");
                        }
                    }
                },
                _ = &mut cancelled => return,
            }
        }
    });
    JobHandle {
        cancellation: Some(cancellation),
        task,
    }
}

#[derive(Debug)]
struct RunFailure {
    attempts: u32,
    kind: &'static str,
    message: String,
}

async fn run_with_retry<F, Fut, E>(
    job: &Arc<F>,
    policy: JobPolicy,
    retry: RetryPolicy,
    span: &tracing::Span,
) -> Result<u32, RunFailure>
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), E>> + Send + 'static,
    E: Display + Send + 'static,
{
    for attempt in 1..=retry.max_attempts {
        let outcome = if let Some(timeout) = policy.timeout {
            match time::timeout(timeout, (job)().instrument(span.clone())).await {
                Ok(result) => result.map_err(|error| ("error", error.to_string())),
                Err(_) => Err(("timeout", format!("execution exceeded {timeout:?}"))),
            }
        } else {
            (job)()
                .instrument(span.clone())
                .await
                .map_err(|error| ("error", error.to_string()))
        };
        match outcome {
            Ok(()) => return Ok(attempt),
            Err((kind, message)) if attempt == retry.max_attempts => {
                return Err(RunFailure {
                    attempts: attempt,
                    kind,
                    message,
                });
            }
            Err(_) => time::sleep(retry.delay_after(attempt)).await,
        }
    }
    unreachable!("a validated retry policy always executes at least once")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn calculates_deterministic_occurrences() {
        let schedule = CronSchedule::parse("0 */5 * * * * *").unwrap();
        let start = Utc.with_ymd_and_hms(2026, 1, 1, 0, 1, 0).unwrap();
        let values = schedule.upcoming(&start, 2);
        assert_eq!(
            values[0],
            Utc.with_ymd_and_hms(2026, 1, 1, 0, 5, 0).unwrap()
        );
        assert_eq!(
            values[1],
            Utc.with_ymd_and_hms(2026, 1, 1, 0, 10, 0).unwrap()
        );
    }

    #[tokio::test]
    async fn spawned_jobs_can_be_cancelled() {
        let schedule = CronSchedule::parse("0/1 * * * * * *").unwrap();
        let handle = spawn_on(
            &tokio::runtime::Handle::current(),
            schedule,
            JobPolicy {
                timeout: Some(std::time::Duration::from_millis(10)),
            },
            || async {},
        );
        handle.cancel().await;
    }

    #[tokio::test]
    async fn fallible_jobs_retry_with_independent_policy() {
        let attempts = Arc::new(AtomicUsize::new(0));
        let job_attempts = Arc::clone(&attempts);
        let job = Arc::new(move || {
            let job_attempts = Arc::clone(&job_attempts);
            async move {
                if job_attempts.fetch_add(1, Ordering::SeqCst) < 2 {
                    Err("transient")
                } else {
                    Ok(())
                }
            }
        });
        let retry =
            RetryPolicy::new(3, Duration::from_millis(1), Duration::from_millis(2)).unwrap();
        let result = run_with_retry(
            &job,
            JobPolicy {
                timeout: Some(Duration::from_secs(1)),
            },
            retry,
            &tracing::Span::none(),
        )
        .await;
        assert_eq!(result.unwrap(), 3);
        assert_eq!(attempts.load(Ordering::SeqCst), 3);
    }
}
