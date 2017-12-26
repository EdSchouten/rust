extern crate cloudabi;

use sync::atomic::AtomicU32;

pub struct RWLock {
    lock: AtomicU32,
}

impl RWLock {
    pub const fn new() -> RWLock {
        RWLock {
            lock: AtomicU32::new(cloudabi::LOCK_UNLOCKED.0),
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
        // TODO(ed): Implement!
    }
}
