use core::sync::atomic::{AtomicUsize, Ordering};

pub struct RingBuffer<'a, T> {
    head: AtomicUsize, // next write
    tail: AtomicUsize, // next read
    buf: &'a mut [T],  // backing storage
    capacity: usize,
}

impl<'a, T> RingBuffer<'a, T> {
    pub fn new(storage: &'a mut [T]) -> Self {
        let capacity = storage.len();
        Self {
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            buf: storage,
            capacity,
        }
    }

    /// Push with overwrite: newest data wins
    pub fn push(&mut self, value: T) {
        let head = self.head.load(Ordering::Relaxed);
        let next = (head + 1) % self.capacity;

        let tail = self.tail.load(Ordering::Acquire);

        // FULL: advance tail (drop oldest)
        if next == tail {
            self.tail
                .store((tail + 1) % self.capacity, Ordering::Release);
        }

        self.buf[head] = value;
        self.head.store(next, Ordering::Release);
    }

    /// Pop (None if empty)
    pub fn pop(&self) -> Option<T>
    where
        T: Copy,
    {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);

        if tail == head {
            return None;
        }

        let value = self.buf[tail];
        self.tail
            .store((tail + 1) % self.capacity, Ordering::Release);
        Some(value)
    }
}
