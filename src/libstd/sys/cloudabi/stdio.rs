use io;

pub struct Stderr(());

impl Stderr {
    pub fn new() -> io::Result<Stderr> { Ok(Stderr(())) }

    pub fn write(&self, data: &[u8]) -> io::Result<usize> {
        Ok(data.len())
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }
}

pub fn is_ebadf(err: &io::Error) -> bool {
    // TODO(ed): Implement!
    false
}
