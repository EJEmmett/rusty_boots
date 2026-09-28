pub mod executor;
pub mod keyboard;

use alloc::boxed::Box;
use core::sync::atomic::{AtomicU64, Ordering};
use core::{future::Future, pin::Pin};

use core::task::{Context, Poll};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
struct TaskID(u64);

impl TaskID {
    #[inline]
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskID(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub struct Task {
    id: TaskID,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    #[inline]
    pub fn new(future: impl Future<Output = ()> + 'static) -> Self {
        Self {
            id: TaskID::new(),
            future: Box::pin(future),
        }
    }

    #[inline]
    pub fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}
