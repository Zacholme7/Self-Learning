// Notes
// ------------
// - ask yourself hwo your users will interact with any errors that are returned
//  - look at whether nature of error is likely to affect what the caller does upon receiving it
// - can represent errors via enumeration and erasure
//  - error type enumerate the possible error conditions so caller can distinguish them
//  - provide caller with single opaque error

// - copies bytes from input stream to some output stream
// - want to know if error is in the input or output stream
// - type should implement the std::error::Error trait
// - type should implement Display and Debug
// - if possible, type should implement both send and sync
// - if possible, error type should be 'static
#[derive(Debug)]
pub enum CopyError {
    In(std::io::Error),
    Out(std::io::Error),
}

// implement the display trait
use std::fmt;
impl fmt::Display for CopyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CopyError::In(err) => write!(f, "Input stream error: {}", err),
            CopyError::Out(err) => write!(f, "Output stream error: {}", err),
        }
    }
}

// implement the error trait, this requires display and debug
impl std::error::Error for CopyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CopyError::In(err) | CopyError::Out(err) => Some(err),
        }
    }
}

// Demonstrating how CopyError could be used
fn copy_streams(
    input: &mut dyn std::io::Read,
    output: &mut dyn std::io::Write,
) -> Result<(), CopyError> {
    let mut buffer = [0u8; 1024];
    loop {
        match input.read(&mut buffer) {
            Ok(0) => return Ok(()), // End of input stream
            Ok(n) => {
                if let Err(e) = output.write_all(&buffer[..n]) {
                    return Err(CopyError::Out(e));
                }
            }
            Err(e) => return Err(CopyError::In(e)),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
