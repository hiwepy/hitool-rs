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

use super::timer_task::TimerTask;

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

enum TimerCommand {
    Add(TimerTask),
    Stop,
}
