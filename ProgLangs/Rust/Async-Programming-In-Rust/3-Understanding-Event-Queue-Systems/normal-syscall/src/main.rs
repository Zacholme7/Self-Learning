use std::io;
// crate pretty much does this for us. https://github.com/rust-lang/libc

#[cfg(target_family = "unix")]
#[link(name = "c")] // tells compiler to link to the c library on the system
extern "C" { // say we want to use the c calling convention, using rust ffi (foreign function) interface
    fn write(fd: u32, buf: *const u8, count: usize) -> i32;
}

// wrap the write function in a normal rust function
fn syscall(message: String) -> io::Result<()> {
    let msg_ptr = message.as_ptr();
    let len = message.len();
    let res = unsafe { write(1, msg_ptr, len) }; // unsafe block since external function
    if res == -1 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

fn main() {
    let message = "hello world from syscall\n";
    let message = String::from(message);
    syscall(message).unwrap();
}
