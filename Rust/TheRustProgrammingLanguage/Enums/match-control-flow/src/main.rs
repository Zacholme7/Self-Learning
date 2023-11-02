#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // rest of states
}
enum Coin {
    Penny, 
    Nickle,
    Dime,
    Quarter(UsState),
}

fn main() {
    // match allows you to compare a value against a series of patterns
    // you can then execute code based on which pattern matches
    // think of match like a coin sorting machine

    let five = Some(5);
    let six = plus_one(five);
    let seven = plus_one(six);

    // matches are exaustive, they must cover all possibilities

    // can use catch all patterns and _ placeholder
    let dice_roll = 9;
    // use other when we want to use the value
    match dice_roll {
        3 => println!("this is a 3"),
        7 => println!("this is a 7"),
        other => println!("this is the other {}", other),
    }
    // use _ when we dont want to use the value
    match dice_roll {
        3 => println!("this is a 3"),
        7 => println!("this is a 7"),
        _ => (),
    }

    // how matches interact with ownership
    // if enum contains non copyable data, must be careful if it will move or borrow
    let opt: Option<String> = Some(String::from("hello world")); // opt: r/o
    match opt {
        Some(_) => println!("Some"), // not moving the opt since we use _
        None => println!("None"),
    }
    println!("{:?}", opt);

    let opt: Option<String> = Some(String::from("hello world")); // opt: r/o
    match opt {
        Some(s) => println!("Some: {}", s), // opt is moved and loses all permissions
        None => println!("None"),
    }
    // println!("{:?}", opt); lost permissions, wont work

    // should match a reference is we dont want to move
    let opt: Option<String> = Some(String::from("hello world")); // opt: r/o
    match &opt { // opt: r
        Some(s) => println!("Some: {}", s), // we have read permissions on the borrow
        None => println!("None"),
    }
    println!("{:?}", opt);

    // if let expression, less boilerplate
    let coin: Coin::Quater(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("state quater from {:?}", state);
    } else {
        count += 1;
    }


}

// match function for a coin enum
fn value_in_cents(coin: Coin) -> u8 {
    // match expression can be any type
    match coin {
        // arms of match consists of a pattern and some code
        // if pattern matches, then code is executed
        Coin::Penny => {
            println!("lucky penny");
            1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}


// matching an Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}