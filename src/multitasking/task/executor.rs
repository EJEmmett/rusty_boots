use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::task::Wake;
use core::task::{Context, Poll, Waker};

use crate::multitasking::task::{Task, TaskID};
use crossbeam_queue::ArrayQueue;

static TASK_QUEUE_SIZE: usize = 100;

struct TaskWaker {
    task_id: TaskID,
    task_queue: Arc<ArrayQueue<TaskID>>,
}

impl TaskWaker {
    #[inline]
    fn new_waker(task_id: TaskID, task_queue: Arc<ArrayQueue<TaskID>>) -> Waker {
        Waker::from(Arc::new(Self {
            task_id,
            task_queue,
        }))
    }

    #[inline]
    fn wake_task(&self) {
        self.task_queue
            .push(self.task_id)
            .expect("Task Queue Full.")
    }
}

impl Wake for TaskWaker {
    #[inline]
    fn wake(self: Arc<Self>) {
        self.wake_task()
    }

    #[inline]
    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task()
    }
}

pub struct Executor {
    tasks: BTreeMap<TaskID, Task>,
    wakers: BTreeMap<TaskID, Waker>,
    task_queue: Arc<ArrayQueue<TaskID>>,
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

impl Executor {
    #[inline]
    pub fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            wakers: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(TASK_QUEUE_SIZE)),
        }
    }

    #[inline]
    pub fn run(&mut self) -> ! {
        use x86_64::instructions::interrupts;

        loop {
            self.run_tasks();
            interrupts::disable();
            if self.task_queue.is_empty() {
                interrupts::enable_and_hlt()
            }
            interrupts::enable()
        }
    }

    #[inline]
    pub fn spawn(&mut self, task: Task) {
        let task_id = task.id;
        if self.tasks.insert(task_id, task).is_some() {
            panic!("Error: Task already in queue!")
        }
        self.task_queue.push(task_id).expect("Task Queue Full.");
    }

    #[inline]
    fn run_tasks(&mut self) {
        let Self {
            tasks,
            wakers,
            task_queue,
        } = self;

        while let Ok(task_id) = task_queue.pop() {
            let task = match tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue,
            };

            let waker = wakers
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new_waker(task_id, task_queue.clone()));

            let mut context = Context::from_waker(waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {
                    tasks.remove(&task_id);
                    wakers.remove(&task_id);
                }
                Poll::Pending => {}
            }
        }
    }
}
