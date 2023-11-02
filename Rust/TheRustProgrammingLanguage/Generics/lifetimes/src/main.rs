use std::fmt::Display;

// lifetime annotation in struct definitions
struct ImporantExcept<'a> {
    part: &'a str,
}

// lifetime names for struct fields always need to be declared after impl and used after structs name
impl<'a> ImportantExcept<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// static lifetime denotes reference can live for entire program
// string literals have static lifetime
let s: &'static str = "I have a static lifetime";

fn main() {
    // lifetimes ensure that references are valid as long as we need them to be
    // every reference has a lifetime which is the scope for which that reference is valid
    // most of the time, they are implict and inferred
    // must only annitate lifetimes when there can be multiple
    // goal of lifetimes is to prevent danling references
    // rusts borrow checker compares scopes to determine whether vorrows are valid

    // this works
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("the longest string is {}", result);

    // this works
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("zyx");
        let result = longest(string1.as_str(), string2);
        println!("the longest string is {}", result);
    }

    // this does not work
    // lifetime is annoted 'a but string2 does not live to the outer scope
    /* 
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("zyx");
        result = longest(string1.as_str(), string2);
    }
    println!("the longest string is {}", result);
    */

    // rust compiiler will automatically annotate some lifetimes, lifetime ellision
    // three rules
    // first rule: compiler assigns different lifetime param to each lifetimein each input type
    // second rule: if there is exactly one input lifetime param, it is assigned to all output lifetime params
    // third rule: if multiple input lifetime params and one is &self or &mut slef, lifetime of self is assigned to all output lifetime params


}

// lifetime annotation syntax
// describe the relationships of lifetimes of multiple references to each other
// without affecting the lifetimes
// the function takes two params, both are str slices that live at least as long as lifetime 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


// generic type parameter, trait bounds, and lifetime together
fn longest_with_an_announcement<'a, T(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcment {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


