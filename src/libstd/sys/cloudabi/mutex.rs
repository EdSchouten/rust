use sync::atomic::AtomicU32;

pub unsafe fn raw(m: &Mutex) -> &mut AtomicU32 {
    &mut m.lock
}

pub struct Mutex {
    lock: AtomicU32,
}

pub struct ReentrantMutex {
    lock: AtomicU32,
    recursion: u32,
}
