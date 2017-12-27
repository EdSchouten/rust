// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate cloudabi;

use cell::UnsafeCell;
use mem;
use sync::atomic::{AtomicU32, Ordering};

extern "C" {
    #[thread_local]
    static __pthread_thread_id: cloudabi::tid;
}

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
        // This function should normally reinitialize the mutex after
        // moving it to a different memory address. This implementation
        // does not require adjustments after moving.
    }

    pub unsafe fn try_lock(&self) -> bool {
        // Attempt to acquire the lock.
        let lock = self.lock.get();
        match (*lock).compare_exchange(
            cloudabi::LOCK_UNLOCKED.0,
            __pthread_thread_id.0 | cloudabi::LOCK_WRLOCKED.0,
            Ordering::Acquire,
            Ordering::Relaxed,
        ) {
            Ok(_) => {
                // Success.
                true
            }
            Err(old) => {
                // Failure. Lock cannot be acquired by the current thread.
                assert_ne!(
                    old & !cloudabi::LOCK_KERNEL_MANAGED.0,
                    __pthread_thread_id.0 | cloudabi::LOCK_WRLOCKED.0
                );
                false
            }
        }
    }

    pub unsafe fn lock(&self) {
        // TODO(ed): Implement!
    }

    pub unsafe fn unlock(&self) {
        // TODO(ed): Implement!
    }

    pub unsafe fn destroy(&self) {
        // TODO(ed): Implement!
        let lock = self.lock.get();
        assert_eq!((*lock).load(Ordering::Relaxed), cloudabi::LOCK_UNLOCKED.0);
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
        self.lock = UnsafeCell::new(AtomicU32::new(cloudabi::LOCK_UNLOCKED.0));
        self.recursion = 0;
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
