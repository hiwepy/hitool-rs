//! Explicitly owned timer and timing-wheel primitives.

#![allow(clippy::missing_panics_doc)]

use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    fmt,
    sync::{Arc, Mutex, mpsc},
    thread::{self, JoinHandle},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::CronError;

type TaskFn = Box<dyn FnOnce() + Send + 'static>;

fn now_millis() -> i64 {
    i64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_millis(),
    )
    .unwrap_or(i64::MAX)
}

struct TimerTaskInner {
    delay_ms: u64,
    deadline_ms: i64,
    task: Mutex<Option<TaskFn>>,
}

/// A one-shot task with a relative delay.
#[derive(Clone)]
pub struct TimerTask(Arc<TimerTaskInner>);

impl TimerTask {
    /// Creates a delayed one-shot task.
    pub fn new<F>(task: F, delay: Duration) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        let delay_ms = u64::try_from(delay.as_millis()).unwrap_or(u64::MAX);
        let deadline_delay = i64::try_from(delay_ms).unwrap_or(i64::MAX);
        Self(Arc::new(TimerTaskInner {
            delay_ms,
            deadline_ms: now_millis().saturating_add(deadline_delay),
            task: Mutex::new(Some(Box::new(task))),
        }))
    }

    /// Returns the configured relative delay.
    #[must_use]
    pub fn delay(&self) -> Duration {
        Duration::from_millis(self.0.delay_ms)
    }

    /// Returns the configured delay in milliseconds.
    #[must_use]
    pub fn delay_ms(&self) -> u64 {
        self.0.delay_ms
    }

    /// Returns the absolute deadline used by timing wheels.
    #[must_use]
    pub(crate) fn deadline_ms(&self) -> i64 {
        self.0.deadline_ms
    }

    /// Executes the task at most once and reports whether it ran.
    pub fn execute(&self) -> bool {
        let task = self
            .0
            .task
            .lock()
            .expect("timer task mutex poisoned")
            .take();
        if let Some(task) = task {
            task();
            true
        } else {
            false
        }
    }
}

impl fmt::Debug for TimerTask {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TimerTask")
            .field("delay_ms", &self.0.delay_ms)
            .field("deadline_ms", &self.0.deadline_ms)
            .field(
                "pending",
                &self
                    .0
                    .task
                    .lock()
                    .expect("timer task mutex poisoned")
                    .is_some(),
            )
            .finish()
    }
}

/// A timing-wheel bucket.
#[derive(Debug, Default)]
pub struct TimerTaskList {
    expiration_ms: i64,
    tasks: Vec<TimerTask>,
}

impl TimerTaskList {
    /// Creates an empty, unscheduled bucket.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            expiration_ms: -1,
            tasks: Vec::new(),
        }
    }

    /// Changes the expiration and reports whether it changed.
    pub fn set_expiration(&mut self, expiration_ms: i64) -> bool {
        if self.expiration_ms == expiration_ms {
            false
        } else {
            self.expiration_ms = expiration_ms;
            true
        }
    }

    /// Returns the expiration in Unix milliseconds.
    #[must_use]
    pub const fn expiration(&self) -> i64 {
        self.expiration_ms
    }

    /// Adds a task to this bucket.
    pub fn add_task(&mut self, task: TimerTask) {
        self.tasks.push(task);
    }

    /// Removes one task by shared identity.
    pub fn remove_task(&mut self, task: &TimerTask) -> bool {
        if let Some(index) = self
            .tasks
            .iter()
            .position(|candidate| Arc::ptr_eq(&candidate.0, &task.0))
        {
            self.tasks.remove(index);
            true
        } else {
            false
        }
    }

    /// Drains the bucket through a caller-supplied consumer.
    pub fn flush<F>(&mut self, mut consumer: F)
    where
        F: FnMut(TimerTask),
    {
        for task in self.tasks.drain(..) {
            consumer(task);
        }
        self.expiration_ms = -1;
    }

    /// Returns the non-negative remaining delay.
    #[must_use]
    pub fn delay(&self, now_ms: i64) -> Duration {
        Duration::from_millis(
            u64::try_from(self.expiration_ms.saturating_sub(now_ms).max(0)).unwrap_or_default(),
        )
    }

    /// Compares bucket deadlines.
    #[must_use]
    pub fn compare_to(&self, other: &Self) -> Ordering {
        self.expiration_ms.cmp(&other.expiration_ms)
    }

    /// Returns the current task count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    /// Returns whether the bucket is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

/// A single-level timing wheel. Tasks beyond its interval are rejected so a
/// caller can forward them to an overflow wheel or another bounded queue.
pub struct TimingWheel {
    tick_ms: i64,
    wheel_size: usize,
    interval_ms: i64,
    current_time_ms: i64,
    buckets: Vec<TimerTaskList>,
    consumer: Arc<dyn Fn(&TimerTaskList) + Send + Sync>,
}

impl fmt::Debug for TimingWheel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TimingWheel")
            .field("tick_ms", &self.tick_ms)
            .field("wheel_size", &self.wheel_size)
            .field("current_time_ms", &self.current_time_ms)
            .finish_non_exhaustive()
    }
}

impl TimingWheel {
    /// Creates a wheel at the current system time.
    pub fn new<F>(tick: Duration, wheel_size: usize, consumer: F) -> Result<Self, CronError>
    where
        F: Fn(&TimerTaskList) + Send + Sync + 'static,
    {
        Self::with_current_time(tick, wheel_size, now_millis(), consumer)
    }

    /// Creates a wheel at an explicit Unix-millisecond time.
    pub fn with_current_time<F>(
        tick: Duration,
        wheel_size: usize,
        current_time_ms: i64,
        consumer: F,
    ) -> Result<Self, CronError>
    where
        F: Fn(&TimerTaskList) + Send + Sync + 'static,
    {
        Self::with_consumer(tick, wheel_size, current_time_ms, Arc::new(consumer))
    }

    fn with_consumer(
        tick: Duration,
        wheel_size: usize,
        current_time_ms: i64,
        consumer: Arc<dyn Fn(&TimerTaskList) + Send + Sync>,
    ) -> Result<Self, CronError> {
        let tick_ms = i64::try_from(tick.as_millis()).unwrap_or(i64::MAX);
        if tick_ms == 0 || wheel_size == 0 {
            return Err(CronError::InvalidTimingWheel);
        }
        let wheel_size_i64 =
            i64::try_from(wheel_size).map_err(|_| CronError::InvalidTimingWheel)?;
        let interval_ms = tick_ms
            .checked_mul(wheel_size_i64)
            .ok_or(CronError::InvalidTimingWheel)?;
        Ok(Self {
            tick_ms,
            wheel_size,
            interval_ms,
            current_time_ms: current_time_ms - current_time_ms.rem_euclid(tick_ms),
            buckets: (0..wheel_size).map(|_| TimerTaskList::new()).collect(),
            consumer,
        })
    }

    /// Adds a task when its deadline fits this wheel's current interval.
    pub fn add_task(&mut self, task: TimerTask) -> bool {
        let deadline = task.deadline_ms();
        if deadline < self.current_time_ms.saturating_add(self.tick_ms)
            || deadline >= self.current_time_ms.saturating_add(self.interval_ms)
        {
            return false;
        }
        let virtual_id = deadline / self.tick_ms;
        let wheel_size = i64::try_from(self.wheel_size).unwrap_or(i64::MAX);
        let index = usize::try_from(virtual_id.rem_euclid(wheel_size)).unwrap_or_default();
        let expiration = virtual_id.saturating_mul(self.tick_ms);
        let bucket = &mut self.buckets[index];
        bucket.set_expiration(expiration);
        bucket.add_task(task);
        (self.consumer)(bucket);
        true
    }

    /// Advances the wheel and flushes every elapsed bucket.
    pub fn advance_clock<F>(&mut self, timestamp_ms: i64, mut flush: F)
    where
        F: FnMut(TimerTask),
    {
        if timestamp_ms < self.current_time_ms.saturating_add(self.tick_ms) {
            return;
        }
        self.current_time_ms = timestamp_ms - timestamp_ms.rem_euclid(self.tick_ms);
        for bucket in &mut self.buckets {
            if bucket.expiration() >= 0 && bucket.expiration() <= self.current_time_ms {
                bucket.flush(&mut flush);
            }
        }
    }
}

#[derive(Clone)]
struct ScheduledTask {
    deadline_ms: i64,
    sequence: u64,
    task: TimerTask,
}

impl PartialEq for ScheduledTask {
    fn eq(&self, other: &Self) -> bool {
        (self.deadline_ms, self.sequence) == (other.deadline_ms, other.sequence)
    }
}

impl Eq for ScheduledTask {}

impl PartialOrd for ScheduledTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScheduledTask {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.deadline_ms, other.sequence).cmp(&(self.deadline_ms, self.sequence))
    }
}

enum TimerCommand {
    Add(TimerTask),
    Stop,
}

/// Explicitly started and stopped one-shot timer service.
#[derive(Debug)]
pub struct SystemTimer {
    delay_queue_timeout: Duration,
    pending: Vec<TimerTask>,
    sender: Option<mpsc::Sender<TimerCommand>>,
    worker: Option<JoinHandle<()>>,
}

impl Default for SystemTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemTimer {
    /// Creates a stopped timer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            delay_queue_timeout: Duration::from_millis(100),
            pending: Vec::new(),
            sender: None,
            worker: None,
        }
    }

    /// Sets the maximum worker wake-up interval.
    pub fn set_delay_queue_timeout(&mut self, timeout: Duration) -> Result<&mut Self, CronError> {
        if timeout.is_zero() || self.sender.is_some() {
            return Err(CronError::InvalidTimerState);
        }
        self.delay_queue_timeout = timeout;
        Ok(self)
    }

    /// Starts the owned worker. Starting twice is rejected.
    pub fn start(&mut self) -> Result<&mut Self, CronError> {
        self.start_with(|receiver, initial, timeout| {
            thread::Builder::new()
                .name("hutool-system-timer".to_owned())
                .spawn(move || run_timer(&receiver, initial, timeout))
        })
    }

    fn start_with<S>(&mut self, spawner: S) -> Result<&mut Self, CronError>
    where
        S: FnOnce(
            mpsc::Receiver<TimerCommand>,
            Vec<TimerTask>,
            Duration,
        ) -> std::io::Result<JoinHandle<()>>,
    {
        if self.sender.is_some() {
            return Err(CronError::TimerAlreadyStarted);
        }
        let (sender, receiver) = mpsc::channel();
        let timeout = self.delay_queue_timeout;
        let initial = std::mem::take(&mut self.pending);
        self.install_worker(sender, spawner(receiver, initial, timeout))
    }

    fn install_worker(
        &mut self,
        sender: mpsc::Sender<TimerCommand>,
        worker: std::io::Result<JoinHandle<()>>,
    ) -> Result<&mut Self, CronError> {
        let worker = worker.map_err(CronError::TimerThread)?;
        self.sender = Some(sender);
        self.worker = Some(worker);
        Ok(self)
    }

    /// Adds a task before or after start.
    pub fn add_task(&mut self, task: TimerTask) -> Result<(), CronError> {
        if let Some(sender) = &self.sender {
            sender
                .send(TimerCommand::Add(task))
                .map_err(|_| CronError::TimerStopped)
        } else {
            self.pending.push(task);
            Ok(())
        }
    }

    /// Stops the worker and waits for completion.
    pub fn stop(&mut self) {
        if let Some(sender) = self.sender.take() {
            let _ = sender.send(TimerCommand::Stop);
        }
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }

    /// Returns whether the worker is running.
    #[must_use]
    pub const fn is_started(&self) -> bool {
        self.sender.is_some()
    }
}

impl Drop for SystemTimer {
    fn drop(&mut self) {
        self.stop();
    }
}

fn run_timer(receiver: &mpsc::Receiver<TimerCommand>, initial: Vec<TimerTask>, max_wait: Duration) {
    let mut queue = BinaryHeap::new();
    let mut sequence = 0_u64;
    for task in initial {
        queue.push(ScheduledTask {
            deadline_ms: task.deadline_ms(),
            sequence,
            task,
        });
        sequence = sequence.wrapping_add(1);
    }
    loop {
        while queue
            .peek()
            .is_some_and(|entry| entry.deadline_ms <= now_millis())
        {
            let entry = queue
                .pop()
                .expect("a successful heap peek guarantees one queued task");
            entry.task.execute();
        }
        let wait = queue.peek().map_or(max_wait, |entry| {
            bounded_wait(entry.deadline_ms, now_millis(), max_wait)
        });
        match receiver.recv_timeout(wait) {
            Ok(TimerCommand::Add(task)) => {
                queue.push(ScheduledTask {
                    deadline_ms: task.deadline_ms(),
                    sequence,
                    task,
                });
                sequence = sequence.wrapping_add(1);
            }
            Ok(TimerCommand::Stop) | Err(mpsc::RecvTimeoutError::Disconnected) => return,
            Err(mpsc::RecvTimeoutError::Timeout) => {}
        }
    }
}

fn bounded_wait(deadline_ms: i64, now_ms: i64, max_wait: Duration) -> Duration {
    let remaining = deadline_ms.saturating_sub(now_ms).max(0);
    Duration::from_millis(
        u64::try_from(remaining).expect("a non-negative i64 is representable as u64"),
    )
    .min(max_wait)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

    use super::*;

    fn do_nothing() {}

    fn ignore_list(_list: &TimerTaskList) {}

    fn ignore_task(_task: TimerTask) {}

    #[test]
    fn timer_tasks_and_lists_are_one_shot_identity_aware_and_ordered() {
        let runs = Arc::new(AtomicUsize::new(0));
        let task_runs = Arc::clone(&runs);
        let task = TimerTask::new(
            move || {
                task_runs.fetch_add(1, AtomicOrdering::SeqCst);
            },
            Duration::from_millis(5),
        );
        assert_eq!(task.delay_ms(), 5);
        assert_eq!(task.delay(), Duration::from_millis(5));
        assert!(format!("{task:?}").contains("pending: true"));
        let mut list = TimerTaskList::new();
        assert!(list.is_empty());
        assert!(list.set_expiration(10));
        assert!(!list.set_expiration(10));
        list.add_task(task.clone());
        assert_eq!(list.len(), 1);
        assert!(list.remove_task(&task));
        assert!(!list.remove_task(&task));
        list.add_task(task.clone());
        let mut flushed = Vec::new();
        list.flush(|task| flushed.push(task));
        assert_eq!(flushed.len(), 1);
        assert!(flushed[0].execute());
        assert!(!flushed[0].execute());
        assert_eq!(runs.load(AtomicOrdering::SeqCst), 1);
        assert_eq!(list.expiration(), -1);
        assert_eq!(list.delay(0), Duration::ZERO);
        let mut later = TimerTaskList::new();
        later.set_expiration(20);
        assert_eq!(later.compare_to(&list), Ordering::Greater);

        let first = ScheduledTask {
            deadline_ms: 10,
            sequence: 1,
            task: TimerTask::new(do_nothing, Duration::ZERO),
        };
        let second = ScheduledTask {
            deadline_ms: 20,
            sequence: 2,
            task: TimerTask::new(do_nothing, Duration::ZERO),
        };
        assert!(first != second);
        assert_eq!(first.partial_cmp(&second), Some(first.cmp(&second)));
        do_nothing();
        ignore_task(TimerTask::new(do_nothing, Duration::ZERO));
    }

    #[test]
    fn timing_wheel_accepts_current_interval_and_flushes_elapsed_buckets() {
        let bucket_notifications = Arc::new(AtomicUsize::new(0));
        let notification_counter = Arc::clone(&bucket_notifications);
        let now = now_millis();
        let mut wheel =
            TimingWheel::with_current_time(Duration::from_millis(10), 64, now, move |_| {
                notification_counter.fetch_add(1, AtomicOrdering::SeqCst);
            })
            .unwrap();
        let mut live_wheel = TimingWheel::new(Duration::from_millis(10), 8, ignore_list).unwrap();
        assert!(live_wheel.add_task(TimerTask::new(do_nothing, Duration::from_millis(20))));
        live_wheel.advance_clock(now_millis(), ignore_task);
        live_wheel.advance_clock(now_millis() + 100, ignore_task);
        assert!(TimingWheel::with_current_time(Duration::ZERO, 1, now, ignore_list).is_err());
        assert!(
            TimingWheel::with_current_time(Duration::from_millis(1), usize::MAX, now, ignore_list,)
                .is_err()
        );
        assert!(
            TimingWheel::with_current_time(
                Duration::from_millis(i64::MAX as u64),
                2,
                now,
                ignore_list,
            )
            .is_err()
        );
        let task = TimerTask::new(do_nothing, Duration::from_millis(20));
        assert!(wheel.add_task(task));
        assert_eq!(bucket_notifications.load(AtomicOrdering::SeqCst), 1);
        wheel.advance_clock(now, ignore_task);
        let flushed = Arc::new(AtomicUsize::new(0));
        let count = Arc::clone(&flushed);
        wheel.advance_clock(now + 100, move |task| {
            task.execute();
            count.fetch_add(1, AtomicOrdering::SeqCst);
        });
        assert_eq!(flushed.load(AtomicOrdering::SeqCst), 1);
        wheel.advance_clock(now + 101, ignore_task);
        let far = TimerTask::new(do_nothing, Duration::from_secs(5));
        assert!(!wheel.add_task(far));
        assert!(format!("{wheel:?}").contains("TimingWheel"));
        assert_eq!(
            bounded_wait(20, 10, Duration::from_secs(1)),
            Duration::from_millis(10)
        );
        assert_eq!(
            bounded_wait(20, 10, Duration::from_millis(5)),
            Duration::from_millis(5)
        );
    }

    #[test]
    fn system_timer_requires_explicit_lifecycle_and_runs_pending_tasks() {
        let runs = Arc::new(AtomicUsize::new(0));
        let task_runs = Arc::clone(&runs);
        let mut timer = SystemTimer::new();
        timer
            .set_delay_queue_timeout(Duration::from_millis(2))
            .unwrap();
        timer
            .add_task(TimerTask::new(
                move || {
                    task_runs.fetch_add(1, AtomicOrdering::SeqCst);
                },
                Duration::from_millis(1),
            ))
            .unwrap();
        timer.start().unwrap();
        assert!(timer.is_started());
        assert!(timer.start().is_err());
        assert!(
            timer
                .set_delay_queue_timeout(Duration::from_millis(1))
                .is_err()
        );
        let live_runs = Arc::clone(&runs);
        timer
            .add_task(TimerTask::new(
                move || {
                    live_runs.fetch_add(1, AtomicOrdering::SeqCst);
                },
                Duration::from_millis(1),
            ))
            .unwrap();
        timer
            .add_task(TimerTask::new(do_nothing, Duration::from_secs(1)))
            .unwrap();
        thread::sleep(Duration::from_millis(15));
        timer.stop();
        assert!(!timer.is_started());
        assert_eq!(runs.load(AtomicOrdering::SeqCst), 2);
        assert!(
            SystemTimer::new()
                .set_delay_queue_timeout(Duration::ZERO)
                .is_err()
        );

        let mut default_timer = SystemTimer::default();
        assert!(!default_timer.is_started());
        let (sender, receiver) = mpsc::channel();
        drop(receiver);
        default_timer.sender = Some(sender);
        assert!(
            default_timer
                .add_task(TimerTask::new(do_nothing, Duration::ZERO))
                .is_err()
        );
        default_timer.sender = None;

        let mut failed_timer = SystemTimer::new();
        let (sender, _receiver) = mpsc::channel();
        assert!(
            failed_timer
                .install_worker(
                    sender,
                    Err(std::io::Error::other("injected thread failure")),
                )
                .is_err()
        );
    }
}
