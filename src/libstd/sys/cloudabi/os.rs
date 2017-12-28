use ffi::CStr;
use libc::{self, c_int};
use str;

pub fn errno() -> i32 {
    extern "C" {
        #[thread_local]
        static errno: c_int;
    }

    unsafe { errno as i32 }
}

/// Gets a detailed string description for the given error number.
pub fn error_string(errno: i32) -> String {
    // cloudlibc's strerror() is guaranteed to be thread-safe. There is
    // thus no need to use strerror_r().
    str::from_utf8(unsafe { CStr::from_ptr(libc::strerror(errno)) }.to_bytes())
        .unwrap()
        .to_owned()
}

pub fn page_size() -> usize {
    unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
}

pub fn exit(code: i32) -> ! {
    unsafe { libc::exit(code as c_int) }
}
