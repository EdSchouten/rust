use sync::atomic::AtomicU32;

pub struct Mutex {
    lock: AtomicU32,
}

pub struct ReentrantMutex {
    lock: AtomicU32,
    recursion: u32,
}
