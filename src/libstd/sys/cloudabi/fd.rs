extern crate cloudabi;

use mem;
use sys_common::AsInner;

#[derive(Debug)]
pub struct FileDesc {
    fd: cloudabi::fd,
}

impl FileDesc {
    pub fn new(fd: cloudabi::fd) -> FileDesc {
        FileDesc { fd: fd }
    }

    pub fn raw(&self) -> cloudabi::fd { self.fd }

    /// Extracts the actual filedescriptor without closing it.
    pub fn into_raw(self) -> cloudabi::fd {
        let fd = self.fd;
        mem::forget(self);
        fd
    }
}

impl AsInner<cloudabi::fd> for FileDesc {
    fn as_inner(&self) -> &cloudabi::fd { &self.fd }
}
