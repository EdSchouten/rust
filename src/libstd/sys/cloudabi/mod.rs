extern crate cloudabi;

use io;
use libc;
use mem;

pub mod cmath;
pub mod condvar;
pub mod memchr;
pub mod mutex;
pub mod os;
pub mod os_str;
pub mod rwlock;
pub mod stack_overflow;
pub mod stdio;
pub mod thread;
pub mod thread_local;
pub mod time;

pub fn init() {}

pub fn decode_error_kind(errno: i32) -> io::ErrorKind {
    match errno {
        x if x == cloudabi::errno::AGAIN as i32 => io::ErrorKind::WouldBlock,
        x if x == cloudabi::errno::CONNREFUSED as i32 => io::ErrorKind::ConnectionRefused,
        x if x == cloudabi::errno::CONNRESET as i32 => io::ErrorKind::ConnectionReset,
        x if x == cloudabi::errno::PERM as i32 || x == cloudabi::errno::ACCES as i32 => {
            io::ErrorKind::PermissionDenied
        }
        x if x == cloudabi::errno::PIPE as i32 => io::ErrorKind::BrokenPipe,
        x if x == cloudabi::errno::NOTCONN as i32 => io::ErrorKind::NotConnected,
        x if x == cloudabi::errno::CONNABORTED as i32 => io::ErrorKind::ConnectionAborted,
        x if x == cloudabi::errno::ADDRNOTAVAIL as i32 => io::ErrorKind::AddrNotAvailable,
        x if x == cloudabi::errno::ADDRINUSE as i32 => io::ErrorKind::AddrInUse,
        x if x == cloudabi::errno::NOENT as i32 => io::ErrorKind::NotFound,
        x if x == cloudabi::errno::INTR as i32 => io::ErrorKind::Interrupted,
        x if x == cloudabi::errno::INVAL as i32 => io::ErrorKind::InvalidInput,
        x if x == cloudabi::errno::TIMEDOUT as i32 => io::ErrorKind::TimedOut,
        x if x == cloudabi::errno::EXIST as i32 => io::ErrorKind::AlreadyExists,
        _ => io::ErrorKind::Other,
    }
}

pub unsafe fn abort_internal() -> ! {
    ::core::intrinsics::abort();
}

pub use libc::strlen;

pub fn hashmap_random_keys() -> (u64, u64) {
    unsafe {
        let mut v = mem::uninitialized();
        libc::arc4random_buf(&mut v as *mut _ as *mut libc::c_void, mem::size_of_val(&v));
        v
    }
}
