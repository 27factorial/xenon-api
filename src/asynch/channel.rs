use super::task::Task;
use crate::ffi;
use critical_section as cs;
use std::{cell::RefCell, sync::Arc};

pub(crate) fn new() -> (Sender, Receiver) {
    let shared = Arc::new(Shared::new());
    let sender = Sender(Arc::clone(&shared));
    let receiver = Receiver(Arc::clone(&shared));

    (sender, receiver)
}

#[derive(Clone)]
pub(crate) struct Sender(Arc<Shared>);

impl Sender {
    pub fn send(&self, task: Arc<Task>) {
        self.0.push(task);
        unsafe { ffi::asynch::resume() }
    }
}

#[derive(Clone)]
pub(crate) struct Receiver(Arc<Shared>);

impl Receiver {
    pub fn recv(&self) -> Option<Arc<Task>> {
        loop {
            match self.0.pop() {
                Some(task) => break Some(task),
                None => unsafe {
                    ffi::asynch::wait();
                },
            }
        }
    }
}

// A RefCell wrapped in a critical section Mutex is enough to protect the inner Vec, since wasm is
// single-threaded and acquiring the critical section guarantees that the Wasm code won't be
// preempted by an interrupt.
struct Shared(cs::Mutex<RefCell<Vec<Arc<Task>>>>);

impl Shared {
    const fn new() -> Self {
        Self(cs::Mutex::new(RefCell::new(Vec::new())))
    }

    fn push(&self, task: Arc<Task>) {
        cs::with(|cs| self.0.borrow_ref_mut(cs).push(task))
    }

    fn pop(&self) -> Option<Arc<Task>> {
        cs::with(|cs| self.0.borrow_ref_mut(cs).pop())
    }
}
