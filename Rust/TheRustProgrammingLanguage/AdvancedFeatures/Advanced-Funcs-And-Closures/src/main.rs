fn main() {
    // can pass functions to functions
    // functions coerce to type fn, which is a function pointer
    // fn is a type rather than a closure
    // fn implements Fn, FnMut, and FnOnce
    // 1
    let answer = do_twice(add_one, 5);
}


// 1
fn add_one(x: i32) -> i32 {
    x + 1
}

// function pointer in the argument 
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

