use spin::Mutex;

pub struct Spinlock<T> {
    lock: Mutex<T>,
}

impl<T> Spinlock<T> {
    pub const fn new(data: T) -> Self {
        Spinlock {
            lock: Mutex::new(data),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<T> {
        self.lock.lock()
    }
}
