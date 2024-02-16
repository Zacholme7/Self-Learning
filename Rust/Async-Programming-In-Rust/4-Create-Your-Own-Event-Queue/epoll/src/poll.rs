use std::{io::{self, Result}, net::TcpStream, os::fd::AsRawFd};
use crate::ffi;

type Events = Vec<ffi::Event>;

pub struct Poll {
    registry: Registry,
}

/// Represents the event queue itself
impl Poll {
    /// Creates a new event queue
    pub fn new() -> Result<Self> {
        todo!()
    }

    /// Get a reference to the registry. The registry can be used to register interest
    /// to be notified about new events
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    /// Blocks the thead until and event is ready or times out
    pub fn poll(&mut self, events: &mut Events, timeout: Option<i32>) -> Result<()> {
        todo!()
    }
}

/// Handle that allows the register interest in new events
/// Why do we need registry? Need to remember that mio abstracts over epoll, kqueue, and IOCP
/// In mio, registry wraps around a Selector where each platform has their own selector
/// implementation corresponding to relevant syscalled
pub struct Registry {
    raw_fd: i32,
}

impl Registry {
    pub fn register(&self, source: &TcpStream, token: usize, interests: i32) -> Result<()> {
        todo!()
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        todo!()
    }
}





