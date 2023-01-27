use crate::runtime::Runtime;
use std::collections::LinkedList;
use std::future::Future;
use std::ops::Deref;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, LazyLock, Mutex};
use std::task::Poll;

type Task = ();

// how we can improve it?
pub(crate) type Queue = Arc<Mutex<LinkedList<Arc<Task>>>>;

pub(crate) static INSTANCE: LazyLock<CustomRio> = LazyLock::new(|| CustomRio::new());

pub struct CustomRio {
    pub(crate) task_queue: Queue,
    pub(crate) size_queue: AtomicUsize,
}

impl CustomRio {
    fn new() -> Self {
        CustomRio::start();
        let queue = Arc::new(Mutex::new(LinkedList::new()));
        CustomRio {
            task_queue: queue.to_owned(),
            size_queue: AtomicUsize::new(0),
        }
    }

    /// start the runtime by spowing the event look on a thread!
    fn start() {
        std::thread::spawn(|| loop {
            let task = match CustomRio::get().pop_front() {
                Some(task) => task,
                None => continue,
            };

            let Poll::Ready(_) = task.poll();
        });
    }

    pub fn get() -> &'static CustomRio {
        INSTANCE.deref()
    }

    pub fn pop_front(&self) -> Option<Arc<Task>> {
        self.task_queue.lock().unwrap().pop_front()
    }

    /// This is the function that gets called by the `spawn` function to
    /// actually create a new `Task` in our queue. It takes the `Future`,
    /// constructs a `Task` and then pushes it to the back of the queue.
    pub fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        //self.inner_spawn(Task::new(false, future));
    }
    /// This is the function that gets called by the `spawn_blocking` function to
    /// actually create a new `Task` in our queue. It takes the `Future`,
    /// constructs a `Task` and then pushes it to the front of the queue
    /// where the runtime will check if it should block and then block until
    /// this future completes.
    pub fn spawn_blocking(&self, future: impl Future<Output = ()> + Send + 'static) {
        //self.inner_spawn_blocking(Task::new(true, future));
    }

    /// This function just takes a `Task` and pushes it onto the queue. We use this
    /// both for spawning new `Task`s and to push old ones that get woken up
    /// back onto the queue.
    pub(crate) fn inner_spawn(&self, task: Arc<Task>) {
        //self.task_queue.lock().unwrap().push_back(task);
    }

    /// This function takes a `Task` and pushes it to the front of the queue
    /// if it is meant to block. We use this both for spawning new blocking
    /// `Task`s and to push old ones that get woken up back onto the queue.
    pub(crate) fn inner_spawn_blocking(&self, task: Arc<Task>) {
        //self.task_queue.lock().unwrap().push_front(task);
    }
}

impl Runtime for CustomRio {
    fn new() -> &'static Self {
        CustomRio::get()
    }

    fn block_on(&self, future: impl std::future::Future<Output = ()> + Send + 'static) {
        self.spawn_blocking(future);
    }
}
