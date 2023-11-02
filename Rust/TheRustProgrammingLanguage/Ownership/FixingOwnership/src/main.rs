use std::rc::Rc;

fn main() {
    // --------------------------------------------------------
    // FIXING UNSAFE PROGRAM: copying vs moving out of a collection
    // --------------------------------------------------------
    let v: Vec<i32> = vec![1, 2, 3]; // v: r/o
    let n_ref: &i32 = &v[0]; // v: no permissions, n_ref: r/o, *n_ref: r
    let n: i32 = *n_ref;

    // the same thing with a string gives an error
    /* 
    let v: Vec<String> = vec![String::from("hello world")]; // v: r/o
    let s_ref: &String = &v[0]; // v: no permissions, s_ref: r/o, *s_ref: r
    let s: String = *s_ref; // needs r/o permission, tried to take ownershp from string from vector, but reference is non owning pointers
    */

    // if a value does not own heap data, then it can be copied without a move

    // if we have non copy tupe, how to safely access
    // 1) avoid taking ownership and just use immutable refernce
    let v: Vec<String> = vec![String::from("hello world")];
    let s_ref: &String = &v[0]; 
    println!("{s_ref}");

    // 2) can clone the data if you wnat to get ownership while leaving the vector alone
    let v: Vec<String> = vec![String::from("hello world")];
    let mut s: String = v[0].clone();
    s.push('!');
    println!("{s}");

    // 3) can use methods to move string out of vector
    let v: Vec<String> = vec![String::from("hello world")];
    let mut s: String = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v.len() == 0);


    // --------------------------------------------------------
    // FIXING SAFE PROGRAM: mutating different tuple fields
    // --------------------------------------------------------
    // may conflate two different paths as the same path

    // this is okay since name1 does not lose write permissions when we reference first
    let mut name = (String::from("hello"), String::from("world")); // name/name.0/name.1: r/w/o
    let first = &name.0; // name/name.0: r, first: r/o, *first: r
    name.1.push_str(", Esq");
    println!("{first} {}", name.1);

    // can lose track when we call function
    /* 
    let first = get_first(&name); // removes all permissions except for read
    name.1.push_str(", Esq"); // first is still in use, so name1 does not regain write after the function call
    println!("{first} {}", name.1);
    */


    // --------------------------------------------------------
    // FIXING SAFE PROGRAM: mutating different array elements
    // --------------------------------------------------------
    let mut a = [0, 1, 2, 3];
    let x = &mut a[0]; // a: no permissions, x: r/o, *x: r/w
    *x += 1; 
    // x no longer in use so a regains all permissions
    println!("{a:?}");
    

}

fn get_first(name: &(String, String)) -> &String {
    &name.0
}



// --------------------------------------------------------
// FIXING UNSAFE PROGRAM: Returning reference to the stack
// --------------------------------------------------------
/* 
// s is deallocated when we return from the function since it is on the stack so the reference is not valid
fn return_a_string() -> &String {
    let s = String::from("hello world");
    &s
}
*/

// valid: move ownership of string out of the function
fn return_a_string_ownership() -> String {
    let s = String::from("hello world");
    s
}

// valid: return string literal, we dont want to change it after
fn return_a_string_literal() -> &'static str {
    "hello world"
}

// valid: defer borrow checking to runtime
fn return_a_string_defer_borrow() -> Rc<String> {
    let s = Rc::new(String::from("hello world"));
    Rc::cloen(&s)
}

// valid: slot the string
fn return_a_string_slot(output: &mut String) {
    output.rreplace_range(..., "hello world5jA;")
}
// --------------------------------------------------------


// --------------------------------------------------------
// FIXING UNSAFE PROGRAM: not enough permissions
// --------------------------------------------------------
// common issue is trying to mutate read only data, trying to drop data behind reference

// name is immuable reference but push requires write permissions
/* 
fn stringify_name_with_title(name: &Vec<String>) -> String { // name: r/o, *name: r
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}*/

// one solution: make input mutable
// not good, functions should not mutate inputs if the caller would not expect it
fn stringify_name_with_title(name: &mut Vec<String>) -> String {
    name.push(String::from("Esq. "));
    let full = name.join(" ");
    full
}

// another solution: take ownership of name
// not good, rare for rust functio to take ownership of heap owning data structure
fn stringify_name_with_title_ownership(mut name: Vec<String>) -> String {
    name.push(String::from("Esq. "));
    let full = name.join(" ");
    full
}

// original choice what a good one, but change body of function
fn stringify_name_with_title(name: &Vec<String>) -> String { // name: r/o, *name: r
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}


// --------------------------------------------------------
// FIXING UNSAFE PROGRAM: Aliasing and mutating a data structure
// --------------------------------------------------------
// usafe to use reference to heap data that gets deallocated by another alias
// all solutionss share same idea: shorten lifetime of borrows on dst to not overlap with mutation of dst

// rejected since let largest = removed write permission on dst but dst.push needs the write permission
// dst.push could deallocate contents of dst which would invalidate the reference largest
/* 
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap(); // removes w permission on dst
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}*/

// one solution: shorten lifetime of largest to not overlap with dst.push, could clone largest
// could have performance hit for allocating and copying the string data
fn add_big_strings_clone(dst: &mut Vec<String>, src: &[String]) {
    let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}

// another solution: perform all length comparisons first then mutate dst
// could have performance hit for allocating the vector to_add
fn add_big_strings_compare(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
    let to_add: Vec<String> = src.iter().filter(|s| s.len() > largest.len()).cloned().collect();
    dst.extend(to_add);
}

// another solution: copy out length of largest
// best solution
fn add_big_strings_copy(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}





