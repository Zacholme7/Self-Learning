struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Message {
    Hello { id: i32 },
}

fn main() {
    // can match literals directly
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching named variables
    // values in the match will shadow outside variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched {y}"),
        _ => println!("default"),
    }

    // multiple patterns
    // can match multiple using |
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        _ => println!("anything"),
    }

    // matching ranges
    // can use ..= to match a range
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // can destructure a struct with matching
    let p = Point { x: 0, y: 7 };
    let Point {x, y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // match struct based on value
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // can destructure enums
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }

    // can ignore entire values with _
    foo(3, 4);

    // can ignore just parts of a value
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    // can ignore a unused variable by starting its name with _
    // still binds but makes it so there is not a warning
    let _x = 5;
    let y = 10;

    // can ignore remaining values with ...
    let origin = Point { x: 0, y: 0};

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    // can also use a match guard with a pattern
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    // @ bindings
    // lets us create variable taht holds a value at the same time were testing that value for a pattern match
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7, // checks if it is in range 3..7 then binds it to id_variable
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

// ignoring the first param
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
