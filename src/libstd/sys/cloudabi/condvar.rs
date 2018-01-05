// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
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
use sys::mutex::{self, Mutex};
use sys::time::dur2intervals;
use time::Duration;

extern "C" {
    #[thread_local]
    static __pthread_thread_id: cloudabi::tid;
}

pub struct Condvar {
    condvar: UnsafeCell<AtomicU32>,
}

unsafe impl Send for Condvar {}
unsafe impl Sync for Condvar {}

impl Condvar {
    pub const fn new() -> Condvar {
        Condvar {
            condvar: UnsafeCell::new(AtomicU32::new(cloudabi::CONDVAR_HAS_NO_WAITERS.0)),
        }
    }

    pub unsafe fn init(&mut self) {}

    pub unsafe fn notify_one(&self) {
        let condvar = self.condvar.get();
        if (*condvar).load(Ordering::Relaxed) != cloudabi::CONDVAR_HAS_NO_WAITERS.0 {
            let ret = cloudabi::condvar_signal(
                condvar as *mut cloudabi::condvar,
                cloudabi::scope::PRIVATE,
                1,
            );
            assert_eq!(
                ret,
                cloudabi::errno::SUCCESS,
                "Failed to signal on condition variable"
            );
        }
    }

    pub unsafe fn notify_all(&self) {
        let condvar = self.condvar.get();
        if (*condvar).load(Ordering::Relaxed) != cloudabi::CONDVAR_HAS_NO_WAITERS.0 {
            let ret = cloudabi::condvar_signal(
                condvar as *mut cloudabi::condvar,
                cloudabi::scope::PRIVATE,
                cloudabi::nthreads::max_value(),
            );
            assert_eq!(
                ret,
                cloudabi::errno::SUCCESS,
                "Failed to broadcast on condition variable"
            );
        }
    }

    pub unsafe fn wait(&self, mutex: &Mutex) {
        let mutex = mutex::raw(mutex);
        assert_eq!(
            (*mutex).load(Ordering::Relaxed) & !cloudabi::LOCK_KERNEL_MANAGED.0,
            __pthread_thread_id.0 | cloudabi::LOCK_WRLOCKED.0,
            "This lock is not write-locked by this thread"
        );

        // Call into the kernel to wait on the condition variable.
        let condvar = self.condvar.get();
        let subscription = cloudabi::subscription {
            type_: cloudabi::eventtype::CONDVAR,
            union: cloudabi::subscription_union {
                condvar: cloudabi::subscription_condvar {
                    condvar: condvar as *mut cloudabi::condvar,
                    condvar_scope: cloudabi::scope::PRIVATE,
                    lock: mutex as *mut cloudabi::lock,
                    lock_scope: cloudabi::scope::PRIVATE,
                },
            },
            ..mem::zeroed()
        };
        let mut event: cloudabi::event = mem::uninitialized();
        let mut nevents: usize = mem::uninitialized();
        let ret = cloudabi::poll(&subscription, &mut event, 1, &mut nevents);
        assert_eq!(
            ret,
            cloudabi::errno::SUCCESS,
            "Failed to wait on condition variable"
        );
        assert_eq!(
            event.error,
            cloudabi::errno::SUCCESS,
            "Failed to wait on condition variable"
        );
    }

    pub unsafe fn wait_timeout(&self, mutex: &Mutex, dur: Duration) -> bool {
        let mutex = mutex::raw(mutex);
        assert_eq!(
            (*mutex).load(Ordering::Relaxed) & !cloudabi::LOCK_KERNEL_MANAGED.0,
            __pthread_thread_id.0 | cloudabi::LOCK_WRLOCKED.0,
            "This lock is not write-locked by this thread"
        );

        // Call into the kernel to wait on the condition variable.
        let condvar = self.condvar.get();
        let subscriptions = [
            cloudabi::subscription {
                type_: cloudabi::eventtype::CONDVAR,
                union: cloudabi::subscription_union {
                    condvar: cloudabi::subscription_condvar {
                        condvar: condvar as *mut cloudabi::condvar,
                        condvar_scope: cloudabi::scope::PRIVATE,
                        lock: mutex as *mut cloudabi::lock,
                        lock_scope: cloudabi::scope::PRIVATE,
                    },
                },
                ..mem::zeroed()
            },
            cloudabi::subscription {
                type_: cloudabi::eventtype::CLOCK,
                union: cloudabi::subscription_union {
                    clock: cloudabi::subscription_clock {
                        clock_id: cloudabi::clockid::MONOTONIC,
                        timeout: dur2intervals(&dur),
                        ..mem::zeroed()
                    },
                },
                ..mem::zeroed()
            },
        ];
        let mut events: [cloudabi::event; 2] = mem::uninitialized();
        let mut nevents: usize = mem::uninitialized();
        let ret = cloudabi::poll(subscriptions.as_ptr(), events.as_mut_ptr(), 2, &mut nevents);
        assert_eq!(
            ret,
            cloudabi::errno::SUCCESS,
            "Failed to wait on condition variable"
        );
        for i in 0..nevents {
            assert_eq!(
                events[i].error,
                cloudabi::errno::SUCCESS,
                "Failed to wait on condition variable"
            );
            if events[i].type_ == cloudabi::eventtype::CONDVAR {
                return true;
            }
        }
        false
    }

    pub unsafe fn destroy(&self) {
        let condvar = self.condvar.get();
        assert_eq!(
            (*condvar).load(Ordering::Relaxed),
            cloudabi::CONDVAR_HAS_NO_WAITERS.0,
            "Attempted to destroy a condition variable with blocked threads"
        );
    }
}
