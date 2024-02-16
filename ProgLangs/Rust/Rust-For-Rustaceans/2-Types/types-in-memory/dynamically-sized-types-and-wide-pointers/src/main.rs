// Notes: Dynamically Sized Types and Wide Pointers
// -----------------------------------------------------
//
// - most types in rust implement the Sized trait automatically (size known at compile time)
//   - trait objects and slices do not (they are dynamically sized types, dyn)
//   - [T] and dyn Trait
// - compiler requires types to be sized nearly everywhere
//   - every single type bound you write includes T: Sized, can explicity opt out with T: ?Sized
//     - ? : may not be
// - wide/fat pointer can be used to bridge gap between unsized and sized types
//   - like normal pointer but includes extra word sized field
//   - when you take ref to DST, wide pointer is automatically constructed
//   - wide pointer is twice the size of usize, one for pointer and one for extra info
//
// sidenote: when to use slices?
// 1) borrowing data: when you just want to borrow view of sequences w/o taking ownership, slices
//    are the way to go
// 2) function parameters: if function should work with any contiguous sequence regardless of
//    container type, use slice
// 3) performance: using slices can avoid unnecessary data cloning
//
// sidenote: when to use vec<t>?
// 1) ownership and lifetime management: when you need to own and manage a dynamically sized list
//    of elements throughout its lifetime
// 2) dynamic resizing: if your application needs to dynamically resize, vec is a good option
// 3) returning allocated data from fucntions: when you need to return newly allocated list, return
//    a vector

// good use case for using a slice
fn sum_elements(elements: &[i32]) -> i32 {
    elements.iter().sum()
}

// good use case for taking in a slice but returning vector
fn collect_positive_numbers(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().cloned().filter(|&x| x > 0).collect()
}

// slice does not implemented the Sized trait, so the slice is fat pointers
fn print_slice(slice: &[i32]) {
    for element in slice {
        println!("{}", element);
    }
}

// can write generic function with ?Sized to relax default Sized requirement
fn process<T: ?Sized>(value: &T) {
    // do work, value may be unsized
} // this function can now take sized types (i32, string, ..) and dynamically sized types ([i32], dyn trait)

fn main() {
    let numbers = vec![1, 2, 3];
    // &numbers creates a wide pointer to a slice of i32 from vec<i32>
    // includes starting memory address and length of slice
    print_slice(&numbers);

    // slice vs vec examples
    let nums = vec![1, 2, 3, 4, 5];
    let array = [1, 2, 3, 4, 5];
    println!("Sum of nums: {}", sum_elements(&nums));
    println!("Sum of array: {}", sum_elements(&array));

    let mixed_numbers = [1, -2, 3, -4, 5];
    let positives = collect_positive_numbers(&mixed_numbers);
    println!("Positive numbers: {:?}", positives);
}
