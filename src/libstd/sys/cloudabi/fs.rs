extern crate cloudabi;

use io::{self, SeekFrom};
use mem;
use sys::fd::FileDesc;
use sys::time::SystemTime;
use sys_common::AsInner;

#[derive(Debug)]
pub struct File(FileDesc);

#[derive(Clone)]
pub struct FileAttr {
    stat: cloudabi::filestat,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FileType {
    filetype: cloudabi::filetype,
}

impl FileAttr {
    pub fn size(&self) -> u64 { self.stat.st_size as u64 }

    pub fn file_type(&self) -> FileType {
        FileType { filetype: self.stat.st_filetype }
    }

    pub fn modified(&self) -> io::Result<SystemTime> {
        Ok(SystemTime::from(self.stat.st_mtim))
    }

    pub fn accessed(&self) -> io::Result<SystemTime> {
        Ok(SystemTime::from(self.stat.st_atim))
    }
}

impl AsInner<cloudabi::filestat> for FileAttr {
    fn as_inner(&self) -> &cloudabi::filestat { &self.stat }
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        self.filetype == cloudabi::filetype::DIRECTORY
    }

    pub fn is_file(&self) -> bool {
        self.filetype == cloudabi::filetype::REGULAR_FILE
    }

    pub fn is_symlink(&self) -> bool {
        self.filetype == cloudabi::filetype::SYMBOLIC_LINK
    }
}

impl File {
    pub fn file_attr(&self) -> io::Result<FileAttr> {
        let mut stat: cloudabi::filestat;
        let ret = unsafe { cloudabi::file_stat_fget(self.0.raw(), &mut stat) };
        if ret != cloudabi::errno::SUCCESS {
            Err(io::Error::from_raw_os_error(ret as i32))
        } else {
            Ok(FileAttr { stat: stat })
        }
    }

    pub fn fsync(&self) -> io::Result<()> {
        let ret = unsafe { cloudabi::fd_sync(self.0.raw()) };
        if ret != cloudabi::errno::SUCCESS {
            Err(io::Error::from_raw_os_error(ret as i32))
        } else {
            Ok(())
        }
    }

    pub fn datasync(&self) -> io::Result<()> {
        let ret = unsafe { cloudabi::fd_datasync(self.0.raw()) };
        if ret != cloudabi::errno::SUCCESS {
            Err(io::Error::from_raw_os_error(ret as i32))
        } else {
            Ok(())
        }
    }

    pub fn truncate(&self, size: u64) -> io::Result<()> {
        let attr = cloudabi::filestat {
            st_size: size,
            ..mem::zeroed()
        };
        let ret = unsafe { cloudabi::file_stat_fput(self.0.raw(), &attr, cloudabi::fsflags::SIZE) };
        if ret != cloudabi::errno::SUCCESS {
            Err(io::Error::from_raw_os_error(ret as i32))
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
            SeekFrom::Start(off) => (cloudabi::whence::SET, off as i64),
            SeekFrom::End(off) => (cloudabi::whence::END, off),
            SeekFrom::Current(off) => (cloudabi::whence::CUR, off),
        };
        let mut newoffset: cloudabi::filesize = 0;
        let ret = unsafe { cloudabi::fd_seek(self.0.raw(), offset, whence, &mut newoffset) };
        if ret != cloudabi::errno::SUCCESS {
            Err(io::Error::from_raw_os_error(ret as i32))
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
