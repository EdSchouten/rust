extern crate cloudabi;

use cell::UnsafeCell;
use mem;
use sync::atomic::AtomicU32;

pub unsafe fn raw(m: &Mutex) -> *mut AtomicU32 {
    m.lock.get()
}

pub struct Mutex {
    lock: UnsafeCell<AtomicU32>,
}

impl Mutex {
    pub const fn new() -> Mutex {
        Mutex {
            lock: UnsafeCell::new(AtomicU32::new(cloudabi::LOCK_UNLOCKED.0)),
        }
    }

    pub unsafe fn init(&mut self) {
        // TODO(ed): Implement!
    }

    pub unsafe fn try_lock(&self) -> bool {
        // TODO(ed): Implement!
        false
    }

    pub unsafe fn lock(&self) {
        // TODO(ed): Implement!
    }

    pub unsafe fn unlock(&self) {
        // TODO(ed): Implement!
    }

    pub unsafe fn destroy(&self) {
        // TODO(ed): Implement!
    }
}

pub struct ReentrantMutex {
    lock: UnsafeCell<AtomicU32>,
    recursion: u32,
}

impl ReentrantMutex {
    pub unsafe fn uninitialized() -> ReentrantMutex {
        mem::uninitialized()
    }

    pub unsafe fn init(&mut self) {
        // TODO(ed): Implement!
    }

    pub unsafe fn try_lock(&self) -> bool {
        // TODO(ed): Implement!
        false
    }

    pub unsafe fn lock(&self) {
        // TODO(ed): Implement!
    }

    pub unsafe fn unlock(&self) {
        // TODO(ed): Implement!
    }

    pub unsafe fn destroy(&self) {
        // TODO(ed): Implement!
    }
}
