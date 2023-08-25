use alloc::{collections::VecDeque, sync::Arc};
use core::ops::Deref;
use core::sync::atomic::{AtomicIsize, Ordering};

use crate::BaseScheduler;

/// A task wrapper for the [`SimpleScheduler`].
pub struct SimpleTask<T> {
    inner: T,
    time_slice_used: AtomicIsize,
    priority: AtomicIsize,
}

impl<T> SimpleTask<T> {
    /// Creates a new [`SimpleTask`] from the inner task struct.
    pub const fn new(inner: T) -> Self {
        Self {
            inner,
            time_slice_used: AtomicIsize::new(0_isize),
            priority: AtomicIsize::new(0_isize),
        }
    }

    fn time_slice_used(&self) -> isize {
        self.time_slice_used.load(Ordering::Acquire)
    }

    fn priority(&self) -> isize {
        self.priority.load(Ordering::Acquire)
    }

    fn reset_time_slice(&self) {
        self.time_slice_used.store(0_isize, Ordering::Release);
    }

    fn reset_priority(&self) {
        self.priority.store(0_isize, Ordering::Release);
    }


    /// Returns a reference to the inner task struct.
    pub const fn inner(&self) -> &T {
        &self.inner
    }
}

impl<T> Deref for SimpleTask<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// A simple scheduler.
/// My impl : Simple 2-Level Feedback Queen
/// 

pub struct SimpleScheduler<T> {
    ready_queue: [VecDeque<Arc<SimpleTask<T>>>;2],
}

const MAX_TIME_SLICE_0: isize = 2; // priority0, with time_slice 2
const MAX_TIME_SLICE_1: isize = 5; // priority1, with time_slice 5

pub fn max_time_slice(priority:isize) -> isize{
    match priority {
        0 => MAX_TIME_SLICE_0,
        1 => MAX_TIME_SLICE_1,
        _ => MAX_TIME_SLICE_1,
    }
}

impl<T> SimpleScheduler<T> {
    /// Creates a new empty [`SimpleScheduler`].
    pub const fn new() -> Self {
        Self {
            ready_queue : [VecDeque::new(),VecDeque::new()]
        }
    }
    /// get the name of scheduler
    pub fn scheduler_name() -> &'static str {
        "Simple"
    }
}

impl<T> BaseScheduler for SimpleScheduler<T> {
    type SchedItem = Arc<SimpleTask<T>>;

    fn init(&mut self) {}

    fn add_task(&mut self, task: Self::SchedItem) {
        trace!("######### add_task");
        self.ready_queue[0].push_back(task);
    }

    fn remove_task(&mut self, task: &Self::SchedItem) -> Option<Self::SchedItem> {
        trace!("######### remove_task");
        if let Some(v) = self.ready_queue[0]
            .iter()
            .position(|t| Arc::ptr_eq(t, task))
            .and_then(|idx| self.ready_queue[0].remove(idx)) {
                Some(v)
            }
        else{
            self.ready_queue[1]
            .iter()
            .position(|t| Arc::ptr_eq(t, task))
            .and_then(|idx| self.ready_queue[1].remove(idx))
        }
    }

    fn pick_next_task(&mut self) -> Option<Self::SchedItem> {
        if !self.ready_queue[0].is_empty(){
            return self.ready_queue[0].pop_front();
        }
        self.ready_queue[1].pop_front()
    }

    fn put_prev_task(&mut self, prev: Self::SchedItem, _preempt: bool) {
        // put task to low-priority-queue
        if prev.time_slice_used() < max_time_slice(prev.priority()) && _preempt {
            self.ready_queue[prev.priority() as usize].push_front(prev)
        } else {
            prev.reset_time_slice();
            let next_priority = match prev.priority() {
                1 => 1,
                _ => prev.priority() + 1
            };
            self.ready_queue[next_priority as usize].push_back(prev)
        }
    }

    fn task_tick(&mut self, _current: &Self::SchedItem) -> bool {
        let old_slice = _current.time_slice_used.fetch_add(1, Ordering::Release);
        old_slice >= max_time_slice(_current.priority()) - 1
    }

    fn set_priority(&mut self, _task: &Self::SchedItem, _prio: isize) -> bool {
        false
    }
}