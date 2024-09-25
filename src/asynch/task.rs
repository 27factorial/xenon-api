use critical_section as cs;
use futures::{
    future::BoxFuture,
    task::{waker_ref, ArcWake, WakerRef},
};
use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
    task::{Context, Poll, Wake, Waker},
};

use super::channel::Sender;

pub struct Task {
    pub(crate) future: cs::Mutex<RefCell<Option<BoxFuture<'static, ()>>>>,
    pub(crate) sender: Sender,
}

impl Task {
    pub(crate) fn poll(self: Arc<Self>) {
        cs::with(|cs| {
            let mut slot = self.future.borrow_ref_mut(cs);

            let Some(mut fut) = slot.take() else {
                panic!("Attempted to poll task after it returned `Poll::Ready`")
            };

            let waker = waker_ref(&self);
            let mut ctx = Context::from_waker(&waker);

            if fut.as_mut().poll(&mut ctx).is_pending() {
                *slot = Some(fut)
            }
        });
    }
}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let task = Arc::clone(self);

        // A critical section is needed here, to ensure that no interrupts in ESP32-land attempt to
        // take the sender/receiver's internal mutex. It will be released in the executor's poll
        // loop.
        self.sender.send(task);
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        Wake::wake_by_ref(arc_self);
    }
}
