
// core iterative streaming abstraction is the Iterator
// core async streaming abstraction is Stream

// Read, Write, AsyncRead, AsyncWrite
// backpressure and lazy iteration, research this

// Kinds of streams in typical system
// source: a stream that can produce data
// sink: a stream that can consume data
// through: stream that consumes data, opeartes on it, and then produces new data
// - implemented using either AsyncRead or Stream
// duplex: stream can produce data, and independently can also consume data
// - implemented using AsyncRead + AsyncWrite
// - split to split into reader and writer half

// AsyncRead: Source, Through, Duplex
// AsyncWrite: Sink, Duplex
// Stream: Source, Through


// AsyncRead vs Stream
// - AsyncRead is a byte stream that opeartes on borrowed data
//  - yields unparsed data
// - Stream is object stream that operates on owned data
//  - yields parsed data

// sink requests data from a stream
// - referred to as lazy iteration or streams with backpressure

// Sink: does not bring anything to the table that cannot be impl for elegantly
// with 3 standard stream traits


// AsyncRead/Write have support for vectored IO
// poll_read_vectoried/poll_write_vectorized

fn main() {

}