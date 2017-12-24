extern crate cloudabi;

use io::{self, Error, ErrorKind, SeekFrom};
use mem;
use path::{Path, PathBuf};
use sys::fd::FileDesc;

#[derive(Debug)]
pub struct File(FileDesc);

#[derive(Clone)]
pub struct FileAttr {}

#[derive(Debug)]
pub struct ReadDir {}

pub struct DirEntry {}

#[derive(Clone, Debug)]
pub struct OpenOptions {
    // generic
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FilePermissions {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FileType { mode: u8 }

#[derive(Debug)]
pub struct DirBuilder {}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
            create_new: false,
        }
    }

    pub fn read(&mut self, read: bool) { self.read = read; }
    pub fn write(&mut self, write: bool) { self.write = write; }
    pub fn append(&mut self, append: bool) { self.append = append; }
    pub fn truncate(&mut self, truncate: bool) { self.truncate = truncate; }
    pub fn create(&mut self, create: bool) { self.create = create; }
    pub fn create_new(&mut self, create_new: bool) { self.create_new = create_new; }
}

impl File {
    pub fn file_attr(&self) -> io::Result<FileAttr> {
        let mut stat: cloudabi::filestat;
        let ret = unsafe { cloudabi::file_stat_fget(self.0.raw(), &mut stat) };
        if ret != cloudabi::errno::SUCCESS {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            Ok(())
        }
    }

    pub fn fsync(&self) -> io::Result<()> {
        let ret = unsafe { cloudabi::fd_sync(self.0.raw()) };
        if ret != cloudabi::errno::SUCCESS {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            Ok(())
        }
    }

    pub fn datasync(&self) -> io::Result<()> {
        let ret = unsafe { cloudabi::fd_datasync(self.0.raw()) };
        if ret != cloudabi::errno::SUCCESS {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            Ok(())
        }
    }

    pub fn truncate(&self, size: u64) -> io::Result<()> {
        let attr = cloudabi::filestat {
            st_size: size,
            ..mem::zeroed()
        };
        let ret = unsafe { cloudabi::fd_datasync(self.0.raw(), &attr, cloudabi::fsflags::SIZE) };
        if ret != cloudabi::errno::SUCCESS {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            Ok(())
        }
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }

    pub fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        self.0.read_at(buf, offset)
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    pub fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        self.0.write_at(buf, offset)
    }

    pub fn flush(&self) -> io::Result<()> { Ok(()) }

    pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
        let (whence, offset) = match pos {
            SeekFrom::Start(off) => (cloudabi::whence::SET, off),
            SeekFrom::End(off) => (cloudabi::whence::END, off),
            SeekFrom::Current(off) => (cloudabi::whence::CUR, off),
        };
        let mut newoffset: cloudabi::filesize = 0;
        let ret = unsafe { cloudabi::fd_seek(self.0.raw(), offset, whence, &mut newoffset) };
        if ret != cloudabi::errno::SUCCESS {
            Err(io::Error::from_raw_os_error(ret))
        } else {
            Ok(newoffset)
        }
    }

    pub fn duplicate(&self) -> io::Result<File> {
        self.0.duplicate().map(File)
    }

    pub fn fd(&self) -> &FileDesc { &self.0 }

    pub fn into_fd(self) -> FileDesc { self.0 }
}

pub fn readdir(p: &Path) -> io::Result<ReadDir> {
    Err(Error::new(ErrorKind::Other, "Not implemented"))
}

pub fn unlink(p: &Path) -> io::Result<()> {
    Err(Error::new(ErrorKind::Other, "Not implemented"))
}

pub fn rename(old: &Path, new: &Path) -> io::Result<()> {
    Err(Error::new(ErrorKind::Other, "Not implemented"))
}

pub fn set_perm(p: &Path, perm: FilePermissions) -> io::Result<()> {
    Err(Error::new(ErrorKind::Other, "Not implemented"))
}

pub fn rmdir(p: &Path) -> io::Result<()> {
    Err(Error::new(ErrorKind::Other, "Not implemented"))
}

pub fn remove_dir_all(path: &Path) -> io::Result<()> {
    Err(Error::new(ErrorKind::Other, "Not implemented"))
}

pub fn readlink(p: &Path) -> io::Result<PathBuf> {
    Err(Error::new(ErrorKind::Other, "Not implemented"))
}

pub fn symlink(src: &Path, dst: &Path) -> io::Result<()> {
    Err(Error::new(ErrorKind::Other, "Not implemented"))
}

pub fn link(_src: &Path, _dst: &Path) -> io::Result<()> {
    Err(Error::new(ErrorKind::Other, "Not implemented"))
}

pub fn stat(p: &Path) -> io::Result<FileAttr> {
    Err(Error::new(ErrorKind::Other, "Not implemented"))
}

pub fn lstat(p: &Path) -> io::Result<FileAttr> {
    Err(Error::new(ErrorKind::Other, "Not implemented"))
}

pub fn canonicalize(p: &Path) -> io::Result<PathBuf> {
    Err(Error::new(ErrorKind::Other, "Not implemented"))
}

pub fn copy(from: &Path, to: &Path) -> io::Result<u64> {
    Err(Error::new(ErrorKind::Other, "Not implemented"))
}
