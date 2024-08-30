

// future needs to be pin
// wrap self with pin to guarantee it is not going to move
// future has start for every point, start, every await, and finish

// goal if pin is to make it safe to manipulate self referential types generated
// by the compiler from an asyn cfunciton or implemented with unsafe code in a runtime

// pinned typestate
// once an object is pinned, it can never be moved again

// Pin is a wrapper type that can wrap any kind of pointer 
// - pointer put into pinned typestate so it must never be moved again

// Safe mutation
// - Pin<&mut T> allows safe mutation if T:Unpin
// - unpin is an auto trait, allows getting mutable ref from pinned pointer w/o unsafe code
// if type cant be self referential

// Unsafe mutation
// -= when T: !Unpin need to use unsafe code 

// guarantee that value will not be moved out of memory location and
// remaid valid at the same memory location

// allows us to put value which exists at some locaiton in mem into state where safe
// code cannot move that value to different location in mem

// must use unsafe to interact with value in any way that many potentially violate guarantees


// what is moving
// - compiler copies, byte for byte, value from one location to another
// - same as a copy, but carries semantics of ownership
// - all values are trivially moveable, compiler has freedom to move them around in 
// memory as it sees fir for optimizaitons. doesnt change semantics but changes 
// where it is physically stored

// compiler is free to move around memory but must preserve the semantics of the program

// so, pinning means that it has been put in state where it is guaranteed to remain in the same
// place in memory from the time it is pinned until its drop is called

// - pin allows the impl of safe interfaces around types which have some state during which they
// become address sensitive

fn main() {

}