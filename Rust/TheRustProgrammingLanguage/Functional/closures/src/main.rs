use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // unwrap or else
        // takes closure without arguments that returns a value T
        // if Option is Some, returns that value, else calls closure and returns that value
        // captures an immutable reference to self inventory
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    // closures are anonymous functions you can save in a variable or pass as arguments to other functions
    // can create in one place and call elsewhere
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // closures dont usually require you to annotate the types of the parameters or the return value
    // closures are not used in an exposed interface like fucntions
    // they are stored in variables and used without naming them and exposing them to users of our library

    // typically short and revelet only within narrow context
    // although, can annotate type if we would like
    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowley..");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // will all produce the same behavior
    //fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    //let add_one_v2 = |x: u32| -> u32 { x + 1 };
    //let add_one_v3 = |x|             { x + 1 };
    //let add_one_v4 = |x|               x + 1  ;

    // rust will infer a concrete type for params and return
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    //let n = example_closure(5); error since it was already inferred to take a string

    // clsoures can capture values from the environment in 3 ways
    // borrow immutably, borrow mutably, taking ownershp
    // decide which based on what body of the function does with captured value

    // captures immutable reference since that is all it needs
    let list = vec![1, 2, 3];
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // closure now captures mutable reference
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(10);
    // cannot print here since no other borrowrs are allowed when there is a mutable borrow
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    // can use "move" to take force ownership
    // only needs immutable reference, but move says it should take ownership
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // closure body can do any of the following
    // move captured value out of closure
    // mutate captured value,
    // neither move nor mutate
    // capture nothing to begin with

    // the way closure captures/handles values from env affects which traits the closure implemented
    // closures will auto implemente one, two, or three of these fn traits
    // FnOnce: applies to closures that can be called once. all closures implement this trait
        // closure that moves captures values out of its body will only implement this since it can only be called once
    // FnMut: applies to closures that dont move captured calues out of their body but might mutate captured value
        // can be called more than once
    // Fn: closures that dont move captured calues out of body and dont mutate captured values
        // as well as clsoures that capture nothing from their environment
        // can be called more thatn once without mutating their environment

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // sort by key uses FnMut
    // calls the clsoure multipel times, one for each slice
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);


}

/*
T is the generic type representing the type of the value in the Some variant
F type is the type param named f which is the closure we provide
trait bound on F is FnOnce() -> T which means F must be able to be called once, take no args and return T
    expressed constraint that F will only be called at most once
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
*/

// closures must name captured lifetimes
// when function accepts or returns closures, you need to think about lifetime
// need to tell rust that the closure returned must not live longer than s_ref
fn make_a_cloner(s_ref: &str) -> impl Fn() -> String + '_ {
    move || s_ref.to_string()
}
