use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};

use crate::CustomRio;

type PinFuture = Mutex<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>;

/// The `Task` is the basic unit for the executor. It represents a `Future`
/// that may or may not be completed. We spawn `Task`s to be run and poll
/// them until completion in a non-blocking manner unless specifically asked
/// for.
pub(crate) struct Task {
    /// This is the actual `Future` we will poll inside of a `Task`. We `Box`
    /// and `Pin` the `Future` when we create a task so that we don't need
    /// to worry about pinning or more complicated things in the runtime.
    future: PinFuture,
    /// We need a way to check if the runtime should block on this task and
    /// so we use a boolean here to check that!
    block: bool,
    // The waker is a self reference of the stack but if it is
    // not None, this mean that it is already been pool
    waker: Option<Arc<Waker>>,
}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        if self.is_blocking() {
            CustomRio::get().inner_spawn_blocking(self);
        } else {
            CustomRio::get().inner_spawn(self);
        }
    }
}

impl Drop for Task {
    fn drop(&mut self) {
        CustomRio::get().size_queue.fetch_sub(1, Ordering::Relaxed);
    }
}

impl Task {
    pub(crate) fn new(block: bool, future: impl Future<Output = ()> + Send + 'static) -> Arc<Self> {
        CustomRio::get().size_queue.fetch_add(1, Ordering::Relaxed);
        Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            block,
            waker: None,
        })
    }

    /// Pool the following task!
    ///
    /// See more https://cfsamson.github.io/books-futures-explained/3_waker_context.html
    pub fn poll(self: &Arc<Self>) -> Poll<()> {
        // If the waker exist there is no need to
        // poll a new waker, this feature is already in the background
        if let None = self.waker {
            let waker = self.waker();
            let mut ctx = Context::from_waker(&waker);
            // FIXME: this is the good place where to remove the element
            // from the queue?
            self.future.lock().unwrap().as_mut().poll(&mut ctx)
        } else {
            Poll::Pending
        }
    }

    pub fn waker(self: &Arc<Self>) -> Waker {
        self.clone().into()
    }

    /// The Task is blocking.
    pub fn is_blocking(&self) -> bool {
        self.block
    }
}
