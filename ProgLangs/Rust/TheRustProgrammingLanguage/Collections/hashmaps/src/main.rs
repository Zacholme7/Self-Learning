// import the hashmap
use std::collections::HashMap;

fn main() {
    // stores as mapping of keys of type K to values of type V using a hasing function
    
    // creating hash map
    let mut scores = HashMap::new();

    // data is stored on the heap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green", 20);

    // accessing
    let team_name = String::)rom("Blue");
    // get method returns Option<&V>
    // if no value for key, None will be returned
    // calling copied on the Opption gets Option<i32> instead of Option<&i32>
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // iterating
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // for values that implement copy, they are copied into the has map
    // for owned values, they are moved into the hashmap and it will be the owner
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are not invalid

    // updating hash mpa
    // use same key with different value to replace
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores); // will print 25

    // adding key/value only if it is not present
    // "entry" checks if key exists
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // update value based on old value

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);


}
