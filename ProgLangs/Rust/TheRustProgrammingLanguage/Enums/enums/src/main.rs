// define an enum, custom datatype that we can use anywhere in our code
// can put data directly into variants
// each variant can have different types and amount of associated data
enum IpAddrKind {
    V4(String, u8, u8, u8, u8), 
    V6(String),
}

// can also define methods on enums
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    // enums allow you to define a type by enumerating its possible variants
    // can say a value is one of a possible set of values

    // create instances of an enum
    let four = IpAddrKind::V4(String::from("127.0.0.0"), 127, 0, 0, 1);;
    let six = IpAddrKind::V6(String::from("::1"));

    route(four);
    route(six);

    // call method on enum
    let m = Message::Write(String::from("hello"));
    m.call();

    // Option is an enum defined by the standard library
    // encodes when a value can be something or nothing
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    // must convert Option<T> to a T before you can perofmr T operations with it
    // to get value, usually want a case where there is a Some and when there is a None
}

// can define a function that takes an enum
fn route(ip_kind: IpAddrKind) {}
