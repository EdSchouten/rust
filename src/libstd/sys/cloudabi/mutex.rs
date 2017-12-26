extern crate cloudabi;

use mem;
use sync::atomic::AtomicU32;

pub unsafe fn raw(m: &Mutex) -> &mut AtomicU32 {
    &mut m.lock
}

pub struct Mutex {
    lock: AtomicU32,
}

impl Mutex {
    pub const fn new() -> Mutex {
        Mutex { lock: AtomicU32::new(cloudabi::LOCK_UNLOCKED.0) }
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
    lock: AtomicU32,
    recursion: u32,
}

impl ReentrantMutex {
    pub unsafe fn uninitialized() -> ReentrantMutex {
        ReentrantMutex { ..mem::uninitialized() }
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
