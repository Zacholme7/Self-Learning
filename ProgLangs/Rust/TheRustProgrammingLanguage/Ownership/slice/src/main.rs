fn main() {
    // a slice lets you reference a contigous sequence of elements in a collection rather than the whole colleciotn
    // it is a non owning pointer

    // compiles but word can change and invalidae the word index
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();

    // a string slice is a reference to part of a string

    let s = String::from("hello world");
    let hello: &str = &s[0..5]; // reference to 0..5 bit "hello"
    let word: &str = &s[6..11]; // reference to 6..11 bit "world"
    let s2: &String = &s; // points to s 

    // slices are fat pointers, pointers with metadata
    // slices are also references, so they also change permissions on referenced data

    let mut s = String::from("hello world"); // s: r/w/o
    let hello: &str = &s[0..5]; // s: r, hello: r/o, *hello: r
    println!("{hello}"); 
    // hello and *hello loses all permissions since not in use, s regarins r/w/o
    s.push_str(" world"); 
    // s loses all permissions since it is not in use anymore

    // range syntax
    // if want to start at index 0, drop first value
    // if want to end at len - 1, drop second value

    // the following 2 are equal
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];

    // the following 2 are equal
    let s = String::from("hello")
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];

    // string literals are slices
    let s = "hello world"; // s is &str, an immutable reference


}

// returns single value that is tired to the underlying data
fn first_word_slice(s: &str) -> &str {
    let bytes = a.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}