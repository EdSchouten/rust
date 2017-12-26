extern crate cloudabi;

use io;
use libc::c_int;
use mem;
use sys_common::AsInner;

#[derive(Debug)]
pub struct FileDesc {
    fd: c_int,
}

impl FileDesc {
    pub fn new(fd: c_int) -> FileDesc {
        FileDesc { fd: fd }
    }

    pub fn raw(&self) -> c_int { self.fd }

    /// Extracts the actual filedescriptor without closing it.
    pub fn into_raw(self) -> c_int {
        let fd = self.fd;
        mem::forget(self);
        fd
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let iovs = [cloudabi::iovec {
            buf: (buf.as_mut_ptr() as *mut _ as *mut (), buf.len())
        }];
        let mut nread: usize = 0;
        let ret = unsafe { cloudabi::fd_read(cloudabi::fd(self.fd as u32), &iovs, &mut nread) };
        if ret != cloudabi::errno::SUCCESS {
            Err(io::Error::from_raw_os_error(ret as i32))
        } else {
            Ok(nread)
        }
    }

    pub fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        let iovs = [cloudabi::iovec {
            buf: (buf.as_mut_ptr() as *mut _ as *mut (), buf.len())
        }];
        let mut nread: usize = 0;
        let ret = unsafe { cloudabi::fd_pread(cloudabi::fd(self.fd as u32), &iovs, offset, &mut nread) };
        if ret != cloudabi::errno::SUCCESS {
            Err(io::Error::from_raw_os_error(ret as i32))
        } else {
            Ok(nread)
        }
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let iovs = [cloudabi::ciovec {
            buf: (buf.as_ptr() as *const _ as *const (), buf.len())
        }];
        let mut nwritten: usize = 0;
        let ret = unsafe { cloudabi::fd_write(cloudabi::fd(self.fd as u32), &iovs, &mut nwritten) };
        if ret != cloudabi::errno::SUCCESS {
            Err(io::Error::from_raw_os_error(ret as i32))
        } else {
            Ok(nwritten)
        }
    }

    pub fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        let iovs = [cloudabi::ciovec {
            buf: (buf.as_ptr() as *const _ as *const (), buf.len())
        }];
        let mut nwritten: usize = 0;
        let ret = unsafe { cloudabi::fd_pwrite(cloudabi::fd(self.fd as u32), &iovs, offset, &mut nwritten) };
        if ret != cloudabi::errno::SUCCESS {
            Err(io::Error::from_raw_os_error(ret as i32))
        } else {
            Ok(nwritten)
        }
    }

    pub fn duplicate(&self) -> io::Result<FileDesc> {
        unsafe {
            let mut fd: cloudabi::fd = mem::uninitialized();
            let ret = cloudabi::fd_dup(cloudabi::fd(self.fd as u32), &mut fd);
            if ret != cloudabi::errno::SUCCESS {
                Err(io::Error::from_raw_os_error(ret as i32))
            } else {
                Ok(FileDesc::new(fd.0 as c_int))
            }
        }
    }
}

impl AsInner<c_int> for FileDesc {
    fn as_inner(&self) -> &c_int { &self.fd }
}

impl Drop for FileDesc {
    fn drop(&mut self) {
        unsafe { cloudabi::fd_close(cloudabi::fd(self.fd as u32)) };
    }
}
