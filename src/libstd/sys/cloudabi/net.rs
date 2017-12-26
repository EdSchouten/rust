extern crate cloudabi;

use ffi::CStr;
use io;
use libc::{self, c_int, size_t, EAI_SYSTEM};
use net::Shutdown;
use str;
use sys::fd::FileDesc;
use sys_common::{AsInner, FromInner, IntoInner};

pub use sys::{cvt, cvt_r};
pub extern crate libc as netc;

pub type wrlen_t = size_t;

pub struct Socket(FileDesc);

impl AsInner<c_int> for Socket {
    fn as_inner(&self) -> &c_int { self.0.as_inner() }
}

impl FromInner<c_int> for Socket {
    fn from_inner(fd: c_int) -> Socket { Socket(FileDesc::new(fd)) }
}

impl IntoInner<c_int> for Socket {
    fn into_inner(self) -> c_int { self.0.into_raw() }
}

pub fn init() {}

pub fn cvt_gai(err: c_int) -> io::Result<()> {
    if err == 0 {
        return Ok(())
    }
    if err == EAI_SYSTEM {
        return Err(io::Error::last_os_error())
    }

    let detail = unsafe {
        str::from_utf8(CStr::from_ptr(libc::gai_strerror(err)).to_bytes()).unwrap()
            .to_owned()
    };
    Err(io::Error::new(io::ErrorKind::Other,
                       &format!("failed to lookup address information: {}",
                                detail)[..]))
}

impl Socket {
    pub fn new_pair(_: c_int, _: c_int) -> io::Result<(Socket, Socket)> {
        // TODO(ed): Implement!
        Err(io::Error::new(io::ErrorKind::Other, "Unimplemented!"))
    }

    pub fn duplicate(&self) -> io::Result<Socket> {
        self.0.duplicate().map(Socket)
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        // TODO(ed): Implement!
        Err(io::Error::new(io::ErrorKind::Other, "Unimplemented!"))
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    pub fn shutdown(&self, _: Shutdown) -> io::Result<()> {
        // TODO(ed): Implement!
        Err(io::Error::new(io::ErrorKind::Other, "Unimplemented!"))
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        // TODO(ed): Implement!
        Err(io::Error::new(io::ErrorKind::Other, "Unimplemented!"))
    }
}

pub fn res_init_if_glibc_before_2_26() -> io::Result<()> {
    Ok(())
}
