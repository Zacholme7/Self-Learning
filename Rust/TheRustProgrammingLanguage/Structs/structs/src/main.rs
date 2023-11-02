// defintiion of a struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Point {
    x: i32,
    y: i32,
}

// tuple structs
// useful when we want to name tuple and make differet types
struct Color(i32, i32, i32);

// unit like struct
struct AlwaysEqual;

fn main() {
    // structs are more flexible than tuples
    // create instance by giving concrete values
    let mut user1 = User {
        email: String::from("zacholme@gmail.com"),
        username: String::from("zacholme"),
        active: true,
        sign_in_count: 1,
    };

    // use dot notation to access and change if mutable
    user1.email = String::from("zacholme2@gmail.com");

    // make new struct using update syntax
    // this moves the data!!!!
    let user2 = User {
        email: String::from("zacholme3@gmail.com"),
        ..user1
    };

    // init tuple struct
    let black = Color(0, 0, 0);

    // init unit like struct
    let subject = AlwaysEqual;

    // rust borrow checker will track ownership on structs
    // both on struct level and field level
    let mut p = Point {x: 0, y: 0}; // p: r/w/o, p.x: r/w/o, p.y: r/w/o
    let x = &mut p.x; // p: no perm, p.x: no perm, x: r/o, *x: r/w
    *x += 1; 


}

fn build_user(email: String, username: String) -> User {
    // can use shothant to init
    User {
        active: true,
        username,
        email, 
        sign_in_count: 1,
    }
}
