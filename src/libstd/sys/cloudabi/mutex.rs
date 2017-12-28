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
use sys::rwlock::{self, RWLock};
use sync::atomic::{AtomicU32, Ordering};

extern "C" {
    #[thread_local]
    static __pthread_thread_id: cloudabi::tid;
}

// Implement Mutex using an RWLock. This doesn't introduce any
// performance overhead in this environment, as the operations would be
// implemented identically.
pub struct Mutex(RWLock);

pub unsafe fn raw(m: &Mutex) -> *mut AtomicU32 {
    rwlock::raw(&m.0)
}

impl Mutex {
    pub const fn new() -> Mutex {
        Mutex(RWLock::new())
    }

    pub unsafe fn init(&mut self) {
        // This function should normally reinitialize the mutex after
        // moving it to a different memory address. This implementation
        // does not require adjustments after moving.
    }

    pub unsafe fn try_lock(&self) -> bool {
        self.0.try_write()
    }

    pub unsafe fn lock(&self) {
        self.0.write()
    }

    pub unsafe fn unlock(&self) {
        self.0.write_unlock()
    }

    pub unsafe fn destroy(&self) {
        self.0.destroy()
    }
}

pub struct ReentrantMutex {
    lock: UnsafeCell<AtomicU32>,
    recursion: UnsafeCell<u32>,
}

impl ReentrantMutex {
    pub unsafe fn uninitialized() -> ReentrantMutex {
        mem::uninitialized()
    }

    pub unsafe fn init(&mut self) {
        self.lock = UnsafeCell::new(AtomicU32::new(cloudabi::LOCK_UNLOCKED.0));
        self.recursion = UnsafeCell::new(0);
    }

    pub unsafe fn try_lock(&self) -> bool {
        // Attempt to acquire the lock.
        let lock = self.lock.get();
        let recursion = self.recursion.get();
        if let Err(old) = (*lock).compare_exchange(
            cloudabi::LOCK_UNLOCKED.0,
            __pthread_thread_id.0 | cloudabi::LOCK_WRLOCKED.0,
            Ordering::Acquire,
            Ordering::Relaxed,
        ) {
            // If we fail to acquire the lock, it may be the case
            // that we've already acquired it and may need to recurse.
            if old & !cloudabi::LOCK_KERNEL_MANAGED.0
                == __pthread_thread_id.0 | cloudabi::LOCK_WRLOCKED.0
            {
                *recursion += 1;
                true
            } else {
                false
            }
        } else {
            // Success.
            assert_eq!(*recursion, 0, "Mutex has invalid recursion count");
            true
        }
    }

    pub unsafe fn lock(&self) {
        if !self.try_lock() {
            // Call into the kernel to acquire a write lock.
            let lock = self.lock.get();
            let subscription = cloudabi::subscription {
                type_: cloudabi::eventtype::LOCK_WRLOCK,
                union: cloudabi::subscription_union {
                    lock: cloudabi::subscription_lock {
                        lock: lock as *mut cloudabi::lock,
                        lock_scope: cloudabi::scope::PRIVATE,
                    },
                },
                ..mem::zeroed()
            };
            let mut event: cloudabi::event = mem::uninitialized();
            let mut nevents: usize = mem::uninitialized();
            let ret = cloudabi::poll(&subscription, &mut event, 1, &mut nevents);
            assert_eq!(ret, cloudabi::errno::SUCCESS, "Failed to acquire mutex");
            assert_eq!(
                event.error,
                cloudabi::errno::SUCCESS,
                "Failed to acquire mutex"
            );
        }
    }

    pub unsafe fn unlock(&self) {
        let lock = self.lock.get();
        let recursion = self.recursion.get();
        assert_eq!(
            (*lock).load(Ordering::Relaxed) & !cloudabi::LOCK_KERNEL_MANAGED.0,
            __pthread_thread_id.0 | cloudabi::LOCK_WRLOCKED.0,
            "This mutex is locked by a different thread"
        );

        if *recursion > 0 {
            *recursion -= 1;
        } else if !(*lock)
            .compare_exchange(
                __pthread_thread_id.0 | cloudabi::LOCK_WRLOCKED.0,
                cloudabi::LOCK_UNLOCKED.0,
                Ordering::Release,
                Ordering::Relaxed,
            )
            .is_ok()
        {
            // Lock is managed by kernelspace. Call into the kernel
            // to unblock waiting threads.
            let ret = cloudabi::lock_unlock(lock as *mut cloudabi::lock, cloudabi::scope::PRIVATE);
            assert_eq!(ret, cloudabi::errno::SUCCESS, "Failed to unlock a mutex");
        }
    }

    pub unsafe fn destroy(&self) {
        let lock = self.lock.get();
        let recursion = self.recursion.get();
        assert_eq!(
            (*lock).load(Ordering::Relaxed),
            cloudabi::LOCK_UNLOCKED.0,
            "Attempted to destroy locked mutex"
        );
        assert_eq!(*recursion, 0, "Recursion counter invalid");
    }
}
