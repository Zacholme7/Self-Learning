type Document = Vec<String>;
fn main() {
    // most programming languages use garbage collection
    // scan through memory and find data that is no longer used
    // drawbacks are performance, overheads, and unpredictability

    // ownership at runtime
    // local vars allocated in stack frame
    // local vars can hold data or pointers
    // pointers created through boxes (owning pointers) or references (non owning pointers)
    let mut a_num = 0;
    inner(&mut a_num);

    // ownership at compile time
    // tracks read, write, and ownership permissions on each variable
    // must have appropriate permissions to perform an operation

    // ex: if variable not declared let mut, it does not have write and cannot be mutated
    let n = 0; // n: r/o
    // n += 1; does not have write permission so we cannot mutate

    // permissions are changed if a variable is moved or borrowed
    // move of var with non copyable type requres read and ownership and the move elims permissions on that variable
    let s = String::from("hello world"); // s: r/o
    //consume_string(s); s is moved into the function and loses all permissions
    // println!("{}", s); error, s was moved and no longer have access to it

    // borrowing a variable temporarily removes some of the permissions
    // immutable borrow creates immutable reference and disables mutations and moves
    let mut s = String::from("hello"); // s: r/w/o
    let s_ref = &s; // s: no permissions, s_ref: r/o, *s_ref: r
    println!("{s_ref}"); // s_ref loses all permissions since it is not used after and s regains 

    // cannot mutate a immutable reference
    let mut s = String::from("hello"); // s: r/w/o
    let s_ref = &s; // s: no permissions, s_ref: r/o, *s_ref: r
    //s_ref.push_str(" world");  s ref does not have write permission

    // cannot mutate immutably borrowed data
    let mut s = String::from("hello"); // s: r/w/o
    let s_ref = &s; // s: r permissions, s_ref: r/o, *s_ref: r
    // s.push_str(" world")  s does not have write permission

    // cannot move data out of a reference
    let mut s = String::from("hello"); // s: r/w/o
    let s_ref = &s; // s: r permissions, s_ref: r/o, *s_ref: r
    // let s2 = *s_ref; cannot move the data out of the s_ref

    // mutable borrow creates mutable reference which disables borrowed data from read, write, and move
    let mut s = String::from("hello"); // s: r/w/o
    let s_ref = &mut s; // s: no permissions, s_ref: r/o, *s_ref: r/w
    s_ref.push_str(" world");
}

fn inner(x: &mut i32) {
    let another_num = 1; // own local variable on the stac
    let a_stack_ref = &another_num; // on the stack, points to another_num

    let a_box = Box::new(2); // allocated 2 on the heap and a_box is on the stack and points to the value
    let a_box_stack_ref = &a_box; // a_box_stack_ref is on the stack and points to a_box on the stac
    let a_box_heap_ref = &*a_box; // a_box_heap_ref points to the value 2 on the heap

    *x += 5; // x points to value in main on the stack, mutable refernce so we can increment
}

