// Notes: Exitential types
// -----------------------
// - type inference is why you dont have to specify types that often
//  - only for vars and arguments/return types of closures
// - all functions marked with something like async fn or returning impl Trait have existential
// return type
//  - signature does not give the true type of the return value, just a hint
// - zero cost type erasure

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let even_numbers = filter_even(numbers);

    for num in even_numbers {
        println!("{}", num);
    }
}

// Here we're using `impl Trait` in the return position to indicate
// that this function returns some type that implements `Iterator<Item=i32>`,
// without specifying what the concrete type is.
fn filter_even(numbers: Vec<i32>) -> impl Iterator<Item = i32> {
    numbers.into_iter().filter(|&x| x % 2 == 0)
}
