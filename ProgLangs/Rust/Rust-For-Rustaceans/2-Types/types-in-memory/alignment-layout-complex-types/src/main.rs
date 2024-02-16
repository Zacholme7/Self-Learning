// Notes: Alignment, Layout, Complex types
// ------------------
// - types tell you how to interpret bits in memory
// - alignment dictates where the bytes for a type can be stored
//  - ex: pointers point to bytes, not bits
// - on cpu, memory often accessed in block larger than a single byte (cpu word size)
// - operations on data that is not aligned is called misaligned accesses, can lead to poor performance
// - cpu prefer/require argument that are naturally aligned (alignment matches its size)
// - complex types are assigned the largest alignment of all the types they contain
// - rust provides repr() attribute to request particular memory representation
//   - repr(c) gives predictable c layout, good when working with unsafe raw pointers
//   - repr(transparent): used only on types with single field and guarantees that layout of outer type is exactly the same as the inner type
// - c repr requires we place all fiields in same order they appear in original struct def
//   - repr(Rust) removes this
// - place fields in decreasing size so smaller ones are used as padding
// - use #[repr(packed)] to say you are fine with misalignment, need every bytes
//   - may lead to much slower code and can even cause crash if instr needs alignment
// - use #[repr(align(n))] if you want to give field or type larger alignment than it requires
//   - can be used to make values end up in diff cache lines, avoid false sharing
// - tuple: represented like a struct with fields of the same type as the tuple values in the same order
// - array: represented as contiguous seq of the contained type w/ no padding
// - union: layout chosen ind for each variant, alignment is max across all variants
// - enumeration: same as union w/ field additional field that stores enum variant discrimant
//   - value code uses to determine which of the the enum variants a given value holds

#[repr(C)]
struct Foo {
    tiny: bool,  // sees tiny field whose size is 1 bit, but given 1 byte
    normal: u32, // 4 bytes but tiny is 1 which gives us misalignment, compiler pads with 3 bytes
    // to make tiny + normal 8 bytes
    small: u8,  // 1 byte and aligned so it is okay
    long: u64,  // not 8 byte aligned, so have to add 7 bytes to make long be aligned
    short: u16, // just need 2 byte alignmetn here
} // align foo, which will follow the biggest alignment (8)
  // total is 32 bytes bytes
  // 1 (tiny) + 3 (padding) + 4 (u32) + 1(small) + 7 (padding) + 8 (long) + 2 (short) + 6 (padding)

fn main() {
    println!("Hello, world!");
}
