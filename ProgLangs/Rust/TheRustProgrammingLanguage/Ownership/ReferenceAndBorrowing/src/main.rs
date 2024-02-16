fn main() {
    // references are non owning pointers because they do not own the data they point too
    let m1 = String::from("hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // m1 and m2 passed in as reference so ownership is not transferred
    let s = format!("{} {}", m1, m2);

    // underlying mechanims to follow pointer is dereferencing
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x; // *x reads the heap value, so a = 1
    *x *= 1; // *x modifies the heap value, so x points to a box of the value 2

    let r1: &Box<i32> = &x; // r1 points to x on the stack which then points to the box on the heap
    let b: i32 = **r1; // follow reference from r1 -> x -> 2

    let r2: &i32 = &*x; // r2 points to the heap value directly
    let c: i32 = *r2; // read the value from the box

    /*
    rust usually implicitly references and defererences

    rust avoids simultaneous aliasing and mutation
    aliasing: accessing the same data through different variables
    enforces for boxes by disallowing aliasing, assigning box from one var to another will move ownership
    owned data can only be accessed through the owner

    borrow checker three kinds of permissions, 
    read: data can be copied to another location
    write: data can be mutated in place
    own: data can be moved or dropped
    these permissions only exist at compile time
    by default, have read/own permission on data, if annoted with mut then read/own/write permissions
    references can temporarily remove these permissions 
    */
    
    let mut vec: Vec<i32> = vec![1, 2, 3]; // r/w/o
    let num: &i32 = &vec[2]; // vec: r, num: r/o, *num: r
    println!("Third element is {}", *num); // vec: r/w/o, num is no longer in use so it loses permissions and vecs permissions are restored
    vec.push(4); // vec loses all permissions since it is not used anymore

    let o = 0; // o: r/o
    let mut o_ref = &o; // o: r, o_ref: r/w/o, *o_ref: r    can modify what o_ref points to, o_ref = &y, but cannot mutated the pointed data

    /*
    permissions are defined on paths, anything you can put on the left hand side of an assignment
    variables (a)
    dereferences of paths (*a)
    array accesses of paths (a[0])
    fields of paths (a.0)
    any combination of above (*((*a)[0]).1)
    */

    // creating reference to data causes that data to be temporarily read only until the refernece is no longer used

    // use mutable references to mutate data
    let mut vec2: Vec<i32> = vec![1, 2, 3]; // vec2: r/w/o
    let num2: &i32 = &vec2[2]; // vec2: r, num2: r/o, *num2: r
    //vec2.push(4); error, vec does not have write permissions since num2 is stil in use
    println!("Third element is {}", *num2);

    // mutable references are unique and non owning
    let mut vec3: Vec<i32> = vec![1, 2, 3]; // vec3: r/w/o
    let num3: &mut i32 = &mut vec3[2]; // vec3: nothing, num: r/o, *num: r/w
    *num3 += 1; 
    println!("This element is {}", *num3); // vec3: r/w/o since num3 is no longer is use after
    println!("Vector is now {:?}", vec3);  

    // mutable reference is created with &mut
    // allow mutation by prevent aliasing

    // mutable refernces can be temporarily downgraded
    let mut vec: Vec<i32> = vec![1, 2, 3]; // vec: r/w/o
    let num: &mut i32 = &mut vec[2]; // vec: nothing, num: r/o, *num: r/w
    let num2: &i32 = &*num; // *num: r, *num2: r, num: r, num2: r/o
    println!("{} {}", *num, *num2);

    // permissions are returned at the end of a references lifetime
    // lifetime: range of code spanning from its birth (where reference is created) to its death (last time it is used)
    let mut x = 1; // x: r/w/o
    let y = &x; // lifetime of y starts here, x: r, y: r/o, *y: r
    let z = *y; // z* r/o 
    // y is no longer used so x gets back r/w/o
    x += z;

    // data must outlive all of its references
    // must have ownership permission to drop

    // flow permission: expected whenever an expression uses an input reference or returns an output reference
    // does not change throughout body of a function

    // input/output references are trated differently than references within a function body
    // rust used a different mechanism, the flow permission, to check the safety of those references
}

// reference since we dont want to take ownership
fn greet(g1: &String, g2: &String) {
    println!("{} {}", g1, g2);
}

fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0]; // v: r, *v: r, c: r/o, *c: r
    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase(); // *v: r/w
    } else {
        // *v: r/w
        println!("Already capitalized");
    }
}
