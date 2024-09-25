use super::{
    channel::{self, Receiver, Sender},
    task::Task,
};
use critical_section as cs;
use futures::FutureExt;
use std::{cell::RefCell, future::Future, sync::Arc};

// Wasm is single threaded, so the executor design can be fairly simple.
#[derive(Clone)]
pub(crate) struct Executor {
    queue: Receiver,
    spawner: Spawner,
}

impl Executor {
    pub(crate) fn new() -> Self {
        let (sender, receiver) = channel::new();

        let spawner = Spawner { sender };

        Self {
            queue: receiver,
            spawner,
        }
    }

    pub(crate) fn start(&'static mut self, init: impl FnOnce(Spawner)) {
        init(self.spawner.clone());

        while let Some(task) = self.queue.recv() {
            task.poll();
        }
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct Spawner {
    sender: Sender,
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        let future = future.boxed();

        let task = Arc::new(Task {
            future: cs::Mutex::new(RefCell::new(Some(future))),
            sender: self.sender.clone(),
        });

        self.sender.send(task);
    }
}
