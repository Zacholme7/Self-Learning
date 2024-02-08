#[repr(C)]
struct Foo {
    tiny: bool, // sees tiny field whose size is 1 bit, but given 1 byte
    normal: u32, // 4 bytes but tiny is 1 which gives us misalignment, compiler pads with 3 bytes
                 // to make tiny + normal 8 bytes
    small: u8, // 1 byte and aligned so it is okay
    long: u64, // not 8 byte aligned, so have to add 7 bytes to make long be aligned
    short: u16, // just need 2 byte alignmetn here
} // align foo, which will follow the biggest alignment (8)
// total is 32 bytes bytes
// 1 (tiny) + 3 (padding) + 4 (u32) + 1(small) + 7 (padding) + 8 (long) + 2 (short) + 6 (padding)


fn main() {
    println!("Hello, world!");
}
