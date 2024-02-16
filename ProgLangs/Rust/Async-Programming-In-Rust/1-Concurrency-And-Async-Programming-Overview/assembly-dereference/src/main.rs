use std::arch::asm;

// notes
// {0} and {1} are templates that tell the compiler that we are referring to registers that
// out(reg) and in(reg( represent
// mov instructs cpu to take first 8 bytes it gets when reading the mem location that {1} points to
// and place it it in reg represented by {0}
// [] tells it to treat the data as memory address

fn main() {
    let t = 100;
    let t_ptr: *const usize = &t; // if you comment out this
    // and uncomment the line below the program will fail
    // let t_ptr = 99999999999999 as *const usize;
    let x = dereference(t_ptr);
    println!("{}", x);
}

fn dereference(ptr: *const usize) -> usize {
    let mut res: usize;
    unsafe { 
        asm!("mov {0}, [{1}]", out(reg) res, in(reg) ptr)
    };
    res
}
