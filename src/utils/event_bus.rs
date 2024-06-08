use std::sync::Arc;
use tokio::sync::broadcast::{channel, Receiver, Sender};

pub struct EventBus<T> {
    sender: Sender<T>,
}

impl<T: Clone> EventBus<T> {
    pub fn new() -> Self {
        let (sender, _) = channel(100);
        EventBus { sender }
    }

    pub fn subscribe(&self) -> Receiver<T> {
        self.sender.subscribe()
    }

    pub fn publish(&self, event: T) {
        let _ = self.sender.send(event);
    }
}

pub type SharedEventBus<T> = Arc<EventBus<T>>;