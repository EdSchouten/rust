extern crate cloudabi;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Instant {
    t: u64
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct SystemTime {
    t: cloudabi::timestamp,
}

impl From<cloudabi::timestamp> for SystemTime {
    fn from(t: cloudabi::timestamp) -> SystemTime { SystemTime { t: t } }
}

pub const UNIX_EPOCH: SystemTime = SystemTime {
    t: 0,
};
