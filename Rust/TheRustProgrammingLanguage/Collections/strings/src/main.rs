fn main() {
    // strings are a collection of bytes
    // rust has only one string type in the core language, string slice "str"
        // usually seen in its borrowed form &str
    // "String" type, is a growable, mutable, owned, utf-8 encoded string type
    // String is implemented as a wrapper around a vector of bytes

    // create a new String
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents"); // equivalent to just above

    // updating a string
    // grow a string
    let mut s = String::from("hello");
    let s2 = " world";
    s.push_str(s2); // does not take ownership of str slice
    println!("{}", s2); // can print s2 since the push does not take ownership

    let mut s = String::from("lo");
    s.push('l'); // push a single character

    let s1 = String::from("hello");
    let s2 = String::from(" world");
    // adding reference of the second string to the first string
    let s3 = s1 + &s2; // s1 has been moved so we can no longer use it 

    // can also use formate macro
    // uses references so it does not take owenership
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");

    // indexing into strings
    // rust strings to not support indexing

    // this will fail
    // let s1 = String::from("hello");
    // let h = s1[0]

    // index into string will not always correspond to one byte
    // cannot to it in constant time since it would have to walk through string and see how many bytes

    // use range to create a sgring slice
    // should use caution with this

    // iterate over strings
    // be explicit if you want characters or bytes
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }




}
