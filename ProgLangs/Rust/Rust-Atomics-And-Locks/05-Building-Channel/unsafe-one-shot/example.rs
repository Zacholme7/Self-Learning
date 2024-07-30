use unsafe_one_shot::Channel;


fn main() {
    let channel : Channel<usize>= Channel::new();
    unsafe { channel.send(10); }

}