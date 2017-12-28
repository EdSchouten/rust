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
    match errno as libc::c_int {
        libc::ECONNREFUSED => io::ErrorKind::ConnectionRefused,
        libc::ECONNRESET => io::ErrorKind::ConnectionReset,
        libc::EPERM | libc::EACCES => io::ErrorKind::PermissionDenied,
        libc::EPIPE => io::ErrorKind::BrokenPipe,
        libc::ENOTCONN => io::ErrorKind::NotConnected,
        libc::ECONNABORTED => io::ErrorKind::ConnectionAborted,
        libc::EADDRNOTAVAIL => io::ErrorKind::AddrNotAvailable,
        libc::EADDRINUSE => io::ErrorKind::AddrInUse,
        libc::ENOENT => io::ErrorKind::NotFound,
        libc::EINTR => io::ErrorKind::Interrupted,
        libc::EINVAL => io::ErrorKind::InvalidInput,
        libc::ETIMEDOUT => io::ErrorKind::TimedOut,
        libc::EEXIST => io::ErrorKind::AlreadyExists,

        // These two constants can have the same value on some systems,
        // but different values on others, so we can't use a match
        // clause
        x if x == libc::EAGAIN || x == libc::EWOULDBLOCK => io::ErrorKind::WouldBlock,

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
