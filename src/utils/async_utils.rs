use std::future::Future;
use tokio::sync::Mutex;

pub type AsyncMutex<T> = Mutex<T>;

pub async fn lock_and_execute<T, F, R>(mutex: &AsyncMutex<T>, f: F) -> R
where
    F: FnOnce(&mut T) -> R + Send + 'static,
    R: Send + 'static,
{
    let mut guard = mutex.lock().await;
    f(&mut *guard)
}