use std::arch::asm;

#[inline(never)] // tells the compiler that we never want this function to be inlined during
                 // optimization (when compiler omits function call and just copis the body of the
                 // function instead of calling it)
fn syscall(message: String) {
    // get raw pointer to message adn the length
    let msg_ptr = message.as_ptr();
    let len = message.len();
    unsafe {
        asm! {
            "mov rax, 1", // puts value 1 in rax, tells kernel we ant to make a write
            "mov rdi, 1", // put 1 in rdi, tell kernel we want to write to stdout
            "syscall", // issues a software interrupt
            in("rsi") msg_ptr, // writes the address to the buffer where out text is stored in rsi
            in("rdx") len, // writes len of text buffer to rdx
            // next four lines tell compiler that it cant store anything in these registers and
            // assume the data is untouched when we exit the inline assembly
            out("rax") _, 
            out("rdi") _,
            lateout("rsi") _,
            lateout("rdx") _,
        };
    }
}

fn main() {
    let message = "hello world from raw syscall\n";
    let message = String::from(message);
    syscall(message);
}
