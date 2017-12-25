extern crate cloudabi;

use io;
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

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let iovs = [cloudabi::iovec { buf: (buf.as_mut_ptr(), buf.len()) }];
        let mut nread: usize = 0;
        let ret = unsafe { cloudabi::fd_read(self.fd, &iovs, &mut nread) };
        if ret != cloudabi::errno::SUCCESS {
            Err(io::Error::from_raw_os_error(ret as i32))
        } else {
            Ok(nread)
        }
    }

    pub fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        let iovs = [cloudabi::iovec { buf: (buf.as_mut_ptr(), buf.len()) }];
        let mut nread: usize = 0;
        let ret = unsafe { cloudabi::fd_pread(self.fd, &iovs, offset, &mut nread) };
        if ret != cloudabi::errno::SUCCESS {
            Err(io::Error::from_raw_os_error(ret as i32))
        } else {
            Ok(nread)
        }
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let iovs = [cloudabi::ciovec { buf: (buf.as_ptr(), buf.len()) }];
        let mut nwritten: usize = 0;
        let ret = unsafe { cloudabi::fd_write(self.fd, &iovs, &mut nwritten) };
        if ret != cloudabi::errno::SUCCESS {
            Err(io::Error::from_raw_os_error(ret as i32))
        } else {
            Ok(nwritten)
        }
    }

    pub fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        let iovs = [cloudabi::ciovec { buf: (buf.as_ptr(), buf.len()) }];
        let mut nwritten: usize = 0;
        let ret = unsafe { cloudabi::fd_pwrite(self.fd, &iovs, offset, &mut nwritten) };
        if ret != cloudabi::errno::SUCCESS {
            Err(io::Error::from_raw_os_error(ret as i32))
        } else {
            Ok(nwritten)
        }
    }
}

impl AsInner<cloudabi::fd> for FileDesc {
    fn as_inner(&self) -> &cloudabi::fd { &self.fd }
}

impl Drop for FileDesc {
    fn drop(&mut self) {
        unsafe { cloudabi::fd_close(self.fd) };
    }
}
