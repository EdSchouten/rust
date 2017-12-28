extern crate cloudabi;

use cell::UnsafeCell;
use sync::atomic::{AtomicU32, Ordering};

pub struct RWLock {
    lock: UnsafeCell<AtomicU32>,
}

unsafe impl Send for RWLock {}
unsafe impl Sync for RWLock {}

impl RWLock {
    pub const fn new() -> RWLock {
        RWLock {
            lock: UnsafeCell::new(AtomicU32::new(cloudabi::LOCK_UNLOCKED.0)),
        }
    }

    pub unsafe fn try_read(&self) -> bool {
        // TODO(ed): Implement!
        false
    }

    pub unsafe fn read(&self) {
        // TODO(ed): Implement!
    }

    pub unsafe fn read_unlock(&self) {
        // TODO(ed): Implement!
    }

    pub unsafe fn try_write(&self) -> bool {
        // TODO(ed): Implement!
        false
    }

    pub unsafe fn write(&self) {
        // TODO(ed): Implement!
    }

    pub unsafe fn write_unlock(&self) {
        // TODO(ed): Implement!
    }

    pub unsafe fn destroy(&self) {
        let lock = self.lock.get();
        assert_eq!((*lock).load(Ordering::Relaxed), cloudabi::LOCK_UNLOCKED.0);
    }
}
