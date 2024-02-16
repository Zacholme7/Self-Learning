// enum to represent different types being stored
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // data the collections store are stored on the heap
    // vector: allows you to store a variable number of values next to each other
    // string: collection of characters
    // hash-map: associate value with a particular key

    // vector
    // call vec::new to create a new vector
    // need type annotation if its going to start empty
    let v: Vec<i32> = Vec::new();
    // can also use the vec! macro
    let v = vec![1, 2, 3];
    // adding elements to a vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    // to reference value in vector
    // 1) indexing
    // 2) using "get" method
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // gives reference to the element
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("the third element is {third}"),
        None => println!("There is no third element"),
    }
    // index when we want out of bounds to fail
    // "get" when we want to handle out of bounds
    // owernship rules apply

    // iterate over values in a vector
    let v = vec![1, 2, 3];
    for n_ref in &v {
        let n_plus_one: i32 = *n_ref + 1;
        println!("{n_plus_one}");
    }

    let mut v = vec![100, 32, 57];
    for n_ref in &mut v {
        *n_ref += 50;
    }

<<<<<<< HEAD:Rust/TheRustProgrammingLanguage/common-collections/src/main.rs
    // non copytable types cannot be moved out of a vector by indexing

=======
    // iterators
    // conati a pointer to data wihtin the vector
    let muv v: Vec<i32> = vec![1, 2]; // v is a pointer to a vector on the heap
    let mut iter: Iter<'_, i32> = v.iter(); // points to the start of the vector, removed the write permission
    let n1: &i32 = iter.next().unwrap(); // n1 points to 1 on the heap, iter points to 2
    let n2: &i32 = iter.next().unwrap(); // n2 points to 2 on the heap, iter is invalidated
    let end: Option<&i32> = iter.next(); // none since iter is invalidated 
>>>>>>> 0e47ad2f820e9994776d0bb5a353e2c738175227:Rust/TheRustProgrammingLanguage/Collections/vectors/src/main.rs
    
    // can use range to iterate without ap ointer
    let mut v: Vec<i32> = vec![1, 2];
    let mut iter: Range<usize> = 0 .. v.len();
    let i1: usize = iter.next().unwrap();
    let n1: &i32 = &v[i1];

    // vectors can only store values of the same type
    // enums can store multiple types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Hello")),
        SpreadsheetCell::Float(10.12),
    ];

    // dropping a vector drops its element
    {
        let v = vec![1, 2];
        // do stuff with v
    } // v goes out of scope and is freed here
}
