use crate::ffi;
use std::{
    io::{self, Result},
    net::TcpStream,
    os::fd::AsRawFd,
};

type Events = Vec<ffi::Event>;

pub struct Poll {
    registry: Registry,
}

/// Represents the event queue itself
impl Poll {
    /// Creates a new event queue
    pub fn new() -> Result<Self> {
        // create a new epoll event queue
        let res = unsafe { ffi::epoll_create(1) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            registry: Registry { raw_fd: res },
        })
    }

    /// Get a reference to the registry. The registry can be used to register interest
    /// to be notified about new events
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    /// Blocks the thead until and event is ready or times out
    pub fn poll(&mut self, events: &mut Events, timeout: Option<i32>) -> Result<()> {
        // get the raw fd for the queue
        let fd = self.registry.raw_fd;
        // if no timeout set, set it to -1 which tells os we want to block until event occurs even
        // though that might never happen
        let timeout = timeout.unwrap_or(-1);
        // get capacity so we know how much space is alocated
        let max_events = events.capacity() as i32;

        // res will tell us how many events have occured
        let res = unsafe {ffi::epoll_wait(fd, events.as_mut_ptr(), max_events, timeout) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }
        // set events to the number of events that have occured
        unsafe { events.set_len(res as usize) };
        Ok(())

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
        // create event objct
        let mut event = ffi::Event {
            events: interests as u32,
            epoll_data: token,
        };
        // register this fd
        let op = ffi::EPOLL_CTL_ADD;
        let res = unsafe {
            ffi::epoll_ctl(self.raw_fd, op, source.as_raw_fd(), &mut event)
        };

        if res < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        // just close the fd
        let res = unsafe {ffi::close(self.raw_fd)};
        if res < 0 {
            let err = io::Error::last_os_error();
            eprintln!("Error: {err:?}");
        }
    }
}


































