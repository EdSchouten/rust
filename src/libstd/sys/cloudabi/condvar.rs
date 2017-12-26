extern crate cloudabi;

use mem;
use sync::atomic::{AtomicU32, Ordering};
use sys::mutex::{self, Mutex};
use time::Duration;

extern {
    #[thread_local]
    static __pthread_thread_id: cloudabi::tid;
}

pub struct Condvar {
    condvar: AtomicU32,
}

impl Condvar {
    pub const fn new() -> Condvar {
        Condvar { condvar: AtomicU32::new(cloudabi::CONDVAR_HAS_NO_WAITERS.0) }
    }

    pub unsafe fn init(&mut self) {}

    pub unsafe fn notify_one(&self) {
        if self.condvar.load(Ordering::Relaxed) != cloudabi::CONDVAR_HAS_NO_WAITERS.0 {
            let ret = cloudabi::condvar_signal(
                &mut self.condvar as *mut _ as *mut cloudabi::condvar,
                cloudabi::scope::PRIVATE, 1);
            assert_eq!(ret, cloudabi::errno::SUCCESS);
        }
    }

    pub unsafe fn notify_all(&self) {
        if self.condvar.load(Ordering::Relaxed) != cloudabi::CONDVAR_HAS_NO_WAITERS.0 {
            let ret = cloudabi::condvar_signal(
                &mut self.condvar as *mut _ as *mut cloudabi::condvar,
                cloudabi::scope::PRIVATE, u32::max_value());
            assert_eq!(ret, cloudabi::errno::SUCCESS);
        }
    }

    pub unsafe fn wait(&self, mutex: &Mutex) {
        assert_eq!(mutex::raw(mutex).load(Ordering::Relaxed) & cloudabi::LOCK_KERNEL_MANAGED.0,
                   __pthread_thread_id.0 | cloudabi::LOCK_WRLOCKED.0);

        let subscription = cloudabi::subscription {
            type_: cloudabi::eventtype::CONDVAR,
            union: cloudabi::subscription_union {
                condvar: cloudabi::subscription_condvar {
                    condvar: &mut self.condvar as *mut _ as *mut cloudabi::condvar,
                    condvar_scope: cloudabi::scope::PRIVATE,
                    lock: mutex::raw(mutex) as *mut _ as *mut cloudabi::lock,
                    lock_scope: cloudabi::scope::PRIVATE,
                }
            },
            ..mem::zeroed()
        };
        // TODO(ed): Implement!
    }

    pub unsafe fn wait_timeout(&self, mutex: &Mutex, dur: Duration) -> bool {
        // TODO(ed): Implement!
        false
    }

    pub unsafe fn destroy(&self) {
        assert_eq!(self.condvar.load(Ordering::Relaxed),
                   cloudabi::CONDVAR_HAS_NO_WAITERS.0);
    }
}
