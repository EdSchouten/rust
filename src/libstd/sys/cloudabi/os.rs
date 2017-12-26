use os::unix::prelude::*;

use ffi::{CStr, CString, OsStr, OsString};
use io;
use libc::{self, c_char, c_int};
use marker::PhantomData;
use memchr;
use ptr;
use vec;

pub fn errno() -> i32 {
    extern "C" {
        #[thread_local]
        static errno: c_int;
    }

    unsafe { errno as i32 }
}

/// Gets a detailed string description for the given error number.
pub fn error_string(_: i32) -> String {
    // TODO(ed): Implement!
    return String::from("TODO");
}

pub struct Env {
    iter: vec::IntoIter<(OsString, OsString)>,
    _dont_send_or_sync_me: PhantomData<*mut ()>,
}

impl Iterator for Env {
    type Item = (OsString, OsString);
    fn next(&mut self) -> Option<(OsString, OsString)> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub unsafe fn environ() -> *mut *const *const c_char {
    extern "C" {
        static mut environ: *const *const c_char;
    }
    &mut environ
}

/// Returns a vector of (variable, value) byte-vector pairs for all the
/// environment variables of the current process.
pub fn env() -> Env {
    unsafe {
        let mut environ = *environ();
        if environ == ptr::null() {
            panic!(
                "os::env() failure getting env string from OS: {}",
                io::Error::last_os_error()
            );
        }
        let mut result = Vec::new();
        while *environ != ptr::null() {
            if let Some(key_value) = parse(CStr::from_ptr(*environ).to_bytes()) {
                result.push(key_value);
            }
            environ = environ.offset(1);
        }
        let ret = Env {
            iter: result.into_iter(),
            _dont_send_or_sync_me: PhantomData,
        };
        return ret;
    }

    fn parse(input: &[u8]) -> Option<(OsString, OsString)> {
        // Strategy (copied from glibc): Variable name and value are separated
        // by an ASCII equals sign '='. Since a variable name must not be
        // empty, allow variable names starting with an equals sign. Skip all
        // malformed lines.
        if input.is_empty() {
            return None;
        }
        let pos = memchr::memchr(b'=', &input[1..]).map(|p| p + 1);
        pos.map(|p| {
            (
                OsStringExt::from_vec(input[..p].to_vec()),
                OsStringExt::from_vec(input[p + 1..].to_vec()),
            )
        })
    }
}

pub fn getenv(k: &OsStr) -> io::Result<Option<OsString>> {
    let k = CString::new(k.as_bytes())?;
    unsafe {
        let s = libc::getenv(k.as_ptr());
        let ret = if s.is_null() {
            None
        } else {
            Some(OsStringExt::from_vec(CStr::from_ptr(s).to_bytes().to_vec()))
        };
        return Ok(ret);
    }
}

pub fn page_size() -> usize {
    unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
}

pub fn exit(code: i32) -> ! {
    unsafe { libc::exit(code as c_int) }
}
