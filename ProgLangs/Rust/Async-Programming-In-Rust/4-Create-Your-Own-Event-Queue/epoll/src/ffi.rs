// - contains mappings to syscalls and datastructures we need to communicate with the operating
// system

pub const EPOLL_CTL_ADD: i32 = 1;
pub const EPOLLIN: i32 = 0x1; // interested in read operations on the file handle
pub const EPOLLET: i32 = 1 << 31; // interested in getting events notified with epoll set to an
                                  // edge triggered mode

#[link(name = "c")]
extern "C" {
    // syscall to create epoll queue, size is there for historical reasons and will be ignored
    pub fn epoll_create(size: i32) -> i32;
    // syscall to close the file descriptor from epoll instance
    pub fn close(fd: i32) -> i32;
    // control interface used to perform operations on our epoll instance
    // call this to register interest in events on a source
    // add, modify, delete
    pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut Event) -> i32;
    // block the current thread and wait to recieve a notification that an event has occured or it
    // times out
    pub fn epoll_wait(epfd: i32, events: *mut Event, maxevents: i32, timeout: i32) -> i32;
}

// event used to communicate with the operating system in epoll_ctl
// system expects packed Event Struct, hence the packed
#[derive(Debug)]
#[repr(C, packed)]
pub struct Event {
    pub(crate) events: u32,
    // token to identify event
    pub(crate) epoll_data: usize,
}

impl Event {
    pub fn token(&self) -> usize {
        self.epoll_data
    }
}
